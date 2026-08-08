/// Errors from the anchoring pipeline.
#[derive(Debug, thiserror::Error)]
pub enum AnchorError {
    /// Batch is empty — nothing to anchor.
    #[error("anchor batch is empty")]
    EmptyBatch,

    /// Merkle tree construction failed.
    #[error("merkle tree error: {0}")]
    MerkleConstruction(String),

    /// Chain publication failed.
    #[error("chain publish error on {chain}: {reason}")]
    ChainPublish {
        /// Which chain was targeted.
        chain: String,
        /// What went wrong.
        reason: String,
    },

    /// Chain verification failed.
    #[error("chain verify error on {chain}: {reason}")]
    ChainVerify {
        /// Which chain was queried.
        chain: String,
        /// What went wrong.
        reason: String,
    },

    /// Serialization or deserialization failure.
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
}
