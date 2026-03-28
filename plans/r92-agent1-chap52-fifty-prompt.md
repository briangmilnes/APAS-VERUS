# R92 Agent 1 — Prove the Fifty Flipping Chap52 Holes, STEP 20

## Objective

Chap52 has 50 holes across 4 files. Most are assumes from R91's ClonePreservesWf
work. Your job: prove as many assumes as possible, converting them to real proofs.
Every assume you kill is a real hole eliminated.

## The Files

| # | Chap | File | Holes | Type |
|---|------|------|-------|------|
| 1 | 52 | AdjTableGraphStEph.rs | ~14 | assumes + 3 external_body |
| 2 | 52 | AdjTableGraphStPer.rs | ~14 | assumes + 3 external_body |
| 3 | 52 | AdjTableGraphMtPer.rs | ~16 | assumes + 2 external_body |
| 4 | 52 | EdgeSetGraphMtPer.rs | ~6 | assumes |

## Categories of assumes (attack in this order)

### 1. Capacity bounds (EASY — move to requires)

```rust
assume(self.adj@.dom().len() + 1 < usize::MAX as nat);
```

These should be `requires` on the function, not assumes in the body. Move them
up. Check that callers can provide the bound (they almost certainly can — graph
sizes are bounded).

### 2. Clone wf preservation (MEDIUM — use ClonePreservesWf chain)

```rust
assume(updated.spec_adjtablegraphsteph_wf());  // after table insert/delete
```

Table insert/delete now have `ensures self.spec_tablesteph_wf()` and
`spec_stored_value` ensures (from R90/R91 agent1 work). Use the chain:
- Table wf → entries have no duplicate keys
- spec_stored_value → the value at each key is what was inserted
- ClonePreservesWf → cloned values preserve wf
- Therefore: all stored values in the new table are wf → graph wf holds

You need to prove the GRAPH CLOSURE part: "every neighbor is also a vertex."
This requires showing that insert/delete operations maintain the closure
invariant. For insert_vertex (adds empty neighbor set) — trivially preserves
closure. For insert_edge — adds v to u's neighbors AND ensures v is a vertex.
For delete_edge — removes an edge, can't break closure.

### 3. Verus ICE blocked (LEAVE AS ASSUME)

Any assume that would require `assert forall` over `Map<V::V, Set<V::V>>` is
blocked by Verus ICE. Add comment `// blocked by Verus ICE: sst_to_air crash on
Set<V::V> in proof quantifier` and move on. Do NOT try to write the quantifier.

### 4. Weak MtPer postconditions (LEAVE AS ASSUME for MtPer)

OrderedTableMtPer find/insert have weaker ensures than StEph Table. MtPer
assumes that reference these are blocked. Add comment `// blocked by
OrderedTableMtPer weak ensures` and move on.

## Strategy

Work AdjTableGraphStEph FIRST. It has the cleanest Table API (find_ref,
spec_stored_value, full insert/delete ensures). Every proof pattern you
establish in StEph can be copied to StPer and adapted for MtPer.

For each function:
1. Read the current assumes
2. Classify each (capacity / clone wf / ICE / weak API)
3. Prove what you can, comment what's blocked

## Read first

- `src/Chap52/AdjTableGraphStEph.rs` — primary target
- `src/Chap42/TableStEph.rs` — find_ref, spec_stored_value, insert/delete ensures
- `src/vstdplus/clone_view.rs` — ClonePreservesWf trait

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT add NEW assumes. Only remove or comment existing ones.
- Do NOT trigger the Verus ICE.
- Moving an assume to requires IS progress (category 1).
- Adding a `// blocked by` comment IS progress (categorizes the hole).
- Even 15 of 50 is a great round.
- Agents 2/3/4 are also working Chap52. You focus on the ASSUMES. They
  focus on the external_body iteration functions. Do NOT modify num_edges,
  vertices, or delete_vertex — those are theirs.

## STEP 20

## Report

Write `plans/agent1-r92-chap52-fifty-report.md`.
