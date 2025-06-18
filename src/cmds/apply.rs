use crate::configs::LdfmConfig;

pub fn run(config: LdfmConfig) -> anyhow::Result<()> {
    let repo_config = config.get_repo_config()?;
    for (key, value) in repo_config.files.iter() {
        let Some(full_path) = simple_expand_tilde::expand_tilde(value) else {
            continue;
        };
        tracing::info!("{}: {:?}", key, full_path);
    }
    Ok(())
}
