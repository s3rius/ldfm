use crate::{
    configs::{LdfmConfig, RepoConfig},
    utils::{git_clone, git_commit},
};
use std::{io::Write, path::PathBuf};

pub fn run(
    config_path: PathBuf,
    local_path: PathBuf,
    git_repo: Option<String>,
) -> anyhow::Result<()> {
    let config = LdfmConfig {
        local_path: std::path::absolute(&local_path)?,
    };
    let mut config_file = std::fs::File::create(&config_path)?;
    if let Some(repo_uri) = git_repo {
        tracing::info!("Cloning repository from {}", repo_uri);
        git_clone(&repo_uri, &local_path.to_string_lossy())?;
    } else {
        tracing::info!(
            "Assuming local path {} is a git repository",
            local_path.display()
        );
    };
    tracing::info!("Writing configuration to {}", config_path.display());
    let config_str = toml::to_string_pretty(&config)?;
    config_file.write_all(config_str.as_bytes())?;

    let repo_config = local_path.join("ldfm.toml");
    if !repo_config.exists() {
        tracing::info!(
            "Creating repository configuration at {}",
            repo_config.display()
        );
        let repo_config_content = toml::to_string_pretty(&RepoConfig::default())?;
        let mut repo_config_file = std::fs::File::create(&repo_config)?;
        repo_config_file.write_all(repo_config_content.as_bytes())?;
        git_commit(&local_path.to_string_lossy(), "Initialized ldfm")?;
    }
    Ok(())
}
