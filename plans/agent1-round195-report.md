# Agent 1, Round 195 Report

## Result

**`UnionFindPCStEph::find()` is fully proven.** Zero errors, zero `external_body`.

Isolated validate: `759 verified, 0 errors` (UnionFind chapter + transitive deps).

## What changed

The R195 prompt's diagnosis (closed predicates for the loop invariant) was
correct in spirit but `pub closed spec fn` only hides the body **across
modules**. Inside the same module Verus still unfolded `spec_light_wf`,
`spec_find_preserved`, and `spec_same_domain`, so Z3 saw all the inner
quantifiers and the parent-in-domain forall self-chained 25,809 times in
`find()`'s loop body — the rlimit timeout the prompt was trying to fix.

The actual fix is `#[verifier::opaque]`. That hides the body inside the
module too. With those three predicates opaque, Z3 sees them as black-box
booleans in the loop invariant; the matching loop disappears entirely.

### Concrete edits to `src/UnionFind/UnionFindPCStEph.rs`

1. Switched the three predicates from `pub closed spec fn` to
   `#[verifier::opaque] pub open spec fn`:
   - `spec_light_wf` (line 90)
   - `spec_find_preserved` (line 107)
   - `spec_same_domain` (line 117)

2. Added `reveal(spec_light_wf)` to every method body that needs to read
   parent-in-domain / rank invariant facts:
   - `new()` (one block)
   - `insert()` (one block, the existing big proof block)
   - `find_root()` (each proof block in the loop body, plus the
     post-loop dead-code block)
   - `union()` (every per-branch proof block: the post-find sanity
     block, the rank_u<v branch, the rank_u>v branch, the equal-rank
     branch)

3. Added `reveal(spec_light_wf); reveal(spec_same_domain);
   reveal(spec_find_preserved);` once before `find()`'s while loop so
   the initial invariant entry can be discharged.

4. Added the bridge `reveal(spec_find_preserved); reveal(spec_same_domain);`
   plus explicit equality assertions inside `find()`'s "curr == root"
   exit branch so postconditions resolve to old(self) facts.

5. Pulled the `let ghost orig_parent = self.parent@;` etc. **before** the
   `find_root(v)` call, and added these to the loop invariant:
   - `orig_parent == old(self).parent@`
   - `orig_rank == old(self).rank@`
   - `orig_n == old(self).spec_n()`
   This bridges the loop invariant to the function postcondition without
   relying on Verus tracking that `&self` calls preserve identity through
   trait dispatch (which it apparently doesn't, in this loop context).

6. Bumped `lemma_compress_iter`'s rlimit from 80 to 200. With the
   reveals inside the lemma body Z3 still uses ~150 of the 200, but it
   no longer has to fight the matching loop.

## What was not changed

- `union_sets` — already named `union` in the trait. No rename needed.
- No new `assume`, `admit`, `accept`, or `external_body` added.
- No proof code deleted; only added reveals and one ghost-capture
  reordering.

## Validation

```
verification results:: 759 verified, 0 errors
Elapsed: 141s
peak rust_verify RSS: 606MB, peak z3 RSS: 6886MB
```

Compare R194 isolate run: 758 verified, 1 error (find rlimit-exceeded).

## Note on the broader pattern

`pub closed spec fn` is a module-boundary visibility tool. To make a
spec function opaque to Z3 *within* its defining module — which is what
you want when you're stitching together a loop invariant out of bundled
predicates — use `#[verifier::opaque]` and pair it with `pub open spec fn`.
This is the same lever vstd uses for things like `endian::to_nat` and
`hash::contains_key`. Worth a feedback memory if it isn't already there.
