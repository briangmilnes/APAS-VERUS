# Lecture Data — APAS-AI Quantitative Snapshot

Generated: 2026-04-12 from APAS-AI at `1c27bd0`.
Counterpart APAS-VERUS data from: `lectures/quantitatives/scale.md` (main at `7ec8eb2bf`).

---

## Section 1: Project Timeline

| # | Metric | Value |
|---|--------|-------|
| 1 | First commit date | 2025-08-31 |
| 2 | Last commit date (all) | 2025-11-26 |
| 3 | Calendar span (first → last) | 87 days (~2.9 months) |
| 4 | Active development end | 2025-10-28 (last high-commit-count day on `apas-ai/src/`) |
| 5 | Active development span | ~59 days (~1.9 months) |
| 6 | Total git commits | 347 |
| 7 | Commits in active period (≤ 2025-10-29) | 339 |
| 8 | Stray-fix commits (after APAS-VERUS started, 2025-11-01 – 2025-11-26) | 8 |
| 9 | APAS-VERUS first commit | 2025-11-03 |

**Note on active vs. stray.** APAS-AI active development ran 2025-08-31 through
2025-10-28. After APAS-VERUS launched on 2025-11-03, eight stray commits landed
in APAS-AI (mostly documentation cleanup, one Chap52 API fix on 2025-11-01, one
partition-function correctness fix on 2025-11-25). These post-launch commits are
not counted as part of the APAS-AI project effort. The "active development span"
of ~1.9 months reflects the true engineering window.

---

## Section 2: File and Module Counts

| # | Metric | Value |
|---|--------|-------|
| 10 | Chapter modules (src/Chap*/ directories) | 42 |
| 11 | Total `apas-ai/src/*.rs` files | 238 |
| 12 | Named module files (StEph/MtEph/StPer/MtPer variants) | 200 |
| 13 | `Exercise*.rs` files | 8 |
| 14 | `Problem*.rs` files | 3 |
| 15 | `Algorithm*.rs` files | 4 |
| 16 | `Example*.rs` files (not present — APAS-AI used Exercise/Problem names) | 0 |
| 17 | Other named src files (MathSeq, ChainedHashTable, etc.) | 13 |
| 18 | `lib.rs` + `Types.rs` | 2 |
| 19 | `apas-ai/tests/*.rs` files | 246 |
| 20 | `apas-ai/benches/*.rs` files | 171 |
| 21 | `experiments/src/*.rs` files | 19 |

### St / Mt / Per breakdown (src only)

| # | Variant | File count | Notes |
|---|---------|------------|-------|
| 22 | StEph (sequential, ephemeral) | 88 | Most chapters |
| 23 | MtEph (multi-threaded, ephemeral) | 49 | Parallel coverage |
| 24 | StPer (sequential, persistent) | 31 | Functional structures |
| 25 | MtPer (multi-threaded, persistent) | 20 | Concurrent persistent |
| 26 | Unnamed / misc module files | 13 | Pre-naming-convention files |

### Chapter coverage vs. APAS-VERUS

Both projects cover chapters: 03 05 06 11 12 17 18 19 21 23 26 27 28 35 36 37 38
39 40 41 42 43 44 45 47 49 50 51 52 53 54 55 56 57 58 59 61 62 63 64 65 66 (42 chapters).

| # | Chapter | In APAS-AI | In APAS-VERUS |
|---|---------|-----------|--------------|
| 27 | Chap02 (HFScheduler) | No | Yes |
| 28 | Chap30 | No | Yes |

APAS-VERUS has 44 chapters; APAS-AI has 42, missing Chap02 (parallel scheduler
infrastructure) and Chap30. All 42 APAS-AI chapters are present in APAS-VERUS.

---

## Section 3: Lines of Code

### rusticate-count-loc totals

`rusticate-count-loc` uses AST-aware parsing (token-level, not line-by-line regex).
Counts include blank lines. Ran with `-c` (src + tests + benches) from `~/projects/APAS-AI`.

| # | Scope | Files | LOC |
|---|-------|-------|-----|
| 29 | `apas-ai/src/` | 238 | 45,485 |
| 30 | `apas-ai/tests/` | 246 | 55,223 |
| 31 | `apas-ai/benches/` | 171 | 13,890 |
| 32 | `experiments/` | 19 | 1,326 |
| 33 | **Total (all, with experiments)** | **674** | **115,924** |

