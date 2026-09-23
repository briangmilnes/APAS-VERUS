# r219: stabilize the 4 proofs that fail only in the full crate

Start: `main` at the commit that adds this plan (parent `747c6a127`), worked in
worktree `~/projects/APAS-VERUS-r219`, branch `r219/stabilize-proofs`.
Toolchain: Verus `0.2026.09.13.671956e`, Z3 4.16.0.

## Problem

Full `scripts/validate.sh`: 5612 verified, 4 errors
(`logs/validate.20260922-202527.log`). Each chapter verifies alone.

| # | Chap | File | Function | Chapter | Whole | Spinoff, chapter |
|---|------|------|----------|---------|-------|------------------|
| 1 | 26 | ETSPMtEph.rs | lemma_combined_cycle (223) | pass | fail | pass |
| 2 | 26 | ETSPMtEph.rs | etsp_parallel_inner (344) | pass | fail | fail |
| 3 | 35 | OrderStatSelectMtEph.rs | select_inner (553) | pass | fail | fail |
| 4 | 35 | OrderStatSelectMtPer.rs | select_inner (551) | pass | fail | fail |

r218 B (`analyses/r218-z3-query-diff/`, `plans/r218-z3-query-diff-report.md`)
showed the SMT content is identical in chapter and whole runs; the outcome
depends on prelude order, on earlier queries in the same Z3 process, and on
the Z3 seed. For Chap26 ScanDCMtPer the deciding order change involved the
`vstd::seq_lib` `drop_last` axioms versus `spec_add` and the broadcasts
`add_empty_left`, `add_empty_right`, `push_distributes_over_add`.

The proofs are fragile. The fix is to make each proof stand on its own.

## Steps

1. Profile each failing module in the whole-crate configuration, limited to
   that module (the whole-crate `scripts/profile.sh` panics in Verus at
   `profiler.rs:29` on Chap05 RelationStEph): run Verus exactly as
   `scripts/validate.sh` does, plus `--verify-module <module> --profile`
   (and `--rlimit 1` if the log is too large, per the Verus guide
   `profiling.md`). Confirm the failure reproduces. Record the top
   quantifiers (instantiations, cost) per function. If `--verify-module`
   does not reproduce, profile in the chapter configuration with
   `#[verifier::spinoff_prover]` temporarily on the function (it fails
   there too) and say so.
2. For each function, improve the proof so it does not depend on prelude
   order or on earlier queries:
   - fix over-instantiating quantifiers (tighter triggers, explicit lemma
     calls instead of broad `forall`s);
   - state explicitly the facts Z3 currently finds only in a favorable order;
   - split long proofs into lemmas.
   Do not change requires/ensures, do not raise rlimit, do not add assume,
   admit, accept, or external_body. The minimal proof is the goal.
3. Acceptance per function: verifies with `#[verifier::spinoff_prover]` on
   it (fresh Z3 process) under `scripts/validate.sh isolate ChapNN`. Keep the
   attribute only if it is needed for the whole-crate result; report which.
4. After all four: `scripts/validate.sh isolate Chap26`, `isolate Chap35`
   (0 errors, 0 warnings, 0 trigger notes), then full `scripts/validate.sh`
   (target 0 errors), then `scripts/rtt.sh`, then `scripts/ptt.sh` (PTT can
   run only once the library verifies).
5. Report `plans/r219-stabilize-4-proofs-report.md`: per function, the
   profile finding, the change, before/after rlimit use (`--time-expanded`),
   and log names; tables with a Chap column.
6. Commit with `git add -A` on the branch, push the branch.

## Success criteria

Full crate 0 errors; each of the 4 verifies in a fresh Z3 process; specs
unchanged; no holes added; RTT failures limited to the two known ones
(Chap62 `star_contract_mt`, Chap37 BB[α]); PTT result reported.
