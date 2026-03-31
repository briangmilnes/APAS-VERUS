# R124 Agent 1 — Alg Analysis Code Review: Chap18

## Summary

Replaced all 105 `Claude-Opus-4.6 (1M): NONE` placeholders across 7 files in Chap18
with independent code review annotations.

## Files Modified

| # | Chap | File | NONEs | Matches | DIFFERS |
|---|------|------|-------|---------|---------|
| 1 | 18 | ArraySeq.rs | 18 | 6 | 12 |
| 2 | 18 | ArraySeqStEph.rs | 17 | 6 | 11 |
| 3 | 18 | ArraySeqStPer.rs | 17 | 6 | 11 |
| 4 | 18 | ArraySeqMtEph.rs | 18 | 6 | 12 |
| 5 | 18 | ArraySeqMtPer.rs | 17 | 6 | 11 |
| 6 | 18 | LinkedListStEph.rs | 9 | 5 | 4 |
| 7 | 18 | LinkedListStPer.rs | 9 | 5 | 4 |
| | | **Total** | **105** | **40** | **65** |

## Key Findings

### All implementations are sequential
Every function across all 7 files uses sequential `while` loops over Vec-backed storage.
No divide-and-conquer, no parallel join, no thread spawning in the trait implementations.
Consequence: Span = Work for every function.

### Common DIFFERS patterns

**1. Span differs on parallelizable operations (most common)**
APAS cost specs (from Ch20 CS 20.2) assume parallel implementations with O(1) or
O(lg n) span for map, tabulate, filter, reduce, scan, flatten, append, inject, ninject.
Code implements all of these as sequential loops, so span = work = O(n).

Affected functions: append, filter, update, inject, ninject, reduce, scan, map, tabulate,
flatten (across all 5 ArraySeq files).

**2. subseq: O(j) not O(1)**
APAS says Work O(1), Span O(1) for array subseq (slice-based, no copy).
Code clones j elements in a sequential loop: Work O(j), Span O(j).
Affects: ArraySeq.rs, ArraySeqStEph.rs, ArraySeqStPer.rs, ArraySeqMtEph.rs, ArraySeqMtPer.rs.

**3. collect: O(n^2) not O(n lg n)**
APAS says Work O(W(cmp) * |a| * lg|a|). Code uses linear `find_key` scan per pair,
giving O(n^2) total work. Only in ArraySeq.rs.

**4. LinkedList: Vec-backed, not pointer-linked**
LinkedList files are named as linked lists but backed by Vec. This makes:
- nth: O(1) not O(i) (Vec random access vs linked list traversal)
- subseq_copy: O(j) not O(start+j) (no traversal to start position)
- append: O(|a|+|b|) not O(|a|) (copies both, no pointer linking)
- update: O(n) not O(1) (clones full array)

### Matching functions (same across all files)
- length: O(1) — Vec::len()
- nth: O(1) — Vec index (array files only)
- empty: O(1) — Vec::new()
- singleton: O(1) — single push
- is_empty: O(1) — length check
- is_singleton: O(1) — length check
- iterate: O(n) — inherently sequential, matches APAS

### LinkedList-specific matches
- tabulate, map, filter, reduce: O(n) sequential — matches APAS Ch20 CS 20.7
- scan: O(n) — matches APAS Ch20 CS 20.7

## Technique

For each function: read the implementation body, traced control flow and loop structure,
determined work (total ops) and span (critical path length). Compared against the APAS
line already annotated on each function.
