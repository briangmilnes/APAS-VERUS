# R151 Agent 1 — Add TotalOrder to Trait Bounds. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Pay close attention to `total_order_standard.rs`.

Report file: `plans/r151-agent1-totalorder-bounds-report.md`

## Problem

Veracity rule [23b] flags 25 instances where free functions require `TotalOrder`
but the module's trait does not have it in its bounds. The free fns need
`TotalOrder` for spec-level ordering (`TotalOrder::le`), but the trait only
says `T: StT + Ord` or similar. The fix is to add `+ TotalOrder` to the
trait's type parameter bounds.

## Files (11 files, 25 warnings)

```
Chap35/OrderStatSelectMtEph.rs
Chap35/OrderStatSelectMtPer.rs
Chap35/OrderStatSelectStEph.rs
Chap35/OrderStatSelectStPer.rs
Chap36/QuickSortMtEphSlice.rs
Chap40/BSTKeyValueStEph.rs
Chap41/AVLTreeSetStEph.rs
Chap41/AVLTreeSetStPer.rs
Chap43/OrderedSetStEph.rs
Chap43/OrderedTableStEph.rs
Chap43/OrderedTableStPer.rs
```

## What to do

For each file:

1. Find the trait declaration (e.g., `pub trait FooTrait<T: StT + Ord>`)
2. Add `+ TotalOrder` to the type parameter bound
3. Check if `TotalOrder` is imported — if not, add:
   `use crate::vstdplus::total_order::total_order::TotalOrder;`
4. Validate with `scripts/validate.sh isolate ChapNN`

Example:

```rust
// Before:
pub trait OrderStatSelectStEphTrait<T: StT + Ord>: Sized {

// After:
pub trait OrderStatSelectStEphTrait<T: StT + Ord + TotalOrder>: Sized {
```

The impl block should already have `T: ... + TotalOrder` (since the free fns
that use it are called from the impl). If the impl doesn't compile after the
trait change, add `+ TotalOrder` to the impl's bounds too.

## Caution

- Some traits have multiple type parameters. Only add TotalOrder to the
  parameter that the free fns use it on (usually the key/element type `T`,
  not value types `V`).
- Check that existing callers (Mt wrappers, Aug modules, test files) still
  compile. Adding a bound to a trait means all impl sites must satisfy it.
- If a caller doesn't have TotalOrder in scope, add the import there too.

## Validation

Run `scripts/validate.sh isolate ChapNN` after each chapter. Run full
`scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT change function logic or specs.
- Do NOT weaken ensures.
- All existing RTTs must pass.

## When done

RCP.
