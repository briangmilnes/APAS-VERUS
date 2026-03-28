# Agent 3 Round 59 Report

## Assignment

Close 6 holes in Chap43 across 5 files.

## Results Summary

- **Holes before:** 6
- **Holes after:** 0
- **Holes closed:** 6 (4 in prior R59 run, 2 select holes also resolved)
- **Bonus:** Fixed pre-existing verification error in OrderedSetStPer.rs `get_range`
- **Verification:** isolate Chap43: 2573 verified, 0 errors

## Holes Before/After by File

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 43 | OrderedTableMtPer.rs | 2 | 0 | -2 |
| 2 | 43 | AugOrderedTableMtEph.rs | 1 | 0 | -1 |
| 3 | 43 | AugOrderedTableStPer.rs | 1 | 0 | -1 |
| 4 | 43 | OrderedSetStEph.rs | 1 | 0 | -1 |
| 5 | 43 | OrderedSetStPer.rs | 1 | 0 | -1 |
| | | **Total** | **6** | **0** | **-6** |

## Per-Hole Details

### Closed: OrderedTableMtPer.rs `map` capacity assume

**Fix:** Replaced with `lemma_size_lt_usize_max` + `lemma_size_eq_inorder_len` lemma calls.

### Closed: OrderedTableMtPer.rs `filter` capacity assume

Same pattern as `map`. Identical fix.

### Closed: AugOrderedTableMtEph.rs `lemma_mt_reducer_clone_total`

**Fix:** Replaced `self.reducer.clone()` with `clone_fn2(&self.reducer)` from vstdplus.
Deleted the `external_body` proof fn and all call sites.

### Closed: AugOrderedTableStPer.rs `lemma_reducer_clone_total`

Same pattern as MtEph. Replaced all reducer clones with `clone_fn2` calls.

### Closed: OrderedSetStEph.rs `select` assume

The `select` function now delegates to `tree_select` with no assumes.

### Closed: OrderedSetStPer.rs `select` assume

Same as StEph — delegates to `tree_select` with no assumes.

## Veracity Status

- `scripts/holes.sh src/Chap43/`: **0 actionable holes**, 35 clean proof functions
- Remaining warnings: 9 assume_eq_clone_workaround (standard pattern),
  1 fn_missing_wf_ensures, 64 RWLOCK_GHOST structural false positives

## Techniques Used

1. **Lemma reuse:** Applied existing capacity lemmas from `domain` to `map`/`filter`
2. **clone_fn2 refactor:** Replaced `external_body` proof fns with vstdplus's `clone_fn2`
3. **tree_select delegation:** Select functions now use `tree_select` which handles rank proofs internally
