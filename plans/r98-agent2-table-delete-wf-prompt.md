# R98 Agent 2 — Add delete_wf to Table (preserves stored-value wf), STEP 20

## Objective

Table::delete doesn't ensure stored-value wf is preserved. Add `delete_wf`
(same pattern as `insert_wf` from R96) to TableStEph, TableStPer,
OrderedTableStPer, and OrderedTableMtPer.

## Why

After `Table::delete(key)`, the remaining entries are a subset of the old
entries. Their values were cloned during the rebuild. Without `delete_wf`,
callers can't prove the remaining stored values are still wf — same clone
gap as insert had before insert_wf.

## Design

```rust
fn delete_wf(&mut self, key: &K)
    where K: ClonePreservesView, V: ClonePreservesWf
    requires
        old(self).spec_tablesteph_wf(),
        forall|k: K::V| old(self)@.contains_key(k) ==>
            old(self).spec_stored_value(k).spec_wf(),
    ensures
        self@ =~= old(self)@.remove(key@),
        self.spec_tablesteph_wf(),
        forall|k: K::V| self@.contains_key(k) ==>
            self.spec_stored_value(k).spec_wf();
```

The proof: delete rebuilds entries by cloning all non-key entries. With
`V: ClonePreservesWf`, each cloned value preserves wf. The stored-value-wf
quantifier holds for all remaining keys.

## Files

1. `src/Chap42/TableStEph.rs` — add delete_wf
2. `src/Chap42/TableStPer.rs` — add delete_wf
3. `src/Chap43/OrderedTableStPer.rs` — add delete_wf (external_body with strong ensures)
4. `src/Chap43/OrderedTableMtPer.rs` — add delete_wf (external_body with strong ensures)

## Read first

- `src/Chap42/TableStEph.rs` — existing insert_wf as pattern
- `src/Chap42/TableStPer.rs` — same

## Isolation

```bash
scripts/validate.sh isolate Chap43
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT break existing callers.
- Mirror the insert_wf pattern exactly.
- External_body with strong ensures is fine for Ordered variants.

## STEP 20

## Report

Write `plans/agent2-r98-table-delete-wf-report.md`.
