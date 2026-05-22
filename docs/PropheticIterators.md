<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Prophetic Iterators in APAS-VERUS

This document is the user-facing reference for iteration under the verus
0.2026.05.21 prophetic iterator model (verus PR #2163, "New approach to
specifying iterators via a prophetic sequence encoding"). It supersedes
`docs/APAS-VERUSIterators.rs`, which described the pre-#2163
`ForLoopGhostIterator` design that has been removed from the for-loop
desugaring.

The canonical, verified reference is the standard itself:

- Standard: `src/standards/prophetic_iterators_standard.rs`
- PTT (7 loop forms): `rust_verify_test/tests/standards/Proveprophetic_iterators_standard.rs`
- Worked experiments: `src/experiments/prophetic_iter_{slice_direct,custom_struct,consume}.rs`

The migration that converts every existing APAS collection to this model is
planned in **`plans/verus-0.2026.05.21-iterator-migration.md`** — see that
document for the rounds, risks, and decisions. This file is the technical
reference; the plan is the schedule.

## Two iteration styles — and nothing else

There are exactly two iterator styles, both defined in the standard:

- **delegated** — `iter()` returns a std library iterator
  (`std::slice::Iter`, `std::vec::IntoIter`, `std::collections::hash_set::Iter`,
  or `std::collections::hash_map::Iter`). vstd already provides
  `IteratorSpecImpl` for all four, so the collection writes only an `ensures`
  pinning `IteratorSpec::remaining(&it)` to its contents.
- **custom** — `iter()` returns a type that implements `IteratorSpecImpl` by
  hand: six spec fns (`obeys_prophetic_iter_laws`, `remaining` [prophetic],
  `will_return_none`, `decrease`, `initial_value_relation`, `peek`), a
  `#[verifier::type_invariant]`, a `closed` constructor behind an `open`
  `#[verifier::when_used_as_spec]` spec, and a plain `Iterator::next` whose
  spec lives in the trait impl (no `ensures`).

"Chained", "flatten", "snapshot", "slice", and "hash" are descriptions of *how
a collection obtains its sequence today*, not iterator styles. Of 71 collection
iterators in APAS-VERUS, **68 are delegated, 3 are custom** (the AVLTreeSeq
lazy variants).

## Why proposed = was

The migration swaps each iterator's *type* (custom `XxxIter` wrapper → a std
iterator) and removes the ghost-only `ForLoopGhostIterator` machinery. It does
**not** rewrite the algorithmic body of `iter()`/`next()` — the in-order flatten
(`Node::in_order`), `self.seq.iter()`, `push_left_iter`
(`src/Chap37/AVLTreeSeqStEph.rs:441`), and `tree.nth` are all untouched. So
complexity is preserved by construction.

## Complexity by variant — 7 distinct profiles

| # | Variant (iterator field today) | iter() | next() | Space | First k then break | Full traversal | Files |
|---|--------------------------------|--------|--------|-------|--------------------|----------------|-------|
| 1 | `slice::Iter` wrap (Vec-backed) | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) | 14 |
| 2 | flatten → `vec::IntoIter` (in-order walk) | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) | 22 |
| 3 | flatten → `Vec` snapshot + index | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) | 13 |
| 4 | lazy node-ref stack (AVLTreeSeq StEph/StPer) | O(lg n)→O(lg n) | O(1)†→O(1)† | O(lg n)→O(lg n) | O(lg n+k)→O(lg n+k) | O(n)→O(n) | 2 |
| 5 | lazy tree index (AVLTreeSeq base) | O(1)→O(1) | O(lg n)→O(lg n) | O(1)→O(1) | O(k lg n)→O(k lg n) | O(n lg n)→O(n lg n) | 1 |
| 6 | chained — re-expose inner iterator | inherits | inherits | inherits | inherits | inherits | 15 |
| 7 | `hash_set`/`hash_map::Iter` | O(1)→O(1) | O(1)†→O(1)† | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) | 4 |

† amortized over the traversal. Row 6 inherits its backing collection's profile
(adding only O(1) per-`next()` forwarding, asymptotically free): Relation,
Mapping, and the 8 graphs inherit row 7; ArraySet and the 3 Tables inherit
row 1; OrderedTableMtPer inherits row 2.

Evidence read from source:

- Row 1 — `ArraySeqStEph::iter()` is `ArraySeqStEphIter { inner: self.seq.iter() }`.
- Row 2 — `BSTPlainStEph::into_iter()` is `let traversal = self.root.in_order(); traversal.into_iter()`.
- Row 3 — `BSTAVLMtEph::iter()` annotated `Work O(n), Span O(n) — snapshot iteration`.
- Row 4 — `AVLTreeSeqStEph::iter()` is `push_left_iter(&mut it, &self.root)`, annotated `Work O(lg n)`.
- Row 5 — `AVLTreeSeq::next()` is `self.tree.nth(self.pos); self.pos += 1`.
- Row 7 — `SetStEph::iter()` annotated `Work O(1) — Creates iterator handle`.

