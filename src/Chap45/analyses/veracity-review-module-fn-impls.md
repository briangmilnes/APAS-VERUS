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
| 1 | Chap45 | BalancedTreePQ | 26 | 28 | 0 | 1 | 27 | 2 | 1 | 0 | 28 |
| 2 | Chap45 | BinaryHeapPQ | 18 | 20 | 0 | 11 | 31 | 0 | 30 | 0 | 1 |
| 3 | Chap45 | Example45_2 | 8 | 0 | 0 | 9 | 1 | 8 | 0 | 0 | 9 |
| 4 | Chap45 | HeapsortExample | 2 | 4 | 0 | 21 | 2 | 22 | 1 | 0 | 23 |
| 5 | Chap45 | LeftistHeapPQ | 23 | 26 | 0 | 3 | 26 | 3 | 10 | 4 | 15 |
| 6 | Chap45 | SortedListPQ | 18 | 20 | 0 | 1 | 21 | 0 | 18 | 0 | 3 |
| 7 | Chap45 | UnsortedListPQ | 15 | 17 | 0 | 1 | 18 | 0 | 16 | 0 | 2 |

## Function-by-Function Detail

### Chap45/BalancedTreePQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `_balanced_tree_pq_verified` |  |  |  | Y | Y |  | Y |  | 41 |
| 2 | `empty` | Y | Y |  |  | Y |  | Y |  | 46 |
| 3 | `singleton` | Y | Y |  |  | Y |  | Y |  | 47 |
| 4 | `find_min` | Y | Y |  |  | Y |  | Y |  | 48 |
| 5 | `insert` | Y | Y |  |  | Y |  | Y |  | 49 |
| 6 | `delete_min` | Y | Y |  |  | Y |  | Y |  | 50 |
| 7 | `meld` | Y | Y |  |  | Y |  | Y |  | 51 |
| 8 | `from_seq` | Y | Y |  |  | Y |  | Y |  | 52 |
| 9 | `size` | Y | Y |  |  | Y |  | Y |  | 53 |
| 10 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 54 |
| 11 | `to_seq` | Y | Y |  |  | Y |  | Y |  | 55 |
| 12 | `find_max` | Y | Y |  |  | Y |  | Y |  | 56 |
| 13 | `delete_max` | Y | Y |  |  | Y |  | Y |  | 57 |
| 14 | `insert_all` | Y | Y |  |  | Y |  | Y |  | 58 |
| 15 | `extract_all_sorted` | Y | Y |  |  | Y |  | Y |  | 59 |
| 16 | `contains` | Y | Y |  |  | Y |  | Y |  | 60 |
| 17 | `remove` | Y | Y |  |  | Y |  | Y |  | 61 |
| 18 | `range` | Y | Y |  |  | Y |  | Y |  | 62 |
| 19 | `from_vec` | Y | Y |  |  | Y |  | Y |  | 63 |
| 20 | `to_vec` | Y | Y |  |  | Y |  | Y |  | 64 |
| 21 | `to_sorted_vec` | Y | Y |  |  | Y |  | Y |  | 65 |
| 22 | `is_sorted` | Y | Y |  |  | Y |  | Y |  | 66 |
| 23 | `height` | Y | Y |  |  | Y |  | Y |  | 67 |
| 24 | `split` | Y | Y |  |  | Y |  | Y |  | 68 |
| 25 | `join` | Y | Y |  |  | Y |  | Y |  | 69 |
| 26 | `default` |  | Y |  |  | Y |  | Y |  | 297 |
| 27 | `eq` |  | Y |  |  | Y |  |  | unknown | 316&#8209;317 |
| 28 | `filter` | Y | Y |  |  |  | Y | Y |  | 331 |
| 29 | `map` | Y | Y |  |  |  | Y | Y |  | 332 |

