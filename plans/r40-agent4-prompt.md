# R40 Agent 4: Chap43 OrderedSet + AugOrderedTable Proofs + Warnings

## Baseline
- Main at `c1a1e964`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4290 verified, 186 holes, 29 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**

Read CLAUDE.md before starting.

## Context

Chap43 has 35 holes after the R39 restructure round. Agent 2 is working on the 26
OrderedTableStPer delegation wrappers. Agent 1 may still be working on OrderedTableStEph.
Your job is to prove the remaining Chap43 holes in the files NO OTHER AGENT is touching.

## Assignment

### DO NOT TOUCH these files (other agents are working on them):
- `OrderedTableStPer.rs` (Agent 2)
- `OrderedTableStEph.rs` (Agent 1)
- `OrderedTableMtEph.rs` (already clean per R39)

### File 1: `src/Chap43/OrderedSetStEph.rs` — 1 hole + 1 warning

**Hole** (line 1116): `assume(self@.filter(|x| exists|t: T| TotalOrder::le(t, *k) && ...)`
in `select`. This is a filter cardinality assume. The comment says: "Filter cardinality
requires sortedness of the backing sequence, which is true for AVL trees but not captured
in the wf spec."

**Strategy**: The backing AVLTreeSetStEph stores elements in sorted order. The `select`
operation needs to know that exactly `i` elements are ≤ the i-th element. This connects
filter cardinality to the sorted tree structure. You may need to:
1. Add a sortedness predicate to `spec_orderedsetsteph_wf()`
2. Or prove that AVLTreeSetStEph's `to_seq()` returns a sorted sequence
3. Then connect sortedness to filter cardinality

If this is genuinely hard (it may be), report what you tried and move on.

**Warning** (line 1384): `fn_missing_requires` on `from_sorted_elements`. This is a
constructor that takes a sorted sequence and builds an OrderedSet. It needs a real
requires — likely `elements` is sorted and contains no duplicates.

### File 2: `src/Chap43/OrderedSetStPer.rs` — 1 hole + 1 warning

**Same pattern** as OrderedSetStEph:
- **Hole** (line 1031): Same select filter cardinality assume
- **Warning** (line 1157): Same `fn_missing_requires` on `from_sorted_elements`

Fix both files together — same technique.

### File 3: `src/Chap43/AugOrderedTableMtEph.rs` — 4 holes + 2 warnings

**Hole 1** (line 87): `assume(forall|v1, v2| reducer.requires((v1, v2)))` in
`calculate_reduction`. This is a closure totality assume — the reducer function should
be callable on any two values.

**Hole 2** (line 640): `assume(self@.dom().finite())` in `reduce_val`. The table's domain
should be finite from wf, but the function lacks a requires clause.

**Hole 3** (line 650): `assume(self@.dom().finite())` in `reduce_range`. Same issue.

**Hole 4** (line 656): `external_body` on `reduce_range_parallel`. Parallel reduction
over a key range.

**Warnings** (lines 70, 80): `fn_missing_requires` on `recalculate_reduction` and
`calculate_reduction`. These need `spec_augorderedtablemteph_wf()` requires.

**Strategy for holes 2-3**: Add `requires self.spec_augorderedtablemteph_wf()` and prove
finiteness from wf. Read `lemma_aug_view` (called before the assume) to understand
what it provides.

**Strategy for hole 1**: This is a closure requires propagation. Read
`src/standards/using_closures_standard.rs`. The caller must prove the reducer is total.
Lift the requirement into the function's own `requires` clause:
`requires forall|v1: &V, v2: &V| f.requires((v1, v2))`.

**Strategy for hole 4**: If `reduce_range_parallel` delegates to sequential reduce, it
may be provable by acquiring the lock, computing the range, then reducing sequentially.
If it requires real parallelism, leave the `external_body`.

### File 4: `src/Chap43/AugOrderedTableStPer.rs` — 1 hole

**Hole** (line 124): `assume(forall|v1, v2| cloned.requires((v1, v2)))` in
`lemma_reducer_clone_total`. This is a proof function that says cloning a total reducer
preserves totality. The closure clone bridge pattern — same as eq/clone bridges but for
closures.

This may be irreducible (Verus can't prove properties of cloned closures). If so, report
why and leave it.

### File 5: `src/Chap43/OrderedSetMtEph.rs` — 1 hole

**Hole** (line 344): `external_body` on `to_seq`. This acquires a read lock, calls
`to_seq()` on the inner OrderedSetStEph, and releases. The proof needs to connect the
inner's ensures to the MtEph's ensures through the RwLock.

Read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` for the pattern.

### Priority

1. AugOrderedTableMtEph warnings (2) — add requires, quick win
2. OrderedSetStEph/StPer warnings (2) — add requires to from_sorted_elements
3. AugOrderedTableMtEph finiteness holes (2) — add requires + prove from wf
4. AugOrderedTableMtEph closure hole (1) — lift requires
5. OrderedSetMtEph to_seq (1) — RwLock delegation
6. OrderedSetStEph/StPer select assumes (2) — hardest, may not close

### Expected Results

Conservative: 4-5 holes closed + 4 warnings fixed.
Optimistic: 7-8 holes closed + 4 warnings fixed.

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent4-r40-report.md`.
