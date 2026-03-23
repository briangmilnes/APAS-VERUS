# Agent 4 — Round 63

You are Agent 4 working in `~/projects/APAS-VERUS-agent4`.
You are a **senior proof engineer**. Your job is to prove, not to catalog
reasons you can't. Read the code, read the error, write the proof.

## Baseline

- Main: 4504 verified, 0 errors, 6 holes, 2610 RTT, 147 PTT.
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Target: Close PQMinStPer frontier capacity assume — 1 hole (Chap53, PQMinStPer.rs:255)

### The assume

Line 255: `assume(frontier_updated@.len() + 1 < usize::MAX as nat)`

This is the persistent variant of the same pattern in PQMinStEph.rs (which Agent 3
is working on). The code structure is nearly identical.

### What's already proven

The R61 infrastructure is in place:

- `pq_explore` requires `vertex_universe.finite()` and
  `vertex_universe.len() + 1 < usize::MAX`
- `visited@.subset_of(vertex_universe)` maintained as loop invariant
- Visited capacity is CLOSED (uses `lemma_len_subset`)
- Frontier vertex tracking invariant: every frontier entry's `.1` is in
  `vertex_universe`
- Neighbor vertices ∈ vertex_universe established before inner loop

### Why the assume remains

Same reason as StEph: the frontier contains `Pair<Pair<P,V>,V>` entries.
Multiple entries can share the same vertex with different priorities. No simple
subset injection from frontier to vertex_universe exists.

### Proof strategy

Mirror whatever approach works for PQMinStEph.rs (Agent 3's target). The two
files are structurally identical. Key differences are type suffixes
(StPer vs StEph) and the neighbor iteration uses `neighbors.elements` instead
of `neighbors_seq = neighbors.to_seq()`.

**Read `src/Chap53/PQMinStEph.rs` first** for context, then apply the same
approach to PQMinStPer.rs. The proof structure should be nearly line-for-line
identical.

### Recommended approaches (try in order)

**Approach 1: Inner loop iteration bound.**

At inner loop start, `frontier_updated == frontier_new`. The loop runs `nlen`
times, each adding at most 1 element. So:

```
frontier_updated@.len() <= frontier_new@.len() + i
```

Add this as a loop invariant. Prove it with `lemma_len_union`. Then the assume
becomes:
```
frontier_new@.len() + i + 1 < usize::MAX
```

You need a bound on `frontier_new@.len() + nlen`. Use len_bound lemmas on both
`frontier` and `neighbors_seq`/`neighbors.elements`.

**Approach 2: Ghost max_edges parameter.**

Add `Ghost(max_edges): Ghost<nat>` to `pq_explore` with:
```rust
requires
    max_edges < usize::MAX,
    frontier_init@.len() <= max_edges,
    // Each graph call returns at most max_edges neighbors total.
```

Thread through as an invariant. Push the obligation to the caller.

**Approach 3: Lift the assume.**

If the full proof is too complex, lift the assume from the inner loop body to
`pq_explore`'s requires:
```rust
requires
    // Frontier capacity bound (TODO: prove from graph structure).
    forall|s: Set<...>| s.len() < usize::MAX, // placeholder
```

Even moving the assume to `pq_min_multi` is progress. Commit partial wins.

### PQMinStPer-specific notes

- Uses `AVLTreeSetStPer` and `AVLTreeSeqStPer` instead of StEph variants
- Neighbor access: `neighbors.elements.nth(i)` and `neighbors.elements@.len()`
  (not `neighbors_seq.nth(i)`)
- The len bound lemma is `lemma_wf_implies_len_bound_stper` (if it exists) or
  the StPer equivalent. Search for it:
  ```bash
  grep -rn "lemma_wf_implies_len_bound" src/Chap37/
  ```

### Coordination with Agent 3

Agent 3 is working the StEph variant in parallel. You may arrive at different
solutions — that's fine. If your approach works, Agent 3's file can adopt it
later. Don't block on Agent 3's progress.

## Validation

Run `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh` sequentially.
Write report to `plans/agent4-round63-report.md`. Push to `agent4/ready`.
