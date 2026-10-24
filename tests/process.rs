use allstack::infrastructure::process::{Outcome, Spec};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
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

fn start(dir: &std::path::Path, argv: Vec<String>, seconds: u64) -> Owned {
    let mut child = Owned(
        Command::new(env!("CARGO_BIN_EXE_allstack"))
            .arg("__supervise")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let spec = Spec {
        argv,
        cwd: dir.into(),
        seconds,
        environment: BTreeMap::new(),
        input: None,
        directory: dir.into(),
    };
    let pipe = child.0.stdin.as_mut().unwrap();
    serde_json::to_writer(&mut *pipe, &spec).unwrap();
    pipe.write_all(b"\n").unwrap();
    pipe.flush().unwrap();
    child
}

fn result(child: &mut Owned, dir: &std::path::Path) -> Outcome {
    let until = Instant::now() + Duration::from_secs(10);
    while child.0.try_wait().unwrap().is_none() {
        assert!(
            Instant::now() < until,
            "supervisor did not terminate within test deadline"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
    serde_json::from_slice(&fs::read(dir.join("result.json")).unwrap()).unwrap()
}

#[test]
fn command_result_and_deadline_are_real_process_observations() {
    let dir = tempfile::tempdir().unwrap();
    let mut child = start(
        dir.path(),
        vec!["/usr/bin/printf".into(), "EXECUTED".into()],
        2,
    );
    let observed = result(&mut child, dir.path());
    assert!(observed.success());
    assert_eq!(observed.stdout, "EXECUTED");
    let dir = tempfile::tempdir().unwrap();
    let mut child = start(dir.path(), vec!["/bin/sleep".into(), "20".into()], 1);
    let observed = result(&mut child, dir.path());
    assert!(observed.timed_out);
    assert!(!observed.success());
}

#[test]
fn losing_owner_pipe_terminates_and_reaps_child() {
    let dir = tempfile::tempdir().unwrap();
    let script = format!(
        "echo $$ > '{}'; exec /bin/sleep 20",
        dir.path().join("pid").display()
    );
    let mut child = start(dir.path(), vec!["/bin/sh".into(), "-c".into(), script], 5);
    let until = Instant::now() + Duration::from_secs(3);
    while !dir.path().join("pid").exists() {
        assert!(Instant::now() < until, "child never started");
        std::thread::sleep(Duration::from_millis(10));
    }
    let pid: i32 = fs::read_to_string(dir.path().join("pid"))
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    drop(child.0.stdin.take());
    let observed = result(&mut child, dir.path());
    assert!(observed.owner_lost);
    assert!(!observed.success());
    assert_eq!(
        unsafe { libc::kill(pid, 0) },
        -1,
        "owned child was not reaped"
    );
}
