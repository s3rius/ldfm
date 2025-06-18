use std::path::PathBuf;

#[derive(Debug, Clone, clap::Parser)]
pub struct Cli {
    #[arg(name = "log-level", short, long, default_value = "info")]
    pub log_level: tracing::level_filters::LevelFilter,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Command {
    /// Initialize the local directory as a dotfiles repository.
    Init {
        /// Path to the local directory to initialize
        #[arg(long, short, default_value = "~/.config/dotfiles")]
        local_path: PathBuf,
        /// Path to the remote git repository to use for dotfiles.
        git_repo: Option<String>,
    },
    /// Apply the current state of dotfiles to the local system.
    Apply {
        /// Disable pulling the latest changes from the remote repository before applying
        #[arg(long, short, default_value = "false")]
        no_pull: bool,
    },
    /// Commit current state of dotfiles.
    Commit {
        /// Whether to push the changes to the remote repository
        #[arg(long, short, default_value = "false")]
        push: bool,
    },
    /// Add a file or a directory to the tracking list.
    Track {
        /// Path to the file or directory to track
        path: PathBuf,
    },
    /// Remove a file or a directory from the tracking list.
    Untrack {
        /// Path to the file or directory to untrack
        path: PathBuf,
    },
    /// List all tracked files and directories.
    List,
}
