mod common;
use allstack::{
    application::{
        self,
        ports::{Platform, Start},
    },
    domain::{Host, Policy, Publication, Run, Verification},
};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::cell::{Cell, RefCell};

struct Fake {
    state: RefCell<Run>,
    calls: RefCell<Vec<String>>,
    fail_claim: Cell<bool>,
    fail_completion: Cell<bool>,
    fail_memory: Cell<bool>,
    fail_readback: Cell<bool>,
    published: Cell<bool>,
    candidate: RefCell<String>,
}

impl Fake {
    fn new() -> Self {
        Self {
            state: RefCell::new(common::run()),
            calls: RefCell::new(vec![]),
            fail_claim: Cell::new(false),
            fail_completion: Cell::new(false),
            fail_memory: Cell::new(false),
            fail_readback: Cell::new(false),
            published: Cell::new(false),
            candidate: RefCell::new("commit-one".into()),
        }
    }
}

impl Platform for Fake {
    async fn load(&self, _: &str) -> Result<Run> {
        Ok(self.state.borrow().clone())
    }
    async fn save(&self, run: &mut Run, kind: &str) -> Result<()> {
        run.revision += 1;
        *self.state.borrow_mut() = run.clone();
        self.calls.borrow_mut().push(kind.into());
        Ok(())
    }
    fn initialize(&self, input: &Start) -> Result<Run> {
        let mut run = common::run();
        if input.workflow == "bugfix" {
            run.selection = Some(allstack::domain::WorkflowSelection {
                id: "bugfix".into(),
                path: "workflows/bugfix.toml".into(),
            });
            run.methods = [
                ("plan".into(), "diagnose".into()),
                ("build".into(), "repair".into()),
            ]
            .into();
        }
        Ok(run)
    }
    fn object(&self, key: &str) -> Result<Vec<u8>> {
        Ok(if key == "policy" {
            b"command_seconds=10\n[[gates]]\nname='real tests'\nargv=['cargo','test','--lib']\ntest=true\nsuccess_marker='1 passed'\n".to_vec()
        } else if key == "diagnose" || key == "repair" {
            key.as_bytes().to_vec()
        } else {
            b"request and plan".to_vec()
        })
    }
    fn put(&self, _: &[u8]) -> Result<String> {
        Ok("plan".into())
    }
    fn checkpoint(&self, _: &Run) -> Result<String> {
        Ok("snapshot".into())
    }
    fn restore(&self, _: &Run, _: &str, epoch: u64) -> Result<String> {
        Ok(format!("/workspace-{epoch}"))
    }
    fn create_task(&self, _: &Run) -> Result<String> {
        Ok("task-reference".into())
    }
    fn claim(&self, _: &Run, _: Option<&Run>) -> Result<()> {
        ensure!(
            !self.fail_claim.replace(false),
            "simulated lost task acknowledgement"
        );
        Ok(())
    }
    fn claim_readback(&self, _: &Run) -> Result<()> {
        ensure!(!self.fail_readback.get(), "Chronis claimant changed");
        Ok(())
    }
    fn complete_task(&self, _: &Run) -> Result<()> {
        ensure!(
            !self.fail_completion.replace(false),
            "task store unavailable"
        );
        Ok(())
    }
    fn recall(&self, _: &str) -> Value {
        json!({"authority":"context-only"})
    }
    async fn remember(&self, run: &Run, _: &str, source: &str, _: &str) -> Result<()> {
        ensure!(!self.fail_memory.replace(false), "memory unavailable");
        self.calls
            .borrow_mut()
            .push(format!("remember:{}:{source}", run.workflow_id()));
        Ok(())
    }
    fn invoke(&self, run: &Run, prompt: &str, planning: bool) -> Result<String> {
        self.calls
            .borrow_mut()
            .push(format!("host:{}:{planning}", run.host.name()));
        self.calls
            .borrow_mut()
            .push(format!("prompt:{}", prompt.lines().next().unwrap()));
        Ok("plan or completed build".into())
    }
    fn write_plan(&self, _: &Run, _: &str) -> Result<String> {
        Ok("docs/plans/plan.md".into())
    }
    fn prepare(&self, _: &Run, _: &str) -> Result<String> {
        Ok(self.candidate.borrow().clone())
    }
    fn candidate(&self, _: &Run) -> Result<Verification> {
        Ok(Verification {
            commit: self.candidate.borrow().clone(),
            snapshot: "snapshot".into(),
            policy: "policy".into(),
            workflow: "workflow".into(),
            environment: "environment".into(),
            logs: vec![],
        })
    }
    fn gates(&self, _: &Run, _: &Policy) -> Result<Vec<String>> {
        Ok(vec!["executed-receipt".into()])
    }
    fn publish(&self, run: &Run, _: &str) -> Result<Publication> {
        if !self.published.replace(true) {
            self.calls.borrow_mut().push("push-once".into());
        }
        Ok(Publication {
            commit: self.candidate.borrow().clone(),
            remote: run.remote.clone(),
            branch: run.branch.clone(),
            pr: "https://github.com/example/project/pull/1".into(),
            task_completed: false,
            memory_indexed: false,
        })
    }
    async fn cleanup(&self, _: &Run) -> Result<()> {
        self.calls.borrow_mut().push("cleanup".into());
        Ok(())
    }
}

