//! Chain backends for public ledger anchoring.
//!
//! Each backend implements [`AnchorChain`] to publish and verify Merkle roots
//! on a specific public blockchain. The mock backend is available via the
//! `test-support` feature or in `#[cfg(test)]`; BTC and ETH backends require
//! network access and credentials.

#[cfg(any(test, feature = "test-support"))]
mod mock;

#[cfg(any(test, feature = "test-support"))]
pub use mock::MockChain;

use crate::{AnchorError, receipt::AnchorReceipt};

/// Trait for public chain anchor backends.
///
/// Implementations publish a 32-byte Merkle root to a public blockchain
/// and return a receipt. The same implementation can verify a receipt
/// against the chain.
pub trait AnchorChain {
    /// Publish a Merkle root to the chain. Returns a receipt.
    ///
    /// # Errors
    ///
    /// Returns [`AnchorError::ChainPublish`] on failure.
    fn publish(
        &self,
        root: &[u8; 32],
        batch_size: usize,
        wave: &str,
        gate: &str,
    ) -> Result<AnchorReceipt, AnchorError>;

    /// Verify that a receipt's Merkle root exists on the chain.
    ///
    /// # Errors
    ///
    /// Returns [`AnchorError::ChainVerify`] on failure.
    fn verify(&self, receipt: &AnchorReceipt) -> Result<bool, AnchorError>;

    /// Human-readable chain name.
    fn name(&self) -> &'static str;
}
