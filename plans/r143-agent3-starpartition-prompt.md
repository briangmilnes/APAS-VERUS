# R143 Agent 3 — Investigate and fix StarPartition loops 1,5 (Chap62). AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap62/StarPartitionMtEph.rs` — the parallel star partition.
Read `prompts/Chap62.txt` — APAS description of star partition.

Report file: `plans/r143-agent3-starpartition-report.md`

## Problem

2 DIFFERS (same function, two annotation sites):
```
StarPartitionMtEph.rs:55  — Loops 1, 5 sequential; loops 2, 3, 4, 6 parallel D&C
StarPartitionMtEph.rs:845 — same
```

Loops 2, 3, 4, 6 are already parallel. Loops 1 and 5 are sequential.

## Investigation

1. Read the current implementation carefully. Identify what loops 1 and 5 do.

2. Read APAS Chap62 to understand what the algorithm expects for these loops.

3. Determine whether loops 1 and 5 can be parallelized using the slice-backed
   primitives we now have: parallel filter, map, tabulate, scan, reduce, flatten.

4. If parallelizable, implement it. If not, document why with a clear explanation
   of the dependency structure that prevents parallelism.

## Common parallel patterns for graph algorithms

- **Vertex-parallel map**: `tabulate(|v| f(v), n)` — apply f to each vertex
- **Edge filtering**: `filter(edges, predicate)` — select edges matching condition
- **Prefix sum for compaction**: `scan` to compute output positions, then scatter
- **Star contraction**: mark vertices, relabel edges — often tabulate + filter

## Validation

Run `scripts/validate.sh isolate Chap62`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- If parallelization is not possible, explain the dependency structure clearly
  in the report. Do not annotate as ACCEPTED DIFFERENCE — the user will decide.
- Named closures with explicit ensures for any join() calls.

## When done

RCP.
