# Veracity Tool: Check Ghost Function Coverage for Higher-Order Functions

## Context

You are working in `~/projects/veracity`. Do not touch `~/projects/veracity-agent1`.

## Problem

APAS-VERUS's `using_closures_standard.rs` (Standard 8) requires that higher-order
functions taking closures have a `Ghost(spec_fn)` companion parameter so that
ensures can express results at spec level. Without the ghost companion, the
postcondition is limited to opaque `f.ensures(...)` which callers can't reason
through.

There is no tool to audit which higher-order functions have ghost companions and
which don't.

## What to build

A new veracity binary: `veracity-check-ghost-fn-coverage`.

### Usage

```bash
veracity-check-ghost-fn-coverage -c ~/projects/APAS-VERUS
veracity-check-ghost-fn-coverage -d ~/projects/APAS-VERUS/src/Chap43/
veracity-check-ghost-fn-coverage ~/projects/APAS-VERUS/src/Chap41/OrdKeyMap.rs
```

### Detection

For each exec function (inside `verus!`), check:

1. Does it take a closure parameter? Look for `F: Fn(...)`, `F: FnMut(...)`,
   or `F: FnOnce(...)` in the type parameters or where clauses.

2. If yes, does it ALSO take a `Ghost(spec_fn(...))` or `Ghost(spec_pred)` or
   similar ghost parameter whose type is `spec_fn(...) -> ...`?

3. Classify:
   - **covered**: has closure param AND ghost spec_fn companion
   - **uncovered**: has closure param but NO ghost spec_fn companion
   - **exempt**: closure param but function is `external_body` (can't add ghost)

### Output

Per-file summary:

```
src/Chap41/OrdKeyMap.rs:
  COVERED:   filter (Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>)
  UNCOVERED: map_values (F: Fn(&K, &V) -> V — no ghost companion)
  UNCOVERED: reduce (F: Fn(&V, &V) -> V — no ghost companion)
  COVERED:   tabulate (uses f.ensures directly — acceptable for single-arg)
```

### Summary table

At the end, print a summary:

```
Ghost Function Coverage
========================

| # | Chapter | File                    | Covered | Uncovered | Exempt | Total |
|---|---------|------------------------|---------|-----------|--------|-------|
| 1 | Chap41  | OrdKeyMap.rs            |       2 |         2 |      0 |     4 |
| 2 | Chap42  | TableStEph.rs           |       1 |         3 |      0 |     4 |
| ...

Total: 120 covered, 40 uncovered, 10 exempt out of 170 higher-order functions.
```

### Log file

Write output to `analyses/veracity-check-ghost-fn-coverage.log`.

### What counts as "covered"

A function is covered if ANY of these hold:
1. It has a `Ghost<spec_fn(...)>` parameter
2. It has a `Ghost<FnSpec(...)>` parameter  
3. Its ensures use `spec_pred(...)` or `spec_f(...)` referencing a ghost parameter
4. It's a single-argument closure AND the ensures use `f.ensures(...)` directly
   (acceptable per the standard for simple cases like tabulate)

A function is uncovered if it takes a closure AND:
1. No ghost companion parameter
2. The ensures either don't mention the closure's output at all, or only say
   things like `result.wf()` without relating result values to the closure

### Trait vs impl

Count at the trait level. If the trait method has a ghost parameter, count it
as covered even if we're looking at the impl. Don't double-count trait + impl
of the same method.

### Exclusions

- `external_body` functions — exempt (can't add ghost params)
- Functions in `src/experiments/` — skip
- Functions in `src/standards/` — skip (they're examples)
- Spec and proof functions — skip (ghost companions are for exec functions)
- Functions where the closure is only used for comparison (`Fn(&T, &T) -> Ordering`,
  `Fn(&T, &T) -> bool` for sorting/comparison) — exempt (comparison closures
  don't need ghost companions, the ordering semantics are structural)

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse function signatures
with proper syntax awareness. A string-hacking detector will flag and kill tools
that corrupt source syntax.
