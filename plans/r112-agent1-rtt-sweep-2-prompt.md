# R112 Agent 1 — RTT coverage sweep round 2. AFK. PBOGH.

## Context

You did round 1 in R111: +98 tests across 21 files. Good work. There are
still 14 chapters under 40 tests. Finish the job.

Your R111 report is at `plans/agent1-r111-rtt-sweep-report.md`. Read it to
see what you already covered and what you skipped.

## Chapters to cover (under 40 tests, not yet swept)

| # | Chap | Tests | Description |
|---|------|-------|-------------|
| 1 | 58 | 15 | SSSPResult (Float) |
| 2 | 57 | 24 | SSSPResult (I64) |
| 3 | 66 | 29 | Boruvka |
| 4 | 59 | 31 | BellmanFord, Dijkstra |
| 5 | 21 | 33 | Algorithm21_1 through 21_6 |
| 6 | 27 | 38 | Reduce |
| 7 | 02 | 38 | HFScheduler, FibonacciHFScheduler |
| 8 | 44 | 40 | PriorityQueue |
| 9 | 40 | 41 | BSTRankedSt |
| 10 | 38 | 41 | BSTParaSt |
| 11 | 54 | 42 | BFS |
| 12 | 53 | 43 | GraphSearch, PQMin |
| 13 | 19 | 47 | ArraySeqSlice |
| 14 | 42 | 57 | ArraySet, Table |

Also revisit any chapters from round 1 where you noted thin coverage.

## What to test

For each module, read the trait to understand every public function. Then:

1. **Basic construction**: new/empty, from_vec, singleton.
2. **Insert/delete/update**: Single, multiple, duplicates, removing absent keys.
3. **Lookup/query**: contains, find, get, first, last, previous, next, rank, select.
4. **Boundary cases**: Empty, single element, large (100+), max/min keys.
5. **Edge cases**: Insert-then-delete same, delete from empty, double insert.
6. **Iterator coverage**: Collect and compare to expected sequence.
7. **Mt variants**: Concurrent reads, write-while-reading. Timeouts on threads.
8. **Clone/Eq**: Cloned == original, different structures are !=.

## Rules

- Skip Example files. Algorithm/Exercise/Problem files CAN get RTTs.
- Skip Chap65 (commented out).
- Only run `scripts/rtt.sh`. Do NOT run validate or ptt.
- Commit after every 2-3 chapters.
- No subagents.

## STEP 50

## Report

Write `plans/agent1-r112-rtt-sweep-2-report.md`.
