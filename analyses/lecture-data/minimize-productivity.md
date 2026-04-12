# Lecture Data — Section 6: Veracity Minimize Productivity

Generated: 2026-04-11. Source: `analyses/veracity-minimize-*.log`.

## Run counts

| # | Metric | Value |
|---|--------|-------|
| 1 | Total minimize logs | 65 |
| 2 | Library minimize logs (`-lib*`) | 49 |
| 3 | Proof minimize logs (`-proofs*`) | 16 |

## Proof-minimize aggregate (per-file logs)

| # | Metric | Value | Notes |
|---|--------|-------|-------|
| 4 | Files where asserts were tested | 11 | with non-zero "tested" |
| 5 | Total asserts removed | **22** | summed across all files |
| 6 | Files where proof blocks were tested | 14 | with non-zero "tested" |
| 7 | Total proof blocks removed | **33** | summed across all files |
| 8 | Total proof-minimize wall time | **6,344 s ≈ 105 min** |

## Effort-vs-yield ratio

- 105 min wall to remove 55 distinct proof statements (22 asserts + 33 blocks).
- Average ~2 min/removal — but the per-file CPU-time deltas (recorded
  per-iteration in each minimize log) showed individual removals saving
  40–100 s of CPU each on subsequent verification runs.
- Cumulative steady-state CPU savings on subsequent validates is the
  real ROI, not the wall-time spent minimizing.

## Highlight: R176 Chap43 OrderedTableMtEph

| # | Metric | Value |
|---|--------|-------|
| 1 | Wall time | 16 min 29 s |
| 2 | Asserts tested | 28 |
| 3 | Asserts removed | 8 |
| 4 | Proof blocks tested | 17 |
| 5 | Proof blocks removed | 0 |
| 6 | CPU saved per removed assert (range) | 43–104 s |
| 7 | Memory saved per removed assert (range) | 6–89 MB |

This single file's removals collectively saved **620+ s of CPU**
across the 8 isolated tests vs. baseline (per the R176 commit message,
the stacked savings translated to **z3 RSS −57%** for that file).

## Top removal beneficiaries (single-file data, R176)

| # | Line | Function | CPU delta | Mem delta |
|---|------|----------|-----------|-----------|
| 1 | 568  | union | −81s | −44 MB |
| 2 | 569  | union | −82s | −6 MB |
| 3 | 550  | intersection | −92s | −48 MB |
| 4 | 523  | map | −104s | −53 MB |
| 5 | 503  | domain | −78s | −89 MB |
| 6 | 487  | delete | −96s | −45 MB |
| 7 | 441  | singleton | −47s | −6 MB |
| 8 | 442  | singleton | −43s | −32 MB |

## Library-minimize logs

49 logs covering vstdplus dependency analysis. Format differs from
proof-minimize (no "complete in" line); aggregate wall time and
removal counts not extracted in this pass — flag for follow-up.
The library minimizer runs across multiple agents in parallel
(agent1 through agent6 each have distinct log timestamps).

## Caveats

- "Removed" means the minimizer found the assert/block was not
  load-bearing for verification. The user reviews each before final
  acceptance — counts here are minimizer-recommended, not necessarily
  all merged.
- Proof minimize wall time totals only count successful per-file runs
  (the "complete in Xm Ys" line). Aborted / partial runs are excluded.
- Three minimize-proofs logs report `complete in 0.0s` — likely
  immediate-exit cases (file already minimized or failed precondition).
