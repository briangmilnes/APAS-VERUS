# R113 Agent 1 — RTT coverage sweep round 3. AFK. PBOGH.

## Context

Two prior RTT sweeps (R111, R112) brought us from 3060 to 3413. There are
still chapters under 40 tests. Your goal: get every chapter to 40+ tests,
or document why it can't be done.

## Chapters to cover (under 40 tests)

| # | Chap | Tests | Description |
|---|------|-------|-------------|
| 1 | 03 | 15 | InsertionSort — needs edge cases |
| 2 | 11 | 17 | Fibonacci — Mt variants need tests |
| 3 | 12 | 18 | Exercises — Algorithm/Exercise files can get RTTs |
| 4 | 64 | 20 | SpanTree, TSPApprox — graph topologies |
| 5 | 63 | 27 | Connectivity — more graph shapes |
| 6 | 58 | 29 | BellmanFord — negative weights, predecessors |
| 7 | 62 | 30 | StarPartition, StarContraction, VertexMatching |
| 8 | 61 | 32 | MaxFlow, EdgeContraction |
| 9 | 36 | 34 | QuickSort, Treap — permutations, edge cases |
| 10 | 59 | 38 | Johnson APSP — more graph variants |

## What to test

Read the source trait for each module. Test every public exec function:

1. **Construction**: new, empty, from_vec, singleton.
2. **Core operations**: insert, delete, find, contains, size.
3. **Edge cases**: empty input, single element, duplicates, max/min values.
4. **Graph algorithms**: path, cycle, star, complete, disconnected, self-loop,
   negative weights, unreachable vertices.
5. **Mt variants**: concurrent access with timeouts.
6. **Clone/Eq/Display**: if implemented.
7. **Iterators**: collect and verify order/contents.

## Rules

- Skip Example files. Algorithm/Exercise/Problem files CAN get RTTs.
- Skip Chap65 (commented out).
- Only run `scripts/rtt.sh`. Do NOT run validate or ptt.
- Commit after every 2-3 chapters.
- No subagents.
- Do NOT duplicate test names from existing files — read them first.

## STEP 50

## Report

Write `plans/agent1-r113-rtt-sweep-3-report.md`.
