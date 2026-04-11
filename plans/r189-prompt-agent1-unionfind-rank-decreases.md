# R189 Prompt — Agent 1: Switch to rank-based decreases. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER read `src/Chap65/UnionFindStEph.rs`.** Clean-room.
7. **NEVER modify `Cargo.toml`, `src/lib.rs`, or `scripts/validate.sh`.**
8. **DO NOT REBASE. Your prior work is on your branch. Build on it.**

## Read all standards first

## Context

R188: 726 verified, 5 errors. feq() bridge works. Remaining 5 errors are
all from the fuel-based spec_pure_find — fuel gaps at small values prevent
proving root-ness and trichotomy.

You identified the fix yourself: replace fuel with rank-based decreases.
The array version uses `decreases n - rank[x]` and it works perfectly.
The reason you avoided rank-based decreases was the rank domain bridge.
But you now have the feq bridge pattern — the same approach works for
domain bridging.

## The change

Replace:
```rust
pub open spec fn spec_pure_find(parent: Map<VV, V>, fuel: nat, v: VV) -> VV
    decreases fuel,
```

With:
```rust
pub open spec fn spec_pure_find(parent: Map<VV, V>, rank: Map<VV, int>, n: nat, v: VV) -> VV
    recommends
        parent.dom().contains(v),
        rank.dom().contains(v),
    decreases n - rank[v] when
        parent.dom().contains(v)
        && rank.dom().contains(v)
        && rank[v] >= 0
        && rank[v] < n as int,
```

The `when` guard ensures the decreases clause is only checked when the
domains contain v. The rank_invariant guarantees `rank[parent[v]@] > rank[v]`
for non-roots, so `n - rank[parent[v]@] < n - rank[v]`.

### Why this fixes errors 1-4

With rank-based decreases, spec_pure_find is well-founded for ALL valid
inputs — no fuel bound to run out. So:
- `lemma_find_is_root`: recursive call with parent[v] has rank > rank[v],
  always sufficient. No fuel gap.
- `lemma_find_after_link`: trichotomy works for any depth. No fuel=2 limit.
- `find()` postcondition: `spec_pure_find(v)` is the root, period.
- `union_sets()` same_set: follows directly.

### The rank domain bridge in decreases_when

The `when` guard needs `rank.dom().contains(v)`. This is implied by
`parent.dom().contains(v)` + the wf domain-equality quantifier. But Verus
checks the `when` guard at each recursive call site. You need the guard to
be self-contained — include both domain checks explicitly:

```rust
decreases n - rank[v] when
    parent.dom().contains(v) && rank.dom().contains(v)
    && (parent[v]@ != v ==> parent.dom().contains(parent[v]@))
    && (parent[v]@ != v ==> rank.dom().contains(parent[v]@))
    && rank[v] >= 0 && rank[v] < n as int,
```

This way Verus verifies the recursive call's guard using only the guard's
own facts, without needing to instantiate external quantifiers.

## Also fix error 5: rank overflow

Two distinct roots → n >= 2. If you can prove count >= rank+1 per root
(from the array version's counting machinery), great. If not, try the
simpler argument:

- rank_invariant: non-root rank < parent's rank
- A root of rank r requires r ancestors with strictly increasing ranks
- That's r+1 elements in its subtree
- Two disjoint subtrees: 2*(r+1) <= n
- Therefore r+1 < n

Or just guard the increment: if `rank_u + 1 >= n` that's impossible
because it would require 2^(r+1) > n elements, but prove this as a
lemma rather than a runtime guard.

## What NOT to do

- Do NOT fall back to fuel-based spec_pure_find. Kill it.
- Do NOT rewrite the architecture. Keep the feq bridges from R188.
- Do NOT touch Cargo.toml or lib.rs.

## Validation

```bash
scripts/validate.sh isolate UnionFind
```

## Report

Write `plans/agent1-round189-report.md`.

## RCP

`git add -A && git commit -m "R189 Agent 1: rank-based decreases, kill fuel gaps"`, then `git push`.
