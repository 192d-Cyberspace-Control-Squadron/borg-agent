use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct AgentConfig {
    pub server_addr: String,     // "https://server:50051"
    pub poll_seconds: u64,       // default interval
    pub cache_dir: PathBuf,      // e.g. C:\ProgramData\Department of War\borg\agent\cache
    pub device_id_path: PathBuf, // e.g. C:\ProgramData\Department of War\borg\agent\device_id.txt

    // mTLS client auth
    pub ca_pem_path: PathBuf,
    pub client_cert_pem_path: PathBuf,
    pub client_key_pem_path: PathBuf,
}

impl AgentConfig {
    pub fn load() -> Result<Self> {
        // MVP: load from JSON file next to exe or ProgramData.
        // You can swap to registry or env later.
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));

        let path = exe_dir.join("agent.config.json");
        let bytes = std::fs::read(&path)
            .with_context(|| format!("failed to read config: {}", path.display()))?;
        let cfg: AgentConfig = serde_json::from_slice(&bytes)
            .with_context(|| format!("invalid json config: {}", path.display()))?;

        Ok(cfg)
    }
}
