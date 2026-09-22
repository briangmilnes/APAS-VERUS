# Derives inside `verus!` on Verus 0.2026.09.13

Round r216. Date 2026-09-22. Verus 0.2026.09.13.671956e.
Plan: `plans/r216-derive-inside-verus.md`.

The question this round answers: how much of
`#[derive(Clone, Copy, PartialEq, Eq, Debug)]` can be written inside `verus!`,
what does the prover learn from each derived impl, and which of APAS-VERUS's
615 hand-written `Clone`, `PartialEq`, `Eq`, `Debug`, `Default`, `Hash`,
`PartialOrd` and `Ord` impls can a derive replace.

The short answer: every one of the five derives compiles inside `verus!`, but
only `Clone` on a non-generic `Copy` type carries a specification, and only a
sixth derive, `StructuralEq`, specifies `==`. Neither reaches a generic type or
a type holding a `Vec`, which is what every APAS collection is. The 96
`accept` holes in the crate's `Clone` and `PartialEq` bodies are unaffected.

## 1. How Verus 0.2026.09.13 treats each derive

The mechanism is `rust_verify/src/automatic_derive.rs`. `get_action` assigns
each derived trait one of three actions.

| # | Derive | Action in `automatic_derive.rs` | What the prover gets |
|---|--------|--------------------------------|----------------------|
| 1 | `Clone` | `Special(SpecialTrait::Clone)` | `ensures ret == self`, but only when the emitted body reads `self` |
| 2 | `Copy` | `VerifyAsIs` | nothing to specify; copies are equal by Verus's value model |
| 3 | `PartialEq` | `Ignore` | nothing; `obeys_eq_spec()` stays undefined |
| 4 | `Eq` | `Ignore` | nothing; `Eq` is a marker |
| 5 | `Debug` | `Ignore` | nothing; the impl passes through |
| 6 | `Default` | `Ignore` | nothing; `default()` has no postcondition |
| 7 | `Hash` | `Ignore` | nothing; `obeys_key_model` stays uninterpreted |
| 8 | `PartialOrd` | `Ignore` | nothing; `obeys_partial_cmp_spec()` undefined |
| 9 | `Ord` | `Ignore` | nothing; `obeys_cmp_spec()` undefined |
| 10 | `Structural` | marker impl only | nothing on its own |
| 11 | `StructuralEq` | marker plus emitted `PartialEqSpecImpl` | `obeys_eq_spec() == true`, `eq_spec == spec_eq` |

`clone_add_post_condition` inspects the derived body. A body that reads `self`,
which rustc emits for a non-generic `Copy` type, gets `ensures ret == self`.
A body that is a constructor, which rustc emits for a non-`Copy` type and for
every generic type, gets the warning
`autoderive Clone impl when the clone is not a copy` and no specification. A
body that is a `match`, which rustc emits for a non-`Copy` enum, gets
`autoderive Clone impl does not take the form Verus expects` and no
specification.

`StructuralEq` (`builtin_macros/src/structural.rs`, `derive_structural_eq`)
emits two items: the `unsafe impl Structural`, generated through
`synstructure::gen_impl`, which carries generic parameters; and an
`impl vstd::std_specs::cmp::PartialEqSpecImpl for #name`, written with `quote!`
on the bare type name, which does not. That asymmetry is why `StructuralEq`
fails on a generic type.

## 2. Re-run of the 28 existing derive experiments

The `SUCCEEDS` markers in `src/lib.rs` predate this release by about a year.
All 28 were re-run on 0.2026.09.13 on 2026-09-22. No verdict changed; the
`DATE`, `VERUS` and `LOG` headers in each file were updated, and each file's
`RESULT` now states what the prover can conclude, which the old headers did
not.

Logs are `logs/validate-standard-<name>.20260922-<hhmmss>.log`; the timestamps
run from 111914 to 111927.

