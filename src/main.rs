use std::{fs::File, io::Read};

use clap::Parser;

use crate::cli::Cli;

mod cli;
mod cmds;
mod config;
mod utils;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(args.log_level)
        .init();
    let Some(config_dir) = dirs::config_dir().map(|p| p.join("ldfm")) else {
        anyhow::bail!("Could not find user config directory.");
    };
    std::fs::create_dir_all(&config_dir)?;
    let config_file = config_dir.join("config.toml");
    let config = if config_file.exists() {
        let contents = std::fs::read_to_string(&config_file)?;
        Some(contents)
    } else {
        None
    };

    match args.command {
        cli::Command::Init {
            local_path,
            git_repo,
        } => {
            let local_path = simple_expand_tilde::expand_tilde(&local_path)
                .ok_or(anyhow::anyhow!("Cannot expand tilde from path"))?;
            cmds::init::run(config_file, local_path, git_repo)?;
        }
        cli::Command::Commit { push } => todo!(),
        cli::Command::Track {} => todo!(),
        cli::Command::Untrack {} => todo!(),
        cli::Command::List {} => todo!(),
    }

    Ok(())
}
