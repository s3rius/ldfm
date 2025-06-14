use clap::Parser;

mod cli;
mod config;

fn main() -> anyhow::Result<()> {
    let args = cli::Cli::parse();
    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(args.log_level)
        .init();
    let Some(config_dir) = dirs::config_dir() else {
        anyhow::bail!("Could not find config directory");
    };
    tracing::info!("Starting application with log level: {:?}", args.log_level);
    tracing::warn!("Config directory: {:?}", config_dir);
    tracing::error!("Current command: {:?}", args.command);
    Ok(())
}
