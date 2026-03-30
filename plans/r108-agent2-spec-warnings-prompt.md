# R108 Agent 2 — Fix veracity spec warnings. AFK.

## Objective

Fix 19 veracity spec warnings across 5 chapters. These are missing `requires`
and `ensures` clauses — real spec gaps, not style nits. Every fix strengthens
the verification contract.

Skip Chap65 (Agent 1 is working there) and BSTSplayMtEph (known SMT budget issue).

## The warnings

### Chap06 — 8 missing wf ensures (graph constructors)

All 4 graph StEph files are missing wf ensures on `empty` and `from_sets`/`from_vertices_*`:

| # | File | Line | Function | Fix |
|---|------|------|----------|-----|
| 1 | DirGraphStEph.rs | 116 | empty | add `ensures g.spec_dirgraphsteph_wf()` |
| 2 | DirGraphStEph.rs | 125 | from_sets | add `ensures g.spec_dirgraphsteph_wf()` |
| 3 | LabDirGraphStEph.rs | 72 | empty | add `ensures g.spec_labdirgraphsteph_wf()` |
| 4 | LabDirGraphStEph.rs | 81 | from_vertices_and_labeled_arcs | add `ensures g.spec_labdirgraphsteph_wf()` |
| 5 | LabUnDirGraphStEph.rs | 68 | empty | add `ensures g.spec_labundirgraphsteph_wf()` |
| 6 | LabUnDirGraphStEph.rs | 77 | from_vertices_and_labeled_edges | add `ensures g.spec_labundirgraphsteph_wf()` |
| 7 | UnDirGraphStEph.rs | 99 | empty | add `ensures g.spec_undirgraphsteph_wf()` |
| 8 | UnDirGraphStEph.rs | 108 | from_sets | add `ensures g.spec_undirgraphsteph_wf()` |

These should be straightforward — constructors that build well-formed structures
should say so. Read the wf predicate, confirm the constructor satisfies it, add
the ensures. `scripts/validate.sh isolate Chap06`

### Chap26 — 1 missing requires

| # | File | Line | Function | Fix |
|---|------|------|----------|-----|
| 9 | ETSPMtEph.rs | 629 | point_distance | add real requires (not `requires true`) |

Read the function body. What does it assume about its inputs? Express that.
`scripts/validate.sh isolate Chap26`

### Chap43 — 1 missing wf ensures

| # | File | Line | Function | Fix |
|---|------|------|----------|-----|
| 10 | OrderedTableStPer.rs | 3469 | from_sorted_entries | add `ensures result.spec_orderedtablestper_wf()` |

`scripts/validate.sh isolate Chap43`

### Chap44 — 2 warnings

| # | File | Line | Function | Fix |
|---|------|------|----------|-----|
| 11 | DocumentIndex.rs | 438 | new | add `requires index.spec_documentindex_wf()` |
| 12 | DocumentIndex.rs | 557 | tokens | add real requires |

`scripts/validate.sh isolate Chap44`

### Chap47 — 7 warnings (ParaHashTable)

| # | File | Line | Function | Fix |
|---|------|------|----------|-----|
| 13 | ParaHashTableStEph.rs | 424 | createTable | add `ensures table.spec_hashtable_wf()` |
| 14 | ParaHashTableStEph.rs | 476 | insert | add `requires table.spec_hashtable_wf()` |
| 15 | ParaHashTableStEph.rs | 495 | lookup | add `requires table.spec_hashtable_wf()` |
| 16 | ParaHashTableStEph.rs | 507 | delete | add `requires table.spec_hashtable_wf()` |
| 17 | ParaHashTableStEph.rs | 522 | metrics | add `requires table.spec_hashtable_wf()` |
| 18 | ParaHashTableStEph.rs | 531 | loadAndSize | add `requires table.spec_hashtable_wf()` |
| 19 | ParaHashTableStEph.rs | 548 | resize | add both `requires` and `ensures` wf |

`scripts/validate.sh isolate Chap47`

## Work order

1. Chap06 (8 fixes, all similar pattern — batch them)
2. Chap47 (7 fixes, all in one file)
3. Chap43 (1 fix)
4. Chap44 (2 fixes)
5. Chap26 (1 fix)

Commit after each chapter is clean.

## Rules

- Read each file before editing. Understand the wf predicate before adding it.
- Do NOT add `requires true` or tautological requires. Add the REAL precondition.
- Do NOT add `// veracity: no_requires`. Only the user does that.
- Do NOT add assume or accept.
- Do NOT touch Chap65 (Agent 1).
- Do NOT touch BSTSplayMtEph (known SMT budget issue — `// veracity: no_requires`).
- When adding `requires wf` to a function, check callers satisfy it. If a caller
  doesn't already prove wf, you may need to propagate ensures upstream.
- When adding `ensures wf`, you must prove it — Verus must verify the ensures.
  If the proof needs intermediate assertions, add them.
- Use `scripts/validate.sh isolate ChapNN` for iteration.
- Full validate after all chapters done.
- Read logs instead of re-running.

## No subagents. AFK.

## Report

Write `plans/agent2-r108-spec-warnings-report.md`.
