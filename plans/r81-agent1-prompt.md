# R81 Agent 1 — Prove `union` in UnionFindStEph, STEP 20

## Objective

Remove `external_body` from `union` in `src/Chap65/UnionFindStEph.rs:532` and prove it.

## ISOLATION — CRITICAL

To save memory and allow other agents to validate in parallel, **comment out all
chapters except Chap65's dependencies in lib.rs** before your first validate. UnionFindStEph's
transitive closure is small:

**Keep uncommented:**
- `Types` (line 13)
- `Concurrency` (line 14)
- `vstdplus` block (lines 264-292) — entire block, it's all foundation
- `Chap65` block (lines 728-732)

**Comment out everything else** (with `// R81-ISOLATED:` prefix so it's easy to undo):
- `ParaPairs` (line 16)
- `experiments` block (lines 18-236)
- `standards` block (lines 239-262)
- All chapter blocks: Chap02 through Chap44, Chap45 through Chap64, Chap66

This cuts validation from 4915 functions / 8 GB to ~200 functions / <2 GB. You MUST
do this before your first `scripts/validate.sh` run.

**Before pushing to agent1/ready, UNCOMMENT EVERYTHING.** Restore lib.rs to its original
state. Run a full `scripts/validate.sh` to confirm 4915 verified, then push.

## Context

The body (lines 533-550) is already written and correct:
1. Find roots of u and v via `find` (which delegates to `find_root_loop`, no mutation)
2. If roots differ, union by rank: point the smaller-rank root to the larger, bump rank on tie
3. Update ghost `roots` map so all elements in both components share one root

The ensures (lines 417-429) say:
- wf preserved
- parent domain unchanged
- elements unchanged
- roots merge: elements formerly rooted at root_u or root_v now share a common root; others unchanged

## What makes this hard

The body mutates `parent` (one insert) and possibly `rank` (one insert on equal-rank).
The ghost `roots` must be bulk-updated: every element whose old root was root_u or root_v
gets a new root. Proving all 13 wf conjuncts hold after these mutations is the core work.

## Approach

Follow the pattern of `lemma_insert_preserves_wf` (lines 168-251): write a
`lemma_union_preserves_wf` that takes old/new state + the specific mutations and proves
wf. Break into sub-assertions if the monolithic proof times out.

Key facts available from `find`'s ensures:
- `root_u@ == old(self)@.roots[u@]`, `root_v@ == old(self)@.roots[v@]`
- Both roots are self-parents: `parent[root_u@]@ == root_u@` (from `spec_self_parent_is_root` + roots idempotent)
- `find` preserves everything (roots, parent dom, rank, elements) since it uses `find_root_loop`

Things to prove after the parent/rank/roots update:
1. **parent_closed**: the new parent entry points to a key already in dom (root→root, both in dom)
2. **roots_idempotent**: new roots map is idempotent (merged component gets winner root)
3. **parent_preserves_root**: new parent[w]@ has same new-root as w
4. **rank_increases**: only changed for the losing root, which now points to the winner whose rank ≥ old rank
5. **rank_bounded**: rank[w] ≤ rank[new_roots[w]] for all w
6. **self_parent_is_root**: the winner is still a self-parent root; the loser is no longer self-parent
7. **Frame**: elements, elements_forward/backward/distinct unchanged; parent/rank/roots dom unchanged

The `roots` ghost update is: for all x in dom, if old roots[x] == root_u@ or root_v@,
set roots[x] to the winner root; else keep old roots[x]. The winner root is root_u@ when
rank_u ≥ rank_v, and root_v@ when rank_u < rank_v.

## Clone handling

`root_u` and `root_v` are used multiple times (in comparison, in parent.insert, in
rank operations). You'll need clones. Use the `strictly_cloned` + `obeys_feq_full` pattern
from `insert` and `find_root_loop`. Consider a helper like `lemma_three_clones_eq` if
you need multiple clones of the same value.

## Important

- Do NOT delete any commented-out compression code.
- Do NOT add `assume` or `accept` — prove everything.
- Do NOT weaken the ensures clause.
- The named wf sub-predicates (lines 76-146) are available for targeted assertions.

## STEP 20

At most 20 edit/verify iterations. Then stop and report.

## Validation

During development: run `scripts/validate.sh` with isolated lib.rs (fast, low memory).
Before pushing: restore full lib.rs, run `scripts/validate.sh` (expect 4915), then
`scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent1/ready`.

## Report

Write `plans/agent1-round81-report.md` with holes before/after (table with Chap column).
