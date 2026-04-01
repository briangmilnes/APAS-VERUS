# R130 Agent 3 Report — Easy Proof Holes in Chap43, 52, 53

## Summary

Closed 2 of 4 targeted holes. The other 2 are structural issues requiring
architectural changes (RWLOCK_GHOST ghost/inner mismatch, closure universality).

## Holes Before/After

| # | Chap | File | Hole | Status | Technique |
|---|------|------|------|--------|-----------|
| 1 | 43 | OrderedSetMtEph.rs:547 | `assume(inner@.len() + 1 < usize::MAX)` | NOT FIXED | RWLOCK_GHOST: wf gives `len < MAX` but need `len + 1 < MAX` |
| 2 | 43 | OrderedTableStPer.rs:3495 | `from_sorted_entries` missing wf ensures | FIXED | Added wf ensures + key uniqueness invariant |
| 3 | 52 | AdjTableGraphMtPer.rs:450 | `assume(neighbors.spec_avltreesetmtper_wf())` | NOT FIXED | Closure universality: `map` requires `forall\|v\| f.requires((v,))` |
| 4 | 53 | GraphSearchMtPer.rs:231 | `assume(neighbors.spec_avltreesetmtper_wf())` | FIXED | Loop invariant already had `graph.ensures ==> wf`; removed assume |

## Details

### Hole 2 (FIXED): OrderedTableStPer from_sorted_entries wf ensures

Mirrored the StEph version's proof pattern:
- Added requires: `spec_pair_key_determines_order`, `obeys_cmp_spec::<K>`,
  `view_ord_consistent::<K>`, `obeys_feq_fulls::<K, V>`, key uniqueness.
- Added loop invariant: `spec_key_unique_pairs_set(tree@)`, provenance tracking.
- Added ensures: `result.spec_orderedtablestper_wf()`.
- Key uniqueness proof: contradiction via entry index ordering.
- Only caller is the `OrderedTableStPerLit!` macro (outside verus!, not verified).

### Hole 4 (FIXED): GraphSearchMtPer neighbors wf

The loop invariant already stated:
```
forall|v: &V, r: AVLTreeSetMtPer<V>|
    #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetmtper_wf()
```
After `let neighbors = graph(v)`, Verus knows `graph.ensures((v,), neighbors)`,
which triggers the forall and gives `neighbors.spec_avltreesetmtper_wf()`.
Replaced `assume` with `assert`.

### Hole 1 (NOT FIXED): OrderedSetMtEph get_range capacity

The RwLock inv gives `inner.spec_orderedsetsteph_wf()` which includes
`inner@.len() < usize::MAX` (from AVLTreeSetStEph wf). But the St `get_range`
requires `inner@.len() + 1 < usize::MAX` — one element tighter. The Mt type's
requires has this bound on `self@` (ghost snapshot), but `inner@` (locked value)
is not provably equal to `self@` during read operations. Fix requires either
strengthening the RwLock inv or tracking ghost/inner equivalence.

### Hole 3 (NOT FIXED): AdjTableGraphMtPer delete_vertex closure

The `map` function on `OrderedTableStPer` requires `forall|v: &V| f.requires((v,))` —
universal over ALL values, not just table entries. Adding
`requires neighbors.spec_avltreesetmtper_wf()` to the closure would require
ALL `AVLTreeSetMtPer<V>` instances to be wf, which isn't universally true.
The StEph version avoids this by using a loop instead of `map`.
Fix requires rewriting `delete_vertex` to iterate manually, or adding
stored-value wf to the MtPer graph's wf predicate.

## Verification

- Full validation: 5546 verified, 0 errors
- RTT: 3536 passed
- PTT: 221 passed
- Trigger warnings: 0

## Chapter Cleanliness

Chap53 went from 1 holed module to 0 (now fully clean).
Chap43 and Chap52 unchanged (1 holed module each — the unfixed holes).