### Chap45/BinaryHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 30 | `lemma_log2_bound` |  |  |  | Y | Y |  |  | unknown | 102&#8209;109 |
| 31 | `empty` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;130 |
| 32 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 33 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;139 |
| 34 | `insert` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;146 |
| 35 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;156 |
| 36 | `meld` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;164 |
| 37 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;171 |
| 38 | `size` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;174 |
| 39 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;177 |
| 40 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;180 |
| 41 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;186 |
| 42 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;191 |
| 43 | `is_valid_heap` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;194 |
| 44 | `height` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;198 |
| 45 | `level_elements` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;204 |
| 46 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;209 |
| 47 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;212 |
| 48 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;217 |
| 49 | `left_child` |  |  |  | Y | Y |  |  | unknown | 229&#8209;231 |
| 50 | `right_child` |  |  |  | Y | Y |  |  | unknown | 236&#8209;238 |
| 51 | `parent` |  |  |  | Y | Y |  |  | unknown | 243&#8209;244 |
| 52 | `swap_elements` |  |  |  | Y | Y |  |  | unknown | 249&#8209;255 |
| 53 | `bubble_up` |  |  |  | Y | Y |  |  | unknown | 283&#8209;288 |
| 54 | `bubble_down` |  |  |  | Y | Y |  |  | unknown | 323&#8209;329 |
| 55 | `heapify` |  |  |  | Y | Y |  |  | unknown | 374&#8209;379 |
| 56 | `is_heap` |  |  |  | Y | Y |  |  | unknown | 405&#8209;406 |
| 57 | `exec_pow2` |  |  |  | Y | Y |  |  | unknown | 432&#8209;434 |
| 58 | `exec_log2` |  |  |  | Y | Y |  |  | unknown | 459&#8209;462 |
| 59 | `default` |  | Y |  |  | Y |  | Y |  | 705 |
| 60 | `eq` |  | Y |  |  | Y |  |  | unknown | 730&#8209;731 |

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
| 94 | `_leftist_heap_pq_verified` |  |  |  | Y | Y |  | Y |  | 69 |
| 95 | `rank` | Y | Y |  |  | Y |  | Y |  | 76 |
| 96 | `make_node` | Y | Y |  |  | Y |  |  | hole | 77&#8209;78 |
| 97 | `meld_nodes` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 98 | `size` x3 | Y | Y |  |  | Y |  |  | hole | 113&#8209;114 |
| 99 | `height` x3 | Y | Y |  |  | Y |  |  | hole | 118 |
| 100 | `is_leftist` | Y | Y |  |  | Y |  | Y |  | 83 |
| 101 | `is_heap` | Y | Y |  |  | Y |  | Y |  | 84 |
| 102 | `to_vec` x3 | Y | Y |  |  | Y |  | Y |  | 123 |
| 103 | `empty` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 104 | `singleton` | Y | Y |  |  | Y |  |  | hole | 94&#8209;95 |
| 105 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;99 |
| 106 | `insert` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;101 |
| 107 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;107 |
| 108 | `meld` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 109 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;112 |
| 110 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;116 |
| 111 | `extract_all_sorted` | Y | Y |  |  | Y |  | Y |  | 117 |
| 112 | `root_rank` | Y | Y |  |  | Y |  | Y |  | 119 |
| 113 | `is_valid_leftist_heap` | Y | Y |  |  | Y |  | Y |  | 120 |
| 114 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;122 |
| 115 | `to_sorted_vec` | Y | Y |  |  | Y |  | Y |  | 124 |
| 116 | `meld_multiple` | Y | Y |  |  | Y |  | Y |  | 125 |
| 117 | `split` | Y | Y |  |  | Y |  | Y |  | 126 |
| 118 | `default` |  | Y |  |  | Y |  | Y |  | 446 |
| 119 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 474&#8209;476 |
| 120 | `format_node` |  | Y |  |  |  | Y | Y |  | 555&#8209;566 |
| 121 | `efficient_multi_way_merge` |  |  |  | Y |  | Y | Y |  | 573&#8209;580 |
| 122 | `parallel_heap_construction` |  |  |  | Y |  | Y | Y |  | 582&#8209;585 |

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
