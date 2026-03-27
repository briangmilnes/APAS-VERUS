# R90 Agent 4 — Fix RTT compile errors + EdgeSetGraphMtPer default, STEP 10

## Objective

Fix 4 RTT compile errors and the 1 remaining EdgeSetGraphMtPer hole.

## RTT Compile Errors

Run `scripts/rtt.sh` to see them. Known issues:

### 1. view_ord_consistent unresolved import

```
error[E0432]: unresolved import `crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent`
```

Some test file imports `view_ord_consistent` from BSTParaStEph, but it may have
been moved or renamed. Find the import, check where `view_ord_consistent` lives
now, fix the import.

### 2. f64 uninterp spec fns not visible in cargo

```
error[E0432]: unresolved imports `crate::vstdplus::float::float::f64_add_spec`,
  `f64_sub_spec`, `f64_mul_spec`, `f64_sqrt_spec`
```

These are `uninterp spec fn` in vstdplus/float.rs — they only exist under
`verus_keep_ghost`. Some test or source file imports them without a cfg guard.
Fix: add `#[cfg(verus_keep_ghost)]` to the import, or restructure the test.

### 3-4. CycleDetect undeclared types

```
error[E0433]: failed to resolve: use of undeclared type `CycleDetectStEph`
error[E0433]: failed to resolve: use of undeclared type `CycleDetectStPer`
```

Test code references CycleDetect types. These were uncommented in R88 but the
test imports may need updating. Find the test file and fix the import path.

## EdgeSetGraphMtPer default

The `default()` function has external_body because `std::Default` trait doesn't
allow `requires`. Options:
- If `empty()` now has the right preconditions baked into type bounds, `default`
  may just work
- If not, this may need to stay external_body — document why

## Isolation

For RTT: `scripts/rtt.sh`
For EdgeSetGraph: `scripts/validate.sh isolate Chap52`

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT run full validate — only rtt.sh and isolate Chap52.
- Do NOT modify vstdplus/float.rs.
- Do NOT add assume or accept.
- The RTT fixes are mechanical — find the broken import, fix the path or add cfg guard.

## STEP 10

## Report

Write `plans/agent4-r90-rtt-report.md`.
