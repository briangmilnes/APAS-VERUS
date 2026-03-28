# R92 Agent 1 Report: Chap52 Fifty Flipping Assumes

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 52 | AdjTableGraphStEph.rs | 12 | 9 | -3 |
| 2 | 52 | AdjTableGraphStPer.rs | 14 | 9 | -5 |
| 3 | 52 | AdjTableGraphMtPer.rs | 24 | 22 | -2 |
| 4 | 52 | EdgeSetGraphMtPer.rs | 0 | 0 | 0 |
| | | **Total** | **50** | **40** | **-10** |

## Assumes Removed (10)

| # | Chap | File | Function | What was proved |
|---|------|------|----------|----------------|
| 1 | 52 | AdjTableGraphMtPer.rs | empty() | wf: type-level predicates from requires + vacuous graph closure on empty map |
| 2 | 52 | AdjTableGraphMtPer.rs | insert_vertex() | capacity: moved `dom.len() + 1 < usize::MAX` to trait requires |
| 3 | 52 | AdjTableGraphStPer.rs | out_neighbors() | Some branch: Table::find ensures dom.contains(u@) && adj@[u@] == ns@ |
| 4 | 52 | AdjTableGraphStPer.rs | out_neighbors() | None branch: !dom.contains(u@) from find + empty@ == Set::empty() |
| 5 | 52 | AdjTableGraphStEph.rs | from_table() | wf: strengthened requires with table wf + type predicates + stored-value wf |
| 6 | 52 | AdjTableGraphStPer.rs | from_table() | wf: same approach — strengthened requires, all wf conjuncts follow |
| 7 | 52 | AdjTableGraphStEph.rs | insert_edge() | dom.contains(u@): Table::insert ensures dom =~= old_dom.insert(u@) |
| 8 | 52 | AdjTableGraphStEph.rs | insert_edge() | dom.contains(v@): second insert or find_ref proves membership |
| 9 | 52 | AdjTableGraphStPer.rs | insert_edge() | dom.contains(u@): same chain as StEph |
| 10 | 52 | AdjTableGraphStPer.rs | insert_edge() | dom.contains(v@): same chain as StEph |

## Remaining Holes by Category

### Verus ICE blocked (wf after mutations): 6 assumes

All `assume(self/updated.spec_*_wf())` after insert_vertex, insert_edge, delete_edge
in StEph and StPer. Proving the wf quantifiers (stored-value-wf and graph closure)
requires `assert forall` over `Map<V::V, Set<V::V>>`, which causes sst_to_air crash.

### Clone-preserves-view gap: 4 assumes

- StEph/StPer insert_edge `adj[u@].contains(v@)`: proving the inserted set contains v@
  requires `v.clone()@ == v@` which isn't available generically (eq/clone workaround)
- StEph/StPer delete_edge postcondition: proving `!adj[u@].contains(v@)` after
  Set::remove requires connecting through Table::insert combine closure chain

### Capacity bounds (off-by-one): 2 assumes

StEph/StPer insert_edge: `ns@.len() + 1 < usize::MAX`. AVLTreeSetStEph wf gives
`ns@.len() < usize::MAX`, but insert needs `ns@.len() + 1 < usize::MAX` (strict).
The subset reasoning (`ns@ ⊆ domain`) was successfully proved via `assert forall`
firing graph closure triggers, but the off-by-one in the wf bound blocks the final step.

### OrderedTableMtPer weak ensures: 20 assumes

All remaining MtPer assumes. OrderedTableMtPer::find has NO ensures clause at all,
and insert/delete only ensure `dom().finite()`. Without functional specs on find/insert,
the MtPer graph code cannot prove any postconditions. Requires strengthening
OrderedTableMtPer API (Chap43 work).

### External body: 8

- num_edges (3 files): requires loop with domain iteration
- vertices (StEph, StPer): requires loop with domain iteration
- delete_vertex (3 files): requires loop + nested set operations

## Techniques Used

1. **Vacuous closure on empty map**: MtPer empty() — assert domain is empty, graph
   closure holds vacuously
2. **Strengthen requires**: from_table — added table wf, type-level predicates,
   and stored-value-wf quantifier to requires
3. **Move capacity to requires**: MtPer insert_vertex — capacity bound becomes
   caller obligation
4. **Table::find/insert ensures chain**: out_neighbors and insert_edge domain
   membership — Verus automatically proves from Table API postconditions
5. **assert forall with trigger firing**: Successfully proved `ns@ ⊆ domain` via
   `assert forall|w| ns@.contains(w) implies dom.contains(w)` by asserting
   `self.spec_adj().index(u@).contains(w)` to fire the graph closure trigger.
   (Used in capacity investigation; not needed in final code but technique is valid.)

## Verification

- Full validate: 5367 verified, 0 errors
- RTT: 3083 passed
- PTT: 157 passed
