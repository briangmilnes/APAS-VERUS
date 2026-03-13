# Agent 3 Report — Rounds 4–6

## Round 6: Chap45/50/53 Proof Hardening

### Assignment

Chap45/50/52/53/65 (55 holes, target -15). AFK mode.

### Results

Removed 18 holes across 3 chapters (55 → 37). Exceeded target by 3.

#### Round 6a: Chap45 Multiset & Heap Proofs (-10 holes)

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

#### Round 6b: Chap53 PQMinStEph (-2 holes)

| # | File | Hole Type | Technique | Delta |
|---|------|-----------|-----------|-------|
| 1 | PQMinStEph.rs | external_body (pq_min_multi) | Added closure requires, passed sources as visited_init | -1 |
| 2 | PQMinStEph.rs | assume (neighbors wf) | Added graph.ensures wf requires to call chain | -1 |
| | **Net** | | | **-2** |

#### Round 6c: Chap50 RwLock Ghost Tracking (-6 holes)

Introduced `ghost expected_len` field in RwLock predicates (`MatrixChainMtEphDimInv`,
`OptBSTMtEphKeysInv`). Added wf spec connecting ghost view fields to predicate's expected_len.
This bridges the gap between locked data and ghost view, enabling proof of index bounds and
length assertions without assumes.

| # | File | Hole Type | Technique | Delta |
|---|------|-----------|-----------|-------|
| 1 | MatrixChainMtEph.rs | assume (multiply_cost bounds) | DimInv.expected_len + wf requires | -1 |
| 2 | MatrixChainMtEph.rs | assume (set_dimension) | DimInv.expected_len + trait requires chain | -1 |
| 3 | MatrixChainMtEph.rs | assume (update_dimension) | DimInv.expected_len + trait requires chain | -1 |
| 4 | MatrixChainMtEph.rs | assume (num_matrices) | DimInv.expected_len + wf requires | -1 |
| 5 | OptBinSearchTreeMtEph.rs | assume (set_key_prob) | KeysInv.expected_len + wf requires | -1 |
| 6 | OptBinSearchTreeMtEph.rs | assume (update_prob) | KeysInv.expected_len + wf requires | -1 |
| | **Net** | | | **-6** |

### Key Proof Techniques

**RwLock ghost expected_len pattern**: RwLock predicates carry `ghost expected_len: nat` with
`inv(self, v) = v@.len() == self.expected_len`. Constructors pass `expected_len` matching the
ghost view field. A wf spec asserts `arc.pred().expected_len == ghost_field@.len()`. This
bridges the gap: `acquire_read/write` ensures give `data.len() == expected_len`, and wf gives
`expected_len == ghost_field@.len()`, so `data.len() == ghost_field@.len()`. The predicate is
preserved through Vec::set (length-preserving) so release_write succeeds without assumes.

**Closure requires propagation**: When a function calls a closure `graph(&v)`, the closure's
requires must be lifted into the function's own requires. Similarly, `graph.ensures((v,), r)
==> r.spec_wf()` propagates wf knowledge about return values.

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

### Blocked Holes

| Reason | Count | Chapters |
|--------|-------|----------|
| Fork-join/closure proof infrastructure | 8 | Chap50, 53 |
| f64/Probability add_req uninterpreted | 4 | Chap50 |
| StPer wf gap (to_seq lacks ensures) | 3 | Chap53 |
| AVLTreeSetStPer missing ensures | 12 | Chap45 |
| Generic PartialEq spec gap | 2 | Chap45, 65 |
| Overflow assume (content-dependent) | 1 | Chap50 |
| Closure predicate capture limitation | 1 | Chap52 |
| BinaryHeapPQ spec_leq_view gap | 1 | Chap45 |
| Graph search postcondition issue | 5 | Chap53 |

### Per-Chapter Hole Summary (Round 6 scope)

| # | Chap | Before | After | Delta | Status |
|---|------|--------|-------|-------|--------|
| 1 | 45 | 24 | 14 | -10 | 1 BinaryHeapPQ, 12 BalancedTreePQ, 1 Example45_2 |
| 2 | 50 | 17 | 11 | -6 | 1 assume (overflow), 10 external_body |
| 3 | 52 | 1 | 1 | 0 | Blocked (closure predicate) |
| 4 | 53 | 12 | 10 | -2 | Blocked (StPer wf, graph postcondition) |
| 5 | 65 | 1 | 1 | 0 | Blocked (generic PartialEq gap) |
| | **Total** | **55** | **37** | **-18** | |

### Verification

- `scripts/validate.sh`: 3782 verified, 0 errors
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
