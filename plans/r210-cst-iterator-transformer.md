# r210 — a Verus-CST transformer for the mechanical iterator migration, proved on Chap18

Date: 2026-09-21. One subagent, sequential. Two projects are involved:
`~/projects/CSTs` (where the tool is built, under its own `CLAUDE.md` and
`AGENTS.md`) and `~/projects/APAS-VERUS` (the subject; not edited in this
round; a copy of it is the fixture).

The user's instruction named "chap 8". No `Chap08` exists; row 8 of
`docs/ChapterVerusification.md` is Chap18, the first chapter the iterator
study puts in wave 1 and the one that blocks 22 others. This plan takes
"chap 8" to mean Chap18. If that is wrong, the tool is the same and only the
target changes.

## Goal

A CST transformer in `~/projects/CSTs/processes/iterator-upgrade` (binary) and
`iterator-upgrade-lib` (library), built on `datastructs/vcst-lib`, that applies
the mechanical rewrite classes of `docs/IteratorMigrationStudy.md` §4 to a
chapter's source and proof-time tests. It is run on a fixture copy of
APAS-VERUS at `~/projects/CSTs/processes/iterator-upgrade/tests/fixtures/APAS-VERUS`,
on Chap18 first. Acceptance in the fixture: `scripts/validate.sh isolate Chap18`
reports 0 errors and 0 warnings; the Chap18 run-time tests all run and pass;
the Chap18 proof-time tests all verify. Hand fixes are expected for the
non-mechanical sites and are recorded one by one. The live APAS-VERUS tree is
not touched; the round's product for it is a patch series.

## Inputs

- `docs/IteratorMigrationStudy.md` (all of it): the inventory, the class of
  each iterator, the construct mapping with edit shapes (§4), the consumer
  loop rewrites (§6), and the tool recommendation (§8).
- `src/standards/prophetic_iterators_standard.rs`, `iterators_standard.rs`,
  `wrapping_iterators_standard.rs`, `iterator_ptt_standard.rs`: the target
  shapes for definers and for PTTs.
- `docs/StandardsUpgrade.md` §2–3 and behaviour 2.
- `src/Chap05/SetStEph.rs`, `src/Chap06/DirGraphStEph.rs` in APAS-VERUS: two
  delegated migrations done by hand in r208, the reference output.
- `plans/r204-veracity-iterator-upgrade-apply-report.md`: the rewrite classes
  D1–D10 and T1–T10 of the earlier string-based tool and the nine bugs its
  round-trip found (multi-line clauses, trailing commas, indentation, orphan
  lines, the `it@` trigger). The new tool avoids these by construction: it
  edits CST nodes, not lines.
- `~/projects/CSTs/CLAUDE.md`, `AGENTS.md`, `docs/CommentOutVerus.md`,
  `processes/comment-out-verus/` and `-lib/`: the process layout to copy, and
  the existing whole-item selection over APAS-VERUS.
- `~/projects/CSTs/datastructs/vcst-lib/src/`: the lossless rowan Verus CST.
  `edit.rs` (6 lines, a TODO) and `subst.rs` (8 lines) are the splice layer
  the tool needs; they are built in this round.

## Chap18 measured (from the APAS-VERUS working tree, 2026-09-21)

| # | Chap | File | Lines | Iterator impls | Ghost machinery | `it@` sites | `.pos`/`.elements` |
|---|------|------|------:|---------------:|----------------:|------------:|-------------------:|
| 1 | 18 | ArraySeq.rs | 1744 | 5 | 3 | 7 | 11 |
| 2 | 18 | ArraySeqStEph.rs | 1144 | 4 | 3 | 7 | 10 |
| 3 | 18 | ArraySeqStPer.rs | 1130 | 4 | 3 | 7 | 10 |
| 4 | 18 | ArraySeqMtEph.rs | 2016 | 4 | 3 | 7 | 10 |
| 5 | 18 | ArraySeqMtEphSlice.rs | 1775 | 3 | 3 | 5 | 10 |
| 6 | 18 | ArraySeqMtPer.rs | 1952 | 4 | 3 | 7 | 10 |
| 7 | 18 | LinkedListStEph.rs | 975 | 4 | 3 | 7 | 10 |
| 8 | 18 | LinkedListStPer.rs | 957 | 4 | 3 | 7 | 10 |
| 9 | 18 | ArraySeqSpecsAndLemmas.rs | 113 | 0 | 0 | 0 | 0 |

All eight iterators are delegated (`slice::Iter` underneath; study §3). The
current full run reports 22 type errors in the chapter, no `finite()`
warnings. Eight RTT files in `tests/Chap18/` name no iterator type (0 old-model
lines). Eight PTT files in `rust_verify_test/tests/Chap18/` carry 12–34
old-model lines each, about 177 in all: those need the PTT rewrite class.

## Rules

1. `plans/r207-round-plan.md` common rules; `plans/r208-agent1-chap02-06.md`
   rules 1–9. Exec bodies change only by the delegated-iteration type
   substitution (the wrapper struct's `iter()` returns the std iterator it
   wrapped; same O(1) cost). No `Alg Analysis` line is edited; every one in
   every edited file is read and recorded confirmed or disputed.
