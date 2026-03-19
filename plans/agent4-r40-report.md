# Agent 4 — Round 40 Report

## Summary

Removed 4 proof holes from Chap43: 3 assumes in AugOrderedTableMtEph (closure totality,
2 finiteness) and 1 external_body in OrderedSetMtEph (to_seq). Fixed 3 warnings
(2 fn_missing_requires + 1 fn_missing_wf_requires). Strengthened AugOrderedTableMtEph wf
to include reducer totality. Strengthened OrderedSetStEph to_seq ensures.

## Verification

- **4292 verified, 0 errors** (was 4290 baseline, +2)
- **2613 RTT pass, 0 failures**
- **182 total holes** (was 186 baseline, -4)
- **Chap43: 31 holes, 11 warnings** (9 assume_eq_clone_workaround + 2 fn_missing_requires)

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 43 | AugOrderedTableMtEph.rs | -3 holes, -3 warnings: wf strengthened, requires added |
| 2 | 43 | OrderedSetMtEph.rs | -1 external_body: to_seq proved with clone loop |
| 3 | 43 | OrderedSetStEph.rs | Strengthened to_seq ensures (seq wf) |

## AugOrderedTableMtEph.rs — Changes

### wf Strengthened (line 334)

Added reducer totality to `spec_augorderedtablemteph_wf`, matching AugOrderedTableStPer:

```
&& forall|v1: &V, v2: &V| #[trigger] self.reducer.requires((v1, v2))
```

### Holes Removed (-3)

| # | Function | Was | Fix |
|---|----------|-----|-----|
| 1 | calculate_reduction | assume(closure totality) | Lifted to requires |
| 2 | reduce_val | assume(self@.dom().finite()) | Added `requires self@.dom().finite()` |
| 3 | reduce_range | assume(self@.dom().finite()) | Added `requires self.spec_augorderedtablemteph_wf()` |

### Warnings Fixed (-3)

| # | Function | Warning | Fix |
|---|----------|---------|-----|
| 1 | calculate_reduction | fn_missing_requires | Added `requires forall\|v1, v2\| reducer.requires(...)` |
| 2 | recalculate_reduction | fn_missing_requires | Added `requires table.spec_augorderedtablemteph_wf()` |
| 3 | recalculate_reduction | fn_missing_wf_requires | Same fix (wf includes reducer totality) |

### Requires Added to 14 Trait Methods

All mutating methods (`&mut self`) use `old(self).spec_augorderedtablemteph_wf()`.
Non-mutating methods use `self.spec_augorderedtablemteph_wf()` or `self@.dom().finite()`.
Methods: empty, singleton, tabulate, map, intersection, union, difference, restrict,
subtract, join_key, get_key_range, split_rank_key, reduce_val, reduce_range.

### Remaining Holes (1)

`reduce_range_parallel` external_body — uses `ParaPair!` for parallelism. Cannot
sequentialize per project rules.

## OrderedSetMtEph.rs — to_seq Proved

Removed `#[verifier::external_body]` from `to_seq`. New implementation:

1. Acquire read lock, call `inner.to_seq()` → `AVLTreeSeqStPerS<T>` (wf + view ensures)
2. RWLOCK_GHOST bridge: `assume(inner@ =~= self@)` (structural false positive)
3. Release lock. `avl_seq` is owned persistent tree, valid after release.
4. Clone elements via loop: `clone_plus()` + `lemma_cloned_view_eq` per element
5. Construct `ArraySeqStPerS { seq: elements }` directly (not via from_vec) so Z3
   sees `result.seq == elements` without indirection
6. Assert `result@ =~= avl_seq@` (follows from pointwise clone bridge + extensional eq)

### OrderedSetStEph.rs — Strengthened to_seq ensures

Added `seq.spec_avltreeseqstper_wf()` to `to_seq` trait ensures. Required by MtEph's
`values_in_order` call (replaced by direct clone loop, but ensures still useful for
other callers).

## Not Fixed

| # | Chap | File | Hole | Reason |
|---|------|------|------|--------|
| 1 | 43 | OrderedSetStEph.rs | select assume | Needs sortedness in AVLTreeSetStEph wf; cascades through Chap41 |
| 2 | 43 | OrderedSetStPer.rs | select assume | Same as above |
| 3 | 43 | AugOrderedTableStPer.rs | closure clone assume | Verus limitation: no spec bridge for cloned closures |
| 4 | 43 | OrderedSetStEph.rs | from_sorted_elements warning | No real precondition exists |
| 5 | 43 | OrderedSetStPer.rs | from_sorted_elements warning | No real precondition exists |
