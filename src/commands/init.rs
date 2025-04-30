use crate::core::repo::Repo;
use anyhow::Result;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

pub fn run() -> Result<()> {
    let path = Repo::minigit_dir();
    if path.exists() {
        writeln!(
            io::stdout(),
            "Repository already initialized at {}.\n",
            path.display()
        )?;
    } else {
        fs::create_dir_all(&path)?;
        writeln!(
            io::stdout(),
            "Initialized empty repository at {}.\n",
            path.display()
        )?;
    }

    fs::create_dir_all(Repo::objects_dir())?;
    fs::write(Repo::index_path(), "")?;
    fs::write(Repo::head_path(), "ref: refs/heads/master\n")?;

    Ok(())
}
