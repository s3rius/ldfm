use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LdfmConfig {
    pub local_path: PathBuf,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RepoConfig {
    pub files: HashMap<String, String>,
}
