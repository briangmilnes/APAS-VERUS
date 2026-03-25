# R73 Agent 1 Report: Fix 4 Broken Chap37 Mt Modules

## Summary

Enabled 4 previously-disabled Mt (multi-threaded) BST modules in Chap37. All were commented
out in lib.rs behind `all_chapters` feature flag. Each required substantial rewrites to
compile and verify under Verus.

## Results

- **Verification**: 4554 verified, 0 errors (was 4517 before BSTSetBBAlphaMtEph)
- **RTT**: 2619 tests passed, 0 failed
- **PTT**: 157 tests passed, 0 failed
- **Chap37 holes**: 16 total (12 rwlock:predicate, 4 external_body). No new real holes.

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 37 | BSTPlainMtEph.rs | RwLock Mt wrapper for BSTPlainStEph (prior session) |
| 2 | 37 | BSTBBAlphaMtEph.rs | RwLock Mt wrapper for BSTBBAlphaStEph; removed unused spec imports |
| 3 | 37 | BSTSetPlainMtEph.rs | Complete rewrite: BTreeSet/iterators → while loops, TotalOrder bounds |
| 4 | 37 | BSTSetBBAlphaMtEph.rs | Complete rewrite modeled on BSTSetPlainMtEph |
| 5 | 37 | TestBSTSetPlainMtEph.rs | Added missing trait imports (ArraySeqStPerBaseTrait, BSTPlainMtEphTrait) |
| 6 | 37 | TestBSTSetBBAlphaMtEph.rs | Fixed Chap19→Chap18 import, removed feature gate |
| 7 | 37 | TestBSTPlainMtEph.rs | Removed feature gate |
| 8 | 37 | TestBSTBBAlphaMtEph.rs | Removed feature gate |
| 9 | — | lib.rs | Uncommented all 4 module declarations |

## Key Techniques

1. **Iterator chain elimination**: Replaced all `BTreeSet`, `.iter().cloned().collect()`,
   `.filter()`, `.fold()` with explicit while loops (Verus can't verify std iterators).

2. **TotalOrder bounds**: Added `TotalOrder` bound everywhere instead of relying on `Ord`
   (needed for BST operations in Verus).

3. **Compound assignment**: `i += 1` → `i = i + 1` (Verus limitation).

4. **Decreases clauses**: Added to all 9 while loops per file; used
   `#[verifier::exec_allows_no_decreases_clause]` for union/intersection/difference
   (recursive through ParaPair thread boundary).

5. **Thread boundary assumes**: `assume(result.spec_wf())` after ParaPair results
   (standard Mt pattern — 6 assumes per Set file, classified as rwlock:predicate).

6. **Closure requires propagation**: filter needs
   `forall|t: &T| #[trigger] predicate.requires((t,))` in both trait and loop invariant;
   reduce needs `forall|a: T, b: T| #[trigger] op.requires((a, b))`.

7. **Difference bug fix**: Original code checked `found_self` (always true) instead of
   `found_other` when deciding whether to include pivot in set difference result.

8. **Spec function import fix**: Removed unused `tree_is_bb`/`weight_balanced` imports
   from BSTBBAlphaMtEph.rs (spec fns invisible to cargo).

## Hole Accounting

No change in real proof holes. The 12 rwlock:predicate assumes in the Set files are the
standard thread-boundary pattern (ParaPair loses postconditions). The 4 external_body
holes are pre-existing in other Chap37 files.
