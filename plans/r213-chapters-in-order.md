# r213 — every chapter, in order, to 0 errors, 0 warnings, tests passing

Date: 2026-09-22. One subagent, sequential, on the live tree, `main`, starting
at `9f9485a2f`. The user is away and has authorised per-chapter commits and
pushes on `main` for this round.

## Goal

For each chapter in the order below: `scripts/validate.sh isolate ChapNN`
reports 0 errors, 0 warnings, 0 trigger notes; the chapter's run-time tests
pass; then commit and push, update `docs/ChapterVerusification.md`, and go to
the next chapter. Keep working through errors; do not stop at the first hard
one.

Order (numeric, which respects `Cargo.toml` dependencies):
02, 03, 05, 06, 11, 12, 17, 18, 19, 21, 23, 26, 27, 28, 30, 35, 36, 37, 38,
39, 40, 41, 42, 43, 44, 45, 47, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
61, 62, 63, 64, 65, 66.

Chapters already at 0 errors (02, 03, 05, 06, 17, 18, 19, 23, 37, 39, 40, 50,
59, 66 per the fixture and `docs/ChapterVerusification.md`) are re-run on the
live tree and their warnings removed; the rest are migrated.

## Per chapter

1. `scripts/validate.sh isolate ChapNN`; read the log in full.
2. Fix errors in this chapter's files. Tools available:
   - The iterator transformer, `~/projects/CSTs/processes/iterator-upgrade`
     (`--detect`, `--apply`, `--into-iter c`), for chapters still on the old
     iterator model (41, 42, 43, and consumers in 44, 45, 52, 53, 55, 61–65).
     Run it on the live tree's chapter files; it edits CST nodes.
   - The recipes in `docs/HashMigration.md`, `docs/Chap02to06Validation.md`
     (Set::new → filter, Map::new(dom, fv), finite by type),
     `docs/IteratorTransformerWaves1and2.md` (IntoIterator form B or C, custom
     `IteratorSpecImpl`, consumer loops), `docs/RunTimeTestsRestored.md`
     (manual loops on the iterator's own `next()`, `cfg` gates on imports).
3. Remove every deprecated `.finite()` in the chapter's files: delete the
   conjunct from `requires`, `ensures`, invariants, wf predicates, and
   asserts; a `requires`/`ensures` that becomes empty is deleted; a proof step
   whose only purpose was finiteness is deleted. This is a spec cleanup, not a
   weakening: `finite()` is `true` at 09.13.
4. Resource limits. The machine has 31 GB and runs nothing else. When a
   function exceeds its rlimit, profile it first
   (`scripts/validate.sh isolate ChapNN --profile`, then the summary under
   `logs/profile/`). If the profile shows no matching loop, add
   `#[verifier::rlimit(N)]` on that function with N up to 100, and record the
   function, N, time, and peak Z3 memory. If it shows a matching loop, fix the
   trigger. Never raise the budget for the whole crate.
5. Run-time tests for the chapter: `cargo test --release --no-fail-fast`
   filtered to the chapter's `[[test]]` targets (`cargo-nextest` is not
   installed, so `scripts/rtt.sh` does not run). Fix failures in test code
   where the test uses a removed API; never change a test's expected value to
   match a changed result without recording why.
6. PTTs for the chapter: verify their bodies through
   `scripts/validate-standard.sh`'s `ptt` form as r207 and r212 did, where
   PTT files exist.
7. Commit: `git add -A`, message `r213 ChapNN: <N> verified, 0 errors,
   0 warnings; RTT <k> pass`, with the Co-Authored-By line; push.
8. Update the chapter's row in `docs/ChapterVerusification.md` (in the same
   commit or the next).

## Rules

1. `plans/r207-round-plan.md` common rules and `plans/r208-agent1-chap02-06.md`
   rules 1–8 bind. Rule 9 is withdrawn: an algorithmic-analysis discrepancy
   never stops work; record it in `docs/AlgorithmicAnalysisIssues.md` (new
   section per chapter) and continue.
2. Cost preservation: code that relied on a function being O(Q) or Θ(Q) must
   still be O(Q) or Θ(Q) after the change, for every Q. Small exec edits that
   keep the cost are allowed; rewriting an algorithm is not. Record every exec
   edit with cost before and after.
3. No new `assume`, `accept`, `admit`, `external_body`, `requires true`, or
   `// veracity: no_requires`; no `ensures` weakened; no `Alg Analysis` line
   edited. If a function cannot be proved after genuine effort (read the
   error, profile, search vstd, write intermediate asserts and lemmas), leave
   it failing in place (the "leave the corpse" rule), record the exact error
   text and what was tried, do not commit that chapter as clean, and move to
   the next chapter whose dependencies are clean. Report every such function.
4. Never sequentialise an Mt file. Never `#[cfg(not(verus_keep_ghost))]` on
   a fn, impl, or type.
5. No `sed`, `perl`, or Python on source; the Edit tool, or the CST tool.
6. One Verus or cargo process at a time; read every log in full; never rerun
   a validate whose log you have not read.
7. No subagents. Temporary files under `scratch/`.
8. After every fourth chapter, run the regression set (`isolate` on every
   chapter already committed clean) and fix any regression before going on.

## Deliverable

Per-chapter commits on `main`, pushed. `docs/ChapterVerusification.md` kept
current. A running log `docs/ChaptersInOrder.md`: one section per chapter with
the starting error and warning counts, every edit class used, every exec edit
with cost, every rlimit raised, the final counts and log names, RTT and PTT
results, and any function left failing with its error text.
