# R184 Prompt — Agent 1: Prove HashMap UnionFind. PBOGH. AFK.

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

R183 built the scaffold: types, trait, exec bodies, compiles with 5 errors.
All errors are missing proofs. Your job: PROVE EVERYTHING. You have the
array version as your template — the proofs transfer.

## PBOGH — Prove Big or Go Home

You proved the array version in 5 rounds with zero assumes. This HashMap
version uses the same algorithms. The proofs are the same proofs. Port them.
Do NOT deliver a scaffold with "next steps." Deliver a proven module.

## What you have

`src/UnionFind/UnionFindStEph.rs` — scaffold from R183. 5 verification errors.

`src/UnionFind/UnionFindArrayStEph.rs` — the proven array version. READ THIS.
It has every proof you need. Your job is to adapt each proof from Seq<int>
indexing to Map<V::V, V::V> key lookups.

## The 5 errors to fix

### 1. new() — wf not proved

Array version: parent[i]=i, rank[i]=0 for all i in [0,n). Trivial.
Map version: empty maps. Even more trivial — empty map satisfies all
quantifiers vacuously. Just assert the conjuncts.

### 2. insert() — wf preservation not proved

Array version doesn't have insert. But the proof is straightforward:
- New element v not in old domain → add parent[v]=v, rank[v]=0
- All old elements unchanged → old invariants preserved for them
- New element is a root with rank 0 → satisfies rank_invariant, rank_bounded

### 3. find() — termination + correctness

Array version uses `decreases n - rank[x]`. Map version: same, with
`self@.rank[v@]` instead of `rank[x]`. The loop invariant is identical.

Add ghost `elements: Ghost<Seq<V::V>>` now. You need it for count_with_root.
Initialize in new() as empty, push in insert(). The ghost field gives you
a Seq to iterate over for counting, exactly like the array version's [0,n).

### 4. union_sets() — wf + merge + stability

Port directly from the array version. The three branches (rx < ry, rx > ry,
rx == ry) have identical proof structure. Map updates via
`self.parent.insert(loser, winner)` correspond to `parent.set(loser_idx, winner_idx)`.

For the =~= problem: the array version NEVER uses =~= on domains. It uses
pointwise assertions:
```rust
assert(forall|i| 0 <= i < n ==> new_parent[i] == if i == loser { winner } else { old_parent[i] });
```

The Map version should do the same:
```rust
assert(forall|k: V::V| self@.parent.dom().contains(k) ==>
    new_parent[k] == if k == loser@ { winner } else { old_parent[k] });
```

NO `=~=` on Map domains. Use `forall|k| dom.contains(k) ==> ...` everywhere.

### 5. rank overflow in equal-rank

Same as array version: need count_with_root >= rank+1, two disjoint roots
sum <= n, therefore 2*(r+1) <= n, r+1 < n. You proved this already.
Port it using the ghost `elements` Seq for counting.

## Ghost elements field

Add this to the struct:

```rust
pub struct UnionFind<V: StT + Hash> {
    pub parent: HashMapWithViewPlus<V, V>,
    pub rank: HashMapWithViewPlus<V, usize>,
    pub ghost elements: Seq<V::V>,
}
```

Maintain:
- new(): `elements = Seq::empty()`
- insert(v): `elements = elements.push(v@)`
- find(): elements unchanged
- union(): elements unchanged

The wf includes:
- `forall|i| 0 <= i < elements.len() ==> parent.dom().contains(elements[i])`
- `forall|k| parent.dom().contains(k) ==> elements.contains(k)`
- `elements` has no duplicates

This gives a bijection between elements indices and parent domain keys,
letting count_with_root work over elements indices exactly like the array version.

## The =~= rule

**NEVER write `a =~= b` where a or b is a Map or Map domain.**

Instead:
- Domain equality: `forall|k| a.dom().contains(k) <==> b.dom().contains(k)`
- Map equality: `forall|k| a.dom().contains(k) ==> a[k] == b[k]` + domain eq
- Or use a closed predicate

This is the entire lesson of R172-R175. Do not repeat the mistake.

## Validation

```bash
scripts/validate.sh isolate UnionFind
```

If UnionFind isolate doesn't include HashMapWithViewPlus, use:
```bash
scripts/validate.sh
```

Target: **0 errors.**

## Report

Write `plans/agent1-round184-report.md`.

## RCP

`git add -A && git commit -m "R184 Agent 1: prove HashMap UnionFind — zero errors"`, then `git push`.
