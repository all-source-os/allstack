use allstack::infrastructure::bootstrap_fs::Root;
use serde_json::Value;
use std::{
    fs,
    os::unix::fs::{PermissionsExt, symlink},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};

struct Owned(Child);
impl Drop for Owned {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn bounded(mut command: Command) -> (bool, String, String) {
    let output = tempfile::tempdir().unwrap();
    let mut child = Owned(
        command
            .stdout(fs::File::create(output.path().join("out")).unwrap())
            .stderr(fs::File::create(output.path().join("err")).unwrap())
            .stdin(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let deadline = Instant::now() + Duration::from_secs(45);
    let status = loop {
        if let Some(status) = child.0.try_wait().unwrap() {
            break status;
        }
        assert!(
            Instant::now() < deadline,
            "bootstrap test child exceeded deadline"
        );
        std::thread::sleep(Duration::from_millis(10));
    };
    (
        status.success(),
        fs::read_to_string(output.path().join("out")).unwrap(),
        fs::read_to_string(output.path().join("err")).unwrap(),
    )
}

struct Fixture {
    _tmp: tempfile::TempDir,
    repo: PathBuf,
    path: String,
}
impl Fixture {
    fn new(real: bool) -> Self {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path().join("repository");
        fs::create_dir(&repo).unwrap();
        let mut git = Command::new("git");
        git.args(["init", "--quiet"]).current_dir(&repo);
        assert!(bounded(git).0);
        fs::write(
            repo.join("Cargo.toml"),
            "[package]\nname = \"bootstrap-fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        )
        .unwrap();
        fs::write(
            repo.join("Cargo.lock"),
            "version = 4\n\n[[package]]\nname = \"bootstrap-fixture\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        fs::create_dir(repo.join("src")).unwrap();
        fs::write(
            repo.join("src/lib.rs"),
            "pub fn installed() -> bool {\n    true\n}\n",
        )
        .unwrap();
        fs::create_dir(repo.join("tests")).unwrap();
        fs::write(
            repo.join("tests/installed.rs"),
            "#[test]\nfn installed() {\n    assert!(bootstrap_fixture::installed());\n}\n",
        )
        .unwrap();
        let path = if real {
            std::env::var("PATH").unwrap()
        } else {
            let bin = tmp.path().join("bin");
            fs::create_dir(&bin).unwrap();
            // A shell fixture models only the cn process protocol; real-store proof is separate.
            fs::write(bin.join("cn"), "#!/bin/sh\ncase \"$1\" in\n--version) echo 'cn protocol-fixture';;\ninit) mkdir .chronis; printf fixture > .chronis/protocol;;\nlist) test -f .chronis/protocol;;\n*) exit 3;;\nesac\n").unwrap();
            fs::set_permissions(bin.join("cn"), fs::Permissions::from_mode(0o755)).unwrap();
            for tool in ["codex", "claude", "gh", "cargo", "rustc"] {
                symlink("/usr/bin/false", bin.join(tool)).unwrap();
            }
            format!("{}:{}", bin.display(), std::env::var("PATH").unwrap())
        };
        Self {
            _tmp: tmp,
            repo,
            path,
        }
    }
    fn run(&self, args: &[&str]) -> (bool, String, String) {
        let binary = std::env::var("ALLSTACK_TEST_BINARY")
            .unwrap_or_else(|_| env!("CARGO_BIN_EXE_allstack").into());
        let mut command = Command::new(binary);
        command
            .arg("-C")
            .arg(&self.repo)
            .args(args)
            .env("PATH", &self.path)
            .current_dir(&self.repo);
        bounded(command)
    }
    fn json(&self, args: &[&str]) -> Value {
        let (ok, out, err) = self.run(args);
        assert!(ok, "{out}\n{err}");
        serde_json::from_str(&out).unwrap()
    }
    fn fingerprint(&self) -> Vec<(PathBuf, Vec<u8>)> {
        fn walk(root: &Path, dir: &Path, out: &mut Vec<(PathBuf, Vec<u8>)>) {
            for entry in fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if [".git", ".chronis", ".allstack", ".prime"]
                    .iter()
                    .any(|name| path.file_name().unwrap() == *name)
                {
                    continue;
                }
                if path.is_dir() {
                    walk(root, &path, out);
                } else {
                    out.push((
                        path.strip_prefix(root).unwrap().into(),
                        fs::read(path).unwrap(),
                    ));
                }
            }
        }
        let mut out = Vec::new();
        walk(&self.repo, &self.repo, &mut out);
        out.sort();
        out
    }
}

#[test]
fn check_and_doctor_leave_fresh_repository_untouched() {
    let f = Fixture::new(false);
    let before = f.fingerprint();
    let doctor = f.json(&["doctor"]);
    assert_eq!(doctor["setup_ready"], false);
    assert_eq!(doctor["stores_opened"], false);
    let check = f.json(&["init", "--check", "--profile", "rust"]);
    assert_eq!(check["ready"], true);
    assert_eq!(check["applied"], false);
    assert_eq!(f.fingerprint(), before);
    assert!(!f.repo.join(".allstack").exists());
    assert!(!f.repo.join(".chronis").exists());
}

fn assert_setup(f: &Fixture) {
    let first = f.json(&["init", "--profile", "rust"]);
    println!("BOOTSTRAP {first}");
    assert_eq!(first["applied"], true);
    assert!(f.repo.join(".chronis").is_dir());
    assert!(!f.repo.join(".allstack").exists());
    let doctor = f.json(&["doctor"]);
    println!("DOCTOR {doctor}");
    assert_eq!(doctor["setup_ready"], true, "{doctor}");
    assert_eq!(doctor["chronis"], "readable");
    assert_eq!(doctor["authentication"], "not-probed");
    let before = f.fingerprint();
    let second = f.json(&["init"]);
    assert!(
        second["files"]
            .as_array()
            .unwrap()
            .iter()
            .all(|f| f["status"] == "unchanged")
    );
    assert_eq!(f.fingerprint(), before);
    for host in [".agents", ".claude"] {
        let skill = f.repo.join(host).join("skills/allstack-delivery/SKILL.md");
        let link = skill
            .parent()
            .unwrap()
            .join("../../../docs/reference/allstack.md");
        assert!(link.canonicalize().unwrap().is_file());
    }
    fs::write(f.repo.join("workflows/methods/build.md"), "custom method").unwrap();
    let before = f.fingerprint();
    let (ok, _, err) = f.run(&["init"]);
    assert!(!ok && err.contains("conflict"), "{err}");
    assert_eq!(f.fingerprint(), before);
}

#[test]
fn initializes_protocol_boundary_and_reruns_without_replacing_custom_files() {
    let f = Fixture::new(false);
    fs::write(f.repo.join("AGENTS.md"), "existing instructions").unwrap();
    fs::write(
        f.repo.join(".gitignore"),
        "/.allstack/\n!/.allstack/\ncustom/",
    )
    .unwrap();
    fs::create_dir(f.repo.join("docs")).unwrap();
    fs::write(f.repo.join("docs/README.md"), "# Existing documentation").unwrap();
    assert_setup(&f);
    assert_eq!(
        fs::read_to_string(f.repo.join("AGENTS.md")).unwrap(),
        "existing instructions"
    );
    assert!(
        fs::read_to_string(f.repo.join("docs/README.md"))
            .unwrap()
            .starts_with("# Existing documentation\n")
    );
    assert!(
        fs::read_to_string(f.repo.join(".gitignore"))
            .unwrap()
            .starts_with("/.allstack/\n!/.allstack/\ncustom/\n")
    );
}

#[test]
fn effective_existing_ignores_keep_original_order_and_trailing_comments() {
    let f = Fixture::new(false);
    let original = "/.prime/\n/.allstack/\n/.chronis/\n# existing comment\n";
    fs::write(f.repo.join(".gitignore"), original).unwrap();
    f.json(&["init", "--profile", "rust"]);
    assert_eq!(
        fs::read_to_string(f.repo.join(".gitignore")).unwrap(),
        original
    );
    assert_eq!(f.json(&["doctor"])["setup_ready"], true);
}

#[test]
fn check_mode_never_opens_existing_chronis_engine() {
    let f = Fixture::new(false);
    fs::create_dir(f.repo.join(".chronis")).unwrap();
    let cn = f._tmp.path().join("bin/cn");
    let script = fs::read_to_string(&cn).unwrap().replace(
        "list) test -f .chronis/protocol",
        "list) touch .chronis/engine-opened; exit 3",
    );
    fs::write(cn, script).unwrap();
    let check = f.json(&["init", "--check", "--profile", "rust"]);
    assert_eq!(check["ready"], true);
    assert!(check["chronis"].as_str().unwrap().contains("deferred"));
    assert!(!f.repo.join(".chronis/engine-opened").exists());
    assert!(!f.run(&["init", "--profile", "rust"]).0);
    assert!(f.repo.join(".chronis/engine-opened").exists());
    assert!(!f.repo.join("workflows").exists());
}

#[test]
fn conflicts_and_missing_profile_fail_before_chronis_or_file_writes() {
    let f = Fixture::new(false);
    let before = f.fingerprint();
    assert!(!f.run(&["init"]).0);
    assert_eq!(f.fingerprint(), before);
    fs::create_dir(f.repo.join("workflows")).unwrap();
    fs::write(f.repo.join("workflows/delivery.toml"), "custom workflow").unwrap();
    let before = f.fingerprint();
    let check = f.json(&["init", "--check", "--profile", "rust"]);
    assert_eq!(check["ready"], false);
    assert_eq!(check["conflicts"][0], "workflows/delivery.toml");
    assert!(!f.run(&["init", "--profile", "rust"]).0);
    assert_eq!(f.fingerprint(), before);
    assert!(!f.repo.join(".chronis").exists());
}

#[test]
fn custom_policy_is_validated_preserved_and_used_by_doctor() {
    let f = Fixture::new(false);
    let custom = "command_seconds=60\n[[gates]]\nname='custom'\nargv=['cargo','test','--locked']\ntest=true\ncompile=true\nsuccess_marker='test result: ok.'\n";
    fs::write(f.repo.join("policy.toml"), custom).unwrap();
    f.json(&["init", "--policy", "policy.toml"]);
    assert_eq!(
        fs::read_to_string(f.repo.join("policy.toml")).unwrap(),
        custom
    );
    assert!(!f.repo.join("workflows/verification.toml").exists());
    assert_eq!(
        f.json(&["doctor", "--policy", "policy.toml"])["setup_ready"],
        true
    );
    fs::write(f.repo.join("policy.toml"), "gates=[]").unwrap();
    let before = f.fingerprint();
    assert!(!f.run(&["init", "--policy", "policy.toml"]).0);
    assert_eq!(f.fingerprint(), before);
}

#[test]
fn symlink_traversal_and_no_clobber_races_preserve_destination() {
    let tmp = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let root = Root::new(tmp.path()).unwrap();
    symlink(outside.path(), tmp.path().join("docs")).unwrap();
    assert!(root.read("docs/README.md").is_err());
    assert!(root.create("docs/README.md", b"unsafe").is_err());
    assert!(root.create("../escape", b"unsafe").is_err());
    assert!(root.read("/absolute").is_err());
    assert!(!outside.path().join("README.md").exists());
    assert_eq!(root.read("new.txt").unwrap(), None);
    fs::write(tmp.path().join("new.txt"), b"other writer").unwrap();
    assert!(root.create("new.txt", b"replacement").is_err());
    assert!(root.append("new.txt", b"old", b"suffix").is_err());
    assert_eq!(
        fs::read(tmp.path().join("new.txt")).unwrap(),
        b"other writer"
    );
    symlink(outside.path().join("missing"), tmp.path().join("dangling")).unwrap();
    assert!(root.read("dangling").is_err());
}

#[test]
fn unsafe_paths_and_missing_chronis_fail_without_bootstrap_writes() {
    let f = Fixture::new(false);
    for path in ["../escape.toml", "/tmp/escape.toml", ".git/bootstrap.toml"] {
        let before = f.fingerprint();
        assert!(!f.run(&["init", "--profile", "rust", "--policy", path]).0);
        assert_eq!(f.fingerprint(), before);
    }
    let outside = tempfile::tempdir().unwrap();
    symlink(outside.path(), f.repo.join("docs")).unwrap();
    assert!(!f.run(&["init", "--profile", "rust"]).0);
    assert!(!outside.path().join("reference").exists());
    assert!(!f.repo.join(".chronis").exists());
    fs::remove_file(f.repo.join("docs")).unwrap();
    let cn = f._tmp.path().join("bin/cn");
    fs::remove_file(&cn).unwrap();
    symlink("/usr/bin/false", cn).unwrap();
    let before = f.fingerprint();
    assert_eq!(
        f.json(&["init", "--check", "--profile", "rust"])["ready"],
        false
    );
    assert!(!f.run(&["init", "--profile", "rust"]).0);
    assert_eq!(f.fingerprint(), before);
}

#[test]
fn failed_chronis_initialization_reports_completed_paths_and_rerun_converges() {
    let f = Fixture::new(false);
    let cn = f._tmp.path().join("bin/cn");
    let script = fs::read_to_string(&cn).unwrap().replace(
        "init) mkdir",
        "init) if test ! -f ../failed-once; then touch ../failed-once; exit 7; fi; mkdir",
    );
    fs::write(cn, script).unwrap();
    let (ok, _, err) = f.run(&["init", "--profile", "rust"]);
    assert!(
        !ok && err.contains("completed:") && err.contains("Chronis initialization uncertain"),
        "{err}"
    );
    assert!(f.repo.join("workflows/verification.toml").is_file());
    assert!(!f.repo.join(".chronis").exists());
    let retry = f.json(&["init"]);
    assert_eq!(retry["applied"], true);
    assert_eq!(retry["chronis"], "initialized");
    assert!(
        retry["files"]
            .as_array()
            .unwrap()
            .iter()
            .all(|f| f["status"] == "unchanged")
    );
}

#[test]
#[ignore = "requires actual Chronis; run explicitly against installed ALLSTACK_TEST_BINARY"]
fn installed_binary_initializes_real_chronis_and_executes_generated_policy() {
    let f = Fixture::new(true);
    let before = f.fingerprint();
    assert_eq!(
        f.json(&["init", "--check", "--profile", "rust"])["ready"],
        true
    );
    assert_eq!(f.fingerprint(), before);
    assert_setup(&f);
    let policy: allstack::domain::Policy =
        toml::from_str(include_str!("../templates/verification-rust.toml")).unwrap();
    policy.validate().unwrap();
    for gate in policy.gates {
        let mut command = Command::new(&gate.argv[0]);
        command.args(&gate.argv[1..]).current_dir(&f.repo);
        let (ok, out, err) = bounded(command);
        println!("GATE {} success={ok}\n{out}\n{err}", gate.name);
        assert!(ok, "{}: {out}\n{err}", gate.name);
        if gate.test {
            assert!(allstack::infrastructure::gates::executed_tests(&format!("{out}\n{err}")) > 0);
        }
    }
}
