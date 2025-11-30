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
    let mut dotfiles_path = config.local_path.clone();
    if let Some(df_root) = &repo_config.root {
        dotfiles_path = dotfiles_path.join(df_root);
        std::fs::remove_dir_all(&dotfiles_path)?;
        std::fs::create_dir_all(&dotfiles_path)?;
    } else {
        dotfiles_path = config.local_path.clone();
        for entry in std::fs::read_dir(&dotfiles_path)? {
            let entry = entry?.path();
            let Some(file_name) = entry.file_name() else {
                continue;
            };
            if [".git", "ldfm.toml"].contains(&file_name.to_string_lossy().as_ref()) {
                continue;
            }
            if entry.is_dir() {
                std::fs::remove_dir_all(entry)?;
            } else {
                std::fs::remove_file(entry)?;
            }
        }
    }
    let df_contents = fs_extra::dir::get_dir_content(&dotfiles_path)?;
    // Clean up the dotfiles directory by removing files and directories
    let git_dir = config.local_path.join(".git").display().to_string();
    let ldfm_conig = config.local_path.join("ldfm.toml").display().to_string();
    df_contents
        .files
        .iter()
        .filter(|filepath| {
            // We filter out files that are in the .git directory or the ldfm.toml config file
            !(filepath.starts_with(git_dir.as_str()) || filepath.starts_with(ldfm_conig.as_str()))
        })
        .for_each(|file| {
            tracing::debug!("Removing file: {}", file);
            fs_extra::remove_items(&[file]).ok();
        });
    df_contents
        .directories
        .iter()
        .filter(|dir| {
            // We filter out files that are in the .git directory or the dotfiles directory iteslf.
            !(dir.starts_with(git_dir.as_str()) || dir == &&config.local_path.display().to_string())
        })
        .for_each(|dir| {
            tracing::info!("Removing directory: {}", dir);
            fs_extra::remove_items(&[dir]).ok();
        });
    std::fs::create_dir_all(dotfiles_path)?;
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
