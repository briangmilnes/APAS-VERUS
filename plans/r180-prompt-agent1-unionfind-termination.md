# R180 Prompt — Agent 1: Fix spec_pure_find termination + remaining errors. AFK.

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

R179: 709 verified, 3 errors, 0 assumes. The 3 errors are interconnected.

## Error 1: spec_pure_find termination (the blocker)

`spec_pure_find` is an `open spec fn` that recurses on `parent[x]`.
`decreases n` doesn't work because n doesn't change in the recursive call.
The real measure is rank (strictly decreases along parent chain), but
Verus needs to see the decrease syntactically.

### The fix: decreases rank[x] with rank as a parameter

```rust
pub open spec fn spec_pure_find(
    parent: Seq<int>, rank: Seq<int>, n: nat, x: int
) -> int
    recommends
        0 <= x < n as int,
        parent.len() == n,
        rank.len() == n,
        // rank_invariant holds
    decreases rank[x] when 0 <= x < n as int,
{
    if x < 0 || x >= n as int { x }
    else if parent[x] == x { x }
    else { spec_pure_find(parent, rank, n, parent[x]) }
}
```

The `when` guard makes Verus only check the decreases when the recommends
hold. Since rank_invariant guarantees `rank[parent[x]] > rank[x]` for
non-roots, and `rank[x] >= 0`, the measure `rank[x]` strictly decreases.

If Verus rejects `decreases rank[x]` because it can't see the inequality
from recommends, try:

```rust
decreases (n - rank[x]) when 0 <= x < n as int,
```

This works because rank[x] increases toward the root, so n - rank[x]
decreases. And n - rank[x] >= 0 since rank[x] < n (from spec_rank_bounded).

### Alternative: back to fuel

If decreases on rank doesn't work, go back to the fuel approach from R178
but fix errors 2-3 with explicit `reveal_with_fuel(spec_pure_find, 2)` or
a two-step unfolding helper:

```rust
proof fn lemma_find_two_steps(parent: Seq<int>, n: nat, x: int, fuel: nat)
    requires
        spec_wf(...), 0 <= x < n, parent[x] != x,
        parent[parent[x]] == parent[x],  // parent[x] is a root
        fuel >= 2,
    ensures
        spec_pure_find(parent, n, x, fuel) == parent[x],
{
    // Step 1: x -> parent[x]
    // Step 2: parent[x] is root, done
    reveal_with_fuel(spec_pure_find, 2);
}
```

## Error 2: lemma_find_after_link root_a case (line 197)

This error is caused by error 1 — spec_pure_find can't unfold because of
the termination failure. Once termination is fixed, this should resolve.
If not, the two-step chase needs explicit assertions:

```rust
// After linking root_a -> root_b:
assert(parent_new[root_a as int] == root_b as int);   // by the link
assert(parent_new[root_b as int] == root_b as int);   // root_b unchanged
// Therefore find(root_a) = find(parent[root_a]) = find(root_b) = root_b
```

## Error 3: rank_bounded equal-rank case (line 463)

After `rank[root_x] += 1`, need `new_rank < n`. Currently can't prove this.

### Simple proof: two distinct roots imply n >= 2

You don't need size >= 2^rank for rank_bounded. You need: if two distinct
roots have rank r, then r + 1 < n. Proof:

1. root_x != root_y (precondition)
2. Both are valid indices: 0 <= root_x, root_y < n
3. n >= 2 (two distinct elements exist)
4. rank[root_x] == rank[root_y] == r
5. By rank_invariant: every non-root has rank < its parent's rank
6. The maximum possible rank for a root is n-1 (chain of n elements)
7. But if root_x has rank r, its subtree needs >= r+1 elements
8. root_y's subtree also needs >= r+1 elements (distinct subtrees)
9. Total: 2*(r+1) <= n, so r+1 <= n/2 <= n-1, so r+1 < n

The key insight for step 7: a root of rank r requires a chain of r
distinct ancestors in its subtree (each with strictly increasing rank
by rank_invariant). That's r+1 elements including the root itself.

Prove this as a lemma:

```rust
proof fn lemma_subtree_size_bound(
    parent: Seq<int>, rank: Seq<int>, n: nat, root: int,
)
    requires
        spec_wf(...), spec_rank_invariant(...),
        spec_is_root(parent, root),
    ensures
        // number of elements whose find == root is >= rank[root] + 1
```

Then for the equal-rank case: two disjoint subtrees each of size >= r+1,
total <= n, therefore r+1 <= n/2, therefore r+1 < n (since n >= 2*(r+1)
implies n > r+1).

Read `~/projects/AlgoStar/autoclrs/ch21-disjoint-sets/CLRS.Ch21.UnionFind.Lemmas.fst`
lines 140-312 for the F* approach (count_with_root + union_size_bound).

## Order

1. Fix spec_pure_find termination (error 1)
2. Fix lemma_find_after_link (error 2 — likely auto-fixes with error 1)
3. Fix rank_bounded (error 3 — subtree size argument)
4. Validate: `scripts/validate.sh isolate UnionFind`

## Report

Write `plans/agent1-round180-report.md`.

## RCP

`git add -A && git commit -m "R180 Agent 1: fix UnionFind termination + remaining errors"`, then `git push`.
