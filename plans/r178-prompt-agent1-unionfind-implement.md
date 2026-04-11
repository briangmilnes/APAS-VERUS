# R178 Prompt — Agent 1: Implement array-based UnionFind. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, or `accept`.**
6. **NEVER read `src/Chap65/UnionFindStEph.rs`.** Clean-room implementation.

## Read all standards first

## Context

R177 set up the isolate feature (`scripts/validate.sh isolate UnionFind`).
The scaffold file exists at `src/UnionFind/UnionFindArrayStEph.rs` but
contains only a rough sketch. Your job is to write the real implementation
and PROVE IT.

## Reference: F* CLRS Chapter 21

Read these for the data structures, specs, and proof strategy:

0. `~/projects/AlgoStar/autoclrs/ch21-disjoint-sets/README.md`
1. `~/projects/AlgoStar/autoclrs/ch21-disjoint-sets/CLRS.Ch21.UnionFind.Spec.fst`
2. `~/projects/AlgoStar/autoclrs/ch21-disjoint-sets/CLRS.Ch21.UnionFind.Impl.fsti`
3. `~/projects/AlgoStar/autoclrs/ch21-disjoint-sets/CLRS.Ch21.UnionFind.Lemmas.fst`

Translate the approach to Verus. Do NOT copy F* syntax.

## What to build

Overwrite `src/UnionFind/UnionFindArrayStEph.rs` with a complete, proven
array-based Union-Find. The data structure:

```rust
pub struct UnionFindArray {
    pub parent: Vec<usize>,
    pub rank: Vec<usize>,
}
```

Elements are indices [0, n). No HashMap, no Map, no ghost roots, no =~=.

### Operations to implement and prove

1. **new(n)** — create n-element forest, parent[i]=i, rank[i]=0
2. **find(&self, x)** — root-chasing, no path compression
3. **union(&mut self, x, y)** — union by rank
4. **num_sets(&self)** — count roots
5. **size(&self)** — return n

### Specs needed

- `spec_pure_find(view, x)` — recursive root-chasing on spec Seq, decreases on rank
- `spec_wf(view)` — lengths match, parent in bounds, rank non-negative
- `spec_rank_invariant(view)` — non-root rank < parent's rank
- `spec_is_root(parent, x)` — parent[x] == x
- `spec_same_set(view, x, y)` — pure_find(x) == pure_find(y)

### Key proofs

- pure_find terminates (rank decreases)
- pure_find returns root and is in bounds
- new establishes wf + rank_invariant
- find returns spec_pure_find result
- union preserves wf + rank_invariant
- union merges sets (same_set after)
- union stability (disjoint elements unchanged)

### Style

Follow APAS-VERUS conventions: trait with specs, impl with bodies, Table of
Contents standard, named return values. Single file, everything inside verus!.

## Target: zero external_body, zero assume, zero admit

The F* version achieves zero admits and zero assumes. This Verus version
should too. If a proof is hard, keep working on it. Do NOT add external_body
to skip it. Leave the verification error and report what's failing.

## Validation

```bash
scripts/validate.sh isolate UnionFind
```

## Report

Write `plans/agent1-round178-report.md`.

## RCP

`git add -A && git commit -m "R178 Agent 1: implement and prove array-based UnionFind"`, then `git push`.
