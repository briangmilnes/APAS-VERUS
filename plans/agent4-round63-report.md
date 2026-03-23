# Agent 4 — Round 63 Report

## Assignment

Close PQMinStPer frontier capacity assume — 1 hole in `src/Chap53/PQMinStPer.rs`.

The assume `assume(frontier_updated@.len() + 1 < usize::MAX as nat)` guarded the
`AVLTreeSetStPer::union` precondition during frontier expansion. Tree wf gives
`frontier@.len() < usize::MAX` but union with singleton needs `frontier@.len() + 1 < usize::MAX`
— off by exactly 1.

## Approach: Ghost spec_priority + Injection Proof

Added a ghost `spec_priority: spec_fn(V::V) -> P::V` parameter that captures the
priority function's deterministic behavior at the view level. This enables an injection
proof: frontier entries are uniquely determined by their vertex component (via the entry
structure invariant `e.0 == (spec_priority(e.1), e.1)`), so `|frontier| <= |vertex_universe|`,
and since `vertex_universe.len() + 1 < usize::MAX`, the capacity bound follows.

### Key Technical Details

1. **Ghost parameter threading**: `Ghost(spec_priority): Ghost<spec_fn(V::V) -> P::V>`
   added to all 7 function signatures (trait pq_min, trait pq_min_multi, impl pq_min,
   impl pq_min_multi, free fn pq_min, free fn pq_min_multi, fn pq_explore).

2. **View-determinism precondition**: `forall|v: &V, p: P| priority_fn.ensures((v,), p) ==> p@ == spec_priority(v@)` — links exec priority_fn to ghost spec_priority.

3. **Entry structure invariant**: `forall|e| frontier@.contains(e) ==> e.0 == (spec_priority(e.1), e.1)` — maintained through both the initial frontier loop and the inner exploration loop.

4. **Injection proof** (replaces the assume):
   - Define `f(e) = e.1` (extract vertex component)
   - Prove `injective_on(f, frontier@)` using entry structure invariant
   - `frontier@.map(f) ⊆ vertex_universe` (from vertex tracking invariant)
   - `lemma_map_size` + `lemma_len_subset` gives `|frontier| <= |vertex_universe|`
   - Combined with `vertex_universe.len() + 1 < usize::MAX`: done

5. **clone_plus() vs clone()**: The original code used `neighbor.clone()` for one copy and
   `neighbor.clone_plus()` for another. `clone_plus()` verifies view equality with Z3 but
   `clone()` does not (both have `ensures cloned(...)` but Z3 handles them differently).
   Changed both copies to use `clone_plus()`. Also removed unnecessary `neighbor_p.clone()`
   / `p.clone()` (P values aren't reused after entry construction).

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 53 | PQMinStPer.rs | Ghost spec_priority param, injection proof, clone_plus fixes |
| 2 | — | tests/Chap53/TestPQMinStPer.rs | Added Ghost::assume_new() for new param at all call sites |

## Results

| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| Verified | 4504 | 4504 | 0 |
| Errors | 0 | 0 | 0 |
| Warnings | 0 | 0 | 0 |
| Holes | 6 | 5 | -1 |
| Clean chapters | 42 | 42 | 0 |
| Holed chapters | 4 | 4 | 0 |
| RTT | 2610 | 2610 | 0 |
| PTT | 147 | 147 | 0 |

### Hole Change Detail

| # | Chap | File | Before | After |
|---|------|------|--------|-------|
| 1 | 53 | PQMinStPer.rs | 1 | 0 |

### Remaining Holes (5)

| # | Chap | File | Holes | Type |
|---|------|------|-------|------|
| 1 | 43 | OrderedSetMtEph.rs | 1 | assume |
| 2 | 45 | BinaryHeapPQ.rs | 1 | assume |
| 3 | 47 | ParaHashTableStEph.rs | 2 | assume |
| 4 | 53 | PQMinStEph.rs | 1 | assume (Agent 3 assignment) |

## Techniques

- **Ghost spec function parameter**: Thread a `spec_fn` through signatures to capture
  exec function behavior at the view level
- **Set injection via `lemma_map_size`**: Prove `|S| == |S.map(f)|` when f is injective
  on S, then bound the image by a superset
- **clone_plus() for view equality**: `clone_plus()` (from vstdplus) enables Z3 to prove
  `result@ == original@` while standard `clone()` does not
- **Direct value move in Pair**: Avoid cloning P values when they're only used once
  (move directly into Pair constructor)
