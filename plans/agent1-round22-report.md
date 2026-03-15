# Agent 1 — Round 22 Report

## Mission

Prove `external_body` holes in Chap37 (BST chapter, root blocker for 9 downstream chapters).

## Results Summary

- **Chap37 holes**: 15 → 10 (−5)
- **Verified functions**: 3958 → 3963 (+5)
- **Errors**: 0
- **RTT**: 2600 passed
- **PTT**: 147 passed

## Holes Eliminated

| # | Chap | File | Function | Technique |
|---|------|------|----------|-----------|
| 1 | 37 | AVLTreeSeqStEph.rs | `set` | Strengthened `set_link` ensures with inorder update spec |
| 2 | 37 | AVLTreeSeqStPer.rs | `set` | Strengthened `set_rec` ensures; feq broadcast for clone view equality |
| 3 | 37 | AVLTreeSeqStPer.rs | `subseq_copy` | Body already satisfies `wf` ensures from `from_vec` |
| 4 | 37 | AVLTreeSeqStPer.rs | `values_in_order` | Rewrote using `nth + clone_plus` loop with feq bridge |
| 5 | 37 | AVLTreeSeqStPer.rs | `to_arrayseq` | Follows from `values_in_order` + `from_vec` spec |

## Remaining Holes (10)

| # | Chap | File | Holes | Category | Blocker |
|---|------|------|------:|----------|---------|
| 1 | 37 | AVLTreeSeq.rs | 1 | external_body | Iterator::next has no `requires` slot (structural) |
| 2 | 37 | AVLTreeSeqMtPer.rs | 2 | external_body | Thread boundary: `&[T]` slices can't be `'static` (structural) |
| 3 | 37 | BSTSplayStEph.rs | 6 | external_body | All helpers (`splay`, `bst_insert`, `find_link`, `min_link`, `max_link`) have `ensures true` |
| 4 | 37 | BSTSplayStEph.rs | 1 | trivial_spec_wf | `spec_bstsplaysteph_wf` is `true` (amortized analysis gap) |

## Techniques Used

1. **Inorder update spec**: Added `spec_inorder(*node) =~= spec_inorder(*old(node)).update(index, value@)` to `set_link`/`set_rec`. Verus proves extensional equality of concatenated sequences under point updates automatically.

2. **feq broadcast for Arc clones**: In StPer (Arc-based persistent trees), can't move values out of Arc for `lemma_cloned_view_eq`. Used `assert(obeys_feq_full_trigger::<T>())` to trigger broadcast axiom, then `assert(n_val@ == n.value@)` — Verus connects `clone_plus` postcondition through `obeys_feq_clone` + `obeys_feq_view` quantifiers.

3. **nth + clone_plus loop**: Rewrote `values_in_order` from tree traversal (`inorder_collect` with `ensures true`) to index-based loop with `nth + clone_plus + lemma_cloned_view_eq`. O(n log n) vs O(n) performance trade-off, but verifiable.

## BSTSplayStEph Analysis

Attempted proving `insert` by strengthening `bst_insert` and `splay` ensures:
- `bst_insert`: `spec_contains_link(link, value)` postcondition proved automatically.
- `bst_insert`: `forall|x| contains(old, x) ==> contains(new, x)` failed — SMT can't prove universal preservation over recursive predicate across `match` arms. Needs per-arm proof decomposition with ghost captures of pre-mutation state.
- `splay`: Element preservation (`forall|x| contains(Some(root), x) <==> contains(Some(result), x)`) failed — 6 rotation cases each need explicit proof. Structurally tractable but requires ~100 lines of proof assertions.
- Conclusion: Proving BSTSplay holes requires building spec infrastructure from scratch (all helpers have `ensures true`). Multi-round effort.

## What Blocks Further Progress

1. **BSTSplayStEph** (7 holes): Needs spec infrastructure for splay element preservation, BST insert membership, BST search correctness. Requires strengthening `spec_bstsplaysteph_wf` from `true` to `spec_is_bst_link`.
2. **AVLTreeSeq iterator** (1 hole): Iterator::next can't have `requires` — irreducible.
3. **AVLTreeSeqMtPer threads** (2 holes): `'static` closure boundary — irreducible without sequentializing.
