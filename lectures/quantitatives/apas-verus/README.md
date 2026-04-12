# Lecture Quantitative Data

Generated 2026-04-11 from main at `7ec8eb2bf`. See
`plans/lecture-quantitatives-plan.md` for the source plan.

## Files

| # | File | Section | Status |
|---|------|---------|--------|
| 1 | `scale.md` | 1. Scale (LOC, files, modules, agent-rounds) | done |
| 2 | `coverage.md` | 2. Coverage (verified counts, APAS textbook %) | **deferred** — needs manual APAS classification + spec-strength sampling |
| 3 | `trust-base.md` | 3. Trust base (external_body, assume, accept, opaque) | done |
| 4 | `cost.md` | 4. Cost (validate/RTT/PTT timing, RSS, rlimits) | done |
| 5 | `holes-over-time.md` | 5. Holes over time (R20→R196) | done |
| 6 | `minimize-productivity.md` | 6. Veracity minimize productivity | done |
| 7 | `eq-clone.md` | 7. Eq/Clone/View counts | done |
| 8 | `optimization-history.md` | 8. Optimization patterns + landings | done |

## Key headlines (TL;DR)

- **5,674 verified items, 0 errors** as of R196 (2026-04-11).
- **3,776 RTT pass, 221 PTT pass** in 21s + 259s respectively.
- **45 of 46 chapters clean**, 4 residual holes (mostly veracity-counted opaque markers).
- Holes dropped from **238 (R20) → 4 (R196)** — a 98.3% reduction in 27 rounds.
- **186,223 LOC** of algorithm code across 262 files in 44 chapters.
- **281 agent-round reports**, 196 rounds, 8 agents touched.
- Minimize tooling removed **22 asserts + 33 proof blocks** in 105 min wall;
  R176 Chap43 alone netted z3 RSS **−57%**.
- Validate runs in **80–230s**, peak rust_verify ~10–14 GB, peak Z3 ~0.5–8 GB.
- 15 production `rlimit` overrides; 8 of them in Chap65 (UnionFind).

## Deferred: Section 2 (Coverage)

Requires manual classification:
- APAS textbook coverage per chapter (done / partial / skipped)
- Spec strength taxonomy (strong / partial / weak / none) — recommend
  sampling 5 representative chapters rather than all 44.

Both need user-driven judgment; not appropriate for automated extraction.

## How to refresh

These files snapshot a point in time. To regenerate after future rounds:
1. Run all the bash one-liners documented in
   `plans/lecture-quantitatives-plan.md` against the current state.
2. Update the headline numbers in each file.
3. For `holes-over-time.md`, append a new row per new R-tagged commit.

A future `scripts/lecture-data-refresh.sh` would automate this — not
written this round.
