# R104 Agent 1 — Fix `assert forall ... ==>` warnings

## Summary

Replaced `==>` with `implies` as the outermost connective in all `assert forall`
statements across the codebase. With `implies`, the antecedent is assumed in the
`by` block, which is almost always the intended semantics.

## Changes

16 edits across 4 files. All were Case 1 (direct `==>` → `implies`).

| # | Chap | File | Lines changed | Count |
|---|------|------|---------------|-------|
| 1 | 39 | BSTTreapStEph.rs | 1945, 1949, 1987, 1992, 2358, 2521, 2602 | 7 |
| 2 | 39 | BSTParaTreapMtEph.rs | 851, 887, 894, 954, 1368, 1543, 1638 | 7 |
| 3 | exp | f64_sort.rs | 67 | 1 |
| 4 | exp | f64_bits_sort.rs | 73 | 1 |

All instances were the same pattern:
```rust
// Before:
assert forall|s: T, o: T| #![trigger A, B]
    A && B ==> conclusion by { ... }

// After:
assert forall|s: T, o: T| #![trigger A, B]
    A && B implies conclusion by { ... }
```

## Not changed (correctly analyzed as false positives)

- **Chap47 hash tables** (LinProb, QuadProb, DoubleHash): `==>` appears inside nested
  `forall|j|` within the conclusion of an outer `assert forall ... implies`. These are
  spec-level `==>` inside a nested quantifier, not the outer connective.
- **Chap62/StarPartitionMtEph.rs:505**: `==>` inside parenthesized sub-expression in the
  conclusion. Outer connective is already `implies`.
- **Chap42/TableStPer.rs:1312**: `==>` inside a nested `forall` in the antecedent.
- **Chap52/AdjTableGraphStEph.rs:616, AdjTableGraphStPer.rs:574**: Already fixed (noted
  in prompt as pre-done).

## Verification

- Before: 5426 verified, 0 errors
- After: 5426 verified, 0 errors
- Zero warnings about `==>` in `assert forall`
