# R185 Prompt — Agent 1: REAL proofs for HashMap UnionFind. AFK.

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

## What you delivered in R184

7 verified items with toy specs. Let me be blunt: **this is not acceptable.**

- `find()` ensures "returns element in domain" — USELESS. Kruskal needs
  `find(v) == canonical root of v's set`. Without that, find is just a
  random domain element.
- `union_sets()` preserves wf — BUT DOESN'T MERGE SETS. The entire point
  of union is that `find(u) == find(v)` afterward. Your spec doesn't say that.
- No `spec_pure_find`. No `spec_same_set`. No merge correctness. No stability.
- Runtime `if rank < usize::MAX` guard instead of proving rank_bounded. That's
  dodging the proof, not doing the proof.
- 7 new verified items vs the array version's 718. You didn't prove anything.

You gutted the specs to make verification pass. That is the exact antipattern
CLAUDE.md calls out: "DO NOT WEAKEN ensures TO MAKE PROOFS EASIER." An
`external_body` with a strong spec is worth more than a real body with a
gutted spec. At least the external_body admits the debt.

## What you must deliver

The array version (`src/UnionFind/UnionFindArrayStEph.rs`) has these specs
and they all PROVE. Port them to HashMap:

### spec_pure_find

```rust
pub open spec fn spec_pure_find(
    parent: Map<VV, VV>, rank: Map<VV, int>, v: VV,
) -> VV
    recommends parent.dom().contains(v),
    decreases rank[v] when parent.dom().contains(v),
```

Recursive root-chasing on the Map. `parent[v] == v` → return v. Else
recurse on `parent[v]`. Decreases on `rank[v]` because rank_invariant
guarantees `rank[parent[v]] > rank[v]` for non-roots.

This is the SAME spec_pure_find as the array version. Seq<int> indexing
becomes Map<VV, VV> lookup. That's it.

### find() ensures

```rust
fn find(&self, v: &V) -> (root: V)
    requires self.spec_wf(), self.parent@.dom().contains(v@),
    ensures
        self.parent@.dom().contains(root@),
        root@ == spec_pure_find(self.parent@, self.rank_view(), v@),
        spec_is_root(self.parent@, root@),
```

Not "returns element in domain." Returns THE ROOT.

### union_sets() ensures

```rust
fn union_sets(&mut self, u: &V, v: &V)
    requires old(self).spec_wf(), ...,
    ensures
        self.spec_wf(),
        // MERGE: u and v are in the same set afterward
        spec_pure_find(self.parent@, self.rank_view(), u@)
            == spec_pure_find(self.parent@, self.rank_view(), v@),
        // STABILITY: disjoint elements unchanged
        forall|z: VV| self.parent@.dom().contains(z)
            && spec_pure_find(old(self).parent@, old(self).rank_view(), z)
                != spec_pure_find(old(self).parent@, old(self).rank_view(), u@)
            && spec_pure_find(old(self).parent@, old(self).rank_view(), z)
                != spec_pure_find(old(self).parent@, old(self).rank_view(), v@)
            ==> spec_pure_find(self.parent@, self.rank_view(), z)
                == spec_pure_find(old(self).parent@, old(self).rank_view(), z),
```

Not "wf preserved." MERGE + STABILITY. This is what Kruskal needs.

### rank_bounded — PROVE IT

No `if rank < usize::MAX` runtime guards. Prove `rank[v] < n` for all v.
You already proved this in the array version using count_with_root and
size_rank_inv. Port it. Add ghost `elements: Ghost<Seq<VV>>` for counting.

## How to proceed

1. Read `src/UnionFind/UnionFindArrayStEph.rs` — every spec fn, every
   proof fn, every ensures clause. That is your target.
2. Add `spec_pure_find` on Maps with `decreases rank[v]`.
3. Add `spec_is_root`, `spec_same_set`, `spec_wf` with full invariants
   including rank_bounded and rank_invariant.
4. Add ghost `elements: Ghost<Seq<VV>>` for counting infrastructure.
5. Port `lemma_pure_find_in_bounds`, `lemma_pure_find_is_root`.
6. Port `lemma_find_after_link` (the trichotomy).
7. Port `lemma_count_additive`, `lemma_count_disjoint`.
8. Port `lemma_rank_lt_n_minus_1` (the 2*(r+1) <= n argument).
9. Prove `new`, `insert`, `find`, `union_sets` with FULL specs.

Each of these exists in the array version and proves. The Map version uses
`parent.dom().contains(k)` where the array version uses `0 <= k < n`. The
proof structure is identical.

## The =~= rule (again)

**NEVER write `a =~= b` where a or b is a Map, Map domain, or Set.**

Use pointwise quantifiers:
```rust
forall|k: VV| old_parent.dom().contains(k) <==> new_parent.dom().contains(k)
```

## Validation

```bash
scripts/validate.sh isolate UnionFind
```

Target: **0 errors with REAL specs.** Not toy specs. Not gutted ensures.
If you can't prove something, leave the error and report what failed.
Do NOT weaken the spec to make verification pass.

## Report

Write `plans/agent1-round185-report.md`.

## RCP

`git add -A && git commit -m "R185 Agent 1: HashMap UnionFind with real proofs"`, then `git push`.
