# OrdKeyMap Refactor: Eliminating the Set-to-Map Bridge

## Status: COMPLETE (R152-R158)

OrdKeyMap is built, deployed, and OrderedTable delegates to it. This doc
records the design rationale and final results.

## Problem

OrderedTableStEph had 5,569 proof lines — the largest module in the codebase.
Most were bridge proof: converting between ParamBST's `Set<(K::V, V::V)>`
view and OrderedTable's `Map<K::V, V::V>` view. The same bridge was duplicated
in StPer (4,339 lines). Total: ~9,900 lines for the same abstraction twice.

The root cause was architectural. ParamBST (Chap38) views as `Set<T::V>`. When
`T = Pair<K,V>`, that gives `Set<(K::V, V::V)>` — a set of key-value tuples.
OrderedTable (Chap43) needs `Map<K::V, V::V>`. Every operation re-proved that
BST properties imply Map properties.

## Why ParamBST's Set View Is Correct

ParamBST is `ParamBST<T: Ord>` — a BST of elements ordered by the elements
themselves. When `T` is a plain key type (no value), the BST is a set:
`find(x)` searches for `x` by comparing `x` against nodes. OrderedSet uses
it this way. `Set<T::V>` is the correct view at this level.

The Map abstraction only arises when `T = Pair<K,V>` and comparison uses
only the key. ParamBST doesn't know that — it just sees `T: Ord` and
compares.

| Consumer | T | Correct View |
|----------|---|-------------|
| OrderedSet (Chap43) | `K` | `Set<K::V>` — ParamBST provides this |
| OrderedTable (Chap43) | `Pair<K,V>` | `Map<K::V, V::V>` — needs bridge |

Rust/Verus does not support switching a View associated type based on the
type parameter. So ParamBST stays as `Set<T::V>` and the Map bridge belongs
in a wrapper: OrdKeyMap.

## Architecture

```
Chap38: ParamBST<Pair<K,V>>     ──View──>  Set<(K::V, V::V)>
              │
Chap41: OrdKeyMap<K,V>          ──View──>  Map<K::V, V::V>
              │                             27 methods, bridge proofs once
Chap43: OrderedTableStEph<K,V>  ──View──>  Map<K::V, V::V>
                                            thin wrapper, delegates to OrdKeyMap
```

OrdKeyMap lives in Chap41 (not Chap38) because it needs ArraySetStEph (Chap41)
for domain/tabulate/restrict/subtract. Chap41 depends on Chap38, so the
dependency direction is correct.

## OrdKeyMap Methods (27)

| Category | Methods |
|----------|---------|
| Construction | new, Clone |
| Lookup | find, size, is_empty |
| Modification | insert, delete |
| Bulk | union, union_with, intersect, intersect_with, difference |
| Ordering | next_key, prev_key, first_key, last_key, rank_key, select_key |
| Range | split, get_key_range, split_rank_key |
| Higher-order | filter, map_values, reduce, collect |
| ArraySetStEph | domain, tabulate, restrict, subtract |

## Results

### Line counts

| Module | Before (R152) | After (R157) | Delta |
|--------|--------------|-------------|-------|
| OrderedTableStEph | 5,569 | 2,183 | **-3,386** |
| OrderedTableStPer | 4,339 | 2,281 | **-2,058** |
| OrdKeyMap (new) | 0 | ~4,800 | +4,800 |
| **Net** | **9,908** | **~9,264** | **-644** |

Net reduction is modest because OrdKeyMap absorbed the proofs. The real win
is structural: bridge proofs written once instead of 2-3 times, and the
`bst_*_by_key` re-implementations (6 functions, ~2,000 lines total) eliminated.

### Verification

| Metric | Before | After |
|--------|--------|-------|
| Verified | 5,702 | 5,763 |
| Errors | 0 | 0 |
| Holes | 4 | 4 |
| RTT | 3,690 | 3,776 |

No regressions. +61 verified functions (OrdKeyMap methods), +86 RTTs.

### What was eliminated from OrderedTable

- 6 `bst_*_by_key` functions (~2,000 lines): bst_find_by_key, bst_next_by_key,
  bst_prev_by_key, bst_split_by_key, bst_rank_by_key, bst_select_by_rank
- 17 bridge lemmas (~500 lines): key_unique_*, pair_set_to_map_*, view_gen_*,
  set_to_map_*, cmp_* — all moved to OrdKeyMap
- Bridge spec fns (~100 lines): spec_pair_set_to_map, spec_key_unique_pairs_set,
  spec_set_pair_view_generated

### What remains in OrderedTable

Methods that can't delegate due to type mismatches:
- `collect` returns `AVLTreeSeqStPerS`, OrdKeyMap returns `Vec`
- `reduce` takes `(R, &K, &V) -> R`, OrdKeyMap takes `(&V, &V) -> V`
- `join_key` — no OrdKeyMap equivalent
- Iterator infrastructure (iter, IntoIterator)
- `_iter` suffix variants (iterative alternatives per standard)

These are thin (~50-100 lines each) and don't carry bridge proofs.

## Key Technique: Trigger Bridge for Closure Ensures

Delegating union/intersect through OrdKeyMap's `union_with`/`intersect_with`
hit a Verus limitation: closure ensures from `&F` don't unify with ensures
from `F` in the SMT encoding. The fix (discovered by agent 3, R156):

```rust
proof {
    assert forall|k: K::V| hypothesis(k) implies conclusion(k) by {
        let vk = result@[k];
        assert(result@[k] == vk);  // materializes ground term for Z3
    };
}
```

Binding `result@[k]` to a fresh spec variable creates a ground term that
matches the trigger pattern in OrdKeyMap's ensures, firing the quantifier
instantiation.

## Rounds

| Round | What | Delta |
|-------|------|-------|
| R152 | Built OrdKeyMap: struct, View, wf, 7 basic methods, 17 bridge lemmas | +1,467 |
| R153 | Added union/intersect/difference + next/prev/rank/select. Migrated StEph/StPer struct fields | +1,363, -116 |
| R154 | Delegated StEph 5 methods, StPer 8 methods, deleted 4 bst_*_by_key | -1,732 |
| R155 | Added union_with/intersect_with, split disjointness, first/last_key, get_key_range, split_rank_key. StPer -884 | +450, -884 |
| R156 | Added filter/map/reduce/collect/Clone. StEph -960, StPer -1,147 | +196, -2,107 |
| R157 | Added domain/tabulate/restrict/subtract. StEph union/intersect delegated. Dead code cleanup -293 | +200, -1,127 |
| R158 | Final delegation + cleanup (in progress) | TBD |
