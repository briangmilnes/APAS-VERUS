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
| 1 | Chap45 | BalancedTreePQ | 26 | 28 | 0 | 1 | 29 | 0 | 28 | 0 | 1 |
| 2 | Chap45 | BinaryHeapPQ | 18 | 20 | 0 | 17 | 37 | 0 | 36 | 0 | 1 |
| 3 | Chap45 | Example45_2 | 8 | 8 | 0 | 9 | 9 | 0 | 0 | 0 | 9 |
| 4 | Chap45 | HeapsortExample | 2 | 3 | 0 | 22 | 5 | 20 | 1 | 0 | 24 |
| 5 | Chap45 | LeftistHeapPQ | 24 | 27 | 0 | 7 | 31 | 3 | 29 | 0 | 5 |
| 6 | Chap45 | SortedListPQ | 19 | 21 | 0 | 2 | 23 | 0 | 21 | 0 | 2 |
| 7 | Chap45 | UnsortedListPQ | 15 | 17 | 0 | 1 | 18 | 0 | 16 | 0 | 2 |

## Function-by-Function Detail

### Chap45/BalancedTreePQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `_balanced_tree_pq_verified` |  |  |  | Y | Y |  | Y |  | 74 |
| 2 | `empty` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 3 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;89 |
| 4 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;97 |
| 5 | `insert` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;108 |
| 6 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;119 |
| 7 | `meld` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;128 |
| 8 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 9 | `size` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 10 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;144 |
| 11 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;149 |
| 12 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;157 |
| 13 | `delete_max` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;167 |
| 14 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;175 |
| 15 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;180 |
| 16 | `contains` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;185 |
| 17 | `remove` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;193 |
| 18 | `range` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;198 |
| 19 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;203 |
| 20 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;208 |
| 21 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;213 |
| 22 | `is_sorted` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;218 |
| 23 | `height` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;223 |
| 24 | `split` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;231 |
| 25 | `join` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;239 |
| 26 | `filter` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;249 |
| 27 | `map` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;255 |
| 28 | `default` |  | Y |  |  | Y |  |  | unknown | 833&#8209;834 |
| 29 | `eq` |  | Y |  |  | Y |  |  | unknown | 855&#8209;856 |

### Chap45/BinaryHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 30 | `lemma_log2_bound` |  |  |  | Y | Y |  |  | unknown | 93&#8209;100 |
| 31 | `lemma_heap_root_le_all` |  |  |  | Y | Y |  |  | unknown | 116&#8209;123 |
| 32 | `lemma_heap_parent_le` |  |  |  | Y | Y |  |  | unknown | 143&#8209;149 |
| 33 | `lemma_swap_preserves_multiset` |  |  |  | Y | Y |  |  | unknown | 162&#8209;164 |
| 34 | `lemma_le_preserved_by_multiset_eq` |  |  |  | Y | Y |  |  | unknown | 188&#8209;197 |
| 35 | `empty` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;234 |
| 36 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;243 |
| 37 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;254 |
| 38 | `insert` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;266 |
| 39 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 270&#8209;287 |
| 40 | `meld` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;298 |
| 41 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 302&#8209;307 |
| 42 | `size` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;311 |
| 43 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 314&#8209;315 |
| 44 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 318&#8209;320 |
| 45 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;328 |
| 46 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;338 |
| 47 | `is_valid_heap` | Y | Y |  |  | Y |  |  | unknown | 341&#8209;342 |
| 48 | `height` | Y | Y |  |  | Y |  |  | unknown | 345&#8209;347 |
| 49 | `level_elements` | Y | Y |  |  | Y |  |  | unknown | 350&#8209;354 |
| 50 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 357&#8209;361 |
| 51 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 364&#8209;366 |
| 52 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 369&#8209;376 |
| 53 | `left_child` |  |  |  | Y | Y |  |  | unknown | 383&#8209;385 |
| 54 | `right_child` |  |  |  | Y | Y |  |  | unknown | 391&#8209;393 |
| 55 | `parent` |  |  |  | Y | Y |  |  | unknown | 399&#8209;401 |
| 56 | `swap_elements` |  |  |  | Y | Y |  |  | unknown | 407&#8209;415 |
| 57 | `bubble_up` |  |  |  | Y | Y |  |  | unknown | 478&#8209;485 |
| 58 | `bubble_down` |  |  |  | Y | Y |  |  | unknown | 533&#8209;541 |
| 59 | `bubble_down_heap` |  |  |  | Y | Y |  |  | unknown | 599&#8209;608 |
| 60 | `bubble_up_heap` |  |  |  | Y | Y |  |  | unknown | 783&#8209;808 |
| 61 | `heapify` |  |  |  | Y | Y |  |  | unknown | 978&#8209;986 |
| 62 | `is_heap` |  |  |  | Y | Y |  |  | unknown | 1030&#8209;1032 |
| 63 | `exec_pow2` |  |  |  | Y | Y |  |  | unknown | 1061&#8209;1063 |
| 64 | `exec_log2` |  |  |  | Y | Y |  |  | unknown | 1094&#8209;1097 |
| 65 | `default` |  | Y |  |  | Y |  | Y |  | 1831 |
| 66 | `eq` |  | Y |  |  | Y |  |  | unknown | 1860&#8209;1861 |

