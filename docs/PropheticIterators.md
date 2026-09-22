<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Prophetic Iterators in APAS-VERUS

This document is the user-facing reference for iteration under the verus
0.2026.09.13 prophetic iterator model (introduced at 0.2026.05.21 by verus
PR #2163, "New approach to specifying iterators via a prophetic sequence
encoding"; `initial_value_relation` removed by PR #2739). The pre-#2163
`ForLoopGhostIterator` design is gone from the for-loop desugaring;
`docs/APAS-VERUSIterators.rs` restates the current model as a component
checklist, and `docs/StandardsUpgrade.md` records the 09.13 measurements
behind the rules below.

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
  hand: five spec fns (`obeys_prophetic_iter_laws`, `remaining` [prophetic],
  `will_return_none` [prophetic], `decrease`, `peek`), a
  `#[verifier::type_invariant]`, a `closed` constructor behind an `open`
  `#[verifier::when_used_as_spec]` spec, and a plain `Iterator::next` whose
  spec lives in the trait impl (no `ensures`). Optional extensions:
  `ExactSizeIteratorSpecImpl::exact_len` and
  `DoubleEndedIteratorSpecImpl::peek_back`.

A collection that wraps another collection (Mapping over Relation, the tables
over ArraySeq) re-exposes the inner collection's iterator: same type, same
postconditions restated over the outer view. Only a module that must own its
iterator type writes an adaptor whose spec fns forward to the inner
iterator's; see `src/standards/wrapping_iterators_standard.rs`.

## Constructor postconditions

Every `iter()`, `into_iter()` and custom constructor carries three
postconditions, the guide's triple (`~/projects/verus/examples/guide/iterators.rs`):

1. `IteratorSpec::remaining(&it) == <contents>` — the prophetic sequence, as
   `self@.as_ref()` for a borrowing iterator and `self@` for a consuming one;
2. the same sequence tied to the non-prophetic contents that `peek` reads —
   `vstd::std_specs::slice::into_iter_elts(it) == self@` for `slice::Iter`,
   `vstd::std_specs::vec::into_iter_elts(it) == self@` for `vec::IntoIter`,
   `IteratorSpec::remaining(&it) == it.elts()` for a custom type;
3. `IteratorSpec::decrease(&it) is Some` — lets a `for` loop prove
   termination without an explicit `decreases`.

When the constructor is a *trait method* and returns a type whose
`IteratorSpecImpl` and `Iterator::next` are verified in this crate (an
adaptor or a custom iterator), clauses 1 and 3 name the inner std iterator
(`IteratorSpec::remaining(&it.inner)`), not the returned type; naming the
returned type there makes its `next` fail verification on 09.13. An inherent
method or a free function may name the returned type directly.

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
Reference invariants for each appear in `Proveprophetic_iterators_standard.rs`;
the six-pattern collection template is in `Proveiterators_standard.rs`.

A `for` loop names its wrapper and reasons through `it.index()` (items
consumed), the prophetic `it.seq()` (the whole sequence), and `it.history()`
(items consumed so far):

    for x in it: coll.iter()
        invariant
            it.seq() == orig.as_ref(),
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len()
                ==> #[trigger] collected@[i] == *it.seq()[i],
    { collected.push(*x); }
    assert(collected@ =~= orig);   // it.index() == it.seq().len() after the loop

A manual `loop` runs on the iterator's own `next()`, whose contract is the
`IteratorSpec` one (vstd/std_specs/iter.rs:35), with a ghost counter `pos`
of items consumed and the wrapper's `wf_inner` written out as two invariants
over the prophetic `IteratorSpec::remaining(&it)` (r211,
`src/experiments/prophetic_manual_loop_next.rs`, 10 verified, 0 errors):

    let mut it = coll.iter();
    let ghost mut pos: int = 0;
    loop
        invariant
            IteratorSpec::obeys_prophetic_iter_laws(&it),
            IteratorSpec::decrease(&it) is Some,
            0 <= pos <= orig.len(),
            IteratorSpec::remaining(&it).len() == orig.len() - pos,
            forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == orig[pos + i],
            collected.len() == pos,
            forall|i: int| 0 <= i < collected.len()
                ==> #[trigger] collected@[i] == orig[i],
        decreases IteratorSpec::decrease(&it)->0,
    {
        let ghost old_pos = pos;
        match it.next() {
            Some(x) => {
                proof { pos = pos + 1; assert(orig[old_pos] == *x); }
                collected.push(*x);
            },
            None => { assert(pos == orig.len()); assert(collected@ =~= orig); break; },
        }
    }

It does not use `VerusForLoopWrapper`: vstd declares
`#[cfg(verus_keep_ghost)] pub mod std_specs;` (vstd/vstd.rs:96), so a loop
written on the wrapper does not compile under `cargo` (r208 wrote 119 such
loops; r211 rewrote them, `docs/RunTimeTestsRestored.md`). The `verus!`
macro desugars a `for` loop through the wrapper in both build modes, so
`for` loops are unaffected. A `skip`-based single invariant
(`remaining(&it).unref() == orig.skip(pos)`) does not verify without the
seq `skip` lemmas (`src/experiments/prophetic_manual_loop_next_skip.rs`).

The `decreases` rule: the prophetic `remaining()` (and a `for` loop's
`it.seq()`) may not appear in `decreases`. A manual loop measures the
iterator's non-prophetic `IteratorSpec::decrease(&it)->0`, and draws its
conclusion before `break`, because the prophetic equality does not survive
the break; a `return` inside the loop sees only the invariants, so a fact
such as `orig == a.seq@` must be one of them. `it` is not in scope after a
`for` loop; post-loop facts come from the iterator's
`#[verifier::when_used_as_spec]` form, not from naming `it`.

## See also

- **Migration plan:** `plans/verus-0.2026.05.21-iterator-migration.md` —
  upgrade fixes, the 178-error breakdown, the old → new API map, file
  inventory, the proposed round structure, and §10 which mirrors the tables
  above as part of the migration schedule.
- **Standards (verified):** `src/standards/prophetic_iterators_standard.rs`
  (both styles), `src/standards/iterators_standard.rs` (delegated, with the
  loop idioms), `src/standards/wrapping_iterators_standard.rs` (re-expose and
  adaptor).
- **PTTs:** `rust_verify_test/tests/standards/Proveprophetic_iterators_standard.rs`,
  `Proveiterators_standard.rs`, `Provewrapping_iterators_standard.rs`.
- **09.13 measurements:** `docs/StandardsUpgrade.md`.
- **Experiments:** `src/experiments/prophetic_iter_slice_direct.rs`,
  `src/experiments/prophetic_iter_custom_struct.rs`,
  `src/experiments/prophetic_iter_consume.rs`,
  `src/experiments/prophetic_manual_loop_next.rs` (the manual loop on
  `next()`), `src/experiments/prophetic_manual_loop_next_skip.rs` (FAILS).
- **Upstream verus reference:** `~/projects/verus/examples/guide/iterators.rs`
  (the canonical `VecIterator` example) and
  `~/projects/verus/source/vstd/std_specs/iter.rs` (the `IteratorSpec` /
  `IteratorSpecImpl` external-trait extension and `VerusForLoopWrapper`).
