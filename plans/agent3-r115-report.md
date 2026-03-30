# R115 Agent 3 Report

**Task:** Chap37 compare-par-mut warning analysis + veracity false positive documentation
**Result:** Complete analysis and documentation of all 101 warnings

## Summary

| Metric | Value |
|--------|-------|
| Total warnings analyzed | 101 |
| False positives | 93 |
| Real but intentional | 5 |
| Fixable (non-trivial proof work) | 3 |
| Code changes made | 0 |
| Verification required | No (analysis-only round) |

## What was done

1. Ran `veracity-compare-par-mut --chapter Chap37` and captured all 101 warnings.
2. Read all MtEph and StEph trait definitions and wf predicates.
3. Categorized every warning into 11 categories.
4. Wrote comprehensive false positive report: `plans/agent3-r115-chap37-false-positives-report.md`.

## Key findings

**93 of 101 warnings are false positives** caused by 6 tool limitations:

| # | Tool limitation | Warnings | Priority |
|---|----------------|----------|----------|
| 1 | Cannot expand wf predicates to check subsumption | ~51 | P1 |
| 2 | Cannot equate `spec_root()` with `@` (View) | ~8 | P2 |
| 3 | Cannot extract ensures from match arms | ~5 | P3 |
| 4 | Warns when MtEph has MORE ensures (stronger) | ~5 | P4 |
| 5 | Ghost fields always flagged as "no counterpart" | ~5 | P5 |
| 6 | Free function spec equivalence not recognized | ~7 | P6 |

**5 warnings are real but intentional:**
- BSTAVLMtEph maintains `tree_is_bst` not `tree_is_avl` (3 warnings)
- BSTPlainMtEph/BSTBBAlphaMtEph missing `delete` (2 warnings)

**3 warnings are fixable** but require non-trivial proof work:
- AVLTreeSeqMtPer `set` missing `spec_seq` update ensures
- BSTSplayMtEph `in_order`/`pre_order` `ensures true` (entire helper chain needs proof)

## Deliverable

The primary deliverable is `plans/agent3-r115-chap37-false-positives-report.md`.
This report is designed to be handed directly to veracity as a prompt for improving
the compare-par-mut tool. It contains:

- Per-category explanation of why warnings are false positives
- Exact file:line, function name, and warning text for every warning
- Concrete suggested tool fixes with implementation guidance
- Priority-ordered improvement recommendations
- Complete appendix listing all 101 warnings with category assignments
