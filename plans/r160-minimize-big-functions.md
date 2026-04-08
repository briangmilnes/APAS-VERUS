# R160 — Minimize Big Proof Functions

One prompt per module. Each prompt lists all functions over 100 asserts
in that module. Assign to available agents — no conflicts since each
prompt owns one file.

All prompts share the common rules and approach below.

## Common Rules

1. **NEVER modify `~/projects/verus/`.**
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.**
4. **NEVER delete `target/`.**
5. Do NOT weaken any ensures or loop invariant.
6. Do NOT add assumes, accepts, or external_body.
7. Keep ALL lemma calls, `choose` expressions, and `assert forall` headers.
8. Keep ALL real math (modular arithmetic, TotalOrder transitivity chains,
   triangular numbers, heap ordering, BST ordering, graph reachability).
9. Remove: tautologies, redundant case splits, intermediate equalities Z3 derives,
   `assert(obeys_feq_full_trigger::<T>())` when broadcast handles it,
   `assert(x == x)` style tautologies, intermediate equalities restating
   what the previous line computed.
10. All existing RTTs must pass.
11. STEP 15 per function. Move on if stuck.
12. Report per function: assert count before/after, line count before/after,
    isolate validation time before/after.
13. RCP when done.

## Approach

For each function:
1. Read the ensures and loop invariants — these are the proof obligations.
2. Comment out ALL proof blocks and standalone asserts.
3. Validate — read which obligations fail.
4. Add back MINIMUM assertions to fix each failure.
5. Iterate (usually 3-5 rounds per function).
6. If a function won't minimize after 5 iterations, stop and move to the next.

---

## Prompt A: Chap39 BSTParaTreapMtEph (17s isolate)

File: `src/Chap39/BSTParaTreapMtEph.rs`
Validate: `scripts/validate.sh isolate Chap39`
Report: `plans/r160-minimize-chap39-report.md`

Functions to minimize:

| # | Function | Lines | Asserts |
|---|----------|-------|---------|
| 1 | join_pair_inner | 208 | 262 |
| 2 | filter_inner | 155 | ~100 |
| 3 | union_inner | 153 | ~100 |
| 4 | difference_inner | 143 | ~95 |
| 5 | intersect_inner | 142 | ~95 |
| 6 | split_inner | 132 | ~90 |

Treap operations with priority-based rotation. Real math but 262 asserts
in 208 lines is absurd. Start with join_pair_inner (worst ratio), apply
patterns to the others.

---

## Prompt B: Chap41 OrdKeyMap (51s isolate)

File: `src/Chap41/OrdKeyMap.rs`
Validate: `scripts/validate.sh isolate Chap41`
Report: `plans/r160-minimize-chap41-report.md`

Functions to minimize:

| # | Function | Lines | Asserts |
|---|----------|-------|---------|
| 1 | ordkeymap_split | 354 | 230 |
| 2 | ordkeymap_prev | 279 | 224 |
| 3 | ordkeymap_next | 282 | 214 |
| 4 | ordkeymap_rank | 257 | 207 |
| 5 | ordkeymap_select | 303 | 168 |
| 6 | union_with | 258 | 164 |
| 7 | union | 230 | 164 |

These were copied from OrderedTableStEph with maximum AI verbosity.
The minimizer already confirmed 79% of union_with asserts are removable.
Expect similar ratios on the others.

---

## Prompt C: Chap42 Table union (94s isolate)

File: `src/Chap42/TableMtEph.rs` — `fn union` (307 lines, 111 asserts)
File: `src/Chap42/TableStEph.rs` — `fn union` (239 lines, 89 asserts)
Validate: `scripts/validate.sh isolate Chap42`
Report: `plans/r160-minimize-chap42-report.md`

Two files but same algorithm. Minimize StEph first, apply to MtEph.
Iterative key-value table union with key-uniqueness proofs.

---

## Prompt D: Chap55 DFS functions (89s isolate)

File: `src/Chap55/CycleDetectStEph.rs` — `fn dfs_check_cycle` (359 lines, 151 asserts)
File: `src/Chap55/CycleDetectStPer.rs` — `fn dfs_check_cycle` (237 lines, 113 asserts)
File: `src/Chap55/TopoSortStEph.rs` — `fn dfs_finish_order` (236 lines, 123 asserts)
File: `src/Chap55/TopoSortStPer.rs` — `fn dfs_finish_order` (204 lines, ~100 asserts)
Validate: `scripts/validate.sh isolate Chap55`
Report: `plans/r160-minimize-chap55-report.md`

DFS graph traversals with visited-set and cycle/reachability invariants.
Real graph proof — expect 30-50% reduction, not 85%. Do StEph variants
first, apply patterns to StPer.

---

## Scheduling

| Prompt | Isolate | Total Asserts | Est. Agent Time |
|--------|---------|--------------|-----------------|
| A (Chap39) | 17s | ~742 | 15-20 min |
| B (Chap41) | 51s | ~1,371 | 30-40 min |
| C (Chap42) | 94s | ~200 | 15-20 min |
| D (Chap55) | 89s | ~487 | 20-30 min |

Total: ~2,800 asserts across 17 functions.
Fastest first: A, then C, then D, then B.
