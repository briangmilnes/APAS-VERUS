# R91 Agent 1 — Table empty/insert/delete wf for StPer + MtEph, STEP 20

## Objective

Extend the Table wf ensures work from R90 (TableStEph only) to TableStPer and
TableMtEph. Add `ensures wf` to empty, and strengthen insert/delete ensures
with `spec_stored_value` preservation — same pattern you already proved for
TableStEph.

## Background

In R90 you added to TableStEph:
- `empty()` ensures `spec_tablesteph_wf()`
- `insert()` ensures `spec_tablesteph_wf()` + `spec_stored_value` for inserted key
- `delete()` ensures `spec_tablesteph_wf()`
- `find_ref()` returning `Option<&V>` with `*v == spec_stored_value(key@)`

TableStPer and TableMtEph need the same treatment. Agent2 is currently building
ClonePreservesWf for AdjTableGraph and will need these ensures on all 3 Table
variants.

## Files to modify

### 1. TableStPer (src/Chap42/TableStPer.rs)

- Add `ensures empty.spec_tablestper_wf()` to `empty()`
- Add `ensures self.spec_tablestper_wf()` to `insert()` and `delete()`
- Add `spec_stored_value` spec fn (same pattern as StEph — `choose` over entries)
- Add `find_ref` returning `Option<&V>` with stored_value ensures
- Strengthen insert ensures with stored_value for inserted key

### 2. TableMtEph (src/Chap42/TableMtEph.rs)

Same pattern. Note: TableMtEph wraps TableStEph in RwLock. The locked operations
delegate to the inner TableStEph, so the wf ensures should flow through from
the inner table's ensures.

Check if `spec_stored_value` makes sense for MtEph (the locked table). It may
need to be defined in terms of the inner table's stored value.

## Read first

- `src/Chap42/TableStEph.rs` — **your R90 work, the template**
- `src/Chap42/TableStPer.rs` — target 1
- `src/Chap42/TableMtEph.rs` — target 2 (has the subtract flaky fix you just did)

## Isolation

```bash
scripts/validate.sh isolate Chap42
```

Then check callers:
```bash
scripts/validate.sh isolate Chap43
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT break existing callers. Backwards compatibility is critical.
- Do NOT add assume or accept.
- Do NOT weaken ensures.
- Prioritize TableStPer over TableMtEph — StPer is simpler (no lock wrapper)
  and AdjTableGraphStPer has 10 holes waiting for it.
- If TableMtEph is too complex (lock delegation), do StPer only and report
  what blocks MtEph.

## STEP 20

## Report

Write `plans/agent1-r91-table-stper-mteph-report.md`.
