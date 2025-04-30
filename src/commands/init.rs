use crate::core::repo::Repo;
use anyhow::Result;
use std::{
    fs,
    io::{self, Write},
};

pub fn run() -> Result<()> {
    let repo_dir = Repo::minigit_dir();
    if repo_dir.exists() {
        if writeln!(
            io::stdout(),
            "Repository already initialized at {}.\n",
            repo_dir.display()
        )
        .is_err()
        {
            eprintln!("Failed to write to stdout.");
        }
    } else {
        fs::create_dir_all(&repo_dir)?;
        writeln!(
            io::stdout(),
            "Initialized empty repository at {}.\n",
            repo_dir.display()
        )?;
    }

    let objects_dir = Repo::objects_dir();
    let index_path = Repo::index_path();
    let head_path = Repo::head_path();

    fs::create_dir_all(&objects_dir)?;
    fs::write(&index_path, "")?;
    fs::write(&head_path, "ref: refs/heads/master\n")?;

    Ok(())
}
