use allstack::infrastructure::{
    git,
    process::{self, Spec},
};
use clap::Parser;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let result = match arguments.get(1).map(String::as_str) {
        Some("__supervise") => process::supervise().map(|_| serde_json::Value::Null),
        Some("__operation") => {
            let mut child_arguments = arguments.clone();
            child_arguments.remove(1);
            allstack::presentation::execute(allstack::presentation::Cli::parse_from(
                child_arguments,
            ))
            .await
        }
        _ => {
            let cli = allstack::presentation::Cli::parse();
            (|| -> anyhow::Result<serde_json::Value> {
                let repo = git::root(&cli.repository)?;
                let mut argv = vec![
                    std::env::current_exe()?.to_string_lossy().into_owned(),
                    "__operation".into(),
                ];
                argv.extend(arguments.into_iter().skip(1));
                let output = process::run(Spec {
                    argv,
                    cwd: std::env::current_dir()?,
                    seconds: 290,
                    environment: BTreeMap::new(),
                    input: None,
                    directory: repo.join(".allstack/process"),
                })?;
                anyhow::ensure!(
                    output.success(),
                    "operation failed or exceeded its deadline: {}",
                    output.stderr
                );
                Ok(serde_json::from_str(&output.stdout)?)
            })()
        }
    };
    match result {
        Ok(value) if !value.is_null() => println!(
            "{}",
            serde_json::to_string_pretty(&value).expect("JSON result")
        ),
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}", serde_json::json!({"error":format!("{error:#}")}));
            std::process::exit(1);
        }
    }
}
