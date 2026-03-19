# R41b Agent 3: Chap41 AVLTreeSetMtPer Delegation Wrappers

## Baseline
- Main at `29641a5e`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- Agent 3 R41a proved all 5 AVLTreeSetMtEph methods. Now do MtPer.

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**

Read CLAUDE.md and `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` before starting.

## Context

You just proved all 5 AVLTreeSetMtEph methods by replacing parallel implementations with
lock-based delegation to the inner StEph. Now apply the same pattern to MtPer.

In R41a you noted MtPer "would need architectural changes." That's wrong — it's the same
RwLock delegation pattern you just used for MtEph. Acquire lock, call inner StPer method,
wrap result in new MtPer. The inner type is `AVLTreeSetStPer<T>` instead of
`AVLTreeSetStEph<T>`, and MtPer methods return new `Self` (persistent) instead of
mutating `&mut self`.

## Assignment

### File: `src/Chap41/AVLTreeSetMtPer.rs` — 7 holes + 1 warning

| # | Method | Line | Notes |
|---|--------|------|-------|
| 1 | from_seq | 224 | Construct from AVLTreeSeqMtPerS — sort vals, build StPer |
| 2 | filter | 276 | Acquire read lock, call inner.filter(), wrap new MtPer |
| 3 | intersection | 339 | Acquire read locks on both, call inner.intersection() |
| 4 | difference | 397 | Same pattern |
| 5 | union | 408 | Same pattern |
| 6 | delete | 498 | Acquire read lock, call inner.delete(), wrap new MtPer |
| 7 | insert | 508 | Acquire read lock, call inner.insert(), wrap new MtPer |

**Warning** (line 230): `fn_missing_requires` on `parallel_sort`. If you replace the
parallel implementation with lock-based delegation, this helper may be eliminated
(like you eliminated parallel_filter and parallel_intersect in MtEph).

### Pattern (same as your MtEph work)

```rust
fn method(&self, ...) -> (result: Self) {
    let read_handle = self.locked_set.acquire_read();
    let inner = read_handle.borrow();
    let st_result = inner.method(...);
    read_handle.release_read();
    // Wrap st_result in new MtPer
    let ghost view = st_result@;
    let locked = new_arc_rwlock(st_result, Ghost(AVLTreeSetMtPerInv { ... }));
    AVLTreeSetMtPer { locked_set: locked }
}
```

For methods 6-7 (insert/delete on persistent): the inner StPer returns a new StPer.
Wrap that in a new MtPer.

For from_seq: extract the sequence values, sort them, build an AVLTreeSetStPer from
the sorted values, wrap in MtPer.

### Also: AVLTreeSetStEph assumes (2 holes) — if time permits

Lines 1085 and 1352: `assume(new_vec@.len() < usize::MAX)` in insert/insert_sorted.

The tree wf gives `self.elements@.len() < usize::MAX`. After inserting one element,
`new_vec@.len() <= self.elements@.len() + 1`. You need `len + 1 < usize::MAX` which
requires `len < usize::MAX - 1`. Check `lemma_wf_implies_len_bound` — if the bound
is tight enough, prove it. If not, can the bound be strengthened?

### Expected Results

Conservative: 5-7 MtPer holes closed.
Optimistic: 7 MtPer + 2 StEph = 9 holes closed. Close Chap41.

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent3-r41b-report.md`.
