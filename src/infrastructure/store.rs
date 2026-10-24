use super::{Artifacts, digest, private_dir};
use crate::domain::{Event, Run, validate_transition};
use allsource_core::embedded::{Config, EmbeddedCore, IngestEvent, Query};
use anyhow::{Context, Result, ensure};
use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    os::unix::fs::OpenOptionsExt,
    path::Path,
};

pub struct Store {
    core: EmbeddedCore,
    _lock: File,
    pub artifacts: Artifacts,
}

impl Store {
    pub async fn open(root: &Path) -> Result<Self> {
        private_dir(root)?;
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .mode(0o600)
            .open(root.join("coordinator.lock"))?;
        lock.try_lock()
            .context("run coordinator busy; another operation owns this store")?;
        let core = EmbeddedCore::open(
            Config::builder()
                .data_dir(root.join("events"))
                .wal_sync_on_write(true)
                .build()?,
        )
        .await?;
        ensure!(
            core.durability_status().wal_enabled,
            "durable WAL is required"
        );
        Ok(Self {
            core,
            _lock: lock,
            artifacts: Artifacts(root.join("objects")),
        })
    }

    pub async fn history(&self, id: &str) -> Result<Vec<Event>> {
        uuid::Uuid::parse_str(id).context("invalid run id")?;
        let entity = format!("allstack-run-{id}");
        let page = self
            .core
            .query_page(Query::new().entity_id(&entity).limit(1001))
            .await?;
        ensure!(
            !page.has_more && page.total_count == page.events.len() && page.events.len() <= 1000,
            "incomplete or oversized run history"
        );
        let mut events = Vec::new();
        let mut ids = HashSet::new();
        for raw in page.events {
            ensure!(
                raw.entity_id == entity && raw.tenant_id == "default" && ids.insert(raw.id),
                "invalid source binding"
            );
            let event: Event = serde_json::from_value(raw.payload)?;
            ensure!(
                event.run.id == id
                    && raw.event_type == format!("allstack.workflow.v1.{}", event.kind),
                "invalid event binding"
            );
            events.push(event);
        }
        events.sort_by_key(|e| e.run.revision);
        let mut previous = String::new();
        let mut prior = None;
        let mut operations = HashSet::new();
        for event in &events {
            ensure!(
                event.previous == previous && operations.insert(&event.operation),
                "broken event chain or duplicate operation"
            );
            validate_transition(prior, event).map_err(anyhow::Error::msg)?;
            previous = digest(&serde_json::to_vec(event)?);
            prior = Some(&event.run);
        }
        Ok(events)
    }

    pub async fn load(&self, id: &str) -> Result<Run> {
        self.history(id)
            .await?
            .pop()
            .map(|e| e.run)
            .context("run not found")
    }

    pub async fn save(&self, run: &mut Run, kind: &str) -> Result<()> {
        let events = self.history(&run.id).await?;
        let last = events.last();
        ensure!(
            last.map_or(0, |e| e.run.revision) == run.revision,
            "stale run revision"
        );
        let mut next = run.clone();
        next.revision += 1;
        let event = Event {
            schema: 1,
            operation: uuid::Uuid::new_v4().to_string(),
            previous: last
                .map(|e| serde_json::to_vec(e).map(|b| digest(&b)))
                .transpose()?
                .unwrap_or_default(),
            kind: kind.into(),
            run: next,
        };
        validate_transition(last.map(|e| &e.run), &event).map_err(anyhow::Error::msg)?;
        self.core
            .ingest(IngestEvent {
                entity_id: &format!("allstack-run-{}", run.id),
                event_type: &format!("allstack.workflow.v1.{kind}"),
                payload: serde_json::to_value(&event)?,
                metadata: None,
                tenant_id: None,
            })
            .await
            .context("append outcome uncertain; inspect run before retry")?;
        let actual = self
            .history(&run.id)
            .await?
            .pop()
            .context("append missing after acknowledgement")?;
        ensure!(
            digest(&serde_json::to_vec(&actual)?) == digest(&serde_json::to_vec(&event)?),
            "append acknowledgement mismatch"
        );
        *run = actual.run;
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<()> {
        self.core.shutdown().await?;
        Ok(())
    }
}
