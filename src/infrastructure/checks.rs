//! Native command receipts add repository context without opening delivery services.
use super::{atomic_write, bootstrap::Scratch, digest, private_dir, process};
use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::Path,
    time::{Duration, Instant},
};

struct Budget(Instant);

impl Budget {
    fn remaining(&self) -> Result<u64> {
        let left = self.0.saturating_duration_since(Instant::now()).as_secs();
        ensure!(left > 0, "command check deadline exhausted");
        Ok(left)
    }

    fn run(
        &self,
        directory: &Path,
        repo: &Path,
        argv: Vec<String>,
        input: Option<String>,
    ) -> Result<process::Outcome> {
        process::run(process::Spec {
            argv,
            cwd: repo.into(),
            seconds: self.remaining()?,
            environment: BTreeMap::new(),
            input,
            directory: directory.into(),
        })
    }

    fn git(&self, directory: &Path, repo: &Path, args: &[&str]) -> Result<process::Outcome> {
        self.run(
            directory,
            repo,
            std::iter::once("git")
                .chain(args.iter().copied())
                .map(str::to_owned)
                .collect(),
            None,
        )
    }
}

fn fingerprint(path: &Path) -> Result<Value> {
    let resolved = path.canonicalize()?;
    ensure!(resolved.is_file(), "executable is not a file");
    Ok(json!({"path":resolved,"sha256":file_digest(&resolved)?.0}))
}

fn file_digest(path: &Path) -> Result<(String, u64)> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut size = 0;
    let mut buffer = [0; 64 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            return Ok((format!("{:x}", hasher.finalize()), size));
        }
        hasher.update(&buffer[..count]);
        size += count as u64;
    }
}

fn streams(directory: &Path) -> Result<Value> {
    if !directory.exists() {
        return Ok(json!([]));
    }
    let mut values = Vec::new();
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        for name in ["stdout", "stderr", "supervisor-stderr"] {
            let file = path.join(name);
            if file.is_file() {
                let (sha256, bytes) = file_digest(&file)?;
                values.push(json!({"channel":name,"path":file,"bytes":bytes,"sha256":sha256}));
            }
        }
    }
    Ok(json!(values))
}

pub fn check(
    repo: &Path,
    argv: &[String],
    expected_exit: u8,
    seconds: u64,
    input: Option<&Path>,
) -> Result<Value> {
    ensure!(
        (2..=270).contains(&seconds),
        "check deadline must be 2..=270 seconds including metadata"
    );
    ensure!(
        !argv.is_empty() && argv.iter().map(String::len).sum::<usize>() <= 64 * 1024,
        "command must contain 1..64 KiB"
    );
    ensure!(
        Path::new(&argv[0]).is_absolute(),
        "check requires an absolute executable path"
    );
    let budget = Budget(Instant::now() + Duration::from_secs(seconds));
    let scratch = Scratch::new()?;
    budget
        .git(
            &scratch.0,
            repo,
            &["check-ignore", "--quiet", ".allstack/checks/probe"],
        )?
        .checked()
        .context("ignore .allstack/ before retaining command evidence")?;
    let directory = repo
        .join(".allstack/checks")
        .join(uuid::Uuid::new_v4().to_string());
    for path in [
        repo.join(".allstack"),
        repo.join(".allstack/checks"),
        directory.clone(),
    ] {
        private_dir(&path)?;
    }
    let receipt_path = directory.join("receipt.json");
    let command_directory = directory.join("command");
    let mut receipt = json!({
        "schema":1,"receipt":receipt_path,"repository":repo,"argv":argv,
        "expected_exit":expected_exit,"deadline_seconds":seconds,"matched":false,
        "source":null,"executable_before":null,"executable_after":null,"input":null,
        "outcome":null,"error":null,"streams":[],
    });
    atomic_write(&receipt_path, &serde_json::to_vec_pretty(&receipt)?)?;
    let mut result: Result<()> = (|| {
        let head = budget.git(
            &directory.join("metadata"),
            repo,
            &["rev-parse", "--verify", "HEAD"],
        )?;
        ensure!(
            !head.timed_out && !head.owner_lost,
            "source revision lookup interrupted"
        );
        let status = budget
            .git(
                &directory.join("metadata"),
                repo,
                &["status", "--porcelain=v1", "--untracked-files=all"],
            )?
            .checked()?;
        receipt["source"] = json!({"head":if head.success() {Some(head.stdout.trim().to_owned())} else {None},"status":status});
        receipt["executable_before"] = fingerprint(Path::new(&argv[0]))?;
        let text = input
            .map(|path| -> Result<String> {
                let path = if path.is_absolute() {
                    path.to_owned()
                } else {
                    repo.join(path)
                };
                let mut bytes = Vec::new();
                fs::File::open(&path)?
                    .take(128 * 1024 + 1)
                    .read_to_end(&mut bytes)?;
                ensure!(bytes.len() <= 128 * 1024, "input exceeds 128 KiB");
                receipt["input"] = json!({"path":path,"sha256":digest(&bytes),"bytes":bytes.len()});
                String::from_utf8(bytes).context("input must be UTF-8")
            })
            .transpose()?;
        atomic_write(&receipt_path, &serde_json::to_vec_pretty(&receipt)?)?;
        let outcome = budget.run(&command_directory, repo, argv.to_vec(), text)?;
        let matched = outcome.code == Some(i32::from(expected_exit))
            && !outcome.timed_out
            && !outcome.owner_lost;
        receipt["outcome"] = serde_json::to_value(&outcome)?;
        receipt["executable_after"] = fingerprint(Path::new(&argv[0]))?;
        ensure!(
            receipt["executable_before"] == receipt["executable_after"],
            "executable changed during command"
        );
        ensure!(
            Instant::now() < budget.0,
            "command check deadline exhausted"
        );
        ensure!(
            matched,
            "expected exit {expected_exit}; observed exit {:?}, timeout {}, owner lost {}",
            outcome.code,
            outcome.timed_out,
            outcome.owner_lost
        );
        Ok(())
    })();
    match streams(&command_directory) {
        Ok(value) => receipt["streams"] = value,
        Err(error) => {
            receipt["streams_error"] = json!(format!("{error:#}"));
            if result.is_ok() {
                result = Err(error.context("raw stream evidence could not be retained"));
            }
        }
    }
    if result.is_ok() && Instant::now() >= budget.0 {
        result = Err(anyhow::anyhow!("command check deadline exhausted"));
    }
    receipt["matched"] = json!(result.is_ok());
    if let Err(error) = &result {
        receipt["error"] = json!(format!("{error:#}"));
    }
    atomic_write(&receipt_path, &serde_json::to_vec_pretty(&receipt)?)?;
    result.with_context(|| format!("command check failed; receipt {}", receipt_path.display()))?;
    Ok(receipt)
}
