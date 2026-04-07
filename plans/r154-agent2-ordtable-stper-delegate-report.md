# R154 Agent 2 Report: OrderedTableStPer Delegation to OrdKeyMap

## Summary

Delegated 8 OrderedTableStPer methods to OrdKeyMap, eliminating ~260 lines of
bridge proof code. Bypassed 2 dead free functions and 1 unused proof lemma.

## Methods Delegated

| # | Chap | File | Method | Delegate To | Lines Removed |
|---|------|------|--------|-------------|---------------|
| 1 | 43 | OrderedTableStPer.rs | `insert` | `OrdKeyMap::insert` (clone+mutate) | ~60 |
| 2 | 43 | OrderedTableStPer.rs | `delete` | `OrdKeyMap::delete` (clone+mutate) | ~25 |
| 3 | 43 | OrderedTableStPer.rs | `previous_key_iter` | `OrdKeyMap::prev_key` | ~2 |
| 4 | 43 | OrderedTableStPer.rs | `next_key_iter` | `OrdKeyMap::next_key` | ~2 |
| 5 | 43 | OrderedTableStPer.rs | `rank_key` | `OrdKeyMap::rank_key` | ~2 |
| 6 | 43 | OrderedTableStPer.rs | `select_key` | `OrdKeyMap::select_key` | ~55 |
| 7 | 43 | OrderedTableStPer.rs | `difference` | `OrdKeyMap::difference` | ~105 |

## Dead Code Bypassed (`#[cfg(any())]`)

| # | Chap | File | Item | Reason |
|---|------|------|------|--------|
| 1 | 43 | OrderedTableStPer.rs | `bst_next_by_key` (~355 lines) | Replaced by OrdKeyMap::next_key |
| 2 | 43 | OrderedTableStPer.rs | `bst_prev_by_key` (~335 lines) | Replaced by OrdKeyMap::prev_key |
| 3 | 43 | OrderedTableStPer.rs | `lemma_cmp_antisymmetry` (~10 lines) | Unused after delegation |

## Not Delegated (with reasons)

| # | Chap | File | Method | Reason |
|---|------|------|--------|--------|
| 1 | 43 | OrderedTableStPer.rs | `split_key_iter` | OrdKeyMap.split trait doesn't expose BST-level disjointness |
| 2 | 43 | OrderedTableStPer.rs | `empty` | OrdKeyMap::new() doesn't ensure wf |
| 3 | 43 | OrderedTableStPer.rs | `singleton` | Can't call insert without wf from new() |
| 4 | 43 | OrderedTableStPer.rs | `find` | Requires `find_pre` (weaker than `wf`); OrdKeyMap::find requires `wf` |
| 5 | 43 | OrderedTableStPer.rs | `union`/`intersection` | OrderedTableStPer takes combiner fn; OrdKeyMap doesn't |
| 6 | 43 | OrderedTableStPer.rs | `domain`/`tabulate`/`map`/`filter`/`collect` | No OrdKeyMap equivalents |
| 7 | 43 | OrderedTableStPer.rs | `restrict`/`subtract`/`get_key_range`/`split_rank_key` | No OrdKeyMap equivalents |
| 8 | 43 | OrderedTableStPer.rs | `first_key`/`last_key` | No OrdKeyMap equivalents |

## Line Count

- Before: 4326 lines
- After: 4093 lines
- Net: -233 lines (27 insertions, 260 deletions)

## Verification

- Isolate Chap43: 2810 verified, 0 errors
- Full validate: 5754 verified, 0 errors
- RTT: 3717 passed, 0 skipped
- PTT: not run per prompt

## Techniques

- **Clone+mutate for persistent semantics**: For `insert`/`delete`, clone the inner
  ParamBST to create a fresh OrdKeyMap, then call the `&mut self` method on it, then
  wrap in OrderedTableStPer. ParamBST::clone ensures `cloned@ == self@`, which preserves
  all wf properties.

- **Direct trait delegation**: For `next_key`, `prev_key`, `rank_key`, `select_key`,
  `difference`, the OrdKeyMap trait methods have matching ensures, so delegation is
  a one-liner with no proof needed.

## Files Modified

- `src/Chap43/OrderedTableStPer.rs` — 8 methods delegated, 3 items bypassed
