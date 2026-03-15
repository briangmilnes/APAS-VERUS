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
| 2 | Chap45 | BinaryHeapPQ | 18 | 20 | 0 | 12 | 32 | 0 | 30 | 1 | 1 |
| 3 | Chap45 | Example45_2 | 8 | 8 | 0 | 9 | 9 | 0 | 8 | 0 | 1 |
| 4 | Chap45 | HeapsortExample | 2 | 3 | 0 | 22 | 5 | 20 | 2 | 0 | 23 |
| 5 | Chap45 | LeftistHeapPQ | 24 | 27 | 0 | 7 | 31 | 3 | 29 | 0 | 5 |
| 6 | Chap45 | SortedListPQ | 19 | 21 | 0 | 1 | 22 | 0 | 20 | 0 | 2 |
| 7 | Chap45 | UnsortedListPQ | 15 | 17 | 0 | 1 | 18 | 0 | 16 | 0 | 2 |

## Function-by-Function Detail

### Chap45/BalancedTreePQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `_balanced_tree_pq_verified` |  |  |  | Y | Y |  | Y |  | 55 |
| 2 | `empty` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 3 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 4 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;72 |
| 5 | `insert` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;76 |
| 6 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;85 |
| 7 | `meld` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;89 |
| 8 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;93 |
| 9 | `size` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 10 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |
| 11 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;105 |
| 12 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;111 |
| 13 | `delete_max` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;120 |
| 14 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 15 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 16 | `contains` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;132 |
| 17 | `remove` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;139 |
| 18 | `range` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;143 |
| 19 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;146 |
| 20 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;150 |
| 21 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;154 |
| 22 | `is_sorted` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;158 |
| 23 | `height` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;162 |
| 24 | `split` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;169 |
| 25 | `join` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;173 |
| 26 | `filter` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;179 |
| 27 | `map` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;181 |
| 28 | `default` |  | Y |  |  | Y |  |  | unknown | 552&#8209;553 |
| 29 | `eq` |  | Y |  |  | Y |  |  | unknown | 604&#8209;605 |

### Chap45/BinaryHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 30 | `lemma_log2_bound` |  |  |  | Y | Y |  |  | unknown | 77&#8209;84 |
| 31 | `empty` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;117 |
| 32 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;124 |
| 33 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;129 |
| 34 | `insert` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;137 |
| 35 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;149 |
| 36 | `meld` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;158 |
| 37 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;165 |
| 38 | `size` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;168 |
| 39 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;171 |
| 40 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;175 |
| 41 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;182 |
| 42 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | hole | 184&#8209;190 |
| 43 | `is_valid_heap` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;193 |
| 44 | `height` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;197 |
| 45 | `level_elements` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;203 |
| 46 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;209 |
| 47 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;213 |
| 48 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;221 |
| 49 | `left_child` |  |  |  | Y | Y |  |  | unknown | 233&#8209;235 |
| 50 | `right_child` |  |  |  | Y | Y |  |  | unknown | 240&#8209;242 |
| 51 | `parent` |  |  |  | Y | Y |  |  | unknown | 247&#8209;249 |
| 52 | `lemma_swap_preserves_multiset` |  |  |  | Y | Y |  |  | unknown | 254&#8209;256 |
| 53 | `swap_elements` |  |  |  | Y | Y |  |  | unknown | 281&#8209;289 |
| 54 | `bubble_up` |  |  |  | Y | Y |  |  | unknown | 349&#8209;356 |
| 55 | `bubble_down` |  |  |  | Y | Y |  |  | unknown | 401&#8209;409 |
| 56 | `heapify` |  |  |  | Y | Y |  |  | unknown | 462&#8209;469 |
| 57 | `is_heap` |  |  |  | Y | Y |  |  | unknown | 512&#8209;514 |
| 58 | `exec_pow2` |  |  |  | Y | Y |  |  | unknown | 540&#8209;542 |
| 59 | `exec_log2` |  |  |  | Y | Y |  |  | unknown | 567&#8209;570 |
| 60 | `default` |  | Y |  |  | Y |  | Y |  | 1064 |
| 61 | `eq` |  | Y |  |  | Y |  |  | unknown | 1089&#8209;1090 |

