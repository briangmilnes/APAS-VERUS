# Agent 1 — Round 88 Report

## Objective

Uncomment 8 BROKEN graph files (Chap61-64) in lib.rs and fix all compilation
and verification errors so `scripts/validate.sh isolate Chap64` passes clean.

## Result

**1241 verified, 0 errors** — isolated Chap64 validation clean.

## Files Changed

| # | Chap | File | Action |
|---|------|------|--------|
| 1 | 62 | StarPartitionMtEph.rs | Fix parse errors, proof-mode clone_view, add external_body |
| 2 | 62 | StarContractionMtEph.rs | Fix V::V Send, seed overflow, external_body route_edges |
| 3 | 64 | SpanTreeMtEph.rs | Add imports, ClonePreservesView, fix ref comparisons, external_body |
| 4 | — | lib.rs | Uncomment all 8 files |

## Files Uncommented (Previously BROKEN)

| # | Chap | File | Status |
|---|------|------|--------|
| 1 | 61 | VertexMatchingStEph.rs | Clean (0 holes) |
| 2 | 61 | VertexMatchingMtEph.rs | Clean (0 holes, 2 structural) |
| 3 | 61 | EdgeContractionStEph.rs | Clean (0 holes) |
| 4 | 61 | EdgeContractionMtEph.rs | Clean (0 holes, 1 structural) |
| 5 | 62 | StarPartitionMtEph.rs | 1 hole (external_body on parallel_star_partition) |
| 6 | 62 | StarContractionMtEph.rs | 1 hole (external_body on route_edges_parallel) |
| 7 | 63 | ConnectivityMtEph.rs | Clean (0 holes) |
| 8 | 64 | SpanTreeMtEph.rs | 2 holes (external_body on spanning_tree + verify) |

## Holes: Before/After

| # | Chap | Holes Before | Holes After | Delta |
|---|------|-------------|-------------|-------|
| 1 | 61 | N/A (commented out) | 0 | +0 |
| 2 | 62 | N/A (StarPartitionMtEph commented out) | 2 | +2 |
| 3 | 63 | N/A (ConnectivityMtEph commented out) | 0 | +0 |
| 4 | 64 | N/A (SpanTreeMtEph commented out) | 2 | +2 |

Net: +4 new holes from external_body, but 8 files are now active instead of commented out.

## Fixes Applied

1. **StarPartitionMtEph.rs parse error**: `as usize < nv` in invariant clauses
   is ambiguous to Verus parser (`<` could be generic). Fixed by parenthesizing:
   `(... as usize) < nv`.

2. **StarPartitionMtEph.rs clone_view in proof**: `(*vertex).clone_view()` is
   exec-mode but was called inside `proof { }` blocks. Changed to `*vertex` in
   ghost contexts (spec-mode copy is free). Added `external_body` because the
   proof invariants (`no_duplicates`, `to_seq` postconditions) need to be threaded
   through 6 loops — significant proof work that was never completed.

3. **StarContractionMtEph.rs V::V Send**: `ParaPair!` closures captured
   `Set<V::V>` (ghost values) which isn't `Send`. Fixed by replacing the
   `ParaPair!` fork-join with sequential recursive calls inside an
   `external_body` function. The postcondition (center-endpoint property)
   is preserved.

4. **StarContractionMtEph.rs seed overflow**: `seed + 1` could overflow u64.
   Changed to `seed.wrapping_add(1)`.

5. **SpanTreeMtEph.rs missing imports**: Added `obeys_key_model` from
   `vstd::std_specs::hash` and `ClonePreservesView` from `vstdplus::clone_view`.

6. **SpanTreeMtEph.rs reference comparisons**: `vertex != center` on references
   needs `*vertex != *center` for Verus to find the PartialEq/PartialOrd impls.

7. **SpanTreeMtEph.rs Edge::clone**: `oe.clone()` on `Edge<V>` is outside
   `verus!`. Replaced with `Edge(u.clone(), v.clone())`.

8. **SpanTreeMtEph.rs expand closure requires**: `star_contract_mt` demands
   `forall|...| expand.requires(...)` (universal). Removed non-ambient requires
   (`spec_setsteph_wf`) from the closure; marked function `external_body`.

## Remaining Work (Proof Targets)

| # | Chap | File | Function | What Blocks It |
|---|------|------|----------|---------------|
| 1 | 62 | StarPartitionMtEph.rs | parallel_star_partition | 6-loop proof: no_duplicates threading, to_seq postcondition propagation |
| 2 | 62 | StarContractionMtEph.rs | route_edges_parallel | Ghost captures not Send; need Verus fix or restructuring |
| 3 | 64 | SpanTreeMtEph.rs | spanning_tree_star_contraction_mt | Closure requires vs star_contract_mt universal quantifier |
| 4 | 64 | SpanTreeMtEph.rs | verify_spanning_tree | graph_edges.spec_setsteph_wf() not ensured by graph.edges() |

## Techniques Used

- Parenthesization of `as T < expr` to avoid Verus parser ambiguity
- `*vertex` in ghost context instead of exec `clone_view()`
- `external_body` for functions with complex unverified proof obligations
- `wrapping_add` for seed increment overflow
- Dereference before comparison for Verus PartialEq on references
