# R34 Agent 1 Report: Chap41 AVLTreeSetStEph Sortedness Infrastructure

## Summary

Added sortedness spec infrastructure to `src/Chap41/AVLTreeSetStEph.rs` enabling
Chap43 OrderedSet/OrderedTable to prove ordering operations (first, last, previous,
next, rank, select). The infrastructure includes spec functions, proof lemmas, and
two TotalOrder-gated inherent methods (`insert_sorted`, `delete_sorted`) with full
sortedness-preservation proofs.

## Verification

- **4164 verified, 0 errors**
- **2613 RTT passed**
- **147 PTT passed**

## Changes

All changes in `src/Chap41/AVLTreeSetStEph.rs`:

### New spec functions (section 6)
- `spec_inorder_values<T>(link: Link<T>) -> Seq<T>`: Mirror of `spec_inorder` returning
  actual values (Seq<T>) instead of views (Seq<T::V>). Recursive over tree structure.
- `spec_seq_sorted<T: TotalOrder>(s: Seq<T>) -> bool`: Pairwise `TotalOrder::le` over
  a sequence. Uses explicit `#[trigger]` on `TotalOrder::le(s[i], s[j])`.

### New proof lemmas (section 7)
- `lemma_inorder_values_maps_to_views`: Proves `spec_inorder_values(link).map_values(|t| t@) =~= spec_inorder(link)`. Bridges between T-level and T::V-level sequences.
- `lemma_push_sorted`: Appending an element >= all existing preserves sortedness.
- `lemma_subseq_sorted`: Subsequence of a sorted sequence is sorted.

### New TotalOrder-gated impl block (section 9)
Inherent methods on `impl<T: StT + Ord + TotalOrder> AVLTreeSetStEph<T>`:
- `spec_elements_sorted(&self) -> bool`: Whether the backing sequence is sorted.
- `spec_values_seq(&self) -> Seq<T>`: The value-level backing sequence.
- `insert_sorted(&mut self, x: T)`: Insert preserving sortedness. Uses TotalOrder::cmp
  for binary search with full proof of sorted postcondition via positional rv tracking
  and feq bridge.
- `delete_sorted(&mut self, x: &T)`: Delete preserving sortedness. Uses ghost
  `orig_idx_map: Seq<int>` to track which orig_vals index each result entry came from,
  proving the result is a sorted subsequence.

### Other
- Added `use crate::vstdplus::total_order::total_order::TotalOrder` import.
- Updated Table of Contents to include sections 6 and 7.

## Holes

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 41 | AVLTreeSetStEph.rs | 1 | 2 | +1 |

**Hole details:**
- `assume(new_vec@.len() < usize::MAX)` in `insert` (line 1059): Pre-existing. The wf
  bound gives `cached_size < usize::MAX`, but `from_vec` needs strictly-less for the
  n+1 size vector. Same structural issue in all insert methods.
- `assume(new_vec@.len() < usize::MAX)` in `insert_sorted` (line 1334): Same structural
  issue as above.

Both assumes are the same root cause: AVLTreeSeqStEph wf bounds `total_size < usize::MAX`
but insert creates a vector of size `total_size + 1`, needing `total_size + 1 < usize::MAX`
(equivalently `total_size < usize::MAX - 1`). Fixing requires either strengthening wf or
adding a precondition that cascades to MtEph callers.

The delete_sorted sorted proof is fully verified (0 assumes).

## Techniques

1. **Positional rv tracking**: Instead of containment-based invariants (`rv.contains(x)`),
   tracked `rv[k] == orig_elems[k]` for the before-insertion portion, `rv[lo] == x_view`
   for the insertion point, and `rv[lo + 1 + k] == orig_elems[lo + k]` for the after-insertion
   portion. This enabled the final sorted proof via case analysis.

2. **Ghost orig_idx_map**: For delete_sorted, tracked a `Seq<int>` mapping each result
   position to its source index in orig_vals. The map is strictly increasing, so the result
   is a sorted subsequence of the sorted original.

3. **feq bridge**: Used `obeys_feq_full::<T>()` to bridge between T-level (TotalOrder::le)
   and T::V-level (view equality) facts. Chain: `new_vals[k]@ == rv[k] == orig_elems[m] == orig_vals[m]@`, under feq gives `new_vals[k] == orig_vals[m]`.

4. **TotalOrder::cmp binary search**: Replaced `*elem < x` (Ord-based) with
   `TotalOrder::cmp(elem, &x)` for the binary search, producing postconditions directly
   in terms of `TotalOrder::le` that feed the sorted proof.

## Impact on Chap43

The new infrastructure exposes:
- `spec_elements_sorted()`: Sortedness predicate for OrderedSet/OrderedTable to include
  in their wf predicates.
- `insert_sorted()` / `delete_sorted()`: Operations that maintain sortedness, enabling
  Chap43 to prove that ordering operations work on a sorted backing store.
- `spec_values_seq()`: Access to the T-level backing sequence for quantifying over
  elements in `first`, `last`, `previous`, `next` ensures clauses.

These are inherent methods (not trait methods), so no cascading changes needed in Chap43's
trait bounds. Chap43 code can call them via the concrete type since `T: TotalOrder` is
already present on the ordering methods.
