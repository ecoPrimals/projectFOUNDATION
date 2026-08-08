// SPDX-License-Identifier: AGPL-3.0-or-later
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
//! # foundation-anchor
//!
//! Public ledger anchoring for the gAIa sovereignty stack (Layer 5).
//!
//! Aggregates loamSpine certificates into BLAKE3 Merkle trees and publishes
//! 32-byte roots to public blockchains (BTC `OP_RETURN`, ETH calldata) as
//! timestamp anchors. The ecosystem treats public chains as infrastructure
//! ("postage"), not currency — no tokens, wallets, or defi.
//!
//! ## Workflow
//!
//! 1. Validation runs produce loamSpine certificates (Layers 1-4)
//! 2. Certificates accumulate in a local [`AnchorBatch`]
//! 3. [`MerkleTree::from_certificates`] builds the BLAKE3 tree
//! 4. An [`AnchorChain`] implementation publishes the root
//! 5. The [`AnchorReceipt`] is stored in nestGate CAS + loamSpine ledger
//! 6. Any party verifies: fetch batch from CAS, rebuild tree, check chain

mod batch;
pub mod chain;
mod error;
mod merkle;
mod receipt;

pub use batch::{AnchorBatch, BatchBuilder};
pub use error::AnchorError;
pub use merkle::MerkleTree;
pub use receipt::AnchorReceipt;
