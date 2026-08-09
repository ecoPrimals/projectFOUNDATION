// SPDX-License-Identifier: AGPL-3.0-or-later
//! Centralized JSON-RPC method name constants.
//!
//! All RPC method names follow the `domain.verb` convention established by
//! primalSpring. Centralizing them here ensures consistency and discoverability.

/// Health check methods (universal across all primals).
pub mod health {
    /// Liveness probe — "is the process alive?"
    pub const LIVENESS: &str = "health.liveness";
    /// Readiness probe — "is the primal ready to serve?"
    pub const READINESS: &str = "health.readiness";
    /// Version/capability check.
    pub const CHECK: &str = "health.check";
}

/// DAG / provenance methods (rhizoCrypt).
///
/// Verified against rhizoCrypt capability.list on ironGate (Aug 9, 2026).
pub mod dag {
    /// Create a new DAG session.
    pub const SESSION_CREATE: &str = "dag.session.create";
    /// Get an existing DAG session.
    pub const SESSION_GET: &str = "dag.session.get";
    /// List all DAG sessions.
    pub const SESSION_LIST: &str = "dag.session.list";
    /// Discard a DAG session.
    pub const SESSION_DISCARD: &str = "dag.session.discard";
    /// Compute the tree hash for a session (finalization).
    pub const SESSION_TREE_HASH: &str = "dag.session.tree_hash";
    /// Append an event to the DAG.
    pub const EVENT_APPEND: &str = "dag.event.append";
    /// Compute Merkle root of the DAG.
    pub const MERKLE_ROOT: &str = "dag.merkle.root";
    /// Generate a Merkle proof.
    pub const MERKLE_PROOF: &str = "dag.merkle.proof";
    /// Verify a Merkle proof.
    pub const MERKLE_VERIFY: &str = "dag.merkle.verify";
}

/// Spine / ledger methods (loamSpine).
///
/// Verified against loamSpine capability.list on ironGate (Aug 9, 2026).
pub mod spine {
    /// Create a new spine (ledger).
    pub const CREATE: &str = "spine.create";
    /// Get a spine by ID.
    pub const GET: &str = "spine.get";
    /// List all spines.
    pub const LIST: &str = "spine.list";
    /// Seal a spine (make immutable).
    pub const SEAL: &str = "spine.seal";
}

/// Entry methods (loamSpine ledger entries).
pub mod entry {
    /// Append data to a spine entry.
    pub const APPEND: &str = "entry.append";
    /// Get a spine entry.
    pub const GET: &str = "entry.get";
    /// Get the tip (latest) entry.
    pub const GET_TIP: &str = "entry.get_tip";
    /// List entries in a spine.
    pub const LIST: &str = "entry.list";
}

/// Session commit (loamSpine — commits a DAG session to permanent ledger).
pub mod session {
    /// Commit a session to the permanent ledger.
    pub const COMMIT: &str = "session.commit";
}

/// Braid / attribution methods (loamSpine hosts braid.commit).
///
/// Note: sweetGrass requires riboCipher signal prefix for direct access.
/// loamSpine exposes `braid.commit` as the primary integration surface.
pub mod braid {
    /// Commit an attribution braid (DAG + spine + attestation).
    pub const COMMIT: &str = "braid.commit";
}

/// Anchor methods (loamSpine — Layer 5 public ledger anchoring).
///
/// Verified against loamSpine capability.list on ironGate (Aug 9, 2026).
/// These are the actual RPC interface for foundation-anchor's chain backends.
pub mod anchor {
    /// Publish a Merkle root to public chain.
    pub const PUBLISH: &str = "anchor.publish";
    /// Publish a batch of Merkle roots.
    pub const PUBLISH_BATCH: &str = "anchor.publish_batch";
    /// Verify an anchor receipt against the chain.
    pub const VERIFY: &str = "anchor.verify";
}

/// Family/composition identity methods (discovery meta-primal).
pub mod family {
    /// Resolve the family/composition ID.
    pub const ID: &str = "family.id";
}

/// Content-addressed storage methods (nestGate CAS).
///
/// Verified against nestGate capability.list on ironGate (Aug 9, 2026).
/// Method names aligned to actual shipped API (self-audit Wave 157a).
pub mod content {
    /// Resolve content location by BLAKE3 hash (mesh-wide lookup).
    pub const RESOLVE: &str = "content.resolve";
    /// Store content in CAS by BLAKE3 hash.
    pub const PUT: &str = "content.put";
    /// Retrieve content from CAS by BLAKE3 hash.
    pub const GET: &str = "content.get";
    /// Check if content exists in CAS.
    pub const EXISTS: &str = "content.exists";
    /// List content in CAS.
    pub const LIST: &str = "content.list";
    /// Pull content from a remote gate into local CAS.
    pub const REPLICATE_PULL: &str = "content.replicate.pull";
    /// Publish content (make available for federation).
    pub const PUBLISH: &str = "content.publish";
    /// Push content to a remote gate.
    pub const PUSH: &str = "content.push";
}

