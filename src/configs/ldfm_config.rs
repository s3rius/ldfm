use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::configs::RepoConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LdfmConfig {
    pub local_path: PathBuf,
}

impl LdfmConfig {
    pub fn get_repo_config_path(&self) -> PathBuf {
        self.local_path.join("ldfm.toml")
    }

    pub fn get_repo_config(&self) -> anyhow::Result<RepoConfig> {
        let config_path = self.get_repo_config_path();
        if config_path.exists() {
            let config_data = std::fs::read_to_string(config_path)?;
            let repo_config = toml::from_str(&config_data)?;
            Ok(repo_config)
        } else {
            anyhow::bail!("Config file not found at {}", config_path.display());
        }
    }
}
