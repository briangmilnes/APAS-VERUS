# R75 Restart Prompt (post-merge)

## State after R75 merge session

### Verified baseline
- **4794 verified, 0 errors, 0 warnings**
- **2619 RTT passed, 157 PTT passed**
- Main at commit `57542019b`
- All agent worktrees need rebase (not yet done — user must approve)

### What was done this session

1. **Merged 4 R75 agent branches** into main (agents 1, 2, 4, 5; agent 3 had no changes — stalled on /tmp writes).
2. **R75 results**: -27 holes (103→76), 42 clean chapters (up from 41), Chap43 now clean.
3. **Fixed agent1 merge**: reverted UnionFindStEph ClonePreservesView bound changes (union find work was broken), ungated ClonePreservesView import in KruskalStEph (exec trait, not spec-only).
4. **Eliminated last Verus warning**: replaced `derive(Clone)` on PQEntry in PrimStEph.rs with manual Clone impl following eq/clone standard.
5. **Regenerated hole analysis**: 76 holes across 4 chapters.

### Agent results

| Agent | Files | Delta | Key work |
|-------|-------|-------|----------|
| 1 | KruskalStEph, PrimStEph, float.rs | -3 | sort_edges_by_weight, 2x mst_weight proved; ClonePreservesView for WrappedF64 |
| 2 | TSPApproxStEph, BoruvkaStEph | -6 | get_neighbors, get_edge_weight, bridge_star_partition, boruvka_mst_with_seed, mst_weight, PartialEq fix |
| 3 | (no changes) | 0 | Stalled on /tmp write permissions |
| 4 | BSTRBMtEph, BSTSplayMtEph, BSTAVLMtEph | -6 | rotate_left/right, flip_colors, fix_up, insert_link, splay; 32 trigger annotations |
| 5 | AVLTreeSeqSt*, OrderedTableMtEph, AugOrderedTableMtEph, BSTSetAVLMtEph | -12 | Iterator external_body removal (4 files clean), BSTSetAVLMtEph BTreeSet removal |

### Hole report (76 total across 4 chapters)

| # | Chap | File | Holes | Type |
|---|------|------|-------|------|
| 1 | 37 | BSTRBMtEph.rs | 3 | 2 external_body (filter/reduce_parallel) + 1 assume (height) |
| 2 | 37 | BSTSplayMtEph.rs | 5 | 4 external_body (clone, build_balanced, filter/reduce_parallel) + 1 assume (height) |
| 3 | 37 | BSTSetAVLMtEph.rs | 13 | 8 assume (obeys_feq_clone) + 2 external_body (filter, reduce) + 3 external_body (union, intersection, difference) |
| 4 | 37 | BSTSetRBMtEph.rs | 16 | Same iterator/BTreeSet pattern as BSTSetAVLMtEph — needs same rewrite |
| 5 | 37 | BSTSetSplayMtEph.rs | 13 | Same iterator/BTreeSet pattern as BSTSetAVLMtEph — needs same rewrite |
| 6 | 64 | SpanTreeStEph.rs | 2 | star_contract closure interface + edges() missing wf ensures |
| 7 | 64 | TSPApproxStEph.rs | 2 | euler_tour_dfs (mutable visited set) + euler_tour (blocked by it) |
| 8 | 65 | UnionFindStEph.rs | 5 | 5 external_body |
| 9 | 65 | KruskalStEph.rs | 1 | 1 external_body (kruskal_mst — depends on UnionFind) |
| 10 | 65 | PrimStEph.rs | 2 | 1 external_body + 1 assume (float TotalOrder) |
| 11 | 66 | BoruvkaMtEph.rs | 12 | 11 external_body + 1 external |
| 12 | 66 | BoruvkaStEph.rs | 2 | vertex_bridges (iterator finiteness) + boruvka_mst (termination proof) |

### Notes for R76 planning

- **BSTSetRBMtEph (16) and BSTSetSplayMtEph (13)** need the same BTreeSet removal rewrite that Agent 5 did for BSTSetAVLMtEph. Agent 5's report says it "pioneers the pattern."
- **UnionFindStEph (5)** — Agent 1 tried ClonePreservesView approach and failed. We have at least one clean UnionFind file elsewhere that may provide patterns.
- **BoruvkaMtEph (12)** — Agent 3 stalled on /tmp permissions. Largest single-file hole count remaining. Needs a fresh attempt.
- **Chap37 has 50 of 76 holes** — most are structural (eq/clone assumes, thread-spawn boundaries, recursive Clone cycle errors in Verus).
- **Agents need rebase** before R76 can start. User must approve.

### R75 session commits (oldest to newest)

```
084476d4b R75: Chap37 BSTRBMtEph -5 holes, BSTSplayMtEph -1 hole, trigger fixes (agent4 FF)
8f9d46a7a Merge branch 'agent5/ready'
98988dd3f Merge branch 'agent2/ready'
f5b4217e0 R75 merge: 3 agents, -24 holes (103→79), 42 clean chapters
e3d24e6e5 Merge branch 'agent1/ready'
cfd7ca59b Fix agent1 merge: revert UnionFind ClonePreservesView bounds, ungate KruskalStEph import
c68744aa0 R75 final: 4 agents merged, -27 holes (103→76), 42 clean chapters
57542019b Replace derive(Clone) with manual Clone impl for PQEntry, eliminating last warning
```

### Daily proof table

| Round | Holes Start | Holes End | Delta | Clean Chaps | Dirty Chaps | Verified |
|-------|-------------|-----------|-------|-------------|-------------|----------|
| R73   | —           | 169       | —     | 40          | 6           | 4735     |
| R74   | 169         | 103       | -66   | 41          | 5           | 4742     |
| R75   | 103         | 76        | -27   | 42          | 4           | 4794     |

### What to do next

1. Rebase agents: `scripts/rebase-agents.sh` (user must approve — agents may have stale state)
2. Write R76 prompts targeting remaining 76 holes
3. Priority targets: BSTSetRBMtEph/BSTSetSplayMtEph (BTreeSet rewrite), UnionFindStEph, BoruvkaMtEph
