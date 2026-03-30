# R112 Agent 4 — RTT coverage sweep (graph + DP chapters). AFK. PBOGH.

## Objective

Add runtime tests for low-coverage chapters. Agent 1 did a first pass in
R111 (+98 tests). You cover the chapters it didn't reach or left thin.

## Chapters to cover

| # | Chap | Tests | Description |
|---|------|-------|-------------|
| 1 | 58 | 15 | SSSPResult (Float) |
| 2 | 57 | 24 | SSSPResult (I64) |
| 3 | 66 | 29 | Boruvka |
| 4 | 59 | 31 | BellmanFord, Dijkstra |
| 5 | 21 | 33 | Algorithm21_1 through 21_6 |
| 6 | 27 | 38 | Reduce |
| 7 | 44 | 40 | PriorityQueue |
| 8 | 54 | 42 | BFS |
| 9 | 53 | 43 | GraphSearch, PQMin |
| 10 | 19 | 47 | ArraySeqSlice |

## What to test

For each module, read the trait to understand every public function. Then:

1. **Basic construction**: new/empty, from_vec, singleton.
2. **Insert/delete/update**: Single, multiple, duplicates, removing absent keys.
3. **Lookup/query**: contains, find, get, first, last, previous, next.
4. **Boundary cases**: Empty, single element, large (100+), max/min keys.
5. **Edge cases**: Insert-then-delete same, delete from empty, double insert.
6. **Graph algorithms**: Various graph topologies — path, cycle, star, complete,
   disconnected, self-loop, negative weights (where applicable).
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

Write `plans/agent4-r112-rtt-sweep-report.md`. Include table of chapters
visited with tests before/after counts.
