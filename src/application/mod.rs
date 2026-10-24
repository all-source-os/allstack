pub mod ports;
use crate::domain::{Host, Policy, Run, Stage};
use anyhow::{Context, Result, ensure};
use ports::{Platform, Start};

pub async fn start(p: &impl Platform, input: Start) -> Result<Run> {
    let mut run = p.initialize(&input)?;
    p.save(&mut run, "started").await?;
    run.pending = Some("task_setup".into());
    p.save(&mut run, "intent").await?;
    run.task = Some(p.create_task(&run)?);
    p.save(&mut run, "task_bound").await?;
    p.claim(&run, None)?;
    run.pending = None;
    p.save(&mut run, "task_claimed").await?;
    Ok(run)
}

pub async fn plan(p: &impl Platform, id: &str, epoch: u64) -> Result<Run> {
    let mut run = ready(p, id, epoch).await?;
    ensure!(run.stage == Stage::Planning, "planning stage required");
    let request = String::from_utf8(p.object(&run.request)?)?;
    let recall = p.recall(&request);
    let method =
        String::from_utf8(p.object(run.methods.get("plan").context("plan method absent")?)?)?;
    run.pending = Some("host_plan".into());
    p.save(&mut run, "intent").await?;
    let text = p.invoke(
        &run,
        &format!("{method}\nREQUEST:\n{request}\nRECALL:\n{recall}"),
        true,
    )?;
    ensure!(!text.trim().is_empty(), "host returned no plan");
    ensure!(
        text.len() <= 128 * 1024,
        "host plan exceeds supported context limit"
    );
    run.plan = Some(p.put(text.as_bytes())?);
    let source = p.write_plan(&run, &text)?;
    run.checkpoint = p.checkpoint(&run)?;
    run.stage = Stage::Building;
    run.pending = None;
    run.memory_pending = true;
    p.save(&mut run, "planned").await?;
    if p.remember(
        &run,
        "plan",
        &source,
        "Indexed accepted delivery plan; open source document before using recalled requirements.",
    )
    .await
    .is_ok()
    {
        run.memory_pending = false;
        p.save(&mut run, "memory_indexed").await?;
    }
    Ok(run)
}

pub async fn build(p: &impl Platform, id: &str, epoch: u64) -> Result<Run> {
    let mut run = ready(p, id, epoch).await?;
    ensure!(run.stage == Stage::Building, "building stage required");
    let plan = String::from_utf8(p.object(run.plan.as_deref().context("plan missing")?)?)?;
    let request = String::from_utf8(p.object(&run.request)?)?;
    let method =
        String::from_utf8(p.object(run.methods.get("build").context("build method absent")?)?)?;
    run.pending = Some("host_build".into());
    p.save(&mut run, "intent").await?;
    p.invoke(
        &run,
        &format!("{method}\nREQUEST:\n{request}\nPLAN:\n{plan}"),
        false,
    )?;
    run.checkpoint = p.checkpoint(&run)?;
    run.pending = None;
    p.save(&mut run, "built").await?;
    Ok(run)
}

pub async fn handoff(p: &impl Platform, id: &str, epoch: u64, host: Host) -> Result<Run> {
    let mut run = ready(p, id, epoch).await?;
    ensure!(
        run.host != host && !matches!(run.stage, Stage::Shipping | Stage::Shipped),
        "handoff needs another host before shipping"
    );
    let previous = run.clone();
    let checkpoint = p.checkpoint(&run)?;
    run.pending = Some("handoff".into());
    run.transfer = Some(crate::domain::Transfer {
        from_host: run.host,
        from_epoch: run.epoch,
        to_host: host,
    });
    run.checkpoint = checkpoint;
    p.save(&mut run, "intent").await?;
    let workspace = p.restore(&run, &run.checkpoint, run.epoch + 1)?;
    run.epoch += 1;
    run.host = host;
    run.workspace = workspace;
    run.verification = None;
    if run.stage == Stage::Verifying {
        run.stage = Stage::Building;
    }
    p.save(&mut run, "ownership_changed").await?;
    p.claim(&run, Some(&previous))?;
    run.pending = None;
    run.transfer = None;
    p.save(&mut run, "handoff_completed").await?;
    Ok(run)
}

pub async fn checkpoint(p: &impl Platform, id: &str, epoch: u64) -> Result<Run> {
    let mut run = p.load(id).await?;
    run.check_epoch(epoch).map_err(anyhow::Error::msg)?;
    ensure!(
        run.pending.is_none() || matches!(run.pending.as_deref(), Some("commit" | "verification")),
        "unresolved external action blocks checkpoint repair"
    );
    p.claim_readback(&run)?;
    if run.pending.as_deref() == Some("commit") {
        run.checkout_snapshot = Some(run.checkpoint.clone());
    }
    run.checkpoint = p.checkpoint(&run)?;
    run.pending = None;
    run.verification = None;
    if run.stage == Stage::Verifying {
        run.stage = Stage::Building;
    }
    p.save(&mut run, "checkpointed").await?;
    Ok(run)
}

