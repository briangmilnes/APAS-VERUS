# R88 Agent 3 — Fix ETSPMtEph + JohnsonMtEphI64 stale API, STEP 20

## Objective

Fix 2 files with stale imports and method calls.

Files to fix:
1. `src/Chap26/ETSPMtEph.rs`
2. `src/Chap59/JohnsonMtEphI64.rs`

## lib.rs — uncomment your files

Uncomment BOTH files in lib.rs. They are currently commented out with `// BROKEN`.
Remove the comment prefix.

## Isolation — use ONLY these commands for validation

For ETSPMtEph:
```bash
scripts/validate.sh isolate Chap26
```

For JohnsonMtEphI64:
```bash
scripts/validate.sh isolate Chap59
```

Do NOT run full `scripts/validate.sh`, `scripts/rtt.sh`, or `scripts/ptt.sh`.
Push to `agent3/ready` when both isolated validates are clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## What to fix

### ETSPMtEph.rs (Chap26)

Multi-threaded version of ETSPStEph (Euclidean TSP heuristic). The StEph version
compiles but has an rlimit issue on `lemma_combined_cycle` (separate problem —
that function is commented out in StEph). The MtEph version has type mismatches
and stale API calls.

Read `src/Chap26/ETSPStEph.rs` for the current StEph API. The MtEph should
wrap it in the standard coarse-locking pattern or use HFScheduler for parallelism.

### JohnsonMtEphI64.rs (Chap59)

Multi-threaded Johnson's all-pairs shortest paths using I64 weights. The StEph
version (`JohnsonStEphI64.rs`) compiles and verifies clean. The MtEph should
wrap it or parallelize the per-source Dijkstra calls.

Read:
- `src/Chap59/JohnsonStEphI64.rs` — working StEph version
- `src/Chap57/DijkstraStEphU64.rs` — Dijkstra that Johnson calls
- `src/Chap58/BellmanFordStEphI64.rs` — BellmanFord that Johnson calls

## Important

- Do NOT modify the StEph versions of these files.
- Do NOT modify ETSPStEph.rs (it has a separate rlimit issue).
- Do NOT add `assume` or `accept`.
- Use `external_body` on functions that are too hard to prove within step budget.

## STEP 20

## Report

Write `plans/agent3-round88-report.md`.