2. No string rewriting of source, in the tool or by hand: the tool selects
   CST nodes by kind and name and deletes, replaces, or substitutes nodes;
   replacement text is generated from templates and re-parsed with `vcst-lib`
   before splicing, so the output is a valid CST. Hand fixes use the Edit tool.
3. Never add `assume`, `accept`, `admit`, `external_body`, or `requires true`;
   never weaken an `ensures`.
4. The fixture is the only APAS-VERUS tree edited. Its scripts resolve
   `PROJECT_ROOT` from their own location, so `scripts/validate.sh isolate
   Chap18` run inside the fixture verifies the fixture. One Verus process at a
   time; read every log in full. No other agent is running.
5. The CSTs project's own rules govern the tool's code, tests, logs, and
   commits there; read them first. Nothing is committed in APAS-VERUS.
6. Temporary files go under the fixture's `scratch/` or the tool's own
   directories, never `/tmp`.

## Steps

| # | Step | Detail | Check |
|---|------|--------|-------|
| 0 | Fixture | copy the APAS-VERUS working tree (its uncommitted r207–r209 state, excluding `target/`, `logs/`, `.git/`) to `processes/iterator-upgrade/tests/fixtures/APAS-VERUS`; initialise a git repository in the fixture and commit it as the baseline so every later diff is against it | `git status` clean in the fixture |
| 1 | Splice layer | implement `vcst-lib` `edit.rs` and `subst.rs`: delete a node, replace a node with re-parsed text, and substitute an expression node inside a clause; unit tests with fixtures per CSTs conventions | `cargo test` in `vcst-lib` passes |
| 2 | Rewrite classes | implement, each as a named class with a unit fixture: (a) delete the iterator struct, its `View` impl, `iter_invariant`, the ghost iterator struct and its `View`, and the `ForLoopGhostIteratorNew` and `ForLoopGhostIterator` impls; (b) delete the `Iterator::next` impl of the wrapper (the std iterator has its own); (c) replace the `iter()` trait declaration and impl, and the `IntoIterator` impls, with the delegated template (`std::slice::Iter<'a, T>` return type, the constructor triple in `ensures`, per the standard); (d) substitute `it@.0` → `it.index()`, `it@.1` → `it.seq()`, delete `iter_invariant(&it)` conjuncts, in loop invariants and PTT bodies; (e) delete `use` lines for removed names; (f) the PTT class: regenerate `invariant` and `decreases` blocks per test-name pattern from the six `iterator_ptt_standard.rs` templates, and rewrite post-loop assertions over `it@` | each class's fixture round-trips: apply, re-parse, compare to the expected file |
| 3 | Detect mode | a `--detect` run lists every site by class, file, and line, and every site it cannot classify (the 92-class sites of the study: `requires` on `IntoIterator` impls, consumer loops with `break`, custom iterators, trait `iter()` under behaviour 2) | detect report for Chap18 saved under the tool's `analyses/` |
| 4 | Apply on Chap18 | `--apply` on `src/Chap18/*.rs` and `rust_verify_test/tests/Chap18/*.rs` in the fixture; `git diff --stat` recorded | the fixture re-parses; `git diff` shows only Chap18 |
| 5 | Validate | `scripts/validate.sh isolate Chap18` in the fixture; read the log; hand-fix the non-mechanical sites with the Edit tool, one at a time, recording each (file, line, what, why); repeat | 0 errors, 0 warnings, 0 trigger notes |
| 6 | RTT | align the fixture's `Cargo.toml` `[dependencies]` `vstd` pin with the `[dev-dependencies]` git pin at `671956e` if it still names the crates.io `0.0.0-2025-08-12-1837` (a Cargo edit, recorded); then run the Chap18 run-time tests. `scripts/rtt.sh` runs the whole suite and other chapters still have type errors, so run `cargo nextest run` (or `cargo test`) filtered to the eight `tests/Chap18/Test*.rs` targets; read the output | all Chap18 RTTs run and pass, none skipped |
| 7 | PTT | `scripts/ptt.sh` cannot run (crate errors, no nextest on nightly). Verify the eight rewritten Chap18 PTT files through `scripts/validate-standard.sh`'s harness form (`ptt` mode with the test bodies as modules, as r207 did), one file at a time; read each log | every Chap18 PTT test verifies |
| 8 | Alg Analysis | read every annotation in the nine Chap18 files against its body; table them | none edited |
| 9 | Patch series | `git format-patch` from the fixture baseline, one patch per file, into `~/projects/APAS-VERUS/plans/r210-chap18-patches/`; the orchestrator applies them to the live tree after review | patches apply cleanly to the live tree with `git apply --check` |

## Deliverables

1. The tool and its tests in `~/projects/CSTs/processes/iterator-upgrade{,-lib}`,
   and the splice layer in `vcst-lib`, committed in CSTs per its rules.
2. `~/projects/APAS-VERUS/docs/IteratorTransformerChap18.md`: the detect
   report summary (sites by class), the apply diff stat, every hand fix with
   its reason, the validate, RTT, and PTT results with log names, the Alg
   Analysis table, the patch list, and what the tool cannot yet do (the
   classes left for the next chapters: chained, custom, consumer loops with
   `break`).
3. `~/projects/APAS-VERUS/plans/r210-chap18-patches/`.
