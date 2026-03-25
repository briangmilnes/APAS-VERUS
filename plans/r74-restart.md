# R74 Restart Prompt

## State at end of R73 merge session

### Verified baseline
- **4735 verified, 0 errors, 130 warnings**
- **2619 RTT passed, 157 PTT passed**
- Main at commit `ec32deedd`
- All 4 agent worktrees rebased and clean at `ec32deedd`
- Agent 5 worktree may need creation: `git worktree add /home/milnes/projects/APAS-VERUS-agent5 -b agent5/ready ec32deedd`

### What was done this session

1. **Merged all 4 R73 agent branches** into main (agents 1, 3, 4 clean; agent 2 had issues).
2. **Fixed Agent 2's sequentialization**: Agent 2 stripped 15 ParaPair! calls from 3 files
   (BSTRBMtEph.rs, BSTSetAVLMtEph.rs, BSTSetRBMtEph.rs) and replaced with sequential
   recursion. Restored 10 ParaPair calls in BSTSetAVLMtEph and BSTSetRBMtEph. The 5 free
   functions in BSTRBMtEph stay sequential because they take `&Link<T>` (borrowed references
   can't be sent to threads without Arc wrapping Node, which would change the data structure).
3. **Added `'static` to `StTInMtT`** in `src/Concurrency.rs` (required for ParaPair thread
   spawning). Also updated the blanket impl.
4. **Removed `dev_only`, `wf`, `all_chapters` feature gates** from Cargo.toml and lib.rs.
   Only `experiments_only` remains. Updated CLAUDE.md.
5. **Ran hole analysis**: 169 holes across 6 chapters (40 clean, 6 holed).
6. **Wrote R74 prompts** for 5 agents targeting 91 of the 169 holes.

### Hole report (169 total)

| Chap | File | Holes | Type |
|------|------|-------|------|
| 37 | AVLTreeSeqStEph.rs | 2 | external_body (iterator) |
| 37 | AVLTreeSeqStPer.rs | 2 | external_body (iterator) |
| 37 | BSTRBMtEph.rs | 20 | 10 assume + 10 external_body |
| 37 | BSTSetAVLMtEph.rs | 17 | 15 external_body + 2 external |
| 37 | BSTSetBBAlphaMtEph.rs | 6 | 6 assume (rwlock wf) |
| 37 | BSTSetPlainMtEph.rs | 6 | 6 assume (rwlock wf) |
| 37 | BSTSetRBMtEph.rs | 16 | 14 external_body + 2 external |
| 37 | BSTSetSplayMtEph.rs | 13 | 13 external_body |
| 37 | BSTSplayMtEph.rs | 8 | 1 assume + 7 external_body |
| 41 | AVLTreeSetMtPer.rs | 5 | 5 assume (type-axiom) |
| 43 | AugOrderedTableMtEph.rs | 2 | 2 external_body (iterator) |
| 43 | OrderedTableMtEph.rs | 2 | 2 external_body (iterator) |
| 43 | OrderedTableStEph.rs | 15 | 13 assume + 2 external_body |
| 43 | OrderedTableStPer.rs | 20 | 20 assume (type-axiom) |
| 64 | SpanTreeStEph.rs | 2 | 2 external_body |
| 64 | TSPApproxStEph.rs | 4 | 4 external_body |
| 65 | KruskalStEph.rs | 3 | 3 external_body |
| 65 | PrimStEph.rs | 3 | 1 assume + 2 external_body |
| 65 | UnionFindStEph.rs | 5 | 5 external_body |
| 66 | BoruvkaMtEph.rs | 12 | 11 external_body + 1 external |
| 66 | BoruvkaStEph.rs | 6 | 5 external_body + 1 external |

### R74 agent assignments

| Agent | Target | Holes | Files |
|-------|--------|-------|-------|
| 1 | Chap43 OrderedTableStPer type-axiom assumes | 20 | OrderedTableStPer.rs |
| 2 | Chap43 OrderedTableStEph assumes + rank/select | 15 | OrderedTableStEph.rs |
| 3 | Chap37 rwlock wf + Chap41 type-axiom | 17 | BSTSetPlainMtEph, BSTSetBBAlphaMtEph, AVLTreeSetMtPer |
| 4 | Chap37 BSTRBMtEph + BSTSplayMtEph | 28 | BSTRBMtEph, BSTSplayMtEph |
| 5 | Chap64/65/66 graph algorithms | 35 | SpanTree, TSPApprox, UnionFind, Kruskal, Prim, Boruvka |

Prompts are at `plans/r74-agent{1-5}-prompt.md`.

### Remaining 78 holes NOT assigned to R74 agents

These are mostly BSTSet*MtEph external_body holes in Chap37 (BSTSetAVLMtEph: 17,
BSTSetRBMtEph: 16, BSTSetSplayMtEph: 13 = 46 total). Many use std iterators (BTreeSet,
filter_map, fold, collect) that Verus can't compile — they may need restructuring to
avoid std iterator adapters. Also: AVLTreeSeqStEph/StPer (4 iterator holes),
AugOrderedTableMtEph/OrderedTableMtEph (4 iterator holes).

### What to do when agents finish

1. Read reports: `plans/agent{1-5}-round74-report.md`
2. Check agent status: `scripts/survey-agents.sh`
3. Merge sequentially: `scripts/merge-agent.sh agent1/ready`, validate, repeat
4. After all merges: `scripts/all-holes-by-chap.sh` to refresh analysis
5. Commit, push, then ask user before rebasing agents
6. Update daily proof table

### Daily proof table

| Round | Holes Start | Holes End | Delta | Clean Chaps | Dirty Chaps | Verified |
|-------|-------------|-----------|-------|-------------|-------------|----------|
| R73   | —           | 169       | —     | 40          | 6           | 4735     |
