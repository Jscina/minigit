use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::core::{
    object,
    repo::Repo,
    utils::{Commit, FileEntry},
};
use std::fs;

pub fn run(message: &str) -> Result<()> {
    let index_contents = fs::read_to_string(Repo::index_path())?;
    let files = index_contents
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            Some(FileEntry::new(
                parts.next()?.to_string(),
                parts.next()?.to_string(),
            ))
        })
        .collect::<Vec<_>>();

    let commit = Commit::new(message.to_string(), files);
    let json = serde_json::to_vec_pretty(&commit)?;
    let hash = object::write_blob(&json)?;

    fs::write(Repo::head_path(), format!("{}\n", hash))?;
    fs::write(Repo::index_path(), "")?;

    println!("Committed as {}", hash);

    Ok(())
}
