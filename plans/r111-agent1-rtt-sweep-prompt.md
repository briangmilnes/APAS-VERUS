# R111 Agent 1 — RTT coverage sweep. AFK. PBOGH.

## Context

We have 3060 RTTs across 46 chapters but coverage is uneven. Some chapters have
500+ test functions, others have 7. Your job is to go chapter by chapter,
read the source modules, read the existing tests, identify gaps in coverage and
edge cases, and write tests.

## Current RTT counts (lowest first)

| # | Chap | Tests | Src Files | Description |
|---|------|-------|-----------|-------------|
| 1 | 03 | 7 | 1 | InsertionSort |
| 2 | 11 | 11 | 5 | Fibonacci (5 variants, 3 missing test files) |
| 3 | 64 | 12 | 3 | SpanTree, TSPApprox |
| 4 | 63 | 13 | 2 | BipartiteMatch |
| 5 | 61 | 14 | 4 | MaxFlow |
| 6 | 58 | 15 | 2 | SSSPResult (Float) |
| 7 | 62 | 17 | 4 | StarPartition, VertexMatching |
| 8 | 36 | 23 | 3 | Treap |
| 9 | 57 | 24 | 3 | SSSPResult (I64) |
| 10 | 17 | 28 | 1 | MathSeq |
| 11 | 66 | 29 | 2 | Boruvka |
| 12 | 38 | 31 | 2 | BSTParaSt |
| 13 | 59 | 31 | 4 | BellmanFord, Dijkstra |
| 14 | 40 | 33 | 3 | BSTRankedSt |
| 15 | 35 | 40 | 4 | AVLTree |

## Missing test files

These source modules have no test file at all:

- Chap02: HFSchedulerMtEph, FibonacciHFScheduler
- Chap11: FibonacciMtEph2Threads, FibonacciMtEphRecomputes, FibonacciMtPerAllThreads, FibonacciMtPerTSM, FibonacciStEph
- Chap19: ArraySeqMtEphSlice
- Chap47: ChainedHashTable, DoubleHashFlatHashTableStEph, FlatHashTable, LinkedListChainedHashTableStEph, LinProbFlatHashTableStEph, QuadProbFlatHashTableStEph, VecChainedHashTableStEph
- Chap52: AdjMatrixGraphMtEph, AdjSeqGraphMtEph

## What to test

For each module, read the trait to understand every public function. Then:

1. **Basic construction**: new/empty, from_vec, singleton — verify the returned
   structure has the expected view.
2. **Insert/delete/update**: Single element, multiple elements, duplicates,
   removing what's not there.
3. **Lookup/query**: contains, find, get, first, last, previous, next, rank,
   select — with present and absent keys.
4. **Boundary cases**: Empty collection, single element, large collections (100+
   elements), max/min keys.
5. **Edge cases**: Insert then delete same element, delete from empty, double
   insert, operations after clear.
6. **Iterator coverage**: If the module has iterators, test that iteration
   produces the right elements in the right order. Compare collected output
   to expected sequence.
7. **Mt variants**: Test concurrent access — multiple threads reading, one
   thread writing while others read. Use timeouts on threaded tests.
8. **Clone/Eq**: If the module implements Clone and PartialEq, test that
   cloned == original, and that different structures are !=.

## How to work

1. Start with the chapters that have the fewest tests (top of the table above).
2. For each chapter:
   a. Read the source files to understand the public API.
   b. Read existing test files to see what's already covered.
   c. Write new tests for uncovered functions and edge cases.
   d. Run `scripts/rtt.sh` to confirm everything passes.
3. Move to the next chapter.
4. Commit after every 2-3 chapters so work is not lost.

## Skip rules

- Skip Example files (e.g., Example41_3.rs) — these are textbook demos.
- Algorithm, Exercise, and Problem files CAN get RTTs — they have real exec code.
- Skip MCSSSpec (Chap28) — spec-only, no exec functions.
- Skip Chap65 — commented out of lib.rs.
- If a module's constructors require complex setup you can't figure out from
  reading the code, skip it and note why in your report.

## Resource rules

- Only run `scripts/rtt.sh` — do NOT run `scripts/validate.sh` or `scripts/ptt.sh`.
- RTTs are cheap. Run them freely after each batch of changes.
- No subagents.

## STEP 50

## Report

Write `plans/agent1-r111-rtt-sweep-report.md`. Include:
- Table of chapters visited with tests before/after counts
- Notable edge cases found
- Modules skipped with reasons
