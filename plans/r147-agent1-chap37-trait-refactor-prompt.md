# R147 Agent 1 — Move Chap37 top-level functions into traits. AFK.

## Setup

Read ALL files in `src/standards/` before starting. Pay close attention to:
- `multi_struct_standard.rs` — per-type traits, bottom-up ordering.
- `helper_function_placement_standard.rs` — trait methods for `&self` helpers.

Report file: `plans/r147-agent1-chap37-trait-refactor-report.md`

## Problem

Chap37 BST files have ~169 top-level free functions that operate on
`BalBinTree<T>` but are not trait methods. These should be methods on
a trait implemented for `BalBinTree<T>` (or the BST-specific type).

Example from BSTPlainStEph.rs:
```rust
fn insert_node<T: TotalOrder>(node: BalBinTree<T>, value: T) -> (inserted: BalBinTree<T>)
fn contains_node<T: TotalOrder>(node: &BalBinTree<T>, target: &T) -> (found: bool)
fn find_node<'a, T: TotalOrder>(node: &'a BalBinTree<T>, target: &T) -> (found: Option<&'a T>)
fn min_node<T: TotalOrder>(node: &BalBinTree<T>) -> (min: Option<&T>)
fn max_node<T: TotalOrder>(node: &BalBinTree<T>) -> (max: Option<&T>)
fn delete_min_node<T: TotalOrder>(node: BalBinTree<T>) -> (pair: (BalBinTree<T>, T))
fn delete_node<T: TotalOrder>(node: BalBinTree<T>, target: &T) -> (deleted: BalBinTree<T>)
```

These should be trait methods. The first parameter (`node: BalBinTree<T>`
or `node: &BalBinTree<T>`) becomes `self` or `&self`.

## What to do

Start with the simplest file: **BSTPlainStEph.rs** (7 top-level functions).
Then do **BSTPlainMtEph.rs** (same 7, duplicated per standalone rule).

### For each file:

1. Read the file. Identify all top-level free functions.

2. For functions where the first parameter is `BalBinTree<T>` or
   `&BalBinTree<T>`, move them into the existing trait as methods.
   - `node: &BalBinTree<T>` → `&self`
   - `node: BalBinTree<T>` → `self` (consuming)

3. Functions returning multiple values (tuples) CAN still be trait methods.
   `delete_min_node` returns `(BalBinTree<T>, T)` — that's fine as a trait
   method: `fn delete_min(self) -> (pair: (Self, T))`.

4. Internal helpers (rotations, rebalance) that are only called by other
   trait methods: make them trait methods too, or leave as free functions
   if they don't take the tree as first arg.

5. Update all call sites: `insert_node(tree, value)` → `tree.insert_node(value)`.
   For consuming methods: same pattern, tree is moved into self.
   For `&self` methods: `contains_node(&tree, target)` → `tree.contains_node(target)`.

6. The MtEph file's Layer 2 calls Layer 1 functions. Update those call sites too:
   `insert_node(tree, value)` → `tree.insert_node(value)`.

### Scope for this round

Do ONLY these two files:
- `src/Chap37/BSTPlainStEph.rs`
- `src/Chap37/BSTPlainMtEph.rs`

Do NOT touch the other Chap37 files (AVL, BB-alpha, RB, Splay). Those are
for future rounds after the pattern is validated on Plain.

## Naming

Keep existing function names. The trait already exists (`BSTPlainStEphTrait`
or similar) — add the methods to it. If no trait exists for Layer 1, create
one following the multi-struct standard.

The `BSTSpecFns` trait in BSTPlainStEph.rs has spec functions. The exec
functions (insert_node, etc.) should go in the same trait or a companion
exec trait.

## Validation

Run `scripts/validate.sh isolate Chap37`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT change function signatures (except `node` → `self`).
- Do NOT change ensures/requires (except `node` → `self`).
- Do NOT change the MtEph trait signatures (the locked trait is the public API).
- All existing RTTs must pass.
- Functions with recursive `decreases` still work as trait methods —
  use `decreases *self` or `decreases self.spec_size()`.

## When done

RCP.
