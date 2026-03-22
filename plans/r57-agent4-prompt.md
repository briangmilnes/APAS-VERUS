<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 4 — Round 57 Prompt

## Branch

Work on `agent4/ready`. Push when done.

## DO NOT TOUCH

- Chap47 (any file)
- Chap41 (any file — Agent 2)
- Chap43 (any file — Agent 1)
- Chap45 (any file — Agent 2)
- Chap05 (any file — Agent 3)
- Chap42 (any file — Agent 3)
- Any file in any other chapter not listed in your assignment

## Assignment: Add fn_missing_ensures to Chap62-64 graph files

These functions have executable code with postconditions but are missing
`ensures` clauses. Add ensures that describe what the function returns.

### Task 1: StarContractionStEph.rs — 2 functions

**File:** `src/Chap62/StarContractionStEph.rs`

| # | Function | Line | What to add |
|---|----------|------|-------------|
| 1 | `star_contract_fuel` | ~67 | ensures on return value |
| 2 | `star_contract` | ~120 | ensures on return value |

Read the function bodies. Understand what they return (contracted graph).
Write ensures that describe the output graph's properties relative to
the input.

### Task 2: StarContractionMtEph.rs — 2 functions

**File:** `src/Chap62/StarContractionMtEph.rs`

| # | Function | Line | What to add |
|---|----------|------|-------------|
| 1 | `star_contract_mt_fuel` | ~86 | ensures on return value |
| 2 | `star_contract_mt` | ~139 | ensures on return value |

Same pattern as the St version. The Mt version should have matching ensures.

### Task 3: ConnectivityStEph.rs — 4 functions

**File:** `src/Chap63/ConnectivityStEph.rs`

| # | Function | Line | What to add |
|---|----------|------|-------------|
| 1 | `count_components` | ~77 | ensures count == ... |
| 2 | `connected_components` | ~100 | ensures on return |
| 3 | `count_components_hof` | ~165 | ensures count == ... |
| 4 | `connected_components_hof` | ~185 | ensures on return |

### Task 4: ConnectivityMtEph.rs — 4 functions

**File:** `src/Chap63/ConnectivityMtEph.rs`

| # | Function | Line | What to add |
|---|----------|------|-------------|
| 1 | `count_components_mt` | ~83 | ensures |
| 2 | `connected_components_mt` | ~106 | ensures |
| 3 | `count_components_hof` | ~153 | ensures |
| 4 | `connected_components_hof` | ~171 | ensures |

### Task 5: SpanTreeStEph.rs + SpanTreeMtEph.rs — 2 functions

**File:** `src/Chap64/SpanTreeStEph.rs`

| # | Function | Line |
|---|----------|------|
| 1 | `spanning_tree_star_contraction` | ~51 |

**File:** `src/Chap64/SpanTreeMtEph.rs`

| # | Function | Line |
|---|----------|------|
| 1 | `spanning_tree_star_contraction_mt` | ~54 |

### Task 6: TSPApproxStEph.rs — 1 function

**File:** `src/Chap64/TSPApproxStEph.rs`

| # | Function | Line |
|---|----------|------|
| 1 | `approx_metric_tsp` | ~315 |

### Approach

1. Read each function body to understand what it computes.
2. Write an `ensures` clause that captures the key postcondition.
   Start simple: return type properties, length bounds, wf of output.
   If the function wraps another function with ensures, propagate those.
3. If proving the ensures requires non-trivial work, start with what Verus
   can prove automatically and note what remains.
4. Validate after each file pair (St + Mt together).

Total: 15 functions across 7 files.

## Validation

Run `scripts/validate.sh` after each file pair. Show full output. Fix all warnings and errors.

## Report

Write `plans/agent4-round57-report.md` with holes before/after table including Chap column.
