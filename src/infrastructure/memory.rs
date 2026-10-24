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
    pub async fn record(
        &self,
        run: &str,
        workflow: &str,
        phase: &str,
        source: &str,
        summary: &str,
    ) -> Result<()> {
        let key = format!("{run}:{phase}");
        let existing = self.bound(&key);
        ensure!(
            existing.len() <= 1,
            "Prime key {key} is bound to {} nodes; reconciliation required before indexing",
            existing.len()
        );
        if let Some(node) = existing.first() {
            ensure!(
                binding_matches(node, &key, run, workflow, source, summary),
                "Prime key {key} already binds a different workflow, source, summary, or run"
            );
            return Ok(());
        }
        self.0
            .add_node(
                "delivery",
                json!({"name":format!("{phase} {run}"),"domain":"allstack","key":key,
            "source":source,"summary":summary,"run_id":run,"workflow_id":workflow}),
            )
            .await?;
        ensure!(
            single_complete_binding(&self.bound(&key), &key, run, workflow, source, summary),
            "Prime write not visible as a single complete binding for key {key}"
        );
        Ok(())
    }
    fn bound(&self, key: &str) -> Vec<Value> {
        self.0
            .nodes_by_type("delivery")
            .into_iter()
            .map(|node| node.properties)
            .filter(|props| node_str(props, "key") == Some(key))
            .collect()
    }
    pub async fn shutdown(self) -> Result<()> {
        self.0.shutdown().await?;
        Ok(())
    }
}

fn node_str<'a>(props: &'a Value, field: &str) -> Option<&'a str> {
    props.get(field).and_then(Value::as_str)
}

// Legacy nodes omit workflow_id; malformed explicit values cannot use that fallback.
fn stored_workflow(props: &Value) -> Option<&str> {
    match props.get("workflow_id") {
        None => Some("delivery"),
        Some(Value::String(workflow)) => Some(workflow.as_str()),
        Some(_) => None,
    }
}

pub fn binding_matches(
    props: &Value,
    key: &str,
    run: &str,
    workflow: &str,
    source: &str,
    summary: &str,
) -> bool {
    node_str(props, "key") == Some(key)
        && node_str(props, "run_id") == Some(run)
        && stored_workflow(props) == Some(workflow)
        && node_str(props, "source") == Some(source)
        && node_str(props, "summary") == Some(summary)
}

pub fn single_complete_binding(
    nodes: &[Value],
    key: &str,
    run: &str,
    workflow: &str,
    source: &str,
    summary: &str,
) -> bool {
    nodes.len() == 1 && binding_matches(&nodes[0], key, run, workflow, source, summary)
}
