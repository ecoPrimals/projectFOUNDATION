# projectFOUNDATION Hand-Off Map — Wave 157a

**Date**: 2026-08-08
**From**: ironGate code team
**Purpose**: Document assets being handed off from projectFOUNDATION to their
natural owners as FOUNDATION evolves from "validate science" to "publish
science as sovereign, attributed, verifiable knowledge" (gAIa substrate).

---

## Context

projectFOUNDATION was born during early atomics evolution (gen3/gen4) to
define the scientific lineage and prove the primals work. With G68 converged,
4/6 gates redeployed, and the primal substrate stabilized at ~140K tests,
much of FOUNDATION's early material belongs closer to the teams that own
each domain.

FOUNDATION retains its core identity: the data catalog, validation targets,
IPC clients, provenance pipeline, and the evolving gAIa publication substrate.

---

## Hand-Off Schedule

### 1. lineage/ -> sporePrint

| Asset | Files | New Owner |
|-------|-------|-----------|
| `lineage/THE_UNIFIED_LINEAGE.md` | Master narrative | sporePrint content team |
| `lineage/THREAD_INDEX.toml` | Machine-readable thread inventory | sporePrint content team |
| `lineage/BASECAMP_PAPER_MAP.toml` | Papers -> threads/springs | sporePrint content team |
| `lineage/SPRING_VERSIONS.toml` | Version drift baseline | Consumed by `check-versions` — stays in FOUNDATION |

**Rationale**: sporePrint owns the public content surface (sporePrint.eco).
Thread index and paper map are content metadata that belong alongside the
publication pipeline. `SPRING_VERSIONS.toml` stays because `foundation
check-versions` consumes it for drift detection.

**Migration**: sporePrint absorbs lineage content into its content tree.
FOUNDATION retains a `lineage/README.md` pointer to the canonical location.

### 2. expressions/ -> spring teams

| Asset | New Owner |
|-------|-----------|
| `ABG_WHOLE_CELL_REBUILD.md` | wetSpring / hotSpring |
| `PLASMA_QCD_SOVEREIGN_GPU.md` | hotSpring |
| `IMMUNO_DRUG_DISCOVERY.md` | wetSpring / healthSpring |
| `ENVIRONMENTAL_GENOMICS.md` | wetSpring / airSpring |
| `LTEE_EVOLUTIONARY_DYNAMICS.md` | wetSpring / neuralSpring |
| `LTEE_EVOLUTION.md` | wetSpring / neuralSpring |
| `SOVEREIGN_HEALTH.md` | healthSpring |
| `GAMING_CREATIVE_SCIENCE.md` | ludoSpring |
| `PROVENANCE_ECONOMICS.md` | ludoSpring / primalSpring |
| `ML_SURROGATES.md` | neuralSpring |
| `MEASUREMENT_SCIENCE.md` | groundSpring |

**Rationale**: Each spring knows its domain better than a centralized
foundation. Expression documents are per-domain narratives that should
evolve with the spring code, not separately.

**Migration**: Each spring absorbs its expression docs into a `docs/` or
`lineage/` directory. FOUNDATION retains `expressions/README.md` as a
cross-reference index pointing to per-spring locations.

### 3. workloads/ -> toadStool catalog or per-spring repos

| Asset | Files | New Owner |
|-------|-------|-----------|
| `workloads/thread01_wcm/` | 4 workload TOMLs | wetSpring |
| `workloads/thread02_plasma/` | 3 workload TOMLs | hotSpring |
| `workloads/thread03_immuno/` | 3 workload TOMLs | wetSpring |
| `workloads/thread04_enviro/` | 3 workload TOMLs | wetSpring / airSpring |
| `workloads/thread05_ltee/` | 3 workload TOMLs | wetSpring |
| `workloads/thread06_ag/` | 2 workload TOMLs | airSpring / groundSpring |
| `workloads/thread07_anderson/` | 3 workload TOMLs | hotSpring |
| `workloads/thread08_health/` | 2 workload TOMLs | healthSpring |
| `workloads/thread09_gaming/` | 3 workload TOMLs | ludoSpring |
| `workloads/thread10_provenance/` | 3 workload TOMLs | primalSpring |
| `workloads/hotspring/` | hotSpring-specific workloads | hotSpring |
| `workloads/groundspring/` | groundSpring-specific workloads | groundSpring |

