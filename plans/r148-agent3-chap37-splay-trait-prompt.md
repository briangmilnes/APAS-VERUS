# R148 Agent 3 — Traitify BSTSplay in Chap37. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap37/BSTPlainStEph.rs` — the completed example. See how
`BSTPlainNodeFns` trait was created and 7 free functions became trait methods.

Report file: `plans/r148-agent3-chap37-splay-trait-report.md`

## Problem

Top-level free functions that operate on the Splay tree type should be trait methods.

## Files and functions

### BSTSplayStEph.rs (12 fns)

```
new_node, size_link, height_link, update, splay, bst_insert,
insert_link, find_link, min_link, max_link, in_order_collect,
pre_order_collect
```

### BSTSplayMtEph.rs (19 fns)

Same core functions plus Mt-specific additions (parallel variants, etc.).
Check each — if the first parameter is the tree type, make it a trait method.

### BSTSetSplayMtEph.rs (4 fns)

Check what these are. Move to trait if they operate on the type.

## Pattern

Follow BSTPlainStEph.rs exactly:
1. Create `BSTSplayNodeFns<T>` trait (StEph) and `BSTSplayMtNodeFns<T>` trait (MtEph)
2. Move free functions into trait impls
3. First param → `self` or `&self`
4. Use `let ghost node = self;` for consuming methods
5. Update all call sites

**Note on Splay:** The `splay` function may consume and return a modified tree.
This is fine as a consuming trait method: `fn splay(self, target: &T) -> Self`.

## Caution: BSTSplayStEph SMT sensitivity

BSTSplayStEph.rs has known SMT budget sensitivity (see CLAUDE.md —
`BSTSplayStEph.rs SMT Budget Limit`). Adding requires to splay helpers
destabilizes the proof. When moving functions to traits, do NOT add new
requires — keep the exact same requires/ensures. The `// veracity: no_requires`
annotations on splay helpers must be preserved.

If verification fails on this file due to SMT flakiness (not your changes),
leave the failing function as a free function and document it.

## Validation

Run `scripts/validate.sh isolate Chap37`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT change function logic or ensures/requires (except node → self).
- Do NOT add requires to splay helpers (SMT sensitivity).
- All existing RTTs must pass.

## When done

RCP.