/// Storage methods (nestGate low-level storage layer).
pub mod storage {
    /// Query storage statistics (object count, size, namespaces).
    pub const STATS: &str = "storage.stats";
    /// List namespaces in storage.
    pub const NAMESPACES_LIST: &str = "storage.namespaces.list";
}

/// Crypto methods (bearDog).
///
/// Verified against bearDog capability.list on ironGate (Aug 9, 2026).
/// Note: P0-A reports depot binary may be a stub — verify signing works.
pub mod crypto {
    /// Sign data with Ed25519.
    pub const SIGN_ED25519: &str = "crypto.sign_ed25519";
    /// Verify an Ed25519 signature.
    pub const VERIFY_ED25519: &str = "crypto.verify_ed25519";
    /// Compute BLAKE3 hash.
    pub const BLAKE3_HASH: &str = "crypto.blake3_hash";
}

/// Foundation-specific methods (this primal's own interface).
pub mod foundation {
    /// Query ecosystem health data.
    pub const ECOSYSTEM_HEALTH: &str = "foundation.ecosystem_health";
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_methods() -> Vec<&'static str> {
        vec![
            health::LIVENESS,
            health::READINESS,
            health::CHECK,
            dag::SESSION_CREATE,
            dag::SESSION_GET,
            dag::SESSION_LIST,
            dag::SESSION_DISCARD,
            dag::SESSION_TREE_HASH,
            dag::EVENT_APPEND,
            dag::MERKLE_ROOT,
            dag::MERKLE_PROOF,
            dag::MERKLE_VERIFY,
            spine::CREATE,
            spine::GET,
            spine::LIST,
            spine::SEAL,
            entry::APPEND,
            entry::GET,
            entry::GET_TIP,
            entry::LIST,
            session::COMMIT,
            braid::COMMIT,
            anchor::PUBLISH,
            anchor::PUBLISH_BATCH,
            anchor::VERIFY,
            content::RESOLVE,
            content::PUT,
            content::GET,
            content::EXISTS,
            content::LIST,
            content::REPLICATE_PULL,
            content::PUBLISH,
            content::PUSH,
            storage::STATS,
            storage::NAMESPACES_LIST,
            crypto::SIGN_ED25519,
            crypto::VERIFY_ED25519,
            crypto::BLAKE3_HASH,
            family::ID,
            foundation::ECOSYSTEM_HEALTH,
        ]
    }

    #[test]
    fn method_names_follow_domain_verb_convention() {
        for method in all_methods() {
            assert!(
                method.contains('.'),
                "method '{method}' must follow domain.verb convention"
            );
        }
    }

    #[test]
    fn health_methods_share_prefix() {
        assert!(health::LIVENESS.starts_with("health."));
        assert!(health::READINESS.starts_with("health."));
        assert!(health::CHECK.starts_with("health."));
    }

    #[test]
    fn dag_methods_share_prefix() {
        for m in [
            dag::SESSION_CREATE,
            dag::SESSION_GET,
            dag::SESSION_LIST,
            dag::SESSION_DISCARD,
            dag::SESSION_TREE_HASH,
            dag::EVENT_APPEND,
            dag::MERKLE_ROOT,
            dag::MERKLE_PROOF,
            dag::MERKLE_VERIFY,
        ] {
            assert!(m.starts_with("dag."), "'{m}' must start with dag.");
        }
    }

    #[test]
    fn content_methods_share_prefix() {
        for m in [
            content::RESOLVE,
            content::PUT,
            content::GET,
            content::EXISTS,
            content::LIST,
            content::REPLICATE_PULL,
            content::PUBLISH,
            content::PUSH,
        ] {
            assert!(m.starts_with("content."), "'{m}' must start with content.");
        }
    }

    #[test]
    fn anchor_methods_share_prefix() {
        assert!(anchor::PUBLISH.starts_with("anchor."));
        assert!(anchor::PUBLISH_BATCH.starts_with("anchor."));
        assert!(anchor::VERIFY.starts_with("anchor."));
    }

    #[test]
    fn crypto_methods_share_prefix() {
        assert!(crypto::SIGN_ED25519.starts_with("crypto."));
        assert!(crypto::VERIFY_ED25519.starts_with("crypto."));
        assert!(crypto::BLAKE3_HASH.starts_with("crypto."));
    }

    #[test]
    fn no_duplicate_method_names() {
        let all = all_methods();
        for (i, a) in all.iter().enumerate() {
            for b in &all[i + 1..] {
                assert_ne!(a, b, "duplicate method name: {a}");
            }
        }
    }
}
