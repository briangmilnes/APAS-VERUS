# Daily Proof Table — R88 through R98

Generated 2026-03-28 from git history and agent reports.

| # | Round | Holes Start | Holes End | Delta | Clean Chaps | Verified | Notes |
|---|-------|-------------|-----------|-------|-------------|----------|-------|
| 1 | R88   | 5           | 5         | 0     | 45          | 5239     | Graph chapters uncommented (Chap52/61-64), all external_body |
| 2 | R89   | 5           | 52        | +47   | 40          | 5269     | 8 graph files activated with holes; QuickSortMtEph proved |
| 3 | R90   | 52          | 35        | -17   | 43          | 5314     | Table ensures strengthened, 3 AdjTableGraph holes proved |
| 4 | R91   | 35          | 59        | +24   | 41          | 5320     | ClonePreservesWf trait, 20 AdjTableGraph ext_body removed (28→8); new MtPer holes exposed |
| 5 | R92   | 59          | 59        | 0     | 41          | 5367     | 10 Chap52 assumes proved, 4 ext_body removed; new holes offset gains |
| 6 | R93   | 59          | 59        | 0     | 41          | 5386     | 5 capacity assumes→requires, 2 clone assumes proved, 7 ensures added |
| 7 | R94   | 59          | 59        | 0     | 41          | 5386     | 2 Chap52 proofs reapplied, wf naming fixed in 8 graph files |
| 8 | R95   | 59          | 43        | -16   | 42          | 5383     | Clone-view gap fixed (-4), edge/postcondition proofs in StEph/StPer |
| 9 | R96   | 43          | 34        | -9    | 42          | 5385     | insert_wf added to Table, 8 MtPer assumes proved, result renames |
|10 | R97   | 34          | 34        | 0     | 42          | 5388     | insert_wf to OrderedTable, 6 stored-value-wf assumes removed, 3 insert_edge proved |
|11 | R98   | 34          | 34*       | 0*    | 42          | 5388     | *In progress — delete_wf merged, validate clean |

## Current State (R98 in progress)

- **Verified**: 5388 verified, 0 errors
- **RTT**: 3083 passed
- **PTT**: 157 passed
- **Holes**: 34 (28 real proof targets + 5 rwlock + 1 other)
- **Clean chapters**: 42 of 46
- **Holed chapters**: 4

## Trajectory

- R88-R89: Graph chapter activation — intentional hole increase as 8 new files brought online
- R90-R91: Infrastructure phase — ClonePreservesWf trait, Table ensures, temporarily increased holes
- R92-R94: Steady state — verified count climbing (+80) while holes held at 59 (proving offset new exposure)
- R95-R96: Breakthrough — holes 59→34 (-25), clone-view gap fixed, insert_wf pattern established
- R97-R98: Consolidation — holes stable at 34, insert_wf/delete_wf propagation, verified count peak 5388
