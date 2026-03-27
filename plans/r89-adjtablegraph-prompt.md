# R89 — Fix AdjTableGraphStEph + AdjTableGraphStPer, STEP 20

## Objective

Uncomment and fix AdjTableGraphStEph.rs and AdjTableGraphStPer.rs in Chap52.
The sst_to_air Verus bug that blocked these files is now fixed. The errors are
regular compile/verification failures from the OrderedTable→Table switch.

## CRITICAL: Work from the working Mt version

The MtPer version (`AdjTableGraphMtPer.rs`) **compiles and verifies clean**.
Read it FIRST. It is your reference for the current Table API, field names,
spec function signatures, and wf predicates. The St files should mirror the
MtPer version's API usage — minus the RwLock wrapper.

Also read:
- `src/Chap52/AdjTableGraphMtPer.rs` — **your primary reference** (working)
- `src/Chap42/TableStEph.rs` — Table trait and API
- `src/Chap42/TableStPer.rs` — Table persistent variant
- `src/Chap52/AdjSeqGraphStEph.rs` — working StEph graph for pattern reference
- `src/Chap52/AdjSeqGraphStPer.rs` — working StPer graph for pattern reference

Do NOT start editing until you have read AdjTableGraphMtPer.rs and understand
its API surface.

## lib.rs — uncomment your files

Uncomment BOTH files in lib.rs. They are currently commented out with `// BROKEN`.
Remove the comment prefix.

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap52
```

Do NOT run full `scripts/validate.sh`, `scripts/rtt.sh`, or `scripts/ptt.sh`.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## What to fix

The files were switched from OrderedTable to Table in R82b but never got past the
Verus sst_to_air crash. Now that the crash is fixed, the remaining errors are:
- Type mismatches from the OrderedTable→Table API change
- Missing or wrong spec function calls
- Verification failures on ensures clauses

For each function:
1. Check how AdjTableGraphMtPer implements it
2. Mirror that implementation for the St variant
3. If a proof is too hard, use `external_body` — but try first

## Important

- Do NOT modify AdjTableGraphMtPer.rs — it's working, don't touch it.
- Do NOT modify files outside Chap52.
- Do NOT add `assume` or `accept`.
- Use `external_body` only as a last resort on functions that are too hard to prove.

## STEP 20

## Report

Write `plans/agent-r89-adjtablegraph-report.md`.
