# R117 Agent 4 — Strengthen Chap06 graph MtEph specs. AFK. DOT.

## Problem

`veracity-compare-par-mut` reports 32 warnings on Chap06. Four graph MtEph
files are missing wf specs and ensures clauses that StEph has. The pattern
is highly repetitive across all four graph types.

## Files (4 MtEph files, 4 StEph references)

| # | MtEph file | StEph file | Warnings |
|---|-----------|-----------|----------|
| 1 | DirGraphMtEph.rs | DirGraphStEph.rs | 5 |
| 2 | LabDirGraphMtEph.rs | LabDirGraphStEph.rs | 11 |
| 3 | LabUnDirGraphMtEph.rs | LabUnDirGraphStEph.rs | 10 |
| 4 | UnDirGraphMtEph.rs | UnDirGraphStEph.rs | 6 |

## Warning categories

### Missing spec functions (4 warnings, 1 per file)

Each MtEph is missing its StEph's wf and structural spec functions:

- **DirGraphMtEph** (line 95): missing `spec_dirgraphsteph_wf`
  → Add `spec_dirgraphmteph_wf` with equivalent invariant.

- **LabDirGraphMtEph** (line 93): missing `spec_labdirgraphsteph_wf`,
  `spec_finite`, `spec_arcs`
  → Add MtEph-named wf plus the spec fns.

- **LabUnDirGraphMtEph** (line 93): missing `spec_labundirgraphsteph_wf`,
  `spec_finite`, `spec_edges`
  → Add MtEph-named wf plus the spec fns.

- **UnDirGraphMtEph** (line 94): missing `spec_undirgraphsteph_wf`,
  `spec_degree`
  → Add MtEph-named wf plus `spec_degree`.

### Missing ensures on constructors (all files)

`empty` and `from_sets`/`from_vertices_and_labeled_*` have fewer ensures in
MtEph than StEph. The missing ensures are typically:
- `g@.V =~= vertices@` (vertices match input)
- `g@.A =~= arcs@` / `g@.A =~= edges@` (arcs/edges match input)
- wf ensures on the constructed graph

These are structural postconditions the RwLock wrapper should preserve.

### Missing ensures on accessors

- **DirGraphMtEph `arcs`**: missing `a@ =~= self@.A`
- **LabDirGraphMtEph `labeled_arcs`**: missing `a@ =~= self@.A`
- **LabDirGraphMtEph `arcs`**: missing `arcs@.finite()`
- **LabDirGraphMtEph `get_arc_label`**: missing label containment ensures
- **LabDirGraphMtEph `n_plus`/`n_minus`**: missing `finite()` ensures
- **LabUnDirGraphMtEph `labeled_edges`**: missing `e@ =~= self@.A`
- **LabUnDirGraphMtEph `get_edge_label`**: missing label containment ensures
- **UnDirGraphMtEph `edges`**: missing `e@ =~= self@.A`

## Strategy

All four files follow the same RwLock wrapper pattern. Fix one file
completely (start with DirGraphMtEph — fewest warnings), then replicate
the pattern to the other three.

1. Read all 8 files (4 StEph + 4 MtEph).
2. Fix DirGraphMtEph: add wf spec, strengthen ensures.
3. Validate Chap06: `scripts/validate.sh isolate Chap06`.
4. Fix UnDirGraphMtEph (same pattern, undirected).
5. Fix LabDirGraphMtEph (labeled variant, more ensures).
6. Fix LabUnDirGraphMtEph (labeled undirected).
7. Validate after each file or pair.
8. RTTs: `scripts/rtt.sh Chap06`.

## Rules

- Do NOT weaken any ensures.
- Do NOT add assume or accept in algorithmic code.
- Mt standalone: do NOT import from StEph. Copy spec fn bodies.
- The wf predicate names must follow convention: `spec_dirgraphmteph_wf`,
  `spec_labdirgraphmteph_wf`, etc. Not the StEph name.
- No subagents.

## STEP 30

## Report

Write `plans/agent4-r117-chap06-graphs-report.md`. Include before/after
warning count per file.
