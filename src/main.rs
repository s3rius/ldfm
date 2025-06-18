use clap::Parser;

use crate::{cli::Cli, configs::LdfmConfig};

mod cli;
mod cmds;
mod configs;
mod utils;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    tracing_subscriber::fmt()
        .with_level(true)
        .with_writer(std::io::stderr)
        .with_max_level(args.log_level)
        .init();
    let Some(config_dir) = dirs::config_dir().map(|p| p.join("ldfm")) else {
        anyhow::bail!("Could not find user config directory.");
    };
    std::fs::create_dir_all(&config_dir)?;
    let config_file = config_dir.join("config.toml");
    let config = if config_file.exists() {
        let contents = std::fs::read_to_string(&config_file)?;
        let config: LdfmConfig = toml::from_str(&contents)?;
        Some(config)
    } else {
        None
    };
    let initialization_error_message = "ldfm was not initialized. Please run `ldfm init` first.";

    match args.command {
        cli::Command::Init {
            local_path,
            git_repo,
        } => {
            let local_path = simple_expand_tilde::expand_tilde(&local_path)
                .ok_or(anyhow::anyhow!("Cannot expand tilde from path"))?;
            cmds::init::run(config_file, local_path, git_repo)?;
        }
        cli::Command::Commit { push } => {
            let Some(config) = config else {
                anyhow::bail!(initialization_error_message)
            };
            cmds::track::sync(config, push)?;
        }
        cli::Command::Track { path } => {
            let Some(config) = config else {
                anyhow::bail!(initialization_error_message)
            };
            cmds::track::add(config, path)?;
        }
        cli::Command::Untrack { path } => {
            let Some(config) = config else {
                anyhow::bail!(initialization_error_message)
            };
            cmds::track::remove(config, path)?;
        }

        cli::Command::List => {
            let Some(config) = config else {
                anyhow::bail!(initialization_error_message)
            };
            cmds::track::list(config)?;
        }
    }

    Ok(())
}
