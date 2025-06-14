use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LdfmConfig {
    local_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoConfig {
    files: HashMap<String, String>,
    excludes: Vec<String>,
}
