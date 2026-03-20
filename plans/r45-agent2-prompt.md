# R45 Agent 2: Chap63 + Chap64 + Chap59 (12 holes)

## Assignment

Continue proving graph algorithm chapters. You proved 7 holes across Chap63+59
last round. Now finish them and add Chap64.

## Baseline

99 holes total. 4388 verified. Your chapters: Chap63 (6), Chap64 (4), Chap59 (2).

## Target Holes

### Chap63 — Connectivity (6 holes)

| # | Chap | File | Function | Line | Type |
|---|------|------|----------|------|------|
| 1 | 63 | ConnectivityMtEph.rs | label_components_mt | 87 | external_body |
| 2 | 63 | ConnectivityMtEph.rs | count_components_mt | 115 | external_body |
| 3 | 63 | ConnectivityMtEph.rs | connected_components_mt | 144 | external_body |
| 4 | 63 | ConnectivityMtEph.rs | is_connected_mt | 162 | external_body |
| 5 | 63 | ConnectivityStEph.rs | label_components | 79 | external_body |
| 6 | 63 | ConnectivityStEph.rs | count_components | 107 | external_body |

Also fix warnings:
- ConnectivityMtEph.rs: `count_components_hof` fn_missing_ensures (line 246)
- ConnectivityMtEph.rs: `connected_components_hof` fn_missing_ensures (line 262)
- ConnectivityStEph.rs: `count_components_hof` fn_missing_ensures (line 190)
- ConnectivityStEph.rs: `connected_components_hof` fn_missing_ensures (line 208)

**Strategy:** Start with StEph (simpler). `label_components` and `count_components`
use UnionFind — check what ensures UnionFindStEph provides. The Mt versions likely
delegate to St or use ParaPair. If Mt versions use ParaPair, use the named-closure
pattern (see plans/parapair-is-not-a-blocker.md).

### Chap64 — SpanTree + TSPApprox (4 holes)

| # | Chap | File | Function | Line | Type |
|---|------|------|----------|------|------|
| 1 | 64 | SpanTreeStEph.rs | spanning_tree | 51 | external_body |
| 2 | 64 | SpanTreeMtEph.rs | spanning_tree_mt | 72 | external_body |
| 3 | 64 | TSPApproxStEph.rs | euler_tour | 89 | external_body |
| 4 | 64 | TSPApproxStEph.rs | shortcut_tour | 106 | external_body |

Also fix: TSPApproxStEph.rs `approx_metric_tsp` fn_missing_ensures (line 295).

### Chap59 — Johnson APSP (2 holes)

| # | Chap | File | Function | Line | Type |
|---|------|------|----------|------|------|
| 1 | 59 | JohnsonMtEphI64.rs | johnson_apsp_mt | 88 | external_body |
| 2 | 59 | JohnsonStEphI64.rs | johnson_apsp | 329 | assume (algorithmic) |

Also fix warnings:
- JohnsonStEphI64.rs: `adjust_distance` fn_missing_requires (line 73)
- JohnsonStEphI64.rs: `reweight_edge` fn_missing_requires (line 89)

**Strategy:** The Mt version likely delegates to St. The St assume is about result
graph size — may need a lemma about BellmanFord output preserving vertex count.

## What NOT to do
- Do NOT add `#[cfg(not(verus_keep_ghost))]` to anything. Forbidden on fn/impl/type.
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses.
- Do NOT sequentialize parallel (Mt) implementations.

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes.
Run `scripts/holes.sh src/Chap63/ src/Chap64/ src/Chap59/`.
Write your report to `plans/agent2-round45-report.md`.
