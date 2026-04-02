# R140 Agent 1 — Fix OrderedTable BST helper proof gaps + StPer O(lg n)

## Summary

Fixed all 5 verification errors in `bst_split_by_key`, `bst_rank_by_key`, and
`bst_select_by_rank`. Also applied O(lg n) fixes to OrderedTableStPer.rs for
first/last/next/previous_key_iter.

Chap43 isolate: **2674 verified, 0 errors** (was 2648/0 before R138, was 2670/0 after R138 StEph fixes).

3627 RTTs pass.

## StEph Proof Fixes (3 functions, 5 errors → 0)

| # | Chap | File | Function | Fix |
|---|------|------|----------|-----|
| 1 | 43 | OrderedTableStEph.rs | `bst_split_by_key` | Found-value: prove contains_key from recursive result upward (not top-down). Size bound: compute |new_right| = |lr| + |right| + 1 via disjoint-union arithmetic. Greater arm: use `reveal(obeys_partial_cmp_spec_properties)` instead of wrong `lemma_cmp_antisymmetry` direction. |
| 2 | 43 | OrderedTableStEph.rs | `bst_rank_by_key` | Replace `spec_rank_pred` references with inline closures to match trait ensures exactly. Verus doesn't unify two closures with extensionally equal bodies. |
| 3 | 43 | OrderedTableStEph.rs | `bst_select_by_rank` | Same closure-matching fix. Also fix transitive argument order: `K::transitive(sel_key, root_pair.0, t)` not `(t, root_pair.0, sel_key)` — we have le(sel_key, root) and le(root, t), not the reverse. |
| 4 | 43 | OrderedTableStEph.rs | `split_rank_key_iter` | Add `lemma_pair_set_to_map_len` before `sorted.nth(i)` to prove i < length. Construct right half via `ParamBST::new()` instead of `Self::empty()` to get wf proof for empty table. |

## StPer O(lg n) Fixes (4 functions, all new)

Added `bst_next_by_key` and `bst_prev_by_key` helper functions to StPer (duplicated from StEph per standalone rule). Also added `lemma_view_gen_subset` and `lemma_cmp_antisymmetry` supporting lemmas.

| # | Chap | File | Function | Old | New |
|---|------|------|----------|-----|-----|
| 1 | 43 | OrderedTableStPer.rs | `first_key_iter` | O(n) in_order scan | O(lg n) BST min_key |
| 2 | 43 | OrderedTableStPer.rs | `last_key_iter` | O(n) in_order scan | O(lg n) BST max_key |
| 3 | 43 | OrderedTableStPer.rs | `previous_key_iter` | O(n) in_order scan | O(lg n) bst_prev_by_key |
| 4 | 43 | OrderedTableStPer.rs | `next_key_iter` | O(n) in_order scan | O(lg n) bst_next_by_key |

## Validation

- Chap43 isolate: 2674 verified, 0 errors
- RTT: 3627 passed, 0 skipped

## Key Proof Techniques

1. **Closure literal matching**: Verus filter closures must be syntactically identical between helper ensures and trait ensures. Using `spec_rank_pred` in helper ensures but inline `exists|t|...` in trait ensures causes a mismatch. Fix: inline the predicate everywhere.

2. **Found-value lifting**: When recursive split returns `found = Some(v)`, prove `tree@.contains_key(k@)` by lifting from subtree: `left@.contains_key(k@)` → `left@.contains((k@, lv))` → `tree@.contains((k@, lv))` → `tree@.contains_key(k@)`.

3. **Set insert length**: `|S.insert(a)| = |S| + 1` when `!S.contains(a)` proved via `S.insert(a) =~= S.union({a})` + `lemma_set_disjoint_lens`.

4. **cmp_spec symmetry**: `k.cmp_spec(root) == Greater` does NOT give `root.cmp_spec(k) == Less` directly. Need `reveal(obeys_partial_cmp_spec_properties)` for the SMT solver to derive it.

## Remaining StPer Work

StPer still has O(n) implementations for: split_key_iter, get_key_range_iter, rank_key_iter, select_key, split_rank_key_iter. These need the remaining 3 helpers (bst_split_by_key, bst_rank_by_key, bst_select_by_rank) duplicated into StPer.
