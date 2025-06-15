use std::{ffi::OsStr, process::ExitStatus};

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

pub fn git_commit(repo: &str, message: &str) -> anyhow::Result<()> {
    let add_code = run_command("git", ["-C", repo, "add", "."])?;
    if !add_code.success() {
        anyhow::bail!("Failed to add changes to git index: {:?}", add_code);
    }
    let commit_code = run_command("git", ["-C", repo, "commit", "-m", message])?;
    if !commit_code.success() {
        anyhow::bail!("Failed to commit changes: {:?}", commit_code);
    }
    Ok(())
}
