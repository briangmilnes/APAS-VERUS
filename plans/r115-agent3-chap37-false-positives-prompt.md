# R115 Agent 3 — Chap37 warning analysis + veracity false positive documentation. AFK. PBOGH.

## Objective

Two tasks:
1. Reduce Chap37 compare-par-mut warnings where possible (101 warnings).
2. Carefully document every remaining false positive for a veracity prompt.

## Current state

Agent 2 (R112) reduced Chap37 from 145 to ~101. The remaining warnings are:
- wf subsumption (~40): MtEph `spec_bst*mteph_wf()` implies `tree_is_bst()` etc.
  but the tool can't match wf against individual StEph clauses.
- spec_root vs @ (~15): StEph uses `self.spec_root().foo()`, MtEph uses `self@.foo()`.
- ghost_root no counterpart (5): structural MtEph field.
- missing fns (5): functions in StEph not in MtEph.
- structural differences (~10): `insert(self) -> Self` vs `insert(&mut self)`.

## Task 1: Fix what you can

For each warning, determine if it's fixable or a false positive:
- **Fixable**: MtEph trait is missing a requires/ensures that could be added.
  Add it, validate with `scripts/validate.sh isolate Chap37`.
- **False positive**: The warning is wrong — the specs ARE equivalent but the
  tool can't see it. Document it.

## Task 2: Document false positives

For EVERY remaining warning after your fixes, write a categorized entry in
your report. Use this format:

```
### Category: wf subsumption
- File: BSTPlainMtEph.rs:123
- Function: `insert`
- Warning: "StEph has ensures clause `tree_is_bst(self.spec_root())` with no match in MtEph"
- Why false positive: MtEph's `spec_bstplainmteph_wf()` includes `tree_is_bst()`
  as a sub-predicate. The ensures `self.spec_bstplainmteph_wf()` implies `tree_is_bst()`.
- Suggested tool fix: When comparing ensures, expand wf predicates and check if
  the reference clause is subsumed by a wf ensures in the current variant.
```

Group by category. Include exact file:line, function name, warning text,
why it's false, and what the tool should do differently.

## Work order

1. Read all 101 warnings: `~/projects/veracity/target/release/veracity-compare-par-mut -c ~/projects/APAS-VERUS --chapter Chap37`
2. Categorize each one.
3. Fix the fixable ones.
4. Validate after fixes.
5. Document all false positives.

## Rules

- Do NOT weaken ensures. Only strengthen.
- Do NOT add assume or accept.
- Run validates sequentially.
- No subagents.

## STEP 30

## Report

Write `plans/agent3-r115-chap37-false-positives-report.md`. This report
will be handed directly to veracity as a prompt, so make it detailed and
precise. Every false positive must have file:line, function, warning text,
and explanation.
