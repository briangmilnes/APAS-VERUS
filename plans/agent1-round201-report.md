# Agent 1 Round 201 Report

## Summary

R201 completed three phases: Phase 1 AIR-bug audit, Phase 2 BSTTreap delete RTT,
Phase 3 iterator PTT backfill for 11 collections. All validation passes clean.

## Phase 1: AIR-Bug Audit (Chap18 ArraySeqStPer IntoIterator)

The AIR bug reported in R199 is resolved. Both `IntoIterator` impls in
`src/Chap18/ArraySeqStPer.rs` are inside `verus!{}`:

- `impl<'a, T: StT> std::iter::IntoIterator for &'a ArraySeqStPer<T>` — inside verus!
- `impl<T: StT> std::iter::IntoIterator for ArraySeqStPer<T>` — inside verus!

No action needed. The bug was fixed upstream before this round.

## Phase 2: BSTTreap Delete RTT

Added `test_delete_requires_left_recursion_after_rotation` to
`tests/Chap39/TestBSTTreapStEph.rs`. This exercises the
`left_pri > right_pri` → `rotate_left` → recurse-left branch at line 1339
of `delete_link` in `src/Chap39/BSTTreapStEph.rs`.

Tree construction: insert 30(p=300), 10(p=200), 50(p=100). Deleting 30:
left_pri=200 > right_pri=100 → rotate_left makes 50 the root with 30 as
50's left child → recurse into 30 (only left child=10) → return 10.
Final: root=50, left=10, right=None. Test verifies size=2, 10 present, 50 present, 30 absent.

RTT result: 4209 passed (includes new test).

## Phase 3: Iterator PTT Backfill

Created 11 new PTT files covering 11 collections that lacked proof coverage.
Fixed a scope bug: `ProveBSTSetTreapMtEph.rs` needed an explicit import of
`view_ord_consistent` from `BSTParaTreapMtEph` since it is not re-exported
by `BSTSetTreapMtEph`.

### New PTT Files

| # | Chap | File | Patterns | Notes |
|---|------|------|----------|-------|
| 1 | 37 | ProveBSTSplayStEph.rs | loop-borrow-into, for-borrow-into | uses BSTSplayStEphLit! |
| 2 | 39 | ProveBSTTreapStEph.rs | loop-borrow-into, for-borrow-into | uses new() (hash macro unsafe) |
| 3 | 39 | ProveBSTSetTreapMtEph.rs | loop-borrow-into, for-borrow-into | requires obeys_cmp_spec + view_ord_consistent; import fix |
| 4 | 39 | ProveBSTParaTreapMtEph.rs | loop-borrow-into, for-borrow-into | requires obeys_cmp_spec + view_ord_consistent |
| 5 | 40 | ProveBSTSizeStEph.rs | loop-borrow-into, for-borrow-into | uses new() (hash macro unsafe) |
| 6 | 40 | ProveBSTKeyValueStEph.rs | loop-borrow-into, for-borrow-into | uses new() (hash macro unsafe) |
| 7 | 40 | ProveBSTReducedStEph.rs | loop-borrow-into, for-borrow-into | uses BSTCountStEph<u64,u64> alias |
| 8 | 41 | ProveArraySetStEph.rs | loop-borrow-iter, loop-borrow-into, for-borrow-iter, for-borrow-into | wrapping iter, item = &T |
| 9 | 41 | ProveAVLTreeSetMtPer.rs | loop-borrow-into, for-borrow-into | uses AVLTreeSetMtPerLit!; no requires |
| 10 | 42 | ProveTableStEph.rs | loop-borrow-iter, loop-borrow-into, for-borrow-iter, for-borrow-into | wrapping iter, item = &Pair<K,V> |
| 11 | 42 | ProveTableStPer.rs | loop-borrow-iter, loop-borrow-into, for-borrow-iter, for-borrow-into | wrapping iter, item = &Pair<K,V> |

Total new PTT tests: 28 (2 patterns × 7 snapshot files + 4 patterns × 3 wrapping files + 2 for AVLTreeSetMtPer).

### Pattern Notes

- **Snapshot iterators** (BST types): yield owned T; 2 patterns (no iter(), no consume).
- **Wrapping borrow iterators** (ArraySet, Table): yield &'a T; 4 patterns (has iter(), no consume since item is reference).
- **Hash macro restriction**: BSTTreapStEph, BSTSizeStEph, BSTKeyValueStEph, BSTReducedStEph macros use DefaultHasher and cannot appear inside verus_code!. Use new() (empty construction) instead.
- **obeys_cmp_spec / view_ord_consistent**: BSTSetTreapMtEph and BSTParaTreapMtEph require both predicates on T for into_iter. Test functions carry explicit requires. view_ord_consistent must be imported from BSTParaTreapMtEph (not re-exported by BSTSetTreapMtEph).

## Validation Results

| Step | Result |
|------|--------|
| validate isolate Chap18 | 1083 verified, 0 errors |
| validate isolate Chap39 | 1305 verified, 0 errors |
| validate (full) | 5728 verified, 0 errors |
| rtt | 4209 passed, 0 failed |
| ptt | 265 passed, 0 failed |

## Holes

No proof holes were added or removed this round. PTT files contain only exec test
functions (no spec/proof code). Hole count unchanged from R200.
