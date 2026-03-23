# Agent 2 — Round 61

You are Agent 2 working in `~/projects/APAS-VERUS-agent2`.

## Baseline

- Main: 4496 verified, 0 errors, 12 holes, 2610 RTT, 147 PTT.
- Chap53 GraphSearch: CLEAN (your R59 work, merged).
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Targets

### Target 1: Chap53 PQMinStEph.rs — 4 holes

Your R59 report identified the blocker: PQMinStEph lacks the
`lemma_wf_implies_len_bound` that PQMinStPer has (from AVLTreeSeqStPer).

**Fix path:**
1. Read `src/Chap37/AVLTreeSeqStPer.rs` — find `lemma_wf_implies_len_bound_stper`.
2. Read `src/Chap37/AVLTreeSeqStEph.rs` — find `lemma_size_eq_inorder_len`
   (private). There should also be a way to bound `len < usize::MAX` from wf.
3. Write (or make public) a `lemma_wf_implies_len_bound_steph` in
   AVLTreeSeqStEph.rs that mirrors the StPer version. The proof should
   follow the same pattern: wf implies bounded size, bounded size implies
   len < usize::MAX.
4. Use this lemma in PQMinStEph.rs to close the `priorities` and
   `initial_frontier` assumes (lines 240, 289).

**For the remaining 2 (visited/frontier_updated, lines 190/212):**
Your R59 report said the `Pair<Pair<P,V>,V>` nesting defeats Z3. Try:
- Extracting vertices from frontier entries into a ghost `Set<V::V>` that
  you maintain as a loop invariant, subset of vertex_universe.
- Helper spec functions that project through the nested Pair to extract
  the vertex component.
- If Z3 still can't handle it, report exactly where it fails.

### Target 2: Chap53 PQMinStPer.rs — 2 holes (lines 183, 204)

Same `visited/frontier_updated` pattern as StEph. Same approach: ghost
vertex set invariant, subset of vertex_universe.

## Validation

Run `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh` sequentially.
Write report to `plans/agent2-round61-report.md`. Push to `agent2/ready`.
