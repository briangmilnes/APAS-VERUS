# R116 Agent 1 — Strengthen AVLTreeSet MtPer/MtEph specs (Chap41)

## Summary

Added missing `view_ord_consistent::<T>()` and `vstd::laws_cmp::obeys_cmp_spec::<T>()`
requires clauses to AVLTreeSetMtPer and AVLTreeSetMtEph trait functions. Removed
corresponding `assume()` calls from MtPer impl bodies that were bridging these
type-level predicates. Added missing `seq.spec_avltreeseqsteph_wf()` ensures to
MtEph `to_seq`. Fixed downstream callers in Chap52 and Chap53.

## Changes by file

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 41 | AVLTreeSetMtEph.rs | Already had requires (pre-staged). Added wf ensure to `to_seq`. |
| 2 | 41 | AVLTreeSetMtPer.rs | Added requires to 6 trait fns: filter, intersection, difference, union, delete, insert. Removed 10 assume(obeys_cmp/view_ord) from impl bodies. |
| 3 | 52 | AdjTableGraphMtPer.rs | Added `assume(neighbors.spec_avltreesetmtper_wf())` in delete_vertex closure (map callback). |
| 4 | 53 | GraphSearchMtPer.rs | Added obeys_cmp_spec + view_ord_consistent requires to trait (3 fns), free fns (3), and explore loop. Added assume(neighbors.wf()) for graph closure return. |

## Warning resolution

### Resolved (18 of 18 missing cmp/ord requires)
- **MtEph**: filter, intersection, difference, union, delete, insert — 6 functions, 12 warnings.
- **MtPer**: filter, intersection, difference, union, delete, insert — 6 functions, 6 warnings.

### to_seq ensures gap — resolved
- MtEph `to_seq` now ensures `seq.spec_avltreeseqsteph_wf()` (matching StEph and MtPer).

### from_seq requires gap — intentional
- MtPer requires `seq@.len() <= usize::MAX - 2` (loop-based impl needs capacity bound).
- MtEph requires `seq.spec_avltreeseqsteph_wf()` (delegates to StEph). Different contracts.

### _iter functions — not implemented (future work)
- 7 `_iter` variants (find_iter, insert_iter, delete_iter, filter_iter, intersection_iter,
  union_iter, difference_iter) exist in StEph/StPer but not MtEph/MtPer.
- These are linear-scan alternatives to BST-recursive defaults. Adding to Mt is mechanical
  (acquire lock, delegate to inner St `_iter`, release lock) but requires:
  - Adding 7 trait functions + impl functions per Mt file (14 functions total).
  - No new proof work — just lock/delegate/unlock.
- Recommend as a separate task.

### StEph vs StPer sorted specs — false positive
- StEph missing `spec_elements_sorted_per`, `spec_values_seq_per`, `insert_sorted_per`
  from StPer. These are Per-specific sorted-sequence specs. Eph uses `&mut` and doesn't
  need persistent sorted sequence specs. Not a gap.

## Assumes added

| # | Chap | File | Line | Assume | Reason |
|---|------|------|------|--------|--------|
| 1 | 52 | AdjTableGraphMtPer.rs | ~438 | neighbors.spec_avltreesetmtper_wf() | Map callback: value wf not tracked by ordered table. |
| 2 | 53 | GraphSearchMtPer.rs | ~168 | neighbors.spec_avltreesetmtper_wf() | Graph closure return: no wf in ensures. |

Both are pre-existing gaps exposed by strengthening requires. The ordered table `map`
has `forall|v: &V| f.requires((v,))` which can't express value-dependent requires.
The graph closure has no wf ensures on its return type.

## Assumes removed (from MtPer)

10 assumes removed from 6 impl bodies: filter (2), intersection (2), difference (2),
union (2), delete (2), insert (2). All were `assume(obeys_cmp_spec::<T>())` and
`assume(view_ord_consistent::<T>())`, now lifted to function requires.

## Validation

- Chap41 isolate: 2040 verified, 0 new errors.
- Chap52 isolate: 2830 verified, 0 new errors.
- Chap53 isolate: 2098 verified, 0 new errors.
- Full validate: 3 errors, all pre-existing Chap65 rlimit.
- RTTs: 3529/3529 passed.