### Chap45/Example45_2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 62 | `_example_45_2_verified` |  |  |  | Y | Y |  | Y |  | 27 |
| 63 | `example_45_2_textbook_example` | Y | Y |  | Y | Y |  |  | unknown | 32 |
| 64 | `example_45_2_reverse_sorted` | Y | Y |  | Y | Y |  |  | unknown | 33 |
| 65 | `example_45_2_already_sorted` | Y | Y |  | Y | Y |  |  | unknown | 34 |
| 66 | `example_45_2_duplicates` | Y | Y |  | Y | Y |  |  | unknown | 35 |
| 67 | `example_45_2_single_element` | Y | Y |  | Y | Y |  |  | unknown | 36 |
| 68 | `example_45_2_empty` | Y | Y |  | Y | Y |  |  | unknown | 37 |
| 69 | `example_45_2_efficiency_demonstration` | Y | Y |  | Y | Y |  |  | unknown | 38 |
| 70 | `run_example_45_2` | Y | Y |  | Y | Y |  |  | unknown | 39 |

### Chap45/HeapsortExample.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 71 | `eq` |  | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 72 | `_heapsort_example_verified` |  |  |  | Y | Y |  | Y |  | 126 |
| 73 | `is_vec_sorted_exec` |  |  |  | Y | Y |  |  | unknown | 131&#8209;132 |
| 74 | `all_results_match` | Y | Y |  |  | Y |  | Y |  | 153 |
| 75 | `all_results_sorted` | Y | Y |  |  | Y |  | Y |  | 155 |
| 76 | `heapsort_unsorted_list` |  |  |  | Y |  | Y | Y |  | 179&#8209;195 |
| 77 | `heapsort_sorted_list` |  |  |  | Y |  | Y | Y |  | 197&#8209;213 |
| 78 | `heapsort_balanced_tree` |  |  |  | Y |  | Y | Y |  | 215&#8209;231 |
| 79 | `heapsort_binary_heap` |  |  |  | Y |  | Y | Y |  | 233&#8209;249 |
| 80 | `heapsort_leftist_heap` |  |  |  | Y |  | Y | Y |  | 251&#8209;267 |
| 81 | `compare_all_heapsorts` |  |  |  | Y |  | Y | Y |  | 269&#8209;279 |
| 82 | `textbook_example` |  |  |  | Y |  | Y | Y |  | 306&#8209;310 |
| 83 | `reverse_sorted_example` |  |  |  | Y |  | Y | Y |  | 312&#8209;316 |
| 84 | `already_sorted_example` |  |  |  | Y |  | Y | Y |  | 318&#8209;322 |
| 85 | `duplicates_example` |  |  |  | Y |  | Y | Y |  | 324&#8209;328 |
| 86 | `single_element_example` |  |  |  | Y |  | Y | Y |  | 330&#8209;334 |
| 87 | `empty_example` |  |  |  | Y |  | Y | Y |  | 336&#8209;340 |
| 88 | `large_example` |  |  |  | Y |  | Y | Y |  | 342&#8209;352 |
| 89 | `efficiency_demonstration` |  |  |  | Y |  | Y | Y |  | 354&#8209;363 |
| 90 | `complexity_analysis` |  |  |  | Y |  | Y | Y |  | 365&#8209;394 |
| 91 | `correctness_verification` |  |  |  | Y |  | Y | Y |  | 396&#8209;410 |
| 92 | `vec_to_array_seq` |  |  |  | Y |  | Y | Y |  | 412&#8209;420 |
| 93 | `vec_to_avl_seq` |  |  |  | Y |  | Y | Y |  | 422&#8209;423 |
| 94 | `is_sorted` |  |  |  | Y |  | Y | Y |  | 425&#8209;426 |
| 95 | `generate_test_sequences` |  |  |  | Y |  | Y | Y |  | 428&#8209;443 |

