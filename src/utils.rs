use std::{
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::ExitStatus,
};

pub fn run_command<T: AsRef<OsStr>>(
    command: impl AsRef<OsStr>,
    args: impl IntoIterator<Item = T>,
) -> anyhow::Result<ExitStatus> {
    let status = std::process::Command::new(command)
        .args(args)
        .spawn()?
        .wait()?;
    Ok(status)
}

pub fn git_clone(repo_url: &str, local_path: &str) -> anyhow::Result<()> {
    let clone_code = run_command("git", ["clone", repo_url, local_path])?;
    if !clone_code.success() {
        anyhow::bail!(
            "Failed to clone repository from {} to {}: {:?}",
            repo_url,
            local_path,
            clone_code
        );
    }
    Ok(())
}

pub fn git_push(repo: &str) -> anyhow::Result<()> {
    let push_code = run_command("git", ["-C", repo, "push"])?;
    if !push_code.success() {
        anyhow::bail!(
            "Failed to push changes to remote repository: {:?}",
            push_code
        );
    }
    Ok(())
}

pub fn git_lfs_pull(repo: &str) -> anyhow::Result<()> {
    // Ensure git-lfs is installed and initialized
    let attrs = PathBuf::from(repo).join(".gitattributes");
    // Git LFS uses a .gitattributes file to track large files
    // if the file does not exist, we assume there are no LFS files to pull
    if !attrs.exists() {
        return Ok(());
    }
    let mut has_lfs = false;
    for line in BufReader::new(File::open(&attrs)?).lines().flatten() {
        if line.contains("filter=lfs") {
            has_lfs = true;
            break;
        }
    }
    tracing::info!("Found LFS tracked files.");
    if !has_lfs {
        tracing::info!("No LFS tracked files found in the repository.");
        return Ok(());
    }
    tracing::info!("Pulling LFS files for repository: {}", repo);
    let pull_code = run_command("git", ["-C", repo, "lfs", "pull"])?;
    if !pull_code.success() {
        anyhow::bail!("Failed to pull LFS files: {:?}", pull_code);
    }
    Ok(())
}

pub fn git_commit(repo: &str, message: &str) -> anyhow::Result<()> {
    let add_code = run_command("git", ["-C", repo, "add", "."])?;
    if !add_code.success() {
        anyhow::bail!("Failed to add changes to git index: {:?}", add_code);
    }
    let commit_code = run_command("git", ["-C", repo, "commit", "-m", message])?;
    if !commit_code.success() {
        anyhow::bail!("Failed to commit changes");
    }
    Ok(())
}
