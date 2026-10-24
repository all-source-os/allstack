//! Legacy and malformed nodes cannot be produced through `Memory::record`
//! (it always writes a complete binding), so they are seeded through Prime's
//! own API, the writer is shut down, and `Memory` reopens the same data dir.

use allsource_core::prime::Prime;
use allstack::infrastructure::memory::{Memory, binding_matches, single_complete_binding};
use serde_json::{Value, json};
use std::path::Path;

const KEY: &str = "run:shipped";
const SRC: &str = "https://github.com/example/project/pull/1";
const SUM: &str = "legacy reproduction";

fn legacy(run: &str, source: &str, summary: &str) -> Value {
    json!({
        "name": format!("shipped {run}"),
        "domain": "allstack",
        "key": KEY,
        "source": source,
        "summary": summary,
        "run_id": run,
    })
}

fn with_workflow(run: &str, workflow: Value, source: &str, summary: &str) -> Value {
    let mut props = legacy(run, source, summary);
    props["workflow_id"] = workflow;
    props
}

async fn seed(dir: &Path, nodes: &[Value]) {
    let prime = Prime::open(dir).await.unwrap();
    for props in nodes {
        prime.add_node("delivery", props.clone()).await.unwrap();
    }
    prime.shutdown().await.unwrap();
}

async fn delivery_properties(dir: &Path) -> Vec<Value> {
    let prime = Prime::open(dir).await.unwrap();
    let props = prime
        .nodes_by_type("delivery")
        .into_iter()
        .map(|node| node.properties)
        .collect();
    prime.shutdown().await.unwrap();
    props
}

#[tokio::test]
async fn legacy_delivery_node_reconciles_only_as_delivery() {
    let dir = tempfile::tempdir().unwrap();
    seed(dir.path(), &[legacy("run", SRC, SUM)]).await;

    let memory = Memory::open(dir.path()).await.unwrap();
    memory
        .record("run", "delivery", "shipped", SRC, SUM)
        .await
        .expect("identical delivery retry matches a workflow-less legacy node");
    assert!(
        memory
            .record("run", "bugfix", "shipped", SRC, SUM)
            .await
            .is_err(),
        "legacy delivery node must not reconcile as bugfix"
    );
    assert!(
        memory
            .record("run", "delivery", "shipped", "https://example/pull/2", SUM)
            .await
            .is_err(),
        "changed source must conflict against a legacy node"
    );
    assert!(
        memory
            .record("run", "delivery", "shipped", SRC, "different result")
            .await
            .is_err(),
        "changed summary must conflict against a legacy node"
    );
    let recall = memory.recall("legacy reproduction");
    assert_eq!(recall["nodes"].as_array().unwrap().len(), 1);
    memory.shutdown().await.unwrap();

    let after = delivery_properties(dir.path()).await;
    assert_eq!(after.len(), 1, "no node added or removed");
    assert!(
        after[0].get("workflow_id").is_none(),
        "legacy node keeps its missing workflow_id"
    );
    assert_eq!(after[0]["source"], SRC);
    assert_eq!(after[0]["summary"], SUM);
}

#[tokio::test]
async fn matching_key_with_wrong_or_missing_run_id_fails() {
    for seeded in [with_workflow("other", json!("delivery"), SRC, SUM), {
        let mut props = with_workflow("run", json!("delivery"), SRC, SUM);
        props.as_object_mut().unwrap().remove("run_id");
        props
    }] {
        let dir = tempfile::tempdir().unwrap();
        seed(dir.path(), &[seeded]).await;
        let memory = Memory::open(dir.path()).await.unwrap();
        assert!(
            memory
                .record("run", "delivery", "shipped", SRC, SUM)
                .await
                .is_err(),
            "a key match with a different or absent run_id must not reconcile"
        );
        memory.shutdown().await.unwrap();
        assert_eq!(delivery_properties(dir.path()).await.len(), 1);
    }
}

