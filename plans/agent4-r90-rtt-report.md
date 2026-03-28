# Agent 4 — R90 RTT Compile Fixes Report

## Summary

Fixed all 4 RTT compile errors. EdgeSetGraphMtPer `default()` external_body is structural and must remain.

## RTT Compile Fixes

| # | Chap | File | Error | Fix |
|---|------|------|-------|-----|
| 1 | 52 | AdjTableGraphStEph.rs | `view_ord_consistent` unresolved import | Added `#[cfg(verus_keep_ghost)]` — it's a spec fn, invisible to cargo |
| 2 | 26 | ETSPMtEph.rs | `f64_add_spec` etc. unresolved | Split import: exec fns unconditional, `_spec` fns under `#[cfg(verus_keep_ghost)]` |
| 3 | 55 | TopoSortStEph.rs | `CycleDetectStEph` undeclared | Removed `#[cfg(verus_keep_ghost)]` from import — struct/trait used in exec code |
| 4 | 55 | TopoSortStPer.rs | `CycleDetectStPer` undeclared | Split import: struct/trait unconditional, `spec_cycledetectstper_wf` under cfg guard |

## RTT Result

3076 tests run: 3076 passed, 0 skipped.

## EdgeSetGraphMtPer `default()` Analysis

The `external_body` on `default()` at `EdgeSetGraphMtPer.rs:355` is **structural** and cannot be removed:

- `default()` calls `Self::empty()`, which `requires` ordering axioms (`obeys_cmp_spec`, `view_ord_consistent`) for both `V` and `Pair<V,V>`.
- These requires exist because `spec_edgesetgraphmtper_wf` includes the ordering axioms as conjuncts — `empty()` must prove wf holds, which needs the axioms assumed.
- `std::Default::default()` is an external trait that cannot have `requires` in Verus.
- The `external_body` bridges this gap: the real body (`Self::empty()`) is sound, but the preconditions can't be expressed on the trait method.

This is the same pattern as `AdjTableGraphMtPer.rs:269` which also has `external_body` on `default()`.

## Verification

| Validation | Result |
|---|---|
| RTT | 3076 passed, 0 errors |
| Chap52 isolate | 2766 verified, 0 errors |
| Chap55 isolate | 2147 verified, 0 errors |
| Chap26 isolate | 1065 verified, 0 errors |

## Holes

No hole changes. EdgeSetGraphMtPer remains at 1 hole (`default()` external_body — structural).
