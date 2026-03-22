<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 3 — Round 57 Prompt

## Branch

Work on `agent3/ready`. Push when done.

## DO NOT TOUCH

- Chap47 (any file)
- Chap41 (any file — Agent 2)
- Chap43 (any file — Agent 1)
- Chap45 (any file — Agent 2)
- Chap62, Chap63, Chap64 (any file — Agent 4)
- Any file in any other chapter not listed in your assignment

## Assignment: Fix fn_missing_wf in Chap05 Sets + Chap42 TableStPer

### Task 1: SetStEph.rs — 6 fn_missing_wf errors

**File:** `src/Chap05/SetStEph.rs`

Add the missing `spec_setsteph_wf()` requires and ensures to these functions
in the trait definition:

| # | Function | Fix |
|---|----------|-----|
| 1 | `elt_cross_set` | Add `requires s2.spec_setsteph_wf()`, `ensures product.spec_setsteph_wf()` |
| 2 | `cartesian_product` | Add `requires s2.spec_setsteph_wf()`, `ensures product.spec_setsteph_wf()` |
| 3 | `all_nonempty` | Add `requires parts.spec_setsteph_wf()` |
| 4 | `partition_on_elt` | Add `requires parts.spec_setsteph_wf()` |
| 5 | `partition` | Add `requires parts.spec_setsteph_wf()` |

Read the function signatures to see which parameters need wf and which
return values need wf ensures. Add wf to the trait, then verify the impl
bodies still pass (they likely already maintain wf — the spec was just
missing from the signature).

### Task 2: SetMtEph.rs — 7 fn_missing_wf errors

**File:** `src/Chap05/SetMtEph.rs`

Same pattern as SetStEph, same functions. Add `spec_setmteph_wf()` requires
and ensures to the trait definition for:

| # | Function | Fix |
|---|----------|-----|
| 1 | `elt_cross_set` | requires s2 wf, ensures product wf |
| 2 | `cartesian_product` | requires s2 wf, ensures product wf |
| 3 | `all_nonempty` | requires parts wf |
| 4 | `partition_on_elt` | requires parts wf |
| 5 | `partition` | requires parts wf |

### Task 3: TableStPer.rs — 1 fn_missing_wf_ensures

**File:** `src/Chap42/TableStPer.rs`
**Line ~2035:** `collect_by_key` is missing `ensures grouped.spec_tablestper_wf()`

Add the ensures to the trait definition. Verify the impl body proves it.

### Approach

These are all mechanical: add wf predicates to trait signatures. The impl
bodies should already maintain the invariants — you're just making the
contracts explicit. Validate after each file.

## Validation

Run `scripts/validate.sh` after each file. Show full output. Fix all warnings and errors.

## Report

Write `plans/agent3-round57-report.md` with holes before/after table including Chap column.
