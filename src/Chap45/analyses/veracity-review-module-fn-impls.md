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
| 1 | `_balanced_tree_pq_verified` |  |  |  | Y | Y |  | Y |  | 54 |
| 2 | `empty` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 3 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 4 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 5 | `insert` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 6 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;78 |
| 7 | `meld` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;81 |
| 8 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 9 | `size` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;87 |
| 10 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 11 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 12 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;98 |
| 13 | `delete_max` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;105 |
| 14 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;108 |
| 15 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 16 | `contains` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;114 |
| 17 | `remove` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;119 |
| 18 | `range` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;122 |
| 19 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 20 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;128 |
| 21 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 22 | `is_sorted` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 23 | `height` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 24 | `split` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;140 |
| 25 | `join` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;143 |
| 26 | `filter` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;149 |
| 27 | `map` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;151 |
| 28 | `default` |  | Y |  |  | Y |  | Y |  | 379 |
| 29 | `eq` |  | Y |  |  | Y |  |  | unknown | 429&#8209;430 |

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
| 40 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;172 |
| 41 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;179 |
| 42 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;185 |
| 43 | `is_valid_heap` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;188 |
| 44 | `height` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;192 |
| 45 | `level_elements` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;198 |
| 46 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;204 |
| 47 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;208 |
| 48 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;214 |
| 49 | `left_child` |  |  |  | Y | Y |  |  | unknown | 226&#8209;228 |
| 50 | `right_child` |  |  |  | Y | Y |  |  | unknown | 233&#8209;235 |
| 51 | `parent` |  |  |  | Y | Y |  |  | unknown | 240&#8209;241 |
| 52 | `swap_elements` |  |  |  | Y | Y |  |  | unknown | 246&#8209;252 |
| 53 | `bubble_up` |  |  |  | Y | Y |  |  | unknown | 280&#8209;285 |
| 54 | `bubble_down` |  |  |  | Y | Y |  |  | unknown | 320&#8209;326 |
| 55 | `heapify` |  |  |  | Y | Y |  |  | unknown | 371&#8209;376 |
| 56 | `is_heap` |  |  |  | Y | Y |  |  | unknown | 402&#8209;403 |
| 57 | `exec_pow2` |  |  |  | Y | Y |  |  | unknown | 429&#8209;431 |
| 58 | `exec_log2` |  |  |  | Y | Y |  |  | unknown | 456&#8209;459 |
| 59 | `default` |  | Y |  |  | Y |  | Y |  | 715 |
| 60 | `eq` |  | Y |  |  | Y |  |  | unknown | 740&#8209;741 |

### Chap45/Example45_2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 61 | `_example_45_2_verified` |  |  |  | Y | Y |  | Y |  | 27 |
| 62 | `example_45_2_textbook_example` | Y | Y |  | Y | Y |  |  | unknown | 32 |
| 63 | `example_45_2_reverse_sorted` | Y | Y |  | Y | Y |  |  | unknown | 33 |
| 64 | `example_45_2_already_sorted` | Y | Y |  | Y | Y |  |  | unknown | 34 |
| 65 | `example_45_2_duplicates` | Y | Y |  | Y | Y |  |  | unknown | 35 |
| 66 | `example_45_2_single_element` | Y | Y |  | Y | Y |  |  | unknown | 36 |
| 67 | `example_45_2_empty` | Y | Y |  | Y | Y |  |  | unknown | 37 |
| 68 | `example_45_2_efficiency_demonstration` | Y | Y |  | Y | Y |  |  | unknown | 38 |
| 69 | `run_example_45_2` | Y | Y |  | Y | Y |  |  | unknown | 39 |

