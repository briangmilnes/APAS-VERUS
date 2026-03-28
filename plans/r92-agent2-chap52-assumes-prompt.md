# R92 Agent 2 — Prove Chap52 assumes, STEP 20

## Objective

You added ~36 assumes across AdjTableGraph×3 and EdgeSetGraphMtPer in R91.
Now prove as many as possible, replacing assumes with real proof assertions.

## Categories of assumes (from your R91 report)

1. **Clone gap** — `assume(stored_value_wf_after_clone)`. Now that Table
   insert/delete have `spec_stored_value` ensures (from agent1 R90+R91),
   you may be able to prove these through the stored_value chain.

2. **Verus ICE on Set<V::V>** — `assume(graph_closure)` because
   `assert forall` over `Map<V::V, Set<V::V>>` crashes Verus. These are
   BLOCKED — leave the assumes and note "blocked by Verus ICE" in comments.
   Do NOT try to write the quantifier that crashes.

3. **Capacity bounds** — `assume(@.len() + 1 < usize::MAX)`. These can be
   added to `requires` instead of assumed. Propagate to callers.

4. **Weak OrderedTableMtPer postconditions** — MtPer table find/insert have
   no view-level ensures. These are BLOCKED by the MtPer table API.

## Strategy

- **Category 1 (clone gap)**: Use `Table::spec_stored_value` ensures from
  insert/delete + `ClonePreservesWf` to prove the value is wf after mutations.
  The chain: table wf → stored values wf → insert rebuilds entries with
  clone → clone_wf preserves wf → stored values still wf.

- **Category 3 (capacity)**: Move to requires. Add
  `self@.dom().len() + 2 < usize::MAX as nat` to insert_edge requires.
  Check that callers can provide this.

- **Categories 2, 4**: Leave assumes, add `// blocked by Verus ICE` or
  `// blocked by OrderedTableMtPer weak ensures` comments.

## Priority

Work StEph first (cleanest Table API), then StPer, then MtPer. Each assume
you prove in StEph is a template for the other variants.

## Read first

- Your R91 changes in all 3 AdjTableGraph files
- `src/Chap42/TableStEph.rs` — spec_stored_value, find_ref, insert ensures
- `src/Chap42/TableStPer.rs` — same (agent1 R91 additions)
- `src/vstdplus/clone_view.rs` — ClonePreservesWf trait

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT add new assumes. Only remove existing ones.
- Do NOT trigger the Verus ICE — don't write `assert forall` over Map<V::V, Set<V::V>>.
- Converting an assume to a requires (category 3) is progress — it moves the
  obligation to callers instead of hiding it.
- Even proving 10 of 36 assumes is a good round.

## STEP 20

## Report

Write `plans/agent2-r92-assumes-report.md`.
