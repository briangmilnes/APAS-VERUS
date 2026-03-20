# R44 Agent 1: Chap61 + Chap62 (17 holes)

## Assignment

Prove external_body functions in Chap61 (EdgeContraction, VertexMatching) and
Chap62 (StarPartition, StarContraction). All 17 holes are external_body on
functions with complete implementations already present.

## Baseline

125 holes total. 4366 verified. Your chapters: Chap61 (8), Chap62 (9).

## Target Holes

| # | Chap | File | Function | Line |
|---|------|------|----------|------|
| 1 | 61 | EdgeContractionStEph.rs | edge_contract | 65 |
| 2 | 61 | EdgeContractionStEph.rs | contract_round | 116 |
| 3 | 61 | EdgeContractionMtEph.rs | build_edges_parallel | 127 |
| 4 | 61 | EdgeContractionMtEph.rs | contract_round_mt | 193 |
| 5 | 61 | VertexMatchingStEph.rs | parallel_matching_st | 82 |
| 6 | 61 | VertexMatchingMtEph.rs | parallel_matching_mt | 67 |
| 7 | 61 | VertexMatchingMtEph.rs | flip_coins_parallel | 90 |
| 8 | 61 | VertexMatchingMtEph.rs | should_select_edge | 199 |
| 9 | 62 | StarPartitionStEph.rs | sequential_star_partition | 47 |
| 10 | 62 | StarPartitionMtEph.rs | parallel_star_partition | 51 |
| 11 | 62 | StarContractionStEph.rs | star_contract | 68 |
| 12 | 62 | StarContractionStEph.rs | build_quotient_graph | 94 |
| 13 | 62 | StarContractionStEph.rs | contract_to_vertices | 127 |
| 14 | 62 | StarContractionMtEph.rs | star_contract_mt | 72 |
| 15 | 62 | StarContractionMtEph.rs | build_quotient_graph_parallel | 98 |
| 16 | 62 | StarContractionMtEph.rs | route_edges_parallel | 119 |
| 17 | 62 | StarContractionMtEph.rs | contract_to_vertices_mt | 173 |

## Strategy

**Start with the St (sequential) files.** They are simpler. Then adapt the pattern
to Mt files.

### Proof patterns that work (from R43 Agent 2):

1. **Sequential loops**: Remove `external_body`, add loop invariant. Use `for x in
   collection.iter()` with `invariant valid_key_type::<V>()` or `spec_graphview_wf(graph@)`.

2. **HashMapWithViewPlus iteration**: `for (k, v) in map.iter()` works. Use
   `clone_plus()` (from `crate::vstdplus::clone_plus::clone_plus::*`) for owned copies.

3. **SetStEph construction**: `SetStEph::empty()` or `SetLit![]` for initial value.
   `result.insert(x.clone_plus())` in loop body.

4. **Delegation wrappers** (e.g., `contract_to_vertices` calls `star_contract`):
   Just remove `external_body` — the body already delegates.

5. **Recursive functions** (`star_contract`, `count_components`): These are recursive
   with external_body. You may need to keep external_body on recursive functions that
   Verus can't verify the termination of. Focus on the non-recursive helpers first.

### Key imports you'll need:
```rust
use crate::vstdplus::clone_plus::clone_plus::*;
```

### What NOT to do:
- Do NOT add `#[cfg(not(verus_keep_ghost))]` to anything. It is forbidden on fn/impl/type.
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses to make proofs easier.
- Do NOT sequentialize Mt (parallel) implementations.

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes. Run `scripts/holes.sh src/Chap61/ src/Chap62/`.
Write your report to `plans/agent1-round44-report.md`.
