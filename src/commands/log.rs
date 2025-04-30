use crate::core::{
    object,
    repo::Repo,
    utils::{Commit, FileEntry},
};
use anyhow::{Context, Result};
use std::{fs, path::PathBuf};

pub fn run() -> Result<()> {
    let head = fs::read_to_string(Repo::head_path())?.trim().to_string();

    let commit_path = Repo::objects_dir().join(&head);
    let content =
        fs::read(&commit_path).with_context(|| format!("Failed to read commit object {}", head))?;

    let commit: Commit = serde_json::from_slice(&content)?;

    println!("commit {}\n", head);
    println!("Date: {}\n", commit.timestamp);
    println!("    {}\n", commit.message);

    println!("Staged files:");
    for file in commit.files {
        println!("    {} -> {}", file.path, file.hash);
    }

    Ok(())
}
