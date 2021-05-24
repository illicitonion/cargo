//! Implementation of a hasher that produces the same values across releases.
//!
//! The hasher should be fast and have a low chance of collisions (but is not
//! sufficient for cryptographic purposes).
#![allow(deprecated)]

use std::hash::{Hasher, SipHasher};
use std::io::Read;
use std::path::Path;

use anyhow::Context;

use crate::CargoResult;

pub struct StableHasher(SipHasher);

impl StableHasher {
    pub fn new() -> StableHasher {
        StableHasher(SipHasher::new())
    }

    pub fn hash_file<P: AsRef<Path>>(path: P) -> CargoResult<u64> {
        let path = path.as_ref();
        let mut hasher = Self::new();
        std::fs::File::open(path)
            .and_then(|mut f| {
                let mut buf = [0; 8192];
                loop {
                    let len = f.read(&mut buf)?;
                    if len == 0 {
                        return Ok(());
                    }
                    hasher.write(&buf[0..len]);
                }
            })
            .with_context(|| format!("failed to hash file {}", path.display()))?;
        Ok(hasher.finish())
    }
}

impl Hasher for StableHasher {
    fn finish(&self) -> u64 {
        self.0.finish()
    }
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }
}
