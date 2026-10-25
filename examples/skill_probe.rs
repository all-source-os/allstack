//! Real-host qualification reuses the production owner-pipe supervisor and receipts.
use allstack::infrastructure::process::{self, Spec};
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let argument = std::env::args()
        .nth(1)
        .context("supply a private Spec JSON file")?;
    if argument == "__supervise" {
        return process::supervise();
    }
    let spec: Spec = serde_json::from_slice(&std::fs::read(argument)?)?;
    let outcome = process::run(spec)?;
    println!("{}", serde_json::to_string(&outcome)?);
    anyhow::ensure!(
        outcome.success(),
        "native skill probe did not complete successfully"
    );
    Ok(())
}
