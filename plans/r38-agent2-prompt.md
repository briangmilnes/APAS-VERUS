# R38 Agent 2: Chap43 OrderedSet/OrderedTable Proofs

## Baseline
- Main at `485299d3`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4332 verified, 204 holes, 29 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
If you write `accept(` in any file, you have failed your assignment. Use `assume()`
for eq/clone bridges inside Clone::clone and PartialEq::eq bodies ONLY. Everywhere
else, PROVE the obligation or leave the existing assume in place.

**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true`.** Add real preconditions or leave the warning.
**DO NOT add `// veracity: no_requires`.** Only the user adds those.
**DO NOT convert assume() to accept().** Leave assumes as assumes.

Read CLAUDE.md and the relevant standards before starting.

## Assignment

You are Agent 2 for R38. Your scope is **all holed files in Chap43**.

## Task 1: Fix fn_missing_requires/ensures warnings (5 warnings)

1. `src/Chap43/AugOrderedTableMtEph.rs:72` — `recalculate_reduction`: add `requires table.spec_augorderedtablemteph_wf()`
2. `src/Chap43/AugOrderedTableMtEph.rs:85` — `calculate_reduction`: add `requires base.spec_orderedtablemteph_wf()`
3. `src/Chap43/OrderedSetStEph.rs` — `from_sorted_elements`: add `requires` with `spec_orderedsetsteph_wf` on output
4. `src/Chap43/OrderedSetStPer.rs` — `from_sorted_elements`: same pattern
5. `src/Chap43/OrderedTableMtEph.rs` — fn_missing_wf_ensures: add wf ensures

Read each function, understand what it needs, add the REAL precondition.

## Task 2: Prove external_body delegation wrappers

These are functions that delegate to a base type method. Remove `#[verifier::external_body]`,
call the base method, and let its ensures satisfy ours.

1. `src/Chap43/OrderedSetMtEph.rs:344` — `to_seq` external_body
   - Delegates through RwLock to AVLTreeSetStEph::to_seq
   - Pattern: acquire lock, call inner.to_seq(), return result

2. `src/Chap43/OrderedTableStEph.rs` — `collect` external_body
   - Delegates to base table's collect

3. `src/Chap43/OrderedTableStPer.rs` — `collect` external_body
   - Same pattern as StEph

4. `src/Chap43/OrderedTableMtEph.rs` — `rank_key` and `select_key` external_body (2 holes)
   - Delegate through RwLock to OrderedTableStEph methods

## Task 3: Prove algorithmic assumes

1. `src/Chap43/AugOrderedTableMtEph.rs:92` — closure requires assume
   - Need to propagate closure requires from caller. Read `using_closures_standard.rs`.

2. `src/Chap43/AugOrderedTableStPer.rs:124` — lemma_reducer_clone_total assume
   - Clone bridge for closures. Hard — may need to leave if stuck.

3. `src/Chap43/OrderedSetStEph.rs` — 2 assumes (to_seq clone bridge, select filter cardinality)
4. `src/Chap43/OrderedSetStPer.rs` — 1 assume (select filter cardinality)
5. `src/Chap43/OrderedTableStEph.rs` — rank_key assume + select_key assume(false)
6. `src/Chap43/OrderedTableStPer.rs` — rank_key assume + select_key assume(false)

## Strategy

Start with Task 1 (warnings — quick wins). Then Task 2 (delegation wrappers — mechanical).
Then Task 3 (algorithmic assumes — hardest). If a proof is genuinely stuck after real
effort, leave the assume and report what you tried.

## Validation

Run `scripts/validate.sh` after each file. Must be 0 errors.
Write your report to `plans/agent2-r38-report.md`.
