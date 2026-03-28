# R100 Agent 2 — Strengthen OrderedTableMtPer::map ensures, STEP 20

## Objective

`OrderedTableMtPer::map` currently only ensures `dom().finite()`. This blocks
2 delete_vertex holes in AdjTableGraphMtPer. Strengthen the ensures.

## What map needs to ensure

```rust
fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> (result: Self)
    ensures
        result@.dom() =~= self@.dom(),                     // domain preserved
        forall|k: K::V| self@.contains_key(k) ==>
            result@[k] == f_spec(self@[k]),                 // values mapped
        result.spec_orderedtablemtper_wf(),                 // wf preserved
```

The domain preservation is the critical one — AdjTableGraphMtPer::delete_vertex
needs to prove `!result.dom().contains(v@)` after `delete(v)` then `map(...)`.
With `map.dom() == delete.dom()` and `delete.dom() == old.dom().remove(v@)`,
the chain proves `!result.dom().contains(v@)`.

## Approach

Same pattern as insert_wf/delete_wf — add a new method `map_wf` or strengthen
the existing `map` with external_body and strong ensures. Since map is complex
(parallel tree traversal), external_body with strong ensures is the right call.

Check what the inner `OrderedTableStPer::map` ensures. If it's also weak,
strengthen that first.

## Read first

- `src/Chap43/OrderedTableMtPer.rs` — map signature
- `src/Chap43/OrderedTableStPer.rs` — inner map (may need strengthening too)
- `src/Chap52/AdjTableGraphMtPer.rs` — delete_vertex caller (lines 362-363)

## Isolation

```bash
scripts/validate.sh isolate Chap43
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT break existing callers.
- External_body with strong ensures is fine.
- Even just `result@.dom() =~= self@.dom()` unblocks 1 of the 2 delete_vertex holes.

## STEP 20

## Report

Write `plans/agent2-r100-map-report.md`.