| # | Chap | File | Old | New | Verus counts |
|---|------|------|-----|-----|--------------|
| 1 | exp | derive_clone_enum_in_verus.rs | SUCCEEDS | SUCCEEDS | 1 verified, 1 warning |
| 2 | exp | derive_clone_struct_in_verus.rs | SUCCEEDS | SUCCEEDS | 1 verified, 1 warning |
| 3 | exp | derive_clone_struct_with_vec_in_verus.rs | SUCCEEDS | SUCCEEDS | 1 verified, 1 warning |
| 4 | exp | derive_copy_enum_in_verus.rs | SUCCEEDS | SUCCEEDS | 1 verified, 0 errors |
| 5 | exp | derive_copy_struct_in_verus.rs | SUCCEEDS | SUCCEEDS | 1 verified, 0 errors |
| 6 | exp | derive_debug_enum_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 7 | exp | derive_debug_struct_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 8 | exp | derive_debug_struct_with_vec_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 9 | exp | derive_default_enum_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 10 | exp | derive_default_struct_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 11 | exp | derive_default_struct_with_vec_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 12 | exp | derive_display_enum_in_verus.rs | FAILS | FAILS | E0432 `derive_more` |
| 13 | exp | derive_display_struct_in_verus.rs | FAILS | FAILS | E0432 `derive_more` |
| 14 | exp | derive_eq_enum_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 15 | exp | derive_eq_struct_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 16 | exp | derive_eq_struct_with_vec_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 17 | exp | derive_hash_enum_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 18 | exp | derive_hash_struct_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 19 | exp | derive_hash_struct_with_vec_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 20 | exp | derive_ord_enum_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 21 | exp | derive_ord_struct_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 22 | exp | derive_ord_struct_with_vec_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 23 | exp | derive_partial_eq_enum_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 24 | exp | derive_partial_eq_struct_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 25 | exp | derive_partial_eq_struct_with_vec_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 26 | exp | derive_partial_ord_enum_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 27 | exp | derive_partial_ord_struct_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |
| 28 | exp | derive_partial_ord_struct_with_vec_in_verus.rs | SUCCEEDS | SUCCEEDS | 0 verified, 0 errors |

These 28 files ask only "does it compile". Twenty-two of them report
`0 verified, 0 errors`, which means Verus found no function to check: the
derived impl is ignored and the file declares nothing else. A `SUCCEEDS` of
that shape is evidence that the derive is accepted, and evidence of nothing
further. The 22 new experiments in section 3 were written to ask what the
prover can conclude.

## 3. The 22 new experiments

All are in `src/experiments/`, named `derive_0913_*`, verified individually
with `scripts/validate-standard.sh --experiment <name>`, and listed
commented-out in `src/lib.rs`. Logs are
`logs/validate-standard-<name>.20260922-<hhmmss>.log`.

