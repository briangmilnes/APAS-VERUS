<style>
body { max-width: 95% !important; width: 95% !important; margin: 0 auto !important; padding: 1em !important; }
.markdown-body { max-width: 95% !important; width: 95% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 95% !important; width: 95% !important; }
table { width: 95% !important; table-layout: auto; }
</style>

# Derives applied (r217)

Plan: `plans/r217-apply-derives.md`. Evidence: `docs/DerivesInsideVerus.md`
(r216) and the `src/experiments/derive_0913_*` files, plus one new experiment
written this round. Start: `main` at `15a289dec`. Toolchain: Verus
`0.2026.09.13.671956e`, Z3 4.16.0, Rust 1.98.1.

Three changes, each measured, verified and committed on its own: replace the
hand-written impls a derive can carry, add `StructuralEq` where it buys a
specified `==`, and replace the `Debug` impls a derive reproduces exactly.

Every candidate list below was re-derived from the source this round. Where a
re-derivation disagrees with r216's summary, the disagreement is stated.

## 0. The new experiment this round rests on

r216 measured the derived-`Clone` postcondition on a struct of `u64`
(`derive_0913_clone_copy_struct.rs`). Every type part A touches in Chap26 has
`f64` fields, and `f64` is not `Structural`
(`builtin/src/lib.rs` carries a TODO for it), so the `u64` measurement does
not settle the `f64` case. `src/experiments/derive_0913_clone_copy_f64_struct.rs`
asks it directly.

| # | Chap | File | Question | Result |
|---|------|------|----------|--------|
| 1 | exp | derive_0913_clone_copy_f64_struct.rs | derived `Clone` on an `f64` `Copy` struct, and on a struct of those | SUCCEEDS, 6 verified, 0 errors |

The answer: `clone_add_post_condition` keys on the shape of the emitted body,
not on the field types, so `ensures ret == self` attaches to an `f64` struct
exactly as it does to a `u64` one. Spec equality over `f64` fields is
available even though `f64` is not `Structural`, because `Structural` governs
the `==` operator and not `spec_eq`. A struct whose fields are themselves such
structs — `Edge`, whose fields are `Point`s — inherits both facts. Log:
`logs/validate-standard-derive_0913_clone_copy_f64_struct.20260922-130315.log`.

## 1. Part A — the 11 hand-written impls a derive carries

### 1.1 Re-derived candidate list

`grep` over `src/`, excluding `src/experiments/` and `src/standards/`, finds
36 hand-written `impl Clone for X` and `impl Copy for X` on non-generic types.
Reading each receiving type's fields leaves four files: every other receiver
holds a `Vec`, a `HashMap` or an APAS collection, for which rustc emits a
constructor body and Verus attaches no postcondition
(`docs/DerivesInsideVerus.md` §1). Two further sites were examined and
excluded: `src/vstdplus/threads_plus.rs`'s `impl Copy for IsThreadPlus`, a
`tracked struct` whose `Clone` lives outside `verus!`; and the `Clone` impls
inside the `macro_rules!` bodies of `src/vstdplus/checked_int.rs` and
`src/vstdplus/checked_nat.rs`, which are macro expansions rather than impls on
a named type. The re-derivation agrees with r216's table: 11 impls, in four
files.

### 1.2 Every impl deleted, and the fact checked before and after

| # | Chap | File | Impl deleted | Fact before | Fact after |
|---|------|------|--------------|-------------|------------|
| 1 | 26 | ETSPStEph.rs | `impl Copy for Point` | marker, none | marker, none |
| 2 | 26 | ETSPStEph.rs | `impl Copy for Edge` | marker, none | marker, none |
| 3 | 26 | ETSPStEph.rs | `impl Clone for Point` | `cloned == *self` | `ret == self` |
| 4 | 26 | ETSPStEph.rs | `impl Clone for Edge` | `cloned == *self` | `ret == self` |
| 5 | 26 | ETSPMtEph.rs | `impl Copy for Point` | marker, none | marker, none |
| 6 | 26 | ETSPMtEph.rs | `impl Copy for Edge` | marker, none | marker, none |
| 7 | 26 | ETSPMtEph.rs | `impl Clone for Point` | `cloned == *self` | `ret == self` |
| 8 | 26 | ETSPMtEph.rs | `impl Clone for Edge` | `cloned == *self` | `ret == self` |
| 9 | 57 | DijkstraStEphU64.rs | `impl Clone for PQEntry` | `cloned@ == self@` | `ret == self` |
| 10 | 57 | DijkstraStEphU64.rs | `impl PartialEqSpecImpl for PQEntry` | `obeys_eq_spec`, `eq_spec == self@ == other@`, unchecked | the same two, compiler-checked |
| 11 | 57 | DijkstraStEphF64.rs | `impl Clone for PQEntry` | `cloned@ == self@` | `ret == self` |

Rows 3 to 8 state the same proposition before and after: the hand-written
`ensures` was literally `cloned == *self`, and the derived `Clone` on a
non-generic `Copy` type carries `ensures ret == self`.

Rows 9 and 11 are stronger after than before. `PQEntry`'s `View` is the
identity (`type V = Self; view(&self) -> Self { *self }`), so
`ret == self` implies `ret@ == self@` by congruence, and not conversely in
general. Nothing in either chapter relied on the weaker form.

Row 10 states the same two propositions, but earns them. Before, the
`PartialEqSpecImpl` was hand-written beside a derived `PartialEq` that Verus
ignores, so `obeys_eq_spec() == true` was an unchecked claim of the same
standing as an `assume` (`docs/DerivesInsideVerus.md` §4). After,
`#[derive(StructuralEq)]` emits the same `PartialEqSpecImpl` alongside an
`unsafe impl Structural` whose body is one
`AssertParamIsStructural<#ty>` per field, so rustc refuses the derive unless
every field type carries the marker. `eq_spec` becomes `spec_eq(self, other)`,
which for an identity `View` is the same predicate as `self@ == other@`.

### 1.3 The derives applied

| # | Chap | File | Type | Derive added |
|---|------|------|------|--------------|
| 1 | 26 | ETSPStEph.rs | `Point` | `Clone, Copy` |
| 2 | 26 | ETSPStEph.rs | `Edge` | `Clone, Copy` |
| 3 | 26 | ETSPMtEph.rs | `Point` | `Clone, Copy` |
| 4 | 26 | ETSPMtEph.rs | `Edge` | `Clone, Copy` |
| 5 | 57 | DijkstraStEphU64.rs | `PQEntry` | `Clone, Copy, StructuralEq` |
| 6 | 57 | DijkstraStEphF64.rs | `PQEntry` | `Clone, Copy` |