**Rationale**: Workloads belong near the compute dispatch system (toadStool)
or the spring that defines the science. FOUNDATION should declare WHAT to
validate (targets), not HOW to execute it (workloads).

**Migration**: Each spring absorbs its workload TOMLs into a `workloads/`
directory. toadStool maintains a catalog index that discovers workloads from
spring repos at runtime. FOUNDATION retains `workloads/README.md` as a
cross-reference.

### 4. benchmarks/ -> barraCuda or benchScale

| Asset | New Owner |
|-------|-----------|
| `benchmarks/barracuda_cpu_parity/` (6 Python scripts, 32 test cases) | barraCuda repo or benchScale infra |

**Rationale**: CPU parity benchmarks are compute-primal concerns. They
should live with barraCuda (GPU math primal) or benchScale (infra
benchmarking framework in projectNUCLEUS).

**Migration**: barraCuda absorbs the benchmark scripts. FOUNDATION removes
the directory and documents the new location.

### 5. sporeprint/ -> lithoSpore

| Asset | New Owner |
|-------|-----------|
| `sporeprint/validation-summary.md` | lithoSpore / sporePrint |
| `sporeprint/spores/` | lithoSpore |

**Rationale**: lithoSpore owns pseudoSpore packaging and sporePrint owns
content publication. Gallery generation artifacts belong with the publication
pipeline, not the knowledge layer.

**Migration**: lithoSpore absorbs gallery content. `foundation-publish`
crate continues to generate gallery pages but writes to lithoSpore's tree
rather than a local `sporeprint/` directory.

### 6. deploy/*.sh -> retire after Phase C

| Asset | Disposition |
|-------|-------------|
| `deploy/foundation_validate.sh` | Retire when `foundation validate` reaches Phase C parity |
| `deploy/fetch_sources.sh` | Retire when `foundation fetch` reaches Phase C parity |
| `deploy/backfill_hashes.sh` | Retire when `foundation backfill --write` ships |
| `deploy/lib/*.sh` (6 modules) | Archive after Rust parity |
| `deploy/discovery_defaults.toml` | Already consumed by Rust; bash reader becomes vestigial |

**Rationale**: The Rust UniBin is the typed, testable successor. The bash
layer is transitional (Phase B++ complete, Phase C in progress, Phase D
retires bash).

**Migration**: No external hand-off — FOUNDATION retires these internally
as Phase C items ship.

---

## FOUNDATION Retains (Core Identity)

| Asset | Why |
|-------|-----|
| `data/sources/*.toml` | The catalog of WHAT to validate |
| `data/targets/*.toml` | Numerical targets with tolerances |
| `data/data_catalog.toml` | Master data inventory (38 datasets / 362 GB / 17 domains) |
| `crates/foundation-core` | Typed vocabulary consumed by other gardens |
| `crates/foundation-ipc` | JSON-RPC clients for primal IPC |
| `crates/foundation-fetch` | Manifest-driven data acquisition with BLAKE3 |
| `crates/foundation-validate` | 8-phase pipeline |
| `crates/foundation-publish` | Publication bridge |
| `crates/foundation-cli` | UniBin entry point |
| `graphs/` | Foundation-specific deploy graph references |
| `specs/` | Contracts and boundary specs |
| `validation/` | Results, braids, gap reports (geological record) |
| `docs/` | Primers and guides |

---

## FOUNDATION Gains (gAIa Substrate Evolution)

| New Capability | Crate | Layer | Description |
|---------------|-------|-------|-------------|
| Public ledger anchoring | `foundation-anchor` | 5 | Merkle root publishing to BTC/ETH |
| Radiating attribution | `foundation-publish` (extended) | 6 | sunCloud attribution calculation from sweetGrass braids |
| CAS federation client | `foundation-ipc` (extended) | 0 | Cross-gate `content.locate` for westGate data + strandGate compute |

---

## Execution Notes

Hand-offs are documented here. The actual file migrations happen through
normal ecosystem cascade: each team absorbs their assets on their own
schedule. FOUNDATION marks handed-off directories with README pointers
to canonical locations.

No content is deleted until the receiving team confirms absorption.

---

*Wave 157a — projectFOUNDATION evolves from "validate science" to "publish
science as sovereign, attributed, verifiable knowledge."*
