# veracity-compare-par-mut Phase 3b: Fix ordering + requires/ensures presence check (Round 3)

## Context

Phases 1-3 are done. Phase 3 found traits, matched functions across variants,
recognized Eph/Per patterns. Two issues to fix, then one new comparison to add.

Do NOT re-run phases 1, 2, or 3. Build on the existing code.

## Fix 1: Reference variant ordering

The reference variant (the one others are compared against) must follow this priority:

    StPer > MtPer > StEph > MtEph

StPer is the clean functional specification. Eph is the mutable optimization.
Mt wraps St. Compare downward from the highest-priority variant present.

Currently the tool uses alphabetical first (usually StEph). Fix it to pick the
highest-priority variant as reference.

The Eph/Per recognition logic must also flip direction accordingly:
- Reference is Per: Eph variants may have `&mut self` instead of `&self → Self`
- Reference is Eph (when no Per exists): Per variants may lack mutation functions

## Fix 2: Eph/Per parameter count classification

When comparing a Per function against its Eph counterpart, parameter count
differences are often intentional:
- Per `insert(&self, k, v) -> Self` has 3 params
- Eph `insert(&mut self, k, v)` has 3 params but returns `()`
- Per `delete(&self, k) -> Self` has 2 params, returns Self
- Eph `delete(&mut self, k)` has 2 params, returns `()`

These should be `info:` (recognized design pattern), not `warning:`.

Only flag `warning:` when param counts differ AND the difference isn't explained
by the Eph/Per return-type shift.

## New: Requires/ensures presence comparison

Phase 3 already extracts `has_requires` and `has_ensures` per function. Now compare
them across matched function pairs.

For each matched function pair (same name, reference variant vs other variant):

| Reference | Other | Level | Message |
|-----------|-------|-------|---------|
| has ensures | no ensures | warning | `fn foo: reference has ensures but <variant> does not` |
| no ensures | has ensures | info | `fn foo: <variant> has ensures but reference does not` |
| has requires | no requires | warning | `fn foo: reference has requires but <variant> does not` |
| no requires | has requires | info | `fn foo: <variant> has requires but reference does not` |
| both have ensures | both have ensures | info | `fn foo: both have ensures` |
| neither | neither | (skip) | |

The high-value finding: Mt variant has ensures missing that the St variant has.
That's the most common real bug — someone wraps St in RwLock and forgets to
propagate the spec.

## Output format

Same emacs compilation-mode format as phases 1-3:

```
src/Chap18/ArraySeqStPer.rs:0: info: reference variant for group ArraySeq (Chap18)
src/Chap18/ArraySeqMtEph.rs:120: warning: fn insert: StPer has ensures but MtEph does not
src/Chap18/ArraySeqMtPer.rs:95: info: fn delete: both have ensures
src/Chap19/ArraySeqStEph.rs:0: info: reference variant for group ArraySeq (Chap19) — no Per, using StEph
```

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All parsing must be token-aware or AST-aware.
