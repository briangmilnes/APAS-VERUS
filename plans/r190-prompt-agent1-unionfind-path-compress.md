# R190 Prompt — Agent 1: Add path compression to HashMap UnionFind. AFK.

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

R189: HashMap-based UnionFind fully proven. 731 verified, 0 errors.
`find()` takes `&self` (immutable, no path compression). APAS Chapter 65
specifies path compression. The F* CLRS version has it proved.

## What to build

Create `src/UnionFind/UnionFindPCStEph.rs` — a copy of `UnionFindStEph.rs`
with path compression added to `find()`. Add it to lib.rs:

```rust
pub mod UnionFind {
    pub mod UnionFindArrayStEph;
    pub mod UnionFindStEph;
    pub mod UnionFindPCStEph;
}
```

### Changes from UnionFindStEph

1. **find() signature**: `&self` → `&mut self` (compression mutates parent)

2. **find() body**: two-pass algorithm (CLRS §21.3):
   - Pass 1: chase parent pointers to find root (same as current find)
   - Pass 2: walk the path again, setting each node's parent directly to root

3. **find() ensures**: same postconditions PLUS:
   - `spec_pure_find` results unchanged for ALL elements (not just the queried one)
   - wf preserved
   - rank unchanged
   - domain unchanged (pointwise, no =~=)

4. **union_sets()**: calls the new `find(&mut self)` instead of `find(&self)`

5. **equals()**: `&self` → `&mut self` (calls find internally)

### The key proof: compression preserves find for all elements

After compressing the path from v to root, every node on the path now
points directly to root. But `spec_pure_find(z)` for ALL z must be
unchanged — not just nodes on the compressed path.

The proof: for any z:
- If z's find path doesn't go through any compressed node: unchanged
  (parent pointers outside the path are untouched)
- If z's find path goes through a compressed node c: c now points to root.
  Before compression, c pointed to some ancestor a on the path. Both c→a→...→root
  and c→root lead to the same root. So find(z) still reaches the same root.

The F* version proves this as `compress_preserves_find` and
`compress_preserves_find_all`:

Read: `~/projects/AlgoStar/autoclrs/ch21-disjoint-sets/CLRS.Ch21.UnionFind.Spec.fst`

Search for `compress_preserves_find`. Study the proof structure.

### Compression preserves wf

- Parent values stay in domain (root is in domain, we're setting parent to root)
- rank_invariant: for compressed nodes, rank[node] < rank[root] (was true
  before — rank doesn't change, root is the same root)
- rank_bounded: unchanged (rank array not modified)
- size_rank_inv: subtrees don't change (same roots, same membership)

### No =~= rule

Same as UnionFindStEph: **never use =~= on Map domains.** Compression
changes parent values but not the domain. Express this as:
```rust
forall|k: VV| self@.parent.dom().contains(k) <==> old(self)@.parent.dom().contains(k)
```

## Also: remove Chap65 external_body functions

While this module validates the chapter, also attempt to rewire Chap65's
Kruskal to use the new UnionFindPCStEph. If Kruskal migration is too large
for one round, at least get the UnionFindPCStEph module proven and report
what Kruskal needs.

The 3 remaining `external_body` in Chap65/UnionFindStEph.rs
(lemma_union_merge_wf, lemma_union_merge_wf_half_a, fn union) become
irrelevant once Kruskal uses the new module.

## Validation

```bash
scripts/validate.sh isolate UnionFind
```

Or full validate. Target: 0 errors on UnionFindPCStEph.

## Report

Write `plans/agent1-round190-report.md`.

## RCP

`git add -A && git commit -m "R190 Agent 1: UnionFindPCStEph — path compression proved"`, then `git push`.
