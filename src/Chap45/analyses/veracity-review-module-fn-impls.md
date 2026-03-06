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
| 1 | Chap45 | BalancedTreePQ | 26 | 28 | 0 | 1 | 29 | 0 | 27 | 0 | 2 |
| 2 | Chap45 | BinaryHeapPQ | 18 | 20 | 0 | 11 | 31 | 0 | 30 | 0 | 1 |
| 3 | Chap45 | Example45_2 | 8 | 8 | 0 | 9 | 9 | 0 | 8 | 0 | 1 |
| 4 | Chap45 | HeapsortExample | 2 | 3 | 0 | 21 | 4 | 20 | 1 | 0 | 23 |
| 5 | Chap45 | LeftistHeapPQ | 23 | 26 | 0 | 3 | 26 | 3 | 20 | 4 | 5 |
| 6 | Chap45 | SortedListPQ | 18 | 20 | 0 | 1 | 21 | 0 | 19 | 0 | 2 |
| 7 | Chap45 | UnsortedListPQ | 15 | 17 | 0 | 1 | 18 | 0 | 16 | 0 | 2 |

## Function-by-Function Detail

### Chap45/BalancedTreePQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `_balanced_tree_pq_verified` |  |  |  | Y | Y |  | Y |  | 41 |
| 2 | `empty` | Y | Y |  |  | Y |  |  | unknown | 46&#8209;47 |
| 3 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;50 |
| 4 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;55 |
| 5 | `insert` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;58 |
| 6 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;65 |
| 7 | `meld` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 8 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 9 | `size` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 10 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 11 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 12 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;85 |
| 13 | `delete_max` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;92 |
| 14 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;95 |
| 15 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |
| 16 | `contains` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;101 |
| 17 | `remove` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;106 |
| 18 | `range` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 19 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 20 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 21 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 22 | `is_sorted` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 23 | `height` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;124 |
| 24 | `split` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;127 |
| 25 | `join` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;130 |
| 26 | `filter` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;136 |
| 27 | `map` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;138 |
| 28 | `default` |  | Y |  |  | Y |  | Y |  | 366 |
| 29 | `eq` |  | Y |  |  | Y |  |  | unknown | 416&#8209;417 |

### Chap45/BinaryHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 30 | `lemma_log2_bound` |  |  |  | Y | Y |  |  | unknown | 90&#8209;97 |
| 31 | `empty` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 32 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 33 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;130 |
| 34 | `insert` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;137 |
| 35 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;147 |
| 36 | `meld` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;155 |
| 37 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;162 |
| 38 | `size` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;165 |
| 39 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;168 |
| 40 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;171 |
| 41 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;177 |
| 42 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;182 |
| 43 | `is_valid_heap` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;185 |
| 44 | `height` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;189 |
| 45 | `level_elements` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;195 |
| 46 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;200 |
| 47 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;203 |
| 48 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;208 |
| 49 | `left_child` |  |  |  | Y | Y |  |  | unknown | 220&#8209;222 |
| 50 | `right_child` |  |  |  | Y | Y |  |  | unknown | 227&#8209;229 |
| 51 | `parent` |  |  |  | Y | Y |  |  | unknown | 234&#8209;235 |
| 52 | `swap_elements` |  |  |  | Y | Y |  |  | unknown | 240&#8209;246 |
| 53 | `bubble_up` |  |  |  | Y | Y |  |  | unknown | 274&#8209;279 |
| 54 | `bubble_down` |  |  |  | Y | Y |  |  | unknown | 314&#8209;320 |
| 55 | `heapify` |  |  |  | Y | Y |  |  | unknown | 365&#8209;370 |
| 56 | `is_heap` |  |  |  | Y | Y |  |  | unknown | 396&#8209;397 |
| 57 | `exec_pow2` |  |  |  | Y | Y |  |  | unknown | 423&#8209;425 |
| 58 | `exec_log2` |  |  |  | Y | Y |  |  | unknown | 450&#8209;453 |
| 59 | `default` |  | Y |  |  | Y |  | Y |  | 709 |
| 60 | `eq` |  | Y |  |  | Y |  |  | unknown | 734&#8209;735 |

### Chap45/Example45_2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 61 | `_example_45_2_verified` |  |  |  | Y | Y |  | Y |  | 19 |
| 62 | `example_45_2_textbook_example` | Y | Y |  | Y | Y |  |  | unknown | 24 |
| 63 | `example_45_2_reverse_sorted` | Y | Y |  | Y | Y |  |  | unknown | 25 |
| 64 | `example_45_2_already_sorted` | Y | Y |  | Y | Y |  |  | unknown | 26 |
| 65 | `example_45_2_duplicates` | Y | Y |  | Y | Y |  |  | unknown | 27 |
| 66 | `example_45_2_single_element` | Y | Y |  | Y | Y |  |  | unknown | 28 |
| 67 | `example_45_2_empty` | Y | Y |  | Y | Y |  |  | unknown | 29 |
| 68 | `example_45_2_efficiency_demonstration` | Y | Y |  | Y | Y |  |  | unknown | 30 |
| 69 | `run_example_45_2` | Y | Y |  | Y | Y |  |  | unknown | 31 |

