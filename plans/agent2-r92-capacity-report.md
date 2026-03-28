# R92 Agent 2 Report: Capacity assumes to requires

## Summary

Moved capacity `assume` calls to `requires` clauses in Chap52 AdjTableGraph files,
converting hidden proof obligations into explicit caller contracts per the
capacity_bounds_standard.

## Results

| # | Chap | File | Function | Action | Technique |
|---|------|------|----------|--------|-----------|
| 1 | 52 | AdjTableGraphStEph.rs | insert_edge | assume removed | requires + graph closure subset proof |
| 2 | 52 | AdjTableGraphStPer.rs | insert_edge | assume removed | requires + graph closure subset proof |
| 3 | 52 | AdjTableGraphMtPer.rs | insert_vertex | assume removed | requires (direct) |
| 4 | 52 | AdjTableGraphMtPer.rs | insert_edge | 1 assume removed | requires covers first table insert |
| 5 | 52 | AdjTableGraphMtPer.rs | delete_edge | assume removed | requires (direct) |

**5 capacity assumes removed, 3 capacity assumes retained** in MtPer insert_edge.

## Retained assumes (MtPer insert_edge)

Three capacity assumes remain in `insert_edge` because `OrderedTableMtPer::insert` ensures
only `updated@.dom().finite()` — no domain size guarantee. After the first table insert,
domain size is lost, so subsequent insert preconditions cannot be derived from the function's
requires. These await stronger `OrderedTableMtPer::insert` ensures.

## Requires added

| # | Chap | File | Function | Requires added |
|---|------|------|----------|----------------|
| 1 | 52 | AdjTableGraphStEph.rs | insert_edge | `old(self).spec_adj().dom().len() + 1 < usize::MAX as nat` |
| 2 | 52 | AdjTableGraphStPer.rs | insert_edge | `self.spec_adj().dom().len() + 1 < usize::MAX as nat` |
| 3 | 52 | AdjTableGraphMtPer.rs | insert_vertex | `self.spec_adj().dom().len() + 1 < usize::MAX as nat` |
| 4 | 52 | AdjTableGraphMtPer.rs | insert_edge | `self.spec_adj().dom().len() + 2 < usize::MAX as nat` |
| 5 | 52 | AdjTableGraphMtPer.rs | delete_edge | `self.spec_adj().dom().len() + 1 < usize::MAX as nat` |

## Proof technique (StEph/StPer)

The neighbor set capacity bound derives from graph closure:
1. Graph closure: `adj[u@] ⊆ domain` (every neighbor is a vertex)
2. `vstd::set_lib::lemma_len_subset` + `lemma_entries_to_map_finite`: subset + finite => `ns@.len() <= dom.len()`
3. Requires `dom.len() + 1 < usize::MAX` => `ns@.len() + 1 <= dom.len() + 1 < usize::MAX`

## Holes

- Before: 44 (Chap52)
- After: 39 (Chap52)
- Delta: -5

## Verification

- Full validate: 5386 verified, 0 errors
- RTT: 3083 passed
