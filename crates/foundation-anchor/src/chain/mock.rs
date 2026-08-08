use std::collections::HashMap;
use std::sync::Mutex;

use crate::error::AnchorError;
use crate::receipt::{AnchorReceipt, ChainId};

use super::AnchorChain;

/// In-memory mock chain for testing the anchoring pipeline.
///
/// Stores published roots in a `HashMap` keyed by tx hash. Thread-safe
/// via interior `Mutex` so tests can share a single instance.
///
/// # Panics
///
/// Methods panic if the internal mutex is poisoned (only possible after
/// a prior panic in the same process).
pub struct MockChain {
    anchors: Mutex<HashMap<String, [u8; 32]>>,
    tx_counter: Mutex<u64>,
}

impl MockChain {
    /// Create a new empty mock chain.
    #[must_use]
    pub fn new() -> Self {
        Self {
            anchors: Mutex::new(HashMap::new()),
            tx_counter: Mutex::new(0),
        }
    }

    /// Number of anchors stored in the mock chain.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    #[must_use]
    #[expect(clippy::expect_used, reason = "mutex poisoning is unrecoverable")]
    pub fn anchor_count(&self) -> usize {
        self.anchors.lock().expect("mock lock").len()
    }
}

impl Default for MockChain {
    fn default() -> Self {
        Self::new()
    }
}

impl AnchorChain for MockChain {
    #[expect(clippy::expect_used, reason = "mutex poisoning is unrecoverable")]
    fn publish(
        &self,
        root: &[u8; 32],
        batch_size: usize,
        wave: &str,
        gate: &str,
    ) -> Result<AnchorReceipt, AnchorError> {
        let mut counter = self.tx_counter.lock().expect("mock lock");
        *counter += 1;
        let tx_id = *counter;
        drop(counter);
        let tx_hash = format!("mock_tx_{tx_id:08}");

        self.anchors
            .lock()
            .expect("mock lock")
            .insert(tx_hash.clone(), *root);

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());

        Ok(AnchorReceipt {
            chain: ChainId::Mock,
            tx_hash,
            merkle_root: *root,
            batch_size,
            cost_smallest_unit: 0,
            timestamp,
            wave: wave.to_owned(),
            gate: gate.to_owned(),
        })
    }

    #[expect(clippy::expect_used, reason = "mutex poisoning is unrecoverable")]
    fn verify(&self, receipt: &AnchorReceipt) -> Result<bool, AnchorError> {
        let anchors = self.anchors.lock().expect("mock lock");
        Ok(anchors
            .get(&receipt.tx_hash)
            .is_some_and(|stored_root| *stored_root == receipt.merkle_root))
    }

    fn name(&self) -> &'static str {
        "mock"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publish_and_verify() {
        let chain = MockChain::new();
        let root = *blake3::hash(b"test-root").as_bytes();
        let receipt = chain.publish(&root, 100, "157a", "ironGate").unwrap();
        assert_eq!(receipt.chain, ChainId::Mock);
        assert!(chain.verify(&receipt).unwrap());
    }

    #[test]
    fn verify_unknown_tx_returns_false() {
        let chain = MockChain::new();
        let receipt = AnchorReceipt {
            chain: ChainId::Mock,
            tx_hash: "nonexistent".to_owned(),
            merkle_root: [0; 32],
            batch_size: 0,
            cost_smallest_unit: 0,
            timestamp: 0,
            wave: String::new(),
            gate: String::new(),
        };
        assert!(!chain.verify(&receipt).unwrap());
    }

    #[test]
    fn tampered_root_fails_verify() {
        let chain = MockChain::new();
        let root = *blake3::hash(b"real-root").as_bytes();
        let mut receipt = chain.publish(&root, 50, "157a", "ironGate").unwrap();
        receipt.merkle_root = [0xff; 32];
        assert!(!chain.verify(&receipt).unwrap());
    }

    #[test]
    fn multiple_publishes_tracked() {
        let chain = MockChain::new();
        let r1 = *blake3::hash(b"root-1").as_bytes();
        let r2 = *blake3::hash(b"root-2").as_bytes();
        chain.publish(&r1, 10, "157a", "ironGate").unwrap();
        chain.publish(&r2, 20, "157a", "ironGate").unwrap();
        assert_eq!(chain.anchor_count(), 2);
    }

    #[test]
    fn tx_hashes_are_unique() {
        let chain = MockChain::new();
        let root = *blake3::hash(b"root").as_bytes();
        let r1 = chain.publish(&root, 10, "157a", "ironGate").unwrap();
        let r2 = chain.publish(&root, 10, "157a", "ironGate").unwrap();
        assert_ne!(r1.tx_hash, r2.tx_hash);
    }
}
