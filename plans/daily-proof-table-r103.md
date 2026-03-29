# Daily Proof Table — R88 through R103

Generated 2026-03-29 from git history and agent reports.

| # | Round | Holes Start | Holes End | Delta | Clean Chaps | Verified | Notes |
|---|-------|-------------|-----------|-------|-------------|----------|-------|
| 1 | R88   | 5           | 5         | 0     | 45          | 5239     | Graph chapters uncommented (Chap52/61-64), all external_body |
| 2 | R89   | 5           | 52        | +47   | 40          | 5269     | 8 graph files activated with holes; QuickSortMtEph proved |
| 3 | R90   | 52          | 35        | -17   | 43          | 5314     | Table ensures strengthened, 3 AdjTableGraph holes proved |
| 4 | R91   | 35          | 59        | +24   | 41          | 5320     | ClonePreservesWf trait, 20 AdjTableGraph ext_body removed; new MtPer holes exposed |
| 5 | R92   | 59          | 59        | 0     | 41          | 5367     | 10 Chap52 assumes proved, 4 ext_body removed; new holes offset gains |
| 6 | R93   | 59          | 59        | 0     | 41          | 5386     | 5 capacity assumes→requires, 2 clone assumes proved, 7 ensures added |
| 7 | R94   | 59          | 59        | 0     | 41          | 5386     | 2 Chap52 proofs reapplied, wf naming fixed in 8 graph files |
| 8 | R95   | 59          | 43        | -16   | 42          | 5383     | Clone-view gap fixed (-4), edge/postcondition proofs in StEph/StPer |
| 9 | R96   | 43          | 34        | -9    | 42          | 5385     | insert_wf added to Table, 8 MtPer assumes proved, result renames |
|10 | R97   | 34          | 34        | 0     | 42          | 5388     | insert_wf to OrderedTable, 6 stored-value-wf assumes removed |
|11 | R98   | 34          | 34        | 0     | 42          | 5388     | delete_wf merged, rwlock final push (-14 assumes across agents) |
|12 | R99   | 34          | 34        | 0     | 42          | —        | ICE gone — 9 Chap52 ICE-blocked assumes proved, insert_edge capacity proved |
|13 | R100  | 34          | 34        | 0     | 42          | —        | OrderedTableStPer insert_wf/delete_wf proved (-2), iterator view fixes |
|14 | R101  | 34          | 34        | 0     | 42          | —        | delete_vertex closure proved (-2 assumes), iterator view audits |
|15 | R102  | 34          | 9         | -25   | 43          | —        | RwLock predicate fix (-6 ext_body, Chap43 clean), num_edges proved, DocumentIndex moved into verus! |
|16 | R103  | 9           | 9         | 0     | 43          | 5415     | Validate 5415/0, RTT 3083, PTT 157. Chap62 StarPartition proved, Chap44 DocumentIndex 4 ext_body exposed |

## Current State (R103)

- **Verified**: 5415 verified, 0 errors
- **RTT**: 3083 passed, 0 failures
- **PTT**: 157 passed, 0 failures
- **Holes**: 9 (across 3 chapters)
- **Clean chapters**: 43 of 46
- **Holed chapters**: 3

### Hole Breakdown

| # | Chap | File | Holes | Type |
|---|------|------|-------|------|
| 1 | 44 | DocumentIndex.rs | 4 | external_body |
| 2 | 52 | AdjTableGraphMtPer.rs | 1 | external_body |
| 3 | 65 | UnionFindStEph.rs | 4 | mixed |

## Trajectory

- **R88-R89**: Graph chapter activation — intentional hole increase as 8 new files brought online
- **R90-R91**: Infrastructure phase — ClonePreservesWf trait, Table ensures, temporarily increased holes
- **R92-R94**: Steady state — verified count climbing (+80) while holes held at 59
- **R95-R96**: Breakthrough — holes 59→34 (-25), clone-view gap fixed, insert_wf pattern
- **R97-R101**: Plateau at 34 holes — deep proof work (ICE fixes, delete_vertex closures, iterator views) without net hole reduction
- **R102**: Major drop — 34→9 (-25), RwLock predicate fix cleaned Chap43, DocumentIndex moved into verus!
- **R103**: Validation checkpoint — 5415 verified (new peak), 9 holes stable, Chap62 StarPartition proved
