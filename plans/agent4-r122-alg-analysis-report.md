# Agent 4 — R122 Algorithmic Analysis Report

## Scope

Chapters 52-59. Replaced all 118 `NONE` placeholders with independent code review annotations.

## Validation

5449 verified, 0 errors. No code changes — annotation-only edits.

## Summary

| # | Chap | Annotations | Matches | Differs |
|---|------|-------------|---------|---------|
| 1 | 52 | 80 | 29 | 51 |
| 2 | 53 | 0 | 0 | 0 |
| 3 | 54 | 8 | 0 | 8 |
| 4 | 55 | 2 | 2 | 0 |
| 5 | 56 | 0 | 0 | 0 |
| 6 | 57 | 4 | 4 | 0 |
| 7 | 58 | 4 | 0 | 4 |
| 8 | 59 | 8 | 4 | 4 |
| | **Total** | **106** | **39** | **67** |

Note: 106 code review lines from 118 NONE replacements — some NONE lines were on
free functions (indented differently), but all 118 were replaced.

## Differs — Reasons by Category

### 1. Sequential span = work (most common)

Most DIFFERS arise because APAS gives the parallel span but the code implements a
sequential variant where span = work. This affects:

- **Chap52**: `out_neighbors`, `out_degree` in EdgeSetGraph (filter loop); `out_neighbors`,
  `out_degree`, `complement` in AdjMatrixGraph (row scan); `has_edge`, `out_neighbors` in
  AdjSeqGraph (linear scan/tabulate); `set_edge` in AdjMatrixGraph and AdjSeqGraph.

- **Chap54**: All 8 BFS functions. APAS references parallel Alg 54.4 (Work O(m lg n),
  Span O(d lg^2 n)) but all 4 implementations (StEph, StPer, MtEph, MtPer) use
  sequential queue/layer BFS with Work and Span both O(n + m).

- **Chap58**: All 4 Bellman-Ford functions. APAS says Span O(n lg n) (parallel edge
  relaxation per round), but code does sequential relaxation: Span O(nm).

- **Chap59**: 4 JohnsonStEph functions (F64/I64, trait + impl). Sequential Dijkstra loop
  gives Span O(mn lg n) vs APAS O(m lg n). The 4 JohnsonMtEph functions correctly use
  ParaPair! divide-and-conquer and match APAS.

### 2. num_edges not cached (Chap52)

APAS annotates `num_edges` as O(1)/O(1) for AdjTable, AdjSeq, and AdjMatrix
representations. But the implementations compute edge count by looping over all
vertices/entries:

- AdjTableGraph: O(n) sequential loop summing neighbor set sizes
- AdjSeqGraph: O(n) sequential loop summing degrees
- AdjMatrixGraph: O(n^2) nested sequential loops counting true entries

This affects all 4 variants (StEph, StPer, MtEph, MtPer) of each representation — 12 DIFFERS.

### 3. out_neighbors returns reference (Chap52 StPer/MtPer AdjSeq)

AdjSeqGraphStPer and AdjSeqGraphMtPer return `&ArraySeq*` (reference) for
`out_neighbors` — O(1)/O(1). APAS says O(d_g(v))/O(1) which models the cost of
mapping over neighbors, not just returning the collection.

### 4. set_edge rebuilds row (Chap52 AdjMatrix)

APAS says O(1)/O(1) for ephemeral matrix set_edge, but both MtEph and StEph rebuild
the entire row using tabulate — O(n) work and span.
