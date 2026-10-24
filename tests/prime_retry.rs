use allstack::infrastructure::memory::Memory;

#[tokio::test]
async fn identical_retry_keeps_one_source_linked_record() {
    let dir = tempfile::tempdir().unwrap();
    let memory = Memory::open(dir.path()).await.unwrap();
    for _ in 0..2 {
        memory
            .record(
                "run",
                "bugfix",
                "shipped",
                "https://github.com/example/project/pull/1",
                "repair verified",
            )
            .await
            .unwrap();
    }
    assert_eq!(
        memory.recall("repair")["nodes"].as_array().unwrap().len(),
        1
    );
    memory.shutdown().await.unwrap();
}

#[tokio::test]
async fn conflicting_retry_cannot_claim_another_workflow_or_source() {
    for (workflow, source, summary) in [
        (
            "delivery",
            "https://github.com/example/project/pull/1",
            "repair verified",
        ),
        (
            "bugfix",
            "https://github.com/example/project/pull/2",
            "repair verified",
        ),
        (
            "bugfix",
            "https://github.com/example/project/pull/1",
            "different result",
        ),
    ] {
        let dir = tempfile::tempdir().unwrap();
        let memory = Memory::open(dir.path()).await.unwrap();
        memory
            .record(
                "run",
                "bugfix",
                "shipped",
                "https://github.com/example/project/pull/1",
                "repair verified",
            )
            .await
            .unwrap();
        assert!(
            memory
                .record("run", workflow, "shipped", source, summary)
                .await
                .is_err(),
            "conflicting retry must fail: {workflow}, {source}, {summary}"
        );
        let recall = memory.recall("repair");
        let nodes = recall["nodes"].as_array().unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0]["properties"]["workflow_id"], "bugfix");
        assert_eq!(
            nodes[0]["properties"]["source"],
            "https://github.com/example/project/pull/1"
        );
        memory.shutdown().await.unwrap();
    }
}
