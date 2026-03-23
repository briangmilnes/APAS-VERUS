# Agent 3 — Round 60

You are Agent 3 working in `~/projects/APAS-VERUS-agent3`.

## Baseline

- Main: 4496 verified, 0 errors, 18 holes, 2610 RTT, 147 PTT.
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Targets

### Target 1: Chap39 BSTParaTreapMtEph.rs — 2 fn_missing_requires warnings

- `param_treap_assert_finite` (line 206): Read the body, determine the real
  precondition. Likely needs `self.spec_bstparatreapmteph_wf()`.
- `tree_priority_internal` (line 419): Same approach.

### Target 3: Chap57/59 fn_missing_requires — 3 warnings

- `DijkstraStEphU64.rs:104` `pq_entry_new`: Read the body, add real requires.
- `JohnsonStEphI64.rs:73` `adjust_distance`: Read the body, add real requires.
- `JohnsonStEphI64.rs:89` `reweight_edge`: Read the body, add real requires.

These are graph algorithm files with float/integer arithmetic. The requires
likely involve bounds on the distance values or graph structure predicates.

**Note:** The 14 Chap05 SetStEph/SetMtEph fn_missing_wf warnings are
veracity false positives. Those functions already have wf predicates using
`spec_setsteph_wf_generic(s2)` (free function form) instead of
`s2.spec_setsteph_wf()` (method form). The free function is needed because
the parameter has a different type parameter (`U` vs `T`). Do NOT touch
Chap05 Set files.

## Goal

Reduce the real warning count (13 warnings, excluding the 14 Chap05 FPs).
These are all spec-quality fixes, not proof holes — but they matter for
contract completeness.

## Validation

Run `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh` sequentially.
Ensure zero new verification errors. Write report to
`plans/agent3-round60-report.md`. Push to `agent3/ready`.