async fn build_flow(p: &Fake) -> Run {
    let run = application::start(
        p,
        Start {
            workflow: "delivery".into(),
            request: "request".into(),
            policy: "policy".into(),
            host: Host::Codex,
            allow_commit: true,
            allow_publish: true,
        },
    )
    .await
    .unwrap();
    application::plan(p, &run.id, 1).await.unwrap();
    application::handoff(p, &run.id, 1, Host::ClaudeCode)
        .await
        .unwrap();
    application::build(p, &run.id, 2).await.unwrap();
    application::handoff(p, &run.id, 2, Host::Codex)
        .await
        .unwrap()
}

#[tokio::test]
async fn full_flow_preserves_host_sequence_and_reconciles_after_publication() {
    let p = Fake::new();
    let run = build_flow(&p).await;
    application::verify(&p, &run.id, 3, "feat(example): implement accepted change")
        .await
        .unwrap();
    p.fail_completion.set(true);
    assert!(
        application::ship(&p, &run.id, 3, "feat(example): implement accepted change")
            .await
            .is_err()
    );
    let shipped = application::ship(&p, &run.id, 3, "feat(example): implement accepted change")
        .await
        .unwrap();
    assert_eq!(shipped.stage, allstack::domain::Stage::Shipped);
    assert!(shipped.publication.unwrap().memory_indexed);
    assert_eq!(
        p.calls
            .borrow()
            .iter()
            .filter(|s| *s == "push-once")
            .count(),
        1
    );
    assert!(p.calls.borrow().contains(&"host:codex:true".into()));
    assert!(p.calls.borrow().contains(&"host:claude-code:false".into()));
}

#[tokio::test]
async fn failed_task_transfer_can_resume_without_reinvoking_host() {
    let p = Fake::new();
    let run = application::start(
        &p,
        Start {
            workflow: "delivery".into(),
            request: "request".into(),
            policy: "policy".into(),
            host: Host::Codex,
            allow_commit: true,
            allow_publish: true,
        },
    )
    .await
    .unwrap();
    application::plan(&p, &run.id, 1).await.unwrap();
    p.fail_claim.set(true);
    assert!(
        application::handoff(&p, &run.id, 1, Host::ClaudeCode)
            .await
            .is_err()
    );
    let recovered = application::recover(&p, &run.id, 2).await.unwrap();
    assert_eq!(recovered.host, Host::ClaudeCode);
    assert!(recovered.pending.is_none());
    assert!(application::build(&p, &run.id, 1).await.is_err());
    assert_eq!(
        p.calls
            .borrow()
            .iter()
            .filter(|s| s.starts_with("host:"))
            .count(),
        1
    );
}

#[tokio::test]
async fn changed_candidate_cannot_reuse_verification() {
    let p = Fake::new();
    let run = build_flow(&p).await;
    application::verify(&p, &run.id, 3, "feat(example): accepted change")
        .await
        .unwrap();
    *p.candidate.borrow_mut() = "different-commit".into();
    assert!(
        application::ship(&p, &run.id, 3, "feat(example): accepted change")
            .await
            .is_err()
    );
    assert!(!p.published.get());
}

