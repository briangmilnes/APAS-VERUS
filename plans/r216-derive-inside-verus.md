
# r216 — what `#[derive(...)]` can do inside `verus!` on 0.2026.09.13

Date: 2026-09-22. One subagent, background. Subject: `~/projects/APAS-VERUS`.

## Question

APAS-VERUS hand-writes `Clone`, `PartialEq`, `Eq`, `Debug`, `Default`, `Hash`,
`PartialOrd` and `Ord` impls all over the crate, many of them with an
`assume` in the body (the permitted eq/clone pattern) or placed outside
`verus!`. Some of that was forced by older Verus releases. On
`0.2026.09.13.671956e`, how much of

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
```

can be written inside `verus!` and left to the derive, and what does the
derived impl give the prover? The deliverable is a per-trait answer backed by
an experiment, and a plan for deleting the manual impls that the derive can
replace.

## What already exists

`src/lib.rs` lists these experiments, commented out, from an older release:
`derive_clone_struct_in_verus`, `derive_clone_enum_in_verus`,
`derive_clone_struct_with_vec_in_verus`, `derive_copy_*`, `derive_debug_*`,
`derive_default_*`, `derive_eq_*`, `derive_hash_*`, `derive_ord_*`,
`derive_partial_eq_*`, `derive_partial_ord_*`. Each is marked SUCCEEDS against
a release that is now a year old. Re-run every one of them on 09.13 first and
record the result; a header that says SUCCEEDS against an old release is not
evidence today (ExperAImentalThinking rule 11).

## Rules

`plans/r207-round-plan.md` common rules; ExperAImentalThinking (its 11 rules
govern every experiment file: one question each, `RESULT`, `DATE`, `VERUS`,
`LOG` headers, failures left as corpses, listed commented-out in `lib.rs`,
every run logged). Experiments live in `src/experiments/`, never in `/tmp`.
Verify each with `scripts/validate-standard.sh --experiment <name>`. Do not run
`scripts/validate.sh`, `rtt.sh`, or `ptt.sh`; another agent may hold Verus, and
one Verus process runs at a time. Do not edit any chapter file: this round
measures and recommends, and the deletions are a later round. No subagents.
Read `src/standards/partial_eq_eq_clone_standard.rs` and
`src/standards/constructor_feq_standard.rs` before writing anything.

## Experiments to write

One file per question, named `derive_0913_<trait>_<shape>.rs`. For each
derived trait, the experiment must answer three things: does the file verify;
what can the prover conclude about the derived impl (state it as an `assert`
or a small proof that would fail if the fact were missing); and does the
derive interact with the APAS `View` and `feq` machinery.

| # | Trait | Shapes to try | The fact to test for |
|---|-------|---------------|----------------------|
| 1 | `Clone` | struct of primitives; struct with `Vec<T>`; struct with a generic `T: Clone`; enum | `cloned(x, y)`; `y@ == x@`; whether `vstd::laws_eq`/`clone_view` give it without an `assume` |
| 2 | `Copy` | struct of primitives; generic struct | that `Copy` is accepted at all inside `verus!`, and that a copy is equal to its source |
| 3 | `PartialEq`, `Eq` | the same four shapes | `a == b <==> a@ == b@`; whether `PartialEqSpecImpl`/`obeys_eq_spec` can be proved rather than assumed; what `vstd::std_specs::cmp` and `laws_eq` at 09.13 provide for derived impls (`#2776` array/slice, `#2830` `Structural` for tuples and arrays, `#2773` str) |
| 4 | `Debug` | struct, enum, generic | whether it may sit inside `verus!` now, or must stay in section 14 |
| 5 | `Default` | struct, enum | the view of the default value |
| 6 | `Hash` | struct, enum | whether a derived `Hash` satisfies `obeys_key_model`, and whether that removes the two `external_body` `Hash` bodies in Chap05 `SetStEph`/`SetMtEph` |
| 7 | `PartialOrd`, `Ord` | struct, enum | `a <= b <==> a@ <= b@`; what `laws_cmp::obeys_cmp` and `OrdSpecImpl` give for a derived impl; whether APAS's `TotalOrder` can be discharged from it |
| 8 | all five at once | `#[derive(Clone, Copy, PartialEq, Eq, Debug)]` on one struct, inside `verus!` | the user's exact line: does it compile, verify, and give the facts above |
| 9 | `Structural` | `#[derive(Structural)]` with the above | what vstd's `Structural` (`#2830`, `#2866`) adds over `PartialEq` |

## Inventory to produce

By `grep` over `src/` (excluding `experiments/`), count and tabulate:
manual `impl Clone for`, `impl PartialEq for`, `impl Eq for`, `impl Debug for`,
`impl Default for`, `impl Hash for`, `impl PartialOrd for`, `impl Ord for`,
each split by whether it is inside or outside `verus!`, whether its body holds
an `assume`, and whether the type is a plain data type (a derive candidate) or
carries an invariant the derive would not respect. Name the files.

## Deliverable: `docs/DerivesInsideVerus.md`

1. The re-run of the existing derive experiments: one table, each with its old
   and new result and its log.
2. The new experiments: one row each, question, result, log, and the facts the
   prover could conclude.
3. A per-trait verdict: derive inside `verus!`, derive outside, or hand-write,
   with the reason.
4. The inventory of manual impls, with a column saying which are replaceable by
   a derive, which are not, and why.
5. The count of `assume`s in `PartialEq::eq` and `Clone::clone` bodies that a
   derive would remove, and what remains trusted afterwards.
6. A proposed order for the deletion round: which chapters, how many impls,
   what each deletion must be checked against.
7. Anything that still cannot be derived, with the upstream issue to file.
