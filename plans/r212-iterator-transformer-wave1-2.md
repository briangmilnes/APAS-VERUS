# r212 — the iterator transformer on waves 1 and 2, in a fresh fixture

Date: 2026-09-21. One subagent, sequential. Tool: `~/projects/CSTs/processes/iterator-upgrade`
(committed at CSTs `e7a371a67`). Fixture:
`~/projects/CSTs/processes/iterator-upgrade/tests/fixtures/APAS-VERUS`, re-cut
from the live APAS-VERUS working tree at the start of this round (tests pass
there since r211). The live tree is not edited; the product is patch series.

## Goal

Migrate, in the fixture, the definer chapters 18, 19, 23 (wave 1), then 37,
38, 39, 40 (wave 2), then the consumer-only chapters 57, 58, 59, to the 09.13
iterator model, using the tool for the mechanical classes and hand edits for
the rest. Per chapter the acceptance is: `scripts/validate.sh isolate ChapNN`
0 errors, 0 warnings, 0 trigger notes; the chapter's run-time tests pass; the
chapter's proof-time tests verify. Keep working through errors until each
chapter meets that or the doc records exactly what is left and why.

## Rules that changed since r210

1. Rule 9 of `plans/r208-agent1-chap02-06.md` is withdrawn. Do not stop at an
   algorithmic-analysis discrepancy; record it in
   `docs/AlgorithmicAnalysisIssues.md` and continue. The rule that replaces
   it: any code that relied on a function being O(Q) or Θ(Q) must still be
   O(Q) or Θ(Q) after your change, for every Q. Small exec edits that keep
   the cost, such as a bounds check, a loop-form change, or a delegated
   iterator, are allowed; rewriting an algorithm is not. Record every exec
   edit with its cost before and after.
2. The `scan` rlimit in `src/Chap18/ArraySeqStEph.rs` (impl `scan`, the
   `while i < len` loop; Z3 4.16 exceeds the budget on a proof identical to
   five sibling files that verify) is accepted as a hole by the user on
   2026-09-21. Mark it with the project's intentional-hole form
   (`crate::vstdplus::accept::accept`, or `admit()` inside a `proof` block
   if `accept` cannot discharge a loop budget), at the smallest scope that
   makes the function verify, with a comment naming Z3 4.16 and this round.
   This is the only new hole authorised. Every other rule on `assume`,
   `accept`, `admit`, `external_body`, `requires true`, and weakened
   `ensures` stands.
3. Do not massively rewrite anything. Edits are per site, with the Edit
   tool, and the tool's own edits are CST-node operations.

## IntoIterator: settle it first

Verus forbids `requires` on an impl of an external trait's method, so the 40
`IntoIterator` impls across the crate that require `spec_*_wf()` are all
rejected at 09.13. Before touching a chapter, run three experiments in
`src/experiments/` of the fixture (with `RESULT` headers, committed in the
fixture), each a slice-backed struct whose well-formedness is
`start + len <= data.len()`:

| # | Form | What changes |
|---|------|--------------|
| A | `#[verifier::type_invariant]` on the struct carrying wf; fields private; `into_iter` needs no `requires` | fields lose `pub`; `open spec fn view` must go through spec accessors |
| B | `into_iter` stays total: `if start + len <= data.len() { slice } else { &[] }`; `ensures wf ==> triple` | one O(1) branch in exec; wf inputs unchanged |
| C | no `IntoIterator` impl; callers write `for x in it: coll.iter()` with `iter()` keeping its `requires` | every `for x in coll` call site becomes `coll.iter()` |

Measure each: does it verify, how many lines it costs at the definer, how
many at the call sites (count them with `grep` across the fixture). Choose
the form with the smallest total edit that verifies, apply it uniformly to
every `IntoIterator` impl the round touches, and state the choice and the
numbers in the doc. Form B is expected to win on edit count; if it does, its
branch is the O(1) exec edit rule 1 allows.

## Steps

| # | Step | Check |
|---|------|-------|
| 0 | Re-cut the fixture from the live tree (excluding `.git/`, `target/`, `logs/`); commit as baseline | `git status` clean; `diff -rq` empty |
| 1 | The three `IntoIterator` experiments; decision recorded | each experiment has a result and log |
| 2 | Chap18: tool `--detect`, `--apply`; hand fixes; the `scan` hole per rule 2; `isolate Chap18`; Chap18 RTTs (`cargo test` on the 8 targets); Chap18 PTT bodies via the harness | 0 errors, 0 warnings; tests pass; PTTs verify |
| 3 | Chap19, then Chap23, the same way (Chap23's two files include `BalBinTreeStEph.rs` with three iterators: PreOrder, PostOrder, InOrder, all flatten-to-`vec::IntoIter`) | same |
| 4 | Extend the tool for what waves 1–2 need and it lacks: chained re-exposure (a struct wrapping another APAS collection's iterator), constructor `ensures` without a struct (Chap43 form, not yet), consumer loops. Each new class gets a fixture test in CSTs and a commit there | tool tests pass |
| 5 | Chap37 (20 files; 4 custom iterators: `AVLTreeSeq.rs`, `AVLTreeSeqStEph.rs`, `AVLTreeSeqStPer.rs`, and the borrow iterator in `AVLTreeSeqMtPer.rs`), Chap38, Chap39, Chap40. For the custom four, write the `IteratorSpecImpl` per `docs/IteratorMigrationStudy.md` §5 and behaviour 2; their `next` bodies are `external_body` today, so a verified `next` removes a hole and a still-`external_body` `next` adds none. Try; if one does not verify after genuine effort, leave it `external_body` as it is and record the error | each chapter 0 errors, 0 warnings; tests pass; PTTs verify |
| 6 | Consumers Chap57, 58, 59 (33 loops over `SetStEph` in the study; loops with `break` draw their conclusion before the `break`) | same |
| 7 | Regression: `isolate` for Chap02, 03, 05, 06, 17, 50, 66; full `scripts/validate.sh`; `cargo test --release --no-fail-fast` on the whole fixture | isolate runs unchanged; full-run error count falls by the migrated chapters' share; all tests pass |
| 8 | Patch series per chapter in `~/projects/APAS-VERUS/plans/r212-patches/ChapNN/`, from the fixture baseline; `git apply --check` on the live tree | all clean |

## Rules carried over

`plans/r207-round-plan.md`; `plans/r208-agent1-chap02-06.md` rules 1–8 (not
9); `plans/r210-cst-iterator-transformer.md` rules 2–6. Every `Alg Analysis`
line in every edited file is read and recorded, none edited; disputes go to
`docs/AlgorithmicAnalysisIssues.md` as a new section and do not stop work. One
Verus or cargo process at a time; read every log in full. No subagents.
Nothing committed in APAS-VERUS; fixture and CSTs commits per CSTs rules.
Temporary files under the fixture's `scratch/`.

## Deliverable: `docs/IteratorTransformerWaves1and2.md`

Per chapter: detect counts by class, apply diff stat, every hand fix (file,
line, what, why, cost before and after for exec edits), the validate, RTT,
and PTT results with log names, and what is left with its error text. Plus:
the `IntoIterator` decision with its numbers; the `scan` hole's exact form
and location; the tool classes added; the regression table; the patch list;
and the tool's remaining limitations.
