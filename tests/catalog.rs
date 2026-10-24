mod common;
use allstack::{
    domain::{Event, WorkflowSelection, validate_transition},
    infrastructure::{catalog, digest, memory::Memory},
};
use std::{fs, os::unix::fs::symlink, path::Path};

fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("workflows/methods")).unwrap();
    let source = Path::new(env!("CARGO_MANIFEST_DIR"));
    for name in [
        "catalog.toml",
        "delivery.toml",
        "bugfix.toml",
        "methods/plan.md",
        "methods/build.md",
        "methods/diagnose.md",
        "methods/repair.md",
    ] {
        fs::copy(
            source.join("workflows").join(name),
            dir.path().join("workflows").join(name),
        )
        .unwrap();
    }
    dir
}

#[test]
fn discovery_selects_distinct_methods_without_opening_stores() {
    let dir = fixture();
    let all = catalog::inspect(dir.path(), None).unwrap();
    assert_eq!(all["stores_opened"], false);
    assert_eq!(all["workflows"].as_array().unwrap().len(), 2);
    let bugfix = catalog::select(dir.path(), "bugfix").unwrap();
    let delivery = catalog::select(dir.path(), "delivery").unwrap();
    assert!(
        String::from_utf8(bugfix.methods["plan"].bytes.clone())
            .unwrap()
            .contains("exact failed condition")
    );
    assert_ne!(
        bugfix.methods["plan"].digest,
        delivery.methods["plan"].digest
    );
    assert_ne!(
        bugfix.methods["build"].digest,
        delivery.methods["build"].digest
    );
    let error = catalog::select(dir.path(), "review")
        .err()
        .unwrap()
        .to_string();
    assert!(error.contains("available: bugfix, delivery"));
    for name in [".allstack", ".prime", ".chronis"] {
        assert!(!dir.path().join(name).exists());
    }
}

#[test]
fn absent_catalog_keeps_legacy_delivery_available() {
    let dir = fixture();
    fs::remove_file(dir.path().join("workflows/catalog.toml")).unwrap();
    let entries = catalog::entries(dir.path()).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].id, "delivery");
    assert!(catalog::select(dir.path(), "bugfix").is_err());
}

#[test]
fn accepted_scope_uses_frozen_paths_and_rejects_definition_or_method_changes() {
    let dir = fixture();
    let selected = catalog::select(dir.path(), "bugfix").unwrap();
    let mut run = common::run();
    run.selection = Some(WorkflowSelection {
        id: selected.id,
        path: selected.path,
    });
    run.workflow = selected.digest;
    run.methods = selected
        .methods
        .iter()
        .map(|(k, v)| (k.clone(), v.digest.clone()))
        .collect();
    run.method_paths = selected
        .methods
        .iter()
        .map(|(k, v)| (k.clone(), v.path.clone()))
        .collect();
    catalog::verify_scope(dir.path(), &run).unwrap();
    fs::write(
        dir.path().join("workflows/catalog.toml"),
        "invalid replacement catalog",
    )
    .unwrap();
    fs::write(
        dir.path().join("workflows/delivery.toml"),
        "unrelated changed definition",
    )
    .unwrap();
    catalog::verify_scope(dir.path(), &run).unwrap();
    let path = dir.path().join(run.workflow_path());
    fs::write(&path, b"changed selected definition").unwrap();
    assert!(
        catalog::verify_scope(dir.path(), &run)
            .unwrap_err()
            .to_string()
            .contains("workflow definition changed")
    );
    fs::write(path, selected.bytes).unwrap();
    fs::write(
        dir.path().join(&run.method_paths["plan"]),
        "changed selected method",
    )
    .unwrap();
    assert!(
        catalog::verify_scope(dir.path(), &run)
            .unwrap_err()
            .to_string()
            .contains("workflow method changed")
    );
}