| # | Chap | File | Question | Result |
|---|------|------|----------|--------|
| 1 | exp | derive_0913_clone_copy_struct.rs | derived `Clone`, `Copy` struct | SUCCEEDS, 2 verified |
| 2 | exp | derive_0913_clone_copy_enum.rs | derived `Clone`, `Copy` enum | SUCCEEDS, 2 verified |
| 3 | exp | derive_0913_clone_vec_struct.rs | derived `Clone`, `Vec` field | FAILS, 1 error |
| 4 | exp | derive_0913_clone_generic_struct.rs | derived `Clone`, generic | FAILS, 1 error |
| 5 | exp | derive_0913_clone_enum.rs | derived `Clone`, non-`Copy` enum | FAILS, 1 error |
| 6 | exp | derive_0913_copy_struct.rs | `Copy` accepted, copy equals source | SUCCEEDS, 2 verified |
| 7 | exp | derive_0913_copy_generic_struct.rs | `Clone, Copy` on a generic struct | FAILS, 1 error |
| 8 | exp | derive_0913_partial_eq_struct.rs | derived `PartialEq` alone | FAILS, 1 error |
| 9 | exp | derive_0913_structural_eq_struct.rs | `StructuralEq` on a struct | SUCCEEDS, 3 verified |
| 10 | exp | derive_0913_structural_eq_enum.rs | `StructuralEq` on an enum | SUCCEEDS, 3 verified |
| 11 | exp | derive_0913_structural_eq_vec_struct.rs | `StructuralEq` with a `Vec` field | FAILS, E0277 |
| 12 | exp | derive_0913_structural_eq_generic_struct.rs | `StructuralEq` on a generic | FAILS, E0107 |
| 13 | exp | derive_0913_structural_eq_view_bridge.rs | `StructuralEq` vs the APAS view spec | SUCCEEDS, 2 verified |
| 14 | exp | derive_0913_structural_marker_only.rs | `Structural` without `StructuralEq` | FAILS, 1 error |
| 15 | exp | derive_0913_debug_in_verus.rs | derived `Debug` inside `verus!` | SUCCEEDS, 1 verified |
| 16 | exp | derive_0913_default_spec.rs | value of a derived `default()` | FAILS, 1 error |
| 17 | exp | derive_0913_hash_key_model.rs | derived `Hash` vs `obeys_key_model` | FAILS, 1 error |
| 18 | exp | derive_0913_ord_obeys_cmp.rs | derived `Ord` vs `<=` and `obeys_cmp` | FAILS, 2 errors |
| 19 | exp | derive_0913_ord_with_ordspecimpl.rs | hand `OrdSpecImpl` beside derived `Ord` | SUCCEEDS, trusted |
| 20 | exp | derive_0913_ord_totalorder_bridge.rs | APAS `TotalOrder` from a derived `Ord` | SUCCEEDS, 18 of 197 |
| 21 | exp | derive_0913_all_five_struct.rs | the five-derive line | PARTIAL, 2 verified, 1 error |
| 22 | exp | derive_0913_all_five_plus_structural.rs | the six-derive line | SUCCEEDS, 4 verified |

The facts each experiment establishes, in the order of the table:

1. `p.clone()` gives `cloned == *p`, and `cloned@ == p@` follows by congruence,
   with no `accept`.
2. The same holds for an enum once `Copy` is derived alongside `Clone`.
3. `cloned@ == s@` fails: "datatype is opaque here".
4. `cloned.value == s.value` fails, the weakest fact a caller could want.
5. `cloned == *n` fails; the enum warning names a different body shape.
6. A copying move leaves an equal value; Verus models the copy, not the trait.
7. `cloned == *b` fails although `Copy` is derived: rustc emits a constructor
   body for a generic struct, so the `Copy` exemption does not apply.
8. Both directions of `equal == (*a == *b)` fail.
9. `equal == (*a == *b)` holds, and `axiom_structural_obeys_concrete_eq` fires,
   so a generic function requiring `obeys_concrete_eq::<T>()` is callable.
10. The same holds for a non-generic enum, including a variant test.
11. `error[E0277]: the trait bound Vec<u64>: Structural is not satisfied`.
12. `error[E0107]: missing generics for struct BoxS`.
13. With an injective view, `StructuralEq` gives the APAS postcondition
    `equal == (a@ == b@)` in full; with a view that drops a field, it gives
    `equal ==> a@ == b@` and not the converse.
14. `obeys_concrete_eq()` is uninterpreted: the marker alone is not enough.
15. The derive compiles on a struct, an enum and a generic struct, and the
    impl is reachable from `format!` outside the `verus!` block.
16. `c.n == 0` fails after `CounterS::default()`.
17. `obeys_key_model()` is uninterpreted; vstd's own documentation says a user
    key type must `assume` it.
18. `<=` proves nothing, and `obeys_cmp` breaks down with `obeys_eq_spec()`
    satisfied by `StructuralEq` and the four ordering properties uninterpreted.
19. A hand-written `OrdSpecImpl` does specify `cmp`, `lt` and `le`, but Verus
    never checks the derived body against `cmp_spec`, so
    `obeys_cmp_spec() == true` beside a derived `Ord` is an unchecked claim of
    the same standing as an `assume`.