Row 6 takes no `StructuralEq`: `PQEntry`'s `dist` field is a `WrappedF64`,
whose own field is an `f64`, and `f64` is not `Structural`, so the derive is
refused with `E0277`. Its hand-written `PartialEqSpecImpl` therefore stays,
and remains unchecked; see §2.3.

A derive is an attribute, so it sits on the type definition in section 4
rather than in section 12. Section 12 held only the impls that are now gone,
so the section header and its table-of-contents line were removed from
`ETSPStEph.rs`, `ETSPMtEph.rs` and `DijkstraStEphU64.rs`. `ETSPStEph.rs`'s
section 9a held only `impl Copy for Point` and went the same way.
`DijkstraStEphF64.rs` keeps section 12 for its surviving `PartialEqSpecImpl`.
`DijkstraStEphU64.rs`'s `use vstd::std_specs::cmp::PartialEqSpecImpl` became
unused, because the macro writes the trait's full path, and was removed.

### 1.4 Cost

No exec body changed. A derived `Clone` on a `Copy` type emits a read of
`self`, which is the same machine operation as the hand-written `{ *self }`
body it replaces, and the same as the field-by-field constructor the two
`PQEntry` bodies used: work O(1), span O(1), before and after. Deriving `Copy`
on `PQEntry`, which did not have it, turns its moves into copies of the same
two machine words; work and span are unchanged. No `/// - Alg Analysis` line
was edited, and no discrepancy against one was found.

### 1.5 Part A results

| # | Chap | Command | Verified | Err | Warn | Trigger notes | Log |
|---|------|---------|---------:|----:|-----:|--------------:|-----|
| 1 | 26 | validate isolate, before | 1099 | 0 | 0 | 0 | validate.20260922-130041.log |
| 2 | 26 | validate isolate, after | 1099 | 0 | 0 | 0 | validate.20260922-130221.log |
| 3 | 57 | validate isolate, before | 2583 | 0 | 0 | 0 | validate.20260922-130106.log |
| 4 | 57 | validate isolate, after | 2583 | 0 | 0 | 0 | validate.20260922-130403.log |

| # | Chap | Targets | Tests | Pass | Fail | Log |
|---|------|---------|------:|-----:|-----:|-----|
| 1 | 26 | 8 | 59 | 59 | 0 | rtt.20260922-130602.log |
| 2 | 57 | 3 | 48 | 48 | 0 | rtt.20260922-130607.log |

The verified count is unchanged in both chapters: each deleted `fn clone` body
is replaced by a derived one that Verus checks in its place, and the `Copy`
and `PartialEqSpecImpl` impls carry no checked function. The three rustc
warnings the library build reports (`Chap43/OrderedTableStEph.rs` `cfg(never)`
and two negative impls in `vstdplus/threads_plus.rs`) predate this round and
are untouched.

One incidental difference appears in the part A diff: the Edit tool trims
trailing whitespace from its arguments, so `DijkstraStEphU64.rs`'s
`verus! ` line lost its trailing space. It has no effect on the build, the
proof or the reader, and could not be restored with the Edit tool.

## 2. Part B — `StructuralEq` where it buys a specified `==`

### 2.1 Re-derived candidate list

`grep -rn "#\[derive.*PartialEq" src/`, excluding `src/experiments/`,
`src/standards/` and `src/lib.rs`, finds 14 sites. Reading each type
definition classifies them.

| # | Chap | File | Type | Shape | `StructuralEq` |
|---|------|------|------|-------|----------------|
| 1 | 50 | MatrixChainStEph.rs | `MatrixDim` | 2 × `usize` | added |
| 2 | 50 | MatrixChainStPer.rs | `MatrixDim` | 2 × `usize` | added |
| 3 | 50 | MatrixChainMtEph.rs | `MatrixDim` | 2 × `usize` | added |
| 4 | 50 | MatrixChainMtPer.rs | `MatrixDim` | 2 × `usize` | added |
| 5 | 37 | BSTRBMtEph.rs | `Color` | unit-variant enum | added |
| 6 | 47 | ParaHashTableStEph.rs | `LoadAndSize` | 2 × `usize` | added |
| 7 | 57 | DijkstraStEphU64.rs | `PQEntry` | `i64`, `usize` | added in part A |
| 8 | 57 | DijkstraStEphF64.rs | `PQEntry` | reaches an `f64` | refused, E0277 |
| 9 | 47 | FlatHashTable.rs | `FlatEntry<Key, Value>` | generic enum | refused, E0107 |
| 10 | 65 | PrimStEph.rs | `PQEntry<V>` | generic | refused, E0107 |
| 11 | — | Types.rs | `Edge<V>` | generic | refused, E0107 |
| 12 | — | Types.rs | `LabEdge<V, L>` | generic | refused, E0107 |
| 13 | — | Types.rs | `WeightedEdge<V, W>` | generic | refused, E0107 |
| 14 | — | Types.rs | `Pair<K, V>` | generic | refused, E0107 |

The re-derivation disagrees with `docs/DerivesInsideVerus.md` §5 in one way:
r216's strengthening table lists seven types and does not name rows 9 and 10.
Both are genuine `#[derive(PartialEq)]` sites whose `==` proves nothing, and
both are generic, so they belong with rows 11 to 14 as blocked rather than
omitted. Row 9, `Chap47/FlatHashTable.rs`'s `FlatEntry<Key, Value>`, is the
one r216's own §8 item 1 would reach if `derive_structural_eq` carried
generics; row 10, `Chap65/PrimStEph.rs`'s `PQEntry<V>`, would still need its
`Option<V>` field's parameter to be `Structural`.

The six additions are justified two ways. `Chap50`'s `MatrixDim` is compared
directly — `tests/Chap50/TestMatrixChainMtEph.rs:178` builds two values and
compares them — so the comparison exists and was unspecified. `Color` and
`LoadAndSize` are not compared in verified exec code today; they derive
`PartialEq`, which puts an unspecified `==` on the public interface, and
`src/standards/partial_eq_eq_clone_standard.rs` requires a comparable type to
carry a spec. Nothing else about any of the six types changed.

### 2.2 What `==` proves, before and after

