# R140 Agent 3 — Eliminate capacity assume in AdjTableGraphMtPer

## Result: 1 assume eliminated, 0 new holes

### Problem

`src/Chap52/AdjTableGraphMtPer.rs:472` had:
```rust
assume(neighbors@.len() < usize::MAX as nat);
```
This was needed because `AVLTreeSetMtPer::delete` required `self@.len() < usize::MAX`,
which propagated from `ParamBSTTrait::delete` in BSTParaMtEph.rs.

### Root Cause

`delete` does not grow a collection — it removes an element. The capacity
requires `old(self)@.len() < usize::MAX` was added defensively by Agent2 in R138,
but is unnecessary. Here's why:

`ParamBST::delete` calls:
1. `self.size()` — returns `usize`, proving `self@.len() <= usize::MAX`
2. `split_inner(self, key)` — decomposes into `left, right` where
   `left@ ∪ right@ = self@.remove(key@)`, so `left@.len() + right@.len() <= self@.len()`
3. `left.join_pair_inner(&right)` — requires `self@.len() + right@.len() <= usize::MAX`

Since `left@.len() + right@.len() <= self@.len() <= usize::MAX`, the `join_pair_inner`
precondition is satisfied without any additional capacity requires on `delete`.

### Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 38 | BSTParaMtEph.rs | Removed `old(self)@.len() < usize::MAX as nat` from `ParamBSTTrait::delete` requires |
| 2 | 38 | BSTParaMtEph.rs | Added capacity proof hint in `delete` impl: `assert(left@.len() + right@.len() <= usize::MAX as nat)` |
| 3 | 41 | AVLTreeSetMtPer.rs | Removed `self@.len() < usize::MAX as nat` from `AVLTreeSetMtPerTrait::delete` requires |
| 4 | 52 | AdjTableGraphMtPer.rs | Removed `assume(neighbors@.len() < usize::MAX as nat)` and replaced `assert_avltreesetmtper_bounded_size` with `assert_avltreesetmtper_always_wf` |

### Verification

- Full validate: 5592 verified, 0 errors
- RTT: 3621 passed
- PTT: 221 passed
- Chap52 holes: 0 (was 1)

### Chapter Cleanliness

| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| Clean chapters | 41 | 44 | +3 |
| Holed chapters | 5 | 2 | -3 |
| Global holes | 9 | 6 | -3 |

Note: the stale chapter-cleanliness-status.log showed 41 clean / 9 holes; after
refreshing the analysis, the numbers show 44 clean / 6 holes. The 3-hole improvement
beyond this round's single assume likely reflects holes closed in prior rounds that
hadn't been re-counted.

### Technique

The capacity_bounds_standard says capacity requires belong on operations that GROW
collections (insert, push, append). `delete` REMOVES an element and should not need
a capacity bound. The fix was to remove the defensive requires from the trait chain
(`ParamBSTTrait::delete` → `AVLTreeSetMtPerTrait::delete`) and add a proof assertion
in the impl to help Z3 derive the bound from `size()` returning `usize`.
