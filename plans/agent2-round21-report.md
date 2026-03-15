# Agent 2 — Round 21 Report

## Summary

Full review-against-prose for 8 chapters (21, 23, 26, 27, 28, 30, 35, 36).
All 8 phases executed per `.cursor/rules/apas-verus/review-against-prose.mdc`.
Cost annotations added to all exec functions. Review files written for every chapter.
Verification: 3957 verified, 0 errors.

## Deliverables

| # | Chap | Review File | Cost Annotations | Status |
|---|------|-------------|-----------------|--------|
| 1 | 21 | `src/Chap21/analyses/review-against-prose.md` | Skipped (Example files) | Complete |
| 2 | 23 | `src/Chap23/analyses/review-against-prose.md` | 15 annotations (2 files) | Complete |
| 3 | 26 | `src/Chap26/analyses/review-against-prose.md` | 27 annotations (8 files) | Complete |
| 4 | 27 | `src/Chap27/analyses/review-against-prose.md` | 11 annotations (4 files) | Complete |
| 5 | 28 | `src/Chap28/analyses/review-against-prose.md` | 24 annotations (10 impl files) | Complete |
| 6 | 30 | `src/Chap30/analyses/review-against-prose.md` | 17 annotations (1 file) | Complete |
| 7 | 35 | `src/Chap35/analyses/review-against-prose.md` | 10 annotations (4 files) | Complete |
| 8 | 36 | `src/Chap36/analyses/review-against-prose.md` | 9 annotations (3 files) | Complete |

## Proof Holes

All 8 chapters have **0 proof holes**. No holes were introduced or removed in this round.

| # | Chap | Files | Holes | Notes |
|---|------|-------|-------|-------|
| 1 | 21 | 12 | 0 | Example files, skipped |
| 2 | 23 | 2 | 0 | 9 accept() in Clone/PartialEq (approved pattern) |
| 3 | 26 | 8 | 0 | 17 clean proof functions |
| 4 | 27 | 4 | 0 | 24 clean proof functions |
| 5 | 28 | 11 | 0 | 15 clean proof functions |
| 6 | 30 | 1 | 0 | 11 external_body_accept_hole (accepted) |
| 7 | 35 | 4 | 0 | 6 clean proof functions |
| 8 | 36 | 3 | 0 | 7 clean proof functions |

## Key Findings

### Cost Disagreements (Work)

All work costs agree with APAS across all chapters.

### Cost Disagreements (Span) — Mt Modules

Span disagreements are the dominant finding. All Mt modules use `join()` for genuine
parallelism but achieve worse span than APAS specifies, due to:

| # | Chap | Module | APAS Span | Actual Span | Root Cause |
|---|------|--------|-----------|-------------|------------|
| 1 | 26 | MergeSortMtPer | Θ(lg² n) | Θ(n) | Vec concat is O(n) not O(1) |
| 2 | 26 | ScanDCMtPer | Θ(lg n) | Θ(n) | Vec concat O(n) |
| 3 | 26 | ETSPMtEph | Θ(n lg n) | Θ(n²) | Sequential sort in split |
| 4 | 27 | ReduceContractMtEph | Θ(log n) | Θ(n) | Single-level join, sequential halves |
| 5 | 27 | ScanContractMtEph | Θ(log n) | Θ(n) | Sequential expand_scan |
| 6 | 28 | All Mt variants | Θ(lg n) | Θ(n) | Sequential loops, no thread spawning |
| 7 | 35 | OrderStatSelectMt* | Θ(lg² n) | Θ(n) | Sequential filter in partition |
| 8 | 36 | QuickSortMt* | Θ(lg² n) | Θ(n) | Sequential partition |

**Common pattern**: Mt modules correctly parallelize recursive calls but use sequential
operations for partition/filter/merge steps. APAS assumes O(1) or O(lg n) span for these
sub-operations (via tree-backed sequences or parallel primitives).

### Implementation Fidelity

- Chap23: Vec-backed tree sequences vs APAS's balanced binary tree — all cost
  differences are structural (O(n) operations that would be O(log n) with trees).
- Chap28: All algorithms faithful to APAS; Mt variants structurally correct but
  sequential (no thread spawning).
- Chap35/36: Partition uses sequential filter loops instead of parallel filter.

### Spec Fidelity

All specs are strong across all chapters. No weakened ensures found.

### Test Coverage

| # | Chap | RTT Files | PTT Files | Gaps |
|---|------|-----------|-----------|------|
| 1 | 21 | 1 | 0 | N/A (examples) |
| 2 | 23 | 2 | 2 | Minor: post_order not directly tested in RTT |
| 3 | 26 | 8 | 0 | No PTTs needed (no iterators) |
| 4 | 27 | 4 | 0 | No PTTs needed (no iterators) |
| 5 | 28 | 10 | 0 | No PTTs needed (no iterators) |
| 6 | 30 | 0 | 0 | No RTT file exists |
| 7 | 35 | 4 | 0 | No PTTs needed (no iterators) |
| 8 | 36 | 4 | 0 | Broken test: TestQuickSortMtEphSlice.rs (wrong module refs) |

### Actionable Items Found

1. **Chap30**: No RTT file — low priority (thin f64 wrapper).
2. **Chap36**: `TestQuickSortMtEphSlice.rs` references non-existent module names
   (behind `all_chapters` gate).
3. **All Mt modules**: Sequential partition/filter/merge limits span to O(n).
   Remediation: parallel filter/compact primitives.

## Techniques Used

- 7 parallel review agents for independent chapter processing
- Cost annotation via doc comments (APAS + Claude-Opus-4.6 paired lines)
- Full 8-phase review procedure per review-against-prose.mdc
- Spot-check validation after annotation edits

## Verification

```
verification results:: 3957 verified, 0 errors
```

## Commit

Commit hash: 3b9d0eed