### Chap45/LeftistHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 96 | `_leftist_heap_pq_verified` |  |  |  | Y | Y |  | Y |  | 81 |
| 97 | `total_order_le` |  |  |  | Y | Y |  |  | unknown | 84&#8209;86 |
| 98 | `lemma_total_size_monotone` |  |  |  | Y | Y |  |  | unknown | 104&#8209;107 |
| 99 | `lemma_heap_root_is_min` |  |  |  | Y | Y |  |  | unknown | 115&#8209;121 |
| 100 | `lemma_rank_le_size` |  |  |  | Y | Y |  |  | unknown | 176&#8209;178 |
| 101 | `rank` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;206 |
| 102 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;222 |
| 103 | `meld_nodes` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;236 |
| 104 | `size` x3 | Y | Y |  |  | Y |  |  | unknown | 318&#8209;320 |
| 105 | `height` x3 | Y | Y |  |  | Y |  |  | unknown | 330&#8209;332 |
| 106 | `is_leftist` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;246 |
| 107 | `is_heap` | Y | Y |  |  | Y |  |  | unknown | 247&#8209;248 |
| 108 | `is_rank_bounded` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;251 |
| 109 | `to_vec` x3 | Y | Y |  |  | Y |  |  | unknown | 343&#8209;345 |
| 110 | `empty` | Y | Y |  |  | Y |  |  | unknown | 264&#8209;268 |
| 111 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;273 |
| 112 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;281 |
| 113 | `insert` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;289 |
| 114 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;303 |
| 115 | `meld` | Y | Y |  |  | Y |  |  | unknown | 304&#8209;312 |
| 116 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;317 |
| 117 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;322 |
| 118 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;329 |
| 119 | `root_rank` | Y | Y |  |  | Y |  |  | unknown | 333&#8209;334 |
| 120 | `is_valid_leftist_heap` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;337 |
| 121 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 338&#8209;342 |
| 122 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 346&#8209;352 |
| 123 | `meld_multiple` | Y | Y |  |  | Y |  |  | unknown | 355&#8209;362 |
| 124 | `split` | Y | Y |  |  | Y |  |  | unknown | 363&#8209;367 |
| 125 | `default` |  | Y |  |  | Y |  | Y |  | 1108 |
| 126 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 1134&#8209;1136 |
| 127 | `format_node` x2 |  | Y |  |  |  | Y | Y |  | 1205&#8209;1215 |
| 128 | `efficient_multi_way_merge` |  |  |  | Y |  | Y | Y |  | 1246&#8209;1253 |
| 129 | `parallel_heap_construction` |  |  |  | Y |  | Y | Y |  | 1255&#8209;1258 |

### Chap45/SortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 130 | `_sorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 57 |
| 131 | `lemma_push_preserves_sorted` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;72 |
| 132 | `empty` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;79 |
| 133 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;87 |
| 134 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 135 | `insert` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;103 |
| 136 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;116 |
| 137 | `meld` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;127 |
| 138 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;134 |
| 139 | `size` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 140 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;140 |
| 141 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;144 |
| 142 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;154 |
| 143 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;162 |
| 144 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;168 |
| 145 | `delete_max` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;181 |
| 146 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;188 |
| 147 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;192 |
| 148 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;200 |
| 149 | `is_sorted` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;205 |
| 150 | `default` |  | Y |  |  | Y |  | Y |  | 1023 |
| 151 | `eq` |  | Y |  |  | Y |  |  | unknown | 1050&#8209;1051 |

### Chap45/UnsortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 152 | `_unsorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 57 |
| 153 | `empty` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;71 |
| 154 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;78 |
| 155 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;86 |
| 156 | `insert` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;94 |
| 157 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;114 |
| 158 | `meld` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;122 |
| 159 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 160 | `size` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 161 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;132 |
| 162 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;136 |
| 163 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;144 |
| 164 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;152 |
| 165 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;156 |
| 166 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;160 |
| 167 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;168 |
| 168 | `default` |  | Y |  |  | Y |  | Y |  | 629 |
| 169 | `eq` |  | Y |  |  | Y |  |  | unknown | 656&#8209;657 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
