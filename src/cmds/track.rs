use std::path::PathBuf;

use fs_extra::dir::CopyOptions;

use crate::{
    configs::LdfmConfig,
    utils::{git_commit, git_push},
};

pub fn add(config: LdfmConfig, path: PathBuf) -> anyhow::Result<()> {
    let mut repo_config = config.get_repo_config()?;
    let target_path = std::path::absolute(
        simple_expand_tilde::expand_tilde(path)
            .ok_or(anyhow::anyhow!("Cannot expand tilde from path"))?,
    )?;
    tracing::info!("Tracking file: {}", target_path.display());
    repo_config.track_file(&target_path)?;
    std::fs::write(
        config.get_repo_config_path(),
        toml::to_string_pretty(&repo_config)?,
    )
    .map_err(|e| anyhow::anyhow!("Failed to write config file: {}", e))?;
    Ok(())
}

pub fn remove(config: LdfmConfig, path: PathBuf) -> anyhow::Result<()> {
    let mut repo_config = config.get_repo_config()?;
    let target_path = std::path::absolute(
        simple_expand_tilde::expand_tilde(path)
            .ok_or(anyhow::anyhow!("Cannot expand tilde from path"))?,
    )?;
    tracing::info!("Untracking file: {}", target_path.display());
    if let Some(removed_file) = repo_config.untrack_file(&target_path)? {
        let full_file_path = config
            .local_path
            .join(repo_config.get_local_path(&removed_file));
        if full_file_path.exists() {
            tracing::info!(
                "Removing file from local path: {}",
                full_file_path.display()
            );
            std::fs::remove_dir_all(full_file_path).ok();
        } else {
            tracing::warn!(
                "File {} does not exist in local path, skipping removal.",
                full_file_path.display()
            );
        }
    }
    std::fs::write(
        config.get_repo_config_path(),
        toml::to_string_pretty(&repo_config)?,
    )?;
    Ok(())
}

pub fn list(config: LdfmConfig) -> anyhow::Result<()> {
    let repo_config = config.get_repo_config()?;
    if repo_config.files.is_empty() {
        tracing::info!("No files are currently tracked.");
    } else {
        tracing::info!("Tracked files:");
        for (_, value) in &repo_config.files {
            println!("{}", value);
        }
    }
    Ok(())
}

pub fn sync(config: LdfmConfig, push: bool) -> anyhow::Result<()> {
    let repo_config = config.get_repo_config()?;
    for (key, value) in &repo_config.files {
        let target_path = config.local_path.join(repo_config.get_local_path(key));
        let actual_path = simple_expand_tilde::expand_tilde(&value)
            .ok_or(anyhow::anyhow!("Cannot get home directory"))?;
        if !actual_path.exists() {
            tracing::warn!(
                "File {} does not exist at the expected path: {}",
                key,
                actual_path.display()
            );
            continue;
        }
        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent).ok();
            if !parent.exists() {
                tracing::info!("Creating directory for target path: {}", parent.display());
                std::fs::create_dir_all(parent)?;
            }
        }
        tracing::info!(
            "Copying file from {} to {}",
            actual_path.display(),
            target_path.display()
        );
        fs_extra::copy_items(
            &[actual_path],
            target_path.parent().unwrap(),
            &CopyOptions::new().overwrite(true).copy_inside(true),
        )?;
    }
    let repo_path = config.local_path.display().to_string();
    git_commit(&repo_path, "Dotfiles sync.")?;
    if push {
        tracing::info!("Pushing changes to remote repository.");
        git_push(&repo_path)?;
    }
    Ok(())
}