| # | Chap | File | Type | `==` before | `==` after |
|---|------|------|------|-------------|------------|
| 1 | 50 | MatrixChainStEph.rs | `MatrixDim` | nothing | `equal == (*a == *b)` and `equal == (a@ == b@)` |
| 2 | 50 | MatrixChainStPer.rs | `MatrixDim` | nothing | the same two |
| 3 | 50 | MatrixChainMtEph.rs | `MatrixDim` | nothing | the same two |
| 4 | 50 | MatrixChainMtPer.rs | `MatrixDim` | nothing | the same two |
| 5 | 37 | BSTRBMtEph.rs | `Color` | nothing | `equal == (a == b)`, and the variant test `c == Color::Red` decides `c is Red` |
| 6 | 47 | ParaHashTableStEph.rs | `LoadAndSize` | nothing | `equal == (*a == *b)` |
| 7 | 57 | DijkstraStEphU64.rs | `PQEntry` | claimed, unchecked | `equal == (a@ == b@)`, checked |

"Nothing" is not an estimate: `derive_0913_partial_eq_struct.rs` measures a
bare `#[derive(PartialEq)]` and reports that both directions of
`equal == (*a == *b)` fail. `automatic_derive.rs` maps `PartialEq` to
`Ignore`, so `obeys_eq_spec()` stays uninterpreted and the `ensures` on
vstd's `ExPartialEq::eq` is vacuous.

Rows 1 to 4 gain the view direction as well as the structural one because
`MatrixDim`'s `View` is `(self.rows as nat, self.cols as nat)`, and a pair of
casts is injective. That is the full APAS `PartialEq` postcondition,
`equal == (self@ == other@)`, obtained without a hand-written body and
without an `accept`.

### 2.3 The experiment this rests on

r216's experiments cover a struct of `u64` (9), a unit-variant enum (10) and a
`View` whose type is the field type (13). None covers a `View` of type
`(nat, nat)` built by casting, which is rows 1 to 4. One new experiment states
all four r217 shapes in one file.

| # | Chap | File | Question | Result |
|---|------|------|----------|--------|
| 1 | exp | derive_0913_structural_eq_r217_shapes.rs | `StructuralEq` on the four shapes r217 adds it to | SUCCEEDS, 8 verified, 0 errors |

Log:
`logs/validate-standard-derive_0913_structural_eq_r217_shapes.20260922-131253.log`.
It also records that `PartialEq` alone, without `Eq`, is enough for the
derive, which row 6 relies on: `LoadAndSize` derives `PartialEq` and not `Eq`,
and `PartialEqSpecImpl` extends `PartialEq`, not `Eq`
(`vstd/std_specs/cmp.rs:11`).

### 2.4 The three unchecked `PartialEqSpecImpl` declarations

| # | Chap | File | Type | Before | After |
|---|------|------|------|--------|-------|
| 1 | 57 | DijkstraStEphU64.rs | `PQEntry` | hand-written beside a derived `PartialEq` Verus ignores | deleted; `StructuralEq` emits it with a checked `Structural` marker |
| 2 | 57 | DijkstraStEphF64.rs | `PQEntry` | the same shape | unchanged; see below |
| 3 | 43 | OrderedSetStPer.rs | `OrderedSetStPer<T>` | declared inside `verus!`, its `eq` body outside in section 14 | `eq` moved into `verus!` section 12 with `ensures equal == (self@ == other@)`, proved |

Row 3 is the one that closes cleanly and gains a proof. The old body,
`self.base_set == other.base_set`, sat outside `verus!` where Verus could not
see it. Moved into section 12 with the standard `ensures`, it proves outright
and needs no bridging `accept` of its own: `OrderedSetStPer`'s `View` is
literally `self.base_set@`, and `Chap41/AVLTreeSetStPer.rs`'s own `PartialEq`
already carries `ensures equal == (self@ == other@)` with
`obeys_eq_spec() == true`. Chap43's verified count rises from 2686 to 2687 —
the one new checked function is this `eq`.

Row 2 cannot be closed this round. `PQEntry`'s `dist` is a `WrappedF64`, whose
`val` is an `f64`; `f64` is not `Structural`, so `StructuralEq` is refused
with `E0277` (`derive_0913_structural_eq_vec_struct.rs` measures the same
error for `Vec`). The alternative, a hand-written `eq` inside `verus!` with
the standard `ensures`, has nothing to prove from:
`src/vstdplus/float.rs:484` puts `impl PartialEq for WrappedF64` outside
`verus!` (the block closes at line 467) and gives it no `PartialEqSpecImpl`,
so `WrappedF64 == WrappedF64` is uninterpreted, and underneath it `f64 ==` is
uninterpreted too. Writing that body would therefore require a new `accept`,
which this round forbids. Deleting the declaration would remove the unchecked
claim but weaken what `==` proves from "something, untrusted" to "nothing",
which is a specification change for the user to decide rather than the agent.
It is left exactly as it is and reported. Closing it properly means
specifying `WrappedF64`'s `==` in `src/vstdplus/float.rs`, which is a float
axiom and a vstdplus change, not a derive.

### 2.5 Cost

No exec body changed in rows 1 to 6: `StructuralEq` emits an
`unsafe impl Structural` whose only method is the compile-time
`assert_receiver_is_structural`, plus a `PartialEqSpecImpl` of two `spec fn`s.
Neither is executable. Row 3 of §2.4 moves an existing one-line `eq` body
between sections without changing it: work O(|set|) and span O(|set|) before
and after, since it delegates to `AVLTreeSetStPer::eq`. No
`/// - Alg Analysis` line was edited, and no discrepancy against one was
found.

### 2.6 Part B results

| # | Chap | Verified before | Verified after | Err | Warn | Trigger notes | Log after |
|---|------|----------------:|---------------:|----:|-----:|--------------:|-----------|
| 1 | 37 | 1863 | 1863 | 0 | 0 | 0 | validate.20260922-131112.log |
| 2 | 43 | 2686 | 2687 | 0 | 0 | 0 | validate.20260922-131127.log |
| 3 | 47 | 1161 | 1161 | 0 | 0 | 0 | validate.20260922-131154.log |
| 4 | 50 | 766 | 766 | 0 | 0 | 0 | validate.20260922-131202.log |

Baseline logs: `validate.20260922-130943.log` (37),
`validate.20260922-130957.log` (43), `validate.20260922-131024.log` (47),
`validate.20260922-131032.log` (50).

