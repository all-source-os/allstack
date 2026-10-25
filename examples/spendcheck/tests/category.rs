use std::{
    fs,
    process::{Command, Output, Stdio},
    thread,
    time::{Duration, Instant},
};

fn run(input: &str, args: &[&str]) -> Output {
    let path = std::env::temp_dir().join(format!(
        "spendcheck-{}-{:?}.csv",
        std::process::id(),
        thread::current().id()
    ));
    fs::write(&path, input).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_spendcheck"))
        .arg(&path)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if child.try_wait().unwrap().is_some() {
            break;
        }
        if Instant::now() >= deadline {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("spendcheck did not finish");
        }
        thread::sleep(Duration::from_millis(10));
    }
    let result = child.wait_with_output().unwrap();
    fs::remove_file(path).unwrap();
    result
}

const INPUT: &str = include_str!("../expenses.csv");

#[test]
fn category_selects_exact_records_and_preserves_money_units() {
    let output = run(INPUT, &["--category", "travel"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"records: 3\nEUR 10.00\nUSD 7.00\n");
}

#[test]
fn no_match_is_an_empty_success() {
    let output = run(INPUT, &["--category", "rent"]);
    assert!(output.status.success());
    assert_eq!(output.stdout, b"records: 0\n");
}

#[test]
fn category_matching_is_exact_and_case_sensitive() {
    let output = run(INPUT, &["--category", "Travel"]);
    assert!(output.status.success());
    assert_eq!(output.stdout, b"records: 0\n");
}

#[test]
fn malformed_excluded_record_still_fails_without_partial_success() {
    let output = run(
        "category,currency,amount\ntravel,EUR,1.00\nfood,EUR,bad\n",
        &["--category", "travel"],
    );
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("line 3"));
}

#[test]
fn missing_unknown_and_empty_options_fail() {
    for args in [
        vec!["--category"],
        vec!["--unknown", "travel"],
        vec!["--category", ""],
        vec!["--category", "travel", "extra"],
    ] {
        let output = run(INPUT, &args);
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
    }
}

#[test]
fn unfiltered_cli_preserves_existing_output() {
    let output = run(INPUT, &[]);
    assert!(output.status.success());
    assert_eq!(output.stdout, b"records: 4\nEUR 14.50\nUSD 7.00\n");
}
