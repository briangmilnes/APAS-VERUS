# Agent 3 — Round 12 Report

## Summary

Investigated all 3 Chap37 targets. All have deeper blockers than estimated — none are quick wins. Did not reach BSTParaStEph.rs.

## Hole Changes

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 37 | AVLTreeSeq.rs | 1 | 1 | 0 |
| 2 | 37 | BSTSplayStEph.rs | 1 | 1 | 0 |
| 3 | 37 | AVLTreeSeqStPer.rs | 1 | 1 | 0 |
| 4 | 38 | BSTParaStEph.rs | 15 | 15 | 0 |
| - | - | **Total** | **18** | **18** | **0** |

Target was 18 → ≤ 11: not achieved.

## Chap37 Blocker Analysis

### AVLTreeSeq.rs iterator next (1 external_body)

T-vs-T::V bridge problem. Iterator view is `(int, Seq<T>)` but `nth` ensures only `elem@ == inorder(root)[pos]` (Seq<T::V>). Need `elem == inorder_values(root)[pos]` (Seq<T>) which requires view-injectivity — not available for generic T.

### BSTSplayStEph.rs trivial_wf (1 trivial_spec_wf)

`spec_bstsplaysteph_wf` returns `true`. Strengthening to `spec_is_bst_link(&self.root)` (already defined in file) requires proving BST preservation through splay rotations. All operations (`splay`, `insert_link`, `find_link`) have `ensures true` — zero proof infrastructure exists. Major proof work, not incremental.

### AVLTreeSeqStPer.rs clone assume (1 assume)

`assume(val@ == a@[mid as int]@)` in `build_balanced_from_slice`. Standard clone bridge: generic T::clone() has no verified view-preserving ensures. Fix requires propagating `obeys_feq_clone::<T>()` via requires chain through `build_balanced_from_slice` → `from_vec` → trait callers. API-cascading change.

## Verification

3986 verified, 0 errors (unchanged).

## Commit

TBD
