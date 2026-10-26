use serde_json::Value;
use std::{
    fs,
    path::Path,
    process::{Command, Output, Stdio},
    time::{Duration, Instant},
};

struct Repository(tempfile::TempDir);

fn execute(mut command: Command) -> Output {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        if child.try_wait().unwrap().is_some() {
            return child.wait_with_output().unwrap();
        }
        if Instant::now() >= deadline {
            child.kill().unwrap();
            let output = child.wait_with_output().unwrap();
            panic!("test command timed out: {output:?}");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

impl Repository {
    fn new(ignored: bool) -> Self {
        let repo = Self(tempfile::tempdir().unwrap());
        let mut git = Command::new("git");
        git.args(["init", "--quiet", "--template="])
            .arg(repo.0.path());
        assert!(execute(git).status.success());
        if ignored {
            fs::write(repo.0.path().join(".gitignore"), ".allstack/\n").unwrap();
        }
        repo
    }

    fn check(&self, args: &[&str]) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_allstack"));
        command.arg("-C").arg(self.0.path()).arg("check").args(args);
        execute(command)
    }

    fn receipt(&self) -> Value {
        let checks = self.0.path().join(".allstack/checks");
        let receipts: Vec<_> = fs::read_dir(checks)
            .unwrap()
            .map(|entry| entry.unwrap().path().join("receipt.json"))
            .collect();
        assert_eq!(receipts.len(), 1);
        serde_json::from_slice(&fs::read(&receipts[0]).unwrap()).unwrap()
    }
}

fn assert_no_delivery(repo: &Repository) {
    assert!(!repo.0.path().join(".chronis").exists());
    let entries: Vec<_> = fs::read_dir(repo.0.path().join(".allstack"))
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    assert_eq!(entries, ["checks"]);
}

#[test]
fn expected_negative_exit_is_success_with_separate_raw_channels() {
    let repo = Repository::new(true);
    let output = repo.check(&[
        "--expect-exit",
        "4",
        "--",
        "/bin/sh",
        "-c",
        "printf observed; printf rejected >&2; exit 4",
    ]);
    assert!(output.status.success(), "{output:?}");
    assert!(output.stderr.is_empty());
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value, repo.receipt());
    assert_eq!(value["matched"], true);
    assert_eq!(value["outcome"]["code"], 4);
    assert_eq!(value["outcome"]["stdout"], "observed");
    assert_eq!(value["outcome"]["stderr"], "rejected");
    assert_eq!(value["executable_before"], value["executable_after"]);
    for stream in value["streams"].as_array().unwrap() {
        let bytes = fs::read(stream["path"].as_str().unwrap()).unwrap();
        assert_eq!(stream["sha256"], allstack::infrastructure::digest(&bytes));
    }
    assert_no_delivery(&repo);
}

#[test]
fn unexpected_exit_retains_failed_receipt_and_names_it() {
    let repo = Repository::new(true);
    let output = repo.check(&["--", "/bin/sh", "-c", "exit 4"]);
    assert!(!output.status.success());
    let receipt = repo.receipt();
    assert_eq!(receipt["matched"], false);
    assert_eq!(receipt["outcome"]["code"], 4);
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains(receipt["receipt"].as_str().unwrap()));
    assert!(error.contains("observed exit Some(4)"));
}

#[test]
fn deadline_terminates_child_and_cannot_match_expected_zero() {
    let repo = Repository::new(true);
    let output = repo.check(&["--seconds", "2", "--", "/bin/sleep", "10"]);
    assert!(!output.status.success());
    let receipt = repo.receipt();
    assert_eq!(receipt["outcome"]["timed_out"], true);
    assert_eq!(receipt["matched"], false);
}

#[test]
fn spawn_failure_retains_supervisor_error() {
    use std::os::unix::fs::PermissionsExt;
    let repo = Repository::new(true);
    let path = repo.0.path().join("not-executable");
    fs::write(&path, "not executable").unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
    let output = repo.check(&["--", path.to_str().unwrap()]);
    assert!(!output.status.success());
    let receipt = repo.receipt();
    assert_eq!(receipt["matched"], false);
    assert!(receipt["outcome"].is_null());
    assert!(
        receipt["streams"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s["channel"] == "supervisor-stderr" && s["bytes"].as_u64().unwrap() > 0)
    );
}

#[test]
fn missing_executable_retains_failure_without_claiming_an_exit() {
    let repo = Repository::new(true);
    let missing = repo.0.path().join("missing");
    assert!(
        !repo
            .check(&["--", missing.to_str().unwrap()])
            .status
            .success()
    );
    let value = repo.receipt();
    assert_eq!(value["matched"], false);
    assert!(value["outcome"].is_null());
    assert!(value["error"].is_string());
}

