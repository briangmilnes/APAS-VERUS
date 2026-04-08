# R159 Agent 1: OrdKeyMap Z3 Diet — Report

## Summary

**Goal**: Reduce `Chap41__OrdKeyMap__OrdKeyMap` Z3 quantifier instantiations below the
full-validate baseline (~3.37M stated in prompt; isolate-mode baseline measured at 3,050,645).

**Outcome**: All four tasks failed or produced worse results. Final state is committed with
the Task 1 changes on disk (verifies, RTTs pass, but instantiations increased 38%).

---

## Baseline

| Metric | Value |
|--------|-------|
| Isolate-mode instantiations (original) | 3,050,645 |
| Top quantifier | `lemma_reveal_view_injective_244` |
| Verified count | 2280 |
| RTTs | 3776 passing |

---

## Task Results

### Task 1 — Reduce `lemma_reveal_view_injective::<K>()` outer calls (7 → 2-3)

**Approach taken**: Commented out 6 of 7 outer calls (kept line 1166 for postcondition proof
in `ordkeymap_split`). Added targeted `lemma_reveal_view_injective::<K>()` calls at the TOP
of each `assert forall ... by { }` block that failed without the outer call.

**Result**: 25+ targeted calls placed across `ordkeymap_next`, `ordkeymap_prev`,
`ordkeymap_rank`, `ordkeymap_select`, `first_key`, `last_key`. Verification passes. RTTs pass.

**Instantiation count**: **4,211,307** — 38% WORSE than baseline.

**Root cause**: Each targeted call was placed at the top of a large `assert forall ... by { }`
block (20-40 K-typed terms in scope). With 25+ blocks vs. 7 function-level calls, Z3 fires the
injectivity quantifier far more times. The correct approach would be `assert(x == y) by {
lemma_reveal_view_injective::<K>(); }` with exactly 2 terms in scope per call, but this
requires identifying each specific equality assertion that needs injectivity and wrapping it
individually — a much more invasive refactor not completed in this round.

### Task 2 — Swap `group_set_axioms` → `group_set_axioms_early`

**Result**: **Compile error.** `vstd::set::group_set_axioms_early` does not exist in the
currently pinned vstd version. Reverted immediately.

### Task 3 — Remove `group_feq_axioms` from module-level broadcast

**Result**: **Fails verification.** Removing `group_feq_axioms` from the module-level
`broadcast use` block broke 6+ iterator loop invariants (lines 3072, 3428, 3587, 3977, 4009,
4068, 4162 — "invariant not satisfied before loop"). Root cause: `broadcast use` cannot be
scoped inside function bodies in Verus — it must be module-level. Without the axioms globally
available, iterator-related proofs collapse. Reverted immediately.

### Task 4 — Profile before/after

| State | Instantiations |
|-------|---------------|
| Baseline (7 outer calls, original) | 3,050,645 |
| After Task 1 changes (25+ targeted calls) | 4,211,307 |

Both measured with `scripts/profile.sh isolate Chap41`.

---

## What Would Actually Work

To genuinely reduce instantiations in `ordkeymap_next`, `ordkeymap_prev`, `ordkeymap_rank`,
`ordkeymap_select`:

1. **Tighter `by` blocks**: Find each `assert(root_pair.0 == *k)` or `assert(p.0 == root_pair.0)`
   inside `assert forall` bodies and wrap them as `assert(x == y) by { lemma_reveal_view_injective::<K>(); }`.
   This limits Z3's injectivity forall to exactly 2 K-typed terms. Estimate: 2-5 calls per function
   instead of 5-8 large-block calls per function.

2. **Direct `reveal` instead of lemma call**: Replace `lemma_reveal_view_injective::<K>()` with
   `reveal(obeys_feq_view_injective)` inside the same tight `by` blocks. Same Z3 effect but
   avoids the lemma's `ensures` forall being added to the global context.

3. **Eliminate `group_feq_axioms` module-level broadcast**: Requires first finding a way to
   scope or factor feq axioms so they're not needed globally. Not straightforward with current
   Verus broadcast_use scoping rules.

---

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 41 | `OrdKeyMap.rs` | 6 outer calls commented out, 25+ targeted calls added to forall blocks |

Verification: 2280 verified, 0 errors.
RTTs: 3776 passing.
