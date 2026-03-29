# R102 Agent 4 — Prove MtPer num_edges (2 assumes), STEP 20

## Objective

AdjTableGraphMtPer::num_edges has 2 assumes:
1. Line 228: `assume(count + neighbors@.len() <= self.spec_num_edges())` — overflow
2. Line 240: `assume(count == self.spec_num_edges())` — sum correctness

With the R102 RwLock predicate fix, OrderedTableMtPer::find now has real ensures:
`Some(v) => self@.contains_key(key@) && self@[key@] == v@`. The loop can now
access neighbor set values through find.

## The function

num_edges iterates `domain().to_seq()`, calls `find` per vertex, sums `size()`.
The spec `spec_num_edges` uses `spec_sum_adj_sizes(self.spec_adj())` which
recursively decomposes the map.

## Proof strategy

Agent3 R92 proved num_edges for StEph using:
1. `spec_sum_entry_sizes(entries, n)` — sequential partial sum over entries
2. `lemma_sum_adj_remove(m, k)` — extract any key from recursive map sum
3. `lemma_sum_entry_sizes_eq(entries, n)` — connect sequential to recursive
4. `lemma_sum_entry_sizes_monotone(entries, i, j)` — overflow bounds

The MtPer version needs the same approach but using OrderedTableMtPer::find
instead of direct entry access. The key insight: `find(k)` gives `Some(v)`
where `self@[k] == v@`, and `v.size()` gives `v@.len()`. So the loop sum
accumulates `self@[domain[i]].len()` which equals `spec_sum_adj_sizes`.

You may need to write the inductive lemma connecting the loop sum to the
recursive map sum. Or you may be able to adapt StEph's lemma infrastructure.

## Read first

- `src/Chap52/AdjTableGraphMtPer.rs` — num_edges (around line 215)
- `src/Chap52/AdjTableGraphStEph.rs` — proved num_edges + helper lemmas (your template)
- `src/Chap43/OrderedTableMtPer.rs` — find ensures (just fixed in R102)

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify Chap43 files.
- Do NOT add assume or accept.
- The StEph proof is your template — adapt the lemma chain for MtPer's
  OrderedTableMtPer API.
- Even proving 1 of the 2 assumes is progress.

## STEP 20

## Report

Write `plans/agent4-r102-numedges-report.md`.
