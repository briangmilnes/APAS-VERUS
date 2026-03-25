# Agent 3 Round 78 Report

## Objective

Prove 2 external_body holes in BSTRBMtEph.rs (filter_parallel, reduce_parallel).

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 37 | BSTRBMtEph.rs | 2 | 0 | -2 |

## Verification

- 4900 verified, 0 errors (baseline: 4898)
- 2774 RTT passed
- 157 PTT passed

## Techniques

Both functions had `#[verifier::external_body]` with `ensures true`. The bodies were
sequential recursive traversals using `Arc::clone` to share closures — no actual thread
spawning despite the `_parallel` suffix.

**Changes made:**

1. **Removed `external_body`** from both `filter_parallel` and `reduce_parallel`.

2. **Added closure requires propagation** to the trait and free functions:
   - `filter`: `forall|t: &T| #[trigger] predicate.requires((t,))`
   - `reduce`: `forall|a: T, b: T| #[trigger] op.requires((a, b))`
   - Added `self.spec_bstrbmteph_wf()` to trait requires (matching other trait methods).

3. **Simplified Arc usage**: Removed unnecessary `Arc::clone` calls — since the functions
   pass `&Arc<F>` by reference, no clone is needed for sequential recursion.

4. **Fixed Verus compatibility**:
   - Replaced `predicate(&node.key)` with `(**predicate)(&node.key)` (explicit Arc deref).
   - Replaced `op(a, b)` with `(**op)(a, b)` (explicit Arc deref).
   - Replaced `result.extend(right_vals)` with `result.append(&mut right_vals)` (vstd-spec'd).

5. **Added proof for recursive requires**: `reveal_with_fuel(link_spec_size, 2)` + assertions
   that child subtree sizes fit in usize, satisfying recursive call preconditions.

## Remaining Holes in Chap37

BSTRBMtEph.rs is now clean. The 5 remaining Chap37 holes are in BSTSplayMtEph.rs
(3 external_body blocked by Link clone, 1 assume, 1 external_body on Clone impl).
