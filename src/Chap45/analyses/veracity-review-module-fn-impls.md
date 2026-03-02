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
| 1 | Chap45 | BalancedTreePQ | 26 | 28 | 0 | 1 | 2 | 27 | 1 | 0 | 28 |
| 2 | Chap45 | BinaryHeapPQ | 18 | 20 | 0 | 11 | 31 | 0 | 30 | 0 | 1 |
| 3 | Chap45 | Example45_2 | 8 | 0 | 0 | 9 | 1 | 8 | 0 | 0 | 9 |
| 4 | Chap45 | HeapsortExample | 2 | 4 | 0 | 21 | 2 | 22 | 1 | 0 | 23 |
| 5 | Chap45 | LeftistHeapPQ | 23 | 26 | 0 | 3 | 26 | 3 | 3 | 3 | 23 |
| 6 | Chap45 | SortedListPQ | 18 | 20 | 0 | 1 | 21 | 0 | 18 | 0 | 3 |
| 7 | Chap45 | UnsortedListPQ | 15 | 17 | 0 | 1 | 18 | 0 | 16 | 0 | 2 |

## Function-by-Function Detail

### Chap45/BalancedTreePQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `_balanced_tree_pq_verified` |  |  |  | Y | Y |  | Y |  | 42 |
| 2 | `eq` |  | Y |  |  | Y |  |  | unknown | 60&#8209;61 |
| 3 | `empty` | Y | Y |  |  |  | Y | Y |  | 76 |
| 4 | `singleton` | Y | Y |  |  |  | Y | Y |  | 77 |
| 5 | `find_min` | Y | Y |  |  |  | Y | Y |  | 78 |
| 6 | `insert` | Y | Y |  |  |  | Y | Y |  | 79 |
| 7 | `delete_min` | Y | Y |  |  |  | Y | Y |  | 80 |
| 8 | `meld` | Y | Y |  |  |  | Y | Y |  | 81 |
| 9 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 82 |
| 10 | `size` | Y | Y |  |  |  | Y | Y |  | 83 |
| 11 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 84 |
| 12 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 85 |
| 13 | `find_max` | Y | Y |  |  |  | Y | Y |  | 86 |
| 14 | `delete_max` | Y | Y |  |  |  | Y | Y |  | 87 |
| 15 | `insert_all` | Y | Y |  |  |  | Y | Y |  | 88 |
| 16 | `extract_all_sorted` | Y | Y |  |  |  | Y | Y |  | 89 |
| 17 | `contains` | Y | Y |  |  |  | Y | Y |  | 90 |
| 18 | `remove` | Y | Y |  |  |  | Y | Y |  | 91 |
| 19 | `range` | Y | Y |  |  |  | Y | Y |  | 92 |
| 20 | `from_vec` | Y | Y |  |  |  | Y | Y |  | 93 |
| 21 | `to_vec` | Y | Y |  |  |  | Y | Y |  | 94 |
| 22 | `to_sorted_vec` | Y | Y |  |  |  | Y | Y |  | 95 |
| 23 | `is_sorted` | Y | Y |  |  |  | Y | Y |  | 96 |
| 24 | `height` | Y | Y |  |  |  | Y | Y |  | 97 |
| 25 | `split` | Y | Y |  |  |  | Y | Y |  | 98 |
| 26 | `join` | Y | Y |  |  |  | Y | Y |  | 99 |
| 27 | `filter` | Y | Y |  |  |  | Y | Y |  | 104 |
| 28 | `map` | Y | Y |  |  |  | Y | Y |  | 105 |
| 29 | `default` |  | Y |  |  |  | Y | Y |  | 361 |