20. With that `OrdSpecImpl` in place, all six `TotalOrder` members take empty
    proof bodies, including the two that would otherwise fall back to the
    trait's assuming defaults. The comment in `src/vstdplus/total_order.rs`
    about user types needing those defaults is out of date.
21. The user's exact line compiles and verifies the clone fact; the equality
    fact fails.
22. Adding `StructuralEq` gives the clone fact, both equality facts and
    `obeys_concrete_eq`, replacing a hand-written `Clone`, `PartialEq`, `Eq`,
    `PartialEqSpecImpl` and `Debug` and two `accept` holes with one attribute —
    on a non-generic struct of primitives.

## 4. Per-trait verdict

| # | Trait | Verdict | Reason |
|---|-------|---------|--------|
| 1 | `Clone` | derive inside `verus!`, `Copy` non-generic only | spec only for a `self`-reading body |
| 2 | `Clone` | hand-write, generic or `Vec`-backed | derive attaches no postcondition |
| 3 | `Copy` | derive inside `verus!` | `VerifyAsIs`; nothing to specify |
| 4 | `PartialEq` | derive plus `StructuralEq`, non-generic all-`Structural` | supplies `PartialEqSpecImpl` |
| 5 | `PartialEq` | hand-write otherwise | `StructuralEq` refused, bare derive silent |
| 6 | `Eq` | derive inside `verus!` | marker; derive and hand impl are equivalent |
| 7 | `Debug` | derive inside `verus!`, section 12 | `Ignore`; no reason to sit in section 14 |
| 8 | `Default` | hand-write when the value matters | derive gives no postcondition |
| 9 | `Hash` | hand-write | no derive reaches `obeys_key_model` |
| 10 | `PartialOrd` | hand-write | derive gives no `obeys_partial_cmp_spec` |
| 11 | `Ord` | hand-write | derive gives no `obeys_cmp_spec` |
| 12 | `Structural` | do not use alone | use `StructuralEq` |

Two verdicts deserve their reasons spelled out.

`Debug` may move inside `verus!`. APAS's table-of-contents standard puts it in
section 14, outside the block, and that placement was correct for older
releases. On 0.2026.09.13 a derived `Debug` inside `verus!` compiles and is
usable from ordinary Rust. A hand-written `Debug` with a `write!` body still
belongs outside, because its body is exec Rust that Verus would have to
process.

`Ord` should be hand-written rather than derived-plus-`OrdSpecImpl`. Both forms
compile, and both specify `cmp`, but only the hand-written body is checked
against the spec.

The crate already carries three instances of that unchecked shape for
`PartialEq`. Comparing the 73 `PartialEqSpecImpl` declarations against the 73
hand-written `PartialEq` impls that Verus sees gives three types whose
`obeys_eq_spec() == true` is asserted over a body Verus never inspects:

| # | Chap | File | Type | Why unchecked |
|---|------|------|------|---------------|
| 1 | 57 | DijkstraStEphU64.rs | `PQEntry` | `PartialEq` is derived, so ignored |
| 2 | 57 | DijkstraStEphF64.rs | `PQEntry` | `PartialEq` is derived, so ignored |
| 3 | 43 | OrderedSetStPer.rs | `OrderedSetStPer<T>` | `PartialEq` is in section 14 |

Rows 1 and 2 are a derive Verus ignores. Row 3 is different and worse: the
`PartialEqSpecImpl` is inside `verus!`, and the `PartialEq` body it describes
sits outside the block in section 14, where Verus cannot see it at all. That
also departs from `src/standards/partial_eq_eq_clone_standard.rs`, which puts
`PartialEq` in section 12, inside `verus!`. Row 1 is fixed by `StructuralEq`,
which emits the same spec with a compiler-checked marker. Row 3 is fixed by
moving the impl into `verus!` and giving it the standard `ensures`.

## 5. Inventory of the crate's manual impls