pub async fn recover(p: &impl Platform, id: &str, epoch: u64) -> Result<Run> {
    let mut run = p.load(id).await?;
    run.check_epoch(epoch).map_err(anyhow::Error::msg)?;
    match run.pending.as_deref() {
        Some("task_setup") => {
            run.task = Some(p.create_task(&run)?);
            p.save(&mut run, "task_bound").await?;
            p.claim(&run, None)?;
        }
        Some("host_plan" | "host_build") => {
            // Recovery preserves inspected edits; it never re-executes an uncertain host call.
            run.checkpoint = p.checkpoint(&run)?;
        }
        Some("handoff") => {
            let transfer = run.transfer.clone().context("handoff contract missing")?;
            let mut previous = run.clone();
            previous.host = transfer.from_host;
            previous.epoch = transfer.from_epoch;
            if run.epoch == transfer.from_epoch {
                run.workspace = p.restore(&run, &run.checkpoint, run.epoch + 1)?;
                run.epoch += 1;
                run.host = transfer.to_host;
                run.verification = None;
                if run.stage == Stage::Verifying {
                    run.stage = Stage::Building;
                }
                p.save(&mut run, "ownership_changed").await?;
            }
            p.claim(&run, Some(&previous))?;
            run.transfer = None;
        }
        None => {
            p.claim_readback(&run)?;
            if run.memory_pending {
                p.remember(&run,"plan",&format!("docs/plans/allstack-{}.md",run.id),"Indexed accepted delivery plan; open source document before using recalled requirements.").await?;
                run.memory_pending = false;
                p.save(&mut run, "memory_indexed").await?;
            }
            return Ok(run);
        }
        Some(other) => {
            anyhow::bail!("operation {other} requires its destination-specific resume command")
        }
    }
    run.pending = None;
    p.save(&mut run, "reconciled").await?;
    Ok(run)
}

pub async fn verify(p: &impl Platform, id: &str, epoch: u64, message: &str) -> Result<Run> {
    let mut run = p.load(id).await?;
    run.check_epoch(epoch).map_err(anyhow::Error::msg)?;
    ensure!(
        run.pending.is_none() || matches!(run.pending.as_deref(), Some("commit" | "verification")),
        "unresolved action blocks verification"
    );
    p.claim_readback(&run)?;
    ensure!(
        matches!(run.stage, Stage::Building | Stage::Verifying),
        "build stage required"
    );
    ensure!(
        run.allow_commit,
        "commit authority absent; start must explicitly authorize commit"
    );
    if run.pending.is_none() {
        run.checkpoint = p.checkpoint(&run)?;
    }
    run.stage = Stage::Verifying;
    run.verification = None;
    run.pending = Some("commit".into());
    p.save(&mut run, "intent").await?;
    run.prepared_commit = Some(p.prepare(&run, message)?);
    run.checkout_snapshot = Some(run.checkpoint.clone());
    run.pending = Some("verification".into());
    p.save(&mut run, "intent").await?;
    let before = p.candidate(&run)?;
    let policy: Policy = toml::from_str(&String::from_utf8(p.object(&run.policy)?)?)?;
    policy.validate().map_err(anyhow::Error::msg)?;
    let logs = p.gates(&run, &policy)?;
    let after = p.candidate(&run)?;
    ensure!(
        before.commit == after.commit
            && before.snapshot == after.snapshot
            && before.environment == after.environment,
        "candidate or environment changed during verification"
    );
    run.verification = Some(crate::domain::Verification { logs, ..after });
    run.pending = None;
    p.save(&mut run, "verified").await?;
    Ok(run)
}

pub async fn ship(p: &impl Platform, id: &str, epoch: u64, title: &str) -> Result<Run> {
    let mut run = p.load(id).await?;
    ensure!(run.epoch == epoch, "stale host epoch");
    if run.stage == Stage::Shipped {
        p.cleanup(&run).await?;
        return Ok(run);
    }
    run.check_epoch(epoch).map_err(anyhow::Error::msg)?;
    ensure!(
        run.allow_publish && matches!(run.stage, Stage::Verifying | Stage::Shipping),
        "publication authority or verified stage missing"
    );
    ensure!(
        run.pending.is_none() || run.pending.as_deref() == Some("publication"),
        "unresolved action blocks publication"
    );
    let verified = run.verification.as_ref().context("verification missing")?;
    if run.publication.is_none() {
        p.claim_readback(&run)?;
    }
    let actual = p.candidate(&run)?;
    ensure!(
        actual.commit == verified.commit
            && actual.snapshot == verified.snapshot
            && actual.environment == verified.environment
            && actual.policy == verified.policy
            && actual.workflow == verified.workflow,
        "verification stale"
    );
    run.stage = Stage::Shipping;
    run.pending = Some("publication".into());
    p.save(&mut run, "intent").await?;
    let mut publication = p.publish(&run, title)?;
    run.publication = Some(publication.clone());
    p.save(&mut run, "destination_confirmed").await?;
    p.complete_task(&run)?;
    publication.task_completed = true;
    run.publication = Some(publication.clone());
    p.save(&mut run, "task_completed").await?;
    p.remember(
        &run,
        "shipped",
        &publication.pr,
        "Exact verified commit and remote PR head read back; task completion confirmed.",
    )
    .await?;
    publication.memory_indexed = true;
    run.publication = Some(publication);
    run.stage = Stage::Shipped;
    run.pending = None;
    p.save(&mut run, "shipped").await?;
    p.cleanup(&run).await?;
    Ok(run)
}

async fn ready(p: &impl Platform, id: &str, epoch: u64) -> Result<Run> {
    let run = p.load(id).await?;
    run.check_epoch(epoch).map_err(anyhow::Error::msg)?;
    run.require_resolved().map_err(anyhow::Error::msg)?;
    p.claim_readback(&run)?;
    Ok(run)
}
