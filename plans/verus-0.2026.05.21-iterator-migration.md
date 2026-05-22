# Migration Plan — Verus 0.2026.04.20 → 0.2026.05.21 (Prophetic Iterators)

Status: SCOPING + Round 0 pilot artifacts done (see §9). Direction: forward.
Author: scoping pass, 2026-05-22.

## 9. Round 0 pilot — done (2026-05-22)

The new iterator standard, its PTT, and three exploratory experiments are
written and verified standalone against verus 0.2026.05.21 (the crate itself
still does not compile, so `validate.sh`/`ptt.sh` cannot run yet — these were
verified by invoking verus directly on the self-contained files).

| # | Artifact | Verified |
|---|----------|----------|
| 1 | `src/experiments/prophetic_iter_slice_direct.rs` | 8, 0 errors |
| 2 | `src/experiments/prophetic_iter_custom_struct.rs` | 6, 0 errors |
| 3 | `src/experiments/prophetic_iter_consume.rs` | 7, 0 errors |
| 4 | `src/standards/prophetic_iterators_standard.rs` | 7, 0 errors |
| 5 | `Proveprophetic_iterators_standard.rs` (PTT, 7 forms) | 21, 0 errors* |

\* PTT logic verified via a standalone harness running the 7 test bodies
cross-module against the real standard module; the PTT file itself runs once
the crate compiles.

Decisions resolved:
- **Custom wrappers: drop them.** A `Vec`-backed collection returns
  `std::slice::Iter` / `std::vec::IntoIter` directly (delegated iteration). Only
  non-slice collections (trees) keep a hand-written iterator implementing
  `IteratorSpecImpl` (custom iteration). This roughly halves the per-collection
  migration work.

New findings to fold into the per-chapter rounds:
- `closed` spec fns (`remaining()`, `elts()`) are opaque cross-module, so an
  iterator constructor's `ensures` must state `IteratorSpec::remaining(&it)`
  explicitly — collections cannot rely on callers unfolding `remaining()`.
- A manual `loop` drives `decreases` with the non-prophetic
  `IteratorSpec::decrease(&it.iter)->0`; the prophetic `seq()` is barred from
  `decreases` and its conclusion must be drawn before `break`.
- `it` is not in scope after a `for` loop; post-loop facts come from the
  iterator's `when_used_as_spec` form.
- A `#[verifier::type_invariant]` struct may not have crate-public fields.
- **Compilation-ordering constraint:** `validate.sh` verifies the whole crate;
  it produces zero verification results until all ~50 broken files at least
  *compile*. Round 0 proper must therefore restore crate-wide compilation
  before any chapter can be verified incrementally.

## 1. Situation

`~/projects/verus` was upgraded from `release/rolling/0.2026.04.20.8dcd677`
(the tag the codebase was last clean against — 5765 verified, 0 holes) to
`release/rolling/0.2026.05.21.b02cf68`. The codebase was **not** migrated to
match. `scripts/validate.sh` now fails to compile: **178 errors across 50
files**.

Two fixes already applied (needed regardless of direction):

| # | File | Change |
|---|------|--------|
| 1 | `CLAUDE.md:323` | toolchain `1.93.0` → `1.95.0` (matches `rust-toolchain.toml`) |
| 2 | `scripts/validate.sh:148` | removed obsolete `-V new-mut-ref` flag |

## 2. Root cause — three breaking verus changes

| # | Verus PR | Change | Symptom |
|---|----------|--------|---------|
| 1 | #2163 | New iterator model: prophetic sequence encoding | 177 of 178 errors |
| 2 | #2393 + #2400–2405 | `new-mut-ref` released as default, flag removed | `unexpected extended option` (fixed) |
| 3 | #2377 | Trait bound added to spec `==` operator | 1 error (E0277) |

