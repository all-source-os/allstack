#![allow(dead_code)]
use allstack::domain::{Host, Run, Stage};

pub fn run() -> Run {
    Run {
        id: uuid::Uuid::new_v4().to_string(),
        repository: "/fixture".into(),
        remote: "https://github.com/example/project.git".into(),
        branch: "feature".into(),
        base: "base".into(),
        trunk: "main".into(),
        workflow: "workflow".into(),
        methods: [
            ("plan".into(), "plan-method".into()),
            ("build".into(), "build-method".into()),
        ]
        .into(),
        method_paths: [
            ("plan".into(), "workflows/methods/plan.md".into()),
            ("build".into(), "workflows/methods/build.md".into()),
        ]
        .into(),
        policy: "policy".into(),
        policy_path: "policy.toml".into(),
        request: "request".into(),
        plan: None,
        task: Some("task-reference".into()),
        host: Host::Codex,
        epoch: 1,
        stage: Stage::Planning,
        workspace: "/workspace-1".into(),
        checkpoint: "snapshot".into(),
        pending: None,
        transfer: None,
        memory_pending: false,
        prepared_commit: None,
        checkout_snapshot: None,
        verification: None,
        publication: None,
        allow_commit: true,
        allow_publish: true,
        revision: 0,
    }
}
