# r220: stabilize Chap45 BalancedTreePQ split

Start: `main` at the commit that adds this plan, worked in worktree
`~/projects/APAS-VERUS-r220`, branch `r220/stabilize-chap45`.
Toolchain: Verus `0.2026.09.13.671956e`, Z3 4.16.0.

## Problem

`scripts/ptt.sh` builds the library with `verus --compile`, and that build
reports 5619 verified, 1 error: rlimit on `src/Chap45/BalancedTreePQ.rs:712`,
`split` (`logs/ptt.20260924-*.log` on main after the r219 merge, and
`logs/ptt.20260923-124708.log`, `-124852.log`, `-125026.log` in r219). The
full `scripts/validate.sh` run passes (5620 verified, 0 errors). The proof is
unstable in the sense r218 B described (`plans/r218-z3-query-diff-report.md`):
the outcome depends on prelude order, earlier queries in the Z3 session, and
the Z3 seed. With the library unverified, the PTT run cannot compile, so PTT
gives no result.

## Method (as in r219, `plans/r219-stabilize-4-proofs-report.md`)

1. Measure `split` alone: `--verify-only-module <m> --verify-function <f>`
   via `VERUS_EXTRA_ARGS` to `scripts/validate.sh isolate Chap45`, with
   `--output-json` for the rlimit figure. Also run under Z3 seeds 1-8.
   Record before-figures.
2. Profile `split` (`--profile` on the single function). Record top
   quantifiers, instantiations, cost.
3. Improve the proof so it does not depend on session state: hide broad
   open spec fns and call small intro/elim lemmas, tighten triggers, state
   explicitly facts Z3 finds only in a favorable order, split long proofs
   into lemmas. Do not change requires/ensures, exec code, or Alg Analysis
   annotations; do not raise rlimit; do not add assume, admit, accept, or
   external_body.
4. Acceptance: `split` verifies alone with `#[verifier::spinoff_prover]` and
   under 8 Z3 seeds. Keep spinoff_prover only if the whole crate needs it.
5. Then, sequentially: `scripts/validate.sh isolate Chap45` (0 errors,
   0 warnings, 0 trigger notes), full `scripts/validate.sh`, `scripts/rtt.sh`,
   `scripts/ptt.sh` (the PTT library build must reach 0 errors; report the
   PTT pass/fail counts).
6. If PTT's `--compile` build exposes another unstable function, apply the
   same method to it and report it.
7. Report `plans/r220-stabilize-chap45-split-report.md` with the tables of the
   r219 report (Chap column), log names, before/after rlimit.
8. Commit on the branch with `git add -A`. Do not push.

## Known RTT failures (not targets)

Chap37 BB[α] `mt_bbalpha_comprehensive_operations`; nondeterministic Chap62
`test_spanning_tree_mt_two_vertices`; nondeterministic Chap63
`test_connected_components_mt_multiple` (fails about 1 in 10 runs).
