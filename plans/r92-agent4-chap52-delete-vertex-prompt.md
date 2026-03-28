# R92 Agent 4 — Prove AdjTableGraph delete_vertex, STEP 20

## Objective

Remove external_body from `delete_vertex` in AdjTableGraph (3 files). This is
the hardest of the remaining Chap52 iteration holes — it requires iterating
the domain AND modifying neighbor sets at each step.

## The function

```rust
fn delete_vertex(&self, v: &V) -> (updated: Self) {
    // 1. Remove v from the table (delete key v)
    // 2. For each remaining key k, remove v from neighbors[k]
    // Result: no key v, no value contains v
}
```

The I64/integer graph versions don't have this function (they use different
graph representations). Look at EdgeSetGraphStPer for a simpler delete_vertex
pattern — it filters edges rather than iterating and modifying.

## Strategy

For AdjTableGraphStEph:
1. `self.adj.delete(&v)` — removes v as a key
2. Iterate remaining domain, for each key k:
   - `find_ref(&k)` to get neighbor set reference
   - Build new neighbor set without v
   - `insert(k, new_neighbors)` into new table

The loop invariant must track:
- Keys processed so far have v removed from their neighbor sets
- Keys not yet processed retain original neighbor sets
- Graph closure is maintained (no vertex refers to v)

### The ICE workaround

You CANNOT quantify over `Map<V::V, Set<V::V>>` in proof bodies. Instead:
- Work with the domain sequence from `domain().to_seq()`
- Use `find_ref` for individual lookups
- Track invariants per-index, not per-map-entry
- Use assumes for the final graph-closure postcondition if the quantifier
  would trigger the ICE

## Read first

- `src/Chap52/AdjTableGraphStEph.rs` — your target
- `src/Chap52/EdgeSetGraphStPer.rs` — simpler delete_vertex for pattern reference
- `src/Chap42/TableStEph.rs` — delete(), find_ref(), insert()

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Start with StEph only.
- Do NOT trigger the Verus ICE — no `assert forall` over Map<V::V, Set<V::V>>.
- You MAY use assumes for the graph-closure postcondition if it requires the
  ICE-triggering quantifier. Document with `// blocked by Verus ICE`.
- Do NOT add accept.
- Even proving delete_vertex with an assume for graph-closure is progress over
  external_body — it verifies the algorithmic logic even if the closure property
  is assumed.

## STEP 20

## Report

Write `plans/agent4-r92-delete-vertex-report.md`.