### Chap45/Example45_2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 67 | `_example_45_2_verified` |  |  |  | Y | Y |  | Y |  | 28 |
| 68 | `example_45_2_textbook_example` | Y | Y |  | Y | Y |  | Y |  | 33 |
| 69 | `example_45_2_reverse_sorted` | Y | Y |  | Y | Y |  | Y |  | 34 |
| 70 | `example_45_2_already_sorted` | Y | Y |  | Y | Y |  | Y |  | 35 |
| 71 | `example_45_2_duplicates` | Y | Y |  | Y | Y |  | Y |  | 36 |
| 72 | `example_45_2_single_element` | Y | Y |  | Y | Y |  | Y |  | 37 |
| 73 | `example_45_2_empty` | Y | Y |  | Y | Y |  | Y |  | 38 |
| 74 | `example_45_2_efficiency_demonstration` | Y | Y |  | Y | Y |  | Y |  | 39 |
| 75 | `run_example_45_2` | Y | Y |  | Y | Y |  | Y |  | 40 |

### Chap45/HeapsortExample.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 76 | `eq` |  | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 77 | `_heapsort_example_verified` |  |  |  | Y | Y |  | Y |  | 146 |
| 78 | `is_vec_sorted_exec` |  |  |  | Y | Y |  | Y |  | 149 |
| 79 | `all_results_match` | Y | Y |  |  | Y |  | Y |  | 170 |
| 80 | `all_results_sorted` | Y | Y |  |  | Y |  | Y |  | 172 |
| 81 | `heapsort_unsorted_list` |  |  |  | Y |  | Y | Y |  | 195&#8209;211 |
| 82 | `heapsort_sorted_list` |  |  |  | Y |  | Y | Y |  | 213&#8209;229 |
| 83 | `heapsort_balanced_tree` |  |  |  | Y |  | Y | Y |  | 231&#8209;247 |
| 84 | `heapsort_binary_heap` |  |  |  | Y |  | Y | Y |  | 249&#8209;265 |
| 85 | `heapsort_leftist_heap` |  |  |  | Y |  | Y | Y |  | 267&#8209;283 |
| 86 | `compare_all_heapsorts` |  |  |  | Y |  | Y | Y |  | 285&#8209;295 |
| 87 | `textbook_example` |  |  |  | Y |  | Y | Y |  | 322&#8209;326 |
| 88 | `reverse_sorted_example` |  |  |  | Y |  | Y | Y |  | 328&#8209;332 |
| 89 | `already_sorted_example` |  |  |  | Y |  | Y | Y |  | 334&#8209;338 |
| 90 | `duplicates_example` |  |  |  | Y |  | Y | Y |  | 340&#8209;344 |
| 91 | `single_element_example` |  |  |  | Y |  | Y | Y |  | 346&#8209;350 |
| 92 | `empty_example` |  |  |  | Y |  | Y | Y |  | 352&#8209;356 |
| 93 | `large_example` |  |  |  | Y |  | Y | Y |  | 358&#8209;368 |
| 94 | `efficiency_demonstration` |  |  |  | Y |  | Y | Y |  | 370&#8209;379 |
| 95 | `complexity_analysis` |  |  |  | Y |  | Y | Y |  | 381&#8209;410 |
| 96 | `correctness_verification` |  |  |  | Y |  | Y | Y |  | 412&#8209;426 |
| 97 | `vec_to_array_seq` |  |  |  | Y |  | Y | Y |  | 428&#8209;436 |
| 98 | `vec_to_avl_seq` |  |  |  | Y |  | Y | Y |  | 438&#8209;439 |
| 99 | `is_sorted` |  |  |  | Y |  | Y | Y |  | 441&#8209;442 |
| 100 | `generate_test_sequences` |  |  |  | Y |  | Y | Y |  | 444&#8209;459 |

