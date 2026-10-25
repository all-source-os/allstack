mod common;
use allstack::infrastructure::{bootstrap, knowledge, skills};
use std::{fs, process::Command};

fn repository() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    assert!(
        Command::new("git")
            .args(["init", "--quiet"])
            .current_dir(dir.path())
            .status()
            .unwrap()
            .success()
    );
    dir
}

fn cli(repo: &std::path::Path, args: &[&str]) -> serde_json::Value {
    let timeout = if cfg!(target_os = "macos") {
        "gtimeout"
    } else {
        "timeout"
    };
    let output = Command::new(timeout)
        .args(["-k", "5", "25", env!("CARGO_BIN_EXE_allstack"), "-C"])
        .arg(repo)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn installs_native_hosts_without_project_profile_or_stores_and_preserves_conflicts() {
    let dir = repository();
    let preview = bootstrap::install_skills(dir.path(), true).unwrap();
    assert_eq!(preview["ready"], true);
    assert!(!dir.path().join(".agents").exists());
    bootstrap::install_skills(dir.path(), false).unwrap();
    for host in [".agents", ".claude"] {
        for id in ["shape", "plan", "review", "qa", "ship", "recall", "handoff"] {
            let path = dir
                .path()
                .join(format!("{host}/skills/allstack-{id}/SKILL.md"));
            let text = fs::read_to_string(path).unwrap();
            assert!(text.starts_with(skills::method(id).unwrap()));
        }
    }
    for store in [".allstack", ".chronis", ".prime"] {
        assert!(!dir.path().join(store).exists());
    }
    let again = bootstrap::install_skills(dir.path(), true).unwrap();
    assert!(
        again["files"]
            .as_array()
            .unwrap()
            .iter()
            .all(|f| f["status"] == "unchanged")
    );
    let path = dir.path().join(".agents/skills/allstack-review/SKILL.md");
    fs::write(&path, "custom review").unwrap();
    fs::remove_file(dir.path().join(".claude/skills/allstack-qa/SKILL.md")).unwrap();
    assert!(bootstrap::install_skills(dir.path(), false).is_err());
    assert_eq!(fs::read_to_string(path).unwrap(), "custom review");
    assert!(
        !dir.path()
            .join(".claude/skills/allstack-qa/SKILL.md")
            .exists()
    );
}

#[tokio::test]
async fn independent_knowledge_survives_reopen_with_source_digest_and_no_delivery_state() {
    let dir = repository();
    bootstrap::install_skills(dir.path(), false).unwrap();
    fs::create_dir_all(dir.path().join("docs/plans")).unwrap();
    fs::write(
        dir.path().join("docs/plans/choice.md"),
        "Accepted: exact money units.",
    )
    .unwrap();
    let args = [
        "remember",
        "--kind",
        "decision",
        "--source",
        "docs/plans/choice.md",
        "--summary",
        "Use integer minor units for expenses.",
    ];
    let first = cli(dir.path(), &args);
    let again = cli(dir.path(), &args);
    assert!(first["node"]["id"].is_string());
    assert_eq!(first["node"]["id"], again["node"]["id"]);
    let recall = knowledge::recall(dir.path(), "integer expenses")
        .await
        .unwrap();
    assert_eq!(recall["nodes"].as_array().unwrap().len(), 1);
    assert_eq!(
        recall["nodes"][0]["properties"]["source_digest"],
        allstack::infrastructure::digest(b"Accepted: exact money units.")
    );
    assert!(!dir.path().join(".allstack/events").exists());
    assert!(!dir.path().join(".chronis").exists());
    assert!(
        knowledge::remember(dir.path(), "decision", "Cargo.toml", "invalid")
            .await
            .is_err()
    );
}

#[tokio::test]
async fn active_discovery_folds_history_and_refuses_ambiguous_runs() {
    let dir = tempfile::tempdir().unwrap();
    let store = allstack::infrastructure::store::Store::open(dir.path())
        .await
        .unwrap();
    assert!(store.active().await.unwrap().is_none());
    let mut run = common::run();
    store.save(&mut run, "started").await.unwrap();
    assert_eq!(store.active().await.unwrap().unwrap().id, run.id);
    run.pending = Some("task_setup".into());
    store.save(&mut run, "intent").await.unwrap();
    assert_eq!(
        store.active().await.unwrap().unwrap().pending.as_deref(),
        Some("task_setup")
    );
    let mut other = common::run();
    store.save(&mut other, "started").await.unwrap();
    assert!(
        store
            .active()
            .await
            .unwrap_err()
            .to_string()
            .contains("multiple active")
    );
    store.shutdown().await.unwrap();
}

#[tokio::test]
async fn empty_recall_and_discovery_leave_repository_untouched() {
    let outside_git = tempfile::tempdir().unwrap();
    assert_eq!(
        cli(outside_git.path(), &["skills"])["skills"]
            .as_array()
            .unwrap()
            .len(),
        7
    );
    let dir = repository();
    assert_eq!(
        skills::inspect(None).unwrap()["skills"]
            .as_array()
            .unwrap()
            .len(),
        7
    );
    assert!(skills::inspect(Some("invented")).is_err());
    assert_eq!(
        knowledge::recall(dir.path(), "architecture").await.unwrap()["nodes"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert!(!dir.path().join(".allstack").exists());
}
