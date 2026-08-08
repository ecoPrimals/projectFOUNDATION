/// Identifies which public chain was used for anchoring.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ChainId {
    /// Bitcoin mainnet (`OP_RETURN`).
    Bitcoin,
    /// Ethereum mainnet (calldata).
    Ethereum,
    /// Local mock chain (testing only — not available in production builds).
    #[cfg(any(test, feature = "test-support"))]
    Mock,
}

impl std::fmt::Display for ChainId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bitcoin => write!(f, "bitcoin"),
            Self::Ethereum => write!(f, "ethereum"),
            #[cfg(any(test, feature = "test-support"))]
            Self::Mock => write!(f, "mock"),
        }
    }
}

/// Receipt returned after successful public ledger anchoring.
///
/// This receipt proves that a specific Merkle root was written to a public
/// chain at a specific time. It is stored in nestGate CAS and referenced
/// by a loamSpine ledger entry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnchorReceipt {
    /// Which public chain the root was written to.
    pub chain: ChainId,
    /// Transaction hash on the public chain.
    pub tx_hash: String,
    /// The 32-byte Merkle root that was anchored.
    pub merkle_root: [u8; 32],
    /// Number of certificates in the batch.
    pub batch_size: usize,
    /// Anchoring cost in the chain's smallest unit (satoshis for BTC, wei for ETH).
    pub cost_smallest_unit: u64,
    /// Timestamp when the anchor was published (seconds since epoch).
    pub timestamp: u64,
    /// Wave during which the anchor was created.
    pub wave: String,
    /// Gate that published the anchor.
    pub gate: String,
}

impl AnchorReceipt {
    /// Hex-encoded Merkle root for display.
    #[must_use]
    pub fn root_hex(&self) -> String {
        use std::fmt::Write;
        self.merkle_root
            .iter()
            .fold(String::with_capacity(64), |mut s, b| {
                let _ = write!(s, "{b:02x}");
                s
            })
    }

    /// Cost per certificate (integer division in chain's smallest unit).
    #[must_use]
    pub const fn cost_per_certificate(&self) -> u64 {
        if self.batch_size == 0 {
            return 0;
        }
        self.cost_smallest_unit / self.batch_size as u64
    }
}

impl std::fmt::Display for AnchorReceipt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "anchor({}, tx={}, root={}, batch={}, cost={})",
            self.chain,
            &self.tx_hash[..8.min(self.tx_hash.len())],
            &self.root_hex()[..8],
            self.batch_size,
            self.cost_smallest_unit,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_receipt() -> AnchorReceipt {
        AnchorReceipt {
            chain: ChainId::Bitcoin,
            tx_hash: "abc123def456789012345678901234567890abcdef1234567890abcdef12345678"
                .to_owned(),
            merkle_root: [0xab; 32],
            batch_size: 1000,
            cost_smallest_unit: 50_000,
            timestamp: 1_723_100_000,
            wave: "157a".to_owned(),
            gate: "ironGate".to_owned(),
        }
    }

    #[test]
    fn cost_per_certificate() {
        let receipt = sample_receipt();
        assert_eq!(receipt.cost_per_certificate(), 50);
    }

    #[test]
    fn root_hex_length() {
        let receipt = sample_receipt();
        assert_eq!(receipt.root_hex().len(), 64);
    }

    #[test]
    fn display_format() {
        let receipt = sample_receipt();
        let display = format!("{receipt}");
        assert!(display.contains("bitcoin"));
        assert!(display.contains("batch=1000"));
    }

    #[test]
    fn chain_id_display() {
        assert_eq!(ChainId::Bitcoin.to_string(), "bitcoin");
        assert_eq!(ChainId::Ethereum.to_string(), "ethereum");
        assert_eq!(ChainId::Mock.to_string(), "mock");
    }

    #[test]
    fn zero_batch_no_panic() {
        let mut receipt = sample_receipt();
        receipt.batch_size = 0;
        assert_eq!(receipt.cost_per_certificate(), 0);
    }

    #[test]
    fn roundtrip_json() {
        let receipt = sample_receipt();
        let json = serde_json::to_string(&receipt).unwrap();
        let parsed: AnchorReceipt = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.chain, ChainId::Bitcoin);
        assert_eq!(parsed.batch_size, 1000);
        assert_eq!(parsed.merkle_root, receipt.merkle_root);
    }
}
