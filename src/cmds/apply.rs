use fs_extra::dir::CopyOptions;

use crate::{configs::LdfmConfig, utils::git_pull};

pub fn run(config: LdfmConfig, no_pull: bool) -> anyhow::Result<()> {
    if !no_pull {
        tracing::info!("Pulling latest changes from remote repository...");
        git_pull(&config.local_path.display().to_string())?;
    }
    let repo_config = config.get_repo_config()?;
    for (key, value) in repo_config.files.iter() {
        let Some(to_path) = simple_expand_tilde::expand_tilde(value) else {
            continue;
        };
        let from_path = config.local_path.join(repo_config.get_local_path(key));
        tracing::info!("Copying {} -> {}", from_path.display(), to_path.display());
        fs_extra::copy_items(
            &[from_path],
            to_path
                .parent()
                .ok_or(anyhow::anyhow!("Cannot get a parent directory."))?,
            &CopyOptions::new().overwrite(true).copy_inside(true),
        )?;
    }
    Ok(())
}
