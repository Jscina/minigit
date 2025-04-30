use crate::core::{object, repo::Repo};
use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn run(filenames: &[String]) -> Result<()> {
    for filename in filenames {
        let path = Path::new(filename);

        let contents =
            fs::read(path).with_context(|| format!("Failed to read file: {}", filename))?;

        let hash = object::write_blob(&contents)?;

        let index_line = format!("{} {}\n", filename, hash);
        fs::OpenOptions::new()
            .append(true)
            .open(Repo::index_path())
            .with_context(|| "Failed to open .minigit/index")?
            .write_all(index_line.as_bytes())?;
    }
    println!("Staged {} files", filenames.len());
    Ok(())
}
