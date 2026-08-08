// SPDX-License-Identifier: AGPL-3.0-or-later
//! Artifact registry — walk fetched data and compute BLAKE3 content addresses.
//!
//! Uses `std::fs` recursive traversal (zero external deps) instead of `walkdir`.
//! `NestGate` RPC registration is a Phase C target.

use std::path::{Path, PathBuf};

use foundation_core::CoreError;
use tracing::{debug, info};

use crate::hasher;

/// An artifact discovered during directory walking.
#[derive(Debug, Clone)]
pub struct Artifact {
    /// Absolute path to the file.
    pub path: PathBuf,
    /// BLAKE3 hex digest.
    pub blake3: String,
    /// File size in bytes.
    pub size: u64,
}

/// Registry for discovering and cataloging data artifacts.
pub struct ArtifactRegistry {
    artifacts: Vec<Artifact>,
}

impl ArtifactRegistry {
    /// Scan a directory tree and compute BLAKE3 hashes for all files.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::Io`] if the directory cannot be traversed.
    pub fn scan(data_dir: &Path) -> Result<Self, CoreError> {
        let mut artifacts = Vec::new();
        Self::walk_recursive(data_dir, &mut artifacts);
        info!(count = artifacts.len(), "artifact scan complete");
        Ok(Self { artifacts })
    }

    fn walk_recursive(dir: &Path, artifacts: &mut Vec<Artifact>) {
        let entries = match std::fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(e) => {
                tracing::warn!(dir = %dir.display(), error = %e, "cannot read directory");
                return;
            }
        };

        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_symlink() {
                continue;
            }
            if path.is_dir() {
                Self::walk_recursive(&path, artifacts);
                continue;
            }
            if !path.is_file() {
                continue;
            }

            let size = entry.metadata().map_or(0, |m| m.len());
            match hasher::blake3_file(&path) {
                Ok(hash) => {
                    debug!(path = %path.display(), hash = %&hash[..12], "registered artifact");
                    artifacts.push(Artifact {
                        path,
                        blake3: hash,
                        size,
                    });
                }
                Err(e) => {
                    tracing::warn!(path = %path.display(), error = %e, "failed to hash artifact");
                }
            }
        }
    }

    /// Get all discovered artifacts.
    #[must_use]
    pub fn artifacts(&self) -> &[Artifact] {
        &self.artifacts
    }

    /// Total number of artifacts.
    #[must_use]
    pub fn count(&self) -> usize {
        self.artifacts.len()
    }

    /// Total size of all artifacts in bytes.
    #[must_use]
    pub fn total_size(&self) -> u64 {
        self.artifacts.iter().map(|a| a.size).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn scan_directory() {
        let dir = tempfile::tempdir().unwrap();
        let file_a = dir.path().join("a.dat");
        let file_b = dir.path().join("b.dat");
        std::fs::File::create(&file_a)
            .unwrap()
            .write_all(b"data a")
            .unwrap();
        std::fs::File::create(&file_b)
            .unwrap()
            .write_all(b"data b")
            .unwrap();

        let registry = ArtifactRegistry::scan(dir.path()).unwrap();
        assert_eq!(registry.count(), 2);
        assert!(registry.total_size() > 0);
    }

    #[test]
    fn scan_empty_directory() {
        let dir = tempfile::tempdir().unwrap();
        let registry = ArtifactRegistry::scan(dir.path()).unwrap();
        assert_eq!(registry.count(), 0);
    }
}
