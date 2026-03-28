# R96 Agent 1 — Prove 9 MtPer rwlock assumes using OrderedTableMtPer ensures, STEP 20

## Objective

AdjTableGraphMtPer has 9 assumes classified as `rwlock:predicate`. These
were added because OrderedTableMtPer had no ensures on find/insert/delete.
In R95 agent1 strengthened those ensures. Now prove the assumes.

## The 9 assumes (all in AdjTableGraphMtPer.rs)

Pattern 1 — neighbor set wf after find (4 assumes):
```rust
assume(neighbors.spec_avltreesetmtper_wf());
```
**Now provable**: OrderedTableMtPer::find ensures `Some(v) => self@[key@] == v@`.
If graph wf includes stored-value wf, find returns a wf set.

Pattern 2 — graph wf after mutation (5 assumes):
```rust
assume(updated.spec_adjtablegraphmtper_wf());
```
**Partially provable**: OrderedTableMtPer::insert now ensures domain and content.
Graph closure may still need ICE-blocked quantifier — use assume with comment if so.

## Strategy

For Pattern 1: OrderedTableMtPer::find now has `Some(v) => self@.contains_key(key@) && self@[key@] == v@`. If graph wf includes a stored-value-wf quantifier (or you add one), find result wf follows.

For Pattern 2: insert/delete now ensure domain and content. Graph closure (`forall|u,v| adj[u].contains(v) ==> dom.contains(v)`) may trigger the Verus ICE if you quantify over `Map<V::V, Set<V::V>>`. If so, leave the assume with `// blocked by Verus ICE` comment.

## Read first

- `src/Chap52/AdjTableGraphMtPer.rs` — your file (find the 9 assumes)
- `src/Chap43/OrderedTableMtPer.rs` — the strengthened ensures (R95 agent1 work)
- `src/Chap52/AdjTableGraphStEph.rs` — StEph version for proof patterns

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify OrderedTableMtPer.rs or any Chap43 file.
- Do NOT add new assumes. Only remove or re-classify existing ones.
- If ICE blocks a proof, change the veracity classification comment from
  `[rwlock:predicate]` to `[algorithmic]` with `// blocked by Verus ICE`.

## STEP 20

## Report

Write `plans/agent1-r96-mtper-rwlock-report.md`.
