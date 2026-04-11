# R181 Prompt — Agent 1: Prove rank_bounded in equal-rank union. Last error. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER read `src/Chap65/UnionFindStEph.rs`.** Clean-room.

## Read all standards first

## Context

R180: 712 verified, 1 error, 0 assumes. Everything proves except one thing:

After `rank[root_x] += 1` in the equal-rank union case, you need to prove
`rank[root_x] + 1 < n` (strict). You know `rank[root_x] < n` from
spec_rank_bounded, giving only `rank[root_x] + 1 <= n`. Need to shave off 1.

## The proof

Two distinct roots root_x and root_y exist with equal rank r. Each root's
subtree has at least r+1 elements. The subtrees are disjoint. So n >= 2*(r+1),
which gives r+1 <= n/2, therefore r+1 < n (since n >= 2*(r+1) >= 2 > 0).

### Step 1: Define subtree membership

An element z belongs to root's subtree iff `spec_pure_find(z) == root`.

### Step 2: Prove subtree size >= rank + 1

A root of rank r has a chain of r elements below it with strictly increasing
ranks (by rank_invariant). Including the root, that's r+1 distinct elements.

Prove by induction on rank:
- rank 0: root itself, subtree size >= 1 = rank + 1. ✓
- rank r > 0: by rank_invariant, there exists a child c with rank r-1
  whose find is this root. By induction, c's sub-chain has >= r elements.
  Plus the root: >= r+1 elements.

Actually simpler: you don't need the full subtree count. You just need
to exhibit r+1 distinct elements in the subtree. Build a witness chain:

```rust
proof fn lemma_rank_chain(
    parent: Seq<int>, rank: Seq<int>, n: nat, root: int,
) -> (chain: Seq<int>)
    requires
        spec_wf(...), spec_rank_invariant(...), spec_rank_bounded(...),
        spec_is_root(parent, root),
    ensures
        chain.len() == rank[root] + 1,
        forall|i: int| 0 <= i < chain.len() ==> 0 <= chain[i] < n,
        forall|i: int| 0 <= i < chain.len() ==>
            spec_pure_find(parent, rank, n, chain[i]) == root,
        // All distinct:
        forall|i: int, j: int| 0 <= i < j < chain.len() ==>
            chain[i] != chain[j],
    decreases rank[root],
```

Wait — building a ghost Seq in a proof fn is fine. But you might not need
this. An existence proof suffices:

### Simpler approach: count_with_root

Define a counting spec fn (like F* does):

```rust
pub open spec fn spec_count_with_root(
    parent: Seq<int>, rank: Seq<int>, n: nat, root: int, k: int,
) -> nat
    decreases n as int - k,
{
    if k >= n as int { 0 }
    else if spec_pure_find(parent, rank, n, k) == root { 
        1 + spec_count_with_root(parent, rank, n, root, k + 1) 
    }
    else { spec_count_with_root(parent, rank, n, root, k + 1) }
}
```

Then prove:
1. `lemma_count_disjoint`: for distinct roots root_x, root_y:
   `count_with_root(root_x) + count_with_root(root_y) <= n`
   (each element belongs to exactly one root, by pure_find determinism)

2. `lemma_root_in_own_tree`: `spec_pure_find(root) == root`, so
   `count_with_root(root) >= 1`

3. Therefore: `count_with_root(root_x) + count_with_root(root_y) >= 2`
   and `count_with_root(root_x) + count_with_root(root_y) <= n`, so n >= 2.

4. For the rank bound: you actually just need n >= 2 to prove rank + 1 < n
   when rank < n and n >= 2. Because rank < n and n >= 2 gives rank <= n-1,
   so rank + 1 <= n. But you need STRICT < n.

   Hmm, that's still not enough. rank could be n-1, giving rank+1 = n.

### The real argument

You need: two distinct roots of rank r implies r <= (n-2)/2, so r+1 <= n/2 < n.

The F* version proves this via `union_size_bound`: each root's COUNT >= 1,
two disjoint counts sum to <= n, so... that only gives n >= 2, not r+1 < n.

The actual F* proof that rank + 1 < n uses size >= 2^rank:
- size[root_x] >= 2^r (from size_rank_invariant)
- size[root_y] >= 2^r
- sizes are disjoint counts, sum <= n
- So 2^r + 2^r <= n → 2^(r+1) <= n → r+1 <= log2(n) < n (for n >= 2)

So you DO need the 2^rank bound. Read the F* Lemmas.fst carefully:

`~/projects/AlgoStar/autoclrs/ch21-disjoint-sets/CLRS.Ch21.UnionFind.Lemmas.fst`

Lines 66-90: `size_rank_invariant` definition
Lines 314-398: `pure_union_sized_preserves_invariant` (the inductive proof)
Lines 487-530: logarithmic rank bound

You don't need to track sizes at runtime. You can prove count_with_root(root)
>= 2^rank[root] purely in spec/proof, using induction. Then:

```
count_with_root(root_x) >= 2^r
count_with_root(root_y) >= 2^r
count_with_root(root_x) + count_with_root(root_y) <= n   (disjoint)
2^(r+1) <= n
r+1 <= log2(n) < n  (for any n >= 1, log2(n) < n)
```

This is the hard proof. It's the same proof the F* version does. Take your
time and get it right. This is the last error.

## Validation

```bash
scripts/validate.sh isolate UnionFind
```

Target: **0 errors.**

## Report

Write `plans/agent1-round181-report.md`.

## RCP

`git add -A && git commit -m "R181 Agent 1: prove rank_bounded — zero errors"`, then `git push`.
