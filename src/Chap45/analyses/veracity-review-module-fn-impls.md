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
| 5 | Chap45 | LeftistHeapPQ | 23 | 26 | 0 | 4 | 27 | 3 | 25 | 0 | 5 |
| 6 | Chap45 | SortedListPQ | 18 | 20 | 0 | 1 | 21 | 0 | 19 | 0 | 2 |
| 7 | Chap45 | UnsortedListPQ | 15 | 17 | 0 | 1 | 18 | 0 | 16 | 0 | 2 |

## Function-by-Function Detail

### Chap45/BalancedTreePQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `_balanced_tree_pq_verified` |  |  |  | Y | Y |  | Y |  | 55 |
| 2 | `empty` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;61 |
| 3 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 4 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 5 | `insert` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 6 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;79 |
| 7 | `meld` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 8 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 9 | `size` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 10 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;91 |
| 11 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 12 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;99 |
| 13 | `delete_max` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;106 |
| 14 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 15 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 16 | `contains` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 17 | `remove` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;120 |
| 18 | `range` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 19 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;126 |
| 20 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 21 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;132 |
| 22 | `is_sorted` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;135 |
| 23 | `height` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;138 |
| 24 | `split` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;141 |
| 25 | `join` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;144 |
| 26 | `filter` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;150 |
| 27 | `map` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;152 |
| 28 | `default` |  | Y |  |  | Y |  | Y |  | 380 |
| 29 | `eq` |  | Y |  |  | Y |  |  | unknown | 430&#8209;431 |

### Chap45/BinaryHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 30 | `lemma_log2_bound` |  |  |  | Y | Y |  |  | unknown | 82&#8209;89 |
| 31 | `empty` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;119 |
| 32 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;125 |
| 33 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;130 |
| 34 | `insert` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;138 |
| 35 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;150 |
| 36 | `meld` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;159 |
| 37 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;166 |
| 38 | `size` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;169 |
| 39 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;172 |
| 40 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;176 |
| 41 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;183 |
| 42 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;191 |
| 43 | `is_valid_heap` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;194 |
| 44 | `height` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;198 |
| 45 | `level_elements` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;204 |
| 46 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;210 |
| 47 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;214 |
| 48 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;222 |
| 49 | `left_child` |  |  |  | Y | Y |  |  | unknown | 234&#8209;236 |
| 50 | `right_child` |  |  |  | Y | Y |  |  | unknown | 241&#8209;243 |
| 51 | `parent` |  |  |  | Y | Y |  |  | unknown | 248&#8209;249 |
| 52 | `swap_elements` |  |  |  | Y | Y |  |  | unknown | 254&#8209;260 |
| 53 | `bubble_up` |  |  |  | Y | Y |  |  | unknown | 288&#8209;293 |
| 54 | `bubble_down` |  |  |  | Y | Y |  |  | unknown | 328&#8209;334 |
| 55 | `heapify` |  |  |  | Y | Y |  |  | unknown | 379&#8209;384 |
| 56 | `is_heap` |  |  |  | Y | Y |  |  | unknown | 410&#8209;411 |
| 57 | `exec_pow2` |  |  |  | Y | Y |  |  | unknown | 437&#8209;439 |
| 58 | `exec_log2` |  |  |  | Y | Y |  |  | unknown | 464&#8209;467 |
| 59 | `default` |  | Y |  |  | Y |  | Y |  | 765 |
| 60 | `eq` |  | Y |  |  | Y |  |  | unknown | 790&#8209;791 |

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
| 70 | `eq` |  | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 71 | `_heapsort_example_verified` |  |  |  | Y | Y |  | Y |  | 126 |
| 72 | `all_results_match` | Y | Y |  |  | Y |  | Y |  | 131 |
| 73 | `all_results_sorted` | Y | Y |  |  | Y |  | Y |  | 133 |
| 74 | `heapsort_unsorted_list` |  |  |  | Y |  | Y | Y |  | 160&#8209;176 |
| 75 | `heapsort_sorted_list` |  |  |  | Y |  | Y | Y |  | 178&#8209;194 |
| 76 | `heapsort_balanced_tree` |  |  |  | Y |  | Y | Y |  | 196&#8209;212 |
| 77 | `heapsort_binary_heap` |  |  |  | Y |  | Y | Y |  | 214&#8209;230 |
| 78 | `heapsort_leftist_heap` |  |  |  | Y |  | Y | Y |  | 232&#8209;248 |
| 79 | `compare_all_heapsorts` |  |  |  | Y |  | Y | Y |  | 250&#8209;260 |
| 80 | `textbook_example` |  |  |  | Y |  | Y | Y |  | 287&#8209;291 |
| 81 | `reverse_sorted_example` |  |  |  | Y |  | Y | Y |  | 293&#8209;297 |
| 82 | `already_sorted_example` |  |  |  | Y |  | Y | Y |  | 299&#8209;303 |
| 83 | `duplicates_example` |  |  |  | Y |  | Y | Y |  | 305&#8209;309 |
| 84 | `single_element_example` |  |  |  | Y |  | Y | Y |  | 311&#8209;315 |
| 85 | `empty_example` |  |  |  | Y |  | Y | Y |  | 317&#8209;321 |
| 86 | `large_example` |  |  |  | Y |  | Y | Y |  | 323&#8209;333 |
| 87 | `efficiency_demonstration` |  |  |  | Y |  | Y | Y |  | 335&#8209;344 |
| 88 | `complexity_analysis` |  |  |  | Y |  | Y | Y |  | 346&#8209;375 |
| 89 | `correctness_verification` |  |  |  | Y |  | Y | Y |  | 377&#8209;391 |
| 90 | `vec_to_array_seq` |  |  |  | Y |  | Y | Y |  | 393&#8209;401 |
| 91 | `vec_to_avl_seq` |  |  |  | Y |  | Y | Y |  | 403&#8209;404 |
| 92 | `is_sorted` |  |  |  | Y |  | Y | Y |  | 406&#8209;407 |
| 93 | `generate_test_sequences` |  |  |  | Y |  | Y | Y |  | 409&#8209;424 |

