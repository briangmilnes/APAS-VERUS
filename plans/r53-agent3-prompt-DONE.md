<!-- THESE ARE DONE. DO NOT WORK ON THESE TASKS. IGNORE THIS FILE. -->
You are agent3 working in branch: ~/projects/APAS-VERUS-agent3/.
Your worktree has been updated.
Read senior-proof-engineer.mdc (from .cursor/rules).
REREAD ANY FILES BEFORE MODIFYING THEM as I modify files in emacs.

Round 53 — Chap65 Prim MST + Chap43 AugOrderedTableMtEph.

Current state: 4476 verified, 15 holes, 38 clean chapters.

## Phase 1: Chap65 PrimStEph prim_mst (1 hole)

The last Chap65 hole is external_body on `prim_mst` in PrimStEph.rs:96.
This is Prim's MST algorithm using a BinaryHeapPQ priority queue.

Read the full function (lines 96-180). The algorithm:
  - Maintain a PQ of (distance, vertex, parent_edge) entries
  - Pop minimum, skip if visited
  - Add the edge to MST, mark visited
  - Push all unvisited neighbors

To prove this:
1. Remove `#[verifier::external_body]`
2. Write a while-loop invariant. Key properties:
   - `visited` is a subset of graph vertices
   - `mst_edges` contains only valid graph edges
   - `mst_edges` edges connect visited vertices
   - The result is a well-formed SetStEph
3. The ensures is currently just `result.spec_setsteph_wf()`. That's achievable
   without proving MST correctness — you just need the structural wf.

Start with proving wf-only ensures. The full MST correctness proof (spanning,
minimum weight) is a stretch goal.

Read src/Chap45/BinaryHeapPQ.rs to understand the PQ API and its specs.
Read src/Chap05/SetStEph.rs for SetStEph insert/contains specs.

## Phase 2: Chap43 AugOrderedTableMtEph reduce_range_parallel (1 hole)

AugOrderedTableMtEph.rs:672 has external_body on `reduce_range_parallel`.
This is a fork-join parallel reduction over a key range.

Read src/standards/using_closures_standard.rs and the fork-join-inside-verus rule.
Read the function body (lines 672-710). The pattern:
  - Split the range at midpoint
  - Fork-join reduce both halves
  - Combine with the monoid operation

This is a classic fork-join inside verus! pattern. Use named closures with
explicit ensures, and `join()`.

Key rules:
- WARNING: Do NOT add accept() anywhere.
- Run scripts/validate.sh after changes. Show full output.
- DO NOT touch Chap47 (agent1's territory).
- Search vstd for lemmas before writing new ones.

Success criteria: Close Chap65 (0 holes). Remove AugOrderedTableMtEph external_body.

REPORTING: Write plans/agent3-round53-report.md with holes before/after table.

Execute relentlessly. Propose a plan, then implement it.
