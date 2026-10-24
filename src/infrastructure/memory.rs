use allsource_core::prime::Prime;
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::path::Path;

pub struct Memory(Prime);
impl Memory {
    pub async fn open(path: &Path) -> Result<Self> {
        let prime = Prime::open(path).await?;
        ensure!(
            !prime.is_read_only(),
            "Prime memory is owned by another writer"
        );
        Ok(Self(prime))
    }
    pub fn recall(&self, query: &str) -> Value {
        let terms: Vec<String> = query
            .split_whitespace()
            .filter(|s| s.len() > 3)
            .take(30)
            .map(str::to_lowercase)
            .collect();
        let mut nodes: Vec<_> = self
            .0
            .nodes_by_type("delivery")
            .into_iter()
            .filter_map(|node| {
                let text = node.properties.to_string().to_lowercase();
                let score = terms.iter().filter(|t| text.contains(t.as_str())).count();
                (score > 0).then_some((score, node))
            })
            .collect();
        nodes.sort_by_key(|(score, _)| std::cmp::Reverse(*score));
        json!({"retrieval":"lexical-prime-nodes", "authority":"context-only", "nodes": nodes.into_iter().take(5).map(|(_,n)| n).collect::<Vec<_>>()})
    }
    pub async fn record(&self, run: &str, phase: &str, source: &str, summary: &str) -> Result<()> {
        let key = format!("{run}:{phase}");
        if self
            .0
            .nodes_by_type("delivery")
            .iter()
            .any(|n| n.properties["key"] == key)
        {
            return Ok(());
        }
        self.0
            .add_node(
                "delivery",
                json!({"name":format!("{phase} {run}"),"domain":"allstack","key":key,
            "source":source,"summary":summary,"run_id":run}),
            )
            .await?;
        ensure!(
            self.0
                .nodes_by_type("delivery")
                .iter()
                .any(|n| n.properties["key"] == key),
            "Prime write not visible"
        );
        Ok(())
    }
    pub async fn shutdown(self) -> Result<()> {
        self.0.shutdown().await?;
        Ok(())
    }
}
