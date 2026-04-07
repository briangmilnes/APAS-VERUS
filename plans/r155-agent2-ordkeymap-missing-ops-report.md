# R155 Agent 2 Report: OrdKeyMap Missing Operations

## Summary

Added 4 new methods to `OrdKeyMapTrait` and their implementations in `OrdKeyMap`:

| # | Chap | File | Method | Complexity | Status |
|---|------|------|--------|-----------|--------|
| 1 | 38 | OrdKeyMap.rs | `first_key` | O(lg n) | Verified |
| 2 | 38 | OrdKeyMap.rs | `last_key` | O(lg n) | Verified |
| 3 | 38 | OrdKeyMap.rs | `get_key_range` | O(lg n) | Verified |
| 4 | 38 | OrdKeyMap.rs | `split_rank_key` | O(n) | Verified |

## Techniques

- **first_key / last_key**: Delegate to `ParamBST::min_key()` / `max_key()`, extract key via `clone_plus`, bridge BST ordering (`cmp_spec`) to `TotalOrder::le` using `reveal_param_bst_backings` and the spec_pair_key_determines_order axiom.

- **get_key_range**: Two BST-level splits (`ordkeymap_split` at k1, then at k2 on the right part) plus re-insertion of k1 and k2 if present in original (via `ordkeymap_find`). Uses BST-level `ParamBST::insert` (not OrdKeyMap::insert) to avoid the stricter `dom().len() + 1 < usize::MAX` precondition. Proof tracks BST-level subset: `result_tree@ ⊆ self.inner@`.

- **split_rank_key**: For i >= size, returns (clone_of_self, empty). Otherwise, uses `in_order()` to find the pair at rank i, splits via `ordkeymap_split`, and re-inserts the split key into the right half. O(n) due to in_order traversal.

## What was NOT added (and why)

- **domain** (`-> ArraySetStEph<K>`): ArraySetStEph is in Chap41, which depends on Chap38. Adding this import creates a circular dependency in isolate mode feature flags.

- **collect** (`-> AVLTreeSeqStPerS<Pair<K,V>>`): AVLTreeSeqStPerS is in Chap37, also a forward dependency.

- **tabulate, restrict, subtract**: All require ArraySetStEph input parameters. Same circular dependency issue.

- **filter, map**: These are implementable (no external type dependencies) but were deferred since Priority 1 and 3 items were more valuable for enabling OrderedTable delegation.

## Verification

- Isolate Chap38: 1215 verified, 0 errors
- Full: 5758 verified, 0 errors
- RTT: 3717 passed, 0 skipped
