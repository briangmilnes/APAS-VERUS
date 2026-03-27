# Agent 3 — Round 84 Report

## Objective

Remove `external_body` from `kruskal_mst` in `KruskalStEph.rs`.

## Result

**Partial success.** The `external_body` was removed from `kruskal_mst` and the function
body is now verified for phases 1-3 (vertex insertion, edge collection, sorting). The
greedy edge-selection loop (phase 4) was factored into a separate helper function
`kruskal_greedy_phase` which retains `external_body` due to Z3 memory exhaustion (19+ GB)
on the loop invariant verification.

## Holes Before / After

| # | Chap | File | Before | After | Notes |
|---|------|------|--------|-------|-------|
| 1 | 65 | KruskalStEph.rs | 1 (kruskal_mst) | 1 (kruskal_greedy_phase) | Hole moved to smaller helper |

**Net change: 0 holes removed.** The hole moved from the main algorithm to a smaller,
more focused helper function.

## Changes Made

### KruskalStEph.rs

1. **Removed `external_body` from `kruskal_mst`** (line 178).

2. **Added `broadcast use crate::Types::Types::group_LabEdge_axioms`** — needed for
   `SetStEph::empty()` precondition (`valid_key_type::<LabEdge<V, WrappedF64>>()`).

3. **Added `#[cfg(verus_keep_ghost)] use vstd::float::FloatBitsProperties`** — needed
   for `f64::is_finite_spec()` in proof blocks.

4. **Added finiteness precondition** to `kruskal_mst`:
   ```
   forall|e: (<V as View>::V, <V as View>::V, f64)|
       #[trigger] graph@.A.contains(e) ==> e.2.is_finite_spec()
   ```
   Needed for `sort_edges_by_weight` which requires finite weights for comparisons.
   Note: single-variable trigger `graph@.A.contains(e)` was essential — the three-variable
   trigger `graph@.A.contains((u, w, wt))` did not fire in the Verus SMT encoding.

5. **Restructured iterator loops to index-based `while` loops** using `to_seq()`. The
   original `loop { if let Some(v) = iter.next() { ... } else { break; } }` pattern
   loses the `next()` None ensures after `break` — Verus's loop abstraction doesn't
   carry information from the breaking iteration. Index-based `while vi < seq.len()`
   gives `vi >= seq.len()` after loop exit, enabling the vertex-to-UF bridge proof.

6. **Added `lemma_sorted_edge_in_uf`** — proof function that chains: sort provenance →
   pre_sort view → edge_seq view → mapped_es → labeled_view → graph@.A → graph wf →
   UF domain. Factored to reduce rlimit pressure on the greedy loop.

7. **Factored `kruskal_greedy_phase`** as a separate function with its own rlimit budget.
   Currently `external_body` — see "What Blocks the Greedy Loop" below.

## What Blocks the Greedy Loop

The greedy loop maintains 12 quantifier-heavy invariants across calls to `UnionFindStEph::equals`
(modifies UF), `SetStEph::insert` (modifies MST set), and `UnionFindStEph::union` (modifies
UF). The `spec_unionfindsteph_wf` predicate alone has 13 conjuncts, each a forall quantifier
over the UF maps.

At rlimit(50), Z3 uses 10-20 GB RSS and either times out or OOMs (machine has 32 GB).
Attempted mitigations:
- Domain preservation invariant (`uf@.parent.dom() =~= initial_dom`) instead of full
  vertex-forall — reduced one quantifier but overall still OOM.
- Factoring endpoint proof into `lemma_sorted_edge_in_uf` — eliminated 8 assertions from
  the loop body but UF wf maintenance is the bottleneck.
- Various rlimit values (40-80) — all either timeout or OOM.

**Root cause:** The combination of UF wf (13-quantifier conjunction) + domain preservation
+ 6 fixed ghost invariants creates a quantifier instantiation explosion that exceeds the
machine's 32 GB RAM.

**Possible approaches for future rounds:**
- Verify on a machine with 64+ GB RAM.
- Factor `equals`/`union` to expose simpler postconditions (just domain preservation).
- Add a `spec_unionfind_domain_only_wf` predicate with fewer conjuncts.
- Use Verus's `--profile` to identify the specific quantifier matching loop.

## Verification

```
scripts/validate.sh isolate Chap65
verification results:: 2398 verified, 0 errors
```
