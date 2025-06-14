#[derive(Debug, Clone, clap::Parser)]
pub struct Cli {
    #[arg(name = "log-level", short, long, default_value = "info")]
    pub log_level: tracing::level_filters::LevelFilter,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Command {
    Init {
        /// Path to the local directory to initialize
        #[arg(long, short, default_value = "~/.config/dotfiles")]
        local_path: String,
    },
    Commit {
        #[arg(long, short, default_value = "false")]
        push: bool,
    },
    Add {},
    Remove {},
    List {},
}