### Chap45/HeapsortExample.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 70 | `eq` |  | Y |  |  | Y |  |  | unknown | 90&#8209;91 |
| 71 | `_heapsort_example_verified` |  |  |  | Y | Y |  | Y |  | 112 |
| 72 | `all_results_match` | Y | Y |  |  | Y |  | Y |  | 117 |
| 73 | `all_results_sorted` | Y | Y |  |  | Y |  | Y |  | 119 |
| 74 | `heapsort_unsorted_list` |  |  |  | Y |  | Y | Y |  | 146&#8209;162 |
| 75 | `heapsort_sorted_list` |  |  |  | Y |  | Y | Y |  | 164&#8209;180 |
| 76 | `heapsort_balanced_tree` |  |  |  | Y |  | Y | Y |  | 182&#8209;198 |
| 77 | `heapsort_binary_heap` |  |  |  | Y |  | Y | Y |  | 200&#8209;216 |
| 78 | `heapsort_leftist_heap` |  |  |  | Y |  | Y | Y |  | 218&#8209;234 |
| 79 | `compare_all_heapsorts` |  |  |  | Y |  | Y | Y |  | 236&#8209;246 |
| 80 | `textbook_example` |  |  |  | Y |  | Y | Y |  | 273&#8209;277 |
| 81 | `reverse_sorted_example` |  |  |  | Y |  | Y | Y |  | 279&#8209;283 |
| 82 | `already_sorted_example` |  |  |  | Y |  | Y | Y |  | 285&#8209;289 |
| 83 | `duplicates_example` |  |  |  | Y |  | Y | Y |  | 291&#8209;295 |
| 84 | `single_element_example` |  |  |  | Y |  | Y | Y |  | 297&#8209;301 |
| 85 | `empty_example` |  |  |  | Y |  | Y | Y |  | 303&#8209;307 |
| 86 | `large_example` |  |  |  | Y |  | Y | Y |  | 309&#8209;319 |
| 87 | `efficiency_demonstration` |  |  |  | Y |  | Y | Y |  | 321&#8209;330 |
| 88 | `complexity_analysis` |  |  |  | Y |  | Y | Y |  | 332&#8209;361 |
| 89 | `correctness_verification` |  |  |  | Y |  | Y | Y |  | 363&#8209;377 |
| 90 | `vec_to_array_seq` |  |  |  | Y |  | Y | Y |  | 379&#8209;387 |
| 91 | `vec_to_avl_seq` |  |  |  | Y |  | Y | Y |  | 389&#8209;390 |
| 92 | `is_sorted` |  |  |  | Y |  | Y | Y |  | 392&#8209;393 |
| 93 | `generate_test_sequences` |  |  |  | Y |  | Y | Y |  | 395&#8209;410 |

### Chap45/LeftistHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 94 | `_leftist_heap_pq_verified` |  |  |  | Y | Y |  | Y |  | 68 |
| 95 | `rank` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 96 | `make_node` | Y | Y |  |  | Y |  |  | hole | 79&#8209;81 |
| 97 | `meld_nodes` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;84 |
| 98 | `size` x3 | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 99 | `height` x3 | Y | Y |  |  | Y |  |  | hole | 134&#8209;136 |
| 100 | `is_leftist` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;91 |
| 101 | `is_heap` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 102 | `to_vec` x3 | Y | Y |  |  | Y |  |  | unknown | 144&#8209;145 |
| 103 | `empty` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;103 |
| 104 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 105 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;109 |
| 106 | `insert` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;112 |
| 107 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;119 |
| 108 | `meld` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 109 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 110 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;130 |
| 111 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 112 | `root_rank` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;138 |
| 113 | `is_valid_leftist_heap` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;140 |
| 114 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;143 |
| 115 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;148 |
| 116 | `meld_multiple` | Y | Y |  |  | Y |  |  | hole | 149&#8209;150 |
| 117 | `split` | Y | Y |  |  | Y |  |  | hole | 151&#8209;152 |
| 118 | `default` |  | Y |  |  | Y |  | Y |  | 504 |
| 119 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 530&#8209;532 |
| 120 | `format_node` x2 |  | Y |  |  |  | Y | Y |  | 601&#8209;611 |
| 121 | `efficient_multi_way_merge` |  |  |  | Y |  | Y | Y |  | 642&#8209;649 |
| 122 | `parallel_heap_construction` |  |  |  | Y |  | Y | Y |  | 651&#8209;654 |

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
| 141 | `is_sorted` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;126 |
| 142 | `default` |  | Y |  |  | Y |  | Y |  | 390 |
| 143 | `eq` |  | Y |  |  | Y |  |  | unknown | 417&#8209;418 |

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
| 160 | `default` |  | Y |  |  | Y |  | Y |  | 291 |
| 161 | `eq` |  | Y |  |  | Y |  |  | unknown | 318&#8209;319 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
