# R47 Agent 1: Graph Chapter Mop-up (Chap59 + Chap62 + Chap65)

## Assignment

Close remaining holes in three graph chapters. You cleaned Chap61 and Chap64
in R46 — same patterns apply here. Agent 2 has Chap39, Agent 3 has Chap47,
Agent 4 has Chap38. No conflicts.

## Baseline

43 holes total. 4413 verified. Your chapters: Chap59 (2), Chap62 (2), Chap65 (2) = 6 holes.

## REQUIRED READING

**Before writing any code**, read these standards:

1. `src/standards/arc_usage_standard.rs` — When Arc is needed vs antipattern
2. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — Plain RwLock pattern for Mt
3. `src/standards/hfscheduler_standard.rs` — HFScheduler fork-join pattern
4. `src/standards/using_closures_standard.rs` — Closure requires/ensures propagation

Do NOT use Arc<RwLock> as a struct field. Use plain RwLock with ghost shadow.

## Current Holes

Run `scripts/holes.sh src/Chap59/`, `scripts/holes.sh src/Chap62/`,
`scripts/holes.sh src/Chap65/` to verify these.

### Chap59 — Johnson's APSP (2 holes, 2 warnings)

| # | Chap | File | Function | Type | Notes |
|---|------|------|----------|------|-------|
| 1 | 59 | JohnsonMtEphI64.rs | parallel_dijkstra_all | external_body | Parallel loop over vertices |
| 2 | 59 | JohnsonStEphI64.rs | reweight_graph | assume | Edge count lemma needed |

Warnings: `adjust_distance` and `reweight_edge` missing requires (fn_missing_requires).
Fix those too — add real preconditions, not `requires true`.

### Chap62 — Star Contraction (2 holes)

| # | Chap | File | Function | Type | Notes |
|---|------|------|----------|------|-------|
| 3 | 62 | StarContractionStEph.rs | star_contract | external_body | Generic contraction |
| 4 | 62 | StarContractionMtEph.rs | star_contract_mt | external_body | Parallel contraction |

### Chap65 — MST: Kruskal + Prim (2 holes, 1 warning)

| # | Chap | File | Function | Type | Notes |
|---|------|------|----------|------|-------|
| 5 | 65 | KruskalStEph.rs | sort_edges_by_weight | external_body | Edge sorting |
| 6 | 65 | PrimStEph.rs | prim_mst | external_body | Full Prim algorithm |

Warning: `pq_entry_new` missing requires (fn_missing_requires). Fix with real precondition.

## Strategy

### Chap59: reweight_graph assume

The assume is `assume(result@.A.len() <= graph@.A.len())`. This needs a graph
partition lemma: sum of out-degrees equals edge count. Check if Chap05/Chap52
already has this lemma. If not, write one — it's a straightforward induction
over vertices.

### Chap59: parallel_dijkstra_all external_body

This runs Dijkstra from each vertex in parallel. The sequential version
(JohnsonStEphI64) already works. The Mt version needs fork-join over vertices.
Use HFScheduler or tabulate pattern. Read the St version first to understand
the spec.

### Chap62: star_contract / star_contract_mt

These are generic higher-order functions (take `base` and `expand` closures).
The body likely involves graph traversal + partition. Read the function
signatures carefully — the closure specs must propagate through requires/ensures.
Read `using_closures_standard.rs` for the pattern.

### Chap65: sort_edges_by_weight

Sorting a Vec of edges by weight. Check if vstd has sort specs. If not, this
may need to stay external_body with strengthened ensures (sorted output,
multiset preservation). A strong spec external_body is acceptable for sort —
Verus cannot verify comparison-based sorting of arbitrary types.

### Chap65: prim_mst

Full Prim's algorithm. This is a graph algorithm with a priority queue loop.
Read the body, understand what the external_body hides. If it's the full
algorithm, you may need to prove the loop invariant. If it's just one step,
factor it.

## What NOT to do
- Do NOT add `#[cfg(not(verus_keep_ghost))]` to anything. Forbidden on fn/impl/type.
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses.
- Do NOT use Arc<RwLock> as a struct field — read arc_usage_standard.rs.
- Do NOT sequentialize parallel implementations.
- Do NOT skip reading the standards listed above.

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes.
Run `scripts/holes.sh src/Chap59/` + `src/Chap62/` + `src/Chap65/`.
Write your report to `plans/agent1-round47-report.md`.
