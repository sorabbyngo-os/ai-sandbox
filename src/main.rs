//! ai-sandbox — Sorabbyngo OS isolated agent execution

mod sandbox;
mod namespace;
mod seccomp;
mod cgroups;
mod fs;
mod network;
mod audit;

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("ai-sandbox ready");
    Ok(())
}
