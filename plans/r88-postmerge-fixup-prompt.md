# R88 Post-Merge Fixup — Compile Errors After Agent Merge

## Objective

Fix all remaining compile errors and warnings so `scripts/validate.sh` passes clean.

## Current Errors

Run `scripts/validate.sh` to see them. Known issues:

### 1. VertexMatchingStEph.rs (Chap61) — `*edge` move out of reference

Lines 69, 104, 153 have `*edge` where `edge` is `&Edge<V>` (from iterator). Edge is Copy
but Verus doesn't see it because Copy is derived outside verus!.

Fix: replace `*edge` with `Edge(edge.0.clone(), edge.1.clone())` — manually construct a
new Edge from cloned components. V has Clone (it's StT), and V::clone IS inside verus!
(via ClonePreservesView). Do NOT use `edge.clone()` — Edge's Clone impl is outside verus!.

### 2. VertexMatchingMtEph.rs (Chap61) — same pattern

Check for any `*edge` or `edge.clone()` that has the same problem. Apply same fix.

### 3. EdgeSetGraphMtPer.rs (Chap52) — `#[derive(Clone)]` triggers `ne` not supported

The `#[derive(Clone)]` on EdgeSetGraphMtPer struct (inside verus!) triggers Verus error
about `core::cmp::impls::ne` not supported. This was already removed — verify it's gone.
If Clone is needed anywhere, add a manual Clone impl outside verus! at the bottom of the file.

### 4. Trigger warnings

Fix any "automatically chose triggers" warnings by adding explicit `#[trigger]` annotations.

## Rules

- Do NOT modify Types.rs or any vstdplus files.
- Do NOT modify lib.rs.
- Do NOT add assume, accept, or external_body.
- Keep fixes minimal.
- Run `scripts/validate.sh` after each fix round.
- Show full validation output (tail -25 minimum).
- When clean, show the verified count and confirm 0 errors, 0 trigger warnings.

## Isolation

Use full validation (not isolate) since changes span Chap52 and Chap61.

## STEP 10

## Report

Write `plans/r88-postmerge-fixup-report.md`.