#[test]
fn input_digest_binds_the_bytes_delivered_to_stdin() {
    let repo = Repository::new(true);
    fs::write(repo.0.path().join("prompt.txt"), "native prompt\n").unwrap();
    let output = repo.check(&["--input", "prompt.txt", "--", "/bin/cat"]);
    assert!(output.status.success(), "{output:?}");
    let value = repo.receipt();
    assert_eq!(
        value["input"]["sha256"],
        allstack::infrastructure::digest(b"native prompt\n")
    );
    assert_eq!(value["outcome"]["stdout"], "native prompt\n");
    assert_no_delivery(&repo);
}

#[test]
fn unignored_evidence_is_refused_before_creating_private_files() {
    let repo = Repository::new(false);
    let output = repo.check(&["--", "/usr/bin/true"]);
    assert!(!output.status.success());
    assert!(!repo.0.path().join(".allstack").exists());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("ignore .allstack/")
    );
}

#[test]
fn relative_program_and_excessive_deadline_are_rejected() {
    let repo = Repository::new(true);
    assert!(!repo.check(&["--", "true"]).status.success());
    assert!(
        !repo
            .check(&["--seconds", "271", "--", "/usr/bin/true"])
            .status
            .success()
    );
    assert!(!repo.0.path().join(".allstack").exists());
}

#[test]
fn signal_cannot_match_a_numeric_expected_exit() {
    let repo = Repository::new(true);
    assert!(
        !repo
            .check(&["--", "/bin/sh", "-c", "kill -TERM $$"])
            .status
            .success()
    );
    let value = repo.receipt();
    assert_eq!(value["matched"], false);
    assert!(value["outcome"]["code"].is_null());
}

#[test]
fn changed_executable_invalidates_matching_exit() {
    use std::os::unix::fs::PermissionsExt;
    let repo = Repository::new(true);
    let program = repo.0.path().join("mutable-program");
    fs::write(
        &program,
        "#!/bin/sh\nprintf '\\n# changed\\n' >> \"$0\"\nexit 0\n",
    )
    .unwrap();
    fs::set_permissions(&program, fs::Permissions::from_mode(0o700)).unwrap();
    let output = repo.check(&["--", program.to_str().unwrap()]);
    assert!(!output.status.success());
    let value = repo.receipt();
    assert_eq!(value["outcome"]["code"], 0);
    assert_eq!(value["matched"], false);
    assert_ne!(value["executable_before"], value["executable_after"]);
}

#[test]
fn source_identity_records_the_actual_checkout() {
    let repo = Repository::new(true);
    let output = repo.check(&["--", "/usr/bin/true"]);
    assert!(output.status.success(), "{output:?}");
    let receipt = repo.receipt();
    assert!(
        receipt["source"]["head"].is_null(),
        "fixture has an unborn HEAD"
    );
    assert!(
        receipt["source"]["status"]
            .as_str()
            .unwrap()
            .contains(".gitignore")
    );
    assert_eq!(
        Path::new(receipt["repository"].as_str().unwrap()),
        repo.0.path().canonicalize().unwrap()
    );
}

#[test]
fn non_utf8_output_preserves_raw_bytes_beside_decoded_text() {
    let repo = Repository::new(true);
    let output = repo.check(&["--", "/usr/bin/printf", "\\377"]);
    assert!(output.status.success(), "{output:?}");
    let value = repo.receipt();
    assert_eq!(value["outcome"]["stdout"], "\u{fffd}");
    let stdout = value["streams"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["channel"] == "stdout")
        .unwrap();
    assert_eq!(fs::read(stdout["path"].as_str().unwrap()).unwrap(), [255]);
    assert_eq!(stdout["sha256"], allstack::infrastructure::digest(&[255]));
}

#[test]
fn oversized_input_fails_before_starting_the_command() {
    let repo = Repository::new(true);
    fs::write(repo.0.path().join("large.txt"), vec![b'x'; 128 * 1024 + 1]).unwrap();
    let output = repo.check(&["--input", "large.txt", "--", "/usr/bin/true"]);
    assert!(!output.status.success());
    let value = repo.receipt();
    assert_eq!(value["matched"], false);
    assert!(value["outcome"].is_null());
    assert_eq!(value["streams"], serde_json::json!([]));
}

#[test]
fn symlink_invocation_keeps_argv_zero_while_recording_resolved_identity() {
    use std::os::unix::fs::{PermissionsExt, symlink};
    let repo = Repository::new(true);
    let real = repo.0.path().join("real-program");
    let alias = repo.0.path().join("program-alias");
    fs::write(&real, "#!/bin/sh\nprintf '%s' \"$0\"\n").unwrap();
    fs::set_permissions(&real, fs::Permissions::from_mode(0o700)).unwrap();
    symlink(&real, &alias).unwrap();
    let output = repo.check(&["--", alias.to_str().unwrap()]);
    assert!(output.status.success(), "{output:?}");
    let value = repo.receipt();
    assert_eq!(value["outcome"]["stdout"], alias.to_str().unwrap());
    assert_eq!(
        value["executable_before"]["path"],
        real.canonicalize().unwrap().to_str().unwrap()
    );
}