PR #2163 ("New approach to specifying iterators via a prophetic sequence
encoding") replaced the entire vstd iterator-specification mechanism. The old
`ForLoopGhostIterator` / `ForLoopGhostIteratorNew` traits still exist in
`vstd/pervasive.rs` but the `for` loop desugaring **no longer uses them** — it
now uses `VerusForLoopWrapper` + the `IteratorSpec` external-trait extension.
The old APAS 10-component iterator standard is therefore obsolete.

## 3. Error breakdown

| # | Error | Count | Cause |
|---|-------|-------|-------|
| 1 | E0599 `no method view` | 84 | `.view()`/`@` removed from std iterators (`slice::Iter`, `vec::IntoIter`, `hash_set::Iter`, `hash_map::Iter`) |
| 2 | E0609 `no field` | 93 | `VerusForLoopWrapper` fields renamed; `it@.0`/`it@.1`, `iter.pos`/`iter.elements` gone |
| 3 | E0277 `SpecEq` | 1 | spec `==` between `T` and `Arc<T>` needs `T: SpecEq<Arc<T>>` |

## 4. Old model → new model

### Old APAS iterator standard (10 components, now obsolete)
Custom `XxxIter` struct → `View` as `(int, Seq<T>)` → `iter_invariant` spec fn →
`Iterator::next` with two-arm `ensures` → ghost `XxxGhostIterator` struct →
`ForLoopGhostIteratorNew` impl → `ForLoopGhostIterator` impl (6 spec fns) →
`View` for ghost iter as `elements.take(pos)` → `iter()` → `IntoIterator`.

### New model (verus PR #2163, see `~/projects/verus/examples/guide/iterators.rs`)
Custom `XxxIter` struct → `#[verifier::type_invariant]` spec fn → spec
constructor + `#[verifier::when_used_as_spec(..)]` `iter()` → plain
`Iterator::next` (**no `ensures`** — spec lives in the trait impl) →
`impl IteratorSpecImpl` (6 spec fns: `obeys_prophetic_iter_laws`, `remaining`
[prophetic], `will_return_none`, `decrease`, `initial_value_relation`, `peek`)
→ optional `DoubleEndedIteratorSpecImpl`.

### API mapping

| # | Old | New |
|---|-----|-----|
| 1 | `it@.0` (loop position) | `it.index()` |
| 2 | `it@.1` (full sequence) | `it.seq()` |
| 3 | `iter.pos` | `iter.index()` |
| 4 | `iter.elements` | `iter.seq()` |
| 5 | items consumed so far | `it.history()` |
| 6 | `self.inner@` on a std iter | `IteratorSpec::remaining(&self.inner)` (or delete the wrapper) |
| 7 | `iter_invariant(&it)` | `it.wf()` (provided by `VerusForLoopWrapper`) |
| 8 | `View for XxxIter = (int,Seq)` | removed — replaced by `IteratorSpecImpl` |
| 9 | `XxxGhostIterator` + 2 impls | removed |
| 10 | `for x in it: c.iter()` invariant `it@.0`/`it@.1` | `it.index()`/`it.seq()` |

### Open architectural question
Most APAS custom iterators merely wrap `std::slice::Iter` to attach the old
`(int, Seq)` view. In the new model `std::slice::Iter`, `vec::IntoIter`, and the
hash iterators **already implement `IteratorSpecImpl` in vstd**. So a
`Vec`-backed collection's `iter()` could return `std::slice::Iter` directly and
delete its custom iterator struct entirely. This would cut the migration roughly
in half. Decision needed (see §8).

## 5. File inventory