| # | Chap | Targets | Tests | Pass | Fail | Log |
|---|------|---------|------:|-----:|-----:|-----|
| 1 | 37 | 24 | 544 | 544 | 0 | rtt.20260922-131306.log |
| 2 | 43 | 11 | 279 | 279 | 0 | rtt.20260922-131313.log |
| 3 | 47 | 7 | 102 | 102 | 0 | rtt.20260922-131327.log |
| 4 | 50 | 9 | 167 | 167 | 0 | rtt.20260922-131314.log |

Chapters 37, 47 and 50 keep their verified counts: the six `StructuralEq`
additions make a fact available that no verified function in those chapters
asks for yet. The measurement that they do supply it is the experiment in
§2.3, not the chapter counts.

`scratch/r212-rtt.sh` names its log by the second, so the Chap47 and Chap50
runs first collided on `rtt.20260922-131314.log` and Chap47's was overwritten;
Chap47 was re-run to give it a log of its own. Both results are the same
102 pass.

## 3. Part C — the `Debug` impls a derive reproduces exactly

### 3.1 Re-derived candidate list, and where it disagrees with r216

`docs/DerivesInsideVerus.md` §5 counts 246 hand-written `Debug` impls over
`src/`, excluding `src/experiments/` and `src/standards/`. Re-deriving that
count finds 289. The 43 missing from r216's figure are the module-qualified
form, `impl ... fmt::Debug for X`, written after a
`use std::fmt::{Debug, Display, Formatter}`; r216's pattern reached
`impl ... std::fmt::Debug for X` (134) and bare `impl ... Debug for X` (112)
and stopped there. The three forms partition the 289 exactly.

r216 then splits the 246 into 86 constant-string bodies and 160 that format
deliberately. That split is the right question but the wrong boundary for a
replacement round, because what decides whether a derive reproduces the text
is the shape of the *receiving type*, not the shape of the body. A derived
`Debug` prints the type name followed by its fields. It therefore reproduces a
constant-string body if and only if the type has no fields.

That gives one criterion, applied to every `Debug` impl in the crate:

| # | Criterion | Count | Action |
|---|-----------|------:|--------|
| 1 | receiver is a unit struct, body prints the type name only | 62 | replaced by `#[derive(Debug)]` |
| 2 | receiver has fields, body prints the type name only | 44 | left alone, §3.5 |
| 3 | receiver is a unit struct in an `Example` file | 3 | left alone, §3.5 |
| 4 | body formats deliberately (fields, placeholders, a summary) | 180 | left alone |
| — | total at `e2f528707` | 289 | |

### 3.2 The two measurements the replacement rests on

The printed text is not something to assume. Two plain-Rust programs measure
it, compiled with `rustc -O` on Rust 1.98.1.

| # | File | Question | Result |
|---|------|----------|--------|
| 1 | scratch/r217-unit-struct-debug.rs | derived vs `write!(f, "X")` on a unit struct | identical under `{:?}` and `{:#?}` |
| 2 | scratch/r217-debug-struct-finish.rs | derived vs `f.debug_struct("X").finish()` on a unit struct | identical under `{:?}` and `{:#?}` |
| 3 | scratch/r217-derive-debug-plain-field.rs | derived vs constant string on a struct with a field | `DerivedS { n: 7 }` against `HandWrittenS` — different |
| 4 | scratch/r217-derive-debug-non-unit.rs | derive on a struct whose field type has no `Debug` | refused, `E0277` |

Rows 1 and 2 are why no test expectation moves: a derived unit struct prints
its bare type name, character for character the string the deleted body wrote.
Rows 3 and 4 are why criterion 2 of §3.1 is a boundary and not a preference —
for the 44 receivers with fields, the derive either prints a different string
or does not compile at all.

The Verus side is `src/experiments/derive_0913_debug_unit_struct.rs`
(SUCCEEDS, 1 verified, 0 errors,
`logs/validate-standard-derive_0913_debug_unit_struct.20260922-131556.log`):
`#[derive(Debug)]` compiles inside `verus!` on a unit struct, including one
that also implements `RwLockPredicate`, which 16 of the 62 receivers do.

### 3.3 Every `Debug` impl replaced

Sixty-two impls in 58 files across 19 chapters. The body shape is
`write!(f, "X")` except for the four marked `debug_struct`, whose body is
`f.debug_struct("X").finish()`. Every receiver is `pub struct X;`.

