# R186 Prompt — Agent 1: Fix 7 domain-tracking errors. AFK.

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

R185: real specs, real architecture, 7 errors. 6 from domain-tracking
(parent.dom vs rank.dom bridging), 1 from rlimit on equal-rank branch.
The architecture is correct. These are mechanical fixes.

## The domain-tracking problem

Array version: `0 <= x < n` gives both `parent[x]` and `rank[x]` access.
Map version: `parent.dom().contains(k)` and `rank.dom().contains(k)` are
separate facts connected by the wf invariant:

```rust
forall|k: VV| parent.dom().contains(k) <==> rank.dom().contains(k)
```

Z3 doesn't fire this quantifier automatically for specific keys. You need
to assert it explicitly at each call site.

### Fix pattern

Before every lemma call that needs `rank.dom().contains(v)`:

```rust
// Bridge: parent domain → rank domain
assert(self@.parent.dom().contains(v@));  // already known
// From wf: parent.dom() == rank.dom()
assert(self@.rank.dom().contains(v@));    // now Z3 has it
```

Or better: write a one-line bridge lemma:

```rust
proof fn lemma_dom_bridge(parent: Map<VV, V>, rank: Map<VV, usize>, k: VV)
    requires
        forall|j: VV| parent.dom().contains(j) <==> rank.dom().contains(j),
        parent.dom().contains(k),
    ensures
        rank.dom().contains(k),
{}  // Z3 instantiates the forall with j=k
```

Call it wherever you need the bridge. It's zero cost — the body is empty,
Z3 just needs the trigger instantiation.

## The rlimit on equal-rank

The equal-rank branch needs the rank_bounded proof: `rank + 1 < n`.
You haven't ported the counting machinery yet. Two options:

### Option A: Port counting (correct, hard)

Port from the array version:
- Ghost `elements: Ghost<Seq<VV>>` field
- `spec_count_with_root` over elements indices
- `lemma_count_additive`, `lemma_count_disjoint`
- `spec_size_rank_inv`: count >= rank + 1
- `lemma_rank_lt_n_minus_1`: 2*(r+1) <= n → r+1 < n

This is the full proof. It's what the array version does.

### Option B: Assert n >= 2 (simpler, sufficient)

In the equal-rank branch, you have two distinct roots u_root and v_root,
both in parent.dom(). Therefore parent.dom().len() >= 2. Since
rank_bounded says rank < n, and n >= 2: if rank == n-1, then both roots
have rank n-1, but rank_invariant requires each non-root to have rank <
its parent's rank, so a root of rank n-1 needs a chain of n-1 ancestors
with strictly increasing ranks... Actually this still needs the counting
argument. Go with Option A.

But if you're short on time: you could prove just `n >= 2` (two distinct
elements in domain) and then prove that two distinct roots of equal rank r
can't have r == n-1 because that would require 2n elements. This is the
lightweight version of the counting argument.

## Order

1. Write `lemma_dom_bridge` (or inline the assertions). Fix all 6 domain errors.
2. Validate — should be down to 1 error (equal-rank rlimit).
3. Port ghost elements + counting for rank_bounded.
4. Validate — target 0 errors.

## Validation

```bash
scripts/validate.sh isolate UnionFind
```

## Report

Write `plans/agent1-round186-report.md`.

## RCP

`git add -A && git commit -m "R186 Agent 1: fix domain tracking + rank_bounded"`, then `git push`.
