# R41 Agent 4: Chap47 Hash Table Cleanup + Chap43 OrderedSet Warnings

## Baseline
- Main at `29641a5e`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4281 verified, 192 holes, 30 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**

Read CLAUDE.md and `src/standards/mod_standard.rs` before starting.

## Context

Chap47 has 14 holes and 19 warnings across 7 holed files. Many holes are clone bridge
assumes (`assume(c == *x)` in `clone_elem` functions) which are the standard irreducible
pattern. But there are also actionable targets: `assume(false)` missing `diverge()`,
wf bridge assumes in DoubleHash, and the StructChainedHashTable resize external_body.

Chap43 has fn_missing_requires warnings in OrderedSetStEph.rs and OrderedSetStPer.rs
(`from_sorted_elements`) that were assigned in R40 but not completed.

## Assignment

### Part A: Chap47 — Fix assume(false) + Add diverge() (3 holes)

Three files have `assume(false)` in unreachable "table full" branches:

| # | File | Line | Notes |
|---|------|------|-------|
| 1 | DoubleHashFlatHashTableStEph.rs | 382 | Insert — table full |
| 2 | LinProbFlatHashTableStEph.rs | 364 | Insert — table full |
| 3 | QuadProbFlatHashTableStEph.rs | 386 | Insert — table full |

These already have `assume(false)` but are missing `diverge()`. Add `diverge()` after
each `assume(false)` to match the standard pattern. This closes 3 holes.

Wait — actually check if `diverge()` is already there. The veracity output says "needs
diverge()" which suggests it's missing. Read each line and add if needed.

### Part B: Chap47 — Fix fn_missing_requires on clone_elem (4 warnings)

Four files have `clone_elem<T: Clone>` functions with `fn_missing_requires`:

| # | File | Line |
|---|------|------|
| 1 | DoubleHashFlatHashTableStEph.rs | 75 |
| 2 | LinProbFlatHashTableStEph.rs | 73 |
| 3 | QuadProbFlatHashTableStEph.rs | 75 |
| 4 | StructChainedHashTable.rs | 76 |

These are clone bridge functions. The standard pattern from
`partial_eq_eq_clone_standard.rs` has `clone()` inside `Clone::clone` impl. But these
are standalone `clone_elem` helpers. They need a real requires — the appropriate module
wf predicate doesn't apply here since these are generic. Since these are bare clone
bridges with `ensures c == *x`, the only real requires would be about the type having
Clone. But `T: Clone` is already a bound.

**Strategy**: These functions are the centralized clone bridge pattern. The `assumes`
inside them are the allowed clone workaround per CLAUDE.md ("assume inside Clone::clone
body"). The fn_missing_requires warning is because they're free functions not in a Clone
impl. Read the actual code — if they truly have no precondition, note them for the user
but do not add `requires true` or `// veracity: no_requires`.

### Part C: Chap47 — ParaHashTableStEph Trait Warnings (8 warnings)

`ParaHashTableStEph.rs` has 8 fn_missing_wf warnings on trait methods (createTable,
insert, lookup, delete, metrics, loadAndSize, resize). These are abstract trait methods
that use `Self::spec_impl_wf(table)` in requires but veracity wants `table.spec_hashtable_wf()`.

The trait uses `spec_impl_wf` (implementation-specific wf) rather than the module-level
`spec_hashtable_wf`. This is intentional — different hash table implementations have
different wf predicates. The trait provides `spec_impl_wf` as the generic wf hook.

**Strategy**: Check if the warnings are about the trait definition or the implementations.
If trait-level: the trait uses `Self::spec_impl_wf` which IS the wf predicate. The trait
methods have `requires Self::spec_impl_wf(table)` which is correct. Report that these are
false positives from veracity not recognizing `spec_impl_wf` as equivalent to wf.

### Part D: Chap47 — DoubleHash wf Bridge Assumes (3 holes)

`DoubleHashFlatHashTableStEph.rs` has 3 wf bridge assumes (lines 116, 395, 541) in
insert, lookup, and delete. These assume that if a slot has a key, the hash matches
the expected pattern. This bridges the runtime step computation to the wf existential.

**Strategy**: The wf predicate says `exists s: usize | ...` (there exists a step that
covers all positions). The runtime computes `step = hash2(key)`. The assume bridges
"the step I computed at runtime IS the existential witness." Read the wf predicate
carefully and try to prove this by specializing the existential witness to the computed
step. If the wf existential doesn't give you the exact statement, you may need to add
an intermediate assertion.

### Part E: Chap47 — StructChainedHashTable resize (1 hole)

`StructChainedHashTable.rs` line 413: `external_body` on `resize`. This rehashes all
entries into a new larger table. Read the implementation — it should iterate all chains,
extract entries, insert into new table. If it delegates to the ParaHashTableStEph trait's
resize, the proof follows from the trait.

### Part F: Chap43 — OrderedSet fn_missing_requires (2 warnings)

| # | File | Line | Method |
|---|------|------|--------|
| 1 | OrderedSetStEph.rs | 1385 | from_sorted_elements |
| 2 | OrderedSetStPer.rs | 1157 | from_sorted_elements |

These need real requires — the input sequence should be sorted and contain no duplicates.
Read the function body to understand what it assumes about its input.

### Priority

1. Part A (3 holes) — add diverge(), quick wins
2. Part D (3 holes) — DoubleHash wf bridge, moderate
3. Part E (1 hole) — StructChained resize
4. Part F (2 warnings) — OrderedSet from_sorted_elements requires
5. Part B (4 warnings) — clone_elem fn_missing_requires (may be report-only)
6. Part C (8 warnings) — ParaHash trait warnings (may be report-only)

### Expected Results

Conservative: 3-4 holes closed + 2 warnings fixed.
Optimistic: 7 holes closed + 6 warnings fixed.

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent4-r41-report.md`.
