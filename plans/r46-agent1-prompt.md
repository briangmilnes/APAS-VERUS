# R46 Agent 1: Chap61 + Chap62 + Chap64 + Chap65 (8 holes)

## Assignment

Finish the graph chapters you and Agent 2 started. These are the leftovers
from R44+R45 that other agents couldn't close.

## Baseline

69 holes total. 4396 verified. Your chapters: Chap61 (1), Chap62 (3), Chap64 (2), Chap65 (2).

## CRITICAL: Read plans/parapair-is-not-a-blocker.md FIRST

Several remaining holes use ParaPair. It is fully provable. Read the pattern doc.

## Target Holes

### Chap61 — VertexMatchingMtEph (1 hole)

| # | Chap | File | Function | Line | Type | Notes |
|---|------|------|----------|------|------|-------|
| 1 | 61 | VertexMatchingMtEph.rs | build_edges_parallel | — | external_body | ParaPair divide-and-conquer |

This uses ParaPair to split edge building. Named closures with explicit ensures.

### Chap62 — StarContraction (3 holes)

| # | Chap | File | Function | Line | Type | Notes |
|---|------|------|----------|------|------|-------|
| 1 | 62 | StarContractionStEph.rs | star_contract | — | external_body | Recursive + generic closure |
| 2 | 62 | StarContractionMtEph.rs | star_contract_mt | — | external_body | ParaPair + recursive |
| 3 | 62 | StarPartitionMtEph.rs | — | — | external_body | Check which function |

Read the files to identify exact functions and line numbers. The recursive closure
pattern may need factoring into a named helper. For ParaPair functions, use the
named-closure pattern.

### Chap64 — TSPApproxStEph (2 holes)

| # | Chap | File | Function | Line | Type | Notes |
|---|------|------|----------|------|------|-------|
| 1 | 64 | TSPApproxStEph.rs | euler_tour | 89 | external_body | Recursive DFS |
| 2 | 64 | TSPApproxStEph.rs | shortcut_tour | 106 | external_body | List dedup |

Agent 2 reported `euler_tour` needs a "complete rewrite" due to recursive DFS with
&mut params. Read the code yourself — Agent 2 may have been too pessimistic. Check
if the &mut pattern follows `src/standards/mut_standard.rs`.

### Chap65 — Kruskal + Prim (2 holes)

| # | Chap | File | Function | Line | Type | Notes |
|---|------|------|----------|------|------|-------|
| 1 | 65 | KruskalStEph.rs | sort_edges_by_weight | 58 | external_body | Sort closure |
| 2 | 65 | PrimStEph.rs | prim_mst | 95 | external_body | PQ while loop |

`sort_edges_by_weight` wraps a Vec::sort_by closure. Check if vstd has sort specs.
`prim_mst` uses BinaryHeapPQ — check what ensures the PQ operations provide.

## What NOT to do
- Do NOT add `#[cfg(not(verus_keep_ghost))]` to anything. Forbidden on fn/impl/type.
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses.
- Do NOT sequentialize parallel (Mt) implementations.
- Do NOT claim ParaPair is a blocker.

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes.
Run `scripts/holes.sh src/Chap61/ src/Chap62/ src/Chap64/ src/Chap65/`.
Write your report to `plans/agent1-round46-report.md`.
