<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 1 — Round 57 Report

## Objective

Close 12 capacity `assume` statements in `src/Chap43/OrderedTableStPer.rs`.

Each was of the form `assume(result_vec@.len() < usize::MAX)` (or `left_vec`/`right_vec` variants),
needed before calling `AVLTreeSeqStPerS::from_vec` which requires `values@.len() < usize::MAX`.

## Technique

For each function:

1. Added loop invariant `result_vec@.len() <= i as nat` (or equivalent) to track the
   accumulated vector length relative to the loop index.
2. After the loop, the invariant gives `result_vec@.len() <= len` where `len` comes from
   the wf-bounded source sequence. The broadcast group `group_avltreeseqstper_len_bound`
   provides `len < usize::MAX`.
3. Replaced `assume(result_vec@.len() < usize::MAX)` with `assert(...)`.

Special cases:

- **`union`**: result grows from two sources; added `self@.dom().len() + other@.dom().len() < usize::MAX` to requires, used `result_vec@.len() <= self_len as nat + i as nat`.
- **`tabulate`**: source is `ArraySetStEph`, not AVL-backed; added `keys.elements@.len() < usize::MAX` to requires; used `unique_seq_to_set()` to connect `key_seq.spec_len()` to `keys@.len()`.
- **`split_rank_key`**: two output vecs; tracked `left_vec@.len() + right_vec@.len() == j as nat`.

## Cascading Precondition Fixes

New preconditions on `tabulate` and `union` cascaded to callers in other Chap43 files:

| # | Chap | File | Change |
|---|:----:|---|---|
| 1 | 43 | `AugOrderedTableStPer.rs` | Added `keys.elements@.len() < usize::MAX` to `tabulate` requires; `self@.dom().len() + other@.dom().len() < usize::MAX` to `union` requires |
| 2 | 43 | `OrderedTableMtPer.rs` | Added inner-view connection proof (`assume(self_inner@.dom().len() == self@.dom().len())`) to `join_key` impl; trait already had the combined-size requires |

The two `assume` statements added to `OrderedTableMtPer::join_key` follow the exact same
pattern as the existing `assume(count == self@.dom().len())` in `size()` — bridging
the locked inner view to the ghost outer view, which the current `RwLockPredicate` design
does not formally connect.

## Holes Before / After

| # | Chap | File | Before | After | Delta |
|---|:----:|---|:---:|:---:|:---:|
| 1 | 43 | `OrderedTableStPer.rs` | 14 | 2 | −12 |
| 2 | 43 | `OrderedTableMtPer.rs` | 0 | 2 | +2 |
| 3 | 43 | `AugOrderedTableStPer.rs` | 2 | 2 | 0 |

Net Chap43 hole change: **−10** (12 removed, 2 added for view-connection in MtPer).

The 2 new assumes in `OrderedTableMtPer` are view-connection holes, the same class as the
existing `assume(count == self@.dom().len())` in `size()`.

## Verification Result

```
verification results:: 4485 verified, 0 errors
warning: 2 warnings emitted (pre-existing, Chap47)
Elapsed: 94s
```
