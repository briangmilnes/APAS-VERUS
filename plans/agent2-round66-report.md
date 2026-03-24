# Agent 2 — Round 66 Report

## Task

Rewrite `src/Chap43/OrderedTableStEph.rs` to change backing store from
`AVLTreeSeqStEphS<Pair<K,V>>` (Chap37) to `ParamBST<Pair<K,V>>` (Chap38).
Update all callers: AugOrderedTableStEph, OrderedTableMtEph, AugOrderedTableMtEph.

## Results

| # | Metric | Value |
|---|--------|-------|
| 1 | Verification | 4488 verified, 0 errors |
| 2 | Trigger warnings | 0 (fixed 43) |
| 3 | RTT | 2609 passed, 1 SIGKILL (known regression) |
| 4 | PTT | 147 passed, 0 failed |
| 5 | Chap43 holes | 0 (was 0 before — external_body removed, real proofs added) |
| 6 | Chap43 modules clean | 10/10 |
| 7 | Chap43 exec fns with spec | 639 (85%) |
| 8 | Chap43 proof fns clean | 11/11 |

## Files Changed

| # | Chap | File | Lines changed | What |
|---|------|------|--------------|------|
| 1 | 43 | OrderedTableStEph.rs | ~4000 | Full rewrite: AVL → ParamBST backing store |
| 2 | 43 | AugOrderedTableStEph.rs | 43 | Update imports, type bounds (+Ord on V) |
| 3 | 43 | OrderedTableMtEph.rs | 89 | Update imports, cfg-gate spec import |
| 4 | 43 | AugOrderedTableMtEph.rs | 26 | Update imports, type bounds (+Ord on V) |

## Techniques

- **Iterative rewrite**: Replaced all 30+ functions from AVL-based tree traversal to
  ParamBST `in_order()` → sorted `Vec<Pair<K,V>>` iteration with fresh `ParamBST::insert`.
- **Trigger annotations**: Fixed all 43 "automatically chose triggers" warnings with
  explicit `#[trigger]` on quantifiers. Common pattern: `(#[trigger] sorted@[j]).0`.
- **split_key_iter three-way comparison**: Fixed logic bug where all non-matching keys
  went to left tree. Now uses `pair.0.cmp(k)` with Less → left, Greater → right,
  Equal → skip.
- **Freshness proofs**: Each function that builds a new ParamBST from sorted pairs
  proves key uniqueness via `lemma_key_unique_insert` and contradiction.
- **spec_pair_set_to_map**: Bridge between `Set<(KV, VV)>` (ParamBST view) and
  `Map<KV, VV>` (OrderedTable view).

## Known Regression

`test_parallel_range_reduction` SIGKILL: ParamBST is an unbalanced BST. Sequential
insertion of 1..=2000 creates a degenerate tree with depth 2000. The `expose()` function
deep-clones subtrees at each level during `collect_in_order`, causing O(n^2) RwLock
allocations (~4M). The old AVL backing store had O(log n) depth (~11). This test passes
on main in 0.35s. The regression is inherent to using an unbalanced BST as backing store.

## Warnings (approved patterns only)

- 9 × `assume_eq_clone_workaround` (Clone/PartialEq bridge — approved pattern)
- 1 × `fn_missing_wf_ensures` (from prior code, not introduced by this change)