Counted over `src/`, excluding `src/experiments/`, `src/standards/` and
`analyses/`. "Inside" is measured by parsing each file with
`veracity-paths-read`, which sees only what is inside a `verus!` block; the
totals come from `grep`.

| # | Trait | Total | Inside `verus!` | Outside | With `accept` |
|---|-------|-------|-----------------|---------|---------------|
| 1 | `Clone` | 151 | 144 | 7 | 48 |
| 2 | `PartialEq` | 79 | 63 | 16 | 48 |
| 3 | `Eq` | 73 | 68 | 5 | 0 |
| 4 | `Debug` | 246 | 0 | 246 | 0 |
| 5 | `Default` | 41 | 38 | 3 | 0 |
| 6 | `Hash` | 9 | 6 | 3 | 0 |
| 7 | `PartialOrd` | 8 | 4 | 4 | 0 |
| 8 | `Ord` | 8 | 4 | 4 | 0 |
| 9 | `Copy` | 7 | 7 | 0 | 0 |
| 10 | `PartialEqSpecImpl` | 73 | 73 | 0 | 0 |
| — | total | 695 | 407 | 288 | 96 |

Of the 154 `fn clone(&self)` bodies, 116 carry an `ensures`; of the 93
`fn eq(&self, ...)` bodies, 72 do. Those contracts are what a derive would
destroy: the derived impl carries no `ensures` at all.

The 96 `accept` bodies, by the shape of the receiving type:

| # | Shape of the receiving type | Clone | PartialEq | Replaceable |
|---|-----------------------------|-------|-----------|-------------|
| 1 | generic in `T` | 42 | 33 | no, E0107, no clone spec |
| 2 | `Vec`/`HashMap`/collection field | 6 | 15 | no, E0277, no clone spec |
| — | total | 48 | 48 | 0 |

No `accept` body sits on a type a derive can reach. The impls a derive can
replace are a disjoint, smaller set, and none of them carries an `accept`:

| # | Chap | File | Impls | Replacement |
|---|------|------|-------|-------------|
| 1 | 26 | ETSPStEph.rs | `Copy`+`Clone`, `Point` and `Edge` | `#[derive(Clone, Copy)]` |
| 2 | 26 | ETSPMtEph.rs | `Copy`+`Clone`, `Point` and `Edge` | `#[derive(Clone, Copy)]` |
| 3 | 57 | DijkstraStEphU64.rs | `Clone`, `PartialEqSpecImpl` | `Clone, Copy, StructuralEq` |
| 4 | 57 | DijkstraStEphF64.rs | `Clone` for `PQEntry` | `#[derive(Clone, Copy)]` |

That is 11 impls: eight in Chap26, two in `DijkstraStEphU64.rs`, one in
`DijkstraStEphF64.rs`. `Point` and `Edge` have `f64` fields and
`DijkstraStEphF64.rs`'s `PQEntry` holds a `WrappedF64`; `f64` is not
`Structural`, so those five types take `Clone, Copy` and not `StructuralEq`.
`src/vstdplus/threads_plus.rs`'s `impl Copy for IsThreadPlus` was examined and
excluded: `IsThreadPlus` is a `tracked struct` whose `Clone` lives outside
`verus!`, and a derive on a ghost type was not measured this round.

`Debug` is a separate case. All 246 impls are outside `verus!` and none is
proof-carrying, so a derive cannot lose a specification; the question is only
the printed text. Of the 246 bodies, 86 are a constant string naming the type,
130 use `debug_struct`, `debug_list`, `debug_tuple` or `debug_map` with chosen
fields, and 50 use a format placeholder. The 86 constant-string bodies print
strictly less than a derive would and are the honest candidates. The other 160
abbreviate on purpose — `src/Chap50/MatrixChainStEph.rs` prints
`dimensions_len` and `memo_len` rather than a whole `Vec` and `HashMap` — and a
derive would change what they print. Four `tests/` files assert on formatted
output, so any `Debug` change must be checked against them.

Strengthening, rather than deletion, is available at eight existing derive
sites that today have an unspecified `==`:

| # | Chap | File | Type | Add |
|---|------|------|------|-----|
| 1 | 50 | MatrixChainStEph.rs | `MatrixDim` | `StructuralEq` |
| 2 | 50 | MatrixChainStPer.rs | `MatrixDim` | `StructuralEq` |
| 3 | 50 | MatrixChainMtEph.rs | `MatrixDim` | `StructuralEq` |
| 4 | 50 | MatrixChainMtPer.rs | `MatrixDim` | `StructuralEq` |
| 5 | 37 | BSTRBMtEph.rs | `Color` | `StructuralEq` |
| 6 | 47 | ParaHashTableStEph.rs | `LoadAndSize` | `StructuralEq` |
| 7 | 57 | DijkstraStEphU64.rs | `PQEntry` | `StructuralEq` |
| 8 | — | Types.rs | `Edge`, `LabEdge`, `WeightedEdge`, `Pair` | blocked, generic |

Row 8 is the important one. `Types::Pair<K, V>` derives
`PartialEq, Eq, PartialOrd, Ord, Hash` and has no `PartialEqSpecImpl`, so
`pair_a == pair_b` in exec code proves nothing anywhere in the crate. The same
holds for `Edge<V>`, `LabEdge<V, L>` and `WeightedEdge<V, W>`. `StructuralEq`
cannot reach them because they are generic, so the fix is a hand-written
`PartialEqSpecImpl` per type — with the trust caveat of section 4 — or the
upstream fix in section 7.

## 6. The `accept` count a derive would remove

| # | Location | Pattern | Count | Removed by a derive |
|---|----------|---------|-------|---------------------|
| 1 | `Clone::clone` bodies | `accept(cloned@ == self@)` etc. | 48 | 0 |
| 2 | `PartialEq::eq` bodies | `accept(equal == (self@ == other@))` | 48 | 0 |
| 3 | `Clone::clone` bodies | `assume(...)` | 0 | 0 |
| 4 | `PartialEq::eq` bodies | `assume(...)` | 0 | 0 |
| — | total | | 96 | 0 |

The crate holds no `assume` in a `Clone` or `PartialEq` body; all 96 bridge
holes are `accept`. Not one is removable by a derive on 0.2026.09.13, because
every receiving type is generic or holds a `Vec`, `HashMap` or APAS collection,
and both blockers are hard: E0107 for `StructuralEq` on a generic, E0277 for
`Vec` not being `Structural`, and no clone postcondition for any constructor
body.

What stays trusted after any derive round is therefore the whole of the 96,
plus two `external_body` `Hash` bodies in `src/Chap05/SetStEph.rs` and
`src/Chap05/SetMtEph.rs`, plus the `obeys_key_model` assumption every
`HashMap`- and `HashSet`-backed module needs, plus the three unchecked
`PartialEqSpecImpl` declarations of section 4. Switching
`src/Chap57/DijkstraStEphU64.rs` to `StructuralEq` and moving
`src/Chap43/OrderedSetStPer.rs`'s `PartialEq` into `verus!` reduces that last
group from three to one.

## 7. Proposed order for a deletion round

The measured benefit is small, so the order below is by risk, cheapest and
safest first. Nothing here changes an algorithm or a cost specification.

| # | Step | Chap | Files | Impls | Check against |
|---|------|------|-------|-------|---------------|
| 0 | move `PartialEq` into `verus!` | 43 | 1 | 0 deleted | `OrderedSetStPer` `==` sites verify |
| 1 | add `StructuralEq` | 50, 37, 47 | 6 | 0 deleted | `==` sites still verify |
| 2 | `StructuralEq` for `PQEntry` | 57 | 1 | 1 deleted | `PartialEqSpecImpl` gone, `==` verifies |
| 3 | `Clone, Copy` for `Point`, `Edge` | 26 | 2 | 8 deleted | `ensures cloned == *self` callers |
| 4 | `Clone, Copy` for `PQEntry` | 57 | 2 | 2 deleted | `ensures cloned@ == self@` callers |
| 5 | derived `Debug`, constant-string only | many | up to 86 | 86 deleted | the four `tests/` format assertions |

