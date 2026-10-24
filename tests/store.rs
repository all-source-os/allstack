mod common;
use allsource_core::embedded::{Config, EmbeddedCore, IngestEvent};
use allstack::{
    domain::Event,
    infrastructure::{Artifacts, digest, store::Store},
};

#[tokio::test]
async fn durable_chain_survives_reopen_and_rejects_stale_writes() {
    let dir = tempfile::tempdir().unwrap();
    let mut run = common::run();
    let store = Store::open(dir.path()).await.unwrap();
    store.save(&mut run, "started").await.unwrap();
    let stale = run.clone();
    store.save(&mut run, "checkpointed").await.unwrap();
    assert!(
        store
            .save(&mut stale.clone(), "checkpointed")
            .await
            .is_err()
    );
    let expected = serde_json::to_value(&run).unwrap();
    store.shutdown().await.unwrap();
    drop(store);
    let reopened = Store::open(dir.path()).await.unwrap();
    assert_eq!(
        serde_json::to_value(reopened.load(&run.id).await.unwrap()).unwrap(),
        expected
    );
    assert_eq!(reopened.history(&run.id).await.unwrap().len(), 2);
    reopened.shutdown().await.unwrap();
}

#[tokio::test]
async fn competing_coordinator_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let first = Store::open(dir.path()).await.unwrap();
    assert!(Store::open(dir.path()).await.is_err());
    first.shutdown().await.unwrap();
}

#[tokio::test]
async fn engine_read_detects_forged_gap_in_retained_history() {
    let dir = tempfile::tempdir().unwrap();
    let mut run = common::run();
    let store = Store::open(dir.path()).await.unwrap();
    store.save(&mut run, "started").await.unwrap();
    let first = store.history(&run.id).await.unwrap().pop().unwrap();
    store.shutdown().await.unwrap();
    drop(store);
    let core = EmbeddedCore::open(
        Config::builder()
            .data_dir(dir.path().join("events"))
            .build()
            .unwrap(),
    )
    .await
    .unwrap();
    run.revision = 3;
    let event = Event {
        schema: 1,
        operation: "gap".into(),
        previous: digest(&serde_json::to_vec(&first).unwrap()),
        kind: "checkpointed".into(),
        run: run.clone(),
    };
    core.ingest(IngestEvent {
        entity_id: &format!("allstack-run-{}", run.id),
        event_type: "allstack.workflow.v1.checkpointed",
        payload: serde_json::to_value(event).unwrap(),
        metadata: None,
        tenant_id: None,
    })
    .await
    .unwrap();
    core.shutdown().await.unwrap();
    drop(core);
    let reopened = Store::open(dir.path()).await.unwrap();
    assert!(reopened.load(&run.id).await.is_err());
    reopened.shutdown().await.unwrap();
}

#[test]
fn retained_artifact_corruption_is_not_treated_as_missing_context() {
    let dir = tempfile::tempdir().unwrap();
    let artifacts = Artifacts(dir.path().join("objects"));
    let key = artifacts.put(b"durable source").unwrap();
    std::fs::write(artifacts.0.join(&key), b"changed").unwrap();
    assert!(artifacts.get(&key).is_err());
    assert!(artifacts.get("../../secret").is_err());
}