### Tier A — visibly broken (178 compile errors, 50 files)
Heavy: `Chap05/SetStEph.rs` (37), `Chap05/SetMtEph.rs` (35),
`Chap05/RelationStEph.rs` (6), `Chap19/ArraySeqMtEph.rs` (5).
Standards (3 ea): `view_`, `table_of_contents_`, `mod_`, `iterators_`,
`deep_view_standard.rs`; `wrapping_iterators_standard.rs` (1).
Reference collections (3 ea): `Chap17/MathSeq`, `Chap18/ArraySeq*` ×6,
`Chap19/ArraySeq*` ×3, `Chap23` ×2.
Consumers (1–4 ea): `Chap37` ×5, `Chap38`, `Chap39` ×4, `Chap40` ×3,
`Chap41` ×4, `Chap43` ×4, `Chap44`, `Chap59` ×2, `Chap63` ×2, `Chap64` ×2.
vstdplus: `hash_map_with_view_plus.rs`, `hash_set_with_view_plus.rs`,
`arc_rwlock.rs` (the E0277).

### Tier B — define old-style iterators, not yet erroring (~20 files)
Compile today (old traits still exist) but their iterators are no longer
for-loop-usable and will surface verification failures once Tier A compiles:
`Chap05/MappingStEph`, `Chap06` ×8 graph files, `Chap37` Mt + AVLTreeSeq files,
`Chap41/ArraySetStEph`, `Chap42` ×3, `Chap43` Mt files.

### Tier C — documentation
`docs/APAS-VERUSIterators.rs`, `.cursor/rules/apas-verus/collection-iterators.mdc`,
CLAUDE.md "Collection Iterator Standard" section.

Total: **~69 collection files** define iterators + 6 standards + 2 vstdplus +
docs ≈ **~78 files** touch the iterator standard.

## 6. Proposed round structure

| # | Round | Scope | Validate | Risk |
|---|-------|-------|----------|------|
| 0 | Pilot | Rewrite `iterators_standard.rs` to the new pattern; migrate `Chap18/ArraySeqStEph.rs` + its consumers; nail the canonical APAS shape | `isolate Chap18` | High — defines the template |
| 1 | Standards | Remaining 5 standards files + `wrapping_iterators_standard.rs` + 2 vstdplus + `arc_rwlock.rs` E0277 + docs (Tier C) | `dov` | Med |
| 2 | Seq collections | Chap17, 18, 19, 23 (ArraySeq/LinkedList/MathSeq/tree-seq) | `isolate` each | Med |
| 3 | BST collections | Chap37, 38, 39, 40 | `isolate` each | Med |
| 4 | Set/Table collections | Chap05, 41, 42, 43 (incl. SetStEph/SetMtEph — the 72-error pair) | `isolate` each | High |
| 5 | Consumers + graph | Chap06, 44, 59, 63, 64 for-loop invariants | `isolate` each | Low |
| 6 | Integration | Full `validate.sh`, then `rtt.sh`, then `ptt.sh`; trigger-note sweep | full | — |

Rounds 2–5 parallelize across the 4 agent worktrees, one chapter group per
agent, per the standard project workflow. Realistic estimate: 6+ rounds; the
SetStEph/SetMtEph pair and the standard rewrite are the long poles.

## 7. Risks / unknowns

1. **Prophetic specs** (`#[verifier::prophetic]`) are new to this codebase — the
   `remaining()` semantics (items that *will* be returned) differ from the old
   `elements`/`pos` model and may need fresh proof work in consumer loops.
2. Tier B count (~20) is a lower bound — more files may surface once Tier A
   compiles and verification (not just compilation) runs.
3. RTT/PTT impact unknown until compilation is restored; the 6 iterator PTT
   patterns in `collection-iterators.mdc` will need rewriting.
4. Whether to keep or delete custom iterator wrappers (§4 open question) changes
   the file count materially.

## 8. Decisions needed before Round 0

1. **Direction confirmed?** Migrate forward (this plan) vs. roll verus back to
   `0.2026.04.20.8dcd677` (codebase clean immediately; defers all of the above).
2. **Custom wrappers:** keep per-collection `XxxIter` structs, or delete them
   and return `std::slice::Iter` directly where the collection is `Vec`-backed?
3. **Pilot file:** `Chap18/ArraySeqStEph.rs` is the documented reference — OK as
   the pilot, or prefer another?

