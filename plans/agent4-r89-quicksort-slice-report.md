# Agent 4 — R89 QuickSortMtEphSlice Report

## Objective

Get `QuickSortMtEphSlice.rs` (Chap36) compiling and verifying. The file was
commented out in `lib.rs` due to 13 verification errors caused by missing
`to_vec` on `ArraySeqMtEphSliceS`.

## Changes Made

### 1. Added `to_vec` to ArraySeqMtEphSlice (Chap19)

File: `src/Chap19/ArraySeqMtEphSlice.rs`

Added `to_vec` method to trait and impl. The method materializes the slice
window into a freshly-allocated `Vec<T>` by cloning elements via `clone_plus`.

```
fn to_vec(&self) -> (v: Vec<T>)
    requires self.spec_arrayseqmtephslice_wf(), obeys_feq_clone::<T>(),
    ensures v@.len() == self.spec_len(),
            forall|i: int| 0 <= i < self.spec_len() ==> v@[i] == self.spec_index(i);
```

### 2. Uncommented QuickSortMtEphSlice in lib.rs

Removed the `//` prefix and `BROKEN:` comment from `pub mod QuickSortMtEphSlice;`.

### 3. Fixed QuickSortMtEphSlice.rs

**Root causes of the 13 original errors:**

1. **`(*la.data).clone()` has no Verus specs.** The original workaround cloned the
   backing `Arc<Vec<T>>` data, but `Vec::clone()` lacks `ensures result@ == self@`
   in Verus. Replaced with `la.to_vec()` using the new method.

2. **`concat_three_vecs` used bare `.clone()`.** Unlike the `QuickSortMtEph` version
   (which uses `T: Copy` and `*left.nth(j)`), the Slice version uses `T: Clone`.
   Standard `clone()` has no Verus specs. Fixed by using `clone_plus()` from
   `crate::vstdplus::clone_plus`, which provides `ensures result == *self` under
   `obeys_feq_clone::<T>()`. Added `obeys_feq_clone` to `concat_three_vecs` requires.

3. **Closure proof chain needed explicit steps.** The closures performing parallel
   recursive sorts needed three proof elements not present in the `MtEph` version:
   - `lemma_total_ordering::<T>()` — satisfies `sort_by`'s `recommends total_ordering(leq)`
   - `lemma_elements_from_vec::<T>(view, slice)` — establishes `elements(slice) =~= view`
   - `ghost pre_elems = elements(la)` — bridges `old(la)` to `left_view` across `&mut` call
   - Explicit forall connecting `to_vec` output to `elements(la)` via `spec_index`

4. **`=~=` didn't give SMT solver length equality automatically.** After `ParaPair!`,
   the assertions `sorted_left@.len() == left_view.len()` required an explicit
   multiset-based length derivation chain:
   ```
   sort_by(leq).to_multiset() =~= original.to_multiset()
   → to_multiset().len() == seq.len()
   → sort_by(leq).len() == original.len()
   ```

## Verification Results

| # | Chap | File | Holes Before | Holes After |
|---|------|------|--------------|-------------|
| 1 | 19 | ArraySeqMtEphSlice.rs | 0 | 0 |
| 2 | 36 | QuickSortMtEphSlice.rs | 13 errors (commented out) | 0 |

- **Isolate Chap36**: 874 verified, 0 errors from Chap19/Chap36 files
  (4 pre-existing experiment bitvector errors only)
- **Full validation**: 5284 verified, 0 new errors
  (16 pre-existing errors from experiments + Chap61)
- **Veracity**: Both files report 0 proof holes, 100% clean

## Techniques Used

- `clone_plus()` for spec-aware cloning inside Verus
- Ghost variable `pre_elems` to bridge `old(la)` across `&mut` calls
- `lemma_total_ordering` inside closures for `sort_by` recommends
- `lemma_elements_from_vec` re-invoked inside closures (proof facts don't carry into closures)
- Multiset-length chain to derive `sort_by().len() == original.len()`

## Files Modified

1. `src/Chap19/ArraySeqMtEphSlice.rs` — added `to_vec` trait method + impl
2. `src/Chap36/QuickSortMtEphSlice.rs` — replaced `(*la.data).clone()` with `to_vec()`,
   fixed `concat_three_vecs` with `clone_plus`, added closure proof infrastructure,
   added post-ParaPair length derivation
3. `src/lib.rs` — uncommented `QuickSortMtEphSlice`
