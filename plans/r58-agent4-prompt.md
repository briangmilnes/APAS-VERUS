<!-- R58 Agent 4 -->
# R58: Fix fn_missing_requires in Chap65 and close Chap38 clone bridge holes

## Assignment

Two tasks:

### Task A: Fix 2 fn_missing_requires in Chap65 (easy)

| # | File | Type | Function |
|---|------|------|----------|
| 1 | Chap65/KruskalStEph.rs | fn_missing_requires | `sort_edges_by_weight` |
| 2 | Chap65/PrimStEph.rs | fn_missing_requires | `pq_entry_new` |

Read the function body. Add the real precondition. If it takes graph
parameters, it likely needs wf on the graph.

### Task B: Close 2 clone bridge holes in Chap38 (medium)

| # | File | Line | Current hole |
|---|------|------|-------------|
| 3 | BSTParaStEph.rs | 479 | `assume(obeys_feq_clone::<T>())` |
| 4 | BSTParaMtEph.rs | 712 | `assume(obeys_feq_clone::<T>())` |

These are `assume(obeys_feq_clone::<T>())` in top-level `expose` functions.
The fix is to lift `obeys_feq_clone::<T>()` into the function's `requires`
clause (in the trait), then remove the `assume`. Callers already satisfy
`obeys_feq_full::<T>()` which implies `obeys_feq_clone::<T>()`.

Pattern:
```rust
// BEFORE (in trait):
fn expose(...)
    requires self.spec_wf(),
// AFTER:
fn expose(...)
    requires self.spec_wf(), obeys_feq_clone::<T>(),
```

Then in the impl body, remove the `proof { assume(obeys_feq_clone::<T>()); }`
line. The requires gives it directly.

Check that callers of `expose` / `expose_internal` already have
`obeys_feq_full::<T>()` or `obeys_feq_clone::<T>()` available. If a caller
doesn't, add it to that caller's requires too (cascade up).

## DO NOT TOUCH

- Any file outside `src/Chap38/` and `src/Chap65/`
- Do not add `assume`, `accept`, `external_body`, or `admit`
- Do not weaken existing ensures

## Validation

```bash
scripts/validate.sh 2>&1 | tee /tmp/validate-r58.txt | tail -15
scripts/holes.sh src/Chap38/
scripts/holes.sh src/Chap65/
```

Write report to `plans/agent4-round58-report.md`. Include Chap column in tables.
Commit and push to `agent4/ready`.
