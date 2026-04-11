# R183 Prompt — Agent 1: HashMap-based UnionFind with proven proof techniques. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, or `accept`.**
6. **NEVER read `src/Chap65/UnionFindStEph.rs`.** Clean-room.

## Read all standards first

## Context

R178-R182: you built `src/UnionFind/UnionFindArrayStEph.rs` — array-based
UnionFind, 718 verified, 0 errors, 0 assumes. The proof architecture works:
fuel-based or rank-decreasing spec_pure_find, count_with_root, size_rank_inv,
count_additive for the rank_bounded proof.

But the array version uses `Vec<usize>` with fixed-size elements [0,n).
Kruskal needs generic elements (`V: StT + Hash`) inserted dynamically.
Your job: build a HashMap-based UnionFind that has the same API as Kruskal
expects, using the proof techniques you just proved work.

## What to build

Create `src/UnionFind/UnionFindStEph.rs` (new file, same directory as
the array version). Add it to lib.rs under the UnionFind module.

### Data structure

```rust
pub struct UnionFind<V: StT + Hash> {
    pub parent: HashMapWithViewPlus<V, V>,
    pub rank: HashMapWithViewPlus<V, usize>,
}
```

No ghost `roots` map. No ghost `elements` Seq. Use `spec_pure_find` on
the parent Map for root lookups, same as the array version uses spec_pure_find
on the parent Seq.

### View

```rust
pub struct UnionFindView<V: View> {
    pub parent: Map<V::V, V::V>,
    pub rank: Map<V::V, int>,
    pub n: nat,  // parent.dom().len()
}
```

### Operations (matching Kruskal's needs)

1. **new()** — empty UnionFind
2. **insert(&mut self, v: V)** — add v as singleton set (parent[v]=v, rank[v]=0)
3. **find(&self, v: &V) -> V** — root-chasing, no path compression
4. **union(&mut self, u: &V, v: &V)** — union by rank
5. **equals(&mut self, u: &V, v: &V) -> bool** — same set check
6. **num_sets(&self) -> usize** — count distinct roots

### Specs

- `spec_pure_find(parent: Map, v: V::V) -> V::V` — recursive root-chasing
  on the Map. Decreases on rank[v].
- `spec_wf` — parent.dom() == rank.dom(), all parent values in dom, rank >= 0
- `spec_rank_invariant` — non-root rank < parent's rank
- `spec_same_set(view, u, v)` — pure_find(u) == pure_find(v)

### The =~= problem and how to handle it

The Chap65 version died on `=~=` cross-firing with `contains_key`.
Your approach:

1. **NEVER use `=~=` on Map domains in function ensures.** Instead use:
   - `self@.parent.dom().contains(k) <==> old(self)@.parent.dom().contains(k)`
     as a forall quantifier, OR
   - A closed predicate `spec_same_dom(a, b)` that wraps the extensional eq

2. **Use closed predicates for all domain equality facts.** Same pattern as
   `spec_elem_at_neq` from R173 — Z3 never sees the raw `=~=`.

3. **Use the array version's proof architecture** — count_with_root for the
   rank_bounded proof, directional reveals for domain containment.

4. **Keep spec_pure_find simple** — decreases on rank[v] with a `when` guard.
   The Map version needs `parent.dom().contains(v)` in the guard.

### Key difference from array version

The Map version has dynamic insertion. The array version starts at size n.
For the rank_bounded proof, you need count_with_root defined over the
domain of the parent Map, not over [0,n). This is trickier because Map
domains don't have a natural iteration order.

One approach: define count_with_root over the `elements` — but you don't
have an elements Seq. Another: use `parent.dom().len()` as n, and note
that finite maps have countable domains.

Actually, you CAN keep a ghost `elements: Ghost<Seq<V::V>>` that tracks
insertion order. This gives you a Seq to count over, same as the array
version. The ghost field costs nothing at runtime.

```rust
pub struct UnionFind<V: StT + Hash> {
    pub parent: HashMapWithViewPlus<V, V>,
    pub rank: HashMapWithViewPlus<V, usize>,
    pub ghost elements: Seq<V::V>,  // insertion-order tracking for counting
}
```

With `elements`, count_with_root iterates over `elements` indices exactly
like the array version iterates over [0,n). The proofs transfer directly.

### What to prove

Same as array version:
- pure_find terminates, returns root, in bounds (in domain)
- new establishes wf
- insert preserves wf, adds singleton
- find returns pure_find result
- union preserves wf, merges sets, stability for disjoint elements
- rank_bounded via count_with_root + size_rank_inv

### external_body budget: ZERO

You proved the array version with zero. This version should too. If the
=~= cross-fire returns, use closed predicates. Do NOT fall back to
external_body.

## Validation

```bash
scripts/validate.sh isolate UnionFind
```

## Report

Write `plans/agent1-round183-report.md`.

## RCP

`git add -A && git commit -m "R183 Agent 1: HashMap-based UnionFind with proven techniques"`, then `git push`.
