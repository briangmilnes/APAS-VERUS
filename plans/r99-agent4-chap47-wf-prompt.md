# R99 Agent 4 — Add wf requires/ensures to ParaHashTable (Chap47), STEP 10

## Objective

Veracity reports 7 `fn_missing_wf_requires` and 2 `fn_missing_wf_ensures`
warnings in `src/Chap47/ParaHashTableStEph.rs`. Add the missing wf clauses.

## The warnings

| # | Function | Missing |
|---|----------|---------|
| 1 | createTable | ensures table.spec_hashtable_wf() |
| 2 | insert | requires table.spec_hashtable_wf() |
| 3 | lookup | requires table.spec_hashtable_wf() |
| 4 | delete | requires table.spec_hashtable_wf() |
| 5 | metrics | requires table.spec_hashtable_wf() |
| 6 | loadAndSize | requires table.spec_hashtable_wf() |
| 7 | resize | requires table.spec_hashtable_wf() |
| 8 | resize | ensures resized.spec_hashtable_wf() |

## What to do

1. Read `src/Chap47/ParaHashTableStEph.rs`
2. Find the wf predicate (`spec_hashtable_wf` or similar)
3. Add `requires` and `ensures` as indicated
4. Verify each function still passes — adding requires should be non-breaking
   (all callers should already have wf), adding ensures may need proof work

## Isolation

```bash
scripts/validate.sh isolate Chap47
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## STEP 10

## Report

Write `plans/agent4-r99-chap47-wf-report.md`.
