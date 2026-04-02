# R136 Agent 4 — Audit: Chap43 OrderedTable in_order() Usage

## Summary

Found **41 `in_order()` call sites** across 3 files (OrderedTableStEph: 21,
OrderedTableStPer: 20, OrderedTableMtEph: 1). Of these:

- **11 are BUGs** — operations APAS CS 43.2 specifies as O(lg n) that use O(n) traversals
- **10 are IMPROVABLE** — bulk operations that could delegate to BST-level set operations
- **20 are CORRECT** — operations that genuinely need all elements

The BST (Chap38 BSTParaStEph/MtEph) already exposes `find`, `split`, `min_key`,
`union`, `intersect`, `difference`, `filter`, and `join_pair` — most of these are unused
by the OrderedTable layer.

## Classification: OrderedTableStEph.rs (21 call sites)

### BUG — Should be O(lg n) per APAS CS 43.2

| # | Chap | File | Line | Function | Classification | BST operation to use | Difficulty |
|---|------|------|------|----------|---------------|---------------------|------------|
| 1 | 43 | OrderedTableStEph.rs | 1065 | `bst_find_by_key` | BUG | `tree.find()` returns `Option<Pair<K,V>>`, extract value | Easy — Agent 1 fixing |
| 2 | 43 | OrderedTableStEph.rs | 2753 | `first_key_iter` | BUG | `tree.min_key()` returns min element directly | Easy |
| 3 | 43 | OrderedTableStEph.rs | 2843 | `last_key_iter` | BUG | Need BST `max_key` (symmetric to `min_key`; not yet implemented) | Medium — needs BST max_key |
| 4 | 43 | OrderedTableStEph.rs | 2929 | `previous_key_iter` | BUG | `tree.split(k)` → left tree → `min_key` on left (or BST `max_key`) | Medium |
| 5 | 43 | OrderedTableStEph.rs | 3048 | `next_key_iter` | BUG | `tree.split(k)` → right tree → `tree.min_key()` on right | Medium |
| 6 | 43 | OrderedTableStEph.rs | 3164 | `split_key_iter` | BUG | `tree.split(k)` returns `(left, found, right)` directly | Medium — need to wrap results as OrderedTables and extract value |
| 7 | 43 | OrderedTableStEph.rs | 3419 | `get_key_range_iter` | BUG | Two `tree.split()` calls: split at k1, split at k2 | Medium |
| 8 | 43 | OrderedTableStEph.rs | 3539 | `rank_key_iter` | BUG | Need BST augmented `rank` (size-based, O(lg n)) | Hard — needs size augmentation at BST level |
| 9 | 43 | OrderedTableStEph.rs | 3669 | `select_key` | BUG | Need BST augmented `select` (size-based, O(lg n)) | Hard — needs size augmentation at BST level |
| 10 | 43 | OrderedTableStEph.rs | 3726 | `split_rank_key_iter` | BUG | Need BST `select` to find the key, then `split` | Hard — depends on select |

### IMPROVABLE — Bulk operations using in_order + rebuild instead of BST ops

| # | Chap | File | Line | Function | Classification | Better approach | Difficulty |
|---|------|------|------|----------|---------------|----------------|------------|
| 1 | 43 | OrderedTableStEph.rs | 1751 | `intersection` | IMPROVABLE | `tree.intersect()` at BST level, then rebuild table | Medium — BST intersect uses split+join; need merge fn for values |
| 2 | 43 | OrderedTableStEph.rs | 1959+2113 | `union` (2 calls) | IMPROVABLE | `tree.union()` at BST level — but needs merge fn for duplicate keys | Medium — same value-merge issue |
| 3 | 43 | OrderedTableStEph.rs | 2329 | `difference` | IMPROVABLE | `tree.difference()` at BST level | Easy — no value merge needed |
| 4 | 43 | OrderedTableStEph.rs | 2460 | `restrict` | IMPROVABLE | `tree.filter()` checking key membership | Easy — filter already exists |
| 5 | 43 | OrderedTableStEph.rs | 2582 | `subtract` | IMPROVABLE | `tree.filter()` checking key non-membership | Easy — filter already exists |

### CORRECT — Operations that genuinely need all elements

