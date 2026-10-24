mod common;
use allstack::domain::{Event, Host, Policy, Stage, validate_transition};

#[test]
fn stale_host_and_unknown_action_cannot_execute() {
    let mut run = common::run();
    run.epoch = 2;
    assert!(run.check_epoch(1).unwrap_err().contains("stale host"));
    run.pending = Some("publication".into());
    assert!(run.require_resolved().is_err());
}

#[test]
fn event_sequence_and_scope_are_immutable() {
    let mut run = common::run();
    run.revision = 1;
    let event = Event {
        schema: 1,
        operation: "one".into(),
        previous: String::new(),
        kind: "started".into(),
        run: run.clone(),
    };
    validate_transition(None, &event).unwrap();
    let mut next = event.clone();
    next.run.revision = 3;
    assert!(validate_transition(Some(&run), &next).is_err());
    next.run.revision = 2;
    next.run.remote = "https://github.com/other/project.git".into();
    assert!(validate_transition(Some(&run), &next).is_err());
}

#[test]
fn cannot_skip_verification_or_invent_delivery() {
    let mut old = common::run();
    old.revision = 1;
    old.stage = Stage::Building;
    let mut next = old.clone();
    next.revision = 2;
    next.stage = Stage::Shipped;
    let event = Event {
        schema: 1,
        operation: "two".into(),
        previous: "hash".into(),
        kind: "shipped".into(),
        run: next,
    };
    assert!(validate_transition(Some(&old), &event).is_err());
}

#[test]
fn policy_requires_execution_evidence_and_bounded_commands() {
    let mut p: Policy = toml::from_str(
        "command_seconds=301\n[[gates]]\nname='test'\nargv=['cargo','test']\ntest=true\n",
    )
    .unwrap();
    assert!(p.validate().is_err());
    p.command_seconds = 30;
    assert!(p.validate().is_err());
    p.gates[0].success_marker = Some("1 passed".into());
    p.validate().unwrap();
    assert_eq!("claude-code".parse::<Host>().unwrap(), Host::ClaudeCode);
}

#[test]
fn snapshot_paths_cannot_escape_or_traverse_symlinks() {
    let dir = tempfile::tempdir().unwrap();
    assert!(allstack::infrastructure::workspace::contained(dir.path(), "../outside").is_err());
    assert!(allstack::infrastructure::workspace::contained(dir.path(), ".git/config").is_err());
    std::os::unix::fs::symlink("/tmp", dir.path().join("link")).unwrap();
    assert!(allstack::infrastructure::workspace::contained(dir.path(), "link/secret").is_err());
}
