# R76 Restart Prompt

## State

- **4794 verified, 0 errors, 0 warnings**
- **2619 RTT, 157 PTT**
- Main at commit `57542019b`
- **76 holes across 4 chapters** (37, 64, 65, 66)
- Agents need rebase before launching

## Agent assignments (76 holes total)

| Agent | Files | Holes | Focus |
|-------|-------|-------|-------|
| 1 | BSTSetRBMtEph (Chap37) | 16 | BTreeSet rewrite — replicate AVL pattern |
| 2 | BSTSetSplayMtEph (Chap37) | 13 | BTreeSet rewrite — replicate AVL pattern |
| 3 | BoruvkaMtEph + BoruvkaStEph (Chap66) | 14 | Largest single-file target, Mt proof narrowing |
| 4 | BSTRBMtEph, BSTSplayMtEph (Chap37) + SpanTreeStEph, TSPApproxStEph (Chap64) + UnionFindStEph (Chap65) | 17 | Mixed: height assumes, closures, DFS rewrite, UnionFind |

Unassigned holes (16): BSTSetAVLMtEph residual (13) + KruskalStEph (1) + PrimStEph (2).
These are structural (obeys_feq_clone, float TotalOrder, ParaPair closures) and deferred.

## Expected outcomes

- **Agents 1+2**: ~20-22 holes eliminated (BTreeSet rewrites, following proven AVL pattern)
- **Agent 3**: 2-6 holes eliminated (Mt narrowing, PartialEq, mst_weight, vertex_bridges)
- **Agent 4**: 3-8 holes eliminated (height assumes, UnionFind, SpanTree, TSPApprox)
- **Optimistic total**: 76→~40-50 holes

## Steps to launch

1. Rebase agents: `scripts/rebase-agents.sh` (user must approve)
2. Launch agents with prompts from `plans/r76-agent{1,2,3,4}-prompt.md`

## Daily proof table

| Round | Holes Start | Holes End | Delta | Clean Chaps | Dirty Chaps | Verified |
|-------|-------------|-----------|-------|-------------|-------------|----------|
| R73   | —           | 169       | —     | 40          | 6           | 4735     |
| R74   | 169         | 103       | -66   | 41          | 5           | 4742     |
| R75   | 103         | 76        | -27   | 42          | 4           | 4794     |
| R76   | 76          |           |       | 42          | 4           |          |
