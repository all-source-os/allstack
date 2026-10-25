//! Standalone skill knowledge uses Prime without creating delivery state or tasks.
use super::{bootstrap::Scratch, bootstrap_fs::Root, digest, memory::Memory, private_dir, process};
use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};
use std::path::Path;

pub async fn remember(repo: &Path, kind: &str, source: &str, summary: &str) -> Result<Value> {
    ensure!(
        ["decision", "review", "qa"].contains(&kind),
        "unsupported knowledge kind"
    );
    ensure!(
        !summary.trim().is_empty() && summary.len() <= 16 * 1024,
        "summary must contain 1..16 KiB"
    );
    ensure!(
        source.starts_with("docs/")
            && source.ends_with(".md")
            && !source.split('/').any(|p| p.starts_with('.')),
        "knowledge source must be a visible Markdown document under docs/"
    );
    let bytes = Root::new(repo)?
        .read(source)?
        .context("knowledge source missing")?;
    let scratch = Scratch::new()?;
    process::checked(
        &scratch.0,
        repo,
        &["git", "check-ignore", "--quiet", ".allstack/prime"],
    )
    .context("ignore .allstack/ before retaining project knowledge")?;
    private_dir(&repo.join(".allstack"))?;
    let memory = Memory::open(&repo.join(".allstack/prime")).await?;
    let result = memory
        .remember(kind, source, &digest(&bytes), summary)
        .await;
    let shutdown = memory.shutdown().await;
    let value = result?;
    shutdown?;
    Ok(value)
}

pub async fn recall(repo: &Path, query: &str) -> Result<Value> {
    if !repo.join(".allstack/prime").exists() {
        return Ok(json!({"retrieval":"local-prime-absent","authority":"context-only","nodes":[]}));
    }
    let memory = Memory::open(&repo.join(".allstack/prime")).await?;
    let value = memory.recall(query);
    memory.shutdown().await?;
    Ok(value)
}
