use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use crate::core::{object, repo::Repo};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Contains information about a commit, including the commit message, timestamp, and files involved.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Commit {
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub files: Vec<FileEntry>,
}

/// Contains information about a file involved in a commit, including its path and hash.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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

/// Strips the Git-style blob header (`blob <len>\0`) from a blob file.
pub fn strip_git_blob_header(blob: &[u8]) -> Result<&[u8]> {
    let null_pos = blob
        .iter()
        .position(|&b| b == 0)
        .context("Invalid blob format: missing null separator")?;
    Ok(&blob[null_pos + 1..])
}

/// Walks through a directory and collects all files, ignoring those that match the provided patterns.
pub fn walk_dir(path: &Path, ignore: &[String]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if should_ignore(&entry_path, ignore) {
            continue;
        }

        if entry_path.is_dir() {
            files.extend(walk_dir(&entry_path, ignore)?);
        } else if entry_path.is_file() {
            files.push(entry_path);
        }
    }
    Ok(files)
}

/// Checks if a file or directory should be ignored based on the provided patterns.
pub fn should_ignore(path: &Path, patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();

    if path_str.contains(".minigit") || path_str.ends_with(".minigitingore") {
        return true;
    }

    for pattern in patterns {
        if pattern.ends_with('/') {
            if path_str.contains(pattern.trim_end_matches('/')) {
                return true;
            }
        } else if pattern.starts_with("*.") {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if pattern.trim_start_matches("*.") == ext {
                    return true;
                }
            }
        } else {
            if path_str.contains(pattern) {
                return true;
            }
        }
    }

    false
}

/// Stages a file by reading its contents, writing it to the object store, and updating the index.
pub fn stage_file(path: &Path) -> Result<usize> {
    let contents =
        fs::read(path).with_context(|| format!("Failed to read file: {}", path.display()))?;

    let hash = object::write_blob(&contents)?;

    let index_line = format!("{} {}\n", path.display(), hash);
    fs::OpenOptions::new()
        .append(true)
        .open(Repo::index_path())?
        .write_all(index_line.as_bytes())?;

    Ok(1)
}

/// Loads ignore patterns from the `.minigitingore` file.
pub fn load_ignore_patterns() -> Vec<String> {
    let ignore_path = Path::new(".minigitingore");
    if !ignore_path.exists() {
        return vec![];
    }

    std::fs::read_to_string(ignore_path)
        .map(|content| {
            content
                .lines()
                .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
                .map(|line| line.trim().to_string())
                .collect()
        })
        .unwrap_or_else(|_| vec![])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::init;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_commit_new() {
        let files = vec![
            FileEntry::new("file1.txt".to_string(), "hash1".to_string()),
            FileEntry::new("file2.txt".to_string(), "hash2".to_string()),
        ];
        let commit = Commit::new("Initial commit".to_string(), files.clone());

        assert_eq!(commit.message, "Initial commit");
        assert_eq!(commit.files, files);
    }

    #[test]
    fn test_strip_git_blob_header() {
        let blob = b"blob 10\0hello world";
        let stripped = strip_git_blob_header(blob).unwrap();
        assert_eq!(stripped, b"hello world");
    }

    #[test]
    fn test_walk_dir() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let files = walk_dir(dir.path(), &[]).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], file_path);
    }

    #[test]
    fn test_should_ignore() {
        let path = Path::new("test.txt");
        let patterns = vec!["*.txt".to_string()];
        assert!(should_ignore(path, &patterns));

        let path = Path::new("test.rs");
        assert!(!should_ignore(path, &patterns));
    }

    #[test]
    fn test_stage_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let mut file = std::fs::File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        let repo_dir = temp_dir.path().join(".minigit");
        std::fs::create_dir(&repo_dir).unwrap();

        std::env::set_current_dir(&temp_dir).unwrap();
        init::run().unwrap();

        let result = stage_file(&file_path);
        assert!(result.is_ok(), "Failed to stage file: {:?}", result.err());

        temp_dir.close().unwrap();
    }

    #[test]
    fn test_load_ignore_patterns() {
        let dir = tempdir().unwrap();
        let ignore_path = dir.path().join(".minigitingore");
        let mut file = File::create(&ignore_path).unwrap();
        writeln!(file, "*.log\n# Comment\n\n*.tmp").unwrap();

        std::env::set_current_dir(&dir).unwrap();
        let patterns = load_ignore_patterns();
        assert_eq!(patterns, vec!["*.log", "*.tmp"]);
    }
}
