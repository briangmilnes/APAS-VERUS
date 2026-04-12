# Agent 1, Round 196 Report

## Result

**Old Chap65/UnionFindStEph.rs retired; Kruskal rewired to UnionFindPCStEph.**

`src/UnionFind/` collapsed into `src/Chap65/`. The three new files are clean:

| # | Chap | File                       | Holes (R195 end) | Holes (R196 end) |
|---|------|----------------------------|------------------|------------------|
| 1 | 65   | UnionFindNoPCStEph.rs      | 0                | 0                |
| 2 | 65   | UnionFindPCStEph.rs        | 3 (opaque)       | 3 (opaque)       |
| 3 | 65   | UnionFindArrayStEph.rs     | 0                | 0                |
| 4 | 65   | KruskalStEph.rs            | 0                | 0                |

The "3 opaque" entries in `UnionFindPCStEph.rs` are `#[verifier::opaque]`
annotations from R195 (`spec_light_wf`, `spec_find_preserved`,
`spec_same_domain`) — by-design quantifier bundling, not real holes.

The retired `Chap65/UnionFindStEph.rs` had 3 `external_body` on
algorithmic logic (`union`, `lemma_union_merge_wf`,
`lemma_union_merge_wf_half_a`). Strict upgrade.

## Validation

| Check         | Count                | Notes                                        |
|---------------|----------------------|----------------------------------------------|
| validate (full) | 5674 verified, 0 errors | 234s, peak rust_verify 20GB                |
| validate (isolate Chap65) | 2616 verified, 0 errors | 166s                                |
| rtt           | 3776 passed, 0 failed | 46s, all chapters                            |
| ptt           | 221 passed, 0 failed  | 263s                                         |

## File moves (git mv preserves history)

```
src/UnionFind/UnionFindStEph.rs       → src/Chap65/UnionFindNoPCStEph.rs
src/UnionFind/UnionFindPCStEph.rs     → src/Chap65/UnionFindPCStEph.rs
src/UnionFind/UnionFindArrayStEph.rs  → src/Chap65/UnionFindArrayStEph.rs
src/Chap65/UnionFindStEph.rs          → (deleted; superseded by UnionFindNoPCStEph)
tests/Chap65/TestUnionFindStEph.rs    → tests/Chap65/TestUnionFindPCStEph.rs
```

`src/UnionFind/` removed (`rmdir`, was empty after the moves).

The inner module name in the renamed no-PC file was updated:
`pub mod UnionFindStEph` → `pub mod UnionFindNoPCStEph`.

## lib.rs / Cargo.toml

`src/lib.rs`:
- Removed top-level `pub mod UnionFind { ... }` block.
- Added the three UF files under `pub mod Chap65 { ... }`, ordered
  bottom-up before `KruskalStEph`/`PrimStEph`.

`Cargo.toml`:
- Renamed test entry `TestUnionFindStEph` → `TestUnionFindPCStEph`.

## API additions to UnionFindPCStEph (trait ensures only)

These were necessary to express Kruskal's loop invariants. The proof
bodies for each method already established the new facts; only the
trait/impl `ensures` clauses were extended. No new `assume`, `accept`,
or `external_body`. No proof body changes.

| Method    | Added ensures                                                                                |
|-----------|----------------------------------------------------------------------------------------------|
| `insert`  | `forall|z| z != v@ ==> (self.spec_contains(z) <==> old.spec_contains(z))`<br>`spec_n() == old.spec_n() + 1` |
| `union`   | `forall|z| old.spec_contains(z) <==> self.spec_contains(z)`<br>`spec_n() == old.spec_n()` |
| `equals`  | `forall|z| old.spec_contains(z) <==> self.spec_contains(z)`<br>`spec_n() == old.spec_n()` |

Without these, callers like Kruskal couldn't carry "vertex set is
preserved across `find`/`union`" through a loop. They follow trivially
from the existing internal proofs (every `union`/`equals` call goes
through two `find`s, which already preserve `spec_contains`; and
`union`'s `parent.insert(existing_key, _)` doesn't change `dom`).

## Kruskal API/spec changes

Trait/free fn signatures changed throughout (`UnionFindStEph<V>` →
`UnionFindPC<V>`, `spec_unionfindsteph_wf()` → `spec_wf()`,
`uf@.parent.contains_key(v)` → `uf.spec_contains(v)`).

Substantive changes:

1. **`uf_opaque_wrappers` nested module deleted.** Its purpose was to
   wrap the old `UnionFindStEph::spec_unionfindsteph_wf()` (an open
   conjunction of 13 quantifiers) so it wouldn't cross-fire with
   `LabEdge` broadcast groups. The new `UnionFindPC::spec_wf()` is
   already opaque-bundled (R195's `#[verifier::opaque]` on
   `spec_light_wf`), so an outer wrapper adds nothing. Deletion, per
   the prompt's explicit guidance ("this is a deletion, not a bypass —
   the new design makes them obsolete").

2. **Vertex-insertion loop.** PC's `insert` requires
   `!old(self).spec_contains(v@)`. The old NoPC `insert` accepted
   duplicates (its precondition was `spec_wf` only). New invariant
   added:

   ```
   vertex_seq@.no_duplicates(),  // from to_seq() ensures
   obeys_feq_view_injective::<V>(),
   forall|k| uf.spec_contains(k) ==> exists|j| j < vi && vertex_seq@[j]@ == k
   ```

   The precondition for the next `insert` is discharged by:
   - `lemma_reveal_view_injective::<V>()` to expose the body of
     `obeys_feq_view_injective`,
   - chasing back through the existential to find a duplicating index
     `j`, then to `vertex_seq@[j] == vertex_seq@[vi]` (view injectivity)
     contradicts `no_duplicates`.

3. **Empty-UF base case.** `UnionFindPC::new()` ensures `spec_n() == 0`
   but the contains forall isn't visible — needed
   `lemma_len0_is_empty` after `reveal(spec_light_wf)` to derive
   `forall|k| !uf.spec_contains(k)`.

4. **`lemma_sorted_edge_in_uf` simplified to `lemma_sorted_edge_in_graph_v`**:
   removed the `uf_parent_dom: Set` parameter and ensures. The lemma
   now just proves the edge endpoints are in `graph_V`; the loop
   invariant chains `graph_V` membership to `uf.spec_contains` in
   one step.

## Test file (RTT)

`tests/Chap65/TestUnionFindStEph.rs` → `TestUnionFindPCStEph.rs`,
fully rewritten against the new PC API:
- `UnionFindStEph` → `UnionFindPC`
- `UnionFindStEphTrait` → `UnionFindPCStEphTrait`
- `num_sets()` → `size()` (PC trait has no `num_sets`)
- `find` and `equals` are `&mut self` in PC (fine — all bindings
  declared `mut uf` already).
- The `size()` semantic also changed: old `num_sets` returned distinct
  component count; new `size()` returns `n` = total elements. Tests
  updated to assert against the new meaning.

24 tests, all pass.

## Bypassed / commented-out / out-of-scope

- Nothing bypassed.
- `PrimStEph.rs` doesn't import the old UF — left untouched per the prompt.
- No `accept`/`external_body`/`assume` added.
- The 3 `#[verifier::opaque]` annotations in `UnionFindPCStEph.rs`
  remain (these are R195 design, not new this round).

## RCP

```
git add -A
git commit -m "R196 Agent 1: relocate UnionFind into Chap65; Kruskal uses UnionFindPCStEph"
git push
```
