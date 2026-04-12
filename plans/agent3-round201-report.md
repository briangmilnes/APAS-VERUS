# Agent 3 — Round 201 Report

Task: APAS-AI quantitative snapshot for lecture comparison.

---

## Step 1: Rusticate Build

**Outcome: Clean.**

```
Finished `release` profile [optimized] target(s) in 35.02s
```

Binary confirmed at `~/projects/rusticate/target/release/rusticate-count-loc`.

---

## Step 2: rusticate-count-loc Headline Totals

Run: `rusticate-count-loc -c` from `~/projects/APAS-AI`.

| # | Scope | Files | LOC |
|---|-------|-------|-----|
| 1 | `apas-ai/src/` | 238 | 45,485 |
| 2 | `apas-ai/tests/` | 246 | 55,223 |
| 3 | `apas-ai/benches/` | 171 | 13,890 |
| 4 | `experiments/` | 19 | 1,326 |
| 5 | **Total (summary line)** | **676** | **115,998** |

Note: spec/proof are not applicable — APAS-AI has no Verus.

---

## Step 3: veracity-count-loc Headline Totals

Run: `veracity-count-loc -c -a` from `~/projects/APAS-AI`.

| # | Metric | Value |
|---|--------|-------|
| 1 | Files | 655 |
| 2 | Lines (with comments) | 114,598 |
| 3 | Lines (without comments, Rust LOC) | 86,469 |
| 4 | Comment lines | 28,129 |
| 5 | Spec / Proof / Exec | 0 / 0 / 0 (no Verus) |

**Discrepancy vs. rusticate:** veracity sees 655 files / 114,598 LOC;
rusticate sees 676 / 115,998. The 21-file gap is `experiments/` (19 files)
plus 2 top-level files that veracity excludes. The 1,400-LOC gap matches
the experiments/ LOC (1,326) plus tool blank-line treatment differences.
Both tools agree to within 1.3%.

---

## Step 4: APAS-AI Git Metadata

| # | Metric | Value |
|---|--------|-------|
| 1 | First commit | 2025-08-31 (`8114191 Initial commit`) |
| 2 | Last commit | 2025-11-26 (`1c27bd0 Add comprehensive ALGORITHMS.md`) |
| 3 | Total commits | 347 |
| 4 | Calendar span | 87 days (~2.9 months) |
| 5 | Active dev end | 2025-10-28 (last high-activity day on src/) |
| 6 | Active dev span | ~59 days (~1.9 months) |
| 7 | Active commits (≤ 2025-10-29) | 339 |
| 8 | Stray commits (after APAS-VERUS started) | 8 |
| 9 | Peak commit day | 2025-10-14 (55 commits) |

The 8 stray commits (2025-11-01 through 2025-11-26) are docs cleanup,
one Chap52 API fix, and one partition-function correctness fix — none
represent material APAS-AI development. They landed while APAS-VERUS
was already underway.

---

## Step 5: File and Chapter Totals

| # | Metric | Value |
|---|--------|-------|
| 1 | Chapter directories | 42 |
| 2 | Chapters absent vs. APAS-VERUS | Chap02 (HFScheduler), Chap30 |
| 3 | Named module files (St/Mt variants) | 200 |
| 4 | StEph / MtEph / StPer / MtPer | 88 / 49 / 31 / 20 |
| 5 | Exercise/Problem/Algorithm files | 15 |
| 6 | Test files | 246 |
| 7 | Bench files | 171 |

---

## Step 6: Key Comparison Row (APAS-AI vs. APAS-VERUS)

| # | Metric | APAS-AI | APAS-VERUS | Ratio |
|---|--------|---------|------------|-------|
| 1 | Src LOC | 45,485 | 186,223 | 0.24× |
| 2 | Total LOC (all) | 115,998 | 275,014 | 0.42× |
| 3 | Src files | 238 | 262 | 0.91× |
| 4 | Chapters | 42 | 44 | 0.95× |
| 5 | Named module variants | 200 | 185 | 1.08× |
| 6 | Active span | ~1.9 mo | ~5.3 mo | 0.37× |
| 7 | Commits | 347 | 2,596 | 0.13× |
| 8 | Formally verified fns | 0 | 4,439+ | — |
| 9 | Bench files | 171 | 0 | — |

---

## Caveats and Blockers

- No blockers encountered.
- rusticate built clean in 35s.
- Both LOC tools ran without error.
- The "active span" boundary is a heuristic (last high-commit-count day on src/).
  The last non-stray src commit is 2025-11-25, but that is a single isolated fix.
  The true active-development end is clearly 2025-10-28.

---

## Files Written

| # | File | Size |
|---|------|------|
| 1 | `lectures/quantitatives/apas-ai-snapshot.md` | Main report |
| 2 | `lectures/quantitatives/raw/apas-ai-rusticate-count-loc.log` | 679 lines |
| 3 | `lectures/quantitatives/raw/apas-ai-veracity-count-loc.log` | 666 lines |
| 4 | `lectures/quantitatives/raw/apas-ai-git-log.txt` | 347 lines |
| 5 | `plans/agent3-round201-report.md` | This file |
