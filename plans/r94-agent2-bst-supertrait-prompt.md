# R94 Agent 2 — Fix 4 BST supertrait View discrepancies (Chap37), STEP 10

## Objective

4 BST StEph traits are missing `View` as a supertrait, but their MtEph
counterparts have it. Add the View supertrait to match.

## The 4 Errors (from veracity-compare-par-mut)

| # | File | StEph Supertrait | MtEph Supertrait |
|---|------|-----------------|-----------------|
| 1 | BSTAVLStEph.rs | `Sized` | `Sized + View<V = BalBinTree<T>>` |
| 2 | BSTBBAlphaStEph.rs | `Sized` | `Sized + View<V = BalBinTree<T>>` |
| 3 | BSTPlainStEph.rs | `Sized` | `Sized + View<V = BalBinTree<T>>` |
| 4 | BSTRBStEph.rs | `Sized` | `Sized + View<V = Link<T>>` |

## What to do

For each file, find the trait declaration and add the View supertrait:

```rust
// Before:
pub trait BSTAVLStEphTrait<T: ...>: Sized {

// After:
pub trait BSTAVLStEphTrait<T: ...>: Sized + View<V = BalBinTree<T>> {
```

For BSTRB, use `Link<T>` instead of `BalBinTree<T>` (matching MtEph).

Check that the struct already has `impl View` — it should. The trait just
needs to declare it as a bound so callers can use `self@` through the trait.

## Verification

After each change, verify the trait's callers still compile. Adding a supertrait
is usually non-breaking (the impl already satisfies View), but callers that use
the trait as a bound may need the View import.

## Isolation

```bash
scripts/validate.sh isolate Chap37
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Only change the trait supertrait declaration.
- Do NOT change View impls, struct definitions, or function bodies.
- Do NOT modify the MtEph files.
- If adding the supertrait causes unexpected errors, report them and revert.

## STEP 10

## Report

Write `plans/agent2-r94-bst-supertrait-report.md`.
