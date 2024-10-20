use std::path::{Path, PathBuf};

use super::ChatFile;

use sha1::{Digest, Sha1};
use tracing::info;

impl ChatFile {
    pub fn new(filename: &str, data: &[u8]) -> Self {
        let hash = Sha1::digest(data);
        Self {
            ext: filename.rsplit('.').next().unwrap_or("txt").to_string(),
            hash: hex::encode(hash),
        }
    }

    pub fn url(&self, ws_id: u64) -> String {
        format!("/files/{}/{}", ws_id, self.hash_to_path())
    }
    pub fn path(&self, base_dir: &Path) -> PathBuf {
        base_dir.join(self.hash_to_path())
    }

    //split hash into 3 parts, first 2 and 3 chars
    pub fn hash_to_path(&self) -> String {
        let (first, rest) = self.hash.split_at(3);
        let (second, third) = rest.split_at(3);
        info!(self.ext);
        format!("{}/{}/{}.{}", first, second, third, self.ext)
    }
}
