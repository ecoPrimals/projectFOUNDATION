# gAIa Substrate Specification

**Date**: 2026-08-08 (Wave 157a)
**Status**: Design — implementation in progress
**Upstream**: whitePaper gen0-gen4 (gAIa vision, sunCloud economics, sovereignty stack)
**Local**: projectFOUNDATION (gAIa substrate library)

## Purpose

This document specifies how projectFOUNDATION evolves from a scientific
validation tool into the gAIa substrate library — the software layer that
enables sovereign publication, radiating attribution, and public ledger
anchoring for the ecoPrimals ecosystem.

gAIa (the 8th primal concept from whitePaper gen0) is a self-owning
scientific knowledge commons. It does not exist as standalone software.
Instead, it emerges from the composition of existing primals through the
sovereignty stack. projectFOUNDATION implements the client-side library
that weaves these layers together.

## The Sovereignty Stack

```
Layer 7: gAIa commons          — self-owning knowledge (endgame)
Layer 6: sunCloud               — value radiates along braids
Layer 5: Public anchor          — BTC/ETH 32-byte timestamp (postage only)
Layer 4: BearDog signatures     — cryptographic proof
Layer 3: sweetGrass braids      — attribution (WHO contributed)
Layer 2: loamSpine ledger       — permanent selective record
Layer 1: rhizoCrypt DAG         — ephemeral working memory
Layer 0: nestGate CAS           — content identity (BLAKE3)
```

### Layer mapping to FOUNDATION crates

| Layer | Crate | Status | Description |
|-------|-------|--------|-------------|
| 0 | `foundation-fetch` | LIVE | BLAKE3 content addressing, artifact registry |
| 1 | `foundation-ipc` | LIVE | rhizoCrypt `session.create`, `entry.append` |
| 2 | `foundation-ipc` | LIVE | loamSpine `certificate.create`, `spine.append` |
| 3 | `foundation-ipc` | LIVE | sweetGrass `braid.create`, `braid.weave` |
| 4 | `foundation-ipc` | LIVE | BearDog `auth.issue_session`, Ed25519 signing |
| 5 | `foundation-anchor` | DESIGN | Merkle root aggregation + public chain publish |
| 6 | `foundation-publish` | DESIGN | sunCloud radiating attribution calculation |
| 0+ | `foundation-ipc` | DESIGN | CAS federation `content.locate` cross-gate |

## Layer 5: Public Ledger Anchoring

### Concept

Public blockchains are used as timestamp ledgers — "postage," not
currency. The ecosystem writes 32-byte Merkle roots to BTC (OP_RETURN)
or ETH (calldata) to transform local provenance into globally verifiable
provenance.

Per whitePaper `gen4/architecture/ANCHORING_PIPELINE.md`:
- Aggregate N loamSpine certificates into a Merkle tree
- Write the 32-byte root to a public chain
- Cost: ~$0.001 per result (aggregated 1000:1)
- No tokens, wallets, exchanges, or DeFi — pure timestamp infrastructure

### Design: foundation-anchor crate

```
foundation-anchor/
  src/
    lib.rs          Public API
    merkle.rs       BLAKE3 Merkle tree construction from loamSpine certificates
    batch.rs        Aggregation: collect N certificates into a batch
    receipt.rs      Anchor receipt type (chain, tx_hash, root, timestamp, batch_size)
    chain/
      mod.rs        Chain trait + discovery
      bitcoin.rs    BTC OP_RETURN publisher (via RPC)
      ethereum.rs   ETH calldata publisher (via RPC)
      mock.rs       Local mock chain for testing
```

### Key types

```rust
/// A batch of loamSpine certificates aggregated for anchoring.
pub struct AnchorBatch {
    pub certificates: Vec<CertificateHash>,
    pub merkle_root: [u8; 32],
    pub timestamp: u64,
}

/// Receipt returned after successful anchoring.
pub struct AnchorReceipt {
    pub chain: ChainId,
    pub tx_hash: String,
    pub merkle_root: [u8; 32],
    pub batch_size: usize,
    pub cost_sats: u64,
}

/// Chain backend trait — implementations for BTC, ETH, mock.
pub trait AnchorChain {
    fn publish(&self, root: &[u8; 32]) -> Result<AnchorReceipt, AnchorError>;
    fn verify(&self, receipt: &AnchorReceipt) -> Result<bool, AnchorError>;
}
```

### Workflow

1. `foundation validate` runs, producing loamSpine certificates
2. Certificates accumulate in a local batch (configurable threshold, default 1000)
3. `foundation anchor` triggers: builds Merkle tree, publishes root
4. Receipt stored in nestGate CAS + loamSpine ledger entry
5. Any party can verify: fetch batch from CAS, rebuild tree, check against chain

## Layer 6: sunCloud Radiating Attribution

### Concept

sunCloud is the economic distribution protocol. It reads sweetGrass braids
(which record WHO contributed WHAT) and distributes value proportionally
along the attribution chain. Per whitePaper `economics/SUNCLOUD_ECONOMIC_MODEL.md`:

- Not a cryptocurrency. Not a payment processor.
- Value radiates backward through braids to every contributor.
- Metabolic rates: Infrastructure 3-7%, Science 2-5%, Product 0%, Ecosystem 1-3%.
- Attribution chain: experiment -> data -> compute -> cited work -> natural sources.

### Design: foundation-publish extension

```
foundation-publish/src/
  ... existing modules ...
  attribution/
    mod.rs          sunCloud attribution calculator
    metabolic.rs    Metabolic rate schedule (configurable per ecosystem wave)
    manifest.rs     Distribution manifest type (who gets what share)
    braid_reader.rs Read sweetGrass braids and extract contributor graph
```

### Key types

```rust
/// A contributor in the attribution graph.
pub struct Contributor {
    pub identity: BearDogIdentity,
    pub role: ContributionRole,
    pub weight: f64,
}

/// Metabolic rate schedule — how value is distributed by category.
pub struct MetabolicSchedule {
    pub infrastructure: RateRange,  // 3-7%
    pub science: RateRange,         // 2-5%
    pub product: RateRange,         // 0%
    pub ecosystem: RateRange,       // 1-3%
}

/// Distribution manifest — the output of attribution calculation.
pub struct DistributionManifest {
    pub source_braid: BraidHash,
    pub total_value: u64,
    pub shares: Vec<(Contributor, u64)>,
    pub unreachable_reverts_to_gaia: u64,
}
```

### Workflow

1. Validated science produces sweetGrass braids (already happening)
2. `foundation publish --attribution` reads braids, traverses contributor graph
3. Applies metabolic rate schedule to calculate proportional shares
4. Emits distribution manifest (JSON + loamSpine certificate)
5. Optionally anchors manifest to public ledger (Layer 5)
6. Unreachable beneficiary shares revert to gAIa commons pool

## CAS Federation (Layer 0 Extension)

### Concept

Cross-gate CAS federation enables any gate to discover and pull content
from any other gate without knowing which gate hosts it. This is the
"invisible data mesh" that connects westGate's 50 TB data CAS with
strandGate's compute memoization CAS.

### Design: foundation-ipc extension

```rust
/// Locate content across the gate mesh.
pub async fn content_locate(
    client: &PrimalClient,
    hash: &[u8; 32],
) -> Result<Vec<GateLocation>, IpcError> {
    // nestGate content.locate -> songBird federation -> mesh resolution
}

/// Pull content from a remote gate into local CAS.
pub async fn content_pull(
    client: &PrimalClient,
    hash: &[u8; 32],
    source: &GateLocation,
) -> Result<PullReceipt, IpcError> {
    // nestGate content.replicate.pull -> remote gate -> local CAS
    // Records the pull in rhizoCrypt DAG
}
```

### Dependency

NG-05 (nestGate TCP transport + content capability registration) must
ship before cross-gate federation is wired. westGate team owns NG-05.

## Gate Collaboration Surface

| Gate | Role | FOUNDATION interaction |
|------|------|----------------------|
| **westGate** | Data CAS (50 TB, 153 datasets) | `content.locate` discovers datasets; `content.pull` fetches to local CAS |
| **strandGate** | Compute NFT (SU(3)/SU(4) memoization) | `content.locate` discovers cached configs; provenance braids carry compute attribution |
| **ironGate** | Integration + publication node | Runs validation, builds Merkle trees, publishes anchors, calculates attribution |

## Relationship to whitePaper Economics

| whitePaper Concept | FOUNDATION Implementation |
|-------------------|--------------------------|
| gAIa (gen0 THE_SEED) | Emerges from composition of Layers 0-6 |
| sunCloud (economics/) | `foundation-publish` attribution module |
| Novel Ferment Transcripts (gen4) | CAS objects with provenance braids (strandGate compute NFTs) |
| The Flywheel (gen4) | Bootstrap: sweetGrass braids before revenue |
| Sovereign Publication (gen4 thesis) | `foundation validate` + `foundation anchor` pipeline |
| Radiating Attribution (economics/) | Distribution manifests from sweetGrass braid traversal |
| Public Anchor (gen4 architecture) | `foundation-anchor` crate: BTC/ETH Merkle root |
| Metabolic Mandate (economics/) | Configurable rate schedule in attribution module |

## Implementation Priority

1. **foundation-anchor** crate scaffold + Merkle tree construction (pure, testable)
2. **CAS federation** client in foundation-ipc (depends on NG-05)
3. **sunCloud attribution** module in foundation-publish (depends on sweetGrass braid format)
4. **CLI integration**: `foundation anchor` + `foundation publish --attribution`
5. **End-to-end**: validate -> anchor -> attribute -> distribute

---

*The substrate is the product. The science that passes automated
reproduction checks IS the publication. The attribution that flows
backward through braids IS the economics.*