## Per-file complexity — all 71 iterators, was → proposed

Every row carries the same five complexity columns as the variant table above,
filled in for that specific file. The "Iterator field today" column is the
field of the current `*Iter` struct, read from source.

| # | Chap | File | Style | Iterator field today | iter() | next() | Space | First k then break | Full traversal |
|---|------|------|-------|----------------------|--------|--------|-------|--------------------|----------------|
| 1 | 05 | SetStEph.rs | delegated | HashSetWithViewPlusIter | O(1)→O(1) | O(1)†→O(1)† | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 2 | 05 | SetMtEph.rs | delegated | HashSetWithViewPlusIter | O(1)→O(1) | O(1)†→O(1)† | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 3 | 05 | RelationStEph.rs | delegated | SetStEphIter | O(1)→O(1) | O(1)†→O(1)† | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 4 | 05 | MappingStEph.rs | delegated | RelationStEphIter | O(1)→O(1) | O(1)†→O(1)† | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 5 | 06 | DirGraphStEph.rs | delegated | SetStEphIter | O(1)→O(1) | O(1)†→O(1)† | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 6 | 06 | DirGraphMtEph.rs | delegated | SetStEphIter | O(1)→O(1) | O(1)†→O(1)† | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 7 | 06 | UnDirGraphStEph.rs | delegated | SetStEphIter | O(1)→O(1) | O(1)†→O(1)† | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 8 | 06 | UnDirGraphMtEph.rs | delegated | SetStEphIter | O(1)→O(1) | O(1)†→O(1)† | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 9 | 06 | LabDirGraphStEph.rs | delegated | SetStEphIter | O(1)→O(1) | O(1)†→O(1)† | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 10 | 06 | LabDirGraphMtEph.rs | delegated | SetStEphIter | O(1)→O(1) | O(1)†→O(1)† | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 11 | 06 | LabUnDirGraphStEph.rs | delegated | SetStEphIter | O(1)→O(1) | O(1)†→O(1)† | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 12 | 06 | LabUnDirGraphMtEph.rs | delegated | SetStEphIter | O(1)→O(1) | O(1)†→O(1)† | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 13 | 17 | MathSeq.rs | delegated | std::slice::Iter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 14 | 18 | ArraySeq.rs | delegated | std::slice::Iter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 15 | 18 | ArraySeqStEph.rs | delegated | std::slice::Iter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 16 | 18 | ArraySeqStPer.rs | delegated | std::slice::Iter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 17 | 18 | ArraySeqMtEph.rs | delegated | std::slice::Iter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 18 | 18 | ArraySeqMtEphSlice.rs | delegated | std::slice::Iter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 19 | 18 | ArraySeqMtPer.rs | delegated | std::slice::Iter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 20 | 18 | LinkedListStEph.rs | delegated | std::slice::Iter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 21 | 18 | LinkedListStPer.rs | delegated | std::slice::Iter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 22 | 19 | ArraySeqStEph.rs | delegated | std::slice::Iter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 23 | 19 | ArraySeqStPer.rs | delegated | std::slice::Iter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 24 | 19 | ArraySeqMtEph.rs | delegated | std::slice::Iter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 25 | 19 | ArraySeqMtEphSlice.rs | delegated | std::slice::Iter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 26 | 23 | BalBinTreeStEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 27 | 23 | PrimTreeSeqStPer.rs | delegated | std::slice::Iter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 28 | 37 | AVLTreeSeq.rs | custom | &tree + position index | O(1)→O(1) | O(lg n)→O(lg n) | O(1)→O(1) | O(k lg n)→O(k lg n) | O(n lg n)→O(n lg n) |
| 29 | 37 | AVLTreeSeqStEph.rs | custom | Vec<&Node> stack (lazy) | O(lg n)→O(lg n) | O(1)†→O(1)† | O(lg n)→O(lg n) | O(lg n+k)→O(lg n+k) | O(n)→O(n) |
| 30 | 37 | AVLTreeSeqStPer.rs | custom | Vec<&Node> stack (lazy) | O(lg n)→O(lg n) | O(1)†→O(1)† | O(lg n)→O(lg n) | O(lg n+k)→O(lg n+k) | O(n)→O(n) |
| 31 | 37 | AVLTreeSeqMtPer.rs | delegated | Vec<T> + position index | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 32 | 37 | BSTAVLStEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 33 | 37 | BSTBBAlphaStEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 34 | 37 | BSTPlainStEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 35 | 37 | BSTRBStEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 36 | 37 | BSTSplayStEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 37 | 37 | BSTAVLMtEph.rs | delegated | Vec<T> + position index | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 38 | 37 | BSTBBAlphaMtEph.rs | delegated | Vec<T> + position index | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 39 | 37 | BSTPlainMtEph.rs | delegated | Vec<T> + position index | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 40 | 37 | BSTRBMtEph.rs | delegated | Vec<T> + position index | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 41 | 37 | BSTSetAVLMtEph.rs | delegated | Vec<T> + position index | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 42 | 37 | BSTSetBBAlphaMtEph.rs | delegated | Vec<T> + position index | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 43 | 37 | BSTSetPlainMtEph.rs | delegated | Vec<T> + position index | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 44 | 37 | BSTSetRBMtEph.rs | delegated | Vec<T> + position index | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 45 | 37 | BSTSetSplayMtEph.rs | delegated | Vec<T> + position index | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 46 | 38 | BSTParaStEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 47 | 39 | BSTTreapStEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 48 | 39 | BSTTreapMtEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 49 | 39 | BSTParaTreapMtEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 50 | 39 | BSTSetTreapMtEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 51 | 40 | BSTKeyValueStEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 52 | 40 | BSTReducedStEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 53 | 40 | BSTSizeStEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 54 | 41 | ArraySetStEph.rs | delegated | ArraySeqStEphIter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 55 | 41 | AVLTreeSetStEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 56 | 41 | AVLTreeSetStPer.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 57 | 41 | AVLTreeSetMtEph.rs | delegated | Vec<T> + position index | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 58 | 41 | AVLTreeSetMtPer.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 59 | 41 | OrdKeyMap.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 60 | 42 | TableStEph.rs | delegated | ArraySeqStEphIter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 61 | 42 | TableMtEph.rs | delegated | ArraySeqMtEphIter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 62 | 42 | TableStPer.rs | delegated | ArraySeqStPerIter | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 63 | 43 | OrderedSetStEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 64 | 43 | OrderedSetStPer.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 65 | 43 | OrderedSetMtEph.rs | delegated | Vec<T> + position index | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 66 | 43 | OrderedTableStEph.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 67 | 43 | OrderedTableStPer.rs | delegated | std::vec::IntoIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 68 | 43 | OrderedTableMtEph.rs | delegated | Vec<Pair> + position index | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 69 | 43 | OrderedTableMtPer.rs | delegated | OrderedTableStPerIter | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) |
| 70 | — | vstdplus/hash_set_with_view_plus.rs | delegated | std::collections::hash_set::Iter | O(1)→O(1) | O(1)†→O(1)† | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |
| 71 | — | vstdplus/hash_map_with_view_plus.rs | delegated | std::collections::hash_map::Iter | O(1)→O(1) | O(1)†→O(1)† | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) |

