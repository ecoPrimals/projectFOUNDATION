# projectFOUNDATION

The gAIa substrate library — validated scientific lineage, sovereign
publication, and radiating attribution for the ecoPrimals ecosystem.

**Organization**: sporeGarden (products built on ecoPrimals)
**Generation**: gen4 — composition and deployment (Wave 76+, evolved Wave 157g → Wave 200+)
**License**: AGPL-3.0-or-later (code), ORC (system mechanics), CC-BY-SA 4.0 (docs)

> **Wave 200+ reconciliation (2026-11-04)**: Sovereignty COMPLETE. The organism now serves
> its own science sovereign: dromedary TLS on 7 subdomains, sporePrint 382 pages + 836
> papers, 21 sweetGrass braids, REEDit unified surface, Nostr + RSS federation.
> golgi VPS DOWN — cellMembrane DEPRECATED. Cloudflare DEPRECATED. LAN is just LAN.
> projectFOUNDATION's data catalog, provenance pipeline, and publication bridge now
> operate entirely on local eastGate infrastructure. No external dependencies.

## What This Is

projectFOUNDATION is evolving from "validate science" to "publish science
as sovereign, attributed, verifiable knowledge." It is the substrate library
for the gAIa vision: a self-owning scientific knowledge commons where
discoveries are cryptographically attributed, publicly anchored, and
economically distributed along sweetGrass braids.

```
                              gAIa Vision (endgame)
                                     ↑
                    ┌────────────────┼────────────────┐
                    │                │                │
              sunCloud          Public Anchor     CAS Federation
          (Layer 6: value    (Layer 5: BTC/ETH   (Layer 0: cross-gate
           radiates along     32-byte Merkle      content.locate for
           braids)            root = postage)     data + compute)
                    │                │                │
                    └────────────────┼────────────────┘
                                     │
                          projectFOUNDATION
                     (data catalog, validation,
                      provenance pipeline, IPC,
                      fetch, publish, anchor)
                                     │
                    ┌────────────────┼────────────────┐
                    │                │                │
              eastGate          strandGate        ironGate
           (brain+gut:       (compute NFT:     (NUCLEUS host:
            17 primals,       SU(3)/SU(4),      integration +
            96GB DDR5,        memoized configs)  publication)
            RTX 4070)
```

projectNUCLEUS defines HOW to deploy (primals, graphs, gates).
projectFOUNDATION defines WHAT to validate and HOW to publish
(data, targets, provenance, attribution, anchoring).
Products define WHO benefits (helixVision, esotericWebb, blueFish).

The foundation is the soil. The sovereignty stack grows on top.
gAIa is the fruit.

## One Foundation, Many Expressions

The science is not separate. Anderson localization threads through
microbial ecology, immunology, digester engineering, and plasma physics.
The provenance trio validates game sessions, biological samples, and
medical records using the same DAG/certificate/braid machinery.
barraCuda's GPU math powers lattice QCD, protein structure prediction,
environmental monitoring, and drug screening.

Separating these into independent foundations would sever the connections
that make the ecosystem powerful. The foundation is one interconnected
graph. Expression documents zoom into subgraphs.

## The 10 Domain Threads

| Thread | Domain | Key Springs | Key Contacts |
|--------|--------|-------------|-------------|
| 1 | Whole-Cell Modeling | hotSpring, wetSpring, healthSpring | ABG community |
| 2 | Plasma Physics / Lattice QCD | hotSpring | Murillo, Chuna, Bazavov |
| 3 | Immunology / Drug Discovery | wetSpring, airSpring, healthSpring | Gonzales, Lisabeth, Neubig |
| 4 | Environmental Genomics | wetSpring, airSpring | Anderson R., Cahill, Liao |
| 5 | Evolutionary Biology / LTEE | wetSpring, neuralSpring | Dolson, Waters |
| 6 | Agricultural Science | airSpring, groundSpring, wetSpring | Dong, Liao |
| 7 | Anderson Mathematics | hotSpring, groundSpring, wetSpring, neuralSpring | Kachkovskiy |
| 8 | Human Health / Clinical | healthSpring | Mok |
| 9 | Gaming / Creative | ludoSpring | — |
| 10 | Provenance / Economics | ludoSpring, primalSpring | — |

See `lineage/THE_UNIFIED_LINEAGE.md` for the complete mapping with
baseCamp paper numbers, spring check counts, and cross-thread connections.

