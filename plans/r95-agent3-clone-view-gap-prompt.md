# R95 Agent 3 — Fix clone-view gap in AdjTableGraph (4 assumes), STEP 20

## Objective

4 assumes in AdjTableGraph StEph/StPer exist because `v.clone()@ == v@` can't
be proved for generic `V: StT + Ord`. Fix by adding `ClonePreservesView` to V
bounds and using `clone_view()` instead of `clone()`.

## The 4 assumes

All are in insert_edge or delete_edge, pattern:
```rust
// After inserting v into neighbor set:
assume(updated.spec_adj()[u@].contains(v@));
// Can't prove because v was cloned, and clone doesn't ensure view preservation
```

Or delete_edge:
```rust
// After removing v from neighbor set:
assume(!updated.spec_adj()[u@].contains(v@));
// Same clone gap on v
```

## Fix strategy

### 1. Add ClonePreservesView to V bounds

In the trait and impl, change:
```rust
// Before:
pub trait AdjTableGraphStEphTrait<V: StT + Ord>

// After:
pub trait AdjTableGraphStEphTrait<V: StT + Ord + ClonePreservesView>
```

### 2. Replace clone() with clone_view()

Where the code says `v.clone()`, use `v.clone_view()` which ensures
`result@ == self@`. Then `set.insert(v.clone_view())` gives
`set@.contains(v.clone_view()@)` which equals `set@.contains(v@)`.

### 3. Prove the assumes

With `clone_view()` providing `result@ == v@`, the set insert/remove
postconditions directly prove the assumes.

## Files

- `src/Chap52/AdjTableGraphStEph.rs` — 2 assumes (insert_edge, delete_edge)
- `src/Chap52/AdjTableGraphStPer.rs` — 2 assumes (insert_edge, delete_edge)

## Read first

- `src/Chap52/AdjTableGraphStEph.rs` — find the assumes in insert_edge/delete_edge
- `src/vstdplus/clone_view.rs` — ClonePreservesView trait, clone_view()
- `src/Chap52/EdgeSetGraphMtPer.rs` — uses ClonePreservesView already (pattern reference)

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Adding ClonePreservesView to the trait may require updating callers. Check
  what uses AdjTableGraphStEphTrait and ensure V satisfies the new bound.
- Do NOT modify MtPer — it uses OrderedTableMtPer which has different issues.
- Do NOT add new assumes.

## STEP 20

## Report

Write `plans/agent3-r95-clone-view-report.md`.
