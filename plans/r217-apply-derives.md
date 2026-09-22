# r217 — replace hand-written impls with derives, and add `StructuralEq`

Date: 2026-09-22. One subagent, sequential, on the live tree, `main`.
Evidence: `docs/DerivesInsideVerus.md` (r216), its 50 experiments, and
`src/standards/partial_eq_eq_clone_standard.rs`.

## What this round does

Three separate changes, in this order, each verified and committed on its own.

### A. Replace the 11 manual impls a derive can carry (deletion)

| # | Chap | File | Impls | Replacement |
|---|------|------|------:|-------------|
| 1 | 26 | `ETSPStEph.rs` | 4 | `#[derive(Copy, Clone)]` on `Point` and `Edge` |
| 2 | 26 | `ETSPMtEph.rs` | 4 | the same two types |
| 3 | 57 | `DijkstraStEphU64.rs` | 2 | derived `Clone`, and `StructuralEq` for the `PartialEqSpecImpl` |
| 4 | 57 | `DijkstraStEphF64.rs` | 1 | derived `Clone` for `PQEntry` |

Re-derive the list from `docs/DerivesInsideVerus.md` §5 before editing; the
table above is that document's summary, not a fresh measurement. A derive is
used only where the experiment shows the derived impl carries the same facts
the hand-written one stated. If a hand-written impl says more than the derive
can, it stays, and the doc records why.

### B. Add `StructuralEq` where it buys a specified `==` (addition)

Find every type in `src/` that derives `PartialEq` and is non-generic with
all-`Structural` fields, including enums, and add `StructuralEq` to its derive
list. `Chap37/BSTRBMtEph.rs`'s `Color` is the known example; there will be
more. Each addition must be justified by the type actually being compared
somewhere, or by the standard's requirement that a comparable type carry a
spec. Record each one, with what `==` on that type proved before and after.
Nothing else about those types changes.

Also fix the three unchecked `PartialEqSpecImpl` declarations r216 found:
`Chap57/DijkstraStEphU64.rs`, `Chap57/DijkstraStEphF64.rs` (a
`PartialEqSpecImpl` asserting `obeys_eq_spec() == true` beside a derived
`PartialEq` that Verus ignores), and `Chap43/OrderedSetStPer.rs` (the claim
sits inside `verus!` while its `eq` body sits outside, in section 14). Bring
all three to the standard.

### C. Replace the constant-string `Debug` impls with derives (deletion)

r216 counts 246 manual `Debug` impls, 86 of them constant-string bodies that a
derive can replace. The user has authorised changing test expectations to
match what the derived `Debug` prints:

1. Re-derive the candidate list; a candidate is a `Debug` impl whose body
   prints only the type name and its fields, with no chosen formatting.
2. Replace each with `#[derive(Debug)]` in section 12, inside `verus!`
   (`derive_0913_debug_*` shows this verifies).
3. Run the tests. Where a test asserts on formatted output, update the
   expected string to what Rust now prints, and record the old and new strings
   in the doc. Four files in `tests/` are known to assert on output; find any
   others.
4. A `Debug` impl that formats deliberately (padding, a chosen field order, a
   summary rather than the fields) is not a candidate and stays.

## Rules

1. `plans/r207-round-plan.md` common rules; `plans/r208-agent1-chap02-06.md`
   rules 1–8. No `Alg Analysis` line is edited; a discrepancy is recorded in
   `docs/AlgorithmicAnalysisIssues.md` and does not stop the work.
2. No new `assume`, `accept`, `admit`, `external_body`, or `requires true`,
   and no `ensures` weakened. A derive that would lose a stated fact is not
   applied.
3. Cost preservation: a derived `Clone` must have the same cost as the impl it
   replaces. Record any exec change with its cost before and after.
4. Every touched chapter reaches `scripts/validate.sh isolate ChapNN` with
   0 errors, 0 warnings, 0 trigger notes, and its run-time tests pass, before
   that change is committed. Commit A, B and C separately, push each.
5. One Verus or cargo process at a time; read every log in full. No subagents.
   Edit tool only; no `sed`, `perl`, or Python on source. Temporary files under
   `scratch/`.
6. Keep `docs/ChaptersInOrder.md` and `docs/ChapterVerusification.md` current
   for every chapter whose numbers change.

## Deliverable: `docs/DerivesApplied.md`

1. Part A: every impl deleted, with the derive that replaced it and the facts
   checked before and after.
2. Part B: every `StructuralEq` added, with what `==` proved before and after;
   the three unchecked declarations and how each was fixed.
3. Part C: every `Debug` impl replaced; every test expectation changed, with
   the old and new strings; every impl left alone and why.
4. Per-chapter validate and test results with log names.
5. The hole count before and after, from
   `~/projects/veracity/target/release/veracity-review-proof-holes` via
   `scripts/holes.sh` on the touched chapters (exclude `scratch/` and `bugs/`;
   see `plans/r215-veracity-feedback.md` items 7 to 9).
6. Anything the derives could not replace after all, with the reason.
