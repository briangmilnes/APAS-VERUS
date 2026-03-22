<!-- R58 Agent 3 -->
# R58: Fix fn_missing_requires in Chap39, Chap57, Chap59, Chap65

## Assignment

Fix 8 `fn_missing_requires` / `fn_missing_ensures` warnings across 5 files.

## Warnings to fix

### Chap39/BSTParaTreapMtEph.rs (3 warnings)

| # | Type | Function | Likely fix |
|---|------|----------|-----------|
| 1 | fn_missing_requires | `param_treap_assert_finite` | Read body, add real precondition |
| 2 | fn_missing_requires | `tree_priority_internal` | Read body, add real precondition |
| 3 | fn_missing_ensures | `reduce_inner` | Add ensures on return value |
| 4 | fn_missing_ensures | `reduce_parallel` | Add ensures on return value |

### Chap39/BSTTreapMtEph.rs (1 warning)

| # | Type | Function | Likely fix |
|---|------|----------|-----------|
| 5 | fn_missing_requires | `size_link` | Add requires for link wf |

### Chap57/DijkstraStEphU64.rs (1 warning)

| # | Type | Function | Likely fix |
|---|------|----------|-----------|
| 6 | fn_missing_requires | `pq_entry_new` | Read body, add real precondition |

### Chap59/JohnsonStEphI64.rs (2 warnings)

| # | Type | Function | Likely fix |
|---|------|----------|-----------|
| 7 | fn_missing_requires | `adjust_distance` | Read body, add real precondition |
| 8 | fn_missing_requires | `reweight_edge` | Read body, add real precondition |

## Fix pattern

For each function:
1. Read the function body to understand what it actually requires.
2. Add the **real** precondition — not `requires true`, not a tautology.
3. If the function takes `&self`, it almost certainly requires the module's wf.
4. For ensures, look at what the function returns and what callers need.

## DO NOT TOUCH

- Any file not listed above
- Do not add `assume`, `accept`, `external_body`, or `admit`
- Do not weaken existing ensures
- Do not add `requires true` or tautological requires
- Do not add `// veracity: no_requires`

## Validation

```bash
scripts/validate.sh 2>&1 | tee /tmp/validate-r58.txt | tail -15
scripts/holes.sh src/Chap39/
scripts/holes.sh src/Chap57/
scripts/holes.sh src/Chap59/
```

Write report to `plans/agent3-round58-report.md`. Include Chap column in tables.
Commit and push to `agent3/ready`.
