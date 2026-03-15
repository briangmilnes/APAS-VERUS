# Agent 4 — Round 22: Verusify Chap58 Bellman-Ford

## Mission

Move Bellman-Ford (Chap58) from outside `verus!` to inside `verus!`. Currently the
algorithm code is wrapped in `#[cfg(not(verus_keep_ghost))]` — Verus never sees it.
It reports 0 holes only because it's invisible. This is the worst kind of "clean."

## Current State

Two files:
- `src/Chap58/BellmanFordStEphI64.rs` — 101 lines, real implementation
- `src/Chap58/BellmanFordStEphF64.rs` — 34 lines, empty stub

The I64 file has:
- A trait with one function signature inside `verus!`
- The entire algorithm outside `verus!` behind `#[cfg(not(verus_keep_ghost))]`
- Uses `std::collections::HashMap` (not Verus-compatible)

## Dependencies (all clean)

- `Chap06::WeightedDirGraphStEphI128` — clean
- `Chap56::SSSPResultStEphI64` — clean, 0 holes
- `Chap05::SetStEph` — 1 hole (unrelated MappingStEph::size)

## Step 1: Replace HashMap with Verus-compatible types

The algorithm uses `HashMap<usize, i64>` for distances. Replace with:
- `Vec<i64>` indexed by vertex (vertices are `0..n`), or
- `ArraySeqStEphS<i64>` from Chap18

Since vertices are `0..n`, a simple `Vec<i64>` with index access is cleanest. Verus
handles `Vec` well.

## Step 2: Move algorithm inside verus!

Remove all `#[cfg(not(verus_keep_ghost))]` gates. Move `bellman_ford` and
`reconstruct_predecessors` inside the `verus!` block. Put them in the trait impl.

## Step 3: Add specs

The trait currently has no requires/ensures:
```rust
pub trait BellmanFordStEphI64Trait {
    fn bellman_ford(graph: &WeightedDirGraphStEphI128<usize>, source: usize)
        -> (sssp: Result<SSSPResultStEphI64, String>);
}
```

Add at minimum:
```rust
fn bellman_ford(graph: &WeightedDirGraphStEphI128<usize>, source: usize)
    -> (sssp: Result<SSSPResultStEphI64, String>)
    requires
        source < graph@.len(),
        graph.spec_weighteddirgraphis_wf(),  // or whatever the wf predicate is
    ensures
        sssp is Ok ==> sssp.unwrap().spec_ssspresult_wf(),
        sssp is Ok ==> sssp.unwrap().source() == source,
```

Read `src/Chap56/SSSPResultStEphI64.rs` to understand SSSPResult's view and wf predicates.
Read `src/Chap06/WeightedDirGraphStEphI128.rs` to understand the graph's spec interface.

Full shortest-path correctness (`d(v) = delta_G(s,v)`) is hard — start with structural
ensures (wf, source correct, distances non-negative for non-negative graphs) and work
toward algorithmic correctness.

## Step 4: Add loop invariants

The algorithm has two nested loops:
- Outer: `for round in 0..n` (relaxation rounds)
- Inner: `for v in 0..n` (vertex scan with in-neighbor relaxation)

Add invariants maintaining:
- Distance array is well-formed (`distances.len() == n`)
- Source distance remains 0
- Distances are monotonically non-increasing across rounds
- After `k` rounds, distances are correct for paths of length ≤ k (the core invariant)

The core invariant may be hard to prove. Get structural invariants working first.

## Step 5: Handle the F64 variant

`BellmanFordStEphF64.rs` is an empty stub. Either:
- Leave it as a stub with a comment explaining float axiom dependency
- Copy the I64 structure with `f64` types and `external_body` on the algorithm

Prefer leaving as stub — float arithmetic axioms are a known project-wide blocker.

## Prose Reference

Read `prompts/Chap58.txt` for the textbook description of Bellman-Ford.

## Important

- Do NOT use `std::collections::HashMap` inside verus! — it's not supported.
- Do NOT add `assume` or `accept` in algorithmic code.
- `external_body` is acceptable ONLY on the `reconstruct_predecessors` helper if proving
  predecessor reconstruction is too complex. The main `bellman_ford` function must have
  a real body.
- `scripts/validate.sh` after changes — 0 errors.
- Read existing Chap57/DijkstraStEphI64.rs for a reference on how another shortest-path
  algorithm is structured inside verus!.

## Deliverables

- BellmanFordStEphI64.rs fully inside verus! with specs and loop invariants.
- `plans/agent4-round22-report.md`
- 0 errors on validate.
- Commit + push to `agent4/ready`.
