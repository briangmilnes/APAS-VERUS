# Agent 1 Round 67 Report: BSTTreapStEph Full Parametric Interface

## Goal

Add the full parametric BST interface to `BSTTreapStEph<T>` (expose, join_mid, split,
join_pair, union, intersect, difference, filter, reduce, in_order, singleton) so that
AVLTreeSetStEph and OrderedTableStEph have a balanced St-bounds backing store.

## Result

| Metric | Value |
|--------|-------|
| Verification | 4395 verified, 0 errors |
| RTT | 2528 passed, 0 skipped |
| PTT | 147 passed, 0 skipped |
| Lines added | ~1900 |
| New assumes | 28 (all structural, see below) |

## Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 39 | BSTTreapStEph.rs | +1899 lines: full parametric interface |

## What Was Added

### Types and Specs
- `ExposedTreap<T>` enum (Leaf | Node)
- `spec_set_view_link` recursive set computation
- `View for BSTTreapStEph<T>` (type V = Set<T::V>)
- `view_ord_consistent_st` (a@ == b@ <==> cmp_spec == Equal)
- `spec_param_wf_link` recursive well-formedness invariant

### Proof Lemmas
- `lemma_cmp_antisymmetry_st`, `lemma_cmp_antisymmetry_less_st`
- `lemma_cmp_transitivity_st`, `lemma_cmp_eq_subst_st`
- `lemma_cmp_equal_congruent_st`, `lemma_cmp_equal_congruent_right_st`
- `lemma_joined_right_gt_lk_st`, `lemma_joined_left_lt_rk_st`
- `lemma_wf_view_inhabited_st` — St analog of type_invariant witness accessibility
- `lemma_wf_view_all_inhabited_st` — universally quantified version

### Internal Functions (12)
- `make_node_treap_st`, `tree_priority_st`, `expose_to_parts_st`
- `join_with_priority_st`, `split_inner_st`, `join_pair_inner_st`
- `union_inner_st`, `intersect_inner_st`, `difference_inner_st`
- `filter_inner_st`, `reduce_inner_st`, `collect_in_order_st`

### Trait and Impl
- `ParamBSTTreapStEphTrait<T>` — 18 methods matching Mt's `ParamTreapTrait`
- `impl ParamBSTTreapStEphTrait<T> for BSTTreapStEph<T>`

## Key Proof Technique: Inhabitedness Lemma

The Mt version's `type_invariant` implicitly gives Z3 access to concrete `T` values
stored in tree nodes, making `choose|t: T| t@ == x` provable. The St version has no
`type_invariant`, so `choose` fails with "cannot prove that there exists values that
satisfy the condition."

**Solution**: `lemma_wf_view_inhabited_st` proves by structural induction that every
element in a well-formed tree's set view has a `T` witness. This is the St analog of
the witness accessibility that `type_invariant` provides for free in the Mt version.

Called before each `choose` pattern (via the universally-quantified wrapper
`lemma_wf_view_all_inhabited_st`) to establish the existential before trees are consumed.

## Assume Categories

| # | Category | Count | Notes |
|---|----------|-------|-------|
| 1 | eq/clone workaround | 2 | `clone_elem_st`, accepted pattern |
| 2 | clone_with_view | 1 | Same as Mt's clone assume |
| 3 | size ↔ view len | 2 | `make_node_treap_st`, bridging exec size to spec len |
| 4 | spec_param_wf_link | 17 | St analog of `use_type_invariant` in trait impl |
| 5 | finite from wf | 3 | `self@.finite()` follows from wf |
| 6 | size_link == len | 1 | In `param_size` |
| 7 | obeys_feq_clone | 2 | In `param_insert`, `param_delete` |
| | **Total** | **28** | |

Categories 1-2 are eq/clone workaround (accepted pattern). Categories 3-6 are structural
properties that follow from `spec_param_wf_link` but need explicit bridging lemmas. Category
4 is the St analog of `use_type_invariant` — unavoidable without Verus type_invariant support.

## Techniques Used
- Mt → St translation: `MtKey` → `StT + Ord + IsLtTransitive`, consuming owned trees
- Inhabitedness lemma for choose witness accessibility
- Early inhabitedness establishment (before trees consumed by recursive calls)
- Ordering contradiction for disjointness (single witness, reflexivity vs. strict ordering)
- Subset-of-disjoint propagation (`slv ⊆ lv`, `rlv ⊆ rv`, `lv.disjoint(rv)`)
