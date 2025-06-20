use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RepoConfig {
    pub root: Option<PathBuf>,
    pub files: HashMap<String, String>,
}

impl RepoConfig {
    fn format_full_path(&self, path: &PathBuf) -> anyhow::Result<PathBuf> {
        let mut file_path = std::path::absolute(path)?;
        file_path = file_path.canonicalize().unwrap_or(file_path);
        let home_dir = std::env::home_dir().ok_or(anyhow::anyhow!("Cannot get home directory"))?;
        let relative_to_home = pathdiff::diff_paths(file_path, home_dir).ok_or(anyhow::anyhow!(
            "Cannot calculate path relative to home directory."
        ))?;
        Ok(PathBuf::from("~").join(relative_to_home))
    }

    pub fn get_local_path(&self, key: &str) -> PathBuf {
        if let Some(root) = &self.root {
            root.join(key)
        } else {
            PathBuf::from(key)
        }
    }

    pub fn track_file(&mut self, target: &PathBuf) -> anyhow::Result<()> {
        let filename = target
            .file_name()
            .ok_or(anyhow::anyhow!("Cannot get filename from given path"))?
            .to_string_lossy()
            .to_string();
        let dotfile_path = self.format_full_path(target)?;
        for (key, value) in self.files.iter() {
            if value == &dotfile_path.display().to_string() {
                tracing::info!(
                    "File {} is already tracked with the same path: {}",
                    key,
                    value
                );
                return Ok(());
            }
        }
        let value = self
            .files
            .entry(filename.clone())
            .or_insert_with(|| dotfile_path.display().to_string());
        if value == &dotfile_path.display().to_string() {
            return Ok(());
        }
        tracing::warn!(
            "File {} is already tracked with a different path: {}",
            filename,
            value
        );
        let mut has_key = true;
        let mut prefix = 0;
        while has_key {
            let new_key = format!("{}-{}", prefix, filename);
            if self.files.contains_key(&new_key) {
                prefix += 1;
            } else {
                has_key = false;
                self.files
                    .insert(new_key, dotfile_path.display().to_string());
            }
        }

        Ok(())
    }

    /// Untrack a file from the repository configuration.
    ///
    /// Target is the path to the file to untrack.
    /// Returns the key of the file if it was successfully untracked, or None if it was not tracked.
    pub fn untrack_file(&mut self, target: &PathBuf) -> anyhow::Result<Option<String>> {
        let dotfile_path = self.format_full_path(target)?.display().to_string();
        let mut found_key = None;
        for (key, value) in self.files.iter() {
            if value == &dotfile_path {
                found_key = Some(key.clone());
            }
        }
        if let Some(key) = found_key {
            self.files.remove(&key);
            return Ok(Some(key.to_string()));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::RepoConfig;

    #[test]
    fn track_file() {
        let mut conf = RepoConfig::default();
        let target = std::env::home_dir().unwrap().join(".config/meme.txt");
        conf.track_file(&target).unwrap();
        assert!(conf.files.contains_key("meme.txt"));
        assert_eq!(conf.files.get("meme.txt").unwrap(), "~/.config/meme.txt");
    }

    #[test]
    fn track_file_twice() {
        let mut conf = RepoConfig::default();
        let target = std::env::home_dir().unwrap().join(".config/meme.txt");
        conf.track_file(&target).unwrap();
        conf.track_file(&target).unwrap();
        assert!(conf.files.len() == 1);
        assert!(conf.files.contains_key("meme.txt"));
        assert_eq!(conf.files.get("meme.txt").unwrap(), "~/.config/meme.txt");
    }

    #[test]
    fn track_file_same_name() {
        let mut conf = RepoConfig::default();
        let target = std::env::home_dir().unwrap().join(".config/hehe/meme.txt");
        let target2 = std::env::home_dir().unwrap().join(".config/ohoh/meme.txt");
        conf.track_file(&target).unwrap();
        conf.track_file(&target2).unwrap();
        assert!(conf.files.len() == 2);
        assert!(conf.files.contains_key("meme.txt"));
        assert!(conf.files.contains_key("0-meme.txt"));
        assert_eq!(
            conf.files.get("meme.txt").unwrap(),
            "~/.config/hehe/meme.txt"
        );
        assert_eq!(
            conf.files.get("0-meme.txt").unwrap(),
            "~/.config/ohoh/meme.txt"
        );
    }

    #[test]
    fn untrack_file() {
        let mut conf = RepoConfig::default();
        let target = std::env::home_dir().unwrap().join(".config/meme.txt");
        conf.track_file(&target).unwrap();
        conf.untrack_file(&target).unwrap();
        assert_eq!(conf.files.len(), 0);
    }
}
