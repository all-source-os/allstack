use allstack::infrastructure::{Artifacts, process, workspace};
use std::{collections::BTreeMap, fs, path::PathBuf};

struct Fixture {
    _temp: tempfile::TempDir,
    repo: PathBuf,
    state: PathBuf,
    artifacts: Artifacts,
}

impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let repo = temp.path().join("repository");
        let state = temp.path().join("state");
        fs::create_dir(&repo).unwrap();
        fs::create_dir(&state).unwrap();
        let f = Self {
            artifacts: Artifacts(state.join("objects")),
            _temp: temp,
            repo,
            state,
        };
        f.git(&["init", "--quiet"]);
        fs::write(f.repo.join("source.txt"), "before\n").unwrap();
        fs::write(f.repo.join(".gitignore"), "*.skill\n").unwrap();
        fs::write(
            f.repo.join(".env.production"),
            "GUARDED_FIXTURE_CONTENT=local\n",
        )
        .unwrap();
        f.git(&[
            "add",
            "-f",
            "--",
            "source.txt",
            ".env.production",
            ".gitignore",
        ]);
        f.commit();
        f
    }

    fn git(&self, args: &[&str]) {
        let outcome = process::run(process::Spec {
            argv: ["git"]
                .into_iter()
                .chain(args.iter().copied())
                .map(String::from)
                .collect(),
            cwd: self.repo.clone(),
            seconds: 10,
            environment: BTreeMap::new(),
            input: None,
            directory: self.state.join("process"),
        })
        .unwrap();
        assert!(outcome.success(), "{}", outcome.stderr);
    }

    fn commit(&self) {
        self.git(&[
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.test",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "--quiet",
            "-m",
            "test fixture",
        ]);
    }

    fn capture(&self) -> anyhow::Result<String> {
        workspace::capture(&self.state, &self.repo, &self.artifacts)
    }
}

fn unchanged_guarded_base_files_restore_without_copying_contents_to_artifacts() {
    let f = Fixture::new();
    fs::write(f.repo.join("source.txt"), "candidate\n").unwrap();
    fs::write(f.repo.join("method.skill"), "native method\n").unwrap();
    f.git(&["add", "-f", "--", "method.skill"]);
    let key = f.capture().unwrap();
    let snapshot = workspace::read_snapshot(&f.artifacts, &key).unwrap();
    assert!(!snapshot.files.contains_key(".env.production"));
    let restored = workspace::restore(&f.state, &f.repo, &f.artifacts, &key, "guarded", 1).unwrap();
    assert_eq!(
        fs::read_to_string(restored.join("source.txt")).unwrap(),
        "candidate\n"
    );
    assert_eq!(
        fs::read_to_string(restored.join("method.skill")).unwrap(),
        "native method\n"
    );
    assert_eq!(
        fs::read_to_string(restored.join(".env.production")).unwrap(),
        "GUARDED_FIXTURE_CONTENT=local\n"
    );
    for entry in fs::read_dir(&f.artifacts.0).unwrap() {
        let bytes = fs::read(entry.unwrap().path()).unwrap();
        assert!(
            !bytes
                .windows(b"GUARDED_FIXTURE_CONTENT".len())
                .any(|w| w == b"GUARDED_FIXTURE_CONTENT")
        );
    }
    assert_eq!(
        workspace::restore(&f.state, &f.repo, &f.artifacts, &key, "guarded", 1).unwrap(),
        restored
    );
}

fn changed_added_or_deleted_guarded_files_still_fail_capture() {
    for mutation in ["unstaged", "staged", "assumed", "new", "deleted"] {
        let f = Fixture::new();
        match mutation {
            "unstaged" | "staged" | "assumed" => {
                if mutation == "assumed" {
                    f.git(&["update-index", "--assume-unchanged", ".env.production"]);
                }
                fs::write(f.repo.join(".env.production"), "changed\n").unwrap();
                if mutation == "staged" {
                    f.git(&["add", "-f", ".env.production"]);
                }
            }
            "new" => {
                fs::write(f.repo.join(".env.local"), "new\n").unwrap();
                f.git(&["add", "-f", ".env.local"]);
            }
            "deleted" => fs::remove_file(f.repo.join(".env.production")).unwrap(),
            _ => unreachable!(),
        }
        assert!(
            f.capture()
                .unwrap_err()
                .to_string()
                .contains("sensitive path"),
            "{mutation}"
        );
    }
}

