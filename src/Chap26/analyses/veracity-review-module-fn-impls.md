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
| 1 | Chap26 | DivConReduceMtPer | 5 | 5 | 0 | 8 | 13 | 0 | 13 | 0 | 0 |
| 2 | Chap26 | DivConReduceStPer | 5 | 5 | 0 | 0 | 5 | 0 | 5 | 0 | 0 |
| 3 | Chap26 | ETSPMtEph | 2 | 2 | 0 | 12 | 14 | 0 | 12 | 1 | 1 |
| 4 | Chap26 | ETSPStEph | 2 | 2 | 0 | 9 | 8 | 3 | 8 | 0 | 3 |
| 5 | Chap26 | MergeSortMtPer | 2 | 2 | 0 | 6 | 8 | 0 | 8 | 0 | 0 |
| 6 | Chap26 | MergeSortStPer | 2 | 2 | 0 | 1 | 3 | 0 | 3 | 0 | 0 |
| 7 | Chap26 | ScanDCMtPer | 1 | 1 | 0 | 2 | 3 | 0 | 3 | 0 | 0 |
| 8 | Chap26 | ScanDCStPer | 2 | 2 | 0 | 1 | 3 | 0 | 3 | 0 | 0 |

## Function-by-Function Detail

### Chap26/DivConReduceMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_fold_left_step` |  |  |  | Y | Y |  |  | unknown | 78&#8209;82 |
| 2 | `lemma_max_fold_left_bound` |  |  |  | Y | Y |  |  | unknown | 92&#8209;97 |
| 3 | `lemma_max_fold_left_achievable` |  |  |  | Y | Y |  |  | unknown | 119&#8209;124 |
| 4 | `max_element_parallel` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;161 |
| 5 | `sum_parallel` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;179 |
| 6 | `product_parallel` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;191 |
| 7 | `any_parallel` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;203 |
| 8 | `all_parallel` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;215 |
| 9 | `call_reduce_max` |  |  |  | Y | Y |  |  | unknown | 224&#8209;227 |
| 10 | `call_reduce_sum` |  |  |  | Y | Y |  |  | unknown | 236&#8209;239 |
| 11 | `call_reduce_product` |  |  |  | Y | Y |  |  | unknown | 248&#8209;251 |
| 12 | `call_reduce_or` |  |  |  | Y | Y |  |  | unknown | 260&#8209;263 |
| 13 | `call_reduce_and` |  |  |  | Y | Y |  |  | unknown | 272&#8209;275 |

### Chap26/DivConReduceStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `max_element` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 15 | `sum` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;93 |
| 16 | `product` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;105 |
| 17 | `any` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;117 |
| 18 | `all` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;129 |

### Chap26/ETSPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 19 | `distance` | Y | Y |  |  | Y |  | Y |  | 337 |
| 20 | `lemma_next_edge_from_eq` |  |  |  | Y | Y |  |  | unknown | 169&#8209;174 |
| 21 | `lemma_point_in_seq_transitive` |  |  |  | Y | Y |  |  | unknown | 180&#8209;186 |
| 22 | `lemma_edge_valid_transitive` |  |  |  | Y | Y |  |  | unknown | 195&#8209;207 |
| 23 | `lemma_mod_successor` |  |  |  | Y | Y |  |  | unknown | 214&#8209;216 |
| 24 | `lemma_combined_cycle` |  |  |  | Y | Y |  |  | unknown | 225&#8209;249 |
| 25 | `etsp_parallel` | Y | Y |  |  | Y |  |  | unknown | 328&#8209;332 |
| 26 | `etsp_parallel_inner` |  |  |  | Y | Y |  |  | unknown | 349&#8209;354 |
| 27 | `sort_and_split` |  |  |  | Y | Y |  |  | unknown | 530&#8209;541 |
| 28 | `find_best_swap` |  |  |  | Y | Y |  |  | unknown | 591&#8209;597 |
| 29 | `point_distance` |  |  |  | Y | Y |  |  | unknown | 604&#8209;605 |
| 30 | `sort_and_split_impl` |  |  |  | Y | Y |  |  | hole | 618 |
| 31 | `find_best_swap_impl` |  |  |  | Y | Y |  |  | unknown | 645&#8209;651 |
| 32 | `find_best_swap_par` |  |  |  | Y | Y |  |  | unknown | 662&#8209;672 |

### Chap26/ETSPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 33 | `lemma_next_edge_from_eq` |  |  |  | Y | Y |  |  | unknown | 140&#8209;145 |
| 34 | `lemma_point_in_seq_transitive` |  |  |  | Y | Y |  |  | unknown | 151&#8209;157 |
| 35 | `lemma_edge_valid_transitive` |  |  |  | Y | Y |  |  | unknown | 167&#8209;179 |
| 36 | `lemma_combined_cycle` |  |  |  | Y | Y |  |  | unknown | 186&#8209;210 |
| 37 | `etsp` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;298 |
| 38 | `etsp_inner` |  |  |  | Y | Y |  |  | unknown | 312&#8209;317 |
| 39 | `sort_and_split` |  |  |  | Y | Y |  |  | unknown | 506&#8209;517 |
| 40 | `find_best_swap` |  |  |  | Y | Y |  |  | unknown | 571&#8209;577 |
| 41 | `distance` | Y | Y |  |  |  | Y | Y |  | 606 |
| 42 | `sort_and_split_impl` |  |  |  | Y |  | Y | Y |  | 609&#8209;631 |
| 43 | `find_best_swap_impl` |  |  |  | Y |  | Y | Y |  | 633&#8209;652 |

### Chap26/MergeSortMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 44 | `lemma_multiset_count_positive_implies_exists` |  |  |  | Y | Y |  |  | unknown | 83&#8209;86 |
| 45 | `lemma_all_le_preserved_by_permutation` |  |  |  | Y | Y |  |  | unknown | 106&#8209;111 |
| 46 | `lemma_all_ge_preserved_by_permutation` |  |  |  | Y | Y |  |  | unknown | 126&#8209;131 |
| 47 | `lemma_sorted_concat_pivot` |  |  |  | Y | Y |  |  | unknown | 147&#8209;154 |
| 48 | `merge_parallel` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;194 |
| 49 | `merge_sort_parallel` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;204 |
| 50 | `binary_search_upper_bound` |  |  |  | Y | Y |  |  | unknown | 213&#8209;220 |
| 51 | `merge_dc` |  |  |  | Y | Y |  |  | unknown | 278&#8209;288 |

### Chap26/MergeSortStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 52 | `lemma_push_sorted` |  |  |  | Y | Y |  |  | unknown | 79&#8209;84 |
| 53 | `merge` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;118 |
| 54 | `merge_sort` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;128 |

### Chap26/ScanDCMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 55 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 82&#8209;85 |
| 56 | `prefix_sums_dc_parallel` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;110 |
| 57 | `prefix_sums_dc_inner` |  |  |  | Y | Y |  |  | unknown | 119&#8209;127 |

### Chap26/ScanDCStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 58 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 74&#8209;77 |
| 59 | `scan_dc` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;106 |
| 60 | `prefix_sums_dc` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;119 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
