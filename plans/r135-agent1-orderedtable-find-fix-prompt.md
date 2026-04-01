# R135 Agent 1 — Fix OrderedTable find: O(n) → O(lg n). AFK.

## Setup

Read ALL files in `src/standards/` before starting.

Report file: `plans/r135-agent1-orderedtable-find-report.md`

## Problem

`src/Chap43/OrderedTableStEph.rs` function `bst_find_by_key` (line ~1048) implements
find by calling `tree.in_order()` (O(n) traversal to sorted Vec) then linear scanning
the Vec for the key. Total cost: O(n). Should be O(lg n) via BST search.

The BST (`ParamBST<Pair<K,V>>` from Chap38) already has O(lg n) find:
`ParamBST::find(&self, key: &T) -> Option<T>` which does recursive BST search via
`expose()` + `cmp` + recurse left/right.

The problem: `ParamBST::find` searches by the full element type `T` (here `Pair<K,V>`).
But `bst_find_by_key` searches by key `K` only. The BST orders by `Pair<K,V>` where
`Pair`'s `Ord` is determined by the key (see `spec_pair_key_determines_order`). So
searching for a key `k` means searching for any `Pair` whose `.0 == k`.

## What to do

1. Read `bst_find_by_key` at line ~1048. Understand the current ensures (strong:
   returns the value and proves containment).

2. Read `ParamBST::find` in `src/Chap38/BSTParaStEph.rs` (line ~645). It returns
   `Option<T>` where T is the full element (here `Pair<K,V>`).

3. Rewrite `bst_find_by_key` to use BST search instead of in_order + linear scan.
   Two approaches:

   **Approach A**: Use `ParamBST::find` with a dummy Pair. Construct a search key
   `Pair(k, dummy_v)` and call `self.tree.find(&search_pair)`. Since Pair's Ord only
   compares keys, the dummy value doesn't affect the search. Extract `.1` from the
   result.

   **Approach B**: Write a recursive BST search directly on `ParamBST<Pair<K,V>>` that
   compares only keys. Use `expose()` to get left/root/right, compare `k` with
   `root.0`, recurse. This avoids needing a dummy value but is more code.

   Approach A is simpler if you can construct a dummy V. Check if V has a Default or
   if you can use any value (the search only compares keys). If V doesn't have a
   convenient default, Approach B is cleaner.

4. Preserve the existing ensures — the result must include value correspondence:
   ```
   Some(v) => spec_pair_set_to_map(tree@).contains_key(k@)
       && v@ == spec_pair_set_to_map(tree@)[k@],
   None => !spec_pair_set_to_map(tree@).contains_key(k@),
   ```

5. Update the alg analysis annotation from O(n) to O(lg n).

6. Check `OrderedTableStPer.rs` — it likely has the same `bst_find_by_key` pattern.
   Fix both files.

7. Check `AugOrderedTableStEph.rs` — its `find` delegates to `base_table.find(k)`
   which calls `bst_find_by_key`. It inherits the fix automatically, but update its
   annotation from O(n) to O(lg n).

8. Check `OrderedTableMtEph.rs` and `AugOrderedTableMtEph.rs` — their `find` delegates
   through the lock to the St version. Update annotations.

## Validation

Run `scripts/validate.sh isolate Chap43`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken ensures.
- The fix must be O(lg n) — BST search, not linear scan.
- Read `ParamBST::find` and `expose()` in Chap38 before writing.
- Update all annotations that reference this function's cost.

## When done

RCP.
