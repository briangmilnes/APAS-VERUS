# Agent 3 R100 — AVLTreeSeqMtPer View: Seq<T> → Seq<T::V> + Cascade

## Objective

Fix AVLTreeSeqMtPer iterator views to use `Seq<T::V>` consistently, matching StPer.

## Finding

The main `View for AVLTreeSeqMtPerS` was **already** `Seq<T::V>` (likely fixed in a prior round).
The inconsistency was in the iterator infrastructure:

| # | Chap | File | Item | Before | After |
|---|------|------|------|--------|-------|
| 1 | 37 | AVLTreeSeqMtPer.rs | `GhostIterator.elements` | `Seq<T>` | `Seq<T::V>` |
| 2 | 37 | AVLTreeSeqMtPer.rs | `View for BorrowIter` | `(int, Seq<T>)` via `spec_inorder_values` | `(int, Seq<T::V>)` via `spec_inorder` |
| 3 | 37 | AVLTreeSeqMtPer.rs | `View for GhostIterator` | `Seq<T>` | `Seq<T::V>` |
| 4 | 37 | AVLTreeSeqMtPer.rs | `ForLoopGhostIterator::Item` | `T` | `T::V` |
| 5 | 37 | AVLTreeSeqMtPer.rs | `ghost_peek_next` return | `Option<T>` | `Option<T::V>` |
| 6 | 37 | AVLTreeSeqMtPer.rs | `next()` ensures | `element == old_seq[i]` | `element@ == old_seq[i]` |
| 7 | 37 | AVLTreeSeqMtPer.rs | trait `iter` ensures | `it@.1.map_values(\|t\| t@) =~= ...` | `it@.1 =~= ...` |
| 8 | 37 | AVLTreeSeqMtPer.rs | `into_iter` ensures | `it@.1.map_values(\|t\| t@) =~= ...` | `it@.1 =~= ...` |

## Dead code removed

- `spec_inorder_values` — no longer referenced after BorrowIter uses `spec_inorder`.
- `lemma_inorder_values_maps_to_inorder` — bridging lemma no longer needed.

## Caller cascade

Checked all callers:
- `src/Chap41/AVLTreeSetMtPer.rs` — no use of `spec_inorder_values` or iterator views. No changes needed.
- `src/Chap52/EdgeSetGraphMtPer.rs` — imports trait only. No changes needed.
- `src/Chap53/GraphSearchMtPer.rs` — imports trait only. No changes needed.

## Verification

| Step | Result |
|------|--------|
| Isolate Chap41 | 2031 verified, 0 errors |
| Full validate | 5389 verified, 0 errors |
| RTT | 3083 passed |
| PTT | 157 passed |

## Steps used: 3 of 20

(Read files, make edits, validate 3 times)