| # | Chap | File | Type | Body shape |
|---|------|------|------|------------|
| 1 | 05 | SetMtEph.rs | `SetMtEphInv` | write! |
| 2 | 06 | DirGraphMtEph.rs | `DirGraphMtEphInv` | write! |
| 3 | 06 | LabDirGraphMtEph.rs | `LabDirGraphMtEphInv` | write! |
| 4 | 06 | LabUnDirGraphMtEph.rs | `LabUnDirGraphMtEphInv` | write! |
| 5 | 06 | UnDirGraphMtEph.rs | `UnDirGraphMtEphInv` | write! |
| 6 | 37 | BSTRBMtEph.rs | `BSTRBMtEphInv` | debug_struct |
| 7 | 37 | BSTSplayMtEph.rs | `BSTSplayMtEphInv` | debug_struct |
| 8 | 39 | BSTTreapMtEph.rs | `BSTTreapMtEphInv` | debug_struct |
| 9 | 39 | BSTTreapMtEph.rs | `Lnk` | debug_struct |
| 10 | 40 | BSTKeyValueStEph.rs | `Lnk` | write! |
| 11 | 40 | BSTReducedStEph.rs | `Lnk` | write! |
| 12 | 40 | BSTSizeStEph.rs | `Lnk` | write! |
| 13 | 43 | OrderedSetMtEph.rs | `OrderedSetMtEphInv` | write! |
| 14 | 43 | OrderedTableMtEph.rs | `OrderedTableMtEphInv` | write! |
| 15 | 47 | DoubleHashFlatHashTableStEph.rs | `DoubleHashFlatHashTableStEph` | write! |
| 16 | 47 | LinProbFlatHashTableStEph.rs | `LinProbFlatHashTableStEph` | write! |
| 17 | 47 | LinkedListChainedHashTableStEph.rs | `LinkedListChainedHashTableStEph` | write! |
| 18 | 47 | QuadProbFlatHashTableStEph.rs | `QuadProbFlatHashTableStEph` | write! |
| 19 | 47 | StructChainedHashTable.rs | `StructChainedHashTableStEph` | write! |
| 20 | 47 | VecChainedHashTableStEph.rs | `VecChainedHashTableStEph` | write! |
| 21 | 49 | MinEditDistMtEph.rs | `MinEditDistMtEphMemoInv` | write! |
| 22 | 49 | MinEditDistMtPer.rs | `MinEditDistMtPerMemoInv` | write! |
| 23 | 49 | SubsetSumMtEph.rs | `SubsetSumMtEphMemoInv` | write! |
| 24 | 49 | SubsetSumMtPer.rs | `SubsetSumMtPerMemoInv` | write! |
| 25 | 50 | OptBinSearchTreeMtEph.rs | `OptBSTMtEphMemoInv` | write! |
| 26 | 50 | OptBinSearchTreeMtPer.rs | `OptBSTMtPerMemoInv` | write! |
| 27 | 53 | GraphSearchMtPer.rs | `SelectAll` | write! |
| 28 | 53 | GraphSearchMtPer.rs | `SelectOne` | write! |
| 29 | 53 | GraphSearchStEph.rs | `SelectAll` | write! |
| 30 | 53 | GraphSearchStEph.rs | `SelectOne` | write! |
| 31 | 53 | GraphSearchStPer.rs | `SelectAll` | write! |
| 32 | 53 | GraphSearchStPer.rs | `SelectOne` | write! |
| 33 | 54 | BFSMtEph.rs | `BFSMtEph` | write! |
| 34 | 54 | BFSMtPer.rs | `BFSMtPer` | write! |
| 35 | 54 | BFSStEph.rs | `BFSStEph` | write! |
| 36 | 54 | BFSStPer.rs | `BFSStPer` | write! |
| 37 | 55 | CycleDetectStEph.rs | `CycleDetectStEph` | write! |
| 38 | 55 | CycleDetectStPer.rs | `CycleDetectStPer` | write! |
| 39 | 55 | DFSStEph.rs | `DFSStEph` | write! |
| 40 | 55 | DFSStPer.rs | `DFSStPer` | write! |
| 41 | 55 | SCCStEph.rs | `SCCStEph` | write! |
| 42 | 55 | SCCStPer.rs | `SCCStPer` | write! |
| 43 | 55 | TopoSortStEph.rs | `TopoSortStEph` | write! |
| 44 | 55 | TopoSortStPer.rs | `TopoSortStPer` | write! |
| 45 | 56 | PathWeightUtilsStEph.rs | `PathWeightUtilsStEphS` | write! |
| 46 | 56 | PathWeightUtilsStPer.rs | `PathWeightUtilsStPerS` | write! |
| 47 | 61 | EdgeContractionMtEph.rs | `EdgeContractionMtEph` | write! |
| 48 | 61 | EdgeContractionStEph.rs | `EdgeContractionStEph` | write! |
| 49 | 61 | VertexMatchingMtEph.rs | `VertexMatchingMtEph` | write! |
| 50 | 61 | VertexMatchingStEph.rs | `VertexMatchingStEph` | write! |
| 51 | 62 | StarContractionMtEph.rs | `StarContractionMtEph` | write! |
| 52 | 62 | StarContractionStEph.rs | `StarContractionStEph` | write! |
| 53 | 62 | StarPartitionMtEph.rs | `StarPartitionMtEph` | write! |
| 54 | 62 | StarPartitionStEph.rs | `StarPartitionStEph` | write! |
| 55 | 63 | ConnectivityMtEph.rs | `ConnectivityMtEph` | write! |
| 56 | 63 | ConnectivityStEph.rs | `ConnectivityStEph` | write! |
| 57 | 64 | SpanTreeMtEph.rs | `SpanTreeMtEph` | write! |
| 58 | 64 | SpanTreeStEph.rs | `SpanTreeStEph` | write! |
| 59 | 64 | TSPApproxStEph.rs | `TSPApproxStEph` | write! |
| 60 | 65 | PrimStEph.rs | `PrimStEph` | write! |
| 61 | 66 | BoruvkaMtEph.rs | `BoruvkaMtEph` | write! |
| 62 | 66 | BoruvkaStEph.rs | `BoruvkaStEph` | write! |

Each impl sat in section 14, outside `verus!`. `docs/DerivesInsideVerus.md` §4
row 7 puts the replacement in section 12, inside `verus!`; a derive is an
attribute, so in practice it sits on the type definition in section 4, which
is inside `verus!` and is the only place rustc accepts it. No section 14
header was emptied: every one of the 58 files keeps a `Display` impl or a
further `Debug` impl under the header the deleted impl shared.

### 3.4 Test expectations changed: none

`grep` over `tests/` for an assertion whose asserted value is a `format!` of a
`{:?}` or `{:#?}` finds five files. Three — `tests/Chap03/TestInsertionSortStEph.rs`,
`tests/Chap36/TestQuickSortStEph.rs`, `tests/Chap36/TestQuickSortMtEph.rs` —
use `{:?}` only inside the assertion's failure message, not in the value being
compared. The other two are
`tests/standards/Testusing_closures_standard.rs:32`
(`"ExampleS([0, 1, 2])"`) and
`tests/standards/Testwrapping_iterators_standard.rs:34`
(`"OuterS(InnerS([1, 1, 1]))"`), whose receivers are in `src/standards/` and
are not among the 62.

No test in the crate asserts on the formatted output of any of the 62 types,
and by §3.2 rows 1 and 2 the printed text is unchanged even where one did.
Zero expected strings were edited; the table the plan asked for is empty.

`docs/DerivesInsideVerus.md` §5's claim that "four `tests/` files assert on
formatted output" does not survive re-derivation: the count is five files, two
of which are assertions on a value and three of which are failure messages,
and none of the five touches this round's types.

### 3.5 Every `Debug` impl left alone, and why

Forty-seven impls print only the type name and were not replaced.

