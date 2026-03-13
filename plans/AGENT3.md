# Agent 3 Report — Rounds 4–6

## Round 6: Chap45 Proof Hardening

### Assignment

Chap45/50/52/53/65 (55 holes, target -15). AFK mode.

### Results

Removed 10 holes from Chap45 (24 → 14). Other chapters (50, 52, 53, 65) had no actionable
holes — blocked by RwLock API gaps, StPer wf gap, generic PartialEq spec gap, or complex
closure/iterator patterns.

| # | File | Hole Type | Technique | Delta |
|---|------|-----------|-----------|-------|
| 1 | BinaryHeapPQ.rs | assume (empty) | Proved multiset: empty seq maps to empty multiset | -1 |
| 2 | BinaryHeapPQ.rs | assume (singleton) | Proved multiset: single-element seq = singleton multiset | -1 |
| 3 | BinaryHeapPQ.rs | assume (delete_min 1-elem) | Proved: 1-element seq = empty.push(elem) | -1 |
| 4 | BinaryHeapPQ.rs | assume (to_sorted_vec) | Proved: sorted property propagates through Vec build loop | -1 |
| 5 | BinaryHeapPQ.rs | assume (insert) | Proved: append → bubble_up preserves multiset via T-level bridge | -1 |
| 6 | BinaryHeapPQ.rs | assume (meld) | Proved: append + heapify preserves multiset via lemma_multiset_commutative | -1 |
| 7 | BinaryHeapPQ.rs | assume (delete_min n-elem) | Proved: rebuild sequence is permutation of subrange(1,n), used to_multiset_remove | -1 |
| 8 | BinaryHeapPQ.rs | assume (swap_elements) | New assume added for multiset chain, then proved via lemma_swap_preserves_multiset | 0 |
| 9 | BalancedTreePQ.rs | external_body (is_sorted) | Replaced f64 comparison with integer while loop | -1 |
| 10 | BalancedTreePQ.rs | external_body (height) | Replaced f64.log2().ceil() with integer loop | -1 |
| 11 | HeapsortExample.rs | external (impl) | Replaced nested fn with is_vec_sorted_exec while loop inside verus! | -1 |
| | **Net** | | | **-10** |

### Key Proof Techniques

**T-level to view-level bridge**: ArraySeqStPerS ensures use `spec_index` (T-level), but
multiset proofs need `@` (view-level). Bridge pattern:
```
assert(x.spec_index(i) == y.seq@[i]);  // T-level
assert(x.spec_index(i)@ == x@[i]);      // view definition
assert(y.seq@[i]@ == y@[i]);            // view definition
```

**Multiset preservation chain**: swap_elements → bubble_up → bubble_down → heapify, each
with `ensures heaped@.to_multiset() =~= seq@.to_multiset()`. Uses `obeys_feq_clone::<T>()`
in invariants and `axiom_cloned_implies_eq_owned` for clone bridges.

**lemma_swap_preserves_multiset**: Custom proof fn using `to_multiset_update` (vstd broadcast)
twice, then `assert_multisets_equal!` with count-level reasoning. Handles all cases
(i==j, i!=j) through `m.insert(b).remove(a).insert(a).remove(b) == m`.

**delete_min permutation proof**: Shows rebuild sequence [self[n-1], self[1], ..., self[n-2]]
has same multiset as self.subrange(1, n) via `lemma_multiset_commutative` on
`Seq::empty().push(last) + rest`, then `to_multiset_remove(self@, 0)` to connect to
`self@.to_multiset().remove(self@[0])`.

### Blocked Holes

| Reason | Count | Chapters |
|--------|-------|----------|
| RwLock API gaps (lock-boundary assumes) | 7 | Chap50 |
| Fork-join/closure proof infrastructure | 9 | Chap50, 53 |
| Generic PartialEq spec gap | 2 | Chap45, 65 |
| StPer wf gap (to_seq lacks ensures) | 3 | Chap53 |
| Complex iterator/closure patterns | 3 | Chap50, 52 |
| AVLTreeSetStPer missing ensures | 11 | Chap45 |

### Per-Chapter Hole Summary (Round 6 scope)

| # | Chap | Before | After | Delta | Status |
|---|------|--------|-------|-------|--------|
| 1 | 45 | 24 | 14 | -10 | 1 BinaryHeapPQ assume, 12 BalancedTreePQ, 1 Example45_2 |
| 2 | 50 | 17 | 17 | 0 | All blocked (RwLock, fork-join) |
| 3 | 52 | 1 | 1 | 0 | Blocked (filter spec limitation) |
| 4 | 53 | 12 | 12 | 0 | Blocked (StPer wf gap, fork-join) |
| 5 | 65 | 1 | 1 | 0 | Blocked (generic PartialEq gap) |
| | **Total** | **55** | **45** | **-10** | |

### Verification

- `scripts/validate.sh`: 3780 verified, 0 errors
- `scripts/rtt.sh`: 2600 tests passed
- No trigger warnings

---

## Round 4 Summary (prior work)

### Round 4a: Chap50 Lock-Boundary Conversions (-11 holes)

Converted 11 external_body functions using arc_deref + accept patterns.

| # | File | Before | After | Delta |
|---|------|--------|-------|-------|
| 1 | MatrixChainMtPer.rs | 3 | 2 | -1 |
| 2 | MatrixChainMtEph.rs | 7 | 2 | -5 |
| 3 | OptBinSearchTreeMtEph.rs | 6 | 2 | -4 |
| 4 | OptBinSearchTreeMtPer.rs | 3 | 2 | -1 |

### Round 4b: Chap53 Graph Search Conversions (-3 holes)

Converted 3 external_body functions using closure requires and while-loop conversion.

| # | File | Function | Delta |
|---|------|----------|-------|
| 1 | GraphSearchMtPer.rs | SelectOne::select | -1 |
| 2 | PQMinStEph.rs | pq_find_min_priority | -1 |
| 3 | PQMinStEph.rs | pq_explore | -1 |

### Next Steps (prioritized)

1. **BinaryHeapPQ extract_all_sorted** (1 assume): Requires heap property invariant
   as loop invariant. Hard — needs spec_is_heap carried through delete_min.
2. **BalancedTreePQ** (12 holes): Blocked by missing AVLTreeSetStPer ensures
   (subseq_copy, values_in_order). Coordinate with Agent 2.
3. **Chap53 StPer wf gap** (3 holes): Need to_seq() ensures in Chap41.
4. **Chap50 parallel_min_reduction** (4 holes): Replace with verified reduce.