### Chap45/BinaryHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 30 | `eq` |  | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 31 | `left_child` |  |  |  | Y | Y |  |  | unknown | 76&#8209;78 |
| 32 | `right_child` |  |  |  | Y | Y |  |  | unknown | 82&#8209;84 |
| 33 | `parent` |  |  |  | Y | Y |  |  | unknown | 88&#8209;89 |
| 34 | `swap_elements` |  |  |  | Y | Y |  |  | unknown | 105&#8209;111 |
| 35 | `bubble_up` |  |  |  | Y | Y |  |  | unknown | 139&#8209;144 |
| 36 | `bubble_down` |  |  |  | Y | Y |  |  | unknown | 179&#8209;185 |
| 37 | `heapify` |  |  |  | Y | Y |  |  | unknown | 230&#8209;235 |
| 38 | `is_heap` |  |  |  | Y | Y |  |  | unknown | 261&#8209;262 |
| 39 | `exec_pow2` |  |  |  | Y | Y |  |  | unknown | 301&#8209;303 |
| 40 | `lemma_log2_bound` |  |  |  | Y | Y |  |  | unknown | 328&#8209;335 |
| 41 | `exec_log2` |  |  |  | Y | Y |  |  | unknown | 348&#8209;351 |
| 42 | `empty` | Y | Y |  |  | Y |  |  | unknown | 373&#8209;374 |
| 43 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 376&#8209;378 |
| 44 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 380&#8209;383 |
| 45 | `insert` | Y | Y |  |  | Y |  |  | unknown | 385&#8209;390 |
| 46 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 392&#8209;400 |
| 47 | `meld` | Y | Y |  |  | Y |  |  | unknown | 402&#8209;408 |
| 48 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 410&#8209;415 |
| 49 | `size` | Y | Y |  |  | Y |  |  | unknown | 417&#8209;418 |
| 50 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 420&#8209;421 |
| 51 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 423&#8209;424 |
| 52 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 426&#8209;430 |
| 53 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 432&#8209;435 |
| 54 | `is_valid_heap` | Y | Y |  |  | Y |  |  | unknown | 437&#8209;438 |
| 55 | `height` | Y | Y |  |  | Y |  |  | unknown | 440&#8209;442 |
| 56 | `level_elements` | Y | Y |  |  | Y |  |  | unknown | 444&#8209;448 |
| 57 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 450&#8209;453 |
| 58 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 455&#8209;456 |
| 59 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 458&#8209;461 |
| 60 | `default` |  | Y |  |  | Y |  | Y |  | 689 |

### Chap45/Example45_2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 61 | `_example_45_2_verified` |  |  |  | Y | Y |  | Y |  | 12 |
| 62 | `example_45_2_textbook_example` | Y |  |  | Y |  | Y | Y |  | 17&#8209;19 |
| 63 | `example_45_2_reverse_sorted` | Y |  |  | Y |  | Y | Y |  | 21&#8209;23 |
| 64 | `example_45_2_already_sorted` | Y |  |  | Y |  | Y | Y |  | 25&#8209;27 |
| 65 | `example_45_2_duplicates` | Y |  |  | Y |  | Y | Y |  | 29&#8209;31 |
| 66 | `example_45_2_single_element` | Y |  |  | Y |  | Y | Y |  | 33&#8209;35 |
| 67 | `example_45_2_empty` | Y |  |  | Y |  | Y | Y |  | 37&#8209;39 |
| 68 | `example_45_2_efficiency_demonstration` | Y |  |  | Y |  | Y | Y |  | 41&#8209;43 |
| 69 | `run_example_45_2` | Y |  |  | Y |  | Y | Y |  | 45&#8209;47 |

### Chap45/HeapsortExample.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 70 | `_heapsort_example_verified` |  |  |  | Y | Y |  | Y |  | 33 |
| 71 | `eq` |  | Y |  |  | Y |  |  | unknown | 90&#8209;91 |
| 72 | `heapsort_unsorted_list` |  |  |  | Y |  | Y | Y |  | 107&#8209;123 |
| 73 | `heapsort_sorted_list` |  |  |  | Y |  | Y | Y |  | 125&#8209;141 |
| 74 | `heapsort_balanced_tree` |  |  |  | Y |  | Y | Y |  | 143&#8209;159 |
| 75 | `heapsort_binary_heap` |  |  |  | Y |  | Y | Y |  | 161&#8209;177 |
| 76 | `heapsort_leftist_heap` |  |  |  | Y |  | Y | Y |  | 179&#8209;195 |
| 77 | `compare_all_heapsorts` |  |  |  | Y |  | Y | Y |  | 197&#8209;207 |
| 78 | `all_results_match` | Y | Y |  |  |  | Y | Y |  | 223&#8209;224 |
| 79 | `all_results_sorted` | Y | Y |  |  |  | Y | Y |  | 225&#8209;226 |
| 80 | `is_sorted` |  | Y |  | Y |  | Y | Y |  | 239 |
| 81 | `textbook_example` |  |  |  | Y |  | Y | Y |  | 249&#8209;253 |
| 82 | `reverse_sorted_example` |  |  |  | Y |  | Y | Y |  | 255&#8209;259 |
| 83 | `already_sorted_example` |  |  |  | Y |  | Y | Y |  | 261&#8209;265 |
| 84 | `duplicates_example` |  |  |  | Y |  | Y | Y |  | 267&#8209;271 |
| 85 | `single_element_example` |  |  |  | Y |  | Y | Y |  | 273&#8209;277 |
| 86 | `empty_example` |  |  |  | Y |  | Y | Y |  | 279&#8209;283 |
| 87 | `large_example` |  |  |  | Y |  | Y | Y |  | 285&#8209;295 |
| 88 | `efficiency_demonstration` |  |  |  | Y |  | Y | Y |  | 297&#8209;306 |
| 89 | `complexity_analysis` |  |  |  | Y |  | Y | Y |  | 308&#8209;337 |
| 90 | `correctness_verification` |  |  |  | Y |  | Y | Y |  | 339&#8209;353 |
| 91 | `vec_to_array_seq` |  |  |  | Y |  | Y | Y |  | 355&#8209;363 |
| 92 | `vec_to_avl_seq` |  |  |  | Y |  | Y | Y |  | 365&#8209;366 |
| 93 | `generate_test_sequences` |  |  |  | Y |  | Y | Y |  | 371&#8209;386 |

