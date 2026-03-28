# R94 Agent 4 — Add wf predicates + fix wf naming in Chap06 graphs, STEP 10

## Objective

Fix 5 warnings from veracity-compare-par-mut in Chap06 graph files:
- 2 StEph files missing wf predicates entirely
- 2 MtEph files with wrong wf naming pattern

## The Warnings

| # | File | Warning |
|---|------|---------|
| 1 | DirGraphStEph.rs | no `spec_*_wf` predicate found |
| 2 | LabDirGraphStEph.rs | no `spec_*_wf` predicate found |
| 3 | DirGraphMtEph.rs | wf name `spec_lockeddirgraphmteph_wf` should be `spec_dirgraphmteph_wf` |
| 4 | LabDirGraphMtEph.rs | wf name `spec_lockedlabdirgraphmteph_wf` should be `spec_labdirgraphmteph_wf` |

Also fix the similar issues for UnDirGraph if they exist.

## What to do

### Add wf to StEph files (#1, #2)

Add `spec fn spec_dirgraphsteph_wf(&self) -> bool` to the DirGraphStEph trait.
The wf should match the MtEph version's wf content — likely `spec_graphview_wf(self@)`.

Same for LabDirGraphStEph → `spec_labdirgraphsteph_wf`.

Check UnDirGraphStEph and LabUnDirGraphStEph too — they may have the same gap.

### Rename MtEph wf (#3, #4)

Rename `spec_lockeddirgraphmteph_wf` → `spec_dirgraphmteph_wf` in DirGraphMtEph.
Rename `spec_lockedlabdirgraphmteph_wf` → `spec_labdirgraphmteph_wf` in LabDirGraphMtEph.

Update all callers of the old name. Use replace-all.

## Read first

- `src/Chap06/DirGraphMtEph.rs` — reference for wf content
- `src/Chap06/DirGraphStEph.rs` — add wf here
- `src/standards/spec_wf_standard.rs` — naming convention

## Isolation

```bash
scripts/validate.sh isolate Chap06
```

Then check callers:
```bash
scripts/validate.sh isolate Chap55
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Renaming wf means updating ALL callers — grep thoroughly.
- Adding wf to StEph is non-breaking (new function, no existing callers yet).
- Do NOT modify the View impls — that's agent3's job.

## STEP 10

## Report

Write `plans/agent4-r94-graph-wf-report.md`.
