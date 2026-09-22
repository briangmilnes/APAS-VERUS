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