#[test]
fn invalid_catalogs_and_unsupported_contracts_fail_closed() {
    for text in [
        "schema_version=2\nworkflows=[]",
        "schema_version=1\nworkflows=[]",
        "schema_version=1\nunknown=true\nworkflows=[]",
        "schema_version=1\n[[workflows]]\nid='bugfix'\npath='workflows/bugfix.toml'",
        "schema_version=1\n[[workflows]]\nid='delivery'\npath='workflows/delivery.toml'\n[[workflows]]\nid='delivery'\npath='workflows/bugfix.toml'",
        "schema_version=1\n[[workflows]]\nid='delivery'\npath='workflows/bugfix.toml'",
        "schema_version=1\n[[workflows]]\nid='delivery'\npath='workflows/../outside.toml'",
    ] {
        let dir = fixture();
        fs::write(dir.path().join("workflows/catalog.toml"), text).unwrap();
        assert!(catalog::entries(dir.path()).is_err(), "{text}");
    }
    for (old, new) in [
        ("allow_trunk_push = false", "allow_trunk_push = true"),
        ("schema_version = 1", "schema_version = 2"),
        (
            "stage_order = [\"plan\", \"build\", \"verify\", \"ship\"]",
            "stage_order = [\"plan\", \"ship\"]",
        ),
        (
            "plan = \"workflows/methods/diagnose.md\"",
            "plan = \"/etc/passwd\"",
        ),
        (
            "build = \"workflows/methods/repair.md\"",
            "build = \"workflows/methods/absent.md\"",
        ),
        (
            "status = \"implementation-preview\"",
            "status = \"implementation-preview\"\nunknown = true",
        ),
    ] {
        let dir = fixture();
        let path = dir.path().join("workflows/bugfix.toml");
        let source = fs::read_to_string(&path).unwrap();
        assert!(source.contains(old));
        fs::write(path, source.replace(old, new)).unwrap();
        assert!(catalog::entries(dir.path()).is_err(), "{new}");
    }
}

#[test]
fn catalog_refuses_symlinks_hardlinks_and_oversized_inputs() {
    let dir = fixture();
    let method = dir.path().join("workflows/methods/diagnose.md");
    fs::remove_file(&method).unwrap();
    symlink("plan.md", &method).unwrap();
    assert!(catalog::entries(dir.path()).is_err());
    fs::remove_file(&method).unwrap();
    fs::hard_link(dir.path().join("workflows/methods/plan.md"), &method).unwrap();
    assert!(catalog::entries(dir.path()).is_err());
    fs::remove_file(&method).unwrap();
    fs::write(&method, vec![b'x'; 65537]).unwrap();
    assert!(catalog::entries(dir.path()).is_err());
}

#[test]
fn selected_scope_is_immutable_and_legacy_event_bytes_survive() {
    let bytes = include_bytes!("fixtures/legacy-event.json");
    let legacy: Event = serde_json::from_slice(bytes).unwrap();
    assert_eq!(legacy.run.workflow_id(), "delivery");
    assert_eq!(legacy.run.workflow_path(), "workflows/delivery.toml");
    assert_eq!(serde_json::to_vec(&legacy).unwrap(), bytes);
    let mut changed = legacy.clone();
    changed.kind = "checkpointed".into();
    changed.previous = digest(bytes);
    changed.run.revision += 1;
    changed.run.selection = Some(WorkflowSelection {
        id: "bugfix".into(),
        path: "workflows/bugfix.toml".into(),
    });
    assert!(validate_transition(Some(&legacy.run), &changed).is_err());
    let before = changed.run.clone();
    changed.run.selection.as_mut().unwrap().path = "workflows/delivery.toml".into();
    changed.run.revision += 1;
    assert!(validate_transition(Some(&before), &changed).is_err());
}

#[tokio::test]
async fn prime_recall_retains_workflow_and_source_attribution() {
    let dir = tempfile::tempdir().unwrap();
    let memory = Memory::open(dir.path()).await.unwrap();
    memory
        .record(
            "legacy",
            "delivery",
            "plan",
            "docs/plans/legacy.md",
            "legacy reproduction",
        )
        .await
        .unwrap();
    memory
        .record(
            "repair",
            "bugfix",
            "shipped",
            "https://github.com/example/project/pull/1",
            "reproduction repaired",
        )
        .await
        .unwrap();
    memory.shutdown().await.unwrap();
    let memory = Memory::open(dir.path()).await.unwrap();
    let recall = memory.recall("reproduction");
    let nodes = recall["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 2);
    let repair = nodes
        .iter()
        .find(|n| n["properties"]["run_id"] == "repair")
        .unwrap();
    assert_eq!(repair["properties"]["workflow_id"], "bugfix");
    assert_eq!(
        repair["properties"]["source"],
        "https://github.com/example/project/pull/1"
    );
    memory.shutdown().await.unwrap();
}
