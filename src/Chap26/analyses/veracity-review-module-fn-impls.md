<style>
  body { max-width: 98%; margin: auto; font-size: 16px; }
  table { width: 100%; border-collapse: collapse; }
  th, td { padding: 4px 8px; }
</style>

# Module Function Implementations Review

## Specification Summary by Module

| Abbr | Meaning |
|------|---------|
| Tr | declared in a `trait` block |
| IT | in `impl Trait for Type` |
| IBI | in bare `impl Type` |
| ML | module-level free fn |
| V! | inside `verus!` macro |
| -V! | outside `verus!` macro |
| Unk | has requires/ensures (strength not assessed) |
| Hole | contains `assume()`, `admit()`, or `#[verifier::external_body]` |
| NoSpec | no spec |

| # | Dir | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|-----|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | Chap26 | DivConReduceMtPer | 5 | 5 | 0 | 3 | 8 | 0 | 8 | 0 | 0 |
| 2 | Chap26 | DivConReduceStPer | 5 | 5 | 0 | 0 | 5 | 0 | 5 | 0 | 0 |
| 3 | Chap26 | ETSPMtEph | 2 | 2 | 0 | 11 | 13 | 0 | 11 | 2 | 0 |
| 4 | Chap26 | ETSPStEph | 2 | 2 | 0 | 8 | 7 | 3 | 7 | 0 | 3 |
| 5 | Chap26 | MergeSortMtPer | 2 | 2 | 0 | 6 | 8 | 0 | 8 | 0 | 0 |
| 6 | Chap26 | MergeSortStPer | 2 | 2 | 0 | 1 | 3 | 0 | 3 | 0 | 0 |
| 7 | Chap26 | ScanDCMtPer | 1 | 1 | 0 | 2 | 3 | 0 | 3 | 0 | 0 |
| 8 | Chap26 | ScanDCStPer | 2 | 2 | 0 | 1 | 3 | 0 | 3 | 0 | 0 |

## Function-by-Function Detail

### Chap26/DivConReduceMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_fold_left_step` |  |  |  | Y | Y |  |  | unknown | 84&#8209;88 |
| 2 | `lemma_max_fold_left_bound` |  |  |  | Y | Y |  |  | unknown | 97&#8209;102 |
| 3 | `lemma_max_fold_left_achievable` |  |  |  | Y | Y |  |  | unknown | 122&#8209;127 |
| 4 | `max_element_parallel` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;163 |
| 5 | `sum_parallel` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;181 |
| 6 | `product_parallel` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;193 |
| 7 | `any_parallel` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;205 |
| 8 | `all_parallel` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;217 |

### Chap26/DivConReduceStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 9 | `max_element` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;67 |
| 10 | `sum` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;85 |
| 11 | `product` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;97 |
| 12 | `any` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;109 |
| 13 | `all` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;121 |

### Chap26/ETSPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `lemma_point_in_seq_transitive` |  |  |  | Y | Y |  |  | unknown | 132&#8209;138 |
| 15 | `lemma_edge_valid_transitive` |  |  |  | Y | Y |  |  | unknown | 148&#8209;160 |
| 16 | `lemma_mod_successor` |  |  |  | Y | Y |  |  | unknown | 167&#8209;169 |
| 17 | `lemma_combined_cycle` |  |  |  | Y | Y |  |  | unknown | 177&#8209;201 |
| 18 | `etsp_parallel` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;315 |
| 19 | `etsp_parallel_inner` |  |  |  | Y | Y |  |  | unknown | 330&#8209;335 |
| 20 | `sort_and_split` |  |  |  | Y | Y |  |  | unknown | 516&#8209;527 |
| 21 | `find_best_swap` |  |  |  | Y | Y |  |  | unknown | 579&#8209;585 |
| 22 | `distance` | Y | Y |  |  | Y |  |  | unknown | 608&#8209;609 |
| 23 | `point_distance` |  |  |  | Y | Y |  |  | hole | 613&#8209;614 |
| 24 | `sort_and_split_impl` |  |  |  | Y | Y |  |  | hole | 629 |
| 25 | `find_best_swap_impl` |  |  |  | Y | Y |  |  | unknown | 655&#8209;661 |
| 26 | `find_best_swap_par` |  |  |  | Y | Y |  |  | unknown | 671&#8209;681 |

### Chap26/ETSPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 27 | `lemma_point_in_seq_transitive` |  |  |  | Y | Y |  |  | unknown | 121&#8209;127 |
| 28 | `lemma_edge_valid_transitive` |  |  |  | Y | Y |  |  | unknown | 137&#8209;149 |
| 29 | `lemma_combined_cycle` |  |  |  | Y | Y |  |  | unknown | 156&#8209;180 |
| 30 | `etsp` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;299 |
| 31 | `etsp_inner` |  |  |  | Y | Y |  |  | unknown | 314&#8209;319 |
| 32 | `sort_and_split` |  |  |  | Y | Y |  |  | unknown | 486&#8209;497 |
| 33 | `find_best_swap` |  |  |  | Y | Y |  |  | unknown | 549&#8209;555 |
| 34 | `distance` | Y | Y |  |  |  | Y | Y |  | 578 |
| 35 | `sort_and_split_impl` |  |  |  | Y |  | Y | Y |  | 589&#8209;611 |
| 36 | `find_best_swap_impl` |  |  |  | Y |  | Y | Y |  | 613&#8209;632 |

### Chap26/MergeSortMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 37 | `lemma_multiset_count_positive_implies_exists` |  |  |  | Y | Y |  |  | unknown | 89&#8209;92 |
| 38 | `lemma_all_le_preserved_by_permutation` |  |  |  | Y | Y |  |  | unknown | 108&#8209;113 |
| 39 | `lemma_all_ge_preserved_by_permutation` |  |  |  | Y | Y |  |  | unknown | 125&#8209;130 |
| 40 | `lemma_sorted_concat_pivot` |  |  |  | Y | Y |  |  | unknown | 143&#8209;150 |
| 41 | `merge_parallel` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;188 |
| 42 | `merge_sort_parallel` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;198 |
| 43 | `binary_search_upper_bound` |  |  |  | Y | Y |  |  | unknown | 209&#8209;216 |
| 44 | `merge_dc` |  |  |  | Y | Y |  |  | unknown | 268&#8209;278 |

### Chap26/MergeSortStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 45 | `lemma_push_sorted` |  |  |  | Y | Y |  |  | unknown | 85&#8209;90 |
| 46 | `merge` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;125 |
| 47 | `merge_sort` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;135 |

### Chap26/ScanDCMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 48 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 75&#8209;78 |
| 49 | `prefix_sums_dc_parallel` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;102 |
| 50 | `prefix_sums_dc_inner` |  |  |  | Y | Y |  |  | unknown | 110&#8209;118 |

### Chap26/ScanDCStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 51 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 80&#8209;83 |
| 52 | `scan_dc` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;114 |
| 53 | `prefix_sums_dc` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;127 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
