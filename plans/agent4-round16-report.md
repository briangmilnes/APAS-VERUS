# Agent 4 — Round 16 Report

## Summary

- **Holes before**: 136
- **Holes after**: 135
- **Delta**: -1
- **Verified**: 4100, 0 errors, 0 warnings
- **RTT**: 2600 pass
- **Clean chapters**: 38 (unchanged)
- **Holed chapters**: 8 (unchanged)
- **Commit**: (pending)

## Holes Fixed

| # | Chap | File | Before | After | Delta | Technique |
|---|------|------|--------|-------|-------|-----------|
| 1 | 41 | ArraySetEnumMtEph.rs | 1 | 0 | -1 | Closure requires + bit-level subset proof |

### ArraySetEnumMtEph.rs filter (Chap41): 1 → 0

Removed `#[verifier::external_body]` from `filter`. Added closure requires
`forall|i: usize| i < self.spec_universe_size() ==> f.requires((i,))` to trait spec.
Rewrote body as while loop with bit-level invariant: any set bit in the result
implies the corresponding bit was set in the source (`u64_view(new_bits@[k])[b] ==>
u64_view(self.bits@[k])[b]`). Used `set_bit64_proof` to maintain invariant across
word modifications, `zero_bit_false` for initialization. Proved `subset_of` and
`finite` from the invariant. No callers needed updating (filter has no call sites
in verified code).

## Assessed but Blocked

### Chap37 (Priority 1) — 4 holes, 0 fixable

| # | File | Hole | Blocker |
|---|------|------|---------|
| 1 | AVLTreeSeq.rs | external_body on Iterator::next() | Verus: trait impl cannot declare requires |
| 2 | AVLTreeSeqMtPer.rs | external_body on build_balanced | ParaPair! + nested fn inside verus! |
| 3 | AVLTreeSeqMtPer.rs | external_body on subseq_copy | spawn/wait parallel code |
| 4 | BSTSplayStEph.rs | trivial_wf (body is `{ true }`) | All helpers have `ensures true`; proving BST preservation requires 6+ splay rotation cases |

**Cannot close Chap37.** The iterator hole is a fundamental Verus limitation
(`std::iter::Iterator::next()` implementations cannot add `requires` clauses).
The MtPer holes require nested fn/parallel code. The splay wf requires propagating
BST invariant through zig/zig-zig/zig-zag rotation cases across 6+ helper functions.

### Chap47 (Priority 2) — 2 holes, 0 fixable

| # | File | Hole | Blocker |
|---|------|------|---------|
| 1 | ParaHashTableStEph.rs | external_body on call_hash_fn | Opaque Fn closure (Verus can't reason about unspecified closures) |
| 2 | ParaHashTableStEph.rs | external_body on compute_second_hash | Uses std::hash::Hash (no Verus spec) |

**Cannot close Chap47.** Both holes bridge opaque runtime features (Fn closures,
std::hash) that Verus cannot model.

### Chap45 (Priority 3) — 2 effective holes, 0 fixable

| # | File | Hole | Blocker |
|---|------|------|---------|
| 1 | BalancedTreePQ.rs | external impl block | filter/map with opaque Fn closures |
| 2 | BinaryHeapPQ.rs | assume(spec_sorted) | Uninterpreted spec_leq_view disconnected from TotalOrder::le; needs heap property preservation proof |

(Example45_2.rs external skipped per instructions.)

**Cannot close Chap45.** BalancedTreePQ needs verified closure support for filter/map.
BinaryHeapPQ needs connecting uninterpreted spec fn to TotalOrder and proving
heap property preservation through delete_min — massive proof effort.

### Chap41 (Priority 4) — 24 holes, 10 remaining targets

| # | File | Holes | Blocker |
|---|------|-------|---------|
| 1 | AVLTreeSetStEph.rs | 1 | Insert vec-len: wf gives n < MAX, need n+1 < MAX (off-by-one cascade) |
| 2 | AVLTreeSetMtEph.rs | 9 | 2 assume (view bridge), 4 external_body (parallel/nested fns), 1 external_body (view bridge to_seq), 2 unsafe Send/Sync |
| 3 | ArraySetEnumMtEph.rs | 0 | FIXED this round |

**AVLTreeSetStEph insert vec-len**: `from_vec` requires `values@.len() < usize::MAX`.
After insert, `new_vec.len() = n + 1`. Wf gives `n < usize::MAX` but `n + 1` could
equal `usize::MAX`. Fix requires either strengthening AVLTreeSeqStEph link-level wf
(invasive cascade through all users) or adding capacity bound to insert's requires
(API change cascading to all callers).

**AVLTreeSetMtEph view bridge**: `ghost_set_view` field not connected to locked
`AVLTreeSetStEph` via RwLock predicate. Read-only operations (`&self`) can't prove
ghost matches locked inner. Needs architectural change to locking approach.

## Techniques Used

1. **Closure requires propagation**: Added `forall|i| i < universe_size ==>
   f.requires((i,))` to trait filter spec, following the closure standard pattern.
2. **Bit-level subset invariant**: `forall|k, b| (... && u64_view(new_bits@[k])[b])
   ==> u64_view(self.bits@[k])[b]` — maintained through `set_bit64_proof` and
   `zero_bit_false`, proved `subset_of` from the invariant.
3. **Word-level proof decomposition**: For each `Vec::set`, split proof into same-word
   (preserved by set_bit64_proof) vs different-word (unchanged by Vec::set) cases.
