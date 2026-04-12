# R192 Prompt — Agent 1: Micro-split compress_step_wf. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER read `src/Chap65/UnionFindStEph.rs`.** Clean-room.
7. **NEVER modify `Cargo.toml`, `scripts/validate.sh`.**
8. **DO NOT REBASE. Build on your branch.**

## Read all standards first

## Context

R191: 713 verified, 2 errors. Z3 RSS down from 27 GB to 3-7 GB. The split
into step_wf + step_find killed the rank/find cross-pollination. But
step_wf itself still has parent-in-domain and rank-invariant quantifiers
cross-pollinating. You identified the fix in your report.

## The fix

Split `lemma_compress_step_wf` into 3 micro-lemmas:

### 1. lemma_compress_parent_in_dom

Proves ONLY: for all k in pn.dom(), pv(pn, k) is in pn.dom().
Requires: po wf (parent-in-domain part only), pn characterization.
Does NOT mention rank.

### 2. lemma_compress_rank_inv

Proves ONLY: for all k in pn.dom() where pn[k] != k, rank[k] < rank[pn[k]].
Requires: po rank_invariant, pn characterization, rank unchanged.
Does NOT re-derive parent-in-domain.

### 3. lemma_compress_basic

Proves: domain finiteness, domain length preserved, root validity,
rank bounded. Small facts that don't involve quantifier chains.

### Call pattern

```rust
proof {
    lemma_compress_basic(...);         // small facts
    lemma_compress_parent_in_dom(...); // parent-in-domain only
    lemma_compress_rank_inv(...);      // rank invariant only
    lemma_compress_step_find(...);     // find preservation only
}
```

Four separate Z3 contexts. No cross-pollination between:
- parent-in-domain quantifiers
- rank invariant quantifiers
- find preservation quantifiers

## Also restore verified count

R191 dropped from 746 to 713 because of light-wf loop invariant. Once
the micro-split works (0 errors), restore size_rank_inv at find() return
points to get full wf. The proof is: compression doesn't change subtrees
or ranks, so size_rank_inv holds trivially.

## Target: 0 errors.

## Validation

```bash
scripts/validate.sh isolate UnionFind
```

## Report

Write `plans/agent1-round192-report.md`.

## RCP

`git add -A && git commit -m "R192 Agent 1: micro-split compress_step_wf — zero errors"`, then `git push`.
