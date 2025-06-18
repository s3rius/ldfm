# Dotfile manager

This project is a helper script for managing dotfiles. It's used to simply track files,
udpate then and pull them from a remote repository.


## Installation

```bash
cargo install --locked ldfm
```

# Usage

At first you need to initialize global config for the tool by running

```bash
ldfm init "git@.../your-repo.git"
```

This will create a `~/.config/ldfm/config.toml` file with a path to local folder with cloned repository.
If repo does not have an `ldfm.toml` it will be created with default values.


## Tracking files

In order to start tracking files, you need to run 

```bash
ldfm track <path>
```

where `<path>` is a path to the file you want to track. It will path to the file and will be using it for updating repo content.

You can alsu track directories, in this case all files in the directory will be tracked.
If you want to exclude some files, you can create a `.gitignore` file in the directory with a list of files to ignore.

If you want to stop tracking file, you need to `untrack` it

```bash
ldfm untrack <path>
```

Where path is a path to the file you want to stop tracking. It will remove the file from the list of tracked files and will remove it from the repo.

### Updating files

ldfm will not automatically update files in the repo. In order to sync your local changes with the repo, you need to run this command manually:

```bash
ldfm commit
```

You can also add `-p` option to push changes to the remote repository after committing:

```bash
ldfm commit -p
```

### Applying changes

In order to apply files from your remote repository, run 
```bash
ldfm apply
```
