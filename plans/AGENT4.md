# Agent 4 Round 6 Report

## Summary

Round 6: -10 holes across Chap45, Chap53, Chap65. 3771 verified, 0 errors.
RTT: 2600 passed. PTT: 147 passed.

## Changes

### BinaryHeapPQ.rs multiset preservation proofs (-5 holes)

Proved that insert, meld, and delete_min preserve multisets.

| # | Function | Technique | Holes |
|---|----------|-----------|-------|
| 1 | empty | Assert view =~= empty Seq | -1 (prior) |
| 2 | singleton | Assert view =~= empty.push(element@) | -1 (prior) |
| 3 | swap_elements | Rewrite with ArraySeqStPerS::update + to_multiset_update lemma | 0 (foundation) |
| 4 | bubble_up | Add multiset invariant + lemma_seq_map_cloned_view_eq after clone | 0 (foundation) |
| 5 | bubble_down | Same pattern as bubble_up | 0 (foundation) |
| 6 | heapify | Same pattern, handle early return for len <= 1 | 0 (foundation) |
| 7 | delete_min (len==1) | Split clone, assert cloned() for bridge | -1 |
| 8 | insert | Connect append view to self@ + single_seq@, call lemma_multiset_commutative | -1 |
| 9 | meld | Same append-view pattern as insert | -1 |
| 10 | delete_min (len>1) | Track multiset through loop via subrange invariant, decompose self@ = take(1) + skip(1) | -1 |

Key techniques:
- `cloned(*ref, value)` assertion triggers `axiom_cloned_implies_eq` broadcast for clone bridges
- `lemma_seq_map_cloned_view_eq(old.seq@, new.seq@)` establishes view equality after clone
- `vstd::seq_lib::to_multiset_update(s, i, a)` for swap multiset preservation
- `vstd::seq_lib::lemma_multiset_commutative(a, b)` for `(a + b).to_multiset() =~= a.to_multiset().add(b.to_multiset())`

### GraphSearch clone bridge fixes (-2 holes)

| # | File | Change | Holes |
|---|------|--------|-------|
| 1 | GraphSearchStEph.rs | Add obeys_feq_clone to SelectionStrategy::select requires, use cloned() | -1 |
| 2 | GraphSearchMtPer.rs | Same pattern | -1 |

### PQMinStEph clone bridge fix (-1 hole)

| # | File | Change | Holes |
|---|------|--------|-------|
| 1 | PQMinStEph.rs | Add obeys_feq_clone to pq_find_min_priority requires, use cloned() | -1 |

### UnionFindStEph eq bridge fix (-1 hole)

| # | File | Change | Holes |
|---|------|--------|-------|
| 1 | UnionFindStEph.rs | Replace assume in equals() with feq() call | -1 |

## Not Attempted (and why)

| Category | Holes | Reason |
|----------|-------|--------|
| BinaryHeapPQ sortedness (2) | assume | Requires heap property invariant through all operations |
| BalancedTreePQ (13) | external_body | Uses non-Verus standard library APIs (binary_search, sort) |
| GraphSearchStPer SelectOne (1) | external_body | AVLTreeSeqStPerS::clone doesn't ensure wf |
| GraphSearch explore functions (3) | external_body | Recursive exploration, no decreases clause |
| PQMinStEph graph wf assume (1) | assume | Requires changing graph closure ensures |
| PQMinStEph/StPer external_body (4) | external_body | Recursive exploration / complex loop |
| Chap50 lock-boundary assumes (7) | assume | Requires threading bounds through RwLock invariants |
| Chap50 external_body (10) | external_body | Memoized recursion behind locks |
| Chap52 out_neighbors (1) | external_body | Filter closure spec limitation |

## Verification

```
verification results:: 3771 verified, 0 errors
RTT: 2600 passed
PTT: 147 passed
```

## Prior Round (Round 4) Summary

192 → 182 holes (-10), 15 warnings fixed, 3713 verified, 0 errors.
See git history for details.
