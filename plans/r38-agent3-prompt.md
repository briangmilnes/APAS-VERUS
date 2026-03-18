# R38 Agent 3: Chap41 AVL + Chap57 Dijkstra + Misc Warnings

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

Read CLAUDE.md before starting.

## Assignment

You are Agent 3 for R38. Your scope is **Chap41 StEph, Chap57, Chap59 StEph, Chap39 warnings**.

## Task 1: Fix requires_true warnings in BSTTreapMtEph.rs (2 warnings)

File: `src/Chap39/BSTTreapMtEph.rs`
- Line 353: `requires true` — replace with real requires (likely `self.spec_bstparatreapmteph_wf()`)
- Line 389: `requires true` — same pattern

Read the functions, understand what they need, add real preconditions.

## Task 2: Fix fn_missing_requires warnings (4 warnings)

1. `src/Chap41/AVLTreeSetMtEph.rs:311` — `parallel_filter`: add real requires
2. `src/Chap41/AVLTreeSetMtEph.rs:372` — `parallel_intersect`: add real requires
3. `src/Chap41/AVLTreeSetMtPer.rs` — `parallel_sort`: add real requires
4. `src/Chap57/DijkstraStEphU64.rs:93` — `pq_entry_new`: add real requires

## Task 3: Fix bare_impl warning in AVLTreeSetStEph.rs

`src/Chap41/AVLTreeSetStEph.rs` — has 1 bare_impl warning. Move the method into the trait.

## Task 4: Prove assumes in AVLTreeSetStEph.rs (2 assumes)

`src/Chap41/AVLTreeSetStEph.rs` — 2 assumes about Vec length bounds after insert.
These are `assume(new_vec.len() <= ...)` style. The proof needs to show that
inserting into a sorted vec doesn't exceed the length bound. Read the insert
function carefully and trace the length arithmetic.

## Task 5: Prove assumes in DijkstraStEphU64.rs (3 assumes)

File: `src/Chap57/DijkstraStEphU64.rs`
- Line 166: `assume(obeys_feq_clone::<PQEntry>())` — feq clone bridge for PQEntry
- Line 201: `assume(BinaryHeapPQ::spec_is_exec_heap(...))` — heap invariant maintenance
- Line 242: `assume(remaining_budget > 0)` — Dijkstra PQ insert budget

For the budget proof: total PQ inserts <= |E| (each edge relaxed at most once).
Track edges processed vs budget consumed.

## Task 6: Fix fn_missing_requires in JohnsonStEphI64.rs (3 warnings)

File: `src/Chap59/JohnsonStEphI64.rs`
- Line 72: `adjust_distance` — add real requires
- Line 88: `reweight_edge` — add real requires
- Line 329: `create_negative_cycle_result` — add real requires

The assume at line 437 (graph size bound) is a stretch goal.

## Strategy

Start with Tasks 1-3 (warnings — fast). Then Task 4 (AVL assumes — medium).
Then Task 5 (Dijkstra — medium-hard). Task 6 last (Johnson warnings — fast).

## Validation

Run `scripts/validate.sh` after each file. Must be 0 errors.
Write your report to `plans/agent3-r38-report.md`.
