use super::process::{self, Spec};
use crate::domain::{Host, Policy, Run};
use anyhow::{Context, Result, ensure};
use serde_json::Value;
use std::{collections::BTreeMap, path::Path};

pub fn invoke(
    state: &Path,
    run: &Run,
    policy: &Policy,
    prompt: &str,
    planning: bool,
) -> Result<String> {
    let claude_build_tools = "Read,Glob,Grep,Edit,Write,Bash(git diff *),Bash(git status *),Bash(git add -- *),Bash(rtk git diff *),Bash(rtk git status *),Bash(rtk git add -- *),Bash(shasum -a 256 *)";
    let prompt = format!(
        "Managed Allstack run {}. Host {}, epoch {}, revision {}. Chronis task {} already claimed and read back by coordinator for this host. Active checkpoint {}. This detached worktree is intentional; original repository branch is {}. Do not change task ownership or run state yourself. Recover source context from the indexed plan; no prior conversation is assumed.\n\n{}",
        run.id,
        run.host.name(),
        run.epoch,
        run.revision,
        run.task.as_deref().unwrap_or("unbound"),
        run.checkpoint,
        run.branch,
        prompt
    );
    let prompt = if run.host == Host::ClaudeCode && !planning {
        format!(
            "{prompt}\n\nShell capability scope: use git diff, git status, or git add -- from the current working directory. Matching rtk git forms and shasum -a 256 are allowed. Do not use git -C, compound shell commands, or other shell programs. Use Read, Glob, Grep, Edit, and Write for source work. The coordinator runs compilation, tests, and publication."
        )
    } else {
        prompt
    };
    let argv: Vec<String> = match run.host {
        Host::Codex => vec![
            "codex",
            "exec",
            "--json",
            "--color",
            "never",
            "--sandbox",
            if planning {
                "read-only"
            } else {
                "workspace-write"
            },
            "-",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
        Host::ClaudeCode => vec![
            "claude",
            "--print",
            "--output-format",
            "json",
            "--permission-mode",
            if planning { "plan" } else { "acceptEdits" },
            "--tools",
            if planning {
                "Read,Glob,Grep"
            } else {
                "Read,Glob,Grep,Edit,Write,Bash"
            },
            "--allowedTools",
            if planning {
                "Read,Glob,Grep"
            } else {
                claude_build_tools
            },
        ]
        .into_iter()
        .map(String::from)
        .collect(),
    };
    let outcome = process::run(Spec {
        argv,
        cwd: (&run.workspace).into(),
        seconds: policy.command_seconds.min(290),
        environment: BTreeMap::new(),
        input: Some(prompt),
        directory: state.join("process"),
    })?;
    ensure!(
        outcome.success(),
        "host failed, timed out, or lost owner; inspect private process evidence and reconcile: {}",
        outcome.stderr
    );
    match run.host {
        Host::ClaudeCode => {
            let value: Value = serde_json::from_str(outcome.stdout.trim())
                .context("invalid Claude result; operation remains unresolved")?;
            ensure!(
                value["is_error"] == false,
                "Claude reported execution error: {}",
                value["subtype"]
            );
            ensure!(
                value
                    .get("permission_denials")
                    .and_then(Value::as_array)
                    .is_none_or(Vec::is_empty),
                "Claude tool permission denied; operation remains unresolved"
            );
            Ok(value["result"]
                .as_str()
                .context("Claude final result missing")?
                .to_string())
        }
        Host::Codex => {
            let events: Vec<Value> = outcome
                .stdout
                .lines()
                .map(serde_json::from_str)
                .collect::<Result<_, _>>()?;
            ensure!(
                events.iter().any(|e| e["type"] == "turn.completed")
                    && !events
                        .iter()
                        .any(|e| e["type"] == "turn.failed" || e["type"] == "error"),
                "Codex turn did not complete"
            );
            events
                .iter()
                .rev()
                .find(|e| e["type"] == "item.completed" && e["item"]["type"] == "agent_message")
                .and_then(|e| e["item"]["text"].as_str())
                .map(String::from)
                .context("Codex final result missing")
        }
    }
}