## 10. Iterator classification — delegated vs custom (per type)

There are exactly two iterator styles, both defined in the standard:
**delegated** — `iter()` returns a std library iterator — and **custom** —
`iter()` returns a type that implements `IteratorSpecImpl` by hand. Of 71
collection iterators, 68 migrate to delegated and 3 to custom.

The migration is **algorithm-preserving**: every collection keeps its current
iterator complexity. Delegated collections already return — or already flatten
to — the std iterator they will return; the 3 custom collections keep their
lazy traversal. See the complexity note below the table.

"Iterator field today" is the field of the current `*Iter` struct, read from
the source. It is the evidence for the Style column:

- `std::slice::Iter` / `std::vec::IntoIter` — already a std iterator; the
  migrated `iter()` returns it directly.
- another APAS collection's `*Iter` — the collection re-exposes that
  collection's iterator, so it migrates after its backing collection.
- `Vec<T> + position index` — a hand-rolled iterator over a `Vec` the
  collection already builds (a tree flattened in order, or a snapshot); the
  migrated `iter()` returns that `Vec`'s `std::vec::IntoIter`.
- `Vec<&Node>` stack / `&tree + position index` — a genuine lazy tree
  traversal with no std iterator to return. These are the 3 custom rows.

| # | Chap | File | Style | Iterator field today |
|---|------|------|-------|----------------------|
| 1 | 05 | SetStEph.rs | delegated | HashSetWithViewPlusIter |
| 2 | 05 | SetMtEph.rs | delegated | HashSetWithViewPlusIter |
| 3 | 05 | RelationStEph.rs | delegated | SetStEphIter |
| 4 | 05 | MappingStEph.rs | delegated | RelationStEphIter |
| 5 | 06 | DirGraphStEph.rs | delegated | SetStEphIter |
| 6 | 06 | DirGraphMtEph.rs | delegated | SetStEphIter |
| 7 | 06 | UnDirGraphStEph.rs | delegated | SetStEphIter |
| 8 | 06 | UnDirGraphMtEph.rs | delegated | SetStEphIter |
| 9 | 06 | LabDirGraphStEph.rs | delegated | SetStEphIter |
| 10 | 06 | LabDirGraphMtEph.rs | delegated | SetStEphIter |
| 11 | 06 | LabUnDirGraphStEph.rs | delegated | SetStEphIter |
| 12 | 06 | LabUnDirGraphMtEph.rs | delegated | SetStEphIter |
| 13 | 17 | MathSeq.rs | delegated | std::slice::Iter |
| 14 | 18 | ArraySeq.rs | delegated | std::slice::Iter |
| 15 | 18 | ArraySeqStEph.rs | delegated | std::slice::Iter |
| 16 | 18 | ArraySeqStPer.rs | delegated | std::slice::Iter |
| 17 | 18 | ArraySeqMtEph.rs | delegated | std::slice::Iter |
| 18 | 18 | ArraySeqMtEphSlice.rs | delegated | std::slice::Iter |
| 19 | 18 | ArraySeqMtPer.rs | delegated | std::slice::Iter |
| 20 | 18 | LinkedListStEph.rs | delegated | std::slice::Iter |
| 21 | 18 | LinkedListStPer.rs | delegated | std::slice::Iter |
| 22 | 19 | ArraySeqStEph.rs | delegated | std::slice::Iter |
| 23 | 19 | ArraySeqStPer.rs | delegated | std::slice::Iter |
| 24 | 19 | ArraySeqMtEph.rs | delegated | std::slice::Iter |
| 25 | 19 | ArraySeqMtEphSlice.rs | delegated | std::slice::Iter |
| 26 | 23 | BalBinTreeStEph.rs | delegated | std::vec::IntoIter |
| 27 | 23 | PrimTreeSeqStPer.rs | delegated | std::slice::Iter |
| 28 | 37 | AVLTreeSeq.rs | custom | &tree + position index |
| 29 | 37 | AVLTreeSeqStEph.rs | custom | Vec<&Node> stack (lazy) |
| 30 | 37 | AVLTreeSeqStPer.rs | custom | Vec<&Node> stack (lazy) |
| 31 | 37 | AVLTreeSeqMtPer.rs | delegated | Vec<T> + position index |
| 32 | 37 | BSTAVLStEph.rs | delegated | std::vec::IntoIter |
| 33 | 37 | BSTBBAlphaStEph.rs | delegated | std::vec::IntoIter |
| 34 | 37 | BSTPlainStEph.rs | delegated | std::vec::IntoIter |
| 35 | 37 | BSTRBStEph.rs | delegated | std::vec::IntoIter |
| 36 | 37 | BSTSplayStEph.rs | delegated | std::vec::IntoIter |
| 37 | 37 | BSTAVLMtEph.rs | delegated | Vec<T> + position index |
| 38 | 37 | BSTBBAlphaMtEph.rs | delegated | Vec<T> + position index |
| 39 | 37 | BSTPlainMtEph.rs | delegated | Vec<T> + position index |
| 40 | 37 | BSTRBMtEph.rs | delegated | Vec<T> + position index |
| 41 | 37 | BSTSetAVLMtEph.rs | delegated | Vec<T> + position index |
| 42 | 37 | BSTSetBBAlphaMtEph.rs | delegated | Vec<T> + position index |
| 43 | 37 | BSTSetPlainMtEph.rs | delegated | Vec<T> + position index |
| 44 | 37 | BSTSetRBMtEph.rs | delegated | Vec<T> + position index |
| 45 | 37 | BSTSetSplayMtEph.rs | delegated | Vec<T> + position index |
| 46 | 38 | BSTParaStEph.rs | delegated | std::vec::IntoIter |
| 47 | 39 | BSTTreapStEph.rs | delegated | std::vec::IntoIter |
| 48 | 39 | BSTTreapMtEph.rs | delegated | std::vec::IntoIter |
| 49 | 39 | BSTParaTreapMtEph.rs | delegated | std::vec::IntoIter |
| 50 | 39 | BSTSetTreapMtEph.rs | delegated | std::vec::IntoIter |
| 51 | 40 | BSTKeyValueStEph.rs | delegated | std::vec::IntoIter |
| 52 | 40 | BSTReducedStEph.rs | delegated | std::vec::IntoIter |
| 53 | 40 | BSTSizeStEph.rs | delegated | std::vec::IntoIter |
| 54 | 41 | ArraySetStEph.rs | delegated | ArraySeqStEphIter |
| 55 | 41 | AVLTreeSetStEph.rs | delegated | std::vec::IntoIter |
| 56 | 41 | AVLTreeSetStPer.rs | delegated | std::vec::IntoIter |
| 57 | 41 | AVLTreeSetMtEph.rs | delegated | Vec<T> + position index |
| 58 | 41 | AVLTreeSetMtPer.rs | delegated | std::vec::IntoIter |
| 59 | 41 | OrdKeyMap.rs | delegated | std::vec::IntoIter |
| 60 | 42 | TableStEph.rs | delegated | ArraySeqStEphIter |
| 61 | 42 | TableMtEph.rs | delegated | ArraySeqMtEphIter |
| 62 | 42 | TableStPer.rs | delegated | ArraySeqStPerIter |
| 63 | 43 | OrderedSetStEph.rs | delegated | std::vec::IntoIter |
| 64 | 43 | OrderedSetStPer.rs | delegated | std::vec::IntoIter |
| 65 | 43 | OrderedSetMtEph.rs | delegated | Vec<T> + position index |
| 66 | 43 | OrderedTableStEph.rs | delegated | std::vec::IntoIter |
| 67 | 43 | OrderedTableStPer.rs | delegated | std::vec::IntoIter |
| 68 | 43 | OrderedTableMtEph.rs | delegated | Vec<Pair> + position index |
| 69 | 43 | OrderedTableMtPer.rs | delegated | OrderedTableStPerIter |
| 70 | — | vstdplus/hash_set_with_view_plus.rs | delegated | std::collections::hash_set::Iter |
| 71 | — | vstdplus/hash_map_with_view_plus.rs | delegated | std::collections::hash_map::Iter |

