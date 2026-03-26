# R82b Agent 3 — Fix Chap52 AdjTableGraph: switch from OrderedTable to Table, STEP 20

## Objective

Fix `AdjTableGraphStEph.rs` and `AdjTableGraphStPer.rs` in Chap52. These use
`OrderedTable<V, AVLTreeSet<V>>` which requires `V: Ord`. But AVLTreeSet does
NOT implement Ord (sets don't have a total order — that's not textbook).

The fix: switch from `OrderedTable` to `Table` (unordered table from Chap42).
Adjacency lists don't need key-ordered lookup.

## Isolation

Use isolated validation during development:
```bash
scripts/validate.sh isolate Chap52
```
This includes Chap52 + all transitive deps.
Before pushing, run a full `scripts/validate.sh` to confirm.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## What to fix

### 1. Uncomment both files in lib.rs

```rust
    pub mod AdjTableGraphStEph;
    pub mod AdjTableGraphStPer;
```

### 2. Replace OrderedTable with Table

In both files:
- Change `use crate::Chap43::OrderedTableStEph` → `use crate::Chap42::TableStEph`
  (and similarly for StPer)
- Change the struct's table field type from `OrderedTable<V, AVLTreeSet<V>>`
  to `Table<V, AVLTreeSet<V>>`
- Update the wf predicate to use `spec_tablesteph_wf` instead of `spec_orderedtablesteph_wf`
- Update all method calls: Table has `find`, `insert`, `delete`, `size`, `collect`
  with the same signatures but different trait names

Read the Table API:
- `src/Chap42/TableStEph.rs` — unordered table trait and impl
- `src/Chap42/TableStPer.rs` — persistent variant

Read the working AdjSeq/AdjMatrix files in Chap52 for the current graph patterns.

### 3. Remove V: Ord bounds where no longer needed

With Table instead of OrderedTable, the value type `AVLTreeSet<V>` no longer
needs `Ord`. Remove `Ord` bounds on the trait and impl where they were only
there to satisfy OrderedTable.

### 4. Fix verification errors

After the Table switch, fix any remaining verification errors. The graph operations
(add_vertex, add_edge, delete_vertex, neighbors, etc.) should have the same logical
specs — only the backing store changed.

## Important

- Do NOT add Ord impls to AVLTreeSet. That's the wrong fix.
- Do NOT add `assume` or `accept`.
- Do NOT modify files in other chapters (Chap41, Chap42, Chap43).
- If the full fix exceeds 20 steps, comment the files back out with an updated
  BROKEN comment and report what remains.

## STEP 20

## Validation

Before pushing: restore lib.rs, run full `scripts/validate.sh`, `scripts/rtt.sh`,
`scripts/ptt.sh`. Push to `agent3/ready`.

## Report

Write `plans/agent3-round82b-report.md`.
