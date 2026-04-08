# R166 Prompt F — Agent 6: Lift repeated proof patterns in Chap39 + Chap47 + Chap55. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent6`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER weaken `ensures` clauses.**

## Goal

Three chapters with within-module proof duplication:

- Chap39 BSTTreap: 7 fns ≥50 in BSTTreapStEph (751 lines), 7 in
  BSTParaTreapMtEph (1335). R165 extracted `BSTTreapSpecsAndLemmas.rs`.
  Now find within-function patterns.
- Chap47 hash tables: QuadProbFlatHashTableStEph has 7 fns ≥50 (800 lines).
  LinProb and DoubleHash have similar patterns. All prove probe sequence
  invariants and slot occupancy after insert/delete.
- Chap55 DFS: CycleDetectStEph (779 lines in 3 fns). R165 extracted
  `DFSSpecsAndLemmas.rs`. Now find within-function patterns.

## Your files

**Chap39:**
- `src/Chap39/BSTTreapSpecsAndLemmas.rs` — ADD lemmas here
- `src/Chap39/BSTTreapStEph.rs` — 7 fns ≥50 (751 total)
- `src/Chap39/BSTTreapMtEph.rs`
- `src/Chap39/BSTParaTreapMtEph.rs` — 7 fns ≥50 (1335 total)

**Chap47:**
- `src/Chap47/QuadProbFlatHashTableStEph.rs` — 7 fns ≥50 (800 total)
- `src/Chap47/LinProbFlatHashTableStEph.rs` — insert (147), delete (101)
- `src/Chap47/DoubleHashFlatHashTableStEph.rs` — insert (109), delete (110)
- `src/Chap47/StructChainedHashTable.rs` — resize (174)
- `src/Chap47/VecChainedHashTableStEph.rs`, `LinkedListChainedHashTableStEph.rs`

**Chap55:**
- `src/Chap55/DFSSpecsAndLemmas.rs` — ADD lemmas here
- `src/Chap55/CycleDetectStEph.rs` — dfs_check_cycle (328 lines)
- `src/Chap55/CycleDetectStPer.rs` — dfs_check_cycle (212 lines)
- `src/Chap55/TopoSortStEph.rs`, `TopoSortStPer.rs`
- `src/Chap55/DFSStEph.rs`, `DFSStPer.rs`, `SCCStEph.rs`, `SCCStPer.rs`

## Approach

**Step 1 — Read all files and all standards.**

**Step 2 — Chap39 BSTTreap.** split, join, union, intersect, difference,
filter all do BST traversal with priority-based decisions. The proof that
the result is a valid treap (BST-ordered AND heap-ordered) repeats in
every operation. Extract treap validity lemmas into `BSTTreapSpecsAndLemmas.rs`.

**Step 3 — Chap47 hash tables.** The 3 flat hash table variants (linear probe,
quadratic probe, double hash) prove the same probe sequence properties:
- Slot at probe index is empty or contains the key
- Probe sequence terminates
- Table size invariant after resize

These are prime candidates for a new `HashTableSpecsAndLemmas.rs` or shared
lemmas within each file.

**Step 4 — Chap55 DFS.** CycleDetectStEph (328 lines) and StPer (212 lines)
are the same algorithm. Find proof blocks in StEph that StPer avoids — those
are redundant in StEph. Also look for patterns shared with TopoSort and SCC.

**Step 5 — Validate:**
```bash
scripts/validate.sh isolate Chap39
scripts/validate.sh isolate Chap47
scripts/validate.sh isolate Chap55
```

## Report

Write `plans/agent6-round166-report.md`.

## RCP

`git add -A && git commit -m "R166 Agent 6: lift proof patterns Chap39+47+55 (−N lines)"`, then `git push`.