### Complexity verification — was vs proposed, all 71 iterators

The migration swaps each iterator's *type* (custom `XxxIter` wrapper → a std
iterator) and deletes the ghost-only `ForLoopGhostIterator` machinery. It does
**not** rewrite the algorithmic body of `iter()`/`next()` — the in-order flatten
(`Node::in_order`), `self.seq.iter()`, `push_left_iter`
(`src/Chap37/AVLTreeSeqStEph.rs:441`), and `tree.nth` are all untouched. So
complexity is preserved by construction. Every cell below is `was → proposed`
and the two are equal.

| # | Variant (iterator field today) | iter() | next() | Space | First k then break | Full | Files |
|---|--------------------------------|--------|--------|-------|--------------------|------|-------|
| 1 | `slice::Iter` wrap (Vec-backed) | O(1)→O(1) | O(1)→O(1) | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) | 14 |
| 2 | flatten → `vec::IntoIter` (in-order walk) | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) | 22 |
| 3 | flatten → `Vec` snapshot + index | O(n)→O(n) | O(1)→O(1) | O(n)→O(n) | O(n)→O(n) | O(n)→O(n) | 13 |
| 4 | lazy node-ref stack (AVLTreeSeq StEph/StPer) | O(lg n)→O(lg n) | O(1)†→O(1)† | O(lg n)→O(lg n) | O(lg n+k)→O(lg n+k) | O(n)→O(n) | 2 |
| 5 | lazy tree index (AVLTreeSeq base) | O(1)→O(1) | O(lg n)→O(lg n) | O(1)→O(1) | O(k lg n)→O(k lg n) | O(n lg n)→O(n lg n) | 1 |
| 6 | chained — re-expose inner iterator | inherits | inherits | inherits | inherits | inherits | 15 |
| 7 | `hash_set`/`hash_map::Iter` | O(1)→O(1) | O(1)†→O(1)† | O(1)→O(1) | O(k)→O(k) | O(n)→O(n) | 4 |

