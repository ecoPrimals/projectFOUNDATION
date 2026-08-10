# gAIa Substrate Specification

**Date**: 2026-08-10 (Wave 157g — ENMESH)
**Status**: Design — implementation in progress
**Upstream**: whitePaper gen0-gen4 (gAIa vision, sunCloud economics, sovereignty stack)
**Local**: projectFOUNDATION (gAIa substrate library)
**Self-audit**: Method names verified against live primal surfaces on ironGate (Aug 9-10, 2026)
**P0 status**: ALL CLOSED (Wave 157g). 0 P0, 0 P1, 2 P2.

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

Method names verified against live primal surfaces on ironGate (Aug 9-10, 2026).

| Layer | Crate | Status | Actual RPC methods used |
|-------|-------|--------|-------------------------|
| 0 | `foundation-fetch` | LIVE | BLAKE3 content addressing, artifact registry |
| 0+ | `foundation-ipc` | LIVE types | nestGate `content.resolve`, `content.replicate.pull`, `content.stat`, `content.ingest` |
| 1 | `foundation-ipc` | LIVE | rhizoCrypt `dag.session.create`, `dag.event.append`, `dag.session.tree_hash` |
| 2 | `foundation-ipc` | LIVE | loamSpine `spine.create`, `entry.append`, `session.commit` |
| 3 | `foundation-ipc` | LIVE | `braid.commit` (loamSpine), `braid.verify` (sweetGrass — method #48, Wave 157g) |
| 4 | `foundation-ipc` | LIVE | BearDog `crypto.sign_ed25519`, `crypto.verify_ed25519` — P0-A CLOSED, real crypto confirmed |
| 5 | `foundation-anchor` | LIVE types, DESIGN chain backends | loamSpine `anchor.publish`, `anchor.publish_batch`, `anchor.verify` |
| 6 | `foundation-publish` | LIVE attribution math, DESIGN braid reader | sunCloud radiating attribution calculation |

### P0 resolution (Wave 157g — all CLOSED)

| Former P0 | Resolution | FOUNDATION impact |
|-----------|-----------|-------------------|
| P0-A (bearDog sign stub) | esotericWebb V32b confirmed real Ed25519 active. Depot binary fixed. | Layer 4 fully operational on all gates |
| P0-B (nestGate API mismatch) | nestGate `60ee88d8` shipped `content.stat/ingest/query/fetch` + `dataset.convergence`. HTTP transport parity. | Method constants updated. CAS federation surface complete. |
| P0-C (biomeOS FD leak) | Process leak CLOSED (coralReef RAII guards). biomeOS FD fix pending. | FOUNDATION uses direct UDS — unaffected |

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
3. `foundation anchor` triggers: builds Merkle tree via `foundation-anchor` crate
4. Publishes root via loamSpine `anchor.publish` (or `anchor.publish_batch` for bulk)
5. Receipt stored in nestGate CAS + loamSpine ledger entry
6. Any party can verify: `anchor.verify` against the chain, fetch batch from CAS, rebuild tree

Note: loamSpine already ships `anchor.publish`, `anchor.publish_batch`, and
`anchor.verify` as live RPC methods (verified on ironGate Aug 9, 2026).
The `foundation-anchor` crate's chain backend design can delegate to these
rather than implementing raw BTC/ETH RPC directly.

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
pub async fn content_resolve(
    client: &PrimalClient,
    hash: &[u8; 32],
) -> Result<Vec<GateLocation>, IpcError> {
    // nestGate content.resolve -> songBird federation -> mesh resolution
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

### Actual nestGate CAS surface (verified Aug 9, 2026)

All content methods require BTSP authentication. Available methods:

| Method | Purpose |
|--------|---------|
| `content.resolve` | Locate content by hash (federation lookup) |
| `content.put` | Store content in CAS |
| `content.get` | Retrieve content from CAS |
| `content.exists` | Check if content exists |
| `content.list` | List CAS contents |
| `content.replicate.pull` | Pull from remote gate |
| `content.publish` | Make content available for federation |
| `content.push` | Push content to remote gate |
| `storage.stats` | CAS statistics (object count, size, namespaces) |

Note: `content.ingest` (directory walk) and `content.stat` (single-object stat)
do NOT exist in nestGate v0.5.0 (P0-B). westGate pipeline adapted with Python
directory walk + per-file `content.put`.

### Dependency

NG-05 (nestGate TCP transport + content capability registration) must
ship before cross-gate federation is wired. westGate team owns NG-05.

## Gate Collaboration Surface

| Gate | Role | FOUNDATION interaction |
|------|------|----------------------|
| **westGate** | Data CAS (50 TB ZFS, 153 datasets, 989K files braided) | `content.resolve` discovers datasets; `content.replicate.pull` fetches to local CAS |
| **strandGate** | Compute NFT (SU(3)/SU(4) memoization) | `content.resolve` discovers cached configs; provenance braids carry compute attribution |
| **ironGate** | Integration + publication node (NUCLEUS host) | Runs validation, builds Merkle trees, publishes anchors via loamSpine `anchor.publish`, calculates attribution |
| **eastGate** | Overwatch node | Ecosystem audit, blurb distribution, self-audit coordination |
| **blueGate** | Marine research node | Domain-specific CAS federation participant |
| **sporeGate** | Cascade origin, depot source | golgi depot, binary distribution via `coord.depot.status` |

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

### Completed (Wave 157a-157g)

1. **foundation-anchor** crate scaffold — Merkle tree construction, batch aggregation,
   mock chain backend, receipt types (27 tests)
2. **sunCloud attribution** module — `calculate_attribution`, metabolic schedule,
   distribution manifests, gAIa commons reversion (16 tests)
3. **CAS federation** types — `GateLocation`, `LocateResult`, `PullReceipt`, `CasStats`,
   gate constants for 6 gates (8 tests)
4. **Method self-audit** — all RPC constants verified against live primal capability.list.
   48 method constants across 12 modules.

### Next (Pipeline Enmeshment Phase — Wave 157g)

1. **CAS federation client** — wire `content.resolve` + `content.replicate.pull` through
   BTSP-authenticated nestGate channel. Depends on gossip mesh enmeshment (TCP 7800
   cross-gate reachability + songBird MeshRelay).
2. **foundation-anchor chain backends** — delegate to loamSpine `anchor.publish` /
   `anchor.verify` rather than raw BTC/ETH RPC. loamSpine already ships these methods.
3. **Braid verification integration** — wire `braid.verify` (sweetGrass method #48)
   into the provenance commit workflow. Enables atomic verification of attribution chains.
4. **CLI integration**: `foundation anchor` + `foundation publish --attribution`
5. **E2E pipeline**: validate -> anchor -> attribute -> distribute (Science pipeline G71)

### ironGate Role in Enmeshment

ironGate is the "downstream host + NFT" gate in the blurb. FOUNDATION's role:
- **Validation hub**: Run foundation validate against data pulled from westGate CAS
- **Publication bridge**: esotericWebb, footPrint, pseudoSpore bundles
- **Anchor node**: Build Merkle trees from validation results, anchor to public ledger
- **Attribution calculator**: Read braids from validation runs, calculate distribution

### Dependencies on Other Teams

| Dependency | Owner | Status |
|-----------|-------|--------|
| Gossip mesh cross-gate TCP 7800 | All gates | OPEN — peers unreachable |
| songBird MeshRelay | songBird | OPEN — not yet shipped |
| BTSP auth flow for nestGate content methods | nestGate + bearDog | LIVE (bearDog BTSP functional) |
| `biome.yaml` manifest convergence | toadStool + biomeOS | 4 structs → 1 canonical |
| sourDough CI wiring | sporeGate + sourDough | OPEN — 12 validators exist, none in post-receive hook |

---

*The substrate is the product. The science that passes automated
reproduction checks IS the publication. The attribution that flows
backward through braids IS the economics.*
