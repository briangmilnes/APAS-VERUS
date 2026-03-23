# Agent 5 — Round 62

You are Agent 5 working in `~/projects/APAS-VERUS-agent5`.
You are a **senior proof engineer**. Your job is to prove, not to catalog
reasons you can't. Read the code, read the error, write the proof.

## Baseline

- Main: ~4496 verified, 0 errors, 10 holes, ~2610 RTT, 147 PTT.
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Target 1: Chap47 ParaHashTableStEph `resize` wf fix — 1 hole (line 534)

Veracity reports `fn_missing_wf_requires` and `fn_missing_wf_ensures` on the trait
method `resize` in `src/Chap47/ParaHashTableStEph.rs`.

The function already requires `Self::spec_impl_wf(table)` and ensures
`Self::spec_impl_wf(&resized)` — and the default for `spec_impl_wf` IS
`spec_hashtable_wf`. But veracity wants the explicit type-level predicate.

### Fix path (mechanical, low risk)

1. Read `src/Chap47/ParaHashTableStEph.rs` lines 534-546 (trait signature for `resize`).

2. Add `spec_hashtable_wf(table)` to the `requires` clause and
   `spec_hashtable_wf(&resized)` to the `ensures` clause. Both are logically
   redundant with the existing `Self::spec_impl_wf` clauses — they just make
   the wf explicit.

3. The concrete implementations (VecChainedHashTableStEph, LinkedListChainedHashTableStEph)
   already assert `spec_hashtable_wf(&new_table)` before returning (see line ~558 in
   VecChainedHashTableStEph.rs). So this should verify cleanly.

4. Validate: `scripts/validate.sh` — must show 0 errors.

## Target 2: Chap53 PQMinStEph capacity bounds — 2 holes (lines 198, 225)

Two assumes in `pq_explore`:
- Line 198: `assume(visited@.len() + 1 < usize::MAX as nat)` — before `visited.union(...)`
- Line 225: `assume(frontier_updated@.len() + 1 < usize::MAX as nat)` — before `frontier_updated.union(...)`

### Why they exist

The function already receives `Ghost(vertex_universe)` and the caller `pq_min_multi`
already requires `vertex_universe.len() + 1 < usize::MAX`. But `pq_explore`'s own
requires clause doesn't include the vertex universe bounds, so the loop body can't
prove the collection sizes are bounded.

### Fix path (medium difficulty)

1. Read `src/Chap53/PQMinStEph.rs` — focus on:
   - `pq_explore` function signature (lines ~150-170): requires and ghost params
   - `pq_min_multi` (line ~270+): the caller that provides `vertex_universe`
   - The loop body (lines ~180-230): where the assumes live
   - Read `src/standards/capacity_bounds_standard.rs` for the correct pattern.

2. **Add vertex universe bounds to `pq_explore`'s requires**:
   ```rust
   requires
       // ... existing ...
       vertex_universe.finite(),
       vertex_universe.len() + 1 < usize::MAX as nat,
       visited_init@.subset_of(vertex_universe),
   ```

3. **Add loop invariants** to the outer while loop:
   - `visited@.subset_of(vertex_universe)` — visited stays within the universe
   - `visited@.len() <= vertex_universe.len()` — derived from subset + finite

   For the frontier bound, you need: `frontier_updated@.len()` is bounded by
   the number of distinct vertices. This is trickier because frontier entries
   are `Pair<Pair<P,V>,V>` not just `V`. However, frontier_updated grows by
   at most one entry per neighbor, and the neighbor count is bounded by the
   graph's vertex count.

4. **Replace the assumes with assertions** derived from the invariants.

5. Check that `pq_min_multi` (the caller) can satisfy the new requires.
   It already has `vertex_universe.finite()` and `vertex_universe.len() + 1 < usize::MAX`
   in its own requires, and passes `AVLTreeSetStEph::empty()` as `visited_init`
   (empty set is trivially a subset of anything).

### Key challenge

The frontier bound (assume #2) is harder than the visited bound (#1). The frontier
entries are `Pair<Pair<P,V>,V>` and Z3 must chain through nested Pair projections
to relate frontier size to vertex_universe. If you can close assume #1 but not #2,
that's still progress — commit what works.

### Strategy for the frontier bound

Consider a simpler invariant: instead of proving frontier ⊆ universe (which requires
Pair projection), prove `frontier_updated@.len() <= vertex_universe.len()` directly
by induction on the neighbor loop iteration count. Each iteration adds at most one
entry, and the loop runs at most `neighbors.size()` times, where neighbors come from
the graph which maps to vertices in the universe.

## Target 3: Chap53 PQMinStPer capacity bounds — 2 holes (lines 183, 204)

Same pattern as Target 2 but for the persistent variant.

1. Read `src/Chap53/PQMinStPer.rs` — find the same structure.
2. Apply the identical fix: add universe bounds to requires, add loop invariants,
   replace assumes.
3. The persistent variant uses `AVLTreeSetStPer` instead of `AVLTreeSetStEph` but
   the proof structure is the same.

## Execution Order

1. Target 1 first (quick win, mechanical)
2. Target 2 (medium difficulty, may need iteration)
3. Target 3 (mirrors Target 2)

Validate after each target. If Target 2 takes more than 5 iterations, commit
whatever progress you have and move to Target 3 — the insights from one will
inform the other.

## Validation

Run `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh` sequentially.
Write report to `plans/agent5-round62-report.md`. Push to `agent5/ready`.