† amortized over the traversal. File total: 14+22+13+2+1+15+4 = 71 (per-file
mapping is the table above). Row 6 inherits its backing collection's profile
(adding O(1) per-`next()` forwarding): Relation/Mapping/8 graphs inherit row 7;
ArraySet/3 Tables inherit row 1; OrderedTableMtPer inherits row 2.

Evidence, read from source:
- Row 1 — `ArraySeqStEph::iter()` is `{ inner: self.seq.iter() }`.
- Row 2 — `BSTPlainStEph::into_iter()` is `self.root.in_order(); ...into_iter()`.
- Row 3 — `BSTAVLMtEph::iter()` annotated `Work O(n) — snapshot iteration`.
- Row 4 — `AVLTreeSeqStEph::iter()` is `push_left_iter(...)`, annotated `O(lg n)`.
- Row 5 — `AVLTreeSeq::next()` is `self.tree.nth(self.pos); pos += 1`.
- Row 7 — `SetStEph::iter()` annotated `Work O(1) — Creates iterator handle`.

The only place a change was possible is rows 4–5 (the 3 lazy AVLTreeSeq files):
flattening them would push iter() to O(n) and space to O(n) and falsify their
`Alg Analysis: Work O(lg n)` annotations. They therefore stay custom — port the
lazy iterator to a from-scratch `IteratorSpecImpl` (per `CountIter` in the
standard). `AVLTreeSeqMtPer` already flattens (row 3); leave it. Row 5's
O(n lg n) full traversal is suboptimal today; the migration preserves it as-is —
improving it is a separate APAS cost-spec question, out of scope here.
