use core::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};
use std::ops::Deref;

type PieceHash = [u8; 20];

#[derive(Clone, Debug)]
pub struct Hashes(pub Vec<PieceHash>);

impl Deref for Hashes {
    type Target = Vec<PieceHash>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct HashesVisitor;
impl<'de> Visitor<'de> for HashesVisitor {
    type Value = Hashes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a byte string whose length is multiple of 20")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if !v.len().is_multiple_of(20) {
            return Err(E::custom(format!(
                "length {} is not a multiple of 20",
                v.len()
            )));
        }

        let hashes = Hashes(
            v.chunks_exact(20)
                .map(|c| c.try_into().expect("chunks_exact yields length 20"))
                .collect(),
        );
        Ok(hashes)
    }
}

impl<'de> Deserialize<'de> for Hashes {
    fn deserialize<D>(deserializer: D) -> Result<Hashes, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(HashesVisitor)
    }
}

impl Serialize for Hashes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.0.as_flattened())
    }
}
