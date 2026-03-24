# Agent 4 — Round 67 Report

## Assignment

Rewrite `OrderedSetStEph.rs` to use ParamBST backing store (via AVLTreeSetStEph) instead
of flat `AVLTreeSeqStEphS<T>` sequences. All ordered operations (first, last, previous,
next, split, get_range, rank, select, split_rank) rewritten to use tree operations
(min_key, max_key, split, expose, size, collect_in_order).

## Approach

Complete rewrite of OrderedSetStEph.rs (~920 lines). Key design decisions:

1. **wf includes axioms**: `spec_orderedsetsteph_wf` includes `obeys_cmp_spec::<T>()` and
   `view_ord_consistent::<T>()`, so every method with `requires self.spec_orderedsetsteph_wf()`
   gets the ordering axioms for free. Only constructors (empty, singleton, from_seq,
   from_sorted_elements) need explicit `requires` for the axioms.

2. **Postconditions use cmp_spec, not TotalOrder::le**: TotalOrder::le is an abstract spec
   function unconnectable to cmp_spec for generic T (different trait hierarchies). All
   ordering postconditions (first, last, previous, next) use `cmp_spec` directly, matching
   ParamBST ensures.

3. **cmp_spec transitivity via module-level proof helpers**: `reveal(obeys_cmp_ord)` creates
   a nested Rust item that can't access outer generic parameters (even inside `proof fn`).
   Fixed by defining local copies of BSTParaStEph's cmp lemmas (lemma_cmp_antisymmetry,
   lemma_cmp_transitivity, lemma_cmp_equal_congruent) using `reveal(obeys_cmp_ord)` without
   the `::<T>` turbofish — Verus/Rust infers the type parameter.

4. **tree_max_key helper**: Right-spine walk via expose(), with termination proof using
   `lemma_len_subset` + `lemma_set_disjoint_lens`, and ordering proof via the three
   transitivity lemmas above.

5. **tree_select helper**: Recursive BST select by rank, with strengthened ensures including
   `(i as nat) < tree@.len() ==> selected.is_some()`.

6. **get_range_iter**: Two splits + conditional inserts. Subset proof between each insert
   establishes `result_tree@ ⊆ self@`, giving size bound for the next insert's precondition.
   Two clone view assumes for k1/k2.

7. **split_iter**: Ghost `old_view` captures self@ before mutation; coverage proof via
   `old_view.remove(k@)` chain + assert-forall.

## Key Technical Findings

- **`reveal()` turbofish bug**: `reveal(fn_name::<T>)` fails with E0401 (generic parameter
  from outer item). `reveal(fn_name)` without turbofish works — Verus infers the type.
  All existing codebase uses the non-turbofish form.

- **BSTParaStEph cmp lemmas are not pub**: Can't be imported via glob. Each module needing
  them must define local copies.

- **ParamBST::insert is `&mut self`**: Returns `()`, not `Self`. All callers must use
  `tree.insert(key);` not `tree = tree.insert(key);`.

- **collect_in_order lacks membership ensures**: Only ensures `out@.len()`. Bridging to
  `result@.to_set() =~= self@` requires 2 assumes.

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 43 | OrderedSetStEph.rs | Complete rewrite for ParamBST backing |
| 2 | — | src/lib.rs | Uncommented OrderedSetStEph module |

## Holes in OrderedSetStEph.rs (5)

| # | Chap | File | Line | Type | Description |
|---|------|------|------|------|-------------|
| 1 | 43 | OrderedSetStEph.rs | 575 | assume | collect_in_order set membership |
| 2 | 43 | OrderedSetStEph.rs | 576 | assume | collect_in_order element containment |
| 3 | 43 | OrderedSetStEph.rs | 767 | assume | clone view gap (k1_clone@ == k1@) |
| 4 | 43 | OrderedSetStEph.rs | 783 | assume | clone view gap (k2_clone@ == k2@) |
| 5 | 43 | OrderedSetStEph.rs | 949 | unsafe | iterator next() standard pattern |

## Results

| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| Verified | 4385 | 4392 | +7 |
| Errors | 0 | 0 | 0 |
| Warnings | 0 | 0 | 0 |
| Chap43 holes | 47 | 52 | +5 |
| Global holes | 47 | 52 | +5 |
| Clean chapters | 45 | 45 | 0 |
| Holed chapters | 1 | 1 | 0 |
| RTT | 2528 | 2528 | 0 |
| PTT | 147 | 147 | 0 |

The +5 holes are expected: OrderedSetStEph.rs was previously commented out (0 holes counted)
and is now active with 5 holes. No regressions in other files.

## Techniques Used

- cmp_spec transitivity lemmas (reveal without turbofish)
- Ghost old_view for capturing pre-mutation state
- Subset chain proofs: mid@ ⊂ right1@ ⊂ self@.remove(k1@) ⊂ self@
- lemma_len_subset for size bounds after subset proofs
- lemma_set_disjoint_lens for termination proofs in tree walks