The only point a change was possible is rows 28–30 (the 3 lazy AVLTreeSeq
files): flattening them would push `iter()` to O(n), space to O(n), and
falsify the existing `Alg Analysis: Work O(lg n)` annotations. They therefore
stay custom — port the lazy iterator to a from-scratch `IteratorSpecImpl` per
`CountIter` in the standard. `AVLTreeSeqMtPer` (row 31) already flattens; leave
it. Row 28's O(n lg n) full traversal is suboptimal today; the migration
preserves it as-is. Whether to improve it is a separate APAS cost-spec
question, out of scope here.

## Loop forms

The PTT exercises seven loop forms — `for-borrow-iter`, `for-borrow-into`,
`for-consume`, `loop-borrow`, `loop-consume`, `for-custom`, `loop-custom`.
Reference invariants for each appear in `Proveprophetic_iterators_standard.rs`.
Two recurring gotchas worth keeping in mind:

- A manual `loop` drives `decreases` with the non-prophetic
  `IteratorSpec::decrease(&it.iter)->0` — the prophetic `it.seq()` cannot
  appear in `decreases`, and the loop must draw its conclusion before `break`.
- `it` is not in scope after a `for` loop. Post-loop facts come from the
  iterator's `#[verifier::when_used_as_spec]` form, not from naming `it`.

## See also

- **Migration plan:** `plans/verus-0.2026.05.21-iterator-migration.md` —
  upgrade fixes, the 178-error breakdown, the old → new API map, file
  inventory, the proposed round structure, and §10 which mirrors the tables
  above as part of the migration schedule.
- **Standard (verified):** `src/standards/prophetic_iterators_standard.rs`.
- **PTT:** `rust_verify_test/tests/standards/Proveprophetic_iterators_standard.rs`.
- **Experiments:** `src/experiments/prophetic_iter_slice_direct.rs`,
  `src/experiments/prophetic_iter_custom_struct.rs`,
  `src/experiments/prophetic_iter_consume.rs`.
- **Upstream verus reference:** `~/projects/verus/examples/guide/iterators.rs`
  (the canonical `VecIterator` example) and
  `~/projects/verus/source/vstd/std_specs/iter.rs` (the `IteratorSpec` /
  `IteratorSpecImpl` external-trait extension and `VerusForLoopWrapper`).
