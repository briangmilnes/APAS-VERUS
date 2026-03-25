# R75 Restart Prompt

## State at end of R74 merge session

### Verified baseline
- **4742 verified, 0 errors, 130 warnings**
- **2619 RTT passed, 157 PTT passed**
- Main at commit after R74 merge
- All 5 agent worktrees rebased and clean

### What was done this session

1. **Merged all 4 R74 agent branches** into main (agents 1-4 clean; agent 5 never started).
2. **R74 results**: -66 holes (169→103), 41 clean chapters (up from 40), Chap41 now clean.
3. **Regenerated hole analysis**: 103 holes across 5 chapters.
4. **Wrote R75 prompts** for 5 agents targeting 74 of the 103 holes.

### Hole report (103 total)

| # | Chap | File | Holes | Type |
|---|------|------|-------|------|
| 1 | 37 | AVLTreeSeqStEph.rs | 2 | 2 external_body (iterator) |
| 2 | 37 | AVLTreeSeqStPer.rs | 2 | 2 external_body (iterator) |
| 3 | 37 | BSTRBMtEph.rs | 8 | 7 external_body + 1 assume |
| 4 | 37 | BSTSetAVLMtEph.rs | 17 | 15 external_body + 2 external |
| 5 | 37 | BSTSetRBMtEph.rs | 16 | 14 external_body + 2 external |
| 6 | 37 | BSTSetSplayMtEph.rs | 13 | 13 external_body |
| 7 | 37 | BSTSplayMtEph.rs | 6 | 5 external_body + 1 assume |
| 8 | 43 | AugOrderedTableMtEph.rs | 2 | 2 external_body (iterator) |
| 9 | 43 | OrderedTableMtEph.rs | 2 | 2 external_body (iterator) |
| 10 | 64 | SpanTreeStEph.rs | 2 | 2 external_body |
| 11 | 64 | TSPApproxStEph.rs | 4 | 4 external_body |
| 12 | 65 | KruskalStEph.rs | 3 | 3 external_body |
| 13 | 65 | PrimStEph.rs | 3 | 2 external_body + 1 assume (float TotalOrder) |
| 14 | 65 | UnionFindStEph.rs | 5 | 5 external_body |
| 15 | 66 | BoruvkaMtEph.rs | 12 | 11 external_body + 1 external |
| 16 | 66 | BoruvkaStEph.rs | 6 | 5 external_body + 1 external |

### R75 agent assignments

| Agent | Target | Holes | Files |
|-------|--------|-------|-------|
| 1 | Chap65 MST + Union-Find | 11 | UnionFindStEph, KruskalStEph, PrimStEph |
| 2 | Chap64 graphs + Chap66 BoruvkaStEph | 12 | SpanTreeStEph, TSPApproxStEph, BoruvkaStEph |
| 3 | Chap66 BoruvkaMtEph | 12 | BoruvkaMtEph |
| 4 | Chap37 BSTRBMtEph + BSTSplayMtEph | 14 | BSTRBMtEph, BSTSplayMtEph |
| 5 | Chap37 BSTSetAVLMtEph + iterators | 25 | BSTSetAVLMtEph, AVLTreeSeqSt*, OrderedTableMtEph, AugOrderedTableMtEph |

Prompts are at `plans/r75-agent{1-5}-prompt.md`.

### Remaining 29 holes NOT assigned to R75 agents

- BSTSetRBMtEph.rs (16 holes) — same iterator pattern as BSTSetAVLMtEph, deferred to R76
- BSTSetSplayMtEph.rs (13 holes) — same iterator pattern, deferred to R76

### What to do when agents finish

1. Read reports: `plans/agent{1-5}-round75-report.md`
2. Check agent status: `scripts/survey-agents.sh`
3. Merge sequentially: `scripts/merge-agent.sh agent1/ready`, validate, repeat
4. After all merges: `scripts/all-holes-by-chap.sh` to refresh analysis
5. Commit, push, then ask user before rebasing agents
6. Update daily proof table

### Daily proof table

| Round | Holes Start | Holes End | Delta | Clean Chaps | Dirty Chaps | Verified |
|-------|-------------|-----------|-------|-------------|-------------|----------|
| R73   | —           | 169       | —     | 40          | 6           | 4735     |
| R74   | 169         | 103       | -66   | 41          | 5           | 4742     |