## Products as Focuses

Products are lenses — each pulls specific threads from the foundation
and presents them for a particular audience.

| Product | Foundation Threads | Audience |
|---------|-------------------|----------|
| **helixVision** | 1, 3, 4, 5 | Field biologists, wastewater engineers, lab scientists |
| **blueFish** | 4 + data pipeline across all threads | Labs needing sovereign ETL and format conversion |
| **esotericWebb** | 9 | Game designers, writers, solo creators |
| **Patient Records** | 8, 10 | Clinicians, patients |
| **Games@Home** | 9 | Players, citizen scientists |

## Launch a Validation Run

### Rust UniBin (`foundation` — Phase B complete, Phase C in progress)

```bash
# Build the foundation binary (pure Rust, ecoBin-compliant, zero C deps)
cargo build --release

# Run the full validation pipeline (skip-fetch for local-only)
# Phases 1-8: health → provenance open → fetch check → registry → execute → compare → commit → report
target/release/foundation validate --skip-fetch

# Check all targets against manifests
target/release/foundation targets --check

# Inspect health of discovered primals (graceful degradation)
target/release/foundation health

# Fetch all sources from manifests (BLAKE3 verified)
target/release/foundation fetch

# Backfill BLAKE3 hashes for local data
target/release/foundation backfill

# Generate sporePrint gallery pages from lithoSpore pseudoSpore registry
target/release/foundation publish --registry ../lithoSpore/pseudospores/registry.toml

# Scan and index domain_profile.toml files from a spring
target/release/foundation profiles --scan-dir ../../springs/hotSpring --spring hotSpring

# Check spring/primal versions for drift (machine-readable JSON output)
target/release/foundation check-versions --eco-root ../../ --json
```

**Current state**: 7 crates, 258 tests, ~11.5k lines, zero clippy warnings.
8 subcommands (9th `anchor` in design). 48 RPC method constants across 12 modules,
all verified against live primal `capability.list` (self-audit Wave 157a/157g).
Typed errors throughout. All constants centralized. IPC with graceful degradation.
CAS federation types for cross-gate `content.resolve` (westGate data + strandGate
compute). `braid.verify` and `content.stat/ingest` constants added (Wave 157g closures).
Env-driven URL resolution (no hardcoded service endpoints). Zero `unsafe` in
production; test-only env manipulation isolated with `#[expect]` + mutex.

**Evolving**: The gAIa substrate adds three capabilities:
- **Layer 5** (`foundation-anchor`): Public ledger anchoring — aggregate N
  loamSpine certificates into Merkle tree, write 32-byte root to BTC/ETH
- **Layer 6** (`foundation-publish` extension): sunCloud radiating attribution —
  read sweetGrass braids, calculate attribution shares, emit distribution manifests
- **Layer 0** (`foundation-ipc` extension): CAS federation — cross-gate
  `content.locate` for westGate data CAS and strandGate compute memoization

**Phase C remaining**: NestGate registration, toadStool dispatch, full `ProvenanceSession`
trio, `backfill --write` TOML mutation, database-specific fetch orchestration, sporePrint
notify trigger from `publish`.

### Bash pipeline (legacy — pre-Phase C cutover)

```bash
# NUCLEUS primals already running on eastGate (17/17 via systemd)
# No VPS deployment needed — golgi is DOWN, eastGate runs everything

# Fetch public data sources and run validation with full provenance
cd "${FOUNDATION_ROOT:-$(git rev-parse --show-toplevel)}/deploy"
bash foundation_validate.sh --thread wcm
```

This fetches genomes from NCBI, proteomes from UniProt, pathway maps from
KEGG — hashes everything with BLAKE3, registers artifacts in NestGate,
executes validation workloads through toadStool, and commits the complete
provenance chain (rhizoCrypt DAG + loamSpine ledger + sweetGrass braid).

See `deploy/README.md` for full options and the sediment layer model.

### Current Publication Surfaces (Wave 200+)

The organism now publishes its science sovereign — no external hosting:

