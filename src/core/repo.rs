use std::path::PathBuf;

/// Represents a repository in the minigit version control system.
pub struct Repo;

impl Repo {
    /// Returns the path to the `.minigit` directory.
    pub fn minigit_dir() -> PathBuf {
        PathBuf::from(".minigit")
    }

    /// Returns the path to a specific subdirectory or file within the `.minigit` directory.
    fn minigit_subpath(subpath: &str) -> PathBuf {
        Self::minigit_dir().join(subpath)
    }

    /// Returns the path to the `objects` directory.
    pub fn objects_dir() -> PathBuf {
        Self::minigit_subpath("objects")
    }

    /// Returns the path to the `HEAD` file.
    pub fn head_path() -> PathBuf {
        Self::minigit_subpath("HEAD")
    }

    /// Returns the path to the `index` file.
    pub fn index_path() -> PathBuf {
        Self::minigit_subpath("index")
    }

    /// Returns the path to the ignore file.
    pub fn minigit_ignore_path() -> PathBuf {
        PathBuf::from(".minigitignore")
    }
}
