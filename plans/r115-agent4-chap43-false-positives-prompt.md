# R115 Agent 4 — Chap43 warning analysis + veracity false positive documentation. AFK. PBOGH.

## Objective

Two tasks:
1. Reduce Chap43 compare-par-mut warnings where possible (327 warnings).
2. Carefully document every remaining false positive for a veracity prompt.

## Current state

Agent 3 (R112) reduced Chap43 from 416 to 348, then compare tool updates
brought it to 327. Remaining warnings are across 7 files:

| # | File | Warnings |
|---|------|----------|
| 1 | OrderedTableMtEph.rs | ~100 |
| 2 | AugOrderedTableMtEph.rs | ~60 |
| 3 | OrderedTableStEph.rs | ~55 |
| 4 | OrderedTableMtPer.rs | ~40 |
| 5 | OrderedSetMtEph.rs | ~35 |
| 6 | AugOrderedTableStEph.rs | ~25 |
| 7 | OrderedSetStEph.rs | ~15 |

Known false positive categories from R112:
- Eph/Per pattern mismatch (~150): `old(self)@` vs `self@`, `self@` vs return name
- Missing ensures requiring new RWLOCK_GHOST assumes (~100)
- Missing functions / _iter variants (~50)
- Cascading requires breaking downstream callers (~40)

## Task 1: Fix what you can

For each warning, determine if it's fixable or a false positive:
- **Fixable**: Missing requires/ensures that can be added without new assumes.
- **False positive**: Tool limitation. Document it.
- **Blocked**: Needs new assumes or would break callers. Document why.

## Task 2: Document false positives

For EVERY remaining warning after your fixes, write a categorized entry.
Use this format:

```
### Category: eph/per pattern mismatch
- File: OrderedTableStEph.rs:456
- Function: `split`
- Warning: "StPer has ensures clause `parts.0@.subset_of(self@)` with no match in StEph"
- Why false positive: StEph has `split.0@.subset_of(old(self)@)` — semantically
  identical after old(self)→self substitution for persistent variant.
- Suggested tool fix: Normalize `old(self)` in ephemeral ensures to `self` in
  persistent ensures before comparing.
```

Group by category. Include exact file:line, function name, warning text,
why it's false, and what the tool should do differently.

## Work order

1. Run: `~/projects/veracity/target/release/veracity-compare-par-mut -c ~/projects/APAS-VERUS --chapter Chap43`
2. Categorize every warning.
3. Fix the fixable ones.
4. Validate with `scripts/validate.sh isolate Chap43` after fixes.
5. Document all false positives.

## Rules

- Do NOT weaken ensures. Only strengthen.
- Do NOT add assume or accept.
- Run validates sequentially.
- No subagents.

## STEP 30

## Report

Write `plans/agent4-r115-chap43-false-positives-report.md`. This report
will be handed directly to veracity as a prompt, so make it detailed and
precise. Every false positive must have file:line, function, warning text,
and explanation.
