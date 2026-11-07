use serde_json::Value;
use std::{fs, os::unix::fs::PermissionsExt, path::PathBuf, process::Command};

const DESIGN: &str = "docs/plans/2026-11-07-harness-design.md";
const PASS: &str = "#!/bin/sh\ncat > CAPTURE\nprintf '%s\\n' '{\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"- none\\nVERDICT: clean\"}}' '{\"type\":\"turn.completed\"}'\n";
const FAIL: &str = "#!/bin/sh\ncat >/dev/null\nexit 3\n";
const NO_VERDICT: &str = "#!/bin/sh\ncat > CAPTURE\nprintf '%s\\n' '{\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"- gap\"}}' '{\"type\":\"turn.completed\"}'\n";
/// Reports `VERDICT: findings` while the `CAPTURE.findings` marker file exists, `clean` otherwise.
const MIXED: &str = r#"#!/bin/sh
cat > CAPTURE
if [ -f CAPTURE.findings ]; then verdict=findings; else verdict=clean; fi
printf '%s\n' "{\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"- gap\\nVERDICT: $verdict\"}}" '{"type":"turn.completed"}'
"#;

struct Fixture {
    tmp: tempfile::TempDir,
    repo: PathBuf,
    path: String,
}

impl Fixture {
    /// The fake `codex` executable replaces the real host on PATH; it captures the review prompt.
    fn new(host_script: &str) -> Self {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path().join("repository");
        fs::create_dir(&repo).unwrap();
        assert!(
            Command::new("git")
                .args(["init", "--quiet"])
                .current_dir(&repo)
                .status()
                .unwrap()
                .success()
        );
        fs::create_dir_all(repo.join("docs/plans")).unwrap();
        fs::write(repo.join(DESIGN), "# Design\n\nFirst draft.\n").unwrap();
        let bin = tmp.path().join("bin");
        fs::create_dir(&bin).unwrap();
        let capture = tmp.path().join("prompt.txt");
        let script = host_script.replace("CAPTURE", &capture.display().to_string());
        let codex = bin.join("codex");
        fs::write(&codex, script).unwrap();
        fs::set_permissions(&codex, fs::Permissions::from_mode(0o755)).unwrap();
        let path = format!("{}:{}", bin.display(), std::env::var("PATH").unwrap());
        Self { tmp, repo, path }
    }

    fn captured_prompt(&self) -> String {
        fs::read_to_string(self.tmp.path().join("prompt.txt")).unwrap()
    }

    fn run(&self, args: &[&str]) -> (bool, String, String) {
        let timeout = if cfg!(target_os = "macos") {
            "gtimeout"
        } else {
            "timeout"
        };
        let output = Command::new(timeout)
            .args(["-k", "5", "90", env!("CARGO_BIN_EXE_allstack"), "-C"])
            .arg(&self.repo)
            .args(args)
            .env("PATH", &self.path)
            .output()
            .unwrap();
        (
            output.status.success(),
            String::from_utf8_lossy(&output.stdout).into_owned(),
            String::from_utf8_lossy(&output.stderr).into_owned(),
        )
    }

    fn json(&self, args: &[&str]) -> Value {
        let (ok, out, err) = self.run(args);
        assert!(ok, "{err}");
        serde_json::from_str(&out).unwrap()
    }
}

fn receipt_of(review: &Value) -> PathBuf {
    let receipt = PathBuf::from(review["receipt"].as_str().unwrap());
    assert!(receipt.is_file(), "{}", receipt.display());
    receipt
}

#[test]
fn review_runs_fresh_host_and_approval_binds_the_digest() {
    let f = Fixture::new(PASS);
    let review = f.json(&["spec", "review", DESIGN, "--host", "codex"]);
    assert_eq!(review["verdict"], "clean");
    receipt_of(&review);
    let prompt = f.captured_prompt();
    assert!(prompt.contains(DESIGN), "{prompt}");
    assert!(prompt.contains("no prior conversation"), "{prompt}");
    assert!(prompt.len() < 2000, "reviewer prompt carries extra context");

    let approved = f.json(&["spec", "approve", DESIGN, "--by", "reviewer"]);
    assert_eq!(approved["without_review"], false);
    assert_eq!(f.json(&["spec", "check", DESIGN])["approved"], true);

    fs::write(f.repo.join(DESIGN), "# Design\n\nSecond draft.\n").unwrap();
    let (ok, _, err) = f.run(&["spec", "check", DESIGN]);
    assert!(!ok && err.contains("changed since approval"), "{err}");
}

