# R46 Agent 3: Chap47 + Chap26 + warnings (9 holes + warnings)

## Assignment

Close remaining hash table holes and ETSP float holes. Also fix fn_missing_*
warnings across chapters.

## Baseline

69 holes total. 4396 verified. Your chapters: Chap47 (5), Chap26 (4).

## Target Holes

### Chap47 — Hash Tables (5 holes)

| # | Chap | File | Function | Line | Type | Notes |
|---|------|------|----------|------|------|-------|
| 1 | 47 | DoubleHashFlatHashTableStEph.rs | — | 359 | assume(false) | Table full unreachable |
| 2 | 47 | LinProbFlatHashTableStEph.rs | — | 355 | assume(false) | Table full unreachable |
| 3 | 47 | ParaHashTableStEph.rs | clone_elem | 99 | assume | Clone workaround |
| 4 | 47 | ParaHashTableStEph.rs | — | 477 | external_body | Check function |
| 5 | 47 | ParaHashTableStEph.rs | clone_elem | 95 | fn_missing_requires | Warning |

The assume(false) holes (#1, #2) guard against "table full" — unreachable with load
factor < 1. These need a proof that the load factor invariant guarantees an empty slot
exists. Check if the wf predicate includes a load factor bound.

The clone_elem assume (#3) is the standard Clone workaround — likely irreducible.

### Chap26 — ETSP (4 holes)

| # | Chap | File | Function | Line | Type | Notes |
|---|------|------|----------|------|------|-------|
| 1-4 | 26 | ETSPMtEph.rs | various | — | external_body/assume | Float distance axioms |

Read the file. These involve `FloatTotalOrder` and float distance computations.
Check `src/vstdplus/float.rs` for available float axioms. The ETSP holes may need
float arithmetic axioms (addition monotonicity) that don't exist yet — if so, report
what axioms would be needed.

### Warnings to fix

Fix fn_missing_requires/ensures warnings in your assigned chapters:
- ParaHashTableStEph.rs: `clone_elem` fn_missing_requires (line 95)
- ParaHashTableStEph.rs: `createTable` fn_missing_wf_ensures (line 589)
- ParaHashTableStEph.rs: `insert` fn_missing_wf_requires (line 640)
- ParaHashTableStEph.rs: `lookup` fn_missing_wf_requires (line 654)
- ParaHashTableStEph.rs: `delete` fn_missing_wf_requires (line 664)
- ParaHashTableStEph.rs: `metrics` fn_missing_wf_requires (line 677)

Add real requires/ensures — not `requires true` or tautologies.

## What NOT to do
- Do NOT add `#[cfg(not(verus_keep_ghost))]` to anything. Forbidden on fn/impl/type.
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses.
- Do NOT add `requires true` or tautological requires.

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes.
Run `scripts/holes.sh src/Chap47/ src/Chap26/`.
Write your report to `plans/agent3-round46-report.md`.
