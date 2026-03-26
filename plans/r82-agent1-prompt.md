# R82 Agent 1 — Fix Chap55: remove .view() on Seq + pub visibility, STEP 15

## Objective

Uncomment and fix all 8 files in Chap55 (DFS, TopoSort, CycleDetect, SCC — StEph and StPer).

## lib.rs Memory Isolation Protocol

Before your first validate, comment out all chapters AFTER Chap55 to save memory.
Use EXACTLY this format:

```
/* R82-ISOLATED: agent 1, working on Chap55
#[cfg(all(not(feature = "experiments_only"), not(feature = "union_find")))]
pub mod Chap56 {
...
R82-ISOLATED */
```

Wrap lines 591-670 (Chap56 through Chap66 + the closing `*/`) in this block.
Do NOT touch anything before Chap55. Do NOT touch Chap05 or Chap06.

**Before pushing to agent1/ready, REMOVE the isolation wrapper.** Restore lib.rs
to match main except for your Chap55 fixes. Verify with:
`git diff origin/main -- src/lib.rs` — only Chap55 lines should differ.

## What to fix

### 1. Uncomment all 8 files in Chap55

In `src/lib.rs`, change the commented-out Chap55 entries to active:
```rust
pub mod Chap55 {
    pub mod DFSStEph;
    pub mod DFSStPer;
    pub mod TopoSortStEph;
    pub mod TopoSortStPer;
    pub mod CycleDetectStEph;
    pub mod CycleDetectStPer;
    pub mod SCCStEph;
    pub mod SCCStPer;
}
```

### 2. Fix visibility in TopoSortStEph.rs

`src/Chap55/TopoSortStEph.rs` line 119: change `proof fn` to `pub proof fn`:
```rust
pub proof fn lemma_set_true_decreases_num_false(s: Seq<bool>, idx: int)
```

### 3. Remove .view() calls on Seq values (48 instances)

All 8 files call `.view()` on values that are already `Seq<T>` (a spec type that
has no `.view()` method). These were written when the underlying type was `Vec<T>`
(which has `.view()` returning `Seq<T>`). The type changed but the `.view()` calls
were never removed.

For each error of the form:
```
no method named `view` found for struct `vstd::seq::Seq<A>`
```

Remove the `.view()` call. For example:
- `some_seq.view().len()` → `some_seq.len()`
- `some_seq.view()[i]` → `some_seq[i]`
- `some_seq.view().subrange(a, b)` → `some_seq.subrange(a, b)`

This is mechanical — find every `.view()` call that the compiler flags, remove it.

## Validation

Run `scripts/validate.sh` (with isolation), then before pushing restore lib.rs
and run full `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh`.
Push to `agent1/ready`.

## STEP 15

At most 15 edit/verify iterations. Then stop and report.

## Report

Write `plans/agent1-round82-report.md` with files fixed and verified count.
