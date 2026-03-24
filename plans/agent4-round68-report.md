# Agent 4 — Round 68 Report

## Summary

Closed 8 of 11 targeted holes across 4 Chap43 files. The 3 remaining holes are all
structural (2 clone view gap + 1 unsafe iterator in OrderedSetStEph) — expected and
documented in the prompt.

## Holes Before/After

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 43 | AugOrderedTableStEph.rs | 2 | 0 | -2 | Finiteness from wf chain |
| 2 | 43 | AugOrderedTableStPer.rs | 2 | 0 | -2 | Same pattern |
| 3 | 43 | OrderedTableMtEph.rs | 2 | 0 | -2 | Axioms lifted to requires |
| 4 | 43 | OrderedSetStEph.rs | 5 | 3 | -2 | to_seq membership proved |
| | | **Total** | **11** | **3** | **-8** | |

## Remaining Holes (3, all structural)

| # | Chap | File | Line | Type | What blocks it |
|---|------|------|------|------|----------------|
| 1 | 43 | OrderedSetStEph.rs | 793 | assume | Clone view gap: `k1_clone@ == k1@` |
| 2 | 43 | OrderedSetStEph.rs | 809 | assume | Clone view gap: `k2_clone@ == k2@` |
| 3 | 43 | OrderedSetStEph.rs | 975 | unsafe | Standard iterator raw pointer pattern |

## Techniques Used

1. **Wf-chain finiteness**: `spec_augorderedtablesteph_wf()` unfolds through
   `spec_orderedtablesteph_wf()` → `spec_bstparasteph_wf()` → `self@.finite()`.
   Z3 handles the 3-level chain automatically with no proof hints. Removed
   `assume(self.base_table.tree@.finite())` from both reduce_val and reduce_range.

2. **Inline reduce_val in reduce_range**: Instead of calling `range_table.reduce_val()`
   (which would require proving `range_table.spec_augorderedtablesteph_wf()`),
   accessed `range_table.cached_reduction.clone()` directly.

3. **Axioms lifted to requires**: `from_sorted_entries` in OrderedTableMtEph now requires
   `obeys_cmp_spec::<Pair<K, V>>()` and `view_ord_consistent::<Pair<K, V>>()`, matching
   the downstream OrderedTableStEph::from_sorted_entries contract. No cascade — only
   callers are macro `OrderedTableMtEphLit!` (outside verus!) and tests.

4. **collect_in_order + from_vec membership bridge**: `collect_in_order` has full
   bidirectional membership ensures. `from_vec` ensures `result.spec_seq() =~=
   values@.map_values(|t: T| t@)`. Chained via ghost `elem_views` intermediate to
   prove `result@.to_set() =~= self@` in to_seq.

## Verification Counts

- 4370 verified, 0 errors, 0 trigger warnings
- 2512 RTT pass
- 145 PTT pass
- Global: 104 holes, 45 clean / 1 holed chapter, 7508 total fns

## Corrections to R67 Memory

R67 recorded "collect_in_order lacks membership ensures" — this is wrong. BSTParaStEph's
`collect_in_order` (lines 1382-1388) has full bidirectional membership ensures:
`forall|i| 0 <= i < out@.len() ==> self@.contains(out@[i]@)` and
`forall|v| self@.contains(v) ==> exists|i| 0 <= i < out@.len() && out@[i]@ == v`.
The R67 assumes were unnecessary — R68 proved this.
