# r224: fix Chap64 spanning tree on the two-vertex graph

Branch `r224/spantree`, worktree `~/projects/APAS-VERUS-r224`, from main
`c22859345`. Toolchain: Verus `0.2026.09.13.671956e`, Z3 4.16.0.

## Problem

`tests/Chap64/TestSpanTreeMtEph.rs::test_spanning_tree_mt_two_vertices` fails
deterministically on main:

```
panicked at tests/Chap64/TestSpanTreeMtEph.rs:69:5:
assertion `left == right` failed
  left: 0
 right: 1
```

For the graph V = {0, 1}, E = {(0, 1)} and seed 42,
`spanning_tree_star_contraction_mt` (`src/Chap64/SpanTreeMtEph.rs`) returns no
edges. The spanning tree of a connected graph on 2 vertices has exactly 1 edge.
Commit `50c0aeacf` ("ungate 9 test files") re-enabled this test, so the defect
may be old. The function ensures only `tree_edges.spec_setsteph_wf()`, so
verification does not detect it.

## Steps

1. Read CLAUDE.md and all of `src/standards/`.
2. Reproduce the failure: `scripts/rtt.sh TestSpanTreeMtEph`.
3. Find the root cause. Trace `spanning_tree_star_contraction_mt` through
   the Chap62 star-contraction framework (`star_contract_mt`, the star
   partition, and the `base` and `expand` closures) for this input. Candidate
   causes to check first:
   - what `partition_map` contains when a satellite maps to its center;
   - whether the base case fires before the edge contributes;
   - how quotient edges map back to original edges.
   State the cause as a concrete statement about the code.
4. Check whether `SpanTreeStEph.rs` and other users of the same Chap62
   framework (grep for `star_contract`) have the same defect. The fix must
   cover every instance (ComputAItionalThinking rule 6).
5. Fix the root cause in the algorithm. Keep it parallel. Do not change the
   algorithm's cost. Do not special-case two vertices.
6. Tests (RTT):
   - Keep `test_spanning_tree_mt_two_vertices` unchanged; it must pass.
   - Add a seed sweep: small graphs (2-8 vertices; paths, stars, cycles,
     complete graphs, and disconnected graphs), seeds 0..200 each, for both
     MtEph and StEph. Check the result with `verify_spanning_tree`, and check
     the edge count: |V| − (number of components).
7. Specs. If the fix makes it tractable, strengthen the ensures of
   `spanning_tree_star_contraction_mt` (and StEph) toward the textbook
   property, at least "every tree edge is an edge of the graph." Do not
   weaken any spec. If a strengthened ensures would need an assume, stop and
   report instead.
8. Verify with `scripts/validate.sh isolate Chap64` (and `isolate Chap62` if
   Chap62 changes) until 0 errors, 0 warnings, and 0 trigger notes.
9. Seeds 1-8 on each changed function (the r219 method:
   `VERUS_EXTRA_ARGS="--verify-only-module ... --smt-option smt.random_seed=N"`).
10. Run `scripts/rtt.sh SpanTree` and `scripts/rtt.sh Chap62`.
11. Run `scripts/holes.sh src/Chap64/` (and `src/Chap62/` if changed): no new holes.
12. Report `plans/r224-spantree-two-vertices-report.md`. Include the root cause,
    the fix, the files changed, the tests added, spec changes, seed results,
    and holes. Tables have `#` and `Chap` columns.
13. Commit on the branch with `git add -A`; push `r224/spantree`. Do not merge.

## Constraints

- A second round (r223, Chap37) runs at the same time on this machine.
  - Do NOT run full `scripts/validate.sh`, full `scripts/rtt.sh`, or
    `scripts/ptt.sh`. Only isolate validates and filtered RTTs.
  - The main session runs the full sequential checks at merge time.
- No assume, admit, accept, or external_body added; no rlimit raised; no spec
  weakened; no `#![auto]`; no formatters; no subagents.
- Work only in `~/projects/APAS-VERUS-r224`.
- Never pipe or filter the output of `scripts/validate.sh`; read the log in `logs/`.
- No Python, no Perl, and no `git clean`.
- Ask before reverting proof work.

## Success criteria

- `test_spanning_tree_mt_two_vertices` passes unchanged.
- The seed-sweep tests pass for MtEph and StEph.
- Isolate Chap64 (and Chap62 if touched) verifies with 0 errors.
- Every changed function verifies under seeds 1-8.
- No new holes.
