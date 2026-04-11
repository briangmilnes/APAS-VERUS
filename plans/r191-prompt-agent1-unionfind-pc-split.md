# R191 Prompt — Agent 1: Split compress_step to kill matching loop. AFK.

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

R190: 746 verified, 2 errors. Path compression built, per-element
`compress_preserves_find` proves in 2s. The matching loop is in
`lemma_compress_step` where the rank invariant quantifiers leak into
the compress-find proof context (27 GB Z3 RSS).

You identified the fix in your report. Do it.

## The fix

Split `lemma_compress_step` into two separate proof fns:

### 1. lemma_compress_step_wf

Proves: parent-in-domain, rank invariant, rank bounded, domain finiteness,
domain length preserved. Does NOT call compress_preserves_find. Does NOT
mention find preservation. Keeps the rank quantifiers isolated.

### 2. lemma_compress_step_find

Takes the new parent map's wf as a requires (from step_wf). Calls
compress_preserves_find for each z. Does NOT re-derive rank invariant.
Keeps the find-preservation quantifiers isolated from rank quantifiers.

### Call pattern in find() compression loop

```rust
// After setting parent[curr] = root:
proof {
    lemma_compress_step_wf(...);   // proves new map is wf
    lemma_compress_step_find(...); // proves find preserved for all z
}
```

Two separate Z3 contexts. Rank quantifiers in one, find quantifiers
in the other. No cross-pollination.

## Also: size_rank_inv through compression

The compression loop invariant in R190 used "light wf" (no size_rank_inv).
If you can maintain size_rank_inv through compression, great — it means
the full wf holds after find(). If not, maintain it separately or prove
it's restored after the full compression pass.

Compression doesn't change subtree membership (find results unchanged) or
ranks. So size_rank_inv should hold trivially through compression:
- subtree(root) = {z | find(z) == root} — unchanged (find preserved)
- rank[root] — unchanged
- |subtree(root)| >= rank[root] + 1 — follows from above

## Target: 0 errors.

## Validation

```bash
scripts/validate.sh isolate UnionFind
```

## Report

Write `plans/agent1-round191-report.md`.

## RCP

`git add -A && git commit -m "R191 Agent 1: split compress_step — zero errors"`, then `git push`.
