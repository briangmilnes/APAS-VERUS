# R193 Prompt — Agent 1: Prove find() compression. Three approaches. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER read `src/Chap65/UnionFindStEph.rs`.** Clean-room.
7. **NEVER modify `Cargo.toml`, `scripts/validate.sh`.**
8. **DO NOT REBASE. Build on your branch.**

## Read all standards first

## Context

R192: 716 verified, 1 error. The last error is find()'s compression loop —
rlimit from spec_pure_find triggers in the find-preservation invariant
interacting with the loop body's lemma results. Z3 RSS stays bounded
(4.7 GB at rlimit 150) but doesn't converge at rlimit 30.

The F* version avoids this because:
- `FStar.Classical.forall_intro` provides a clean context barrier
- `--fuel 1` limits spec fn unfolding per function
- Seq (arrays) have no domain quantifiers

Verus has none of these. We need a different structural approach.

## Three approaches to try (in order)

### Approach 1: Opaque the find-preservation invariant

Make the loop invariant's find-preservation a closed spec fn:

```rust
pub closed spec fn spec_find_preserved(
    parent_old: Map<VV, V>, rank_old: Map<VV, int>, n: nat,
    parent_new: Map<VV, V>, rank_new: Map<VV, int>,
) -> bool {
    forall|z: VV| parent_old.dom().contains(z) ==>
        spec_pure_find(parent_new, rank_new, n, z)
            == spec_pure_find(parent_old, rank_old, n, z)
}
```

Z3 can't unfold this in the loop body — it's opaque. The loop invariant
says `spec_find_preserved(orig_parent, orig_rank, n, self.parent@, self.rank_view())`.
At each iteration, prove it's maintained by calling a reveal lemma in an
isolated `assert ... by { reveal(spec_find_preserved); lemma_call(); }`.

The key: the reveal only happens inside the by-block. The rest of the loop
body never sees the spec_pure_find triggers.

### Approach 2: Separate find_root and compress

Split find() into two functions:

```rust
fn find_root(&self, v: &V) -> (root: V)
    requires self.spec_wf(), self.spec_contains(v@),
    ensures root@ == self.spec_find(v@), self.spec_is_root(root@);

fn compress(&mut self, v: &V, root: &V)
    requires
        old(self).spec_wf(),
        old(self).spec_contains(v@),
        root@ == old(self).spec_find(v@),
        old(self).spec_is_root(root@),
    ensures
        self.spec_wf(),
        // find preserved for all elements
        forall|z: VV| old(self).spec_contains(z) ==>
            self.spec_find(z) == old(self).spec_find(z),
        // domain unchanged
        forall|k: VV| self.parent@.dom().contains(k)
            <==> old(self).parent@.dom().contains(k);
```

`find_root` is already proven (it's the current &self find from
UnionFindStEph.rs). `compress` does the path walk and updates parents.
The trait's `find` calls both:

```rust
fn find(&mut self, v: &V) -> (root: V) {
    let root = self.find_root(v);
    self.compress(v, &root);
    root
}
```

Now the compression proof is isolated in `compress()` — a separate
function with its own Z3 context. The find-preservation ensures on
compress doesn't leak into find_root's context.

For compress itself: the loop still needs the find-preservation invariant,
but now it's the ONLY thing compress proves. No find_root logic in scope.

### Approach 3: Prove compression for the whole path at once

Instead of proving each compression step individually in a loop, build
the full path from v to root as a ghost Seq, then prove the entire
batch compression in one lemma:

```rust
proof fn lemma_compress_path(
    parent_old: Map<VV, V>, rank: Map<VV, int>, n: nat,
    path: Seq<VV>,  // [v, parent[v], parent[parent[v]], ..., root]
    root: VV,
)
    requires
        // path is valid: path[0] = v, path[last] = root, each step follows parent
        // parent_new = parent_old with path[i] → root for all i
    ensures
        // find preserved for all z
```

The proof: by induction on path length. Compressing path[0..k] preserves
find for all z (from compress_preserves_find per node), and composing k
compressions preserves find (since each one preserves the prior results).

The advantage: no loop invariant. One lemma call with the full path.
The disadvantage: building the ghost path Seq in exec code.

## Which to try first

Try Approach 2 first — it's the cleanest separation. find_root is already
proven. compress gets its own Z3 context. If compress still hits rlimit,
try Approach 1 (opaque invariant) inside compress.

If both fail, try Approach 3 (batch path proof).

## Also: rename union_sets to union

The caller (Kruskal) uses `uf.union(&u, &v)`. Rename `union_sets` to
`union` in the trait and impl.

## Validation

```bash
scripts/validate.sh isolate UnionFind
```

Target: 0 errors.

## Report

Write `plans/agent1-round193-report.md`.

## RCP

`git add -A && git commit -m "R193 Agent 1: prove find() compression — zero errors"`, then `git push`.
