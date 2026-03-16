# Agent 4 — Round 23: Verusify Chap59 Johnson + Chap41 ArraySet filter

## Mission

Two independent targets:
1. Move Johnson's algorithm (Chap59) inside verus! — same pattern as BellmanFord in R22.
2. Prove ArraySetStEph::filter (Chap41) — 1 external_body, not blocked by Chap37.

## Part 1: Chap59 Johnson Verusification

### Current State

4 files, all with code outside verus! behind `#[cfg(not(verus_keep_ghost))]`:

| # | Chap | File | Lines | Status |
|---|------|------|------:|--------|
| 1 | 59 | JohnsonStEphI64.rs | 168 | Algorithm outside verus! |
| 2 | 59 | JohnsonMtEphI64.rs | 204 | Algorithm outside verus! |
| 3 | 59 | JohnsonStEphF64.rs | 15 | Empty stub |
| 4 | 59 | JohnsonMtEphF64.rs | 15 | Empty stub |

Focus on JohnsonStEphI64.rs. The Mt variant can follow if time permits.

### Dependencies (all clean)

- Chap06::WeightedDirGraphStEphI128 — clean
- Chap56::SSSPResultStEphI64 — clean
- Chap56::AllPairsResultStEphI64 — clean
- Chap57::DijkstraStEphI64 — clean (but weak specs)
- Chap58::BellmanFordStEphI64 — clean (you verusified it in R22!)

### Johnson's Algorithm Structure

Johnson's APSP has 3 phases:
1. **Bellman-Ford**: Run on augmented graph (dummy source vertex) to get potentials h(v)
2. **Reweight**: Transform edge weights: w'(u,v) = w(u,v) + h(u) - h(v) (non-negative)
3. **n Dijkstras**: Run Dijkstra from each vertex on reweighted graph, adjust distances back

### Approach

Same pattern as BellmanFord R22:
1. Replace any HashMap usage with Vec or ArraySeq
2. Remove `#[cfg(not(verus_keep_ghost))]` gates
3. Move all functions inside `verus!` trait impl
4. Add specs: requires (graph wf, no negative cycles for Ok result), ensures (result wf)
5. Add loop invariants for the n-Dijkstra loop
6. Reference your own BellmanFordStEphI64.rs from R22 for the pattern

### Key Challenge

Johnson calls `bellman_ford()` and `dijkstra()` — both are now inside verus! with specs.
You need their ensures to flow through. Read:
- `src/Chap58/BellmanFordStEphI64.rs` (your R22 work) for BF ensures
- `src/Chap57/DijkstraStEphI64.rs` for Dijkstra ensures (may be weak)

If Dijkstra's ensures are too weak for Johnson to use, add structural ensures to Johnson
and note what Dijkstra needs.

### F64 variants

Leave as stubs — float axiom dependency.

## Part 2: Chap41 ArraySetStEph::filter (1 hole)

### Current State

`src/Chap41/ArraySetStEph.rs` line 598: `external_body` on `filter`.

```rust
#[verifier::external_body]
fn filter<F: PredSt<T>>(
    &self,
    f: F,
    Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
) -> (filtered: Self)
```

This takes a predicate closure and returns elements satisfying it.

### Approach

- Read the filter implementation body
- Read `src/standards/using_closures_standard.rs` for how to verify closure-using functions
- The proof needs to connect `f.requires`/`f.ensures` to `spec_pred`
- Build the filtered result element by element, maintaining the invariant that each
  element satisfies `spec_pred`
- Note: ArraySetEnumMtEph already has a verified filter (weak spec: only `subset_of`).
  Check if you can use a stronger spec here.

### Dependencies

ArraySetStEph does NOT depend on Chap37 — its holes are internal. This is safe to prove
without the Chap37 root blocker.

## Important

- Do NOT add `assume` or `accept` in algorithmic code.
- `external_body` is acceptable ONLY on String creation helpers (same as BellmanFord).
- `scripts/validate.sh` after changes — 0 errors.
- Run `scripts/rtt.sh` to verify tests pass.
- Read existing verusified graph algorithms (BellmanFord, Dijkstra) for patterns.

## Deliverables

- JohnsonStEphI64.rs fully inside verus! with specs
- ArraySetStEph::filter proven (if achievable)
- `plans/agent4-round23-report.md`
- 0 errors on validate, RTT pass.
- Commit + push to `agent4/ready`.
