# R82b Agent 4 — Fix Chap52 EdgeSetGraph verification errors, STEP 15

## Objective

Fix 8 verification errors in `EdgeSetGraphStEph.rs` and `EdgeSetGraphStPer.rs` (Chap52).
These files compile but have pre-existing proof gaps around filter closures and wf preservation.

## Isolation

Use isolated validation during development:
```bash
scripts/validate.sh isolate Chap52
```
This includes Chap52 + all transitive deps.
Before pushing, run a full `scripts/validate.sh` to confirm.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## What to fix

The 8 errors (from agent3's R82 report) are:

**EdgeSetGraphStEph.rs:**
1. Line 187: precondition — filter closure/ghost spec_pred consistency
2. Lines 210, 212: loop invariant — depends on filter proof
3. Line 224: precondition — insert capacity in insert_edge nested calls
4. Lines 114, 119, 127, 132: postcondition — wf preservation through mutations

**EdgeSetGraphStPer.rs:**
1. Line 153: precondition — filter closure/ghost spec_pred consistency
2. Lines 178, 190: loop invariant — depends on filter proof
3. Lines 271, 272: assertion — delete_vertex filter proof (a != v → new_vertices.contains(a))
4. Lines 222, 262, 286: postcondition — wf preservation through mutations

### Root cause

The filter-closure consistency issue is the root: Verus needs proof that
`(edge.0 == u_clone) ↔ (edge_view.0 == u@)` which requires PartialEq/View consistency.
Read `src/standards/using_closures_standard.rs` for the closure verification patterns.
Read `src/standards/partial_eq_eq_clone_standard.rs` for the eq/clone workaround.

### Approach

1. Read both files fully. Understand the graph representation (vertex set + edge set).
2. For each filter closure, ensure the ghost spec predicate matches the exec predicate
   via the feq/clone bridge pattern.
3. For wf preservation, prove that mutations to vertex/edge sets maintain the invariant
   (every edge endpoint is a vertex).
4. For capacity bounds, propagate insert capacity from the outer function's requires.

## Important

- Do NOT add `assume` or `accept`.
- Do NOT weaken ensures clauses.
- Do NOT add `external_body` on algorithmic functions.
- Read the working graph files (AdjSeqGraphStEph, AdjMatrixGraphStEph) for patterns.

## STEP 15

## Validation

Before pushing: restore lib.rs, run full `scripts/validate.sh`, `scripts/rtt.sh`,
`scripts/ptt.sh`. Push to `agent4/ready`.

## Report

Write `plans/agent4-round82b-report.md`.
