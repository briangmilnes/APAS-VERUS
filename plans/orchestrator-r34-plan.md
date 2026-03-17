# R34 Plan: Sortedness Infrastructure + Hash Tables + Quick Wins

## State After R33

- 4154 verified, 0 errors, 2613 RTT, 147 PTT
- 37 clean chapters, 9 holed
- 171 total holes, ~165 structural FPs detected
- Chap52 closed in R33. Chap58 closed in R33.

## R33 Lesson

Agent2's R33 report revealed Chap43 delegation wrappers are NOT
mechanical — the ordering operations (first, last, previous, next,
rank, select) need a sortedness spec that doesn't exist in
AVLTreeSetStEph. Phase 2 of the original plan is blocked until
sortedness infrastructure is added to Chap41.

## R34 Strategy

Split work between infrastructure (unblocking Chap43 for R35+)
and independent hole reduction in other chapters.

## Agent Assignments

| # | Agent | Chapters | Target Holes | Goal |
|---|-------|----------|-------------|------|
| 1 | 1 | 41 | Sortedness spec | Enable Chap43 |
| 2 | 2 | 47 | 14 real | Hash table proofs |
| 3 | 3 | 37,45,57,59 | 5-6 real | Close chapters |
| 4 | 4 | 39 | 6 assumes | RwLock delegation |

No file conflicts between agents.

## Expected Outcomes

- Agent 1: Sortedness spec added to AVLTreeSetStEph. Enables
  R35 mass production on Chap43 ordering operations.
- Agent 2: -10 to -14 holes in Chap47. Possibly close chapter.
- Agent 3: Close Chap37. Reduce Chap45, Chap57, Chap59.
- Agent 4: -4 to -6 holes in BSTTreapMtEph.

**Expected total: -20 to -26 real holes + infrastructure.**

## Merge Order

Any order (chapter-disjoint). Validate after each merge.