| # | Chap | File | Type | Field types | Why left alone |
|---|------|------|------|-------------|----------------|
| 1 | 02 | HFSchedulerMtEph.rs | `ExTaskState<T>` | `TaskState<T>` | tuple struct, derive prints the field |
| 2 | 02 | HFSchedulerMtEph.rs | `PoolState` | `Mutex`, `Condvar` | fields, no `Debug` |
| 3 | 05 | SetMtEph.rs | `LockedSetMtEph<T>` | `RwLock`, `Ghost` | fields, no `Debug` |
| 4 | 06 | DirGraphMtEph.rs | `LockedDirGraphMtEph<V>` | `RwLock`, `Ghost` | fields, no `Debug` |
| 5 | 06 | LabDirGraphMtEph.rs | `LockedLabDirGraphMtEph<V, L>` | `RwLock`, `Ghost` | fields, no `Debug` |
| 6 | 06 | LabUnDirGraphMtEph.rs | `LockedLabUnDirGraphMtEph<V, L>` | `RwLock`, `Ghost` | fields, no `Debug` |
| 7 | 06 | UnDirGraphMtEph.rs | `LockedUnDirGraphMtEph<V>` | `RwLock`, `Ghost` | fields, no `Debug` |
| 8 | 12 | Exercise12_1.rs | `SpinLock` | atomics | fields |
| 9 | 12 | Exercise12_5.rs | `Node<T>` | pointer fields | fields |
| 10 | 12 | Exercise12_5.rs | `ConcurrentStackMt<T>` | pointer fields | fields |
| 11 | 18 | ArraySeqMtEph.rs | `ArraySeqMtEphInv<T>` | 2 × `Seq` ghost | fields, no `Debug` |
| 12 | 37 | AVLTreeSeqStEph.rs | `AVLTreeSeqIterStEph<'a, T>` | iterator state | fields |
| 13 | 37 | AVLTreeSeqStPer.rs | `AVLTreeSeqStPerIter<'a, T>` | iterator state | fields |
| 14 | 37 | BSTAVLMtEph.rs | `BSTAVLMtEph<T>` | tree fields | fields |
| 15 | 37 | BSTAVLMtEph.rs | `BSTAVLMtEphInv<T>` | ghost `Set` | fields, no `Debug` |
| 16 | 37 | BSTBBAlphaMtEph.rs | `BSTBBAlphaMtEph<T>` | tree fields | fields |
| 17 | 37 | BSTBBAlphaMtEph.rs | `BSTBBAlphaMtEphInv<T>` | ghost `Set` | fields, no `Debug` |
| 18 | 37 | BSTPlainMtEph.rs | `BSTPlainMtEph<T>` | tree fields | fields |
| 19 | 37 | BSTPlainMtEph.rs | `BSTPlainMtEphInv<T>` | ghost `Set` | fields, no `Debug` |
| 20 | 37 | BSTRBMtEph.rs | `BSTRBMtEph<T>` | tree fields | fields |
| 21 | 37 | BSTSplayMtEph.rs | `BSTSplayMtEph<T>` | tree fields | fields |
| 22 | 38 | BSTParaMtEph.rs | `BSTParaMtEphInv<T>` | ghost `Set` | fields, no `Debug` |
| 23 | 38 | BSTParaMtEph.rs | `ParamBST<T>` | tree fields | fields |
| 24 | 38 | BSTParaStEph.rs | `BSTParaStEphInv<T>` | ghost `Set` | fields, no `Debug` |
| 25 | 38 | BSTParaStEph.rs | `ParamBST<T>` | tree fields | fields |
| 26 | 39 | BSTParaTreapMtEph.rs | `BSTParaTreapMtEphInv<T>` | ghost `Set` | fields, no `Debug` |
| 27 | 39 | BSTTreapMtEph.rs | `BSTTreapMtEph<T>` | tree fields | fields |
| 28 | 42 | TableStPer.rs | `TableStPer<K, V>` | collection fields | fields |
| 29 | 43 | OrderedTableMtPer.rs | `OrderedTableMtPerInv<K, V>` | ghost `Map` | fields, no `Debug` |
| 30 | 44 | Example44_1.rs | `TweetQueryExamples` | `Box<dyn Fn>` | `Example` file; fields, no `Debug` |
| 31 | 45 | Example45_2.rs | `Example45_2` | none | `Example` file |
| 32 | 50 | MatrixChainMtEph.rs | `MatrixChainMtEphDimInv` | ghost `Seq` | fields, no `Debug` |
| 33 | 50 | MatrixChainMtEph.rs | `MatrixChainMtEphMemoInv` | ghost | fields, no `Debug` |
| 34 | 50 | MatrixChainMtEph.rs | `MatrixChainMtEphV` | `Seq`, `Map` | ghost struct, fields |
| 35 | 50 | MatrixChainMtPer.rs | `MatrixChainMtPerMemoInv` | ghost | fields, no `Debug` |
| 36 | 50 | MatrixChainMtPer.rs | `MatrixChainMtPerV` | `Seq`, `Map` | ghost struct, fields |
| 37 | 50 | MatrixChainStEph.rs | `MatrixChainStEphV` | `Seq`, `Map` | ghost struct, fields |
| 38 | 50 | MatrixChainStPer.rs | `MatrixChainStPerV` | `Seq`, `Map` | ghost struct, fields |
| 39 | 50 | OptBinSearchTreeMtEph.rs | `OptBSTMtEphKeysInv<T>` | ghost `Seq` | fields, no `Debug` |
| 40 | 50 | OptBinSearchTreeMtEph.rs | `OBSTMtEphV<T>` | `Seq`, `Map` | ghost struct, fields |
| 41 | 50 | OptBinSearchTreeMtPer.rs | `OBSTMtPerV<T>` | `Seq`, `Map` | ghost struct, fields |
| 42 | 50 | OptBinSearchTreeStEph.rs | `OBSTStEphV<T>` | `Seq`, `Map` | ghost struct, fields |
| 43 | 50 | OptBinSearchTreeStPer.rs | `OBSTStPerV<T>` | `Seq`, `Map` | ghost struct, fields |
| 44 | 51 | TopDownDPMtEph.rs | `TopDownDPMtEphInv` | ghost | fields, no `Debug` |
| 45 | 51 | TopDownDPMtPer.rs | `TopDownDPMtPerInv` | ghost | fields, no `Debug` |
| 46 | 56 | Example56_1.rs | `Example56_1S` | none | `Example` file |
| 47 | 56 | Example56_3.rs | `Example56_3S` | none | `Example` file |

Rows 1 to 29, 30, and 32 to 45 have fields, so a derive prints a different
string (§3.2 row 3) and, wherever a field type is `Seq`, `Map`, `Set`,
`Ghost`, `RwLock`, `Mutex` or `Box<dyn Fn>`, is refused outright with `E0277`
(§3.2 row 4). Rows 31, 46 and 47 are unit structs whose derive would print
exactly the same text, and are the only replaceable impls this round did not
take: `CLAUDE.md` excludes `Example` and `Problem` files from proof and
refactoring work unless a prompt assigns them, and none did. Row 30 is in an
`Example` file as well but would be refused anyway.

