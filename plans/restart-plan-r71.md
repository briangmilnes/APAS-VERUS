# R71 Restart Plan

## State

- Main: `53e61a030` — R70 merge complete, 4438 verified, 0 errors
- Holes: 0 (3 accepted, 0 actionable)
- Clean chapters: 46 of 46
- LOC: 25,255 spec / 36,119 proof / 55,826 exec / 50,556 rust = 215,393 total (576 files)

## Task: Iterator Standard Review

Review-only round. All 4 agents review 14 files each (56 total) for compliance with
`src/standards/iterators_standard.rs` (10 required components per file).

## Agent Assignments

| # | Agent | Files | Chapters |
|---|-------|-------|----------|
| 1 | Agent 1 | 14 | Chap05, Chap06, Chap17, Chap18 (ArraySeq only) |
| 2 | Agent 2 | 14 | Chap18, Chap19, Chap23, Chap37 (AVLTreeSeq/MtPer) |
| 3 | Agent 3 | 14 | Chap37, Chap41, Chap43 (partial) |
| 4 | Agent 4 | 14 | Chap43 (partial), Chap49, Chap50 |

## Prompt Files

- `plans/r71-agent1-iterator-review.md`
- `plans/r71-agent2-iterator-review.md`
- `plans/r71-agent3-iterator-review.md`
- `plans/r71-agent4-iterator-review.md`
