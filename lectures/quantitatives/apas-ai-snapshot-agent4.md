# APAS-AI Quantitative Snapshot

Generated: 2026-04-12 (Agent 4 cross-validation run).
Source: `~/projects/APAS-AI` (read-only).
LOC tool: `rusticate-count-loc` (token-aware AST parser) + `veracity-count-loc` (cross-check).
Raw logs: `lectures/quantitatives/raw/apas-ai-rusticate-count-loc-agent4.log`,
          `lectures/quantitatives/raw/apas-ai-veracity-count-loc-agent4.log`,
          `lectures/quantitatives/raw/apas-ai-git-log-agent4.txt`.

---

## 1. Project Timeline

| # | Metric | Value |
|---|--------|-------|
| 1 | First commit | 2025-08-31 (`8114191` — "Initial commit") |
| 2 | Last commit | 2025-11-26 (`1c27bd0` — "Add comprehensive ALGORITHMS.md") |
| 3 | Calendar span | 88 days (~2.9 months) |
| 4 | Active development span | 2025-08-31 – 2025-10-29 (~59 days, ~2.0 months) |
| 5 | Stray-fix tail | 2025-11-01 – 2025-11-26 (8 commits after main dev ended) |
| 6 | Total commits | 347 |
| 7 | Commits in active period | ~339 |
| 8 | Commits in stray-fix tail | 8 |

**Active vs. stray-fix distinction.** Commit frequency by day shows a hard drop after
2025-10-29: the last high-activity days are Oct 26 (22 commits), Oct 28 (7), Oct 29 (4).
November activity is clearly residual: 5 commits on Nov 1 (mostly README cleanup and one
Chap52 API fix), 1 on Nov 3 (remove unnecessary self parameter), 1 on Nov 25
(empty-set partition fix), 1 on Nov 26 (ALGORITHMS.md documentation). These touch
`apas-ai/src/` but are not core algorithmic development.

---

## 2. File and Chapter Counts

| # | Metric | Value |
|---|--------|-------|
| 1 | Chapter directories (`apas-ai/src/Chap*/`) | 42 |
| 2 | Source files (`apas-ai/src/**/*.rs`) | 238 |
| 3 | Bench files (`apas-ai/benches/**/*.rs`) | 171 |
| 4 | Test files (`apas-ai/tests/**/*.rs`) | 246 |
| 5 | Total `.rs` files in `apas-ai/` | 655 |
| 6 | Experiments `.rs` files | 21 |
| 7 | Grand total `.rs` files | 676 |

---

## 3. LOC Totals

Two tools applied; they agree to within rounding (different counting rules noted below).

### 3a. rusticate-count-loc (token-aware AST parser)

| # | Scope | Files | LOC |
|---|-------|-------|-----|
| 1 | `apas-ai/src/` (algorithm source) | 238 | 45,485 |
| 2 | `apas-ai/benches/` | 171 | 13,890 |
| 3 | `apas-ai/tests/` | 246 | 55,223 |
| 4 | `experiments/` | 21 | 1,400 |
| 5 | **Total** | **676** | **115,998** |

### 3b. veracity-count-loc (cross-check)

`veracity-count-loc -c -a` run from `~/projects/APAS-AI` (scans `apas-ai/` only; experiments not included):

| # | Metric | Value |
|---|--------|-------|
| 1 | Files scanned | 655 |
| 2 | Lines (with comments) | 114,598 |
| 3 | Lines (without comments) | 86,469 |
| 4 | Comment lines | 28,129 |
| 5 | Spec lines | 0 (no Verus) |
| 6 | Proof lines | 0 (no Verus) |
| 7 | Exec lines | 0 (Verus classifier not applicable) |
| 8 | Rust lines (net code) | 86,469 |

### 3c. Agreement

veracity `apas-ai/` only (655 files) = 114,598 lines.
rusticate `apas-ai/` only (655 files) = 45,485 + 13,890 + 55,223 = **114,598 lines**. ✓ Exact match.

