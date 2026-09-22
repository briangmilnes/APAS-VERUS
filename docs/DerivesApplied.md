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
