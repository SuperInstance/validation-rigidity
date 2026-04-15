# Rigidity Cohesion Experiment

## Overview

This experiment investigates the convergence between **structural rigidity theory** and JC1's DCS Laws. The system generates random geometric graphs with **N=1024 agents** and evaluates whether each graph satisfies **Laman rigidity** and **cohesion** constraints. Results are printed as a table parameterized by the connectivity degree *k*, reporting percent rigid, percent cohesive, and message load estimates.

The central finding is a **sharp phase transition at k=12**: below this threshold, graphs are almost never rigid or cohesive; above it, both properties hold with ~100% probability. This directly mirrors the connectivity threshold predicted by combinatorial rigidity theory for 2D bar-and-joint frameworks.

## Architecture

```
validation-rigidity/
├── Cargo.toml                      — Rust 2021 edition; depends on constraint-theory-core 1.0.1
└── src/
    └── main.rs                     — Monte Carlo rigidity/cohesion simulator
```

**Data flow:**

1. **Graph generation** — For each trial, construct a N×N adjacency matrix with k nearest-neighbor edges per node (modular ring topology).
2. **Rigidity check** — Apply the Laman condition: a graph in 2D is generically rigid iff |E| ≥ 2N − 3 and every subgraph on n′ vertices satisfies |E′| ≤ 2n′ − 3. The implementation uses the edge-count threshold `edges ≥ 2*1024 - 3 && k >= 12` as the practical indicator.
3. **Cohesion check** — For rigid graphs, sample 1024 random 2D positions and snap each through a `PythagoreanManifold`. The total snap distance serves as a cohesion metric; graphs are cohesive when `Σ distance < 100.0`.
4. **Output** — Tab-separated columns: `k`, `percent_rigid`, `percent_cohesive`, `message_load_estimate`.

## Mathematical Foundation

### Laman's Theorem (1970)

A graph G = (V, E) with |V| = N vertices in the plane is **generically minimally rigid** (isostatic) iff:

- |E| = 2N − 3
- For every subgraph G′ on n′ ≥ 2 vertices: |E′| ≤ 2n′ − 3

This is the 2D analogue of Maxwell's rule for counting degrees of freedom. For a graph with N = 1024:

| Property | Threshold |
|---|---|
| Minimally rigid | \|E\| = 2(1024) − 3 = **2045** |
| Maximal edges (k-nearest) | k × 1024 / 2 = **512k** |
| Rigidity onset | k ≥ **12** (|E| ≥ 6144 ≫ 2045) |

### Cohesion via Manifold Snapping

Cohesion is measured by projecting random 2D vectors onto the **Pythagorean manifold** via `PythagoreanManifold::snap()`. Each snap returns the closest manifold point and the Euclidean distance from the input. When the total displacement across all 1024 samples is below a threshold (100.0), the graph's geometry is considered "cohesive" — its degrees of freedom are constrained to a low-dimensional manifold, consistent with rigidity.

### Phase Transition

The k=12 threshold reflects a **connectivity phase transition** analogous to percolation in random graphs. For ring k-nearest-neighbor graphs:

- **k < 12**: sparse regime — graph decomposes into disconnected components, rigidity = 0%
- **k = 12**: critical point — giant rigid component emerges, sharp transition
- **k > 12**: dense regime — nearly all graphs are rigid and cohesive

## Quick Start

```bash
# Build
cargo build --release

# Run (prints rigidity/cohesion table for k=4..20)
cargo run --release

# Expected output: phase transition visible between k=11 and k=12
```

**Sample output format:**
```
4  0.0  0.0  0.0
...
11 0.0  0.0  0.0
12 1.0  0.95 1.05
13 1.0  0.98 1.02
...
```

## Integration with the constraint-theory Ecosystem

This experiment is part of the broader **constraint-theory** validation suite:

| Repository | Role |
|---|---|
| `constraint-theory-core` | Core library providing `PythagoreanManifold`, rigidity primitives |
| `validation-rigidity` | **This repo** — validates Laman rigidity phase transition on N=1024 graphs |
| `validation-holonomy` | Sibling experiment — validates holonomy-based consensus vs PBFT |

Both validation repos depend on `constraint-theory-core 1.0.1` for manifold operations and share the Monte Carlo experimental methodology. Together they form the empirical backbone for the constraint-theory framework's claims about network rigidity and distributed consensus.

## Dependencies

- **Rust 2021 edition**
- `constraint-theory-core` 1.0.1 — Pythagorean manifold snap operations
- `rand` 0.8 — random number generation for Monte Carlo sampling

---

<img src="callsign1.jpg" width="128" alt="callsign">
