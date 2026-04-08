# R163 Prompt D — Compress Chap43/45/55 proof functions. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER delete `target/` or any subdirectory.**
6. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
7. **NEVER weaken `ensures` clauses.**

## Goal

Reduce proof lines across three chapters: OrderedTable (Chap43),
priority queues (Chap45), and graph DFS algorithms (Chap55). Each has
functions with 200+ proof lines and St/Per variant duplication.

## Target files and functions

All functions over 100 proof lines in files that have a 200+ function:

**`src/Chap43/OrderedTableStEph.rs`**

| # | Function | Kind | Proof Lines |
|---|----------|------|-------------|
| 1 | union_bypassed_r158 | exec | 255 |

**`src/Chap45/SortedListPQ.rs`**

| # | Function | Kind | Proof Lines |
|---|----------|------|-------------|
| 2 | meld | exec | 237 |
| 3 | insert | exec | 138 |

**`src/Chap55/CycleDetectStEph.rs`**

| # | Function | Kind | Proof Lines |
|---|----------|------|-------------|
| 4 | dfs_check_cycle | exec | 328 |

**`src/Chap55/CycleDetectStPer.rs`**

| # | Function | Kind | Proof Lines |
|---|----------|------|-------------|
| 5 | dfs_check_cycle | exec | 212 |

**`src/Chap55/TopoSortStEph.rs`**

| # | Function | Kind | Proof Lines |
|---|----------|------|-------------|
| 6 | dfs_finish_order | exec | 223 |

**`src/Chap55/TopoSortStPer.rs`**

| # | Function | Kind | Proof Lines |
|---|----------|------|-------------|
| 7 | dfs_finish_order | exec | 193 |

Total: ~1,586 proof lines across 7 functions in 6 files.

## Approach: Think deeply before coding

**Step 1 — Read all target files.** Read all standards first. Understand
each algorithm and its proof obligations.

**Step 2 — Identify within-chapter and cross-file duplication.**

- **Chap55 St/Per pairs.** `CycleDetectStEph` (328 lines) and
  `CycleDetectStPer` (212 lines) prove the same DFS cycle detection
  with different persistence models. Compare their proof blocks.
  Same for `TopoSortStEph` (223) vs `TopoSortStPer` (193). The StEph
  version is always larger — look for proof steps in StEph that StPer
  avoids. Can StEph use the same shorter approach?

- **Chap55 cross-algorithm.** CycleDetect and TopoSort both do DFS.
  They maintain visited sets, stack invariants, back-edge detection.
  Are there shared DFS invariant lemmas that both could use?

- **Chap45 SortedListPQ.** `meld` (237 lines) merges two sorted lists.
  `insert` (138 lines) inserts into a sorted list. Insert might be a
  special case of meld (meld with a singleton). If so, insert's proof
  could delegate to meld's lemma.

- **Chap43 union_bypassed_r158.** The `_bypassed_r158` suffix suggests
  this was a workaround from a previous round. Read the function — is
  the bypass still needed? Can the proof be simplified now that
  minimization removed unnecessary asserts?

**Step 3 — Extract repeated DFS invariant proofs.** For Chap55, the
DFS functions maintain invariants about:

- Visited set grows monotonically
- Stack entries are all visited
- Back edges connect to stack entries (for cycle detection)
- Finish order respects post-order (for topo sort)

If these invariant proofs are duplicated across the 4 files, extract
them into shared lemmas in a common location.

**Step 4 — Write lemmas and compress.** Validate after each extraction:

```bash
scripts/validate.sh isolate Chap55    # for Chap55 changes
scripts/validate.sh isolate Chap45    # for Chap45 changes
scripts/validate.sh isolate Chap43    # for Chap43 changes
```

## Report

Write `plans/agent4-round163-report.md` with:

- Table of lemmas created: name, file, callers, lines saved
- Table of 7 functions: proof lines before/after
- Cross-file sharing analysis
- Total proof lines removed
- Notes on `union_bypassed_r158` — is the bypass still needed?

## RCP

`git add -A && git commit -m "R163 Agent 4: compress Chap43/45/55 proof (−N lines)"`, then `git push`.
