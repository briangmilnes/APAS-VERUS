# R179 Prompt — Agent 1: Fix 5 UnionFind errors. AFK.

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

R178 built the array-based UnionFind: 708 verified, 5 errors, 0 assumes.
Your job is to fix all 5 errors. The code is in
`src/UnionFind/UnionFindArrayStEph.rs`.

## The 5 errors

### Error 1: lemma_pure_find_in_bounds (line 194)

Fuel=1 non-root case: `spec_pure_find(parent, n, x, 1)` returns `parent[x]`.
You need to prove `parent[x]` is in bounds. This is directly from `spec_wf`:
`parent[x] < n`. Add `assert(0 <= parent[x] < n as int)` using the wf
precondition.

### Errors 2-3: lemma_find_after_link (lines 305, 308)

The z==root_a case: after linking root_a → root_b, `spec_pure_find(z)`
must return root_b. Z3 can't automatically unfold spec_pure_find through
two steps: root_a's parent is now root_b, and root_b is a root.

Fix: add explicit unfolding. After the parent update, assert:
```
assert(new_parent[root_a as int] == root_b as int);
assert(new_parent[root_b as int] == root_b as int);
// Therefore spec_pure_find(new_parent, n, root_a, fuel) == root_b
```

You may need `reveal_with_fuel(spec_pure_find, 2)` or manual unfolding
with a helper that shows the two-step chase.

### Errors 4-5: union equal-rank rank_bounded (lines 633, 673)

After `rank[root_x] += 1` in the equal-rank case, you need to prove
`rank[root_x] + 1 < n`. The current proof can't establish this.

The F* version proves this via the size >= 2^rank invariant:
- Two distinct roots root_x, root_y each have subtrees of size >= 2^rank
- Total elements >= 2^rank + 2^rank = 2^(rank+1)
- Since total <= n: 2^(rank+1) <= n, therefore rank+1 <= log2(n) < n

You don't need to implement the full size tracking. A simpler argument:
- rank_bounded: forall|i| rank[i] < n
- Two distinct roots with equal rank r: at least 2 elements with rank >= r
- By rank_invariant, each root's subtree has depth >= r, so >= r+1 elements
- Two disjoint subtrees: >= 2*(r+1) elements <= n
- Therefore r+1 <= n/2 < n

Or even simpler: if root_x and root_y are distinct roots with rank r, then
there are at least 2 elements (root_x and root_y themselves), so n >= 2,
and you just need r < n-1, i.e., r+1 < n. Prove: if all elements had
rank r, they'd all be roots with rank r and there'd be at least 2 of them
(root_x, root_y), but ranks must be < n by invariant, so r < n, and since
n >= 2 and r < n, we get r+1 <= n. But we need strict < n.

Actually the simplest proof: rank_invariant says non-root rank < parent's
rank. Rank is bounded by n-1 (since rank < n). A root of rank r requires
a chain of r ancestors in its subtree (by rank_invariant, each level has
strictly increasing rank). So the subtree has >= r+1 nodes. Two disjoint
subtrees of rank r have >= 2*(r+1) nodes. Since total <= n: r+1 <= n/2,
so r < n/2 < n, and r+1 <= n/2 <= n-1 < n.

Read `~/projects/AlgoStar/autoclrs/ch21-disjoint-sets/CLRS.Ch21.UnionFind.Lemmas.fst`
lines 260-312 for how the F* version proves this (`union_size_bound`).

## Validation

```bash
scripts/validate.sh isolate UnionFind
```

Target: 0 errors. If you can't fix error 4-5, fix 1-3 and report what
you tried for 4-5. Do NOT add external_body.

## Report

Write `plans/agent1-round179-report.md`.

## RCP

`git add -A && git commit -m "R179 Agent 1: fix UnionFind errors (−N errors)"`, then `git push`.
