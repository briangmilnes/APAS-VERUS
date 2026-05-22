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