Note: rusticate summary shows "676 files 115,998 LOC" including 2 top-level files
(`apas-ai/lib.rs` and a Cargo-level file). The per-directory breakdown above
(674 files, 115,924 LOC) excludes those 2 files. Difference is negligible.

### veracity-count-loc totals

`veracity-count-loc -c -a` from `~/projects/APAS-AI`. APAS-AI has no Verus, so
spec/proof/exec columns are all zero — all code is plain Rust.

| # | Metric | Value |
|---|--------|-------|
| 34 | Files counted | 655 |
| 35 | Lines with comments | 114,598 |
| 36 | Lines without comments (Rust LOC) | 86,469 |
| 37 | Comment lines | 28,129 |
| 38 | Spec LOC | 0 |
| 39 | Proof LOC | 0 |
| 40 | Exec LOC | 0 |

**Discrepancy between tools:** rusticate counts 676 files / 115,998 LOC;
veracity counts 655 files / 114,598 LOC. The 21-file difference is the
`experiments/` subdirectory (19 files) plus 2 top-level files, which veracity
excludes by default. The 1,400-LOC difference aligns with experiments (1,326 LOC)
plus tool differences in blank-line counting.

---

## Section 4: Per-Chapter LOC (src only)

Source in `apas-ai/src/Chap*/`. Counts include Exercise, Problem, and Algorithm files
where present in that chapter.

| # | Chap | Files | LOC |
|---|------|-------|-----|
| 1 | Chap03 | 1 | 35 |
| 2 | Chap05 | 3 | 460 |
| 3 | Chap06 | 16 | 2,965 |
| 4 | Chap11 | 1 | 34 |
| 5 | Chap12 | 3 | 265 |
| 6 | Chap17 | 1 | 254 |
| 7 | Chap18 | 7 | 2,565 |
| 8 | Chap19 | 4 | 1,366 |
| 9 | Chap21 | 12 | 478 |
| 10 | Chap23 | 2 | 215 |
| 11 | Chap26 | 4 | 422 |
| 12 | Chap27 | 4 | 288 |
| 13 | Chap28 | 8 | 607 |
| 14 | Chap35 | 4 | 312 |
| 15 | Chap36 | 3 | 549 |
| 16 | Chap37 | 19 | 5,706 |
| 17 | Chap38 | 2 | 632 |
| 18 | Chap39 | 4 | 1,171 |
| 19 | Chap40 | 3 | 870 |
| 20 | Chap41 | 7 | 1,675 |
| 21 | Chap42 | 4 | 1,514 |
| 22 | Chap43 | 11 | 3,757 |
| 23 | Chap44 | 2 | 535 |
| 24 | Chap45 | 7 | 2,279 |
| 25 | Chap47 | 9 | 1,058 |
| 26 | Chap49 | 8 | 1,461 |
| 27 | Chap50 | 9 | 2,059 |
| 28 | Chap51 | 8 | 1,514 |
| 29 | Chap52 | 14 | 1,956 |
| 30 | Chap53 | 5 | 722 |
| 31 | Chap54 | 4 | 286 |
| 32 | Chap55 | 8 | 694 |
| 33 | Chap56 | 12 | 1,535 |
| 34 | Chap57 | 3 | 332 |
| 35 | Chap58 | 2 | 313 |
| 36 | Chap59 | 4 | 751 |
| 37 | Chap61 | 4 | 629 |
| 38 | Chap62 | 4 | 483 |
| 39 | Chap63 | 2 | 446 |
| 40 | Chap64 | 3 | 584 |
| 41 | Chap65 | 3 | 402 |
| 42 | Chap66 | 2 | 621 |
| | **Total** | **238** | **45,485** |

Chap37 (AVL trees) is the largest at 5,706 LOC — same pattern as APAS-VERUS
where Chap37 is also the largest (20,319 LOC). Chap11 is smallest at 34 LOC.

---

## Section 5: Comparison to APAS-VERUS

APAS-VERUS numbers from `lectures/quantitatives/scale.md` (main at `7ec8eb2bf`, 2026-04-11).

