# R155 Agent 3 — Delegate Remaining OrderedTableStEph Methods — Report

## Summary

No new delegations were made this round. All remaining undelegated methods in
`OrderedTableStEph` depend on OrdKeyMap operations that agents 1 and 2 are adding
in R155. The file shrank from **5466 to 3967 lines** via the R154 agent1 rebase.

## Line Count

| State | Lines |
|-------|-------|
| Before rebase (stale agent3 branch) | 5466 |
| After rebase onto main (R154 agent1 work) | 3967 |
| After agent3 R155 work | 3967 (no change) |

## Methods Already Delegated (from R154 Agent 1)

| # | Method | Delegated to OrdKeyMap |
|---|--------|----------------------|
| 1 | `size` | `self.tree.size()` |
| 2 | `is_empty` | `self.tree.is_empty()` |
| 3 | `empty` | `OrdKeyMap::new()` |
| 4 | `find` | `self.tree.find(k)` |
| 5 | `lookup` | `self.tree.find(k)` |
| 6 | `find_iter` | `self.tree.find(k)` |
| 7 | `insert` | `self.tree.insert(k, v)` |
| 8 | `insert_iter` | `self.tree.insert(k, v)` |
| 9 | `delete` | `self.tree.delete(k)` |
| 10 | `delete_iter` | `self.tree.delete(k)` |
| 11 | `difference` | `self.tree.difference(&other.tree)` |
| 12 | `previous_key` | `self.tree.prev_key(k)` |
| 13 | `previous_key_iter` | `self.tree.prev_key(k)` |
| 14 | `next_key` | `self.tree.next_key(k)` |
| 15 | `next_key_iter` | `self.tree.next_key(k)` |
| 16 | `rank_key` | `self.tree.rank_key(k)` |
| 17 | `rank_key_iter` | `self.tree.rank_key(k)` |
| 18 | `select_key` | `self.tree.select_key(r)` |

## Methods Still Pending Delegation

| # | Chap | Method | Blocker | OrdKeyMap method needed | Agent |
|---|------|--------|---------|------------------------|-------|
| 1 | 43 | `singleton` | `OrdKeyMap::new()` does not ensure `spec_ordkeymap_wf()` | Fix `new()` wf | A1 |
| 2 | 43 | `union<F>` | No combiner-fn union in OrdKeyMap | `union_with<F>` | A1 |
| 3 | 43 | `intersection<F>` | No combiner-fn intersect in OrdKeyMap | `intersect_with<F>` | A1 |
| 4 | 43 | `split_key_iter` | `OrdKeyMap::split` lacks disjointness ensures | Add disjoint to `split` | A1 |
| 5 | 43 | `get_key_range_iter` | Same as above + `first_key`/`last_key` | `split` disjoint + `first_key`/`last_key` | A1+A2 |
| 6 | 43 | `split_rank_key_iter` | Same as above | `split` disjoint + `first_key`/`last_key` | A1+A2 |
| 7 | 43 | `first_key_iter` | No `first_key`/`min_key` in OrdKeyMap | `first_key` | A2 |
| 8 | 43 | `last_key_iter` | No `last_key`/`max_key` in OrdKeyMap | `last_key` | A2 |
| 9 | 43 | `collect` | No `collect`/`in_order` in OrdKeyMap | `collect` + `first_key` | A2 |
| 10 | 43 | `domain` | No `domain`/`in_order` in OrdKeyMap | `domain` | A2 |
| 11 | 43 | `map<F>` | No `map<F>` in OrdKeyMap | `map` | A2 |
| 12 | 43 | `filter<F>` | No `filter<F>` in OrdKeyMap | `filter` | A2 |
| 13 | 43 | `reduce<F>` | No `reduce<F>` in OrdKeyMap (needs `in_order`) | `collect` | A2 |
| 14 | 43 | `restrict` | No `restrict` in OrdKeyMap | `restrict` | A2 |
| 15 | 43 | `subtract` | No `subtract` in OrdKeyMap | `subtract` | A2 |

## Bridge Free Functions Still Active

| # | Chap | Function | Lines | Why Still Needed |
|---|------|----------|-------|-----------------|
| 1 | 43 | `bst_find_by_key` | ~150 | Used by `get_key_range_iter`, `split_rank_key_iter` |
| 2 | 43 | `bst_split_by_key` | ~220 | Used by `split_key_iter`, `get_key_range_iter`, `split_rank_key_iter`; provides disjointness ensures that `OrdKeyMap::split` currently lacks |

Both bridge functions can be removed once agent1 adds disjointness to `OrdKeyMap::split`
and agent2 adds the missing ops. The `bst_find_by_key` calls can then be replaced by
`self.tree.find(k)`, and `bst_split_by_key` by `self.tree.split(k)`.

## One Remaining `.tree.inner@` Access

`difference` (line ~2864) still has:
```rust
proof { lemma_pair_set_to_map_dom_finite(self.tree.inner@); }
```
This is needed because `OrdKeyMap::difference` does not ensure `result@.dom().finite()`.
Once agent1/2 strengthen the OrdKeyMap difference ensures, this can be removed too.

## Validation

- `scripts/validate.sh isolate Chap43`: **2805 verified, 0 errors**
- `scripts/rtt.sh`: **3736 tests passed, 0 failures**
- Full `scripts/validate.sh`: run below before commit

## Techniques Used

None — analysis only. Read all 24 standards files, read OrdKeyMap.rs (3515 lines)
and OrderedTableStEph.rs (3967 lines), mapped every remaining `.tree.inner` access
to its blocking OrdKeyMap method, confirmed no delegation is possible without agent1/2
R155 work.
