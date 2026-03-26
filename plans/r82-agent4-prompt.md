# R82 Agent 4 — Rewrite Chap43 OrderedSetStPer for ParamBST, STEP 20

## Objective

Fix `src/Chap43/OrderedSetStPer.rs` (1650 lines) so it compiles and verifies.
Then fix `src/Chap43/Example43_1.rs` (231 lines) which depends on it.

## lib.rs Memory Isolation Protocol

Before your first validate, comment out all chapters AFTER Chap43 to save memory.
Use EXACTLY this format:

```
/* R82-ISOLATED: agent 4, working on Chap43
#[cfg(all(not(feature = "experiments_only"), not(feature = "union_find")))]
pub mod Chap44 {
...
R82-ISOLATED */
```

Wrap lines 477-670 (Chap44 through Chap66 + closing) in this block.
Do NOT touch anything before Chap43. Do NOT touch Chap05 or Chap06.

**Before pushing to agent4/ready, REMOVE the isolation wrapper.** Restore lib.rs
to match main except for your Chap43 fixes. Verify with:
`git diff origin/main -- src/lib.rs` — only Chap43 lines should differ.

## What to fix

### 1. Uncomment OrderedSetStPer and Example43_1

```rust
pub mod Chap43 {
    ...existing active files...
    pub mod OrderedSetStPer;
    ...
    pub mod Example43_1;
}
```

### 2. Rewrite OrderedSetStPer for ParamBST backing

`AVLTreeSetStPer<T>` was refactored. The old struct had:
```rust
pub struct AVLTreeSetStPer<T> {
    pub elements: AVLTreeSeqStPer<T>,  // OLD — removed
}
```

The new struct has:
```rust
pub struct AVLTreeSetStPer<T: StT + Ord> {
    pub tree: ParamBST<T>,  // NEW
}
```

OrderedSetStPer wraps AVLTreeSetStPer and accesses `.base_set.elements` ~75 times.
Every such access must be rewritten to use the `AVLTreeSetStPerTrait` API.

**Read these files first:**
- `src/Chap41/AVLTreeSetStPer.rs` — the current trait and impl. Understand what
  methods exist: `size()`, `find()`, `insert()`, `delete()`, `in_order()`,
  `union()`, `intersection()`, `difference()`, `iter()`, `to_seq()`, `from_seq()`.
- `src/Chap43/OrderedSetStEph.rs` — the working StEph version. Use this as a
  reference for what the StPer version should look like. It was already rewritten
  for the ParamBST API.

**Key mappings:**
- `self.base_set.elements.length()` → `self.base_set.size()`
- `self.base_set.elements.nth(i)` → use `in_order()` or iterator
- `self.base_set.elements.root` → no direct access; use trait methods
- `self.base_set.elements@` → `self.base_set@` (View is now Set, not Seq)
- `spec_inorder_values_per(self.base_set.elements.root)` → `self.base_set.in_order()@`

**The View changed.** `AVLTreeSetStPer`'s View is now `Set<T::V>` (a mathematical set),
not `Seq<T>` (an ordered sequence). Specs that relied on sequential ordering of
elements need to use `in_order()` which returns a sorted Vec.

### 3. Fix Example43_1

Depends on OrderedSetStPer's exports. Fix import errors after StPer is working.

## Important

- Read `src/Chap43/OrderedSetStEph.rs` as your primary reference — it's the
  working version of the same ADT.
- Do NOT add `assume` or `accept`.
- Do NOT weaken ensures clauses.
- If the full rewrite exceeds 20 steps, get as far as you can and report what
  remains. Comment out functions that don't verify yet with BYPASSED comments.

## STEP 20

At most 20 edit/verify iterations. Then stop and report.

## Validation

Run `scripts/validate.sh` (with isolation), then before pushing restore lib.rs
and run full `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh`.
Push to `agent4/ready`.

## Report

Write `plans/agent4-round82-report.md` with functions fixed, errors before/after,
verified count, and what remains if incomplete.
