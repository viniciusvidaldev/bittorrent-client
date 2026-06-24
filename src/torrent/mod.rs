mod hashes;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::path::Path;

use crate::torrent::hashes::Hashes;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Torrent {
    /// The URL of the tracker.
    pub announce: String,

    pub info: Info,
}
impl Torrent {
    pub async fn read(path: impl AsRef<Path>) -> Result<Self> {
        let torrent_file = tokio::fs::read(path).await?;
        Ok(serde_bencode::from_bytes(&torrent_file)?)
    }

    pub fn info_hash(&self) -> [u8; 20] {
        let bencoded_info =
            serde_bencode::to_bytes(&self.info).expect("re-encode info section should work fine");
        let hash = Sha1::digest(&bencoded_info);
        hash.into()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Info {
    /// The suggested name to save the file (or directory) as. It is purely advisory.
    ///
    /// In the single file case, the name key is the name of a file, in the muliple file case, it's
    /// the name of a directory.
    pub name: String,

    /// The number of bytes in each piece the file is split into.
    ///
    /// For the purposes of transfer, files are split into fixed-size pieces which are all the same
    /// length except for possibly the last one which may be truncated. piece length is almost
    /// always a power of two, most commonly 2 18 = 256 K (BitTorrent prior to version 3.2 uses
    /// 2 20 = 1 M as default).
    #[serde(rename = "piece length")]
    pub plength: u64,

    /// Each entry of `pieces` is the SHA1 hash of the piece at the corresponding index.
    pub pieces: Hashes,

    #[serde(flatten)]
    pub keys: Keys,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Keys {
    /// If length is present then the download represents a single file.
    /// Length of the file in bytes.
    SingleFile { length: u64 },
    /// Otherwise it represents a set of files which go in a directory structure.
    ///
    /// For the purposes of the other keys `Info`, the multi-file case is treated as only
    /// having a single file by concatenating the files in the order they appear
    /// in the files list.
    MultiFile { files: Vec<File> },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
    /// The length of the file, in bytes.
    pub length: u64,
    /// A list of UTF-8 encoded strings corresponding to subdirectory names, the last
    /// of which is the actual file name (a zero length list is an error case).
    pub path: Vec<String>,
}