| # | Metric | APAS-AI | APAS-VERUS | Ratio (AI/VERUS) |
|---|--------|---------|------------|-----------------|
| 1 | Total LOC (src only) | 45,485 | 186,223 | 0.24× |
| 2 | Total LOC (all, incl. tests/benches) | 115,998 | 275,014 | 0.42× |
| 3 | Src files | 238 | 262 | 0.91× |
| 4 | Chapter directories | 42 | 44 | 0.95× |
| 5 | Named module variants | 200 | 185 (StEph+MtEph+StPer+MtPer) | 1.08× |
| 6 | Test files | 246 | n/a (RTT + PTT are separate counts) | — |
| 7 | Bench files | 171 | 0 (APAS-VERUS has no benches) | — |
| 8 | Active span | ~59 days (~1.9 mo) | 160 days (~5.3 mo) | 0.37× |
| 9 | Total git commits | 347 | 2,596 | 0.13× |
| 10 | Active commits | 339 | ~2,596 | 0.13× |
| 11 | Comment lines | 28,129 (veracity) | not measured | — |
| 12 | Spec LOC (Verus spec fns) | 0 | substantial | — |
| 13 | Proof LOC (Verus proofs) | 0 | substantial | — |
| 14 | Formal verification | None | 4,439+ functions verified | — |

**Key takeaways for the lecture:**

- APAS-AI src is 24% of APAS-VERUS src by LOC. Despite this, APAS-AI has more
  named module variant files (200 vs. 185) because it does not require the
  per-function proof infrastructure that inflates APAS-VERUS file sizes.
- APAS-AI covers essentially the same chapter set (42 of 44 chapters) in 37%
  of the calendar time and 13% of the commit count. The absence of proof
  obligations is the primary cause of the time difference.
- APAS-AI has 171 benchmark files; APAS-VERUS has none. APAS-AI was optimized
  for performance measurement; APAS-VERUS is optimized for formal correctness.
- APAS-AI has no Verus spec/proof LOC (verified: all 86,469 non-comment lines
  are plain Rust). APAS-VERUS has tens of thousands of spec and proof lines.

---

## Section 6: Caveats

1. **LOC counting differs between tools.** rusticate-count-loc includes blank lines
   and is token-aware. veracity-count-loc uses a different blank-line treatment.
   rusticate reports 115,998 total; veracity reports 114,598 (with comments) / 86,469
   (without). Use veracity's without-comments figure (86,469) for
   "executable Rust LOC" comparisons. Use rusticate for file-count comparisons.

2. **APAS-AI src LOC understates module complexity.** A 35-LOC Chap11 and a 34-LOC
   Chap03 are thin files — one struct and one function. APAS-VERUS chapters are
   larger because every function carries requires/ensures clauses, spec functions,
   lemmas, and proof bodies.

3. **Chap21 (12 files, 478 LOC) is misleading.** All 12 files are textbook
   demos (Algorithm, Exercise, Problem files), not re-usable module implementations.
   There is no Chap21 module in APAS-AI equivalent to APAS-VERUS's Chap21 collection.

4. **Active development end date is approximate.** The last high-activity day
   on `apas-ai/src/` was 2025-10-26 (16 commits). The last non-stray commit was
   2025-10-28. The 8 post-2025-10-29 commits (of which 3 touch src/) are stray
   fixes applied after APAS-VERUS work had begun.

5. **No Chap02 or Chap30 in APAS-AI.** Chap02 in APAS-VERUS is the HFScheduler
   parallel scheduler. Chap30 is a smaller chapter. Neither was implemented in
   APAS-AI.

6. **APAS-AI bench files (171) have no APAS-VERUS counterpart.** These measure
   wall-clock performance of the unverified implementations. APAS-VERUS's
   correctness guarantee comes from verification, not benchmarking.

---

## Raw Artifacts

| # | File | Contents |
|---|------|----------|
| 1 | `raw/apas-ai-rusticate-count-loc.log` | Full rusticate-count-loc output (679 lines) |
| 2 | `raw/apas-ai-veracity-count-loc.log` | Full veracity-count-loc output (666 lines) |
| 3 | `raw/apas-ai-git-log.txt` | Full `git log --format='%ad %h %s' --date=short` (347 lines) |
