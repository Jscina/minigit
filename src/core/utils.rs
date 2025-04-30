use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Contains information about a commit, including the commit message, timestamp, and files involved.
#[derive(Serialize, Deserialize)]
pub struct Commit {
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub files: Vec<FileEntry>,
}

/// Contains information about a file involved in a commit, including its path and hash.
#[derive(Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub hash: String,
}

impl Commit {
    pub fn new(message: String, files: Vec<FileEntry>) -> Self {
        Self {
            message,
            timestamp: Utc::now(),
            files,
        }
    }
}

impl FileEntry {
    pub fn new(path: String, hash: String) -> Self {
        Self { path, hash }
    }
}