### Chap45/LeftistHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 101 | `rank` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;107 |
| 102 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;124 |
| 103 | `meld_nodes` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;139 |
| 104 | `size` x3 | Y | Y |  |  | Y |  |  | unknown | 666&#8209;668 |
| 105 | `height` x3 | Y | Y |  |  | Y |  |  | unknown | 681&#8209;683 |
| 106 | `is_leftist` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;152 |
| 107 | `is_heap` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;155 |
| 108 | `is_rank_bounded` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;159 |
| 109 | `to_vec` x3 | Y | Y |  |  | Y |  |  | unknown | 698&#8209;700 |
| 110 | `_leftist_heap_pq_verified` |  |  |  | Y | Y |  | Y |  | 503 |
| 111 | `lemma_total_size_monotone` |  |  |  | Y | Y |  |  | unknown | 505&#8209;508 |
| 112 | `lemma_heap_root_is_min` |  |  |  | Y | Y |  |  | unknown | 516&#8209;522 |
| 113 | `lemma_rank_le_size` |  |  |  | Y | Y |  |  | unknown | 579&#8209;581 |
| 114 | `empty` | Y | Y |  |  | Y |  |  | unknown | 601&#8209;605 |
| 115 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 607&#8209;611 |
| 116 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 613&#8209;620 |
| 117 | `insert` | Y | Y |  |  | Y |  |  | unknown | 623&#8209;630 |
| 118 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 633&#8209;646 |
| 119 | `meld` | Y | Y |  |  | Y |  |  | unknown | 649&#8209;657 |
| 120 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 660&#8209;664 |
| 121 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 670&#8209;671 |
| 122 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 673&#8209;679 |
| 123 | `root_rank` | Y | Y |  |  | Y |  |  | unknown | 685&#8209;686 |
| 124 | `is_valid_leftist_heap` | Y | Y |  |  | Y |  |  | unknown | 688&#8209;690 |
| 125 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 692&#8209;696 |
| 126 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 702&#8209;708 |
| 127 | `meld_multiple` | Y | Y |  |  | Y |  |  | unknown | 712&#8209;719 |
| 128 | `split` | Y | Y |  |  | Y |  |  | unknown | 721&#8209;725 |
| 129 | `total_order_le` |  |  |  | Y | Y |  |  | unknown | 734&#8209;735 |
| 130 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 1205&#8209;1208 |
| 131 | `default` |  | Y |  |  | Y |  | Y |  | 1237 |
| 132 | `efficient_multi_way_merge` |  |  |  | Y |  | Y | Y |  | 1283&#8209;1290 |
| 133 | `parallel_heap_construction` |  |  |  | Y |  | Y | Y |  | 1292&#8209;1295 |
| 134 | `format_node` x2 |  | Y |  |  |  | Y | Y |  | 1312&#8209;1322 |

### Chap45/SortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 135 | `_sorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 74 |
| 136 | `lemma_append_push_bridge` |  |  |  | Y | Y |  |  | unknown | 80&#8209;101 |
| 137 | `lemma_push_preserves_sorted` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;150 |
| 138 | `empty` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;158 |
| 139 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;167 |
| 140 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;174 |
| 141 | `insert` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;186 |
| 142 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;201 |
| 143 | `meld` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;214 |
| 144 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;223 |
| 145 | `size` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;227 |
| 146 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;231 |
| 147 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;236 |
| 148 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;247 |
| 149 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;256 |
| 150 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;263 |
| 151 | `delete_max` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;277 |
| 152 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;285 |
| 153 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;290 |
| 154 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;299 |
| 155 | `is_sorted` | Y | Y |  |  | Y |  |  | unknown | 302&#8209;305 |
| 156 | `default` |  | Y |  |  | Y |  | Y |  | 1240 |
| 157 | `eq` |  | Y |  |  | Y |  |  | unknown | 1267&#8209;1268 |

### Chap45/UnsortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 158 | `_unsorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 74 |
| 159 | `empty` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;91 |
| 160 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;99 |
| 161 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;108 |
| 162 | `insert` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;118 |
| 163 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;140 |
| 164 | `meld` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;150 |
| 165 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;156 |
| 166 | `size` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;160 |
| 167 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 168 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;169 |
| 169 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;178 |
| 170 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;187 |
| 171 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;192 |
| 172 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;197 |
| 173 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;206 |
| 174 | `default` |  | Y |  |  | Y |  | Y |  | 830 |
| 175 | `eq` |  | Y |  |  | Y |  |  | unknown | 858&#8209;859 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
