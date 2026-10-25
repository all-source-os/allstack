use crate::domain::{Host, Policy, Publication, Run, Verification};
use anyhow::Result;
use serde_json::Value;

pub struct Start {
    pub native: bool,
    pub workflow: String,
    pub request: String,
    pub policy: String,
    pub host: Host,
    pub allow_commit: bool,
    pub allow_publish: bool,
}

#[allow(async_fn_in_trait)]
pub trait Platform {
    async fn load(&self, id: &str) -> Result<Run>;
    async fn save(&self, run: &mut Run, kind: &str) -> Result<()>;
    fn initialize(&self, input: &Start) -> Result<Run>;
    fn object(&self, key: &str) -> Result<Vec<u8>>;
    fn put(&self, bytes: &[u8]) -> Result<String>;
    fn checkpoint(&self, run: &Run) -> Result<String>;
    fn restore(&self, run: &Run, snapshot: &str, epoch: u64) -> Result<String>;
    fn create_task(&self, run: &Run) -> Result<String>;
    fn claim(&self, run: &Run, previous: Option<&Run>) -> Result<()>;
    fn claim_readback(&self, run: &Run) -> Result<()>;
    fn complete_task(&self, run: &Run) -> Result<()>;
    fn recall(&self, query: &str) -> Value;
    async fn remember(&self, run: &Run, phase: &str, source: &str, summary: &str) -> Result<()>;
    fn invoke(&self, run: &Run, prompt: &str, planning: bool) -> Result<String>;
    fn write_plan(&self, run: &Run, text: &str) -> Result<String>;
    fn prepare(&self, run: &Run, message: &str) -> Result<String>;
    fn candidate(&self, run: &Run) -> Result<Verification>;
    fn gates(&self, run: &Run, policy: &Policy) -> Result<Vec<String>>;
    fn publish(&self, run: &Run, title: &str) -> Result<Publication>;
    async fn cleanup(&self, run: &Run) -> Result<()>;
}
