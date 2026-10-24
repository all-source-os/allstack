use crate::{
    application::{
        self,
        ports::{Platform, Start},
    },
    domain::Host,
    infrastructure::{bootstrap, git, process, runtime::Runtime},
};
use anyhow::{Context, Result, ensure};
use clap::{Parser, Subcommand};
use serde_json::{Value, json};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Durable cross-host delivery workflows")]
pub struct Cli {
    #[arg(short = 'C', long, global = true, default_value = ".")]
    pub repository: PathBuf,
    #[command(subcommand)]
    pub command: Action,
}

#[derive(Subcommand)]
pub enum Action {
    Doctor {
        #[arg(long, default_value = "workflows/verification.toml")]
        policy: String,
    },
    Init {
        #[arg(long)]
        check: bool,
        #[arg(long, value_parser = ["rust"])]
        profile: Option<String>,
        #[arg(long, default_value = "workflows/verification.toml")]
        policy: String,
    },
    Start {
        #[arg(long)]
        request: String,
        #[arg(long)]
        policy: String,
        #[arg(long, default_value = "codex")]
        host: Host,
        #[arg(long)]
        allow_commit: bool,
        #[arg(long)]
        allow_publish: bool,
    },
    Plan {
        run: String,
        #[arg(long)]
        epoch: u64,
    },
    Build {
        run: String,
        #[arg(long)]
        epoch: u64,
    },
    Handoff {
        run: String,
        #[arg(long)]
        epoch: u64,
        #[arg(long)]
        to: Host,
    },
    Checkpoint {
        run: String,
        #[arg(long)]
        epoch: u64,
    },
    Recover {
        run: String,
        #[arg(long)]
        epoch: u64,
    },
    Resume {
        run: String,
        #[arg(long)]
        epoch: u64,
    },
    Verify {
        run: String,
        #[arg(long)]
        epoch: u64,
        #[arg(long)]
        message: String,
    },
    Ship {
        run: String,
        #[arg(long)]
        epoch: u64,
        #[arg(long)]
        title: String,
    },
    Status {
        run: String,
    },
    History {
        run: String,
    },
    Evidence {
        run: String,
    },
    Recall {
        query: String,
    },
    #[command(name = "__supervise", hide = true)]
    Supervise,
}

pub async fn execute(cli: Cli) -> Result<Value> {
    let repo = git::root(&cli.repository)?;
    match &cli.command {
        Action::Init {
            check,
            profile,
            policy,
        } => return bootstrap::initialize(&repo, *check, profile.as_deref(), policy),
        Action::Doctor { policy } => return bootstrap::doctor(&repo, policy),
        _ => {}
    }
    let runtime = Runtime::open(&repo).await?;
    let result = dispatch(&runtime, cli.command).await;
    let shutdown = runtime.shutdown().await;
    let value = result?;
    shutdown?;
    Ok(value)
}

async fn dispatch(p: &Runtime, action: Action) -> Result<Value> {
    let run = match action {
        Action::Doctor { .. } | Action::Init { .. } => {
            unreachable!("setup commands dispatch before opening run storage")
        }
        Action::Start {
            request,
            policy,
            host,
            allow_commit,
            allow_publish,
        } => {
            application::start(
                p,
                Start {
                    request,
                    policy,
                    host,
                    allow_commit,
                    allow_publish,
                },
            )
            .await?
        }
        Action::Plan { run, epoch } => application::plan(p, &run, epoch).await?,
        Action::Build { run, epoch } => application::build(p, &run, epoch).await?,
        Action::Handoff { run, epoch, to } => application::handoff(p, &run, epoch, to).await?,
        Action::Checkpoint { run, epoch } => application::checkpoint(p, &run, epoch).await?,
        Action::Recover { run, epoch } => application::recover(p, &run, epoch).await?,
        Action::Verify {
            run,
            epoch,
            message,
        } => application::verify(p, &run, epoch, &message).await?,
        Action::Ship { run, epoch, title } => application::ship(p, &run, epoch, &title).await?,
        Action::Status { run } => p.load(&run).await?,
        Action::History { run } => return Ok(serde_json::to_value(p.store.history(&run).await?)?),
        Action::Evidence { run } => {
            let run = p.load(&run).await?;
            let logs = run
                .verification
                .as_ref()
                .map(|v| v.logs.as_slice())
                .unwrap_or_default();
            let receipts = logs.iter().map(|key| {
                Ok(json!({"digest":key,"receipt":serde_json::from_slice::<Value>(&p.object(key)?)?}))
            }).collect::<Result<Vec<_>>>()?;
            return Ok(json!({"run":run,"gate_receipts":receipts}));
        }
        Action::Recall { query } => return Ok(p.recall(&query)),
        Action::Resume { run, epoch } => {
            let run = p.load(&run).await?;
            run.check_epoch(epoch).map_err(anyhow::Error::msg)?;
            ensure!(
                run.pending.is_none(),
                "run has unresolved operation; inspect and recover before executing"
            );
            p.claim_readback(&run)?;
            let request = String::from_utf8(p.object(&run.request)?)?;
            let plan = run
                .plan
                .as_ref()
                .map(|key| p.object(key).and_then(|b| Ok(String::from_utf8(b)?)))
                .transpose()?;
            return Ok(
                json!({"run":run,"request":request,"plan":plan,"recall":p.recall(&request)}),
            );
        }
        Action::Supervise => {
            process::supervise().context("supervisor failed")?;
            return Ok(Value::Null);
        }
    };
    Ok(serde_json::to_value(run)?)
}