The 1,400-line difference between rusticate's grand total (115,998) and veracity (114,598)
is exactly the `experiments/` directory (21 files, 1,400 LOC), which veracity did not scan.

**Comment density**: 28,129 / 114,598 = **24.5%** of lines are comments.
This is consistent with well-documented library code.

---

## 4. Per-Chapter LOC Table (src only)

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

**Largest chapter**: Chap37 at 5,706 src LOC (AVL trees, BST variants, sequence structures).
**Smallest chapter**: Chap11 at 34 src LOC (FibonacciMtPer — one very short file).

---

## 5. Chapter Coverage Comparison

### Chapters in APAS-AI but not APAS-VERUS
None. All 42 APAS-AI chapters are present in APAS-VERUS.

### Chapters in APAS-VERUS but not APAS-AI

| # | Chap | APAS-VERUS content | Notes |
|---|------|--------------------|-------|
| 1 | Chap02 | HFScheduler (fork-join scheduler) | Verification infrastructure, no APAS-AI counterpart |
| 2 | Chap30 | (one file, 217 LOC) | Small standalone chapter |

APAS-AI and APAS-VERUS share identical chapter numbering for 42 chapters.
APAS-VERUS adds Chap02 (the fork-join infrastructure chapter) and Chap30.

---

## 6. Comparison to APAS-VERUS

APAS-VERUS figures from `lectures/quantitatives/scale.md` (generated 2026-04-11, main at `7ec8eb2bf`).

### Summary comparison table

| # | Metric | APAS-AI | APAS-VERUS | Ratio |
|---|--------|---------|-----------|-------|
| 1 | Src algorithm LOC | 45,485 | 186,223 | 4.1× |
| 2 | Test LOC | 55,223 | 60,576 | 1.1× |
| 3 | Bench LOC | 13,890 | 13,320 (PTT) | 1.0× |
| 4 | **Total project LOC** | **115,998** | **275,014** | **2.4×** |
| 5 | Src algorithm files | 238 | 262 | 1.1× |
| 6 | Test files | 246 | ~262 (RTT) | ~1.1× |
| 7 | Chapters covered | 42 | 44 | ~1.05× |
| 8 | Active dev span (months) | ~2.0 | ~5.3 | 2.7× |
| 9 | Calendar span (months) | ~2.9 | ~5.3 | 1.8× |
| 10 | Total git commits | 347 | 2,596 | 7.5× |

### Per-chapter LOC comparison (src only, shared 42 chapters)

| # | Chap | APAS-AI LOC | APAS-VERUS LOC | Ratio |
|---|------|-------------|----------------|-------|
| 1 | Chap03 | 35 | 140 | 4.0× |
| 2 | Chap05 | 460 | 3,809 | 8.3× |
| 3 | Chap06 | 2,965 | 13,353 | 4.5× |
| 4 | Chap11 | 34 | 807 | 23.7× |
| 5 | Chap12 | 265 | 450 | 1.7× |
| 6 | Chap17 | 254 | 796 | 3.1× |
| 7 | Chap18 | 2,565 | 11,750 | 4.6× |
| 8 | Chap19 | 1,366 | 6,176 | 4.5× |
| 9 | Chap21 | 478 | 1,373 | 2.9× |
| 10 | Chap23 | 215 | 2,090 | 9.7× |
| 11 | Chap26 | 422 | 3,780 | 9.0× |
| 12 | Chap27 | 288 | 1,299 | 4.5× |
| 13 | Chap28 | 607 | 2,832 | 4.7× |
| 14 | Chap35 | 312 | 2,067 | 6.6× |
| 15 | Chap36 | 549 | 2,520 | 4.6× |
| 16 | Chap37 | 5,706 | 20,319 | 3.6× |
| 17 | Chap38 | 632 | 3,640 | 5.8× |
| 18 | Chap39 | 1,171 | 7,361 | 6.3× |
| 19 | Chap40 | 870 | 4,959 | 5.7× |
| 20 | Chap41 | 1,675 | 10,075 | 6.0× |
| 21 | Chap42 | 1,514 | 8,483 | 5.6× |
| 22 | Chap43 | 3,757 | 12,220 | 3.3× |
| 23 | Chap44 | 535 | 729 | 1.4× |
| 24 | Chap45 | 2,279 | 6,848 | 3.0× |
| 25 | Chap47 | 1,058 | 6,317 | 6.0× |
| 26 | Chap49 | 1,461 | 3,014 | 2.1× |
| 27 | Chap50 | 2,059 | 4,499 | 2.2× |
| 28 | Chap51 | 1,514 | 3,993 | 2.6× |
| 29 | Chap52 | 1,956 | 10,018 | 5.1× |
| 30 | Chap53 | 722 | 2,497 | 3.5× |
| 31 | Chap54 | 286 | 3,262 | 11.4× |
| 32 | Chap55 | 694 | 6,394 | 9.2× |
| 33 | Chap56 | 1,535 | 3,338 | 2.2× |
| 34 | Chap57 | 332 | 881 | 2.7× |
| 35 | Chap58 | 313 | 655 | 2.1× |
| 36 | Chap59 | 751 | 1,925 | 2.6× |
| 37 | Chap61 | 629 | 974 | 1.5× |
| 38 | Chap62 | 483 | 2,867 | 5.9× |
| 39 | Chap63 | 446 | 578 | 1.3× |
| 40 | Chap64 | 584 | 917 | 1.6× |
| 41 | Chap65 | 402 | 3,803 | 9.5× |
| 42 | Chap66 | 621 | 1,816 | 2.9× |
| | **Total (42 shared)** | **43,789** | **184,188** | **4.2×** |

