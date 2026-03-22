<!-- R58 Agent 1 -->
# R58: Fix fn_missing_wf in Chap47/ParaHashTableStEph.rs

## Assignment

Fix 8 `fn_missing_wf_requires` / `fn_missing_wf_ensures` warnings in
`src/Chap47/ParaHashTableStEph.rs`.

## Warnings to fix

| # | Line | Type | Function |
|---|------|------|----------|
| 1 | 412 | fn_missing_wf_ensures | `createTable` |
| 2 | 463 | fn_missing_wf_requires | `insert` |
| 3 | 480 | fn_missing_wf_requires | `lookup` |
| 4 | 492 | fn_missing_wf_requires | `delete` |
| 5 | 507 | fn_missing_wf_requires | `metrics` |
| 6 | 516 | fn_missing_wf_requires | `loadAndSize` |
| 7 | 533 | fn_missing_wf_requires | `resize` |
| 8 | 533 | fn_missing_wf_ensures | `resize` |

## Fix pattern

For `fn_missing_wf_requires`: add `table.spec_hashtable_wf()` (or
`self.spec_hashtable_wf()`) to the function's `requires` clause in the trait.

For `fn_missing_wf_ensures`: add `result.spec_hashtable_wf()` (or
`table.spec_hashtable_wf()`) to the `ensures` clause in the trait.

Read the function signatures to determine the correct parameter/return names.
The wf predicate is `spec_hashtable_wf()`.

## DO NOT TOUCH

- Any file outside `src/Chap47/ParaHashTableStEph.rs`
- Do not add `assume`, `accept`, `external_body`, or `admit`
- Do not weaken existing ensures
- Do not add `requires true` or tautological requires

## Validation

```bash
scripts/validate.sh 2>&1 | tee /tmp/validate-r58.txt | tail -15
scripts/holes.sh src/Chap47/
```

Write report to `plans/agent1-round58-report.md`. Include Chap column in tables.
Commit and push to `agent1/ready`.
