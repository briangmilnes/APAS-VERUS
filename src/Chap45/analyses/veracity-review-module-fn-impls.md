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
| 30 | `lemma_log2_bound` |  |  |  | Y | Y |  |  | unknown | 74&#8209;81 |
| 31 | `empty` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;108 |
| 32 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;112 |
| 33 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;117 |
| 34 | `insert` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;124 |
| 35 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;134 |
| 36 | `meld` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;142 |
| 37 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;149 |
| 38 | `size` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;152 |
| 39 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;155 |
| 40 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;159 |
| 41 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;166 |
| 42 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;172 |
| 43 | `is_valid_heap` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;175 |
| 44 | `height` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;179 |
| 45 | `level_elements` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;185 |
| 46 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;191 |
| 47 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;195 |
| 48 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;201 |
| 49 | `left_child` |  |  |  | Y | Y |  |  | unknown | 213&#8209;215 |
| 50 | `right_child` |  |  |  | Y | Y |  |  | unknown | 220&#8209;222 |
| 51 | `parent` |  |  |  | Y | Y |  |  | unknown | 227&#8209;228 |
| 52 | `swap_elements` |  |  |  | Y | Y |  |  | unknown | 233&#8209;239 |
| 53 | `bubble_up` |  |  |  | Y | Y |  |  | unknown | 267&#8209;272 |
| 54 | `bubble_down` |  |  |  | Y | Y |  |  | unknown | 307&#8209;313 |
| 55 | `heapify` |  |  |  | Y | Y |  |  | unknown | 358&#8209;363 |
| 56 | `is_heap` |  |  |  | Y | Y |  |  | unknown | 389&#8209;390 |
| 57 | `exec_pow2` |  |  |  | Y | Y |  |  | unknown | 416&#8209;418 |
| 58 | `exec_log2` |  |  |  | Y | Y |  |  | unknown | 443&#8209;446 |
| 59 | `default` |  | Y |  |  | Y |  | Y |  | 714 |
| 60 | `eq` |  | Y |  |  | Y |  |  | unknown | 739&#8209;740 |

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
| 70 | `eq` |  | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 71 | `_heapsort_example_verified` |  |  |  | Y | Y |  | Y |  | 125 |
| 72 | `all_results_match` | Y | Y |  |  | Y |  | Y |  | 130 |
| 73 | `all_results_sorted` | Y | Y |  |  | Y |  | Y |  | 132 |
| 74 | `heapsort_unsorted_list` |  |  |  | Y |  | Y | Y |  | 159&#8209;175 |
| 75 | `heapsort_sorted_list` |  |  |  | Y |  | Y | Y |  | 177&#8209;193 |
| 76 | `heapsort_balanced_tree` |  |  |  | Y |  | Y | Y |  | 195&#8209;211 |
| 77 | `heapsort_binary_heap` |  |  |  | Y |  | Y | Y |  | 213&#8209;229 |
| 78 | `heapsort_leftist_heap` |  |  |  | Y |  | Y | Y |  | 231&#8209;247 |
| 79 | `compare_all_heapsorts` |  |  |  | Y |  | Y | Y |  | 249&#8209;259 |
| 80 | `textbook_example` |  |  |  | Y |  | Y | Y |  | 286&#8209;290 |
| 81 | `reverse_sorted_example` |  |  |  | Y |  | Y | Y |  | 292&#8209;296 |
| 82 | `already_sorted_example` |  |  |  | Y |  | Y | Y |  | 298&#8209;302 |
| 83 | `duplicates_example` |  |  |  | Y |  | Y | Y |  | 304&#8209;308 |
| 84 | `single_element_example` |  |  |  | Y |  | Y | Y |  | 310&#8209;314 |
| 85 | `empty_example` |  |  |  | Y |  | Y | Y |  | 316&#8209;320 |
| 86 | `large_example` |  |  |  | Y |  | Y | Y |  | 322&#8209;332 |
| 87 | `efficiency_demonstration` |  |  |  | Y |  | Y | Y |  | 334&#8209;343 |
| 88 | `complexity_analysis` |  |  |  | Y |  | Y | Y |  | 345&#8209;374 |
| 89 | `correctness_verification` |  |  |  | Y |  | Y | Y |  | 376&#8209;390 |
| 90 | `vec_to_array_seq` |  |  |  | Y |  | Y | Y |  | 392&#8209;400 |
| 91 | `vec_to_avl_seq` |  |  |  | Y |  | Y | Y |  | 402&#8209;403 |
| 92 | `is_sorted` |  |  |  | Y |  | Y | Y |  | 405&#8209;406 |
| 93 | `generate_test_sequences` |  |  |  | Y |  | Y | Y |  | 408&#8209;423 |

### Chap45/LeftistHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 94 | `_leftist_heap_pq_verified` |  |  |  | Y | Y |  | Y |  | 66 |
| 95 | `lemma_total_size_monotone` |  |  |  | Y | Y |  |  | unknown | 68&#8209;71 |
| 96 | `rank` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;86 |
| 97 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;89 |
| 98 | `meld_nodes` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 99 | `size` x3 | Y | Y |  |  | Y |  |  | unknown | 138&#8209;140 |
| 100 | `height` x3 | Y | Y |  |  | Y |  |  | unknown | 146&#8209;148 |
| 101 | `is_leftist` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 102 | `is_heap` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 103 | `to_vec` x3 | Y | Y |  |  | Y |  |  | unknown | 156&#8209;158 |
| 104 | `empty` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 105 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 106 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;121 |
| 107 | `insert` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 108 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;131 |
| 109 | `meld` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 110 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;137 |
| 111 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;142 |
| 112 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |
| 113 | `root_rank` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;150 |
| 114 | `is_valid_leftist_heap` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;152 |
| 115 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;155 |
| 116 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;161 |
| 117 | `meld_multiple` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;166 |
| 118 | `split` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;168 |
| 119 | `default` |  | Y |  |  | Y |  | Y |  | 532 |
| 120 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 558&#8209;560 |
| 121 | `format_node` x2 |  | Y |  |  |  | Y | Y |  | 629&#8209;639 |
| 122 | `efficient_multi_way_merge` |  |  |  | Y |  | Y | Y |  | 670&#8209;677 |
| 123 | `parallel_heap_construction` |  |  |  | Y |  | Y | Y |  | 679&#8209;682 |

### Chap45/SortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 124 | `_sorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 55 |
| 125 | `empty` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 126 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 127 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;72 |
| 128 | `insert` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;78 |
| 129 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;86 |
| 130 | `meld` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 131 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 132 | `size` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 133 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 134 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 135 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;112 |
| 136 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 137 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;121 |
| 138 | `delete_max` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;129 |
| 139 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 140 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;137 |
| 141 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;141 |
| 142 | `is_sorted` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;144 |
| 143 | `default` |  | Y |  |  | Y |  | Y |  | 408 |
| 144 | `eq` |  | Y |  |  | Y |  |  | unknown | 435&#8209;436 |

### Chap45/UnsortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 145 | `_unsorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 55 |
| 146 | `empty` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 147 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 148 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;72 |
| 149 | `insert` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;78 |
| 150 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;86 |
| 151 | `meld` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 152 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 153 | `size` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 154 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 155 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 156 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;112 |
| 157 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 158 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 159 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 160 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;132 |
| 161 | `default` |  | Y |  |  | Y |  | Y |  | 309 |
| 162 | `eq` |  | Y |  |  | Y |  |  | unknown | 336&#8209;337 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