#[tokio::test]
async fn duplicate_keys_fail_without_mutation_in_any_order() {
    let identical = with_workflow("run", json!("bugfix"), SRC, SUM);
    for pair in [
        vec![identical.clone(), identical.clone()],
        vec![
            with_workflow("run", json!("bugfix"), SRC, SUM),
            with_workflow("run", json!("delivery"), SRC, SUM),
        ],
        vec![
            with_workflow("run", json!("delivery"), SRC, SUM),
            with_workflow("run", json!("bugfix"), SRC, SUM),
        ],
    ] {
        let dir = tempfile::tempdir().unwrap();
        seed(dir.path(), &pair).await;
        let memory = Memory::open(dir.path()).await.unwrap();
        assert!(
            memory
                .record("run", "bugfix", "shipped", SRC, SUM)
                .await
                .is_err(),
            "duplicate key nodes require reconciliation, never an arbitrary pick"
        );
        memory.shutdown().await.unwrap();
        assert_eq!(
            delivery_properties(dir.path()).await.len(),
            2,
            "a conflicting retry over duplicates adds and removes nothing"
        );
    }
}

#[tokio::test]
async fn invalid_workflow_metadata_is_not_given_the_legacy_fallback() {
    for malformed in [Value::Null, json!(7)] {
        let dir = tempfile::tempdir().unwrap();
        seed(dir.path(), &[with_workflow("run", malformed, SRC, SUM)]).await;
        let memory = Memory::open(dir.path()).await.unwrap();
        assert!(
            memory
                .record("run", "delivery", "shipped", SRC, SUM)
                .await
                .is_err(),
            "an explicit-null or non-string workflow_id binds no workflow"
        );
        memory.shutdown().await.unwrap();
        assert_eq!(delivery_properties(dir.path()).await.len(), 1);
    }
}

#[tokio::test]
async fn matching_retry_across_reopen_keeps_one_unchanged_record() {
    let dir = tempfile::tempdir().unwrap();
    let memory = Memory::open(dir.path()).await.unwrap();
    memory
        .record("run", "bugfix", "shipped", SRC, "repair verified")
        .await
        .unwrap();
    memory.shutdown().await.unwrap();

    let memory = Memory::open(dir.path()).await.unwrap();
    memory
        .record("run", "bugfix", "shipped", SRC, "repair verified")
        .await
        .expect("an identical retry after reopen stays idempotent");
    memory.shutdown().await.unwrap();

    let after = delivery_properties(dir.path()).await;
    assert_eq!(after.len(), 1);
    assert_eq!(after[0]["workflow_id"], "bugfix");
    assert_eq!(after[0]["source"], SRC);
    assert_eq!(after[0]["summary"], "repair verified");
}

#[test]
fn readback_seam_rejects_missing_mismatch_and_duplicate() {
    let good = with_workflow("run", json!("bugfix"), SRC, SUM);
    assert!(single_complete_binding(
        std::slice::from_ref(&good),
        KEY,
        "run",
        "bugfix",
        SRC,
        SUM
    ));
    assert!(
        !single_complete_binding(&[], KEY, "run", "bugfix", SRC, SUM),
        "a missing node never proves a binding"
    );
    let mismatched = with_workflow("run", json!("bugfix"), SRC, "different result");
    assert!(
        !single_complete_binding(
            std::slice::from_ref(&mismatched),
            KEY,
            "run",
            "bugfix",
            SRC,
            SUM
        ),
        "a node that differs in one field never proves a binding"
    );
    assert!(
        !single_complete_binding(&[good.clone(), good], KEY, "run", "bugfix", SRC, SUM),
        "duplicate nodes never prove a binding"
    );
}

#[test]
fn binding_match_workflow_fallback_is_delivery_only() {
    assert!(binding_matches(
        &legacy("run", SRC, SUM),
        KEY,
        "run",
        "delivery",
        SRC,
        SUM
    ));
    assert!(!binding_matches(
        &legacy("run", SRC, SUM),
        KEY,
        "run",
        "bugfix",
        SRC,
        SUM
    ));
    for malformed in [Value::Null, json!(7)] {
        assert!(
            !binding_matches(
                &with_workflow("run", malformed, SRC, SUM),
                KEY,
                "run",
                "delivery",
                SRC,
                SUM
            ),
            "malformed workflow_id never receives the legacy delivery fallback"
        );
    }
    assert!(
        !binding_matches(
            &with_workflow("other", json!("delivery"), SRC, SUM),
            KEY,
            "run",
            "delivery",
            SRC,
            SUM
        ),
        "run identity is part of the immutable binding"
    );
}
