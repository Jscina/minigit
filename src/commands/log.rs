use crate::core::{
    repo::Repo,
    utils::{self, Commit},
};
use anyhow::{Context, Result};
use std::fs;

pub fn run() -> Result<()> {
    let head = fs::read_to_string(Repo::head_path())?.trim().to_string();

    let commit_path = Repo::objects_dir().join(&head);
    let blob =
        fs::read(&commit_path).with_context(|| format!("Failed to read commit object {}", head))?;

    let content = utils::strip_git_blob_header(&blob)?;
    let commit: Commit = serde_json::from_slice(content)?;

    println!("commit {}\n", head);
    println!("Date: {}\n", commit.timestamp);
    println!("    {}\n", commit.message);

    println!("Staged files:");
    for file in &commit.files {
        println!("    {} -> {}", file.path, file.hash);
    }

    Ok(())
}
