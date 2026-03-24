# R66 Continuation Report: Fix empty/singleton by removing type axioms from wf

## Summary

Removed `obeys_cmp_spec::<T>()` and `view_ord_consistent::<T>()` from
`spec_avltreesetsteph_wf` predicate, adding them as explicit `requires` on every
method that needs them. This eliminates the 2 irreducible verification errors in
`empty()` and `singleton()` from R66.

## Verification Results

- **Validate**: 4340 verified, 0 errors (was 4338 verified, 2 errors)
- **RTT**: 2528 tests passed
- **PTT**: 147 tests passed
- **Holes**: 0 across all chapters (unchanged)

## Files Modified

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 41 | AVLTreeSetStEph.rs | Remove axioms from wf; add to 21 method requires; 2 more PartialEq::eq assumes |
| 2 | 41 | AVLTreeSetMtEph.rs | Add axiom requires to trait from_seq/find; add TYPE_AXIOM assumes to 6 RWLOCK methods |
| 3 | 41 | Example41_3.rs | Add assumes for char type axioms (no broadcast for char); add import |
| 4 | 53 | GraphSearchStEph.rs | Add axiom requires to 4 fns + trait (3 methods) + 2 loop invariants |
| 5 | 53 | PQMinStEph.rs | Add axiom requires for V, Pair<Pair<P,V>,V>, Pair<V,P> to 3 fns + trait (2 methods) + 4 loop invariants |

## Design Decisions

1. **wf no longer includes type axioms.** `spec_avltreesetsteph_wf` now contains only
   `self.tree.spec_bstparasteph_wf() && self@.len() < usize::MAX as nat`. This mirrors
   BSTParaStEph's design where type axioms are explicit requires, not embedded in wf.

2. **Axiom propagation through Chap53 call chains.** GraphSearchStEph and PQMinStEph
   required axiom requires on every function in the call chain, plus all loop invariants.
   PQMinStEph needed axioms for three distinct element types: V, Pair<Pair<P,V>,V>,
   and Pair<V,P>.

3. **MtEph from_seq and find use trait requires** (not assumes) to provide axioms.
   This is cleaner than structural_false_positive assumes and eliminates holes.
   The other 6 MtEph methods (filter, intersection, difference, union, delete, insert)
   retain TYPE_AXIOM assumes inside RWLOCK_GHOST proof blocks, classified as info by
   veracity.

4. **char not in group_laws_cmp.** vstd's broadcast group doesn't include char, so
   Example41_3.rs needs explicit assumes for `obeys_cmp_spec::<char>()` and
   `view_ord_consistent::<char>()`. This is Example file code, not algorithmic.