The remaining 180 `Debug` impls format deliberately — `debug_struct` with
chosen fields, `debug_list`, a `{}` placeholder, or a summary such as
`src/Chap50/MatrixChainStEph.rs`'s `dimensions_len` and `memo_len` in place of
a whole `Vec` and `HashMap`. A derive changes what every one of them prints,
so none is a candidate, which is r216's judgement and is unchanged.

### 3.6 Cost

Nothing executable changed. A derived `Debug` for a unit struct emits
`f.write_str("X")`, which is what the `write!(f, "X")` and
`f.debug_struct("X").finish()` bodies compiled to: work O(1), span O(1),
before and after. `Debug` carries no specification either way —
`automatic_derive.rs` maps it to `Ignore` (`docs/DerivesInsideVerus.md` §1
row 5) — so no `ensures` was gained, lost or weakened. No `/// - Alg Analysis`
line was edited, and no discrepancy against one was found.

### 3.7 Part C results

Every count below is `scripts/validate.sh isolate ChapNN`, with 0 errors, 0
warnings and 0 trigger notes in all 19 chapters. The "before" column is the
same command run on `e2f528707`, parts A and B committed and part C absent,
extracted with `git archive` to a directory outside the repository so the live
tree was never reverted; those logs are under `logs/r217-controls/`.

| # | Chap | Impls | Before | After | Log after |
|---|------|------:|-------:|------:|-----------|
| 1 | 05 | 1 | 760 | 760 | validate.20260922-141645.log |
| 2 | 06 | 4 | 1037 | 1037 | validate.20260922-141650.log |
| 3 | 37 | 2 | 1863 | 1863 | validate.20260922-141536.log |
| 4 | 39 | 2 | 1219 | 1219 | validate.20260922-141600.log |
| 5 | 40 | 3 | 1181 | 1181 | validate.20260922-141609.log |
| 6 | 43 | 2 | 2687 | 2687 | validate.20260922-141617.log |
| 7 | 47 | 6 | 1161 | 1161 | validate.20260922-141659.log |
| 8 | 49 | 4 | 1283 | 1283 | validate.20260922-141707.log |
| 9 | 50 | 2 | 766 | 766 | validate.20260922-141714.log |
| 10 | 53 | 6 | 2246 | 2246 | validate.20260922-141722.log |
| 11 | 54 | 4 | 1277 | 1277 | validate.20260922-141741.log |
| 12 | 55 | 8 | 2290 | 2290 | validate.20260922-141749.log |
| 13 | 56 | 2 | 948 | 948 | validate.20260922-141809.log |
| 14 | 61 | 4 | 1243 | 1243 | validate.20260922-141817.log |
| 15 | 62 | 4 | 1256 | 1256 | validate.20260922-141828.log |
| 16 | 63 | 2 | 1271 | 1271 | validate.20260922-141841.log |
| 17 | 64 | 3 | 1271 | 1271 | validate.20260922-141853.log |
| 18 | 65 | 1 | 2531 | 2531 | validate.20260922-141905.log |
| 19 | 66 | 2 | 805 | 805 | validate.20260922-141930.log |

Nineteen chapters, nineteen unchanged counts. That is the measurement that
`Debug` is `Ignore`: a deleted `Debug` impl was never a checked function, and
the derive that replaces it is not one either.

Two of those "before" numbers disagree with `docs/ChaptersInOrder.md`, which
carries Chap39 1218 and Chap40 1180 from r213. Both measure 1219 and 1181
today with part C absent, so the difference predates r217 part C; the two rows
are brought up to date.

Run-time tests: `cargo-nextest` is not installed on this machine, so
`scripts/rtt.sh` exits with "no such command: nextest" and cannot be used.
The measurement is `cargo test --release --no-fail-fast` over the whole crate,
logged to `logs/rtt.20260922-142710.log`.

| # | Target kind | Targets | Pass | Fail |
|---|-------------|--------:|-----:|-----:|
| 1 | lib and integration tests | 265 | 4254 | 0 |
| 2 | doctests | 1 | 1 | 12 |

