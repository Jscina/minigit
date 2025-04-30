use std::path::PathBuf;

pub struct Repo;

impl Repo {
    pub fn minigit_dir() -> PathBuf {
        PathBuf::from(".minigit")
    }
    pub fn objects_dir() -> PathBuf {
        Self::minigit_dir().join("objects")
    }
    pub fn head_path() -> PathBuf {
        Self::minigit_dir().join("HEAD")
    }

    pub fn index_path() -> PathBuf {
        Self::minigit_dir().join("index")
    }
}
