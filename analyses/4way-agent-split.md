<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# 4-Way Agent Split

**Updated:** After main at b39966e. All agent worktrees reset to main.

## Agent Chapter Assignments

| # | Agent | Chapters | Scope |
|---|-------|----------|-------|
| 1 | agent1 | Chap02–Chap23 | Foundation: scheduler, sort, sets, graphs, fib, sequences, trees |
| 2 | agent2 | Chap26–Chap41 | Div-conq, MCSS, order stats, sort, BSTs, sets |
| 3 | agent3 | Chap42–Chap50 | Tables, ordered structures, hash, PQ, DP intro |
| 4 | agent4 | Chap51–Chap66 | DP, graphs, SSSP, MST, Boruvka |

## Explicit Chapter Lists

| Agent | Chapters |
|-------|----------|
| agent1 | Chap02, Chap03, Chap05, Chap06, Chap11, Chap12, Chap17, Chap18, Chap19, Chap21, Chap23 |
| agent2 | Chap26, Chap27, Chap28, Chap35, Chap36, Chap37, Chap38, Chap39, Chap40, Chap41 |
| agent3 | Chap42, Chap43, Chap44, Chap45, Chap47, Chap49, Chap50 |
| agent4 | Chap51, Chap52, Chap53, Chap54, Chap55, Chap56, Chap57, Chap58, Chap59, Chap61, Chap62, Chap63, Chap64, Chap65, Chap66 |

## Tasks Per Agent

### Agent1 (Chap02–Chap23)

1. Run proof holes on Chap02, 03, 05, 06, 11, 12, 17, 18, 19, 21, 23.
2. Propose fixes table for holed files (exclude accepted patterns).
3. Verusification priority: fix bare_impl, non-standard assume, external_body hiding logic.
4. Keep Chap18/19 ArraySeq solid — used by all downstream.

### Agent2 (Chap26–Chap41)

1. Run proof holes on Chap26, 27, 28, 35, 36, 37, 38, 39, 40, 41.
2. Propose fixes table for holed files.
3. Verusification priority: BST modules, AVLTreeSet, ArraySet.
4. Bogus view target: AVLTreeSetMtPer spec_set_view (Chap41).

### Agent3 (Chap42–Chap50)

1. Run proof holes on Chap42, 43, 44, 45, 47, 49, 50.
2. Propose fixes table for holed files.
3. Verusification priority: Table, OrderedTable, OrderedSet, hash tables, PQ, DP.
4. Bogus view targets: OrderedSetMtEph, OrderedTableMtPer (Chap43).

### Agent4 (Chap51–Chap66)

1. Run proof holes on Chap51, 52, 53, 54, 55, 56, 57, 58, 59, 61, 62, 63, 64, 65, 66.
2. Propose fixes table for holed files.
3. Verusification priority: DP, BFS, SSSP, MST, Boruvka.
4. Many Chap52/53/55/59/61 modules commented out — document blockers.

## Worktree Setup

| Worktree | Path | Branch |
|----------|------|--------|
| main | `~/projects/APAS-VERUS` | main |
| agent1 | `~/projects/APAS-VERUS-agent1` | agent1/ready |
| agent2 | `~/projects/APAS-VERUS-agent2` | agent2/ready |
| agent3 | `~/projects/APAS-VERUS-agent3` | agent3/ready |
| agent4 | (create if needed) | agent4/ready |

To add agent4 worktree:

```bash
cd ~/projects/APAS-VERUS
git worktree add -b agent4/ready ../APAS-VERUS-agent4
```

## Reset Agents to Main

After main merges and pushes:

```bash
cd ~/projects/APAS-VERUS-agent1 && ./scripts/reset-agent-to-main.sh
cd ~/projects/APAS-VERUS-agent2 && ./scripts/reset-agent-to-main.sh
cd ~/projects/APAS-VERUS-agent3 && ./scripts/reset-agent-to-main.sh
# agent4 when created
```