#[tokio::test]
async fn memory_outage_keeps_plan_authoritative_and_retries_indexing() {
    let p = Fake::new();
    p.fail_memory.set(true);
    let run = application::plan(&p, &common::run().id, 1).await.unwrap();
    assert!(run.memory_pending);
    assert!(run.plan.is_some());
    assert_eq!(run.stage, allstack::domain::Stage::Building);
    let recovered = application::recover(&p, &run.id, 1).await.unwrap();
    assert!(!recovered.memory_pending);
    assert_eq!(
        p.calls
            .borrow()
            .iter()
            .filter(|s| s.starts_with("host:"))
            .count(),
        1
    );
}

#[tokio::test]
async fn reverification_replaces_receipt_and_cleanup_requires_current_epoch() {
    let p = Fake::new();
    let run = build_flow(&p).await;
    let verified = application::verify(&p, &run.id, 3, "feat(example): accepted change")
        .await
        .unwrap();
    assert_eq!(verified.prepared_commit.as_deref(), Some("commit-one"));
    application::checkpoint(&p, &run.id, 3).await.unwrap();
    *p.candidate.borrow_mut() = "commit-two".into();
    application::verify(&p, &run.id, 3, "feat(example): changed candidate")
        .await
        .unwrap();
    let shipped = application::ship(&p, &run.id, 3, "feat(example): changed candidate")
        .await
        .unwrap();
    assert_eq!(shipped.verification.unwrap().commit, "commit-two");
    assert!(application::ship(&p, &run.id, 2, "stale").await.is_err());
    application::ship(&p, &run.id, 3, "idempotent")
        .await
        .unwrap();
    assert_eq!(
        p.calls
            .borrow()
            .iter()
            .filter(|s| s.as_str() == "cleanup")
            .count(),
        2
    );
}

#[tokio::test]
async fn failed_commit_can_accept_explicit_checkpoint_repair() {
    let p = Fake::new();
    let run = build_flow(&p).await;
    p.state.borrow_mut().pending = Some("commit".into());
    let repaired = application::checkpoint(&p, &run.id, 3).await.unwrap();
    assert!(repaired.pending.is_none());
    assert_eq!(repaired.checkout_snapshot.as_deref(), Some("snapshot"));
    application::verify(&p, &run.id, 3, "feat(example): repair candidate")
        .await
        .unwrap();
    assert!(p.state.borrow().verification.is_some());
}

#[tokio::test]
async fn changed_task_claim_blocks_first_publication() {
    let p = Fake::new();
    let run = build_flow(&p).await;
    application::verify(&p, &run.id, 3, "feat(example): accepted change")
        .await
        .unwrap();
    p.fail_readback.set(true);
    assert!(
        application::ship(&p, &run.id, 3, "feat(example): accepted change")
            .await
            .is_err()
    );
    assert!(!p.published.get());
}

#[tokio::test]
async fn selected_bugfix_methods_follow_handoff_through_verified_publication() {
    let p = Fake::new();
    let run = application::start(
        &p,
        Start {
            workflow: "bugfix".into(),
            request: "request".into(),
            policy: "policy".into(),
            host: Host::Codex,
            allow_commit: true,
            allow_publish: true,
        },
    )
    .await
    .unwrap();
    application::plan(&p, &run.id, 1).await.unwrap();
    application::handoff(&p, &run.id, 1, Host::ClaudeCode)
        .await
        .unwrap();
    application::build(&p, &run.id, 2).await.unwrap();
    application::handoff(&p, &run.id, 2, Host::Codex)
        .await
        .unwrap();
    application::verify(&p, &run.id, 3, "fix(example): repair observed defect")
        .await
        .unwrap();
    let shipped = application::ship(&p, &run.id, 3, "fix(example): repair observed defect")
        .await
        .unwrap();
    assert_eq!(shipped.workflow_id(), "bugfix");
    assert_eq!(shipped.stage, allstack::domain::Stage::Shipped);
    let publication = shipped.publication.unwrap();
    assert!(publication.task_completed && publication.memory_indexed);
    let calls = p.calls.borrow();
    assert!(calls.contains(&"prompt:diagnose".into()));
    assert!(calls.contains(&"prompt:repair".into()));
    assert!(calls.contains(&"remember:bugfix:https://github.com/example/project/pull/1".into()));
}
