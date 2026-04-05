# R147 Agent 1 — Chap37 BSTPlain trait refactor report

## Summary

Moved 7 top-level free functions into traits for both BSTPlainStEph.rs and
BSTPlainMtEph.rs. Functions now dispatch as trait methods on `BalBinTree<T>`.

## Changes

### BSTPlainStEph.rs

Created `BSTPlainNodeFns<T: TotalOrder>` trait extending `BSTSpecFns<T> + BalBinTreeTrait<T>`.
7 free functions moved into `impl BSTPlainNodeFns<T> for BalBinTree<T>`:

| # | Chap | File | Function | Self type | Change |
|---|------|------|----------|-----------|--------|
| 1 | 37 | BSTPlainStEph.rs | insert_node | self (consuming) | free fn -> trait method |
| 2 | 37 | BSTPlainStEph.rs | contains_node | &self | free fn -> trait method |
| 3 | 37 | BSTPlainStEph.rs | find_node | &self | free fn -> trait method |
| 4 | 37 | BSTPlainStEph.rs | min_node | &self | free fn -> trait method |
| 5 | 37 | BSTPlainStEph.rs | max_node | &self | free fn -> trait method |
| 6 | 37 | BSTPlainStEph.rs | delete_min_node | self (consuming) | free fn -> trait method |
| 7 | 37 | BSTPlainStEph.rs | delete_node | self (consuming) | free fn -> trait method |

Call sites in `BSTPlainStEphTrait` impl updated:
- `insert_node(self.root, value)` -> `self.root.insert_node(value)`
- `contains_node(&self.root, target)` -> `self.root.contains_node(target)`
- etc.

### BSTPlainMtEph.rs

Created `BSTPlainMtNodeFns<T: TotalOrder>` trait (separate from StEph per standalone rule).
Same 7 functions with Mt-specific ensures (size/height bounds).
Layer 2 call sites updated:
- `insert_node(tree, value)` -> `tree.insert_node(value)`
- `delete_node(tree, target)` -> `tree.delete_node(target)`
- `contains_node(tree_ref, target)` -> `tree_ref.contains_node(target)`
- etc.

## Technique

For consuming methods (`insert_node`, `delete_min_node`, `delete_node`), added
`let ghost node = self;` at top of body to preserve all existing proof block references
to `node` without modification.

For borrow methods (`contains_node`, `find_node`, `min_node`, `max_node`), `match node`
became `match self` — no proof blocks referenced `node` directly.

## Verification

- Chap37 isolate: 1946 verified, 0 errors
- RTT: 3690 passed, 0 skipped
- No new holes, no new assumes/accepts

## Remaining work

14 free functions across other Chap37 files (AVL, BB-alpha, RB, Splay) follow the
same pattern. Those are for future rounds after this pattern is validated.
