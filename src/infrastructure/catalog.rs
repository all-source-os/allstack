//! Catalog discovery reads repository data without opening mutable run stores.
use super::{bootstrap_fs::Root, digest};
use crate::domain::{Run, WorkflowSelection};
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};

const CATALOG: &str = "workflows/catalog.toml";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Index {
    schema_version: u32,
    workflows: Vec<WorkflowSelection>,
}

#[derive(Serialize)]
pub struct Entry {
    pub id: String,
    pub path: String,
    pub description: String,
    pub digest: String,
    pub methods: BTreeMap<String, Method>,
    pub hosts: Vec<String>,
    pub stage_order: Vec<String>,
    pub host_scope: String,
    #[serde(skip)]
    pub bytes: Vec<u8>,
}

#[derive(Serialize)]
pub struct Method {
    pub path: String,
    pub digest: String,
    #[serde(skip)]
    pub bytes: Vec<u8>,
}

fn read(root: &Root, path: &str) -> Result<Vec<u8>> {
    ensure!(
        path.starts_with("workflows/") && !path.split('/').any(|p| p.starts_with('.')),
        "catalog inputs must be visible files under workflows/: {path}"
    );
    let bytes = root
        .read(path)?
        .with_context(|| format!("catalog input missing: {path}"))?;
    ensure!(
        bytes.len() <= 64 * 1024,
        "catalog input exceeds 64 KiB: {path}"
    );
    Ok(bytes)
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id.as_bytes()[0].is_ascii_lowercase()
        && id
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
}

fn load(root: &Root, selection: WorkflowSelection) -> Result<Entry> {
    ensure!(valid_id(&selection.id), "invalid workflow id");
    ensure!(
        selection.path.ends_with(".toml"),
        "workflow definition must be TOML"
    );
    let bytes = read(root, &selection.path)?;
    let definition: toml::Value = toml::from_str(std::str::from_utf8(&bytes)?)?;
    let supported: toml::Value = toml::from_str(include_str!("../../workflows/delivery.toml"))?;
    let table = definition.as_table().context("workflow must be a table")?;
    ensure!(
        table.keys().all(|key| supported.get(key).is_some()),
        "unknown workflow field"
    );
    ensure!(
        definition.get("id").and_then(toml::Value::as_str) == Some(&selection.id),
        "workflow id differs from catalog entry"
    );
    for key in [
        "schema_version",
        "hosts",
        "host_scope",
        "stage_order",
        "dispositions",
        "storage",
        "handoff",
        "execution",
        "publication",
        "stages",
    ] {
        ensure!(
            definition.get(key) == supported.get(key),
            "unsupported workflow contract: {key}"
        );
    }
    for key in ["version", "status", "description"] {
        ensure!(
            definition
                .get(key)
                .and_then(toml::Value::as_str)
                .is_some_and(|s| !s.trim().is_empty()),
            "workflow {key} must be nonempty text"
        );
    }
    let paths = definition
        .get("methods")
        .and_then(toml::Value::as_table)
        .context("workflow methods missing")?;
    ensure!(
        paths.len() == 2 && paths.contains_key("plan") && paths.contains_key("build"),
        "workflow requires exactly plan and build methods"
    );
    let mut methods = BTreeMap::new();
    for (name, path) in paths {
        let path = path.as_str().context("method path must be text")?;
        let bytes = read(root, path)?;
        ensure!(
            !std::str::from_utf8(&bytes)?.trim().is_empty(),
            "workflow method is empty"
        );
        methods.insert(
            name.clone(),
            Method {
                path: path.into(),
                digest: digest(&bytes),
                bytes,
            },
        );
    }
    Ok(Entry {
        id: selection.id,
        path: selection.path,
        description: definition["description"].as_str().unwrap().into(),
        digest: digest(&bytes),
        methods,
        hosts: vec!["codex".into(), "claude-code".into()],
        stage_order: vec![
            "plan".into(),
            "build".into(),
            "verify".into(),
            "ship".into(),
        ],
        host_scope: "sequential-same-machine".into(),
        bytes,
    })
}

pub fn entries(repo: &Path) -> Result<Vec<Entry>> {
    let root = Root::new(repo)?;
    let selections = match root.read(CATALOG)? {
        Some(bytes) => {
            ensure!(bytes.len() <= 64 * 1024, "catalog exceeds 64 KiB");
            let index: Index = toml::from_str(std::str::from_utf8(&bytes)?)?;
            ensure!(
                index.schema_version == 1
                    && !index.workflows.is_empty()
                    && index.workflows.len() <= 32,
                "unsupported catalog schema or entry count"
            );
            index.workflows
        }
        None => vec![WorkflowSelection {
            id: "delivery".into(),
            path: "workflows/delivery.toml".into(),
        }],
    };
    let mut ids = std::collections::BTreeSet::new();
    let mut paths = std::collections::BTreeSet::new();
    let mut entries = Vec::new();
    for selection in selections {
        ensure!(
            ids.insert(selection.id.clone()) && paths.insert(selection.path.clone()),
            "duplicate catalog id or path"
        );
        entries.push(load(&root, selection)?);
    }
    ensure!(
        ids.contains("delivery"),
        "catalog must retain default delivery workflow"
    );
    entries.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(entries)
}

pub fn select(repo: &Path, id: &str) -> Result<Entry> {
    let entries = entries(repo)?;
    let choices = entries
        .iter()
        .map(|e| e.id.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    entries
        .into_iter()
        .find(|e| e.id == id)
        .with_context(|| format!("unknown workflow {id}; available: {choices}"))
}

pub fn verify_scope(repo: &Path, run: &Run) -> Result<()> {
    let root = Root::new(repo)?;
    ensure!(
        digest(&read(&root, run.workflow_path())?) == run.workflow,
        "workflow definition changed"
    );
    for (method, path) in &run.method_paths {
        ensure!(
            run.methods.get(method) == Some(&digest(&read(&root, path)?)),
            "workflow method changed from accepted scope"
        );
    }
    Ok(())
}

pub fn inspect(repo: &Path, id: Option<&str>) -> Result<serde_json::Value> {
    match id {
        Some(id) => Ok(serde_json::to_value(select(repo, id)?)?),
        None => Ok(
            serde_json::json!({"schema_version":1,"default":"delivery","workflows":entries(repo)?,"stores_opened":false}),
        ),
    }
}
