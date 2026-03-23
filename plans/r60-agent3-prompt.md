# Agent 3 — Round 60

You are Agent 3 working in `~/projects/APAS-VERUS-agent3`.

## Baseline

- Main: 4496 verified, 0 errors, 18 holes, 2610 RTT, 147 PTT.
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Targets

### Target 1: Chap05 fn_missing_wf warnings — 14 warnings across 2 files

**SetStEph.rs (7 warnings):** Add real wf requires/ensures to:
- `elt_cross_set` (line 216): requires `s2.spec_setsteph_wf()`, ensures `product.spec_setsteph_wf()`
- `cartesian_product` (line 227): requires `s2.spec_setsteph_wf()`, ensures `product.spec_setsteph_wf()`
- `all_nonempty` (line 238): requires `parts.spec_setsteph_wf()`
- `partition_on_elt` (line 247): requires `parts.spec_setsteph_wf()`
- `partition` (line 262): requires `parts.spec_setsteph_wf()`

Read the function bodies and trait signatures first. These functions take
a second Set parameter or return a Set — the wf predicate must flow through.
The `&self` parameter should already have wf in the trait; it's the second
Set parameter or return type that's missing.

**SetMtEph.rs (7 warnings):** Same functions, same pattern. SetMtEph wraps
SetStEph, so the wf predicates parallel: `spec_setmteph_wf()`.

**Important:** These are clean-chapter files (Chap05 has 0 holes). Adding
requires/ensures must not break verification. If a new requires clause
causes callers to fail, you need to prove the caller can satisfy it.

### Target 2: Chap39 BSTParaTreapMtEph.rs — 2 fn_missing_requires warnings

- `param_treap_assert_finite` (line 206): Read the body, determine the real
  precondition. Likely needs `self.spec_bstparatreapmteph_wf()`.
- `tree_priority_internal` (line 419): Same approach.

### Target 3: Chap57/59 fn_missing_requires — 3 warnings

- `DijkstraStEphU64.rs:104` `pq_entry_new`: Read the body, add real requires.
- `JohnsonStEphI64.rs:73` `adjust_distance`: Read the body, add real requires.
- `JohnsonStEphI64.rs:89` `reweight_edge`: Read the body, add real requires.

These are graph algorithm files with float/integer arithmetic. The requires
likely involve bounds on the distance values or graph structure predicates.

## Goal

Reduce the warning count from 27 to 0 (or as close as possible). These are
all spec-quality fixes, not proof holes — but they matter for contract
completeness.

## Validation

Run `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh` sequentially.
Ensure zero new verification errors. Write report to
`plans/agent3-round60-report.md`. Push to `agent3/ready`.
