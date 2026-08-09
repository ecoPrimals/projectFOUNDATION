// SPDX-License-Identifier: AGPL-3.0-or-later
//! CAS federation client for cross-gate content discovery and replication.
//!
//! Enables any gate to locate and pull content from the mesh without knowing
//! which gate hosts it. The resolution chain:
//!
//! 1. Local nestGate CAS lookup via `content.exists` (fast path)
//! 2. `content.resolve` via songBird federation (mesh-wide hash query)
//! 3. `content.replicate.pull` from the discovered gate
//!
//! Method names verified against nestGate v0.5.0 capability.list on ironGate
//! (self-audit Aug 9, 2026 — Wave 157a VERTEBRATE EVOLUTION).
//!
//! This is Layer 0 of the gAIa sovereignty stack extended across gates.

use serde::{Deserialize, Serialize};

/// A gate location where content was found.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GateLocation {
    /// Gate identifier (e.g., "westGate", "strandGate").
    pub gate: String,
    /// Transport address (UDS path or TCP host:port).
    pub address: String,
    /// Whether this is the local gate.
    pub is_local: bool,
}

/// Result of a `content.locate` query across the mesh.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocateResult {
    /// The BLAKE3 hash that was queried.
    pub hash: [u8; 32],
    /// Gates where the content was found.
    pub locations: Vec<GateLocation>,
    /// Whether the content was found locally (fast path hit).
    pub local_hit: bool,
}

impl LocateResult {
    /// Whether the content was found anywhere in the mesh.
    #[must_use]
    pub fn found(&self) -> bool {
        !self.locations.is_empty()
    }

    /// Number of gates hosting this content.
    #[must_use]
    pub fn replica_count(&self) -> usize {
        self.locations.len()
    }

    /// The preferred source — local gate first, then first remote.
    #[must_use]
    pub fn preferred_source(&self) -> Option<&GateLocation> {
        self.locations
            .iter()
            .find(|l| l.is_local)
            .or_else(|| self.locations.first())
    }
}

/// Receipt for a completed content pull.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullReceipt {
    /// The BLAKE3 hash of the pulled content.
    pub hash: [u8; 32],
    /// Source gate the content was pulled from.
    pub source: GateLocation,
    /// Size of the pulled content in bytes.
    pub size_bytes: u64,
    /// Whether the pull was recorded in the provenance DAG.
    pub provenance_recorded: bool,
}

/// CAS statistics from a gate's nestGate instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CasStats {
    /// Total number of objects in the CAS.
    pub object_count: u64,
    /// Total size of all objects in bytes.
    pub total_bytes: u64,
    /// Namespaces present (e.g., "data", "compute", "provenance").
    pub namespaces: Vec<String>,
}

impl std::fmt::Display for CasStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mb = self.total_bytes / (1024 * 1024);
        write!(
            f,
            "CAS({} objects, {} MB, {} namespaces)",
            self.object_count,
            mb,
            self.namespaces.len(),
        )
    }
}

/// Collaboration surface definition for cross-gate work.
///
/// Maps the gate roles from the ecosystem blurb to typed constants
/// for use in federation queries.
pub mod gates {
    /// westGate — data CAS (50 TB ZFS, 153 datasets, federation root).
    pub const WESTGATE: &str = "westGate";
    /// strandGate — compute NFT memoization (SU(3)/SU(4) configs).
    pub const STRANDGATE: &str = "strandGate";
    /// ironGate — NUCLEUS host, integration + publication node.
    pub const IRONGATE: &str = "ironGate";
    /// sporeGate — cascade origin, depot source.
    pub const SPOREGATE: &str = "sporeGate";
    /// eastGate — overwatch node.
    pub const EASTGATE: &str = "eastGate";
    /// blueGate — marine research node.
    pub const BLUEGATE: &str = "blueGate";
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_locations() -> Vec<GateLocation> {
        vec![
            GateLocation {
                gate: "westGate".to_owned(),
                address: "10.13.37.11:9100".to_owned(),
                is_local: false,
            },
            GateLocation {
                gate: "ironGate".to_owned(),
                address: "/run/membrane/nestgate.sock".to_owned(),
                is_local: true,
            },
        ]
    }

    #[test]
    fn locate_result_found() {
        let result = LocateResult {
            hash: [0xab; 32],
            locations: sample_locations(),
            local_hit: true,
        };
        assert!(result.found());
        assert_eq!(result.replica_count(), 2);
    }

    #[test]
    fn preferred_source_local_first() {
        let result = LocateResult {
            hash: [0xab; 32],
            locations: sample_locations(),
            local_hit: true,
        };
        let preferred = result.preferred_source().unwrap();
        assert!(preferred.is_local);
        assert_eq!(preferred.gate, "ironGate");
    }

    #[test]
    fn preferred_source_remote_fallback() {
        let locations = vec![GateLocation {
            gate: "westGate".to_owned(),
            address: "10.13.37.11:9100".to_owned(),
            is_local: false,
        }];
        let result = LocateResult {
            hash: [0xab; 32],
            locations,
            local_hit: false,
        };
        let preferred = result.preferred_source().unwrap();
        assert_eq!(preferred.gate, "westGate");
    }

    #[test]
    fn empty_locate_not_found() {
        let result = LocateResult {
            hash: [0; 32],
            locations: vec![],
            local_hit: false,
        };
        assert!(!result.found());
        assert_eq!(result.replica_count(), 0);
        assert!(result.preferred_source().is_none());
    }

    #[test]
    fn cas_stats_display() {
        let stats = CasStats {
            object_count: 15_000,
            total_bytes: 500 * 1024 * 1024,
            namespaces: vec!["data".to_owned(), "compute".to_owned()],
        };
        let display = format!("{stats}");
        assert!(display.contains("15000 objects"));
        assert!(display.contains("500 MB"));
        assert!(display.contains("2 namespaces"));
    }

    #[test]
    fn pull_receipt_json_roundtrip() {
        let receipt = PullReceipt {
            hash: [0xcd; 32],
            source: GateLocation {
                gate: "strandGate".to_owned(),
                address: "10.13.37.10:9100".to_owned(),
                is_local: false,
            },
            size_bytes: 1024 * 1024 * 50,
            provenance_recorded: true,
        };
        let json = serde_json::to_string(&receipt).unwrap();
        let parsed: PullReceipt = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.hash, receipt.hash);
        assert_eq!(parsed.source.gate, "strandGate");
        assert!(parsed.provenance_recorded);
    }

    #[test]
    fn gate_constants_are_distinct() {
        let all = [
            gates::WESTGATE,
            gates::STRANDGATE,
            gates::IRONGATE,
            gates::SPOREGATE,
            gates::EASTGATE,
            gates::BLUEGATE,
        ];
        for (i, a) in all.iter().enumerate() {
            for b in &all[i + 1..] {
                assert_ne!(a, b, "duplicate gate constant");
            }
        }
    }

    #[test]
    fn locate_result_json_roundtrip() {
        let result = LocateResult {
            hash: [0xab; 32],
            locations: sample_locations(),
            local_hit: true,
        };
        let json = serde_json::to_string(&result).unwrap();
        let parsed: LocateResult = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.locations.len(), 2);
        assert!(parsed.local_hit);
    }
}
