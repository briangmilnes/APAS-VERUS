# R152 Agent 2 — Build OrdKeyMap<K,V> in Chap38. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `docs/ordered-bst-refactor.md` — the design doc for this task.
Read `src/Chap38/BSTParaStEph.rs` — the ParamBST you're wrapping.
Read `src/Chap43/OrderedTableStEph.rs` — the consumer you're simplifying.
Pay close attention to the 18 bridge lemmas and `bst_*_by_key` functions in
OrderedTableStEph — those are what moves into OrdKeyMap.

Report file: `plans/r152-agent2-ordkeymap-report.md`

## Problem

OrderedTableStEph has 3,809 proof lines. Most are bridge proof converting
ParamBST's `Set<(K::V, V::V)>` view to OrderedTable's `Map<K::V, V::V>` view.
The same bridge is duplicated in OrderedTableStPer (2,632 lines). The bridge
belongs in a shared module between ParamBST and OrderedTable.

## What to build

Create `src/Chap38/OrdKeyMap.rs` — an ordered key-value map backed by
ParamBST<Pair<K,V>> with View = Map<K::V, V::V>.

### Phase 1: Struct, View, wf, bridge lemmas

Create the module with:

```rust
pub mod OrdKeyMap {
    pub struct OrdKeyMap<K, V> {
        pub inner: ParamBST<Pair<K, V>>,
    }

    impl<K: View, V: View> View for OrdKeyMap<K, V> {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> {
            spec_pair_set_to_map(self.inner@)
        }
    }
}
```

Move the 15 bridge lemmas from OrderedTableStEph into OrdKeyMap:
- `lemma_pair_set_to_map_dom_finite`
- `lemma_pair_set_to_map_len`
- `lemma_pair_in_set_map_contains`
- `lemma_map_contains_pair_in_set`
- `lemma_key_unique_insert`
- `lemma_key_unique_remove`
- `lemma_key_unique_subset`
- `lemma_key_unique_empty`
- `lemma_key_unique_disjoint_union`
- `lemma_set_to_map_union_root`
- `lemma_view_gen_subset`
- `lemma_view_gen_insert`
- `lemma_view_gen_union`
- `lemma_cmp_equal_congruent`
- `lemma_sorted_keys_pairwise_distinct`

Also move `spec_pair_set_to_map`, `spec_key_unique_pairs_set`, and
`spec_set_pair_view_generated` spec functions.

Validate: `scripts/validate.sh isolate Chap38`

### Phase 2: OrdKeyMapTrait with Map-level ensures

Add trait methods that delegate to ParamBST and bridge to Map:

- `find(&self, key: &K) -> Option<&V>` — delegates to `self.inner.find()`
- `insert(&mut self, key: K, val: V)` — delegates to `self.inner.insert()`
- `delete(&mut self, key: &K)` — delegates to `self.inner.delete()`
- `split(&self, key: &K) -> (Self, Option<V>, Self)` — delegates to `self.inner.split()`
- `union(&self, other: &Self) -> Self` — delegates to `self.inner.union()`
- `intersect(&self, other: &Self) -> Self` — delegates to `self.inner.intersect()`
- `difference(&self, other: &Self) -> Self` — delegates to `self.inner.difference()`
- `size(&self) -> usize`
- `is_empty(&self) -> bool`

Each method body: call `self.inner.operation()`, then ~5-20 lines of bridge
proof using the lemmas from Phase 1. The ensures speak in Map terms.

Do NOT implement `next`, `prev`, `rank`, `select` yet — those are harder and
can be Phase 3.

Validate after each method: `scripts/validate.sh isolate Chap38`

### Phase 3 (stretch): next/prev/rank/select

These currently live as 300+ line `bst_*_by_key` functions in OrderedTable.
They re-walk the BST with Map-level invariants at each node. Moving them to
OrdKeyMap means:

1. Copy the function from OrderedTableStEph
2. Change the self type from `&ParamBST<Pair<K,V>>` to `&OrdKeyMap<K,V>`
3. The internal logic uses `self.inner` to access the BST
4. The ensures speak in Map terms (same as OrderedTable's current ensures)

This is a copy-and-adapt, not a rewrite. The proof logic stays the same.

If Phase 3 takes too long, stop after Phase 2 and report what's done.

### Phase 4 (DO NOT DO THIS ROUND): Simplify OrderedTable

After OrdKeyMap is built and verified, OrderedTable can be simplified to
delegate to OrdKeyMap. This is a separate round — do NOT modify OrderedTable
in this round. Build OrdKeyMap, verify it standalone, report.

## File structure

Follow the module standard. OrdKeyMap is a single-type file:

```
//  Table of Contents
//  Section 1. module
//  Section 2. imports
//  Section 3. broadcast use
//  Section 6. spec fns
//  Section 7. proof fns/broadcast groups
//  Section 8. traits
//  Section 9. impls
//  Section 14. derive impls outside verus!
```

## Register in lib.rs and Cargo.toml

Add `pub mod OrdKeyMap;` to the Chap38 module in lib.rs.
Add OrdKeyMap to the Chap38 feature deps in Cargo.toml if needed.

## Validation

Use `scripts/validate.sh isolate Chap38` during development.
Run full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrderedTableStEph or any Chap43 file this round.
- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken any ensures.
- All existing RTTs must pass.
- Bridge lemmas: COPY from OrderedTableStEph, do not delete from there.

## When done

RCP.