#[test]
fn approval_without_review_is_refused_unless_overridden() {
    let f = Fixture::new(PASS);
    let (ok, _, err) = f.run(&["spec", "approve", DESIGN, "--by", "reviewer"]);
    assert!(!ok && err.contains("no clean review receipt"), "{err}");
    let (ok, _, err) = f.run(&["spec", "check", DESIGN]);
    assert!(!ok && err.contains("unapproved"), "{err}");

    let approved = f.json(&[
        "spec",
        "approve",
        DESIGN,
        "--by",
        "reviewer",
        "--without-review",
    ]);
    assert_eq!(approved["without_review"], true);
    assert_eq!(f.json(&["spec", "check", DESIGN])["approved"], true);
}

#[test]
fn approval_is_unreviewed_when_its_receipt_is_removed() {
    let f = Fixture::new(PASS);
    let review = f.json(&["spec", "review", DESIGN]);
    f.json(&["spec", "approve", DESIGN, "--by", "reviewer"]);
    fs::remove_file(receipt_of(&review)).unwrap();
    let (ok, _, err) = f.run(&["spec", "check", DESIGN]);
    assert!(!ok && err.contains("unreviewed"), "{err}");
}

#[test]
fn failed_host_fails_the_review_and_writes_no_receipt() {
    let f = Fixture::new(FAIL);
    let (ok, _, err) = f.run(&["spec", "review", DESIGN]);
    assert!(!ok && err.contains("review host failed"), "{err}");
    assert!(!f.repo.join(".allstack/spec/reviews").exists());
}

#[test]
fn approval_cites_newest_clean_receipt_and_ignores_findings() {
    let f = Fixture::new(MIXED);
    f.json(&["spec", "review", DESIGN]);
    let newest_clean = f.json(&["spec", "review", DESIGN]);
    fs::write(f.tmp.path().join("prompt.txt.findings"), "").unwrap();
    let findings = f.json(&["spec", "review", DESIGN]);
    assert_eq!(findings["verdict"], "findings");
    let approved = f.json(&["spec", "approve", DESIGN, "--by", "reviewer"]);
    assert_eq!(approved["review"], newest_clean["receipt"]);
    assert_eq!(f.json(&["spec", "check", DESIGN])["approved"], true);
}

#[test]
fn findings_or_unknown_verdict_refuses_approval() {
    let f = Fixture::new(MIXED);
    fs::write(f.tmp.path().join("prompt.txt.findings"), "").unwrap();
    f.json(&["spec", "review", DESIGN]);
    let (ok, _, err) = f.run(&["spec", "approve", DESIGN, "--by", "reviewer"]);
    assert!(!ok && err.contains("no clean review receipt"), "{err}");

    let f = Fixture::new(NO_VERDICT);
    let review = f.json(&["spec", "review", DESIGN]);
    assert_eq!(review["verdict"], "unknown");
    let (ok, _, err) = f.run(&["spec", "approve", DESIGN, "--by", "reviewer"]);
    assert!(!ok && err.contains("no clean review receipt"), "{err}");
}

#[test]
fn receipt_edited_to_findings_reports_unreviewed() {
    let f = Fixture::new(PASS);
    let review = f.json(&["spec", "review", DESIGN]);
    f.json(&["spec", "approve", DESIGN, "--by", "reviewer"]);
    let receipt = receipt_of(&review);
    let edited = fs::read_to_string(&receipt)
        .unwrap()
        .replace("\"clean\"", "\"findings\"");
    fs::write(&receipt, edited).unwrap();
    let (ok, _, err) = f.run(&["spec", "check", DESIGN]);
    assert!(!ok && err.contains("unreviewed"), "{err}");
}

#[test]
fn design_path_approver_and_missing_file_are_refused() {
    let f = Fixture::new(PASS);
    for path in [
        "/abs/design.md",
        "../design.md",
        ".hidden/design.md",
        "docs/plans/design.txt",
    ] {
        let (ok, _, err) = f.run(&["spec", "check", path]);
        assert!(
            !ok && err.contains("relative Markdown path"),
            "{path}: {err}"
        );
    }
    let (ok, _, err) = f.run(&["spec", "approve", DESIGN, "--by", "  "]);
    assert!(!ok && err.contains("approver must be"), "{err}");
    let (ok, _, err) = f.run(&["spec", "check", "docs/plans/missing-design.md"]);
    assert!(!ok && err.contains("design missing"), "{err}");
}