| # | Chap | File | Line | Function | Reason |
|---|------|------|------|----------|--------|
| 1 | 43 | OrderedTableStEph.rs | 1351 | `domain` | Must return all keys — O(n) inherent |
| 2 | 43 | OrderedTableStEph.rs | 1550 | `map` | Must transform all values — O(n) inherent |
| 3 | 43 | OrderedTableStEph.rs | 1725 | `reduce` | Must aggregate all entries — O(n) inherent |
| 4 | 43 | OrderedTableStEph.rs | 2705 | `collect` | Must return all entries — O(n) inherent |
| 5 | 43 | OrderedTableStEph.rs | 3928 | `iter` | Must iterate all entries — O(n) inherent |

## Classification: OrderedTableStPer.rs (20 call sites)

StPer mirrors StEph exactly. Every function with a StEph BUG has the same BUG in StPer.

### BUG

| # | Chap | File | Line | Function |
|---|------|------|------|----------|
| 1 | 43 | OrderedTableStPer.rs | 389 | `bst_find_by_key` |
| 2 | 43 | OrderedTableStPer.rs | 2286 | `first_key_iter` |
| 3 | 43 | OrderedTableStPer.rs | 2374 | `last_key_iter` |
| 4 | 43 | OrderedTableStPer.rs | 2458 | `previous_key_iter` |
| 5 | 43 | OrderedTableStPer.rs | 2571 | `next_key_iter` |
| 6 | 43 | OrderedTableStPer.rs | 2683 | `split_key_iter` |
| 7 | 43 | OrderedTableStPer.rs | 2908 | `get_key_range_iter` |
| 8 | 43 | OrderedTableStPer.rs | 3014 | `rank_key_iter` |
| 9 | 43 | OrderedTableStPer.rs | 3144 | `select_key` |
| 10 | 43 | OrderedTableStPer.rs | 3201 | `split_rank_key_iter` |

### IMPROVABLE

| # | Chap | File | Line | Function |
|---|------|------|------|----------|
| 1 | 43 | OrderedTableStPer.rs | 1446 | `intersection` |
| 2 | 43 | OrderedTableStPer.rs | 1602+1716 | `union` (2 calls) |
| 3 | 43 | OrderedTableStPer.rs | 1930 | `difference` |
| 4 | 43 | OrderedTableStPer.rs | 2038 | `restrict` |
| 5 | 43 | OrderedTableStPer.rs | 2140 | `subtract` |

### CORRECT

| # | Chap | File | Line | Function |
|---|------|------|------|----------|
| 1 | 43 | OrderedTableStPer.rs | 1073 | `domain` |
| 2 | 43 | OrderedTableStPer.rs | 1259 | `map` |
| 3 | 43 | OrderedTableStPer.rs | 2241 | `collect` |
| 4 | 43 | OrderedTableStPer.rs | 3402 | `iter` |

Note: StPer has no `reduce` in the in_order list (may use BST reduce directly).

## Classification: OrderedTableMtEph.rs (1 call site)

| # | Chap | File | Line | Function | Classification | Reason |
|---|------|------|------|----------|---------------|--------|
| 1 | 43 | OrderedTableMtEph.rs | 759 | `iter` | CORRECT | Iterator must snapshot all elements |

## Classification: Other Chap43 Files (0 call sites)

AugOrderedTableStEph.rs, AugOrderedTableStPer.rs, AugOrderedTableMtEph.rs,
OrderedSetMtEph.rs, OrderedSetStEph.rs, OrderedSetStPer.rs — **no `in_order()` calls**.

## Fix Priority (by difficulty and impact)

### Tier 1 — Easy fixes, high impact (O(n) → O(lg n))

These use BST operations that already exist in BSTParaStEph:

| # | Function | Both files | Fix |
|---|----------|-----------|-----|
| 1 | `bst_find_by_key` | StEph:1065, StPer:389 | Use `tree.find(Pair(k, dummy))` or add a key-only find. Agent 1 handling. |
| 2 | `first_key_iter` | StEph:2753, StPer:2286 | Use `tree.min_key()` → extract `.0` (key) |
| 3 | `split_key_iter` | StEph:3164, StPer:2683 | Use `tree.split(k)` → wrap left/right as OrderedTable |
| 4 | `get_key_range_iter` | StEph:3419, StPer:2908 | Two `tree.split()` calls: split at k1 for right half, split at k2 for left half |
| 5 | `next_key_iter` | StEph:3048, StPer:2571 | `tree.split(k)` → `right.min_key()` |

### Tier 2 — Medium fixes (need BST max_key or careful split usage)

