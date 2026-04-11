# R182 Prompt — Agent 1: Prove count additive invariant. Last real proof. AFK.

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

R181: 714 verified, 2 errors (same root cause), 0 assumes. You correctly
identified the fix: count >= rank+1 as an inductive invariant maintained
through union. The missing lemma is count additivity across parent updates.

## The one lemma you need

After linking root_a under root_b (setting parent[root_a] = root_b):

```
count_new(root_b) = count_old(root_a) + count_old(root_b)
```

Why: every element z has spec_pure_find(z) == some root. After linking
root_a → root_b:
- If z was in root_b's tree: find(z) is still root_b (unchanged parents)
- If z was in root_a's tree: find(z) now goes root_a → root_b (new link)
- If z was in neither: find(z) unchanged (different root)

So root_b's new count = old count of root_b + old count of root_a.
Root_a's new count = 0 (no longer a root — everything redirected).
All other roots: count unchanged.

### How to prove it

Prove pointwise: for each z in [0, n), show find_new(z) maps correctly.

```rust
proof fn lemma_count_additive(
    parent_old: Seq<int>, parent_new: Seq<int>,
    rank: Seq<int>, n: nat,
    root_a: int, root_b: int,
)
    requires
        spec_wf(old_view), spec_rank_invariant(old_view),
        spec_is_root(parent_old, root_a),
        spec_is_root(parent_old, root_b),
        root_a != root_b,
        // parent_new = parent_old with parent[root_a] = root_b
        parent_new == parent_old.update(root_a as int, root_b),
        // rank unchanged
    ensures
        spec_count_with_root(parent_new, rank, n, root_b, 0)
            == spec_count_with_root(parent_old, rank, n, root_a, 0)
             + spec_count_with_root(parent_old, rank, n, root_b, 0),
```

The proof is by induction on k (the counting index from 0 to n):

For each k:
- `find_new(k) == root_b` iff `find_old(k) == root_a || find_old(k) == root_b`

You already have `lemma_find_after_link` which proves exactly this trichotomy.
Use it pointwise in the counting induction.

### Alternatively: avoid counting entirely

You already know:
- count_with_root(root_x) >= 1 (root_x is in its own tree)
- count_with_root(root_y) >= 1 (root_y is in its own tree)
- count_with_root(root_x) + count_with_root(root_y) <= n (disjoint)

This gives n >= 2. You need rank + 1 < n, i.e., rank <= n - 2.

If you can prove count >= 2^rank (without tracking sizes), then:
2^rank + 2^rank <= n → 2^(rank+1) <= n → rank+1 <= log2(n).

For any n >= 2: log2(n) <= n-1, so rank+1 <= n-1, i.e., rank+1 < n. Done.

Proving count >= 2^rank by induction on union operations:
- After new(): count = 1 >= 2^0 = 1. ✓
- After union with rx < ry: root_b count grows by root_a count.
  count_new(root_b) >= count_old(root_a) + count_old(root_b)
  >= 2^rx + 2^ry >= 2^ry (rank unchanged). ✓
- After union with rx == ry: 
  count_new(root_b) >= 2^r + 2^r = 2^(r+1). New rank = r+1. ✓

But this requires the additive count lemma above. So you need it either way.

### Read the F* version

`~/projects/AlgoStar/autoclrs/ch21-disjoint-sets/CLRS.Ch21.UnionFind.Lemmas.fst`

Lines 160-210: `count_with_root_aux`, `count_disjoint_aux` — the counting
function and disjoint-sum proof by induction on k.

Lines 292-312: `union_size_bound` — connects count to size, proves sum <= n.

The key insight in the F* proof: `tree_membership_unique` (each element
belongs to exactly one root) makes the disjoint counting work. You have
this — pure_find is deterministic.

## Strategy

1. Prove `lemma_count_additive`: after link, winner's count = sum of both old counts.
   Use `lemma_find_after_link` pointwise in the counting induction.
2. Add `spec_count_ge_rank_plus_1` as a predicate to wf (or prove it separately).
3. Prove it's maintained through new() (trivial) and union (use count_additive).
4. In equal-rank union: invoke count >= rank+1 for both roots, sum <= n,
   therefore 2*(r+1) <= n, therefore r+1 <= n/2 < n. QED.
5. Validate: `scripts/validate.sh isolate UnionFind`

## Target: 0 errors.

## Report

Write `plans/agent1-round182-report.md`.

## RCP

`git add -A && git commit -m "R182 Agent 1: prove count invariant — zero errors"`, then `git push`.
