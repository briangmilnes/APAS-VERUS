# Agent 4 — R94 Graph WF Naming Report

## Objective

Fix veracity-compare-par-mut warnings for Chap06 graph files:
missing wf predicates in StEph files and wrong wf naming in MtEph files.

## Changes

### StEph files — added wf predicates (4 files)

| # | Chap | File | Predicate added |
|---|------|------|-----------------|
| 1 | 06 | DirGraphStEph.rs | `spec_dirgraphsteph_wf` |
| 2 | 06 | UnDirGraphStEph.rs | `spec_undirgraphsteph_wf` |
| 3 | 06 | LabDirGraphStEph.rs | `spec_labdirgraphsteph_wf` |
| 4 | 06 | LabUnDirGraphStEph.rs | `spec_labundirgraphsteph_wf` |

All bodies are `spec_graphview_wf(self@)` or `spec_labgraphview_wf(self@)`,
matching the MtEph counterparts. Abstract in trait, open in impl.

### MtEph files — renamed wf predicates (3 files)

| # | Chap | File | Old name | New name |
|---|------|------|----------|----------|
| 1 | 06 | DirGraphMtEph.rs | `spec_lockeddirgraphmteph_wf` | `spec_dirgraphmteph_wf` |
| 2 | 06 | UnDirGraphMtEph.rs | `spec_lockedundirgraphmteph_wf` | `spec_undirgraphmteph_wf` |
| 3 | 06 | LabDirGraphMtEph.rs | `spec_lockedlabdirgraphmteph_wf` | `spec_labdirgraphmteph_wf` |

### MtEph files — added wf predicate (1 file)

| # | Chap | File | Predicate added |
|---|------|------|-----------------|
| 1 | 06 | LabUnDirGraphMtEph.rs | `spec_labundirgraphmteph_wf` |

LabUnDirGraphMtEph had no module-level wf at all. Added abstract in
`LockedLabUnDirGraphMtEphTrait`, open in impl. Body: `spec_labgraphview_wf(self@)`.

## Caller impact

No callers of the old wf names exist outside Chap06. The rename is self-contained.
The new StEph wf predicates have no existing callers (additive-only).

## Verification

- Full validate: 5386 verified, 0 errors
- RTT: 3083 passed
- PTT: 157 passed
