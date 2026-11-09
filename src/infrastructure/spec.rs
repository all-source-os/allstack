//! Design review receipts and digest-bound approvals for the harness spec gate.
use super::{atomic_write, bootstrap_fs::Root, digest, hosts, private_dir, process, store::Store};
use crate::{
    application::spec as rules,
    domain::{Host, SpecApproval, SpecGate, spec_gate, spec_path_valid},
};
use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

const APPROVAL: &str = "allstack.spec.v1.approval";

fn entity(path: &str) -> String {
    format!("allstack-spec-{}", digest(path.as_bytes()))
}

fn reviews(repo: &Path) -> PathBuf {
    repo.join(".allstack/spec/reviews")
}

fn draft(repo: &Path, path: &str) -> Result<String> {
    ensure!(
        spec_path_valid(path),
        "design path must be a relative Markdown path without parent or hidden components"
    );
    let bytes = Root::new(repo)?
        .read(path)?
        .with_context(|| format!("design missing: {path}"))?;
    Ok(digest(&bytes))
}

fn receipt_valid(receipt: &Path, path: &str, design: &str) -> bool {
    fs::read(receipt)
        .ok()
        .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok())
        .is_some_and(|value| {
            value["path"].as_str() == Some(path)
                && value["digest"].as_str() == Some(design)
                && value["verdict"].as_str() == Some("clean")
        })
}

fn latest_receipt(repo: &Path, path: &str, design: &str) -> Result<Option<PathBuf>> {
    let dir = reviews(repo);
    if !dir.is_dir() {
        return Ok(None);
    }
    let prefix = format!("{design}-");
    let mut found = fs::read_dir(&dir)?
        .map(|entry| entry.map(|e| e.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    found.retain(|p| {
        p.file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with(&prefix))
    });
    found.sort();
    Ok(found
        .into_iter()
        .rev()
        .find(|p| receipt_valid(p, path, design)))
}

pub async fn review(repo: &Path, path: &str, host: Host) -> Result<Value> {
    let design = draft(repo, path)?;
    let outcome = process::run(process::Spec {
        argv: hosts::read_only_argv(host),
        cwd: repo.into(),
        // Stays below the 290-second limit on the enclosing operation so a timeout is reported.
        seconds: 280,
        environment: BTreeMap::new(),
        input: Some(rules::review_prompt(path, &repo.to_string_lossy())),
        directory: repo.join(".allstack/process"),
    })?;
    ensure!(
        outcome.success(),
        "review host failed (exit {:?}, timed out {}, owner lost {}); inspect private process evidence at {}: {}",
        outcome.code,
        outcome.timed_out,
        outcome.owner_lost,
        outcome.evidence.display(),
        outcome.stderr
    );
    let findings = hosts::final_text(host, &outcome.stdout)?;
    let verdict = rules::verdict(&findings);
    let dir = reviews(repo);
    private_dir(&dir)?;
    // Zero-padded nanoseconds make lexical filename order match creation order.
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let receipt = dir.join(format!(
        "{design}-{stamp:020}-{}.json",
        uuid::Uuid::new_v4()
    ));
    let record = json!({
        "schema": 1,
        "path": path,
        "digest": design,
        "host": host.name(),
        "verdict": verdict,
        "findings": findings,
    });
    atomic_write(&receipt, &serde_json::to_vec_pretty(&record)?)?;
    Ok(json!({
        "receipt": receipt,
        "path": path,
        "digest": design,
        "host": host.name(),
        "verdict": verdict,
        "findings": findings,
    }))
}

pub async fn approve(
    repo: &Path,
    path: &str,
    approver: &str,
    without_review: bool,
) -> Result<Value> {
    ensure!(
        rules::approver_valid(approver),
        "approver must be 1..128 printable characters"
    );
    let design = draft(repo, path)?;
    let review = if without_review {
        None
    } else {
        let receipt = latest_receipt(repo, path, &design)?.with_context(|| {
            "no clean review receipt for the current digest; run allstack spec review until clean, or pass --without-review"
        })?;
        Some(receipt.to_string_lossy().into_owned())
    };
    let store = Store::open(&repo.join(".allstack")).await?;
    let result = async {
        let sequence = store.facts(&entity(path), APPROVAL).await?.len() as u64 + 1;
        let approval = SpecApproval {
            schema: 1,
            path: path.into(),
            digest: design.clone(),
            approver: approver.trim().into(),
            review: review.clone(),
            without_review,
            sequence,
        };
        store
            .record_fact(&entity(path), APPROVAL, serde_json::to_value(&approval)?)
            .await?;
        Ok::<_, anyhow::Error>(approval)
    }
    .await;
    let shutdown = store.shutdown().await;
    let approval = result?;
    shutdown?;
    Ok(json!({
        "approved": true,
        "path": approval.path,
        "digest": approval.digest,
        "approver": approval.approver,
        "review": approval.review,
        "without_review": approval.without_review,
        "sequence": approval.sequence,
    }))
}

pub async fn check(repo: &Path, path: &str) -> Result<Value> {
    let design = draft(repo, path)?;
    let latest = if repo.join(".allstack").exists() {
        let store = Store::open(&repo.join(".allstack")).await?;
        let facts = store.facts(&entity(path), APPROVAL).await;
        let shutdown = store.shutdown().await;
        let facts = facts?;
        shutdown?;
        facts
            .into_iter()
            .map(serde_json::from_value::<SpecApproval>)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .filter(|a| a.path == path)
            .max_by_key(|a| a.sequence)
    } else {
        None
    };
    let review_ok = match latest.as_ref().and_then(|a| a.review.as_deref()) {
        None => true,
        Some(receipt) => receipt_valid(Path::new(receipt), path, &design),
    };
    let gate: SpecGate = spec_gate(latest.as_ref(), &design, review_ok);
    ensure!(
        gate == SpecGate::Approved,
        "spec {path} is {}",
        gate.reason()
    );
    let approval = latest.context("approved gate without approval")?;
    Ok(json!({
        "approved": true,
        "path": path,
        "digest": design,
        "approver": approval.approver,
        "sequence": approval.sequence,
    }))
}