The 12 doctest failures are in the `//!` code fences of
`src/standards/partial_eq_eq_clone_standard.rs` (10) and
`src/standards/spec_wf_standard.rs` (2). They are Verus syntax inside
```` ``` ```` blocks, which rustdoc tries to compile as Rust and cannot —
`expected one of !, (, ), +, ,, ::, or <, found :` on
`fn clone_view(&self) -> (result: Self)`, and
`cannot find vstdplus in the crate root` on a `use crate::vstdplus::...`
line. Neither file was touched by r217: `git log` puts their last change at
`3e539ce09`, the r206 to r211 migration. They have never appeared in a project
RTT because `cargo nextest` does not run doctests. They are recorded here and
left alone.

## 4. What the three parts did to the full crate

`scripts/validate.sh` over the whole crate is not in `logs/` for any round
before this one, so the round's effect on the full run had to be measured
rather than inferred. Three runs, same Verus, same flags, differing only in
the source tree.

| # | Tree | Commit | Verified | Errors |
|---|------|--------|---------:|-------:|
| 1 | before part A | 15a289dec | 5606 | 8 |
| 2 | parts A and B | e2f528707 | 5607 | 8 |
| 3 | parts A, B and C | working tree | 5607 | 8 |

Runs 2 and 3 fail on the same eight functions, so **part C changes nothing in
the full run**. Run 1 differs from runs 2 and 3 in one position: it fails on
`src/Chap45/BalancedTreePQ.rs:712` and not on
`src/Chap39/BSTTreapStEph.rs:2115`, and runs 2 and 3 do the opposite. Neither
file was edited in any part of r217, and both chapters verify with 0 errors
under `isolate`. Six of the eight failures are `rlimit exceeded` and two are
the "Verus failed to prove an assertion even though all of its sub-assertions
succeeded" shape, which is the message for a proof at the edge of its budget.
Adding items anywhere in the crate changes the axiom set every query carries,
so a proof already at the edge can cross it in either direction; that is what
the one swapped position is.

| # | Chap | File | Function or line | In run 1 | In runs 2, 3 |
|---|------|------|------------------|----------|--------------|
| 1 | 26 | ScanDCMtPer.rs | `prefix_sums_dc_inner` | rlimit | rlimit |
| 2 | 26 | ETSPMtEph.rs | `lemma_combined_cycle` | rlimit | rlimit |
| 3 | 26 | ETSPMtEph.rs | `etsp_parallel_inner` | rlimit | rlimit |
| 4 | 35 | OrderStatSelectMtEph.rs | assertion at 553 | failed | failed |
| 5 | 35 | OrderStatSelectMtPer.rs | assertion at 551 | failed | failed |
| 6 | 65 | UnionFindArrayStEph.rs | `lemma_find_after_link` | rlimit | rlimit |
| 7 | 65 | UnionFindArrayStEph.rs | `lemma_link_preserves_wf` | rlimit | rlimit |
| 8 | 45 | BalancedTreePQ.rs | line 712 | rlimit | passes |
| 9 | 39 | BSTTreapStEph.rs | line 2115 precondition | passes | failed |

Rows 1 to 7 are eight-for-eight identical across all three runs and are
r217's inheritance, not its product. Rows 8 and 9 are the swap. Logs:
`logs/r217-controls/validate-full-at-15a289dec.log`,
`logs/r217-controls/validate-full-at-e2f528707.log`, and
`logs/validate.20260922-141938.log`. None of the nine is a proof hole; they
are proofs that do not currently discharge in the full-crate context, which is
work for a round that can profile them.

## 5. Proof holes, before and after

`scripts/holes.sh src/ChapNN/` on all 19 chapters part C touches, before at
`e2f528707` and after on the working tree. `scratch/` and `bugs/` are outside
`src/` and are not reached by the command.

| # | Chap | Holes before | Holes after | `accept()` before | `accept()` after |
|---|------|-------------:|------------:|------------------:|-----------------:|
| 1 | 05 | 10 | 10 | 6 | 6 |
| 2 | 06 | 29 | 29 | 25 | 25 |
| 3 | 37 | 88 | 88 | 23 | 23 |
| 4 | 39 | 31 | 31 | 15 | 15 |
| 5 | 40 | 23 | 23 | 20 | 20 |
| 6 | 43 | 65 | 65 | 7 | 7 |
| 7 | 47 | 6 | 6 | 5 | 5 |
| 8 | 49 | 2 | 2 | 0 | 0 |
| 9 | 50 | 18 | 18 | 18 | 18 |
| 10 | 53 | 2 | 2 | 0 | 0 |
| 11 | 54 | 0 | 0 | 0 | 0 |
| 12 | 55 | 0 | 0 | 0 | 0 |
| 13 | 56 | 0 | 0 | 0 | 0 |
| 14 | 61 | 3 | 3 | 0 | 0 |
| 15 | 62 | 1 | 1 | 0 | 0 |
| 16 | 63 | 0 | 0 | 0 | 0 |
| 17 | 64 | 0 | 0 | 0 | 0 |
| 18 | 65 | 4 | 4 | 1 | 1 |
| 19 | 66 | 3 | 3 | 1 | 1 |
| — | total | 285 | 285 | 121 | 121 |

Identical in every chapter. The only change in the 19
`analyses/veracity-review-verus-proof-holes.log` files is the line number each
hole is reported at, shifted by the `#[derive(Debug)]` line added above it and
by the impl deleted below it. No hole was introduced, closed or moved between
categories, which is what §3.6 predicts: `Debug` carries no specification, so
deleting or deriving one cannot touch a proof obligation.

That is also the honest summary of part C's value. It deletes 326 lines of
source that state nothing and adds 62 attribute lines that state the same
nothing, more briefly and in a place `docs/DerivesInsideVerus.md` §4 now
prefers. The proof surface is untouched, by construction.

## 6. What the derives could not replace, after all

| # | Gap | Count | Reason, with its evidence |
|---|-----|------:|---------------------------|
| 1 | `Debug` on a type with fields | 44 | derive prints the fields, or `E0277`; §3.2 rows 3 and 4 |
| 2 | `Debug` on a unit struct in an `Example` file | 3 | replaceable, but `CLAUDE.md` excludes `Example` files |
| 3 | `Debug` that formats deliberately | 180 | derive changes the printed text |
| 4 | `Clone` and `PartialEq` carrying an `accept` | 96 | generic or `Vec`-backed receiver; `docs/DerivesInsideVerus.md` §6 |
| 5 | `StructuralEq` on a generic type | 6 | `E0107`; §2.1 rows 9 to 14 |
| 6 | `StructuralEq` reaching an `f64` | 1 | `E0277`; §2.4 row 2 |
| 7 | `Ord` and `PartialOrd` | 16 | no `StructuralOrd` derive; `docs/DerivesInsideVerus.md` §8 row 5 |
| 8 | `Hash` and `obeys_key_model` | 9 | no proof rule; `docs/DerivesInsideVerus.md` §8 row 6 |
| 9 | `Default` with a value in its `ensures` | 18 | derive gives no postcondition; `docs/DerivesInsideVerus.md` §8 row 7 |

Row 4 is the one that matters and is unchanged by all three parts: the 96
bridge `accept`s stand exactly where r216 found them, and
`docs/DerivesInsideVerus.md` §8 item 2 still names the way to retire them — a
vstd law giving `obeys_view_eq::<Vec<T>>()` from `obeys_view_eq::<T>()` —
which is a proof, not a derive.

## 7. Round totals

| # | Part | Impls deleted | Derives added | Files | Chapters | Verified delta | Holes delta |
|---|------|--------------:|--------------:|------:|---------:|---------------:|------------:|
| 1 | A | 11 | 6 | 4 | 2 | 0 | 0 |
| 2 | B | 0 | 6 | 7 | 4 | +1 | 0 |
| 3 | C | 62 | 62 | 58 | 19 | 0 | 0 |
| — | total | 73 | 74 | 69 | 21 | +1 | 0 |

Part B deletes nothing: it adds six `StructuralEq` derives and moves one
`PartialEq` impl from section 14 into section 12 unaltered. The chapter total
is 21 and not 25 because Chap37, 43, 47 and 50 appear in both parts B and C.

The round's one new proved fact is part B's `OrderedSetStPer::eq`, and its one
retired unchecked claim is part A's `DijkstraStEphU64` `PartialEqSpecImpl`.
Everything else is 74 impls of text replaced by 74 attributes with the same
meaning.
