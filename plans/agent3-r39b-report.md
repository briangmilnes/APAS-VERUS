# Agent 3 Round 39b Report

## Summary

Removed both `assume()` holes from DijkstraStEphU64.rs (budget tracking + PQ heap
invariant), making the file proof-clean. The heap invariant fix required adding a
`bubble_up_heap` lemma to BinaryHeapPQ.rs with a strengthened `insert` postcondition.
The budget tracking fix required strengthening SetStEph's `iter()` postcondition to
provide element-wise membership, bypassing a Verus lambda unification limitation.

## Results

- **Verified**: 4341 (unchanged)
- **Holes**: 158 (was 160, **-2**)
- **RTT**: 2613 pass
- **PTT**: 147 pass

## Holes Before/After per File

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 57 | DijkstraStEphU64.rs | 2 | 0 | -2 | Both assumes proven; file is proof-clean |
| 2 | 45 | BinaryHeapPQ.rs | 0 | 0 | 0 | Added `bubble_up_heap` lemma, strengthened `insert` ensures |
| 3 | 05 | SetStEph.rs | 0 | 0 | 0 | Strengthened `iter()` ensures with element-wise membership |

## Technique

### PQ heap invariant assume (heap insert maintains heap property)

The `assume(BinaryHeapPQ::spec_is_exec_heap(pq.spec_seq()))` after each `pq.insert()`
was needed because `insert` didn't guarantee heap property preservation.

**Fix**: Added `proof fn bubble_up_heap` to BinaryHeapPQ.rs that proves inserting an
element and bubbling up maintains the min-heap property. Strengthened
`BinaryHeapPQ::insert` postcondition to include
`spec_is_exec_heap(self.spec_seq()) ==> spec_is_exec_heap(result.spec_seq())`.

### Budget tracking assume (remaining_budget > 0)

The `assume(remaining_budget > 0)` was needed to ensure the PQ size stays bounded
(total inserts <= |E|). The proof required showing each PQ insert corresponds to a
unique graph edge.

**Fix — ghost set approach**: Track `ghost mut used_edges: Set<(usize, usize, i128)>`
that accumulates edges used for PQ inserts. Key invariants:
- `used_edges.subset_of(graph@.A)` — every used edge is a graph edge
- `used_edges.len() == m - remaining_budget` — tight tracking
- `used_edges.contains(e) ==> visited@.contains(e.0)` — edges come from visited vertices
- `it@.1.no_duplicates()` — iterator elements are distinct
- Edge uniqueness: `e.0 != v || exists j < it@.0 ...` — each v-edge maps to a consumed
  iterator position

On each insert, the ghost set grows by one fresh edge. Since
`used_edges.insert(new_edge).subset_of(graph@.A)` and `!used_edges.contains(new_edge)`,
`vstd::set_lib::lemma_len_subset` gives `|used_edges| + 1 <= |graph@.A|`, proving
`remaining_budget > 0`.

**Blocked by lambda unification**: The proof needed `graph@.A.contains(new_edge)`,
which required connecting iterator elements to the `neighbors` set view. SetStEph's
`iter()` postcondition `it@.1.map(|i, k| k@).to_set() == self@` uses a lambda that
doesn't unify with lambdas at other source locations (Verus assigns different DefIds
to syntactically identical lambdas at different locations).

**Fix — iter() postcondition strengthening**: Added a new ensures clause to
`SetStEph::iter()`:
```rust
forall |j: int| 0 <= j < it@.1.len() ==> self@.contains(#[trigger] it@.1[j]@)
```
Provable within iter()'s own scope from `HashSetWithViewPlus::iter()` postcondition
(`s.contains(k) ==> self@.contains(k@)`) — no lambda needed. This gives Dijkstra
direct element membership, which combines with `out_neighbors_weighed`'s postcondition
to prove `graph@.A.contains(new_edge)`.

## AVLTreeSetStEph (assessed, not fixed)

Chap41 AVLTreeSetStEph.rs has 2 assumes for `insert` length bounds
(`old(self)@.len() <= usize::MAX - 1`). These are irreducible without either:
1. Adding a `requires self@.len() < usize::MAX` to `insert` (cascading API change), or
2. A `spec_avltreesetsteph_wf` predicate that includes a size bound

Both require touching all callers. Deferred to a coordinated API change round.

## Chapters Affected

- **Chapter 57**: DijkstraStEphU64.rs — 2 holes removed, now proof-clean
- **Chapter 45**: BinaryHeapPQ.rs — strengthened insert postcondition (no hole change)
- **Chapter 05**: SetStEph.rs — strengthened iter() postcondition (no hole change)
