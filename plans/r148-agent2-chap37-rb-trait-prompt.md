# R148 Agent 2 — Traitify BSTRB in Chap37. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap37/BSTPlainStEph.rs` — the completed example. See how
`BSTPlainNodeFns` trait was created and 7 free functions became trait methods.

Report file: `plans/r148-agent2-chap37-rb-trait-report.md`

## Problem

Top-level free functions that operate on the RB tree type should be trait methods.

## Files and functions

### BSTRBStEph.rs (7 fns)

```
rotate_right, rotate_left, insert_node, contains_node, find_node,
min_node, max_node
```

### BSTRBMtEph.rs (21 fns)

```
new_node, is_red, size_link, update, rotate_left, rotate_right,
flip_colors, fix_up, insert_link, find_link, min_link, max_link,
in_order_collect, pre_order_collect, in_order_parallel, pre_order_parallel,
build_balanced, filter_parallel, reduce_parallel, height_rec,
compute_link_spec_size
```

This is the largest file. Many are internal helpers (is_red, flip_colors,
fix_up, update) but they all operate on the RB tree type as first arg.

### BSTSetRBMtEph.rs (4 fns)

Check what these are. Move to trait if they operate on the type.

## Pattern

Follow BSTPlainStEph.rs exactly:
1. Create `BSTRBNodeFns<T>` trait (StEph) and `BSTRBMtNodeFns<T>` trait (MtEph)
2. Move free functions into trait impls
3. `node`/`link` as first param → `self` or `&self`
4. Use `let ghost node = self;` for consuming methods
5. Update all call sites
6. Helpers like `is_red`, `flip_colors`, `fix_up` that take the tree as first
   arg become trait methods. Helpers that take non-tree first args stay free.

## Validation

Run `scripts/validate.sh isolate Chap37`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT change function logic or ensures/requires (except node → self).
- All existing RTTs must pass.

## When done

RCP.
