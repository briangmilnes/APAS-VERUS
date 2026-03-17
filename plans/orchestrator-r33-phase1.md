# R33 Plan: Phase 1 Quick Wins (4 Agents)

## Goal

Close Chap45, Chap52, Chap58, Chap59. Pilot Chap43 delegation wrappers.
Fix BellmanFord/Johnson string→enum. Prove Dijkstra assumes.

Current: ~139 real holes (196 total minus 53 false positives minus 4 Example).
Target: ~110-120 real holes after this round.

## Agent Assignments (no file conflicts)

### Agent 1: Chap58 + Chap59 (BellmanFord/Johnson enum refactor)
### Agent 2: Chap43 (trivial fixes + OrderedSetStEph delegation pilot)
### Agent 3: Chap45 + Chap52
### Agent 4: Chap57 Dijkstra assumes

## Verification

Each agent runs `scripts/validate.sh` after changes (0 errors required).
Merge order: any (chapter-disjoint assignments).
