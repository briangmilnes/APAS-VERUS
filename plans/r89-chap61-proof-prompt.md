# R89 Agent 4 — Prove Chap61 Holes (EdgeContraction + VertexMatching), STEP 20

## Objective

Fix 12 verification errors across 3 files in Chap61. These are proof holes in
newly-uncommented files — the code compiles, but loop invariants and preconditions
fail verification.

## Files to fix

1. `src/Chap61/EdgeContractionStEph.rs` — 7 errors (4 loop invariant, 3 precondition)
2. `src/Chap61/VertexMatchingStEph.rs` — 2 errors (1 loop invariant, 1 precondition)
3. `src/Chap61/VertexMatchingMtEph.rs` — 3 errors (2 precondition, 1 loop invariant)

## CRITICAL: Read the working Mt versions first

- `src/Chap61/EdgeContractionMtEph.rs` — working Mt version of EdgeContraction
- `src/Chap61/VertexMatchingMtEph.rs` — has 3 errors but shows the parallel pattern

Also read:
- `src/Chap05/SetStEph.rs` — SetStEph API (insert, mem, iter, spec_setsteph_wf)
- `src/Chap06/UnDirGraphStEph.rs` — graph type, edges(), vertices()

## Error Locations

### EdgeContractionStEph.rs

Lines 70, 78, 89, 97 — four `for` loops over iterators. The invariants include
`valid_key_type_Edge::<V>()` and wf predicates but the iterator preconditions
likely need `spec_setsteph_wf()` or `spec_graphview_wf()` to be maintained.

The pattern: each loop iterates over `matching.iter()`, `graph.vertices().iter()`,
`vertex_to_block.iter()`, or `graph.edges().iter()`. The iterator requires the
collection to be well-formed. The invariant must maintain wf through each iteration.

Check what `SetStEph::iter()` requires — likely `self.spec_setsteph_wf()`. If so,
add that to the invariant. Same for `HashMapWithViewPlus::iter()`.

### VertexMatchingStEph.rs

Line 60 — `for edge in graph.edges().iter()` — same iterator wf pattern.
Line 91 — precondition failure in `parallel_matching_st`, likely `edge_vec` access.

### VertexMatchingMtEph.rs

Line 71 — precondition in `flip_coins_parallel`.
Line 221 — `for adj_edge in graph.edges().iter()` — iterator wf.

## Strategy

Most of these errors follow one pattern: the iterator's `requires` needs wf, but
the loop invariant doesn't maintain it. Fix:

1. Add `graph.spec_graphview_wf()` or equivalent to requires where missing
2. Add `self.spec_setsteph_wf()` to loop invariants for sets being iterated
3. For HashMapWithViewPlus iteration, check what `iter()` requires
4. Thread wf through loop bodies — if insert is called, prove wf is maintained

## lib.rs — do NOT modify

These files are already uncommented. No lib.rs changes needed.

## Isolation

```bash
scripts/validate.sh isolate Chap61
```

If Chap61 doesn't have an isolate feature, use:
```bash
scripts/validate.sh isolate Chap64
```
(Chap64 pulls in Chap61 transitively.)

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify EdgeContractionMtEph.rs — it's working.
- Do NOT modify files outside Chap61.
- Do NOT add assume or accept.
- Use external_body only as last resort.
- Do NOT weaken ensures clauses.
- The functions all have `ensures true` — that's fine for now. Focus on getting
  the bodies to verify, not strengthening specs.

## STEP 20

## Report

Write `plans/agent4-r89-chap61-report.md`.
