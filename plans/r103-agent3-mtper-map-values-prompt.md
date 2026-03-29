# R103 Agent 3 — Strengthen MtPer map value ensures + prove delete_vertex, STEP 20

## Objective

AdjTableGraphMtPer::delete_vertex has 1 remaining assume (graph wf after map).
Blocked because OrderedTableMtPer::map only ensures domain preservation, not
value properties.

## The fix chain

1. Strengthen OrderedTableMtPer::map ensures to include value correspondence
2. Use that to prove delete_vertex graph wf

## Step 1: Map value ensures

Current map ensures: `mapped@.dom() =~= self@.dom(), mapped.spec_wf()`

Need to add: `forall|k: K::V| self@.contains_key(k) ==> mapped@[k] == f_spec(k, self@[k])`

But Verus closures don't have spec-level `f_spec`. Alternative approaches:

### Option A: Value-level ensures via ghost

```rust
fn map<F>(&self, f: F, Ghost(f_spec): Ghost<spec_fn(K::V, V::V) -> V::V>) -> (result: Self)
    requires
        forall|k: &K, v: &V| f.requires((k, v)),
        forall|k: &K, v: &V, r: V| f.ensures((k, v), r) ==> r@ == f_spec(k@, v@),
    ensures
        result@.dom() =~= self@.dom(),
        forall|k: K::V| self@.contains_key(k) ==> result@[k] == f_spec(k, self@[k]),
```

### Option B: Specific delete_vertex helper

Skip generic map strengthening. Write a specific `delete_key_from_all_values`
function on OrderedTableMtPer that removes a key from every stored Set value.
This avoids the generic closure problem.

### Option C: external_body with strong ensures

Same pattern as insert_wf/delete_wf. Add a `map_wf` with external_body and
strong ensures. Pragmatic.

## Step 2: Prove delete_vertex

With map ensures, the delete_vertex graph closure follows:
- `delete(v)` removes v from domain
- `map(|k, ns| ns.delete(v))` removes v from all neighbor sets
- Graph closure: every neighbor is a vertex → holds because v is gone from both

## Read first

- `src/Chap52/AdjTableGraphMtPer.rs` — delete_vertex (line 355)
- `src/Chap43/OrderedTableMtPer.rs` — map signature and ensures
- `src/Chap43/OrderedTableStPer.rs` — map in StPer for reference

## Isolation

```bash
scripts/validate.sh isolate Chap43   # for map changes
scripts/validate.sh isolate Chap52   # for delete_vertex
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Option C (external_body strong ensures) is acceptable if generic proves too hard.
- Do NOT add assume or accept.
- This is the last Chap52 hole.

## STEP 20

## Report

Write `plans/agent3-r103-map-values-report.md`.
