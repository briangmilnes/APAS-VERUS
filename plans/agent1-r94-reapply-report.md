# R94 Agent 1 Report: Re-apply Chap52 from_table proofs

## Objective

Re-apply 10 Chap52 assumes proved in R92 that were lost in merge conflicts.

## Assessment

Of the 10 R92 proofs, **8 were already present** in the current codebase (survived
the merge or were equivalent to other agents' work). Only **2 were lost**: the
`from_table` wf proofs in StEph and StPer.

| # | Chap | File | Function | R92 Proof | Status |
|---|------|------|----------|-----------|--------|
| 1 | 52 | AdjTableGraphMtPer.rs | empty() | vacuous graph closure | Already present |
| 2 | 52 | AdjTableGraphMtPer.rs | insert_vertex() | capacity to requires | Already present |
| 3 | 52 | AdjTableGraphStPer.rs | out_neighbors() | Some branch | Already present |
| 4 | 52 | AdjTableGraphStPer.rs | out_neighbors() | None branch | Already present |
| 5 | 52 | AdjTableGraphStEph.rs | from_table() | wf from strengthened requires | **Re-applied** |
| 6 | 52 | AdjTableGraphStPer.rs | from_table() | wf from strengthened requires | **Re-applied** |
| 7 | 52 | AdjTableGraphStEph.rs | insert_edge() | dom.contains(u@) | Already present |
| 8 | 52 | AdjTableGraphStEph.rs | insert_edge() | dom.contains(v@) | Already present |
| 9 | 52 | AdjTableGraphStPer.rs | insert_edge() | dom.contains(u@) | Already present |
| 10 | 52 | AdjTableGraphStPer.rs | insert_edge() | dom.contains(v@) | Already present |

## Changes Made

### from_table (StEph + StPer)

Strengthened `from_table` trait requires with:
- `table.spec_tablesteph_wf()` / `table.spec_tablestper_wf()` — provides `spec_keys_no_dups`
- `vstd::laws_cmp::obeys_cmp_spec::<V>()` + `view_ord_consistent::<V>()` — type-level predicates
- Stored-value wf quantifier: `forall|k| table@.dom().contains(k) ==> table.spec_stored_value(k).spec_avltreesetsteph_wf()`

With these requires, all wf conjuncts follow from the requires + broadcast triggers.
No assume needed.

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 52 | AdjTableGraphStEph.rs | 9 | 8 | -1 |
| 2 | 52 | AdjTableGraphStPer.rs | 9 | 8 | -1 |
| 3 | 52 | AdjTableGraphMtPer.rs | 21 | 21 | 0 |
| | | **Total Chap52** | **39** | **37** | **-2** |

## Verification

- Full validate: 5386 verified, 0 errors
- RTT: 3083 passed
- PTT: 157 passed
