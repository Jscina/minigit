use crate::core::repo::Repo;
use anyhow::Result;
use sha1::{Digest, Sha1};
use std::fs;
use std::path::PathBuf;

pub fn hash_blob(content: &[u8]) -> String {
    let header = format!("blob {}\0", content.len());
    let mut hasher = Sha1::new();
    hasher.update(header.as_bytes());
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}

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

    #[test]
    fn test_hash_blob() {
        let content = b"Hello, world!";
        let hash = hash_blob(content);
        assert_eq!(hash, "5dd01c177f5d7d1be5346a5bc18a569a7410c2ef");
    }
}
