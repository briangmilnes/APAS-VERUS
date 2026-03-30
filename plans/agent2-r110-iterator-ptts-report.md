# Agent 2 R110 — Iterator PTTs Report

## Summary

Wrote iterator PTT patterns for 21 of 22 assigned collections. One module
(AVLTreeSetMtPer) has no iterator infrastructure and was skipped.

Cargo check passes: `cargo check --tests` clean.

## Results Table

| # | Chap | Module | PTT File | Action | Patterns | Notes |
|---|------|--------|----------|--------|----------|-------|
| 1 | 05 | SetStEph | ProveSetStEph.rs | new | 4 borrow | No IntoIterator for Self |
| 2 | 37 | AVLTreeSeqMtPer | ProveAVLTreeSeqMtPer.rs | new | 4 borrow | Consume iter has no View impl |
| 3 | 37 | AVLTreeSeqStEph | ProveAVLTreeSeqStEph.rs | new | 4 borrow | No IntoIterator for Self |
| 4 | 37 | AVLTreeSeqStPer | ProveAVLTreeSeqStPer.rs | new | 4 borrow | No IntoIterator for Self |
| 5 | 37 | BSTSetAVLMtEph | ProveBSTSetAVLMtEph.rs | new | 6 (4+2 consume) | |
| 6 | 37 | BSTSetBBAlphaMtEph | ProveBSTSetBBAlphaMtEph.rs | new | 6 (4+2 consume) | |
| 7 | 37 | BSTSetPlainMtEph | ProveBSTSetPlainMtEph.rs | new | 6 (4+2 consume) | |
| 8 | 37 | BSTSetRBMtEph | ProveBSTSetRBMtEph.rs | new | 6 (4+2 consume) | |
| 9 | 37 | BSTSetSplayMtEph | ProveBSTSetSplayMtEph.rs | new | 6 (4+2 consume) | |
| 10 | 43 | AugOrderedTableMtEph | ProveAugOrderedTableMtEph.rs | new | 4 borrow | Reuses OrderedTableMtEphIter |
| 11 | 43 | AugOrderedTableStEph | ProveAugOrderedTableStEph.rs | new | 4 borrow | Reuses OrderedTableStEphIter |
| 12 | 43 | AugOrderedTableStPer | ProveAugOrderedTableStPer.rs | new | 4 borrow | Reuses OrderedTableStPerIter |
| 13 | 43 | OrderedSetStPer | ProveOrderedSetStPer.rs | new | 2 (iter only) | No IntoIterator at all |
| 14 | 43 | OrderedTableMtEph | ProveOrderedTableMtEph.rs | new | 4 borrow | |
| 15 | 41 | AVLTreeSetMtPer | — | skipped | 0 | No iter() or iterator types in module |
| 16 | 18 | ArraySeq | ProveArraySeq.rs | updated | +4 (borrow-into, consume) | Had 2 + HOF tests |
| 17 | 18 | ArraySeqMtEph | ProveArraySeqMtEph.rs | updated | +4 (borrow-into, consume) | Had 2 |
| 18 | 18 | ArraySeqMtPer | ProveArraySeqMtPer.rs | updated | +4 (borrow-into, consume) | Had 2 |
| 19 | 18 | ArraySeqStPer | ProveArraySeqStPer.rs | updated | +4 (borrow-into, consume) | Had 2 |
| 20 | 18 | LinkedListStEph | ProveLinkedListStEph.rs | updated | +4 (borrow-into, consume) | Had 2 |
| 21 | 18 | LinkedListStPer | ProveLinkedListStPer.rs | updated | +4 (borrow-into, consume) | Had 2 |
| 22 | 41 | AVLTreeSetMtEph | ProveAVLTreeSetMtEph.rs | updated | +1 (loop-borrow-into) | Had 3, now 4 |

## Pattern Counts

- New PTT files created: 14
- Existing PTT files updated: 7
- Total new test patterns written: 89
  - 14 new files x avg ~4.6 patterns = 65
  - 7 updated files x avg ~3.4 patterns = 24

## Files Changed

### New files (14)
- `rust_verify_test/tests/Chap05/ProveSetStEph.rs`
- `rust_verify_test/tests/Chap37/ProveAVLTreeSeqStEph.rs`
- `rust_verify_test/tests/Chap37/ProveAVLTreeSeqStPer.rs`
- `rust_verify_test/tests/Chap37/ProveAVLTreeSeqMtPer.rs`
- `rust_verify_test/tests/Chap37/ProveBSTSetAVLMtEph.rs`
- `rust_verify_test/tests/Chap37/ProveBSTSetBBAlphaMtEph.rs`
- `rust_verify_test/tests/Chap37/ProveBSTSetPlainMtEph.rs`
- `rust_verify_test/tests/Chap37/ProveBSTSetRBMtEph.rs`
- `rust_verify_test/tests/Chap37/ProveBSTSetSplayMtEph.rs`
- `rust_verify_test/tests/Chap43/ProveOrderedSetStPer.rs`
- `rust_verify_test/tests/Chap43/ProveOrderedTableMtEph.rs`
- `rust_verify_test/tests/Chap43/ProveAugOrderedTableStEph.rs`
- `rust_verify_test/tests/Chap43/ProveAugOrderedTableStPer.rs`
- `rust_verify_test/tests/Chap43/ProveAugOrderedTableMtEph.rs`

### Updated files (8)
- `rust_verify_test/tests/Chap18/ProveArraySeq.rs` — +4 patterns
- `rust_verify_test/tests/Chap18/ProveArraySeqMtEph.rs` — +4 patterns
- `rust_verify_test/tests/Chap18/ProveArraySeqMtPer.rs` — +4 patterns
- `rust_verify_test/tests/Chap18/ProveArraySeqStPer.rs` — +4 patterns
- `rust_verify_test/tests/Chap18/ProveLinkedListStEph.rs` — +4 patterns
- `rust_verify_test/tests/Chap18/ProveLinkedListStPer.rs` — +4 patterns
- `rust_verify_test/tests/Chap41/ProveAVLTreeSetMtEph.rs` — +1 pattern
- `rust_verify_test/Cargo.toml` — +18 [[test]] entries

## Design Decisions

1. **AVLTreeSetMtPer (Chap41) skipped**: Module has no `iter()`, no iterator structs,
   no `IntoIterator` impl. Nothing to test.

2. **AVLTreeSeqMtPer consume skipped**: The consuming `IntoIterator for Self` returns
   `AVLTreeSeqMtPerIter<T>` which has no `View` impl and `ensures true` — insufficient
   for proving loop invariants.

3. **OrderedSetStPer only 2 patterns**: Module has `iter()` but no `IntoIterator` impl
   (not even for `&'a`), so only loop-borrow-iter and for-borrow-iter are possible.

4. **AugOrderedTable iter types**: All three AugOrderedTable variants reuse the base
   OrderedTable's iter types. PTTs import both the Aug module and the base module's
   iter types explicitly.

5. **BSTSet*MtEph owned values**: All 5 BSTSet iterators return owned `T` (not `&T`),
   so loop patterns use `items.push(x)` not `items.push(*x)`.
