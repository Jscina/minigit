use crate::core::repo::Repo;
use anyhow::Result;
use sha1::{Digest, Sha1};
use std::fs;

/// Computes the SHA-1 hash of a blob's content using Git's blob format
///
/// The function prepends the header "blob {size}\0" to the content before hashing
pub fn hash_blob(content: &[u8]) -> String {
    let header = format!("blob {}\0", content.len());
    let mut hasher = Sha1::new();
    hasher.update(header.as_bytes());
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}

/// Writes a blob to the Git object store and returns its hash
///
/// This function formats the content with the appropriate header,
/// computes the hash, and writes the data to the objects directory
pub fn write_blob(content: &[u8]) -> Result<String> {
    let header = format!("blob {}\0", content.len());
    let mut data = Vec::new();
    data.extend_from_slice(header.as_bytes());
    data.extend_from_slice(content);

    let hash = hash_blob(content);

    let path = Repo::objects_dir().join(&hash);
    fs::write(path, data)?;

    Ok(hash)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{commands::init, core::repo::Repo};

    #[test]
    fn test_hash_blob() {
        let content = b"Hello, world!";
        let hash = hash_blob(content);
        assert_eq!(hash, "5dd01c177f5d7d1be5346a5bc18a569a7410c2ef");
    }

    #[test]
    fn test_write_blob() {
        let content = b"Hello, world!";
        let hash = write_blob(content);
        if let Err(e) = &hash {
            init::run().unwrap();
        }
        let hash = write_blob(content).expect("Repo not initialized");
        assert_eq!(&hash, "5dd01c177f5d7d1be5346a5bc18a569a7410c2ef");

        // Check if the file was created
        let path = Repo::objects_dir().join(&hash);
        assert!(path.exists());

        // Clean up
        fs::remove_dir_all(Repo::minigit_dir());
    }
}
