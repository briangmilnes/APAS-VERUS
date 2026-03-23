# Agent 3 Round 59 Report

## Assignment

Close 6 holes in Chap43 across 5 files.

## Results Summary

- **Holes before:** 6
- **Holes after:** 2
- **Holes closed:** 4
- **Bonus:** Fixed pre-existing verification error in OrderedSetStPer.rs `get_range` (invariant not satisfied before loop)
- **Verification:** 4485 verified, 0 errors
- **RTT:** 2610 passed, 0 skipped
- **PTT:** 147 passed, 0 skipped

## Holes Before/After by File

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 43 | OrderedTableMtPer.rs | 2 | 0 | -2 |
| 2 | 43 | AugOrderedTableMtEph.rs | 1 | 0 | -1 |
| 3 | 43 | AugOrderedTableStPer.rs | 1 | 0 | -1 |
| 4 | 43 | OrderedSetStEph.rs | 1 | 1 | 0 |
| 5 | 43 | OrderedSetStPer.rs | 1 | 1 | 0 |
| | | **Total** | **6** | **2** | **-4** |

## Per-Hole Details

### Closed: OrderedTableMtPer.rs `map` capacity assume (line 361)

**Was:** `assume(len + 1 < usize::MAX as nat)` after `inner.collect()`.

**Fix:** Replaced with lemma calls already used in the same file's `domain` function:
```rust
lemma_size_lt_usize_max::<Pair<K, V>>(&entries.root);
lemma_size_eq_inorder_len::<Pair<K, V>>(&entries.root);
assert(len < usize::MAX);
```
Changed loop invariant from `len + 1 < usize::MAX as nat` to `len < usize::MAX`. The insert precondition `result@.dom().len() + 1 < usize::MAX` follows from `result@.dom().len() <= i < len < usize::MAX`.

### Closed: OrderedTableMtPer.rs `filter` capacity assume (line 397)

Same pattern as `map`. Identical fix.

### Closed: AugOrderedTableMtEph.rs `lemma_mt_reducer_clone_total` (line 121)

**Was:** `external_body` proof fn asserting that cloning a reducer preserves its `requires`.

**Fix:** Replaced all `self.reducer.clone()` / `range_table.reducer.clone()` calls with `clone_fn2(&self.reducer)` / `clone_fn2(&range_table.reducer)` from `crate::vstdplus::clone_plus`. The `clone_fn2` exec function provides `ensures forall|x, y| f.requires((x,y)) == res.requires((x,y))`, eliminating the need for the proof lemma entirely. Deleted the proof fn definition and all lemma call sites (2 sites).

### Closed: AugOrderedTableStPer.rs `lemma_reducer_clone_total` (line 118)

Same pattern as MtEph. Replaced all `self.reducer.clone()` and `left.reducer.clone()` with `clone_fn2` calls. Deleted the proof fn and all 15 call sites.

### Left: OrderedSetStEph.rs `select` assume (line 1146)

**Assume:** `self@.filter(|x| exists|t| le(t, result) && t@ == x && t@ != result@).len() == i`

**What was tried:**
- Verified that `spec_seq_sorted` exists in AVLTreeSetStEph (Chap41 line 80)
- Verified that `spec_elements_sorted` exists as a separate property
- Checked wf specs: sortedness is NOT part of `spec_avltreesetsteph_wf` or `spec_orderedsetsteph_wf`
- The BST property in the AVL tree wf implies sorted inorder traversal, but no `lemma_wf_implies_sorted` bridge exists

**What's needed:** A recursive proof lemma `lemma_bst_wf_implies_inorder_sorted` that derives `spec_seq_sorted(spec_inorder_values(root))` from the BST ordering invariant in `spec_avltreeseqsteph_wf`. This could then be used to prove the filter cardinality equals the positional index. Alternatively, `spec_elements_sorted()` could be added to the wf predicate, but that cascades to all AVLTreeSet operations.

### Left: OrderedSetStPer.rs `select` assume (line 1060)

Same blocker as OrderedSetStEph. Identical fix path needed.

## Bonus Fix: OrderedSetStPer.rs `get_range` pre-existing error

The `get_range` function had a pre-existing verification error: "invariant not satisfied before loop" on `size as nat == self@.len()`. Fixed by adding `proof { self.base_set.elements@.unique_seq_to_set(); }` before the loop, which bridges `elements@.len()` (sequence length) to `self@.len()` (set cardinality) via the no-duplicates property from wf.

## Techniques Used

1. **Lemma reuse:** Applied existing `lemma_size_lt_usize_max` + `lemma_size_eq_inorder_len` from the same file's `domain` function to `map`/`filter`
2. **clone_fn2 refactor:** Replaced `external_body` proof fns with vstdplus's `clone_fn2` exec function, which provides spec-level ensures for closure cloning. This moves the `external_body` out of Chap43 into the shared vstdplus infrastructure
3. **unique_seq_to_set bridge:** Used vstd's `unique_seq_to_set` lemma to connect sequence length to set cardinality for the pre-existing `get_range` error