| # | Function | Both files | Fix |
|---|----------|-----------|-----|
| 1 | `last_key_iter` | StEph:2843, StPer:2374 | Need `max_key` on BST (symmetric to `min_key`; doesn't exist yet). Or: `tree.split(MAX)` doesn't help. Must add `max_key` to BSTParaStEph/MtEph. |
| 2 | `previous_key_iter` | StEph:2929, StPer:2458 | `tree.split(k)` → need `max_key` on left tree. Blocked on `max_key`. |

### Tier 3 — Hard fixes (need BST augmentation)

| # | Function | Both files | Fix |
|---|----------|-----------|-----|
| 1 | `rank_key_iter` | StEph:3539, StPer:3014 | APAS says "augment the underlying BST with sizes" for O(lg n) rank. BST already stores `size` per node, but no `rank` operation exposed. Need to add BST-level `rank(key)` that walks the tree counting left-subtree sizes. |
| 2 | `select_key` | StEph:3669, StPer:3144 | Need BST-level `select(i)` that walks using subtree sizes. |
| 3 | `split_rank_key_iter` | StEph:3726, StPer:3201 | Depends on `select` + `split`. |

### Tier 4 — IMPROVABLE bulk operations (O(m lg(n/m)) instead of O(n+m))

These are not strictly bugs — they still run in O(n) — but they could use BST-level
set operations which would give better asymptotics for the `m << n` case and parallelism:

| # | Function | Fix |
|---|----------|-----|
| 1 | `difference` | Delegate to `tree.difference()` — straightforward |
| 2 | `restrict` | Delegate to `tree.filter()` with key-membership predicate |
| 3 | `subtract` | Delegate to `tree.filter()` with key-non-membership predicate |
| 4 | `intersection` | Delegate to BST intersect — but table intersection needs a merge fn for values when both keys present |
| 5 | `union` | Delegate to BST union — same value-merge issue as intersection |

## Key Blocker: BST `max_key`

The BST trait (`ParamBSTTrait`) has `min_key` but not `max_key`. Adding `max_key` is
straightforward (mirror `min_key` — traverse right branches instead of left). This
unblocks `last_key_iter` and `previous_key_iter`.

## Key Blocker: BST `rank` and `select`

The BST stores `size` in each node (the `NodeInner.size` field), so the data for O(lg n)
rank and select is already present. What's missing is the algorithmic logic:

- **`rank(key)`**: Walk the tree. At each node, if key < root, recurse left. If key > root,
  add left.size + 1 and recurse right. If key == root, return left.size.
- **`select(i)`**: Walk the tree. At each node, if i < left.size, recurse left. If
  i == left.size, return root key. If i > left.size, recurse right with i - left.size - 1.

These are textbook operations (APAS Chapter 43 mentions them). Adding them to BSTParaStEph
is medium difficulty (algorithmic logic is simple, proof work is the challenge).

## Existing BST Operations Unused by OrderedTable

The OrderedTable layer barely delegates to BST operations. Currently used:

| BST Operation | Used by OrderedTable? |
|--------------|----------------------|
| `new` | Yes |
| `singleton` | No (builds via insert) |
| `expose` | No |
| `join_mid` | No |
| `size` | Yes |
| `is_empty` | Yes |
| `insert` | Yes |
| `delete` | Yes |
| `find` | Yes (but via in_order scan!) |
| `split` | No — should be used for split_key, getRange, previous, next |
| `join_pair` | No |
| `join_m` | No |
| `union` | No — should be used for table union |
| `intersect` | No — should be used for table intersection |
| `difference` | No — should be used for table difference |
| `filter` | Yes (used in `filter` only) |
| `reduce` | No |
| `min_key` | No — should be used for first_key |
| `in_order` | Yes (21 times!) |

## Conclusion

The OrderedTable layer was built using a "flatten to array, scan, rebuild" pattern for
nearly every operation. This is O(n) for everything. For the 10 operations APAS specifies
as O(lg n) (first, last, previous, next, split, join, getRange, rank, select, splitRank),
this is a systematic performance bug — not just isolated cases.

The fix path is:
1. Agent 1 fixes `bst_find_by_key` (in progress)
2. Add `max_key` to BST (unblocks last, previous)
3. Fix first/next/split/getRange to use existing BST split+min_key
4. Add rank/select to BST (unblocks rank, select, splitRank)
5. (Optional) Migrate intersection/union/difference to use BST-level ops
