use super::{atomic_write, private_dir};
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    io::{BufRead, BufReader, Read, Write},
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Spec {
    pub argv: Vec<String>,
    pub cwd: PathBuf,
    pub seconds: u64,
    pub environment: BTreeMap<String, String>,
    pub input: Option<String>,
    pub directory: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Outcome {
    pub code: Option<i32>,
    pub timed_out: bool,
    pub owner_lost: bool,
    pub elapsed_ms: u128,
    pub stdout: String,
    pub stderr: String,
}

impl Outcome {
    pub fn success(&self) -> bool {
        self.code == Some(0) && !self.timed_out && !self.owner_lost
    }
    pub fn checked(self) -> Result<String> {
        ensure!(
            self.success(),
            "command failed (exit {:?}, timeout {}): {}\n{}",
            self.code,
            self.timed_out,
            self.stderr,
            self.stdout
        );
        Ok(self.stdout)
    }
}

struct Guardian {
    child: Child,
    input: Option<std::process::ChildStdin>,
}
impl Drop for Guardian {
    fn drop(&mut self) {
        self.input.take();
        let until = Instant::now() + Duration::from_secs(3);
        while Instant::now() < until {
            if self.child.try_wait().ok().flatten().is_some() {
                return;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

pub fn run(mut spec: Spec) -> Result<Outcome> {
    ensure!(
        !spec.argv.is_empty() && (1..=290).contains(&spec.seconds),
        "invalid command or deadline (maximum 290 seconds plus cleanup)"
    );
    spec.directory = spec.directory.join(uuid::Uuid::new_v4().to_string());
    private_dir(&spec.directory)?;
    let child = Command::new(std::env::current_exe()?)
        .arg("__supervise")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(fs::File::create(spec.directory.join("supervisor-stderr"))?)
        .process_group(0)
        .spawn()?;
    let mut guardian = Guardian { child, input: None };
    guardian.input = guardian.child.stdin.take();
    let input = guardian.input.as_mut().context("missing supervisor pipe")?;
    serde_json::to_writer(&mut *input, &spec)?;
    input.write_all(b"\n")?;
    input.flush()?;
    let end = Instant::now() + Duration::from_secs(spec.seconds + 5);
    loop {
        if guardian.child.try_wait()?.is_some() {
            break;
        }
        ensure!(
            Instant::now() < end,
            "supervisor exceeded bounded cleanup deadline"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    let outcome = fs::read(spec.directory.join("result.json"))
        .context("supervisor exited without receipt; external outcome unknown")?;
    Ok(serde_json::from_slice(&outcome)?)
}

struct OwnedChild(Child);
impl Drop for OwnedChild {
    fn drop(&mut self) {
        unsafe {
            libc::kill(-(self.0.id() as i32), libc::SIGKILL);
        }
        let _ = self.0.wait();
    }
}

pub fn supervise() -> Result<()> {
    let mut owner = BufReader::new(std::io::stdin());
    let mut line = String::new();
    owner.read_line(&mut line)?;
    ensure!(line.len() <= 1024 * 1024, "supervisor input too large");
    let spec: Spec = serde_json::from_str(&line)?;
    ensure!(
        !spec.argv.is_empty() && (1..=290).contains(&spec.seconds),
        "invalid supervisor command"
    );
    private_dir(&spec.directory)?;
    let output = fs::File::create(spec.directory.join("stdout"))?;
    let errors = fs::File::create(spec.directory.join("stderr"))?;
    let input = if let Some(text) = &spec.input {
        let path = spec.directory.join("input");
        atomic_write(&path, text.as_bytes())?;
        Stdio::from(fs::File::open(path)?)
    } else {
        Stdio::null()
    };
    let mut command = Command::new(&spec.argv[0]);
    command
        .args(&spec.argv[1..])
        .current_dir(&spec.cwd)
        .envs(&spec.environment)
        .stdout(output)
        .stderr(errors)
        .stdin(input)
        .process_group(0);
    let start = Instant::now();
    let mut child = OwnedChild(command.spawn()?);
    // The pipe belongs to the caller, so caller death revokes the child's lifetime.
    unsafe {
        libc::fcntl(libc::STDIN_FILENO, libc::F_SETFL, libc::O_NONBLOCK);
    }
    let mut owner_lost = false;
    let mut timed_out = false;
    let mut code = None;
    loop {
        if let Some(status) = child.0.try_wait()? {
            code = status.code();
            break;
        }
        let mut byte = [0];
        match owner.read(&mut byte) {
            Ok(0) => owner_lost = true,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(_) => owner_lost = true,
            _ => {}
        }
        timed_out = start.elapsed() >= Duration::from_secs(spec.seconds);
        let oversized = ["stdout", "stderr"]
            .iter()
            .any(|n| fs::metadata(spec.directory.join(n)).is_ok_and(|m| m.len() > 4 * 1024 * 1024));
        if owner_lost || timed_out || oversized {
            break;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    drop(child);
    let read = |name| -> Result<String> {
        let mut bytes = Vec::new();
        fs::File::open(spec.directory.join(name))?
            .take(4 * 1024 * 1024)
            .read_to_end(&mut bytes)?;
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    };
    let result = Outcome {
        code,
        timed_out,
        owner_lost,
        elapsed_ms: start.elapsed().as_millis(),
        stdout: read("stdout")?,
        stderr: read("stderr")?,
    };
    atomic_write(
        &spec.directory.join("result.json"),
        &serde_json::to_vec(&result)?,
    )
}

pub fn checked(root: &Path, cwd: &Path, argv: &[&str]) -> Result<String> {
    run(Spec {
        argv: argv.iter().map(|s| s.to_string()).collect(),
        cwd: cwd.into(),
        seconds: 30,
        environment: BTreeMap::new(),
        input: None,
        directory: root.join("process"),
    })?
    .checked()
}
