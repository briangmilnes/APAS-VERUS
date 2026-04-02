# R142 Agent 1 (acting as Agent 2) — Chap18/ArraySeqMtEphSlice.rs Hole Fixes

## Summary

Fixed all 3 holes in `src/Chap18/ArraySeqMtEphSlice.rs` by porting fixes from `src/Chap19/ArraySeqMtEphSlice.rs`.

## Holes Before/After

| # | Chap | File | Hole Type | Before | After |
|---|------|------|-----------|--------|-------|
| 1 | 18 | ArraySeqMtEphSlice.rs | bare_impl | 1 | 0 |
| 2 | 18 | ArraySeqMtEphSlice.rs | fn_missing_ensures (flatten_dc_vec) | 1 | 0 |
| 3 | 18 | ArraySeqMtEphSlice.rs | assume (inject) | 1 | 0 |

**Total: 3 → 0 holes**

## Changes

### Hole 1: bare_impl → free functions

Converted the `impl<T: Eq + Clone> ArraySeqMtEphSliceS<T>` bare impl block to
`pub(crate)` free functions. Each function got explicit type parameters
(`T: Eq + Clone + Send + Sync + 'static` for exec fns, `<T>` for proof fns).
All `Self::fn_name(...)` call sites updated to `fn_name(...)`.

Functions converted:
- `lemma_monoid_fold_left`
- `lemma_prefix_fold_matching`
- `lemma_prefix_fold_split`
- `lemma_prefix_fold_eq_fold_left`
- `reduce_dc`
- `map_dc_vec`
- `filter_dc_vec`
- `tabulate_dc_vec`
- `scan_dc_vec`

### Hole 2: fn_missing_ensures on flatten_dc_vec

Added:
- `spec_sum_inner_lens` spec function (section 6)
- `lemma_sum_inner_lens_split` proof function (section 9c)
- `ensures result@.len() == spec_sum_inner_lens(a)` on `flatten_dc_vec`
- `ensures flattened.spec_len() == spec_sum_inner_lens(a)` on `flatten`
- Proof code in `flatten_dc_vec` base case (len==1) and recursive case (ghost sums + closure ensures + loop invariant)

### Hole 3: assume in inject

Replaced the forward-loop inject implementation (which had
`assume(injected_seq =~= expected)`) with an end-to-front loop
that matches `spec_inject`'s recursive structure. The loop invariant
`result_vec@ =~= spec_inject(s, u.subrange(i, ulen))` tracks the
partial application, and the proof unfolds `spec_inject` at each step
via `reveal(spec_inject)` + subrange equalities.

## Validation

- `scripts/validate.sh isolate Chap18`: 1076 verified, 0 errors
- `scripts/rtt.sh`: 3690 passed, 0 skipped
- `scripts/holes.sh`: 0 holes (1 expected assume_eq_clone_workaround warning)
