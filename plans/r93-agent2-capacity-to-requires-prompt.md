# R92 Agent 2 — Move 7 capacity assumes to requires, STEP 10

## Objective

7 assumes in Chap52 AdjTableGraph are capacity bounds:
```rust
assume(self.adj@.dom().len() + 1 < usize::MAX as nat);
```

Move these from assumes in the body to requires on the function. This converts
a hidden proof obligation into an explicit caller contract.

## Files

- `src/Chap52/AdjTableGraphStEph.rs` — 1 capacity assume
- `src/Chap52/AdjTableGraphStPer.rs` — 1 capacity assume
- `src/Chap52/AdjTableGraphMtPer.rs` — 5 capacity assumes

## What to do

For each capacity assume:
1. Find the assume
2. Determine what bound is needed (dom().len() + 1, or dom().len() + 2, etc.)
3. Add the bound to the function's `requires` clause
4. Remove the assume
5. Check callers — if any caller can't satisfy the requires, that caller
   needs the same bound in ITS requires (propagate up)

The bounds follow the capacity_bounds_standard (`src/standards/capacity_bounds_standard.rs`).

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Only touch capacity assumes. Leave ICE-blocked and clone-gap assumes alone.
- Propagate requires to callers if needed.
- Check that the trait signatures match the impl signatures after adding requires.

## STEP 10

## Report

Write `plans/agent2-r92-capacity-report.md`.
