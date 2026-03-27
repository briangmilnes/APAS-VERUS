# R89 Agent 2 — Prove EdgeSetGraphMtPer holes (Chap52), STEP 20

## Objective

Remove external_body from 6 functions in `src/Chap52/EdgeSetGraphMtPer.rs`.

## The 6 Holes

| # | Function | Root Cause |
|---|----------|-----------|
| 1 | out_neighbors | AVLTreeSetMtPer::filter needs Clone on closure — Verus can't verify |
| 2 | out_degree | Downstream of out_neighbors (calls .size() on unproven-wf result) |
| 3 | delete_vertex | Same filter/Clone issue as out_neighbors |
| 4 | insert_edge | Clone-view bridging through chained .insert().insert() |
| 5 | delete_edge | Pair view bridging through delete |
| 6 | default | Calls empty() which needs cmp/ord preconditions |

## Strategy

### Easy wins first: insert_edge, delete_edge, default (holes 4-6)

**insert_edge**: The chain `self.vertices.insert(u.clone()).insert(v.clone())` fails
because Verus can't prove wf propagates through the second insert. Fix: split into
two steps with intermediate let-binding. Use `clone_view()` (from ClonePreservesView)
instead of `clone()` to get view-preservation ensures. Assert wf between steps.

**delete_edge**: `self.edges.delete(&Pair(u.clone(), v.clone()))` fails because
Verus can't bridge `Pair(u.clone(), v.clone())@ == (u@, v@)`. Fix: use `clone_view()`
and assert the Pair view equality explicitly.

**default**: Just calls `Self::empty()`. May need the cmp/ord preconditions in
`default`'s requires, or it may just work once you add them.

### Hard: out_neighbors, delete_vertex (holes 1, 3)

These use `AVLTreeSetMtPer::filter(pred)` which needs `Clone` on the closure.
Verus can't verify closure Clone.

**Workaround: Replace filter with iterate + insert.** Instead of:
```rust
self.edges.filter(|edge| edge.0 == u)
```
Do:
```rust
let mut result = AVLTreeSetMtPer::empty();
for edge in self.edges.iter() {
    if edge.0 == u {
        result = result.insert(edge.1.clone());
    }
}
result
```

Read how `EdgeSetGraphStPer.rs` implements `out_neighbors` — it uses filter on
the StPer set. Your iterate+insert approach is the MtPer equivalent.

### out_degree (hole 2)

Falls automatically once out_neighbors ensures `neighbors.spec_wf()`.

## Read first

- `src/Chap52/EdgeSetGraphMtPer.rs` — your file
- `src/Chap52/EdgeSetGraphStPer.rs` — working StPer version (0 holes, uses filter)
- `src/Chap41/AVLTreeSetMtPer.rs` — MtPer set API: insert, iter, filter, size
- `src/vstdplus/clone_view.rs` — ClonePreservesView trait and clone_view()

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify EdgeSetGraphStPer.rs or any file outside Chap52.
- Do NOT add assume or accept.
- If the iterate+insert workaround for filter is too complex to prove (loop
  invariant threading set equality), use external_body on just those functions
  and report what blocks the proof.
- Prioritize insert_edge, delete_edge, default — those are the tractable wins.

## STEP 20

## Report

Write `plans/agent2-r89-edgeset-report.md`.
