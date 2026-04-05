# R148 Agent 1 — Traitify BSTAVL + BSTBBAlpha + BSTSetPlain in Chap37. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap37/BSTPlainStEph.rs` — the completed example. See how
`BSTPlainNodeFns` trait was created and 7 free functions became trait methods.

Report file: `plans/r148-agent1-chap37-avl-bbalpha-trait-report.md`

## Problem

Top-level free functions that operate on `BalBinTree<T>` should be trait methods.

## Files and functions

### BSTAVLStEph.rs (8 fns)

```
rotate_right, rotate_left, rebalance, insert_node, contains_node,
find_node, min_node, max_node
```

### BSTAVLMtEph.rs (8 fns)

Same functions, duplicated per standalone rule.

### BSTBBAlphaStEph.rs (7 fns)

```
insert_node, contains_node, find_node, min_node, max_node,
delete_min_node, delete_node
```

### BSTBBAlphaMtEph.rs (7 fns)

Same functions, duplicated per standalone rule.

### BSTSetPlainMtEph.rs (4 fns)

Check what these are. If they operate on `BalBinTree<T>` or the set type
as first arg, move to trait.

### BSTSetBBAlphaMtEph.rs (4 fns)

Same check.

### BSTSetAVLMtEph.rs (4 fns)

Same check.

## Pattern

Follow BSTPlainStEph.rs exactly:
1. Create a `<Variant>NodeFns<T>` trait (e.g., `BSTAVLNodeFns<T: TotalOrder>`)
2. Move free functions into `impl <Trait> for BalBinTree<T>`
3. `node: BalBinTree<T>` → `self` (consuming), `node: &BalBinTree<T>` → `&self`
4. Use `let ghost node = self;` at top of consuming methods to preserve proof refs
5. Update all call sites: `insert_node(tree, value)` → `tree.insert_node(value)`
6. Rotations/rebalance are internal helpers but still operate on the tree — make
   them trait methods too (they take `BalBinTree<T>` as first arg)

## Validation

Run `scripts/validate.sh isolate Chap37`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT change function logic or ensures/requires (except node → self).
- All existing RTTs must pass.

## When done

RCP.
