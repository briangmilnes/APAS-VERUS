# Agent 3 Round 6 — Near-Clean + DP + Graph Search (Chap65, 52, 53, 50, 45)

## Mode: AFK — execute relentlessly

Read CLAUDE.md and `src/standards/*.rs` before starting. Do the proof work.
Run `scripts/validate.sh` after each file. Fix errors before moving on.
When done, commit all changes, push to `agent3/ready`, then stop.

## Assignment

**55 holes across 5 chapters. Target: -15.**

| # | Chap | File | Holes | Type |
|---|------|------|-------|------|
| 1 | 65 | UnionFindStEph.rs | 1 | assume (eq bridge) |
| 2 | 52 | EdgeSetGraphMtPer.rs | 1 | external_body |
| 3 | 53 | GraphSearchStEph.rs | 2 | mixed |
| 4 | 53 | GraphSearchStPer.rs | 2 | mixed |
| 5 | 53 | GraphSearchMtPer.rs | 2 | assume |
| 6 | 53 | PQMinStEph.rs | 3 | mixed |
| 7 | 53 | PQMinStPer.rs | 3 | mixed |
| 8 | 50 | MatrixChainMtEph.rs | 7 | 5 assume + 2 ext_body |
| 9 | 50 | MatrixChainMtPer.rs | 2 | ext_body |
| 10 | 50 | OptBinSearchTreeMtEph.rs | 4 | mixed |
| 11 | 50 | OptBinSearchTreeMtPer.rs | 2 | ext_body |
| 12 | 50 | OptBinSearchTreeStEph.rs | 1 | mixed |
| 13 | 50 | OptBinSearchTreeStPer.rs | 1 | mixed |
| 14 | 45 | BinaryHeapPQ.rs | 8 | assume (multiset) |
| 15 | 45 | BalancedTreePQ.rs | 14 | mixed |
| 16 | 45 | Example45_2.rs | 1 | external |
| 17 | 45 | HeapsortExample.rs | 1 | external |

## Strategy

### Phase 1: Close near-clean chapters

**Chap65/UnionFindStEph.rs (1 hole)**: Single eq-bridge assume. Read the assume,
determine if it's a PartialEq bridge (accept per standard) or provable. Close Chap65
for +1 clean chapter.

**Chap52/EdgeSetGraphMtPer.rs (1 hole)**: external_body on out_neighbors. Uses
AVLTreeSetMtPer::filter. Previous assessment: blocked on Chap41 filter spec. Check
if the spec is now sufficient. If still blocked, document and move on.

### Phase 2: Chap53 graph search

- GraphSearchStEph.rs (2 holes): Previously 7, reduced to 2 in Round 4b. Read the
  remaining holes — likely frontier set or ensures gaps.
- PQMinStEph.rs (3 holes): Previously 5, reduced to 3. Read remaining.
- GraphSearchStPer.rs (2 holes): StPer variants of the above.
- PQMinStPer.rs (3 holes): StPer wf gap — may need AVLTreeSetStPer.to_seq() wf fix.
- GraphSearchMtPer.rs (2 holes): Mt wrapper assumes.

### Phase 3: Chap50 DP lock-boundary

MatrixChainMtEph.rs (7 holes) and OptBinSearchTreeMtEph.rs (4 holes) are the main
targets. These have lock-boundary assumes from Arc deref patterns (converted in
Round 4a). Read each assume and determine provability.

The remaining 6 holes in StEph/StPer/MtPer are smaller.

### Phase 4: Chap45 BinaryHeapPQ (if time permits)

BinaryHeapPQ.rs (8 holes): multiset proof obligations — insert/delete maintain multiset
equality. BalancedTreePQ.rs (14 holes): mixed. Example files (2 holes): stay external.

## Rules

- Do NOT convert `assume` to `accept` wholesale. Read each assume, try to prove first.
- Do NOT add new assumes or external_body.
- Do NOT modify files outside your assignment (Chap45, 50, 52, 53, 65 only).
- Run `scripts/validate.sh` after each file change.

## Prior Work

Agent 3's full history is in `plans/AGENT3.md`. Key context:
- Round 4a: Chap50 21→10 holes via arc_deref + accept patterns.
- Round 4b: Chap53 11→8 holes. pq_explore converted to while loop.
- StPer blocked by AVLTreeSetStPer.to_seq() wf gap (coordinate with Agent 4).
- DFS frontier bug identified: `frontier_new = neighbors \ visited_new` drops unselected.

## Baseline

3771 verified, 0 errors.

## Deliverable

When done: commit, push to `agent3/ready`, update `plans/AGENT3.md` with results.