### Chap45/LeftistHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 94 | `_leftist_heap_pq_verified` |  |  |  | Y | Y |  | Y |  | 59 |
| 95 | `rank` | Y | Y |  |  | Y |  | Y |  | 63 |
| 96 | `make_node` | Y | Y |  |  | Y |  |  | hole | 64 |
| 97 | `meld_nodes` | Y | Y |  |  | Y |  | Y |  | 65 |
| 98 | `size` x3 | Y | Y |  |  | Y |  |  | hole | 83 |
| 99 | `height` x3 | Y | Y |  |  | Y |  |  | hole | 86 |
| 100 | `is_leftist` | Y | Y |  |  | Y |  | Y |  | 68 |
| 101 | `is_heap` | Y | Y |  |  | Y |  | Y |  | 69 |
| 102 | `to_vec` x3 | Y | Y |  |  | Y |  | Y |  | 91 |
| 103 | `empty` | Y | Y |  |  | Y |  | Y |  | 75 |
| 104 | `singleton` | Y | Y |  |  | Y |  | Y |  | 76 |
| 105 | `find_min` | Y | Y |  |  | Y |  | Y |  | 77 |
| 106 | `insert` | Y | Y |  |  | Y |  | Y |  | 78 |
| 107 | `delete_min` | Y | Y |  |  | Y |  | Y |  | 79 |
| 108 | `meld` | Y | Y |  |  | Y |  | Y |  | 80 |
| 109 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 110 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 84 |
| 111 | `extract_all_sorted` | Y | Y |  |  | Y |  | Y |  | 85 |
| 112 | `root_rank` | Y | Y |  |  | Y |  | Y |  | 87 |
| 113 | `is_valid_leftist_heap` | Y | Y |  |  | Y |  | Y |  | 88 |
| 114 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 115 | `to_sorted_vec` | Y | Y |  |  | Y |  | Y |  | 92 |
| 116 | `meld_multiple` | Y | Y |  |  | Y |  | Y |  | 93 |
| 117 | `split` | Y | Y |  |  | Y |  | Y |  | 94 |
| 118 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 400&#8209;402 |
| 119 | `default` |  | Y |  |  | Y |  | Y |  | 448 |
| 120 | `format_node` |  | Y |  |  |  | Y | Y |  | 484&#8209;495 |
| 121 | `efficient_multi_way_merge` |  |  |  | Y |  | Y | Y |  | 502&#8209;509 |
| 122 | `parallel_heap_construction` |  |  |  | Y |  | Y | Y |  | 511&#8209;514 |

### Chap45/SortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 123 | `_sorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 41 |
| 124 | `empty` | Y | Y |  |  | Y |  |  | unknown | 48&#8209;49 |
| 125 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;53 |
| 126 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;58 |
| 127 | `insert` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;64 |
| 128 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;72 |
| 129 | `meld` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;78 |
| 130 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 131 | `size` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 132 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 133 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 134 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;97 |
| 135 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |
| 136 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;106 |
| 137 | `delete_max` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;114 |
| 138 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 139 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 140 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 141 | `is_sorted` | Y | Y |  |  | Y |  | Y |  | 125 |
| 142 | `eq` |  | Y |  |  | Y |  |  | unknown | 412&#8209;413 |
| 143 | `default` |  | Y |  |  | Y |  | Y |  | 424 |

### Chap45/UnsortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 144 | `_unsorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 41 |
| 145 | `empty` | Y | Y |  |  | Y |  |  | unknown | 48&#8209;49 |
| 146 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;53 |
| 147 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;58 |
| 148 | `insert` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;64 |
| 149 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;72 |
| 150 | `meld` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;78 |
| 151 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 152 | `size` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 153 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 154 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 155 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;97 |
| 156 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;103 |
| 157 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;106 |
| 158 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 159 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;114 |
| 160 | `eq` |  | Y |  |  | Y |  |  | unknown | 314&#8209;315 |
| 161 | `default` |  | Y |  |  | Y |  | Y |  | 326 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
