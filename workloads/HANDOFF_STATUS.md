# workloads/ — Hand-Off Status

**Wave 157a**: These workload TOMLs are being handed off to their natural
owners per `specs/HANDOFF_MAP.md`.

## Migration Target

Thread-scoped workloads migrate to the spring that owns the science.
Cross-cutting workloads migrate to projectNUCLEUS (which already maintains
a parallel `workloads/` tree organized by spring).

| Directory | Absorbing Repo |
|-----------|---------------|
| `thread01_wcm/` | wetSpring / hotSpring |
| `thread02_plasma/` | hotSpring |
| `thread03_immuno/` | wetSpring / healthSpring |
| `thread04_enviro/` | wetSpring / airSpring |
| `thread05_ltee/` | wetSpring |
| `thread06_ag/` | airSpring / groundSpring |
| `thread07_anderson/` | hotSpring |
| `thread08_health/` | healthSpring |
| `thread09_gaming/` | ludoSpring |
| `thread10_provenance/` | primalSpring |
| `hotspring/` | hotSpring |
| `groundspring/` | groundSpring |

## Status

Files remain here until each receiving team confirms absorption.
The `foundation validate` pipeline discovers workloads via
`data/targets/*.toml` — it does not hard-depend on this directory.
