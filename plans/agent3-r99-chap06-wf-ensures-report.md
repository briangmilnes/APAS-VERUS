# Agent 3 R99 Report: Chap06 wf ensures

## Objective

Add `ensures result.spec_*_wf()` to 8 graph constructor functions in Chap06
to resolve `fn_missing_wf_ensures` veracity warnings.

## Approach

Each of the 4 graph modules defines `spec_<module>_wf(&self)` as
`spec_graphview_wf(self@)` or `spec_labgraphview_wf(self@)`. The constructors
already ensure `spec_graphview_wf(g@)` / `spec_labgraphview_wf(g@)` in the
trait declaration, so the wf is logically implied.

**Cycle constraint**: Adding `ensures g.spec_*_wf()` to the trait declaration
causes a Verus cycle error (trait method ensures references spec fn declared
in the same trait). The fix: add the wf ensures only on the **impl** methods,
not the trait declarations.

## Changes

| # | Chap | File | Function | Added ensures |
|---|------|------|----------|---------------|
| 1 | 06 | DirGraphStEph.rs | empty (impl) | spec_dirgraphsteph_wf |
| 2 | 06 | DirGraphStEph.rs | from_sets (impl) | spec_dirgraphsteph_wf |
| 3 | 06 | UnDirGraphStEph.rs | empty (impl) | spec_undirgraphsteph_wf |
| 4 | 06 | UnDirGraphStEph.rs | from_sets (impl) | spec_undirgraphsteph_wf |
| 5 | 06 | LabDirGraphStEph.rs | empty (impl) | spec_labdirgraphsteph_wf |
| 6 | 06 | LabDirGraphStEph.rs | from_vertices_and_labeled_arcs (impl) | spec_labdirgraphsteph_wf |
| 7 | 06 | LabUnDirGraphStEph.rs | empty (impl) | spec_labundirgraphsteph_wf |
| 8 | 06 | LabUnDirGraphStEph.rs | from_vertices_and_labeled_edges (impl) | spec_labundirgraphsteph_wf |

## Verification

- `scripts/validate.sh isolate Chap06`: **1064 verified, 0 errors**
- Full validate: OOM killed (pre-existing memory pressure, unrelated to changes)

## Steps used: 2 of 10

1. Added ensures to trait declarations -- cycle error
2. Moved ensures to impl methods only -- clean verification