fn preserved_base_changes_invalidate_content_and_refuse_materialization() {
    let f = Fixture::new();
    let original = f.capture().unwrap();
    fs::write(f.repo.join(".env.production"), "changed base\n").unwrap();
    f.git(&["add", "-f", ".env.production"]);
    f.commit();
    let changed = f.capture().unwrap();
    assert_ne!(
        workspace::content_digest(&f.artifacts, &original).unwrap(),
        workspace::content_digest(&f.artifacts, &changed).unwrap()
    );
    fs::write(f.repo.join("source.txt"), "outside change\n").unwrap();
    assert!(workspace::materialize(&f.state, &f.repo, &f.artifacts, &original).is_err());
    assert_eq!(
        fs::read_to_string(f.repo.join("source.txt")).unwrap(),
        "outside change\n"
    );
}

fn guarded_contents_in_artifacts_cannot_overwrite_base_files() {
    let f = Fixture::new();
    let key = f.capture().unwrap();
    let mut snapshot = workspace::read_snapshot(&f.artifacts, &key).unwrap();
    snapshot.preserved.clear();
    snapshot.files.insert(
        ".env.production".into(),
        allstack::domain::FileEntry {
            digest: f.artifacts.put(b"forged fixture contents\n").unwrap(),
            executable: false,
        },
    );
    let forged = f
        .artifacts
        .put(&serde_json::to_vec(&snapshot).unwrap())
        .unwrap();
    assert!(
        workspace::materialize(&f.state, &f.repo, &f.artifacts, &forged)
            .unwrap_err()
            .to_string()
            .contains("guarded contents")
    );
    assert_eq!(
        fs::read_to_string(f.repo.join(".env.production")).unwrap(),
        "GUARDED_FIXTURE_CONTENT=local\n"
    );
}

fn main() -> anyhow::Result<()> {
    let cases: [(&str, fn()); 4] = [
        (
            "restore-without-copy",
            unchanged_guarded_base_files_restore_without_copying_contents_to_artifacts,
        ),
        (
            "reject-guarded-changes",
            changed_added_or_deleted_guarded_files_still_fail_capture,
        ),
        (
            "reject-base-changes",
            preserved_base_changes_invalidate_content_and_refuse_materialization,
        ),
        (
            "reject-guarded-artifacts",
            guarded_contents_in_artifacts_cannot_overwrite_base_files,
        ),
    ];
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--list") {
        if !args.iter().any(|arg| arg == "--ignored") {
            for (name, _) in cases {
                println!("{name}: test");
            }
        }
        return Ok(());
    }
    if let Some((_, case)) = cases
        .iter()
        .find(|(name, _)| args.iter().any(|arg| arg == name))
    {
        case();
        return Ok(());
    }
    anyhow::ensure!(
        !args.iter().any(|arg| arg == "--exact"),
        "unknown workspace proof case"
    );
    match args.get(1).map(String::as_str) {
        Some("__supervise") => return process::supervise(),
        Some("__case") => {
            let (_, case) = cases
                .iter()
                .find(|(name, _)| Some(*name) == args.get(2).map(String::as_str))
                .ok_or_else(|| anyhow::anyhow!("unknown workspace proof case"))?;
            case();
            return Ok(());
        }
        _ => {}
    }
    let state = tempfile::tempdir()?;
    for (name, _) in cases {
        let result = process::run(process::Spec {
            argv: vec![
                std::env::current_exe()?.to_string_lossy().into_owned(),
                "__case".into(),
                name.into(),
            ],
            cwd: state.path().into(),
            seconds: 45,
            environment: BTreeMap::new(),
            input: None,
            directory: state.path().join(name),
        })?;
        anyhow::ensure!(
            result.success(),
            "workspace case {name} failed: {}\n{}",
            result.stdout,
            result.stderr
        );
        println!("workspace case {name}: passed");
    }
    println!("{} workspace cases passed", cases.len());
    Ok(())
}
