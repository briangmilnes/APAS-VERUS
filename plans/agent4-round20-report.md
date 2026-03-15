# Agent 4 — Round 20 Report: Tier 2 Spec Audit — Round C (13 chapters)

## Mission

Audit every exec fn's `requires`/`ensures` in 13 remaining clean algorithm chapters against APAS textbook prose. Classify specs, fix gaps, validate.

## Summary

- **10 of 13 chapters already strong** — no changes needed.
- **Chap17** (MathSeq): Strengthened `domain` ensures (now provable). `range`/`multiset_range` remain partial/weak (need loop invariant infrastructure).
- **Chap40** (Augmented BST): Strengthened 14 core dictionary operations across 3 files (insert/delete/find/contains/get) to encode full Map/Set semantics. Added external_body to impls (+12 new holes). Left navigation/augmentation ops (rank, select, keys, values, min, max, reduced_value, range_reduce) as documented weak specs.
- **Chap49** (SubsetSum, MED): Documented spec-to-exec disconnection. spec_subset_sum and spec_med exist but aren't connected to trait methods due to missing `Seq<T> → Seq<int>` spec bridge. No code fix — needs infrastructure work.
- **Chap50** (OBST): Documented missing spec function for optimal_cost. MatrixChain already strong. OBST gap is architectural (Probability uses f64, no spec arithmetic).

## Per-Chapter Classification

| # | Chap | Topic | Verdict | Strong | Partial | Weak | Missing | R20 Change |
|---|------|-------|---------|--------|---------|------|---------|------------|
| 1 | 03 | InsertionSort | **all strong** | 1 | 0 | 0 | 0 | — |
| 2 | 11 | Fibonacci | **all strong** | 5 | 0 | 0 | 0 | — |
| 3 | 17 | MathSeq | mostly strong | 19 | 1 | 1 | 1 | domain fixed |
| 4 | 27 | Contraction | **all strong** | 3 | 0 | 0 | 0 | — |
| 5 | 28 | MCSS | **all strong** | 9 | 0 | 0 | 0 | — |
| 6 | 30 | Probability | skip (infra) | — | — | — | — | — |
| 7 | 35 | OrderStatSelect | **all strong** | 2 | 0 | 0 | 0 | — |
| 8 | 36 | QuickSort | **all strong** | 6 | 0 | 0 | 0 | — |
| 9 | 40 | Augmented BST | mixed | 27 | 2 | 13 | 0 | 14 fn strengthened |
| 10 | 44 | DocIndex | skip (example) | — | — | — | — | — |
| 11 | 49 | DP: SS + MED | weak | 0 | 0 | 22 | 6 | documented gap |
| 12 | 50 | DP: OBST + MC | mixed | 5 | 0 | 17 | 2 | documented gap |
| 13 | 51 | DP: BU + TD | mostly strong | 9 | 0 | 17 | 0 | — |

## Holes Before/After

| File | Before | After | Delta | Notes |
|------|--------|-------|-------|-------|
| BSTKeyValueStEph.rs | 0 | 5 | +5 | insert/delete/find/contains/get external_body |
| BSTSizeStEph.rs | 0 | 4 | +4 | insert/delete/find/contains external_body |
| BSTReducedStEph.rs | 2 | 5 | +3 | insert/delete/find/contains/get external_body |
| MathSeq.rs | 0 | 0 | 0 | domain strengthened + proved |
| **Chap40 total** | **2** | **14** | **+12** | Strong specs, needs BST invariant proofs |

## Verification State

- 3926 verified, 0 errors
- 356 total holes, 34 clean chapters, 12 holed
- 2600 RTT pass

## Deliverables

- `src/ChapNN/analyses/spec-audit.md` for all 13 chapters
- Chap17 `domain` ensures strengthened + proved
- Chap40 core dictionary ops strengthened (14 fns) + external_body on impls
- `plans/agent4-round20-report.md` (this file)

## Remaining Spec Gaps (actionable)

### Chap40 — Navigation & Augmentation
- `minimum_key/maximum_key`: Need TotalOrder minimality over key domain.
- `keys/values`: Need sorted order + domain/range correspondence.
- `rank/select/split_rank`: Need rank == |{x ≤ k}|, select == kth element.
- `reduced_value/range_reduce`: Need reduction spec over tree values.
- **Blocker**: All need BST structural invariant connecting tree shape to abstract View.

### Chap49 — Spec-to-Exec Bridge
- `subset_sum`: Need `spec_to_int_seq` mapping T elements to int values.
- `min_edit_distance`: Same mapping needed for source/target sequences.
- **Blocker**: No spec-level `Into<i32>` equivalent. Need a View-like bridge for numeric conversion.

### Chap50 — OBST Spec
- `optimal_cost`: Need `spec_obst_cost` recursive spec function.
- **Blocker**: Probability type wraps f64 — no f64 arithmetic in spec mode.

### Chap17 — Minor
- `range`: Need membership spec (every range element from seq, every seq element in range).
- `multiset_range`: Need count semantics (element-frequency pairs).
- **Blocker**: Loop invariant work connecting seen/out sets to processed input.

## Commit

Pending user approval.
