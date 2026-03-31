# R117 Agent 4 — Chap06 Graph MtEph Spec Strengthening Report

## Summary

Strengthened specs on 4 Chap06 MtEph graph files to match their StEph counterparts.
All changes are additive — no ensures were weakened, no assumes or accepts added.

## Changes per file

| # | Chap | File | Warnings Before | Warnings After | Changes |
|---|------|------|-----------------|----------------|---------|
| 1 | 06 | DirGraphMtEph.rs | 5 | 0 | Added `spec_dirgraphmteph_wf`, wf ensures on `empty`/`from_sets`, `arcs` ensures `=~=` |
| 2 | 06 | UnDirGraphMtEph.rs | 6 | 0 | Added `spec_undirgraphmteph_wf`, `spec_degree`, wf ensures on `empty`/`from_sets`, `edges` ensures `=~=` |
| 3 | 06 | LabDirGraphMtEph.rs | 11 | 0 | Added `spec_labdirgraphmteph_wf`, `spec_finite`, `spec_arcs`, wf ensures on constructors, `labeled_arcs`/`arcs` ensures strengthened, `n_plus`/`n_minus` `finite()` ensures, `get_arc_label` `is_some`-style ensures |
| 4 | 06 | LabUnDirGraphMtEph.rs | 10 | 0 | Added `spec_labundirgraphmteph_wf`, `spec_finite`, `spec_edges`, wf ensures on constructors, `labeled_edges` ensures `=~=`, `get_edge_label` `is_some`-style ensures |

## Verification

- `scripts/validate.sh isolate Chap06`: 1067 verified, 0 errors
- `scripts/rtt.sh graph`: 171 tests passed

## Technique

Each MtEph file follows the same RwLock wrapper pattern. The spec functions were
copied from the StEph counterpart with names adjusted to the MtEph convention
(`spec_dirgraphmteph_wf` not `spec_dirgraphsteph_wf`). Ensures clauses were
strengthened to match StEph by adding wf postconditions, extensional equality
(`=~=`), finiteness, and `is_some`-style label ensures.
