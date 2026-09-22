# r211 — make the cargo build and every run-time test pass on `main`, then cut a new fixture

Date: 2026-09-21. One subagent, sequential. Subject: the live APAS-VERUS
working tree on `main` (its uncommitted r207–r210 state). Nothing is
committed in APAS-VERUS. The r210 Chap18 patches are not applied.

## Cause, measured

`logs/rtt.20260921-112745.log` in the r210 fixture: `cargo test` fails to
build the library with 168 errors. vstd at `671956e` declares
`#[cfg(verus_keep_ghost)] pub mod std_specs;` (`vstd/vstd.rs:96`), so under
cargo, which sets no `verus_keep_ghost`, nothing in `std_specs` exists.

| # | Error | Count | Where | Fix |
|---|-------|------:|-------|-----|
| 1 | cannot find type `VerusForLoopWrapper` | 119 | 22 files: Chap05 (3), Chap06 (19); manual loops written in r208 as `let mut it = VerusForLoopWrapper::new(coll.iter())` in exec code | rewrite the manual loop on the iterator's own `next()` (the Verus guide's form, `examples/guide/iterators.rs` `test_next`), or as a `for x in it: expr` loop, which the `verus!` macro desugars in both build modes. Same statements, same order; a loop-form change, recorded per site |
| 2 | cannot find `std_specs` in `vstd` | 45 | ungated `use vstd::std_specs::…;` lines; 164 such lines exist crate-wide | `#[cfg(verus_keep_ghost)]` on the `use` line: the one cfg form CLAUDE.md allows |
| 3 | cannot find trait `IteratorSpecImpl` | 2 | spec-only impl blocks | `#[cfg(verus_keep_ghost)]` on the impl, the same form CLAUDE.md prescribes for `PartialEqSpecImpl` |
| 4 | unresolved import `vstd::iset::…` | 2 | proof-only lemma imports | `#[cfg(verus_keep_ghost)]` on the `use` line |

The three standards that show a manual loop (`prophetic_iterators_standard.rs`,
`iterators_standard.rs`, `iterator_ptt_standard.rs`) use the wrapper too, so
the standard's manual-loop section is wrong for cargo builds and is corrected
in this round.

## Rules

`plans/r207-round-plan.md`; `plans/r208-agent1-chap02-06.md` rules 1–9. In
addition: the only exec change permitted is the loop-form change of row 1,
which keeps every statement of the loop body and their order; no algorithm
changes; every `Alg Analysis` line in every edited file is read and recorded,
none edited; no `assume`, `accept`, `admit`, `external_body`, `requires true`;
no `ensures` weakened. Never `#[cfg(not(verus_keep_ghost))]` on a fn, impl,
or type. Edit tool only; no `sed`, `perl`, or Python on source. One Verus or
cargo process at a time; read every log in full. No other agent is running.

## Steps

| # | Step | Check |
|---|------|-------|
| 1 | Experiment `src/experiments/prophetic_manual_loop_next.rs`: a manual `loop { match it.next() { … } }` over `std::slice::Iter` with invariants over `IteratorSpec::remaining(&it)` and `decreases IteratorSpec::decrease(&it)->0`, plus the `for x in it:` form, both verified; `RESULT`, `DATE`, `VERUS`, `LOG` header; listed commented-out in `lib.rs` | `scripts/validate-standard.sh --experiment prophetic_manual_loop_next`: 0 errors |
| 2 | Standards: rewrite the manual-loop sections of the three standards to the form step 1 proved; re-verify each with `scripts/validate-standard.sh <name> deps ptt`; update `docs/PropheticIterators.md` and the PTT body modules under `scratch/ptt_bodies/` | each standard: 0 errors, 0 warnings |
| 3 | Imports and spec-only impls: gate the 164 `use vstd::std_specs…` lines, the 2 `iset` lemma imports, and the 2 `IteratorSpecImpl` impls with `#[cfg(verus_keep_ghost)]`; table every site | `grep -rnE '^\s*use vstd::std_specs' src` shows every hit preceded by the gate |
| 4 | Manual loops: rewrite the 119 wrapper sites in the 22 Chap05/Chap06 files per step 1; one file at a time; after each file `scripts/validate.sh isolate Chap05` or `Chap06` | Chap05: 760 verified, 0 errors, 0 warnings; Chap06: 0 errors, 0 warnings (it last ran at 1024 verified) |
| 5 | Re-enable the RTT entries commented out in r208 and r209 whose chapters are now migrated (`Cargo.toml` `[[test]]` blocks for `tests/Chap62`, `tests/Chap63`, `tests/Chap66`); `test_partial_order` stays out (its module is out) | `Cargo.toml` diff recorded |
| 6 | `scripts/rtt.sh`; read the log; fix build errors in the test files themselves (they may name deleted types or old iterator forms) with the Edit tool; rerun until every test runs | all run-time tests pass, none skipped; log name recorded |
| 7 | Regression: `scripts/validate.sh isolate` for Chap02, 03, 17, 50, 66; then the full `scripts/validate.sh` | each isolate: 0 errors, 0 warnings; full run: error count not above 162 and the same 51 files |
| 8 | Tool: in `~/projects/CSTs/processes/iterator-upgrade-lib`, change the `add-use-iter` class to emit `#[cfg(verus_keep_ghost)] use vstd::std_specs::iter::*;` and add the gate to any spec-only impl it emits; run its tests; commit in CSTs per its rules | tool tests pass |
| 9 | New fixture: replace the contents of `~/projects/CSTs/processes/iterator-upgrade/tests/fixtures/APAS-VERUS` with the APAS-VERUS working tree at the end of step 7 (excluding `target/`, `logs/`, `.git/`); commit it there as the new baseline | fixture `git status` clean |
| 10 | Re-run the tool on the new fixture's Chap18 (`--detect`, `--apply`), reapply hand fix 1 of r210 if still needed, then in the fixture `scripts/validate.sh isolate Chap18` and `scripts/rtt.sh`; record both. The two open r210 errors (`into_iter` bounds, `scan` rlimit) are expected to remain; do not fix them here | results recorded; a fresh patch series in `plans/r211-chap18-patches/` if the diff differs from r210's |

## Deliverable: `docs/RunTimeTestsRestored.md`

1. The cause, with the vstd line.
2. Tables: every gated import; every manual loop rewritten (Chap, file, fn,
   line, old form, new form, work and span unchanged).
3. The standards changes and the experiment result.
4. RTT result with log name; the re-enabled test entries.
5. Regression table: the isolate runs and the full run.
6. The new fixture's baseline commit and the Chap18 re-run results.
7. Alg Analysis table for every edited file; disputes go to
   `docs/AlgorithmicAnalysisIssues.md` as a new section, none edited.
8. Needs discussion.
