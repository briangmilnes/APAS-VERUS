# R141 Agent 3 Report — ArraySeqMtEphSlice bare_impl + flatten ensures

## Holes Fixed

| # | Chap | File | Hole Type | Fix |
|---|------|------|-----------|-----|
| 1 | 19 | ArraySeqMtEphSlice.rs | bare_impl | Converted 9 functions from bare `impl` to `pub(crate)` free functions |
| 2 | 19 | ArraySeqMtEphSlice.rs | fn_missing_ensures | Added `spec_sum_inner_lens` spec + length ensures on `flatten_dc_vec` and `flatten` |

## Holes Before/After

| # | Chap | File | Before | After |
|---|------|------|--------|-------|
| 1 | 19 | ArraySeqMtEphSlice.rs | 2 | 0 |

## Changes

### Hole 1: bare_impl → free functions

Converted the entire `impl<T: Eq + Clone> ArraySeqMtEphSliceS<T>` block (section 9b)
into module-level `pub(crate)` free functions per standard 19:

- 4 proof functions: `lemma_monoid_fold_left`, `lemma_prefix_fold_matching`,
  `lemma_prefix_fold_split`, `lemma_prefix_fold_eq_fold_left`
- 5 exec functions: `reduce_dc`, `map_dc_vec`, `filter_dc_vec`, `tabulate_dc_vec`,
  `scan_dc_vec`

Each function received its own generic type parameter (proof fns: `<T>`,
exec fns: `<T: Eq + Clone + Send + Sync + 'static, F: ...>`).
All 35 `Self::` call sites updated to direct function calls.

### Hole 2: flatten ensures

Added `spec_sum_inner_lens<T>` spec function (section 6) that computes the total
length of all inner sequences via structural recursion on the outer slice window.

Added `lemma_sum_inner_lens_split<T>` proof function (section 9c) proving the
additive split property needed by the D&C structure.

Strengthened ensures on both `flatten_dc_vec` and `flatten`:
- `flatten_dc_vec`: `ensures result@.len() == spec_sum_inner_lens(a)`
- `flatten`: `ensures flattened.spec_len() == spec_sum_inner_lens(a)`

## Verification

- Isolate Chap19: 880 verified, 0 errors
- Full: 5610 verified, 0 errors
- RTT: 3634 passed
- Chap19 holes: 0
