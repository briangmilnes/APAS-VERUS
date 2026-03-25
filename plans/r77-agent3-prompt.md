# R77 Agent 3 — Chap37 BSTSet residual holes: constructor_feq_standard propagation (15 holes)

## Objective

Apply the `constructor_feq_standard` pattern to eliminate remaining `obeys_feq_clone`
assumes across BSTSetAVLMtEph (5), BSTSetRBMtEph (5), and BSTSetSplayMtEph (5).

## Baseline

- 4869 verified, 0 errors, 0 warnings
- BSTSetAVLMtEph: 5 holes (5 external_body)
- BSTSetRBMtEph: 5 holes (5 external_body)
- BSTSetSplayMtEph: 5 holes (5 external_body)

## Context

The `empty()` and `singleton()` constructors now `requires obeys_feq_clone::<T>()`.
The wf predicates include `obeys_feq_clone::<T>()`. The remaining 15 holes are:

Per file (identical pattern):
- 3 × `external_body` on `union`, `intersection`, `difference` (ParaPair! recursive closures)
- 2 × `external_body` on `filter`, `reduce` (FnMut closure requires)

### ParaPair! functions (union, intersection, difference)

These use `ParaPair!` macro for recursive parallel calls. The `external_body` exists
because the function bodies use `crate::ParaPair!` which expands to thread spawning.

**Strategy**: Check if these can be narrowed. The algorithmic logic (split, recursive
call, join) is verifiable. Only the thread spawn/join is not. Can the function body be
restructured to have a verified algorithmic core with a thin `external_body` wrapper
around just the `ParaPair!` call?

Read `src/standards/using_closures_standard.rs` for the closure verification pattern.
Read how `ParaPair!` is defined and what it expands to.

### filter/reduce

These have FnMut closures whose `requires` can't be proved in Verus. The loop bodies
inside are already verified. These are likely permanent until Verus improves FnMut support.

## Alternative approach

If the ParaPair functions can't be narrowed: focus instead on the `accept()` in
Iterator::next (clone-preserves-value) — is there a way to prove this with the
`strictly_cloned` broadcasts agent4 added to feq.rs?

## Key resources

- `src/Chap37/BSTSetAVLMtEph.rs` — Read fully (reference for RB/Splay)
- `src/standards/constructor_feq_standard.rs` — The new standard
- `src/standards/using_closures_standard.rs` — Closure patterns
- `src/Types/Types.rs` — `ParaPair!` macro definition

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent3/ready`.

## Report

Write `plans/agent3-round77-report.md` with holes before/after (table with Chap column).
