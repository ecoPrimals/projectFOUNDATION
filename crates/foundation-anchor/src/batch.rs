use crate::{AnchorError, MerkleTree};

/// A batch of loamSpine certificate hashes aggregated for anchoring.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnchorBatch {
    /// Certificate hashes in the batch.
    pub certificates: Vec<[u8; 32]>,
    /// The Merkle root of all certificates.
    pub merkle_root: [u8; 32],
    /// Batch creation timestamp (seconds since epoch).
    pub timestamp: u64,
    /// Wave identifier when batch was created.
    pub wave: String,
    /// Gate that created the batch.
    pub gate: String,
}

/// Incrementally builds an [`AnchorBatch`] from certificate hashes.
///
/// Collects hashes until a configured threshold, then finalizes into
/// a batch with a Merkle root ready for chain publication.
pub struct BatchBuilder {
    hashes: Vec<[u8; 32]>,
    threshold: usize,
    wave: String,
    gate: String,
}

impl BatchBuilder {
    /// Create a new builder with a batch size threshold.
    ///
    /// The default aggregation ratio is 1000:1 (1000 certificates per
    /// anchor transaction), per whitePaper `ANCHORING_PIPELINE.md`.
    #[must_use]
    pub fn new(wave: &str, gate: &str) -> Self {
        Self {
            hashes: Vec::new(),
            threshold: 1000,
            wave: wave.to_owned(),
            gate: gate.to_owned(),
        }
    }

    /// Override the default batch threshold.
    #[must_use]
    pub const fn with_threshold(mut self, threshold: usize) -> Self {
        self.threshold = threshold;
        self
    }

    /// Add a certificate hash to the batch.
    pub fn push(&mut self, hash: [u8; 32]) {
        self.hashes.push(hash);
    }

    /// Number of certificates currently in the builder.
    #[must_use]
    pub fn len(&self) -> usize {
        self.hashes.len()
    }

    /// Whether the builder is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.hashes.is_empty()
    }

    /// Whether the batch has reached the threshold.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.hashes.len() >= self.threshold
    }

    /// Finalize the batch: build the Merkle tree and produce an [`AnchorBatch`].
    ///
    /// Can be called before the threshold is reached (manual flush).
    ///
    /// # Errors
    ///
    /// Returns [`AnchorError::EmptyBatch`] if no certificates have been added.
    pub fn finalize(&self) -> Result<AnchorBatch, AnchorError> {
        let tree = MerkleTree::from_certificates(&self.hashes)?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());

        Ok(AnchorBatch {
            certificates: self.hashes.clone(),
            merkle_root: *tree.root(),
            timestamp,
            wave: self.wave.clone(),
            gate: self.gate.clone(),
        })
    }

    /// Finalize and reset the builder for reuse.
    ///
    /// # Errors
    ///
    /// Returns [`AnchorError::EmptyBatch`] if no certificates have been added.
    pub fn flush(&mut self) -> Result<AnchorBatch, AnchorError> {
        let batch = self.finalize()?;
        self.hashes.clear();
        Ok(batch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash_str(s: &str) -> [u8; 32] {
        *blake3::hash(s.as_bytes()).as_bytes()
    }

    #[test]
    fn builder_tracks_count() {
        let mut builder = BatchBuilder::new("157a", "ironGate");
        assert!(builder.is_empty());
        builder.push(hash_str("cert-001"));
        assert_eq!(builder.len(), 1);
        assert!(!builder.is_empty());
    }

    #[test]
    fn builder_threshold_default_1000() {
        let mut builder = BatchBuilder::new("157a", "ironGate");
        for i in 0..999 {
            builder.push(hash_str(&format!("cert-{i:04}")));
        }
        assert!(!builder.is_ready());
        builder.push(hash_str("cert-0999"));
        assert!(builder.is_ready());
    }

    #[test]
    fn builder_custom_threshold() {
        let mut builder = BatchBuilder::new("157a", "ironGate").with_threshold(3);
        builder.push(hash_str("a"));
        builder.push(hash_str("b"));
        assert!(!builder.is_ready());
        builder.push(hash_str("c"));
        assert!(builder.is_ready());
    }

    #[test]
    fn finalize_produces_batch() {
        let mut builder = BatchBuilder::new("157a", "ironGate");
        builder.push(hash_str("cert-001"));
        builder.push(hash_str("cert-002"));
        let batch = builder.finalize().unwrap();
        assert_eq!(batch.certificates.len(), 2);
        assert_eq!(batch.wave, "157a");
        assert_eq!(batch.gate, "ironGate");
        assert!(batch.timestamp > 0);
    }

    #[test]
    fn flush_resets_builder() {
        let mut builder = BatchBuilder::new("157a", "ironGate");
        builder.push(hash_str("cert-001"));
        let batch = builder.flush().unwrap();
        assert_eq!(batch.certificates.len(), 1);
        assert!(builder.is_empty());
    }

    #[test]
    fn empty_finalize_errors() {
        let builder = BatchBuilder::new("157a", "ironGate");
        assert!(builder.finalize().is_err());
    }

    #[test]
    fn batch_merkle_root_matches_tree() {
        let hashes: Vec<[u8; 32]> = (0..10).map(|i| hash_str(&format!("cert-{i:03}"))).collect();
        let mut builder = BatchBuilder::new("157a", "ironGate");
        for h in &hashes {
            builder.push(*h);
        }
        let batch = builder.finalize().unwrap();
        let tree = MerkleTree::from_certificates(&hashes).unwrap();
        assert_eq!(batch.merkle_root, *tree.root());
    }
}
