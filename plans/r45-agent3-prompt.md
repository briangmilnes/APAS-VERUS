# R45 Agent 3: Chap66 + Chap65 (11 holes)

## Assignment

Prove the remaining Boruvka parallel MST functions (Chap66, 9 holes) and the
remaining Kruskal/Prim holes (Chap65, 2 holes). Last round you proved
kruskal_mst and boruvka_mst_mt_with_seed (-2 gross, -1 net after sort helper).

## Baseline

99 holes total. 4388 verified. Your chapters: Chap66 (9), Chap65 (2).

## CRITICAL: Read plans/parapair-is-not-a-blocker.md FIRST

Last round you claimed the remaining Boruvka holes are "ParaPair-dependent" and
structurally blocked. **This is wrong.** ParaPair is fully verifiable. Chap06
(LabDirGraphMtEph) and Chap36 (QuickSortMtEph) both verify through ParaPair with
zero holes. Read `plans/parapair-is-not-a-blocker.md` for the complete pattern:

1. Capture ghost state before closures: `let ghost left_view = left@;`
2. Write named closures with explicit ensures
3. Call `ParaPair!(f1, f2)` or `crate::ParaPair!(f1, f2)`
4. Verus propagates ensures to pair results

**Do NOT classify ParaPair as a blocker.** If a function uses ParaPair and is
external_body, the fix is to write named closures with ensures — not to give up.

## Target Holes

### Chap66 — BoruvkaMtEph.rs (9 holes)

| # | Chap | File | Function | Line | Type | Notes |
|---|------|------|----------|------|------|-------|
| 1 | 66 | BoruvkaMtEph.rs | hash_coin_flips_mt | 138 | external_body | ParaPair divide-and-conquer |
| 2 | 66 | BoruvkaMtEph.rs | compute_remaining_mt | 173 | external_body | Sequential loop |
| 3 | 66 | BoruvkaMtEph.rs | collect_mst_labels_mt | 211 | external_body | Sequential loop |
| 4 | 66 | BoruvkaMtEph.rs | build_partition_map_mt | 249 | external_body | Sequential loop |
| 5 | 66 | BoruvkaMtEph.rs | vertex_bridges_mt | 294 | external_body | ParaPair divide-and-conquer |
| 6 | 66 | BoruvkaMtEph.rs | bridge_star_partition_mt | 350 | external_body | Complex orchestrator |
| 7 | 66 | BoruvkaMtEph.rs | filter_tail_to_head_mt | 389 | external_body | Sequential loop |
| 8 | 66 | BoruvkaMtEph.rs | boruvka_mst_mt | 445 | external_body | Main algorithm loop |
| 9 | 66 | BoruvkaMtEph.rs | reroute_edges_mt | 504 | external_body | ParaPair divide-and-conquer |

Also fix: `boruvka_mst_mt_with_seed` fn_missing_ensures (line 550).

**Triage by difficulty:**

- **Easy** (sequential loops): `compute_remaining_mt` (#2), `collect_mst_labels_mt` (#3),
  `build_partition_map_mt` (#4), `filter_tail_to_head_mt` (#7). These iterate over
  collections — use the iterator proof pattern with loop invariants.

- **Medium** (ParaPair divide-and-conquer): `hash_coin_flips_mt` (#1),
  `vertex_bridges_mt` (#5), `reroute_edges_mt` (#9). These split input, process halves
  in parallel via ParaPair, combine results. Write named closures with ghost captures
  and explicit ensures. The pattern is identical to QuickSort (Chap36).

- **Hard** (orchestrators): `bridge_star_partition_mt` (#6), `boruvka_mst_mt` (#8).
  These call the helpers above. Once helpers have ensures, these may become provable.
  Start with easy+medium, then attempt hard.

### Chap65 — KruskalStEph + PrimStEph (2 holes)

| # | Chap | File | Function | Line | Type | Notes |
|---|------|------|----------|------|------|-------|
| 1 | 65 | KruskalStEph.rs | sort_edges_by_weight | 58 | external_body | Sort closure helper |
| 2 | 65 | PrimStEph.rs | prim_mst | 95 | external_body | Priority queue loop |

Also fix: `pq_entry_new` fn_missing_requires (line 72) — add real requires if one
exists, otherwise report it.

**Strategy:**
- `sort_edges_by_weight`: This wraps a sort closure. Check if the sort can be proved
  via Verus's sort lemmas in vstd, or if the closure ensures are sufficient.
- `prim_mst`: Uses BinaryHeapPQ in a while loop. Try removing external_body and adding
  loop invariants. If PQ operations lack ensures, keep external_body and report.

## What NOT to do
- Do NOT add `#[cfg(not(verus_keep_ghost))]` to anything. Forbidden on fn/impl/type.
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses.
- Do NOT sequentialize parallel (Mt) implementations — keep ParaPair! calls.
- Do NOT claim ParaPair is a blocker. Read plans/parapair-is-not-a-blocker.md.

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes.
Run `scripts/holes.sh src/Chap66/ src/Chap65/`.
Write your report to `plans/agent3-round45-report.md`.
