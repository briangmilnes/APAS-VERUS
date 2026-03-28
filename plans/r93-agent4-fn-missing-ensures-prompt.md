# R93 Agent 4 — Add ensures to 7 fn_missing_ensures functions, STEP 15

## Objective

Veracity flagged 7 exec functions with no ensures clause. Add real ensures
to each — not `ensures true`, but the actual postcondition the function provides.

## The 7 Functions

| # | Chap | File | Function | Line |
|---|------|------|----------|------|
| 1 | 65 | KruskalStEph.rs | mst_weight | 404 |
| 2 | 65 | PrimStEph.rs | mst_weight | 421 |
| 3 | 66 | BoruvkaMtEph.rs | hash_coin_flips_mt | 190 |
| 4 | 66 | BoruvkaMtEph.rs | compute_remaining_mt | 259 |
| 5 | 66 | BoruvkaMtEph.rs | collect_mst_labels_mt | 319 |
| 6 | 66 | BoruvkaMtEph.rs | build_partition_map_mt | 379 |
| 7 | 66 | BoruvkaMtEph.rs | filter_tail_to_head_mt | 632 |

## How to add ensures

For each function:
1. Read the function body and understand what it computes
2. Read the function's callers to see what properties they need
3. Write an ensures clause that captures the function's postcondition
4. Verify it passes

Common patterns:
- **Length preservation**: `result@.len() == input@.len()`
- **wf propagation**: `result.spec_wf()` if the input is wf
- **Domain/content**: `result@ == some_spec_fn(input@)`
- **Bounds**: `result < n` for index-returning functions

For `mst_weight` (Kruskal/Prim): this computes the total weight of MST edges.
The ensures should relate the result to the sum of edge weights.

For BoruvkaMtEph helpers: these are parallel graph algorithm steps. Read the
StEph version (`BoruvkaStEph.rs`) if it exists — it may already have ensures
you can mirror.

## Read first

- `src/Chap65/KruskalStEph.rs` — mst_weight at line 404
- `src/Chap65/PrimStEph.rs` — mst_weight at line 421
- `src/Chap66/BoruvkaMtEph.rs` — 5 functions
- `src/Chap66/BoruvkaStEph.rs` — StEph version for reference ensures

## Isolation

```bash
scripts/validate.sh isolate Chap66
```

(Pulls in Chap65 transitively.)

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT add `ensures true` — that's worse than no ensures.
- Do NOT add tautological ensures (`ensures result@ .len() <= usize::MAX`).
- The ensures must be REAL postconditions that callers can use.
- If a function is too complex to specify (e.g., the ensures would be as
  long as the body), add what you can (wf, length) and note the gap.
- Do NOT weaken existing requires.
- Do NOT add assume or accept.

## STEP 15

## Report

Write `plans/agent4-r93-ensures-report.md`.
