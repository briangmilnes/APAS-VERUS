# Agent 2 Round 39 Report

## Task

Restructure `src/Chap43/OrderedTableStPer.rs` from `TableStPer<K, V>` (flat unsorted array)
backing to `AVLTreeSetStPer<Pair<K, V>>` (balanced BST) backing.

## Results

| # | Metric | Before | After |
|---|--------|--------|-------|
| 1 | Verified | 4337 | 4301 |
| 2 | Errors | 0 | 0 |
| 3 | Total holes | 175 | 200 |
| 4 | RTT | 2613 pass | 2613 pass |

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 43 | OrderedTableStPer.rs | Full restructuring: backing changed to AVLTreeSetStPer |
| 2 | 43 | AugOrderedTableStPer.rs | Added `V: Ord` bound, fixed field refs, added iter requires |

## OrderedTableStPer.rs Hole Count

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 43 | OrderedTableStPer.rs | 1 | 26 | +25 |
| 2 | 43 | AugOrderedTableStPer.rs | 1 | 1 | 0 |

The 25 new holes are all `external_body` on trait methods. This is expected: the old file
had verified implementations using `ArraySeqStPerS::spec_index` and `lemma_view_index`,
which `AVLTreeSeqStPerS` does not provide. All methods now have correct runtime code but
need proof bodies adapted to the new backing store.

## Structural Changes

1. **Struct field**: `base_table: TableStPer<K, V>` -> `base_set: AVLTreeSetStPer<Pair<K, V>>`
2. **View**: `spec_entries_to_map(self.base_set.elements@)` (was `self.base_table@`)
3. **wf**: `self.base_set.spec_avltreesetstper_wf() && spec_keys_no_dups(self.base_set.elements@)`
4. **V bound**: Added `V: Ord` (required by `Pair<K,V>` in `AVLTreeSetStPer`)
5. **collect()**: Simplified to `self.base_set.to_seq()` (no sort_by needed)
6. **Iterator**: New struct with `seq/pos/len` fields (was wrapping `ArraySeqStPerIter`)
7. **Helper lemma**: `lemma_keys_no_dups_implies_no_duplicates` bridges key-uniqueness to pair-uniqueness
8. **empty()**: Verified (not external_body)

## Propagation to AugOrderedTableStPer.rs

- Added `V: Ord` bound to all generic parameters
- Fixed `base_table.base_table.entries` -> `base_table.base_set.elements`
- Fixed `.seq@` -> `@` for AVLTreeSeqStPerS View
- Added `requires self.spec_augorderedtablestper_wf()` to `iter()` and `into_iter()`

## Key Technical Decision

`AVLTreeSeqStPerS` lacks `spec_index(i)` and `lemma_view_index(i)` which `ArraySeqStPerS`
provides. These are needed for proofs that reason about individual elements by index.
Without them, all methods that iterate over elements and reason about individual entries
cannot be verified. The correct fix is to add `spec_index`/`lemma_view_index` to
`AVLTreeSeqStPerS` (Chap37), which would unblock proving most of these 25 methods.

## What Blocks Proving the 25 Holes

All 25 external_body methods need element-level reasoning:
- `spec_index(i: int) -> T::V` on AVLTreeSeqStPerS
- `lemma_view_index(i: usize)` ensuring `nth(i)@ == self@[i as int]`

Once Chap37 provides these, the proof patterns from the old TableStPer backing can be
adapted to the new AVLTreeSetStPer backing with minimal changes.

## Techniques Used

- Replaced backing store throughout (struct, view, wf, all methods)
- Used `external_body` with correct runtime implementations for all methods pending proof
- Leveraged AVLTreeSetStPer API (empty, singleton, find, insert, delete, union, etc.)
- Clone/PartialEq use assume pattern (standard project practice)
- Iterator follows OrderedSetStPer pattern (struct with seq/pos/len)
