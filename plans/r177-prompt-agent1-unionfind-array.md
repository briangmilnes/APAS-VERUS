# R177 Prompt — Agent 1: Array-based UnionFind from scratch. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, or `accept`.**
6. **NEVER read `src/Chap65/UnionFindStEph.rs`.** This is a clean-room implementation.

## Read all standards first

## Reference: F* CLRS Chapter 21 implementation

Read these three files for the data structures, specs, and proof strategy.
They are an F*/Pulse union-find that uses the same array-based approach.
Translate the ideas to Verus — do NOT copy F* syntax.

0. `~/projects/AlgoStar/autoclrs/ch21-disjoint-sets/README.md`
   — Overview: fully proven, zero admits/assumes, summary table of all properties.
1. `~/projects/AlgoStar/autoclrs/ch21-disjoint-sets/CLRS.Ch21.UnionFind.Spec.fst`
   — Pure spec: uf_forest type, pure_find, pure_union, rank_invariant, compress.
2. `~/projects/AlgoStar/autoclrs/ch21-disjoint-sets/CLRS.Ch21.UnionFind.Impl.fsti`
   — API signatures: make_set, find_set, union with full pre/postconditions.
3. `~/projects/AlgoStar/autoclrs/ch21-disjoint-sets/CLRS.Ch21.UnionFind.Lemmas.fst`
   — Rank bound proofs: size ≥ 2^rank, log n bound, union preserves invariant.
   Key: uses `--split_queries always` and rlimit 5. No =~= anywhere.

## Goal

Build a fresh array-based Union-Find in `src/UnionFind/UnionFindArrayStEph.rs`.
This is NOT a port of the existing Chap65 UnionFind. Do NOT read Chap65.
Build from scratch using the data structures and approach described below.

## Data structure

Three `Vec<usize>` arrays, all length n. Elements are integers in [0, n):

```rust
pub struct UnionFindArray {
    pub parent: Vec<usize>,   // parent[i] = i's parent (i if root)
    pub rank: Vec<usize>,     // rank[i] = upper bound on subtree height
}
```

No HashMap. No generic element type. No ghost `roots` Map. Elements are
just indices 0..n-1. This avoids all Map domain `=~=` extensional equality
issues that plague the Chap65 version.

## View

```rust
pub struct UnionFindArrayView {
    pub parent: Seq<int>,
    pub rank: Seq<int>,
    pub n: nat,
}
```

## Spec: pure_find

Root-chasing on the spec-level parent Seq. Decreases on rank (rank_invariant
guarantees parent's rank > child's rank for non-roots):

```rust
pub open spec fn spec_pure_find(parent: Seq<int>, n: nat, x: int) -> int
    decreases n,  // or decreases rank[x] if rank is available
```

`spec_pure_find(parent, n, x)` returns the root of x's tree by following
parent pointers. If `parent[x] == x`, return x. Otherwise recurse on `parent[x]`.

## Well-formedness

```rust
pub open spec fn spec_wf(uf: UnionFindArrayView) -> bool {
    uf.n > 0
    && uf.parent.len() == uf.n
    && uf.rank.len() == uf.n
    && forall|i| 0 <= i < n ==> 0 <= parent[i] < n      // parent in bounds
    && forall|i| 0 <= i < n ==> rank[i] >= 0             // rank non-negative
}
```

## Rank invariant

```rust
pub open spec fn spec_rank_invariant(uf: UnionFindArrayView) -> bool {
    forall|i| 0 <= i < n && parent[i] != i ==> rank[i] < rank[parent[i]]
}
```

This guarantees termination of pure_find (rank strictly decreases along
non-root parent chains).

## Operations

### new(n) -> Self

Create n-element forest: `parent[i] = i`, `rank[i] = 0` for all i.
Ensures: wf, rank_invariant, every element is a root.

### find(&self, x) -> root

Follow parent pointers to root. No path compression (add later).
Requires: wf, rank_invariant, 0 <= x < n.
Ensures: wf unchanged, root is a root, pure_find(x) == root.

### union(&mut self, x, y)

Find roots of x and y. If same, done. Otherwise, union by rank:
- If rank[root_x] < rank[root_y]: parent[root_x] = root_y
- If rank[root_x] > rank[root_y]: parent[root_y] = root_x
- If equal: parent[root_y] = root_x, rank[root_x] += 1

Requires: wf, rank_invariant, 0 <= x,y < n.
Ensures: wf, rank_invariant, same_set(x, y), stability for disjoint elements.

### num_sets(&self) -> count

Count roots (elements where parent[i] == i).

### size(&self) -> n

Return number of elements.

## Key proof obligations

1. **pure_find terminates**: rank strictly decreases along parent chain (from rank_invariant).
2. **pure_find returns root**: by induction on rank.
3. **pure_find in bounds**: by induction, parent[i] < n.
4. **union preserves wf**: parent update stays in bounds, rank update is +1 at most.
5. **union preserves rank_invariant**: only the loser's parent changes (to the winner root),
   and winner's rank >= loser's rank (by construction).
6. **union merges sets**: pure_find(x) == pure_find(y) after union.
7. **union stability**: elements not in x's or y's tree are unchanged.

## Important: decreases for pure_find

The tricky part is the `decreases` clause. You need rank_invariant to prove
termination. Options:
- (a) `decreases rank[x]` — requires rank as a parameter or in scope
- (b) `decreases n` — overly conservative but simple, needs careful proof
- (c) Use a fuel/bound parameter

Option (a) is cleanest. You can make pure_find take the full view:

```rust
pub open spec fn spec_pure_find(uf: UnionFindArrayView, x: int) -> int
    recommends spec_wf(uf), spec_rank_invariant(uf), 0 <= x < uf.n as int,
    decreases uf.rank[x],
```

## Style

Follow APAS-VERUS conventions: trait with specs, impl with bodies, bottom-up
ordering, Table of Contents standard, named return values, `spec_unionfindarraysteph_wf`
for the combined wf predicate.

## Do NOT

- Do NOT read or reference `src/Chap65/UnionFindStEph.rs`
- Do NOT use HashMap, Map, or any map type
- Do NOT use ghost `roots` field
- Do NOT use `=~=` on any Map/Set domain
- Do NOT add `external_body` — prove everything or leave the error

## File

`src/UnionFind/UnionFindArrayStEph.rs` — a scaffold exists, overwrite it entirely.

The module is already in lib.rs:
```rust
pub mod UnionFind {
    pub mod UnionFindArrayStEph;
}
```

## Validation

```bash
scripts/validate.sh isolate Chap02
```

(UnionFind has no chapter feature — it compiles with any isolate target.)

## Report

Write `plans/agent1-round177-report.md`.

## RCP

`git add -A && git commit -m "R177 Agent 1: array-based UnionFind from scratch"`, then `git push`.
