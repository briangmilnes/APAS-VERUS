# R82 Agent 3 — Fix Chap52+53: AVLTreeSet API refactor, STEP 20

## Objective

Fix 4 files in Chap52 (AdjTableGraphStEph, AdjTableGraphStPer, EdgeSetGraphStEph,
EdgeSetGraphStPer) and 1 file in Chap53 (GraphSearchMtPer) so they compile and verify.

The MtPer variants (AdjTableGraphMtPer, EdgeSetGraphMtPer) are left commented out —
they have additional dependency issues beyond this refactor.

## lib.rs Memory Isolation Protocol

Before your first validate, comment out all chapters AFTER Chap53 to save memory.
Use EXACTLY this format:

```
/* R82-ISOLATED: agent 3, working on Chap52+53
#[cfg(all(not(feature = "experiments_only"), not(feature = "union_find")))]
pub mod Chap54 {
...
R82-ISOLATED */
```

Wrap lines 571-670 (Chap54 through Chap66 + closing) in this block.
Do NOT touch anything before Chap52. Do NOT touch Chap05 or Chap06.

**Before pushing to agent3/ready, REMOVE the isolation wrapper.** Restore lib.rs
to match main except for your Chap52/53 fixes. Verify with:
`git diff origin/main -- src/lib.rs` — only Chap52 and Chap53 lines should differ.

## What to fix

### 1. Uncomment 5 files

In Chap52, uncomment:
```rust
    pub mod AdjTableGraphStEph;
    pub mod AdjTableGraphStPer;
```

In Chap53, uncomment:
```rust
    pub mod GraphSearchMtPer;
```

Also uncomment EdgeSetGraph files:
```rust
    pub mod EdgeSetGraphStEph;
    pub mod EdgeSetGraphStPer;
```

### 2. Refactor for new AVLTreeSet API

The files were written when `AVLTreeSetStEph` and `AVLTreeSetStPer` had:
- An `elements` field (AVLTreeSeqStEph/StPer) — now replaced by `tree` (ParamBST)
- An `Ord` trait bound — now uses `TotalOrder` from `vstdplus::total_order`
- Direct field access like `.elements.length()`, `.elements.nth(i)`

The current API uses the trait methods: `size()`, `find()`, `insert()`, `delete()`,
`in_order()`, `iter()`. Read these files to understand the new API:
- `src/Chap41/AVLTreeSetStEph.rs` — current StEph API
- `src/Chap41/AVLTreeSetStPer.rs` — current StPer API

The 107 errors break down as:
- 24× `Sized` not known at compilation time
- 17× type annotations needed
- 16× `AVLTreeSetStPer<V>: std::cmp::Ord` not satisfied
- 9× `AVLTreeSetStEph<V>: std::cmp::Ord` not satisfied
- Various method-not-found due to unsatisfied trait bounds
- 3× `no field elements` on AVLTreeSetMtPer

Key changes needed:
- Replace `Ord` bounds with `TotalOrder` bounds where AVLTreeSet is used as a value type
- Replace `.elements` field access with trait method calls
- Add type annotations where the compiler can't infer after the API change
- Fix Sized issues (may need explicit `Sized` bounds or `where Self: Sized`)

### 3. GraphSearchMtPer (Chap53)

3 errors, same class — `.elements` field gone on AVLTreeSetMtPer. Replace with
trait method calls.

## Important

- Read each broken file fully before editing.
- Read the working files in the same chapter (AdjSeqGraphStEph, AdjMatrixGraphStEph)
  for the current patterns.
- Do NOT add `assume` or `accept`.
- Do NOT weaken ensures clauses.
- If a file is too broken to fix in the step budget, comment it back out with
  an updated BROKEN comment explaining what remains.

## STEP 20

At most 20 edit/verify iterations. Then stop and report.

## Validation

Run `scripts/validate.sh` (with isolation), then before pushing restore lib.rs
and run full `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh`.
Push to `agent3/ready`.

## Report

Write `plans/agent3-round82-report.md` with files fixed, errors before/after, verified count.
