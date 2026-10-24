use super::{git, process};
use crate::domain::{Publication, Run};
use anyhow::{Context, Result, ensure};
use serde_json::Value;
use std::path::Path;

pub fn repository(remote: &str) -> Result<String> {
    git::safe_remote(remote)?;
    let name = remote
        .trim_start_matches("https://github.com/")
        .trim_start_matches("git@github.com:")
        .trim_end_matches(".git");
    ensure!(
        name.split('/').count() == 2
            && name.split('/').all(|p| !p.is_empty()
                && p.bytes()
                    .all(|c| c.is_ascii_alphanumeric() || b"-_.".contains(&c))),
        "invalid GitHub repository"
    );
    Ok(name.into())
}

pub fn live(state: &Path, repo: &Path, name: &str, branch: &str) -> Result<Vec<Value>> {
    let raw = process::checked(
        state,
        repo,
        &[
            "gh",
            "pr",
            "list",
            "--repo",
            name,
            "--head",
            branch,
            "--state",
            "all",
            "--json",
            "state,url,headRefOid",
        ],
    )?;
    let prs: Vec<Value> = serde_json::from_str(&raw)?;
    ensure!(
        !prs.iter().any(|p| p["state"] == "MERGED"),
        "branch PR already merged; dead branch"
    );
    ensure!(
        prs.iter().filter(|p| p["state"] == "OPEN").count() <= 1,
        "multiple open PRs for branch"
    );
    Ok(prs)
}

pub fn remote_tip(state: &Path, repo: &Path, branch: &str) -> Result<Option<String>> {
    let raw = git::call(
        state,
        repo,
        &[
            "ls-remote",
            "--heads",
            "origin",
            &format!("refs/heads/{branch}"),
        ],
    )?;
    let rows: Vec<_> = raw.lines().filter(|s| !s.is_empty()).collect();
    ensure!(rows.len() <= 1, "ambiguous remote branch");
    Ok(rows
        .first()
        .and_then(|s| s.split_whitespace().next())
        .map(String::from))
}

pub fn publish(state: &Path, run: &Run, title: &str) -> Result<Publication> {
    let repo = Path::new(&run.repository);
    let name = repository(&run.remote)?;
    ensure!(
        run.branch != run.trunk
            && git::call(state, repo, &["branch", "--show-current"])? == run.branch,
        "publication must use the authorized live local branch"
    );
    let upstream = git::call(
        state,
        repo,
        &[
            "for-each-ref",
            "--format=%(upstream:short)",
            &format!("refs/heads/{}", run.branch),
        ],
    )?;
    ensure!(
        upstream.is_empty() || upstream == format!("origin/{}", run.branch),
        "unexpected branch upstream"
    );
    ensure!(
        git::call(state, repo, &["remote", "get-url", "origin"])? == run.remote,
        "remote changed"
    );
    git::clean(state, repo)?;
    let commit = &run
        .verification
        .as_ref()
        .context("verification missing")?
        .commit;
    ensure!(
        git::call(state, repo, &["rev-parse", "HEAD"])? == *commit,
        "head changed after verification"
    );
    let prs = live(state, repo, &name, &run.branch)?;
    let before = remote_tip(state, repo, &run.branch)?;
    if before.as_deref() != Some(commit) {
        ensure!(
            before.is_none() || before.as_deref() == Some(&run.base),
            "remote moved outside this run; publication requires review"
        );
        let lease = format!(
            "--force-with-lease=refs/heads/{}:{}",
            run.branch,
            before.unwrap_or_default()
        );
        let destination = format!("HEAD:{}", run.branch);
        git::call(state, repo, &["push", "-u", &lease, "origin", &destination])?;
    }
    ensure!(
        remote_tip(state, repo, &run.branch)?.as_deref() == Some(commit),
        "remote commit unconfirmed; reconcile destination"
    );
    let pr = if let Some(open) = prs.iter().find(|p| p["state"] == "OPEN") {
        open["url"].as_str().context("PR URL missing")?.to_string()
    } else {
        let body = format!(
            "## Summary\n\n{title}\n\n## Verification\n\nExact commit `{commit}` verified by Allstack run `{}`. Required command receipts remain in the local run store.\n\n## Design\n\nSee the indexed delivery plan under `docs/plans/allstack-{}.md`.\n",
            run.id, run.id
        );
        let path = state.join(format!("pr-{}.md", run.id));
        super::atomic_write(&path, body.as_bytes())?;
        process::checked(
            state,
            repo,
            &[
                "gh",
                "pr",
                "create",
                "--repo",
                &name,
                "--head",
                &run.branch,
                "--base",
                &run.trunk,
                "--title",
                title,
                "--body-file",
                path.to_str().context("invalid body path")?,
            ],
        )?
        .trim()
        .to_string()
    };
    let raw = process::checked(
        state,
        repo,
        &[
            "gh",
            "pr",
            "view",
            &pr,
            "--repo",
            &name,
            "--json",
            "state,headRefOid,url",
        ],
    )?;
    let actual: Value = serde_json::from_str(&raw)?;
    ensure!(
        actual["state"] == "OPEN" && actual["headRefOid"] == *commit,
        "PR head or state does not match verified commit"
    );
    Ok(Publication {
        commit: commit.clone(),
        remote: run.remote.clone(),
        branch: run.branch.clone(),
        pr,
        task_completed: run.publication.as_ref().is_some_and(|p| p.task_completed),
        memory_indexed: run.publication.as_ref().is_some_and(|p| p.memory_indexed),
    })
}
