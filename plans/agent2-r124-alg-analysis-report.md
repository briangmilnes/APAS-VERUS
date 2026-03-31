# Agent 2 — R124 Alg Analysis Code Review Report

## Task

Replace all `Claude-Opus-4.6 (1M): NONE` placeholders with independent code review
analysis for Chap19, Chap21, and Chap23.

## Summary

| # | Chap | File | NONEs | Matches | Differs |
|---|------|------|-------|---------|---------|
| 1 | 19 | ArraySeqStEph.rs | 17 | 5 | 12 |
| 2 | 19 | ArraySeqStPer.rs | 16 | 5 | 11 |
| 3 | 19 | ArraySeqMtEph.rs | 19 | 5 | 14 |
| 4 | 19 | ArraySeqMtEphSlice.rs | 3 | 3 | 0 |
| 5 | 21 | Algorithm21_5.rs | 1 | 0 | 1 |
| 6 | 21 | Algorithm21_6.rs | 1 | 0 | 1 |
| 7 | 23 | PrimTreeSeqStPer.rs | 6 | 1 | 5 |
| | | **Total** | **63** | **19** | **44** |

## Analysis

### Chap19 — Array Sequences

All four files implement the APAS array sequence ADT (Ch18/19) with APAS cost spec
CS 20.2 as reference. The key finding: **all implementations use sequential loops**,
even MtEph. No `join()` calls anywhere.

**Matches APAS (19 functions):** Operations with O(1) cost that are inherently
sequential — `length`, `nth`, `empty`, `singleton`, `is_empty`, `is_singleton`,
`iterate` (inherently sequential per APAS CS 20.3). MtEphSlice's 3 functions
all match because they delegate to field access or Arc clone.

**Differs from APAS (38 functions across StEph/StPer/MtEph):** Every function
that APAS specifies with parallel span (tabulate, map, filter, reduce, scan,
flatten, append, inject, ninject) has sequential implementation where span = work.
Specific patterns:

- **tabulate/map**: APAS Span O(max S(f)), actual Span O(Sigma S(f)) — sequential loop
- **reduce**: APAS Span O(lg|a| * max S(f)), actual delegates to reduce_iter — sequential
- **scan**: APAS Span O(lg|a|), actual O(|a|) — sequential loop
- **flatten**: APAS Span O(lg|a|), actual O(sum|a[i]|) — nested sequential loops
- **filter**: Built from sequential map + sequential flatten
- **append**: APAS Span O(1), actual O(|a|+|b|) — sequential tabulate
- **subseq**: APAS O(1) assumes pointer-based, actual O(length) copies elements
- **update/inject**: Clone entire Vec, then modify — O(|a|) work

### Chap21 — Prime Algorithms

Both algorithms use ArraySeqStPer, so inherit its sequential costs.

- **primesBF** (Alg 21.5): Work O(n^(3/2)) matches APAS. Span O(n^(3/2)) differs
  from APAS O(lg n) — sequential tabulate + filter.
- **primeSieve** (Alg 21.6): Work O(n lg n) matches APAS. Span O(n lg n) differs
  from APAS O(lg n) — sequential nested tabulate + flatten + loop.

### Chap23 — Primitive Tree Sequences

PrimTreeSeqStPer is Vec-backed (not tree-backed), so APAS tree-sequence cost spec
CS 20.6 (logarithmic costs) does not apply. All operations are linear:

- **length**: O(1) matches APAS CS 23.2.
- **nth**: O(1), matches CS 22.2 but not CS 20.6 O(lg n) — Vec is better here.
- **expose**: O(|a|) copies into left/right halves — APAS says O(1) for tree-based.
- **append**: O(|a|+|b|) — APAS says O(|lg(|a|/|b|)|) for balanced trees.
- **subseq**: O(length) — APAS says O(lg|a|) for tree-based.
- **update**: O(|a|) copies entire array — APAS says O(lg|a|) for tree-based.

## APAS Accuracy

All APAS annotations correctly reference the textbook cost specifications. The
discrepancies are implementation-level: APAS specifies parallel/tree-based costs,
while the current implementations are sequential/Vec-backed.