| Surface | URL | Content |
|---------|-----|---------|
| sporePrint | `sporeprint.primals.eco` | 382 pages, 836 papers — all via dromedary TLS |
| hypothesis | `hypothesis.primals.eco` | 52 formal hypotheses with falsification |
| detroit | `detroit.primals.eco` | 600+ pages municipal data investigation |
| tuebor | `tuebor.primals.eco` | 35 pages state-level analysis |
| REEDit | `primals.eco/reedit.html` | Unified surface — living tips of 21 sweetGrass braids |
| RSS | `primals.eco/feed.xml` | 1,560 items, WebSub/PubSubHubbub real-time |
| Nostr | NIP-05 verified | gaia@primals.eco + ecoPrimal@primals.eco |
| Tor | `.onion` hidden service | Censorship-resistant mirror of all surfaces |

## The Sovereignty Stack

projectFOUNDATION implements layers 0-6 of the gAIa sovereignty stack:

| Layer | Primal | FOUNDATION Crate | Status |
|-------|--------|-----------------|--------|
| 0 | nestGate CAS — content identity (BLAKE3) | `foundation-fetch` | LIVE |
| 1 | rhizoCrypt DAG — ephemeral working memory | `foundation-ipc` | LIVE |
| 2 | loamSpine ledger — permanent record | `foundation-ipc` | LIVE |
| 3 | sweetGrass braids — attribution | `foundation-ipc` | LIVE |
| 4 | BearDog signatures — cryptographic proof | `foundation-ipc` | LIVE |
| 5 | Public anchor — BTC/ETH 32-byte timestamp | `foundation-anchor` | Design |
| 6 | sunCloud — value radiates along braids | `foundation-publish` | Design |

## Repo Structure

```
crates/               Rust workspace — gAIa substrate library (ecoBin-compliant)
  foundation-core/      Types, TOML parsing, config discovery, env expansion
  foundation-ipc/       JSON-RPC 2.0 clients, health triad, provenance sessions,
                        CAS federation client (content.locate cross-gate)
  foundation-fetch/     Manifest-driven fetch, BLAKE3 content addressing, registry
  foundation-validate/  8-phase pipeline, comparison, execution, reporting
  foundation-publish/   Publication bridge, sporePrint gallery, sunCloud attribution
  foundation-anchor/    Public ledger anchoring — Merkle root to BTC/ETH (Layer 5)
  foundation-cli/       UniBin entry: validate, fetch, health, targets, backfill,
                        publish, profiles, check-versions, anchor
Cargo.toml            Workspace root (edition 2024, AGPL-3.0, clippy pedantic+nursery)
data/                 Data source manifests and validation targets (core identity)
  sources/            Per-thread data source TOMLs (11 files, 165 sources)
  targets/            Per-thread validation target TOMLs (11 files, 185 targets)
  data_catalog.toml   Master catalog (38 datasets / 362 GB / 17 domains)
graphs/               Foundation-specific deploy graphs (references projectNUCLEUS)
deploy/               Operational scripts (retiring — Phase C Rust parity in progress)
specs/                Contracts, boundary specs, hand-off map, gAIa substrate spec
validation/           Results, braids, gap reports (geological record)
docs/                 External-facing primers and guides
lineage/              Thread index + paper map (handing off to sporePrint)
expressions/          Domain thread docs (handing off to spring teams)
workloads/            toadStool workload TOMLs (handing off to spring teams)
benchmarks/           barraCuda CPU parity (handing off to barraCuda)
sporeprint/           Gallery content (handing off to lithoSpore)
```

## Relationship to Other Repos

| Repo | Org | Relationship |
|------|-----|-------------|
| **lithoSpore** | sporeGarden | First Targeted GuideStone — USB-deployable LTEE validation artifact |
| **projectNUCLEUS** | sporeGarden | The spore — deploys 17 primals, gate inventory, sovereignty infrastructure |
| **plasmidBin** | ecoPrimals/infra | Binary depot — 17 primal binaries fetched by nucleus-deploy |
| **primalSpring** | syntheticChemistry | Composition validation — foundation references validated graphs |
| **wateringHole** | ecoPrimals/infra | Standards and guidance — foundation follows these |
| **whitePaper** | ecoPrimals/infra | gAIa vision, sunCloud economics, sovereignty stack design |
| **helixVision** | sporeGarden | Product focus — pulls Threads 1, 3, 4, 5 from foundation |
| **blueFish** | sporeGarden (pending) | Product focus — pulls Thread 4 + data pipeline |
| **esotericWebb** | sporeGarden | Product focus — pulls Thread 9 from foundation |
| **dromedary** | ecoPrimals/primals | Sovereign TLS reverse proxy — serves all foundation publication surfaces |
