# R141 Agent 1 Report: Add missing operations to ArraySeqMtEphSlice

## Summary

Added 7 missing operations to `ArraySeqMtEphSliceS<T>` in `src/Chap19/ArraySeqMtEphSlice.rs`
to match the Vec-backed `ArraySeqMtEphS<T>` interface.

## Operations Added

| # | Chap | File | Operation | Work | Span |
|---|------|------|-----------|------|------|
| 1 | 19 | ArraySeqMtEphSlice.rs | `is_empty` | O(1) | O(1) |
| 2 | 19 | ArraySeqMtEphSlice.rs | `is_singleton` | O(1) | O(1) |
| 3 | 19 | ArraySeqMtEphSlice.rs | `set` | O(n) | O(n) |
| 4 | 19 | ArraySeqMtEphSlice.rs | `append` | O(n+m) | O(n+m) |
| 5 | 19 | ArraySeqMtEphSlice.rs | `update` | O(n) | O(n) |
| 6 | 19 | ArraySeqMtEphSlice.rs | `inject` | O(n+m) | O(n+m) |
| 7 | 19 | ArraySeqMtEphSlice.rs | `ninject` | O(n+m) | O(n+m) |

## Implementation Details

- `is_empty`, `is_singleton`: trivial O(1) checks on `self.len`.
- `set`, `update`: materialize slice window via `to_vec()`, set element, wrap in `from_vec`.
- `append`: allocate new Vec, copy both halves via `nth_cloned` loop, wrap in `from_vec`.
- `inject`: clone backing into Vec, apply updates end-to-front (matches `spec_inject` recursion).
- `ninject`: delegates to `inject`, proves ninject postcondition via `lemma_spec_inject_element`.

All ensures match the Vec-backed version in `ArraySeqMtEph.rs`. Standalone: spec functions
(`spec_inject`, `spec_ninject`) and lemmas (`lemma_spec_inject_len`, `lemma_spec_inject_element`)
are defined locally — no imports from `ArraySeqMtEph`.

## Verification

- `scripts/validate.sh isolate Chap19`: 892 verified, 0 errors.
- `scripts/rtt.sh`: 3653 passed, 0 skipped.

## RTTs Added

25 new runtime tests covering all 7 operations:
- `test_is_empty_true`, `test_is_empty_false`
- `test_is_singleton_true`, `test_is_singleton_false_empty`, `test_is_singleton_false_multi`
- `test_set_first`, `test_set_last`, `test_set_preserves_length`
- `test_append_basic`, `test_append_empty_left`, `test_append_empty_right`, `test_append_both_empty`
- `test_update_basic`
- `test_inject_empty_updates`, `test_inject_single_update`, `test_inject_multiple_updates`, `test_inject_out_of_bounds_ignored`
- `test_ninject_basic`, `test_ninject_empty_updates`

## No new holes

Zero assumes, accepts, or external_body added.