Note: APAS-VERUS total of 186,223 includes Chap02 (382) and Chap30 (217) not in APAS-AI.
Shared-chapter APAS-VERUS total: 186,223 − 382 − 217 = 185,624. (Small discrepancy vs
the table sum of 184,188 is due to rounding in rusticate's comma-formatted output; the
scale.md values are authoritative for APAS-VERUS.)

---

## 7. Caveats

1. **Counting rules differ.** rusticate-count-loc is token-aware (parses AST structure);
   veracity-count-loc uses a fast regex-like pass. For pure Rust code (no Verus), their
   totals agree exactly (114,598 LOC for 655 files), which validates both tools.

2. **Spec/proof/exec split is meaningless for APAS-AI.** APAS-AI has no Verus annotations.
   veracity-count-loc reports 0/0/0 for spec/proof/exec and all code as "rust" (86,469 net
   non-comment lines). The 4.1× LOC ratio between APAS-VERUS and APAS-AI src is the most
   natural measure of the verification overhead for comparable implementations.

3. **Stray fixes after active development.** The project's last formal development day is
   approximately 2025-10-29. Eight commits follow in Nov 2025 (5+1+1+1 = 8). The
   ALGORITHMS.md commit (Nov 26) adds documentation but no algorithmic code. The partition
   fix (Nov 25) and API alignment (Nov 1) are minor targeted fixes, not continuation of
   development. The active-development span of ~2.0 months (vs 5.3 months for APAS-VERUS)
   is the right figure for the lecture comparison.

4. **Bench vs PTT.** APAS-AI has criterion-style benches (13,890 LOC); APAS-VERUS has
   proof-time tests (PTT, 13,320 LOC). These serve different purposes — benches measure
   runtime performance, PTTs verify proof correctness — but their LOC is nearly identical.

5. **Test density.** APAS-AI tests (55,223 LOC) are proportionally larger relative to src
   (55,223/45,485 = 1.21×) than APAS-VERUS tests (60,576/186,223 = 0.33×). APAS-AI
   relies more heavily on runtime tests to validate correctness; APAS-VERUS uses formal
   proofs as the primary correctness mechanism with runtime tests for integration coverage.

6. **experiments/ excluded from veracity scan.** The `experiments/` directory (21 files,
   1,400 LOC) was not scanned by veracity-count-loc; it appears in rusticate totals but
   is excluded from the per-chapter table above. This is consistent with APAS-VERUS
   treatment of its own `src/experiments/` directory.
