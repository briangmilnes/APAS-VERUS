# R99 Agent 3 — Add wf ensures to Chap06 graph constructors, STEP 10

## Objective

8 veracity warnings report `fn_missing_wf_ensures` on graph constructor
functions in Chap06. Add `ensures result.spec_*_wf()` to each.

## The 8 warnings

| # | File | Function | Missing ensures |
|---|------|----------|----------------|
| 1 | DirGraphStEph.rs | empty | spec_dirgraphsteph_wf |
| 2 | DirGraphStEph.rs | from_sets | spec_dirgraphsteph_wf |
| 3 | UnDirGraphStEph.rs | empty | spec_undirgraphsteph_wf |
| 4 | UnDirGraphStEph.rs | from_sets | spec_undirgraphsteph_wf |
| 5 | LabDirGraphStEph.rs | empty | spec_labdirgraphsteph_wf |
| 6 | LabDirGraphStEph.rs | from_vertices_and_labeled_arcs | spec_labdirgraphsteph_wf |
| 7 | LabUnDirGraphStEph.rs | empty | spec_labundirgraphsteph_wf |
| 8 | LabUnDirGraphStEph.rs | from_vertices_and_labeled_edges | spec_labundirgraphsteph_wf |

## What to do

For each function, add the wf ensures to the trait declaration AND the impl.
The wf predicates were added by agent4 R94 — they call `spec_graphview_wf(self@)`.

For `empty()`: the wf should hold trivially (empty graph has vacuous closure).
For `from_sets`/`from_vertices_and_*`: the wf may need a requires on the input
sets (e.g., every arc endpoint is in the vertex set). Check the MtEph version
for what requires/ensures pattern to follow.

## Isolation

```bash
scripts/validate.sh isolate Chap06
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## STEP 10

## Report

Write `plans/agent3-r99-chap06-wf-ensures-report.md`.
