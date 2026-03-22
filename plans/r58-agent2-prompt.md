<!-- R58 Agent 2 -->
# R58: Fix fn_missing_wf in Chap05/SetStEph.rs and Chap05/SetMtEph.rs

## Assignment

Fix 14 `fn_missing_wf_requires` / `fn_missing_wf_ensures` warnings in
`src/Chap05/SetStEph.rs` and `src/Chap05/SetMtEph.rs`.

## Warnings to fix

### SetMtEph.rs (7 warnings)

| # | Line | Type | Function |
|---|------|------|----------|
| 1 | 228 | fn_missing_wf_requires | `elt_cross_set` — add `s2.spec_setmteph_wf()` |
| 2 | 228 | fn_missing_wf_ensures | `elt_cross_set` — add `product.spec_setmteph_wf()` |
| 3 | 239 | fn_missing_wf_requires | `cartesian_product` — add `s2.spec_setmteph_wf()` |
| 4 | 239 | fn_missing_wf_ensures | `cartesian_product` — add `product.spec_setmteph_wf()` |
| 5 | 251 | fn_missing_wf_requires | `all_nonempty` — add `parts.spec_setmteph_wf()` |
| 6 | 260 | fn_missing_wf_requires | `partition_on_elt` — add `parts.spec_setmteph_wf()` |
| 7 | 275 | fn_missing_wf_requires | `partition` — add `parts.spec_setmteph_wf()` |

### SetStEph.rs (7 warnings)

| # | Line | Type | Function |
|---|------|------|----------|
| 8 | — | fn_missing_wf_requires | `elt_cross_set` — add `s2.spec_setsteph_wf()` |
| 9 | — | fn_missing_wf_ensures | `elt_cross_set` — add `product.spec_setsteph_wf()` |
| 10 | — | fn_missing_wf_requires | `cartesian_product` — add `s2.spec_setsteph_wf()` |
| 11 | — | fn_missing_wf_ensures | `cartesian_product` — add `product.spec_setsteph_wf()` |
| 12 | — | fn_missing_wf_requires | `all_nonempty` — add `parts.spec_setsteph_wf()` |
| 13 | — | fn_missing_wf_requires | `partition_on_elt` — add `parts.spec_setsteph_wf()` |
| 14 | — | fn_missing_wf_requires | `partition` — add `parts.spec_setsteph_wf()` |

## IMPORTANT — Chap05 Set wf uses free functions

Chap05 Set files use `spec_setsteph_wf_generic()` / `spec_setmteph_wf_generic()`
free functions due to a Verus 3-node cycle. Do NOT try to use
`self.spec_setsteph_wf()` as a trait method — it will cause a cycle error.
Instead, read the existing code to see how wf is expressed and follow the
same pattern. The wf for input parameters of Set type should use the same
free-function form used elsewhere in the file.

## Fix pattern

Add the wf predicate to the `requires` or `ensures` clause in the **trait
definition**. Match the existing wf pattern in each file (free function vs
trait method). Read the file first.

## DO NOT TOUCH

- Any file outside `src/Chap05/SetStEph.rs` and `src/Chap05/SetMtEph.rs`
- Do not add `assume`, `accept`, `external_body`, or `admit`
- Do not weaken existing ensures
- Do not try to fix the `structural_false_positive` items — those are known

## Validation

```bash
scripts/validate.sh 2>&1 | tee /tmp/validate-r58.txt | tail -15
scripts/holes.sh src/Chap05/
```

Write report to `plans/agent2-round58-report.md`. Include Chap column in tables.
Commit and push to `agent2/ready`.
