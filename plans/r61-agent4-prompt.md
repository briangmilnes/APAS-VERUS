# Agent 4 — Round 61

You are Agent 4 working in `~/projects/APAS-VERUS-agent4`.

## Baseline

- Main: 4496 verified, 0 errors, 18 holes, 2610 RTT, 147 PTT.
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Targets

### Target 1: Chap26 ETSPMtEph.rs `point_distance` — 1 hole

Two agents have assessed this as structural (float arithmetic). Your job:
determine if we can close it with a PARTIAL spec.

1. Read `src/Chap26/ETSPMtEph.rs` — find `point_distance` and its spec
   `spec_point_distance`.
2. Read `src/vstdplus/float.rs` — what axioms exist?
3. The function returns Euclidean distance. Even without full float
   arithmetic axioms, can we prove:
   - `result >= 0.0` (sqrt is non-negative)
   - `point_distance(a, b) == point_distance(b, a)` (symmetry)
   - `point_distance(a, a) == 0.0` (identity)
4. If any of these are provable with existing axioms, remove the
   external_body and prove them. Even a partial spec is progress.
5. If none are provable, write a brief analysis of exactly which float
   axioms are missing and what it would take to add them to
   `vstdplus/float.rs`.

### Target 2: Comprehensive stability audit

Run `scripts/validate.sh` 5 times. Record all results. If any run shows
errors, diagnose the flake source and fix it. The goal is to confirm the
codebase is 100% deterministic after R59-R60 stability work.

### Target 3: Regenerate all analysis files

After any changes:
```bash
scripts/all-holes-by-chap.sh
scripts/all-style-by-chap.sh
scripts/all-fn-impls-by-chap.sh
scripts/chapter-cleanliness-status.sh
```

Commit the regenerated analyses. This ensures the analysis baseline matches
the current code state.

## Validation

Run `scripts/validate.sh` (5x), `scripts/rtt.sh`, `scripts/ptt.sh`.
Write report to `plans/agent4-round61-report.md`. Push to `agent4/ready`.