### Chap45/HeapsortExample.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 70 | `eq` |  | Y |  |  | Y |  |  | unknown | 102&#8209;103 |
| 71 | `_heapsort_example_verified` |  |  |  | Y | Y |  | Y |  | 124 |
| 72 | `all_results_match` | Y | Y |  |  | Y |  | Y |  | 129 |
| 73 | `all_results_sorted` | Y | Y |  |  | Y |  | Y |  | 131 |
| 74 | `heapsort_unsorted_list` |  |  |  | Y |  | Y | Y |  | 158&#8209;174 |
| 75 | `heapsort_sorted_list` |  |  |  | Y |  | Y | Y |  | 176&#8209;192 |
| 76 | `heapsort_balanced_tree` |  |  |  | Y |  | Y | Y |  | 194&#8209;210 |
| 77 | `heapsort_binary_heap` |  |  |  | Y |  | Y | Y |  | 212&#8209;228 |
| 78 | `heapsort_leftist_heap` |  |  |  | Y |  | Y | Y |  | 230&#8209;246 |
| 79 | `compare_all_heapsorts` |  |  |  | Y |  | Y | Y |  | 248&#8209;258 |
| 80 | `textbook_example` |  |  |  | Y |  | Y | Y |  | 285&#8209;289 |
| 81 | `reverse_sorted_example` |  |  |  | Y |  | Y | Y |  | 291&#8209;295 |
| 82 | `already_sorted_example` |  |  |  | Y |  | Y | Y |  | 297&#8209;301 |
| 83 | `duplicates_example` |  |  |  | Y |  | Y | Y |  | 303&#8209;307 |
| 84 | `single_element_example` |  |  |  | Y |  | Y | Y |  | 309&#8209;313 |
| 85 | `empty_example` |  |  |  | Y |  | Y | Y |  | 315&#8209;319 |
| 86 | `large_example` |  |  |  | Y |  | Y | Y |  | 321&#8209;331 |
| 87 | `efficiency_demonstration` |  |  |  | Y |  | Y | Y |  | 333&#8209;342 |
| 88 | `complexity_analysis` |  |  |  | Y |  | Y | Y |  | 344&#8209;373 |
| 89 | `correctness_verification` |  |  |  | Y |  | Y | Y |  | 375&#8209;389 |
| 90 | `vec_to_array_seq` |  |  |  | Y |  | Y | Y |  | 391&#8209;399 |
| 91 | `vec_to_avl_seq` |  |  |  | Y |  | Y | Y |  | 401&#8209;402 |
| 92 | `is_sorted` |  |  |  | Y |  | Y | Y |  | 404&#8209;405 |
| 93 | `generate_test_sequences` |  |  |  | Y |  | Y | Y |  | 407&#8209;422 |

### Chap45/LeftistHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 94 | `_leftist_heap_pq_verified` |  |  |  | Y | Y |  | Y |  | 63 |
| 95 | `rank` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;73 |
| 96 | `make_node` | Y | Y |  |  | Y |  |  | hole | 74&#8209;76 |
| 97 | `meld_nodes` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 98 | `size` x3 | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 99 | `height` x3 | Y | Y |  |  | Y |  |  | hole | 130&#8209;132 |
| 100 | `is_leftist` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;86 |
| 101 | `is_heap` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 102 | `to_vec` x3 | Y | Y |  |  | Y |  |  | unknown | 140&#8209;142 |
| 103 | `empty` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 104 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;101 |
| 105 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;105 |
| 106 | `insert` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;108 |
| 107 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;115 |
| 108 | `meld` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 109 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 110 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;126 |
| 111 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;129 |
| 112 | `root_rank` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 113 | `is_valid_leftist_heap` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;136 |
| 114 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 115 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |
| 116 | `meld_multiple` | Y | Y |  |  | Y |  |  | hole | 146&#8209;147 |
| 117 | `split` | Y | Y |  |  | Y |  |  | hole | 148&#8209;149 |
| 118 | `default` |  | Y |  |  | Y |  | Y |  | 504 |
| 119 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 530&#8209;532 |
| 120 | `format_node` x2 |  | Y |  |  |  | Y | Y |  | 601&#8209;611 |
| 121 | `efficient_multi_way_merge` |  |  |  | Y |  | Y | Y |  | 642&#8209;649 |
| 122 | `parallel_heap_construction` |  |  |  | Y |  | Y | Y |  | 651&#8209;654 |

### Chap45/SortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 123 | `_sorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 55 |
| 124 | `empty` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 125 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 126 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;72 |
| 127 | `insert` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;78 |
| 128 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;86 |
| 129 | `meld` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 130 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 131 | `size` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 132 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 133 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 134 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;112 |
| 135 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 136 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;121 |
| 137 | `delete_max` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;129 |
| 138 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 139 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;137 |
| 140 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;141 |
| 141 | `is_sorted` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;144 |
| 142 | `default` |  | Y |  |  | Y |  | Y |  | 408 |
| 143 | `eq` |  | Y |  |  | Y |  |  | unknown | 435&#8209;436 |

### Chap45/UnsortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 144 | `_unsorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 55 |
| 145 | `empty` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 146 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 147 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;72 |
| 148 | `insert` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;78 |
| 149 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;86 |
| 150 | `meld` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 151 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 152 | `size` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 153 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 154 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 155 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;112 |
| 156 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 157 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 158 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 159 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;132 |
| 160 | `default` |  | Y |  |  | Y |  | Y |  | 309 |
| 161 | `eq` |  | Y |  |  | Y |  |  | unknown | 336&#8209;337 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