Steps 0 to 2 add or check specifications the crate does not have today and
delete one impl; they are the highest value per edit. Steps 3 and 4 delete 10
impls with no change in what is proved. Step 5 is 86 deletions but changes
printed output, so it wants its own round and an RTT pass.

Each step must be checked by `scripts/validate.sh isolate ChapNN` for the
chapters it touches, then a full `scripts/validate.sh`, then `scripts/rtt.sh`
for step 5. The `Clone` steps must confirm that the derived
`ensures ret == self` is at least as strong as the hand-written `ensures` it
replaces: for `Point` and `Edge` the hand-written form is literally
`ensures cloned == *self`, and for `PQEntry` it is `ensures cloned@ == self@`,
which follows from structural equality because `PQEntry`'s `View` is the
identity.

## 8. What still cannot be derived, and the upstream work

| # | Gap | Blocks | Evidence |
|---|-----|--------|----------|
| 1 | `StructuralEq` drops generic parameters | 33 `PartialEq` impls, `Types::Pair` | E0107, experiment 12 |
| 2 | `Vec` is not `Structural` | 15 `PartialEq` impls | E0277, experiment 11 |
| 3 | derived `Clone` has no spec unless the body reads `self` | 48 `Clone` impls | experiments 3, 4, 5 |
| 4 | the `Copy` clone exemption misses generic types | every generic APAS collection | experiment 7 |
| 5 | no `StructuralOrd` | 8 `Ord`, 8 `PartialOrd` impls | experiment 18 |
| 6 | `obeys_key_model` has no proof rule | 2 `external_body` Hash bodies | experiment 17 |
| 7 | derived `Default` has no postcondition | 18 `Default` impls with `ensures` | experiment 16 |
| 8 | `f32` and `f64` are not `Structural` | Chap26, Chap57 F64 | `builtin/src/lib.rs` TODO |

Items 1 and 4 are the two that would change the crate most, and both look
mechanical upstream.

Item 1: `derive_structural_eq` in `builtin_macros/src/structural.rs` builds the
`Structural` impl with `synstructure::gen_impl`, which carries generics, and
then builds the `PartialEqSpecImpl` with a plain `quote!` on `#name`. Routing
the second through `gen_impl` as well, or splicing `s.ast().generics` into it,
would make `StructuralEq` work on generic types. It would still need each field
to be `Structural`, so item 2 governs how much that buys APAS.

Item 4: `clone_add_post_condition` in `rust_verify/src/automatic_derive.rs`
recognises `ExprX::ReadPlace(Local("self"))` and treats everything else as
unsupported. rustc emits a constructor body for a generic `Copy` struct, so a
generic type never takes the recognised branch. A `Ctor` body whose every field
expression is a read of the corresponding field of `self` is the same value as
`self`, so the same `ensures ret == self` could be attached to it. That would
cover every APAS type whose fields are themselves `Copy`.

Item 2 is the one that matters most for APAS and looks least mechanical:
`Vec<T>` is an external type with an `external_body` `PartialEq`, so declaring
it `Structural` would be a claim about the standard library, not a derivation.
The alternative shape is a `VecStructural`-style law in vstd stating
`obeys_view_eq::<Vec<T>>()` given `obeys_view_eq::<T>()`, which would let an
APAS `PartialEq` body prove `equal == (self@ == other@)` instead of accepting
it, and would retire all 48 `PartialEq` accepts without any derive at all.
That, rather than a derive round, is where the proof budget belongs.

Items 5 and 6 are feature requests: a `StructuralOrd` derive emitting
`OrdSpecImpl` and `PartialOrdSpecImpl` with lexicographic specs, checked
against the derived body; and a proof rule for `obeys_key_model` on a type
whose `Hash`, `PartialEq` and `Clone` are all derived, which vstd's own
documentation already names as planned work.
