# Agent 4 — Round 61

You are Agent 4 working in `~/projects/APAS-VERUS-agent4`.

## Baseline

- Main: 4496 verified, 0 errors, 12 holes, 2610 RTT, 147 PTT.
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Targets

### Target 1: Chap26 ETSPMtEph.rs `point_distance` — 1 hole

Two agents assessed this as structural (float arithmetic). Investigate
whether a partial spec is provable:

1. Read `src/Chap26/ETSPMtEph.rs` — find `point_distance` and
   `spec_point_distance`.
2. Read `src/vstdplus/float.rs` — existing float axioms.
3. Can we prove any of: `result >= 0.0`, symmetry, identity of
   indiscernibles? Even one would be progress.
4. If none are provable with existing axioms, write a brief analysis of
   what float axioms are missing.

### Target 2: 5x stability validation

Run `scripts/validate.sh` five times. All must show 0 errors. If any flake,
diagnose and fix.

### Target 3: Regenerate all analysis files

```bash
scripts/all-holes-by-chap.sh
scripts/all-style-by-chap.sh
scripts/all-fn-impls-by-chap.sh
scripts/chapter-cleanliness-status.sh
```

Commit regenerated analyses.

### Target 4: Daily proof table

Generate the daily proof table showing R59 and R60 results:

| Round | Holes Start | Holes End | Delta | Clean Chaps | Dirty Chaps | Verified |
|-------|-------------|-----------|-------|-------------|-------------|----------|
| R59   | 24          | 18        | -6    | 41          | 5           | 4496     |
| R60   | 18          | 12        | -6    | 41          | 5           | 4496     |

(Verify these numbers from the actual data.)

## Validation

Run `scripts/validate.sh` (5x), `scripts/rtt.sh`, `scripts/ptt.sh`.
Write report to `plans/agent4-round61-report.md`. Push to `agent4/ready`.