### Chap45/LeftistHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 94 | `_leftist_heap_pq_verified` |  |  |  | Y | Y |  | Y |  | 73 |
| 95 | `lemma_total_size_monotone` |  |  |  | Y | Y |  |  | unknown | 75&#8209;78 |
| 96 | `rank` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 97 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 98 | `meld_nodes` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 99 | `size` x3 | Y | Y |  |  | Y |  |  | unknown | 156&#8209;158 |
| 100 | `height` x3 | Y | Y |  |  | Y |  |  | unknown | 166&#8209;168 |
| 101 | `is_leftist` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 102 | `is_heap` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 103 | `to_vec` x3 | Y | Y |  |  | Y |  |  | unknown | 176&#8209;178 |
| 104 | `empty` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;125 |
| 105 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;129 |
| 106 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;133 |
| 107 | `insert` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 108 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;147 |
| 109 | `meld` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;152 |
| 110 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;155 |
| 111 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;160 |
| 112 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;165 |
| 113 | `root_rank` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;170 |
| 114 | `is_valid_leftist_heap` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;172 |
| 115 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;175 |
| 116 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;183 |
| 117 | `meld_multiple` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;188 |
| 118 | `split` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;190 |
| 119 | `default` |  | Y |  |  | Y |  | Y |  | 587 |
| 120 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 613&#8209;615 |
| 121 | `format_node` x2 |  | Y |  |  |  | Y | Y |  | 684&#8209;694 |
| 122 | `efficient_multi_way_merge` |  |  |  | Y |  | Y | Y |  | 725&#8209;732 |
| 123 | `parallel_heap_construction` |  |  |  | Y |  | Y | Y |  | 734&#8209;737 |

### Chap45/SortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 124 | `_sorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 63 |
| 125 | `empty` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;74 |
| 126 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;80 |
| 127 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;85 |
| 128 | `insert` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;93 |
| 129 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;103 |
| 130 | `meld` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;111 |
| 131 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 132 | `size` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 133 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 134 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 135 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;131 |
| 136 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;137 |
| 137 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;142 |
| 138 | `delete_max` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;152 |
| 139 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;156 |
| 140 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;160 |
| 141 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;166 |
| 142 | `is_sorted` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;169 |
| 143 | `default` |  | Y |  |  | Y |  | Y |  | 468 |
| 144 | `eq` |  | Y |  |  | Y |  |  | unknown | 495&#8209;496 |

### Chap45/UnsortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 145 | `_unsorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 63 |
| 146 | `empty` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;74 |
| 147 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;80 |
| 148 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;88 |
| 149 | `insert` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;96 |
| 150 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;116 |
| 151 | `meld` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;124 |
| 152 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 153 | `size` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 154 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 155 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |
| 156 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;144 |
| 157 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;152 |
| 158 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;156 |
| 159 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;160 |
| 160 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;168 |
| 161 | `default` |  | Y |  |  | Y |  | Y |  | 491 |
| 162 | `eq` |  | Y |  |  | Y |  |  | unknown | 518&#8209;519 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
