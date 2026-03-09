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
| 1 | Chap45 | BalancedTreePQ | 26 | 28 | 0 | 1 | 29 | 0 | 11 | 16 | 2 |
| 2 | Chap45 | BinaryHeapPQ | 18 | 20 | 0 | 11 | 31 | 0 | 30 | 0 | 1 |
| 3 | Chap45 | Example45_2 | 8 | 8 | 0 | 9 | 9 | 0 | 8 | 0 | 1 |
| 4 | Chap45 | HeapsortExample | 2 | 3 | 0 | 21 | 4 | 20 | 1 | 0 | 23 |
| 5 | Chap45 | LeftistHeapPQ | 24 | 27 | 0 | 7 | 31 | 3 | 29 | 0 | 5 |
| 6 | Chap45 | SortedListPQ | 19 | 21 | 0 | 1 | 22 | 0 | 20 | 0 | 2 |
| 7 | Chap45 | UnsortedListPQ | 15 | 17 | 0 | 1 | 18 | 0 | 16 | 0 | 2 |

## Function-by-Function Detail

### Chap45/BalancedTreePQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `_balanced_tree_pq_verified` |  |  |  | Y | Y |  | Y |  | 52 |
| 2 | `empty` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 3 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 4 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;69 |
| 5 | `insert` | Y | Y |  |  | Y |  |  | hole | 71&#8209;73 |
| 6 | `delete_min` | Y | Y |  |  | Y |  |  | hole | 75&#8209;82 |
| 7 | `meld` | Y | Y |  |  | Y |  |  | hole | 84&#8209;86 |
| 8 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 88&#8209;90 |
| 9 | `size` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 10 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;98 |
| 11 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;102 |
| 12 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;108 |
| 13 | `delete_max` | Y | Y |  |  | Y |  |  | hole | 110&#8209;117 |
| 14 | `insert_all` | Y | Y |  |  | Y |  |  | hole | 119&#8209;121 |
| 15 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 16 | `contains` | Y | Y |  |  | Y |  |  | hole | 127&#8209;129 |
| 17 | `remove` | Y | Y |  |  | Y |  |  | hole | 131&#8209;136 |
| 18 | `range` | Y | Y |  |  | Y |  |  | hole | 138&#8209;140 |
| 19 | `from_vec` | Y | Y |  |  | Y |  |  | hole | 142&#8209;143 |
| 20 | `to_vec` | Y | Y |  |  | Y |  |  | hole | 145&#8209;147 |
| 21 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | hole | 149&#8209;151 |
| 22 | `is_sorted` | Y | Y |  |  | Y |  |  | hole | 153&#8209;155 |
| 23 | `height` | Y | Y |  |  | Y |  |  | hole | 157&#8209;159 |
| 24 | `split` | Y | Y |  |  | Y |  |  | hole | 161&#8209;166 |
| 25 | `join` | Y | Y |  |  | Y |  |  | hole | 168&#8209;170 |
| 26 | `filter` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;176 |
| 27 | `map` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;178 |
| 28 | `default` |  | Y |  |  | Y |  | Y |  | 426 |
| 29 | `eq` |  | Y |  |  | Y |  |  | unknown | 476&#8209;477 |

### Chap45/BinaryHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 30 | `lemma_log2_bound` |  |  |  | Y | Y |  |  | unknown | 76&#8209;83 |
| 31 | `empty` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;114 |
| 32 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;120 |
| 33 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;125 |
| 34 | `insert` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;133 |
| 35 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;145 |
| 36 | `meld` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;154 |
| 37 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;161 |
| 38 | `size` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 39 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;167 |
| 40 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;171 |
| 41 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;178 |
| 42 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;186 |
| 43 | `is_valid_heap` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;189 |
| 44 | `height` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;193 |
| 45 | `level_elements` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;199 |
| 46 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;205 |
| 47 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;209 |
| 48 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;217 |
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
| 94 | `_leftist_heap_pq_verified` |  |  |  | Y | Y |  | Y |  | 81 |
| 95 | `total_order_le` |  |  |  | Y | Y |  |  | unknown | 84&#8209;85 |
| 96 | `lemma_total_size_monotone` |  |  |  | Y | Y |  |  | unknown | 103&#8209;106 |
| 97 | `lemma_heap_root_is_min` |  |  |  | Y | Y |  |  | unknown | 114&#8209;120 |
| 98 | `lemma_rank_le_size` |  |  |  | Y | Y |  |  | unknown | 175&#8209;177 |
| 99 | `rank` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;205 |
| 100 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;221 |
| 101 | `meld_nodes` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;235 |
| 102 | `size` x3 | Y | Y |  |  | Y |  |  | unknown | 317&#8209;319 |
| 103 | `height` x3 | Y | Y |  |  | Y |  |  | unknown | 329&#8209;331 |
| 104 | `is_leftist` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;245 |
| 105 | `is_heap` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;247 |
| 106 | `is_rank_bounded` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;250 |
| 107 | `to_vec` x3 | Y | Y |  |  | Y |  |  | unknown | 342&#8209;344 |
| 108 | `empty` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;267 |
| 109 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;272 |
| 110 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;280 |
| 111 | `insert` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;288 |
| 112 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;302 |
| 113 | `meld` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;311 |
| 114 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;316 |
| 115 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;321 |
| 116 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 322&#8209;328 |
| 117 | `root_rank` | Y | Y |  |  | Y |  |  | unknown | 332&#8209;333 |
| 118 | `is_valid_leftist_heap` | Y | Y |  |  | Y |  |  | unknown | 334&#8209;336 |
| 119 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 337&#8209;341 |
| 120 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 345&#8209;351 |
| 121 | `meld_multiple` | Y | Y |  |  | Y |  |  | unknown | 354&#8209;361 |
| 122 | `split` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;366 |
| 123 | `default` |  | Y |  |  | Y |  | Y |  | 1105 |
| 124 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 1131&#8209;1133 |
| 125 | `format_node` x2 |  | Y |  |  |  | Y | Y |  | 1202&#8209;1212 |
| 126 | `efficient_multi_way_merge` |  |  |  | Y |  | Y | Y |  | 1243&#8209;1250 |
| 127 | `parallel_heap_construction` |  |  |  | Y |  | Y | Y |  | 1252&#8209;1255 |

### Chap45/SortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 128 | `_sorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 57 |
| 129 | `lemma_push_preserves_sorted` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;71 |
| 130 | `empty` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;77 |
| 131 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;84 |
| 132 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;90 |
| 133 | `insert` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;100 |
| 134 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;113 |
| 135 | `meld` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;124 |
| 136 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;131 |
| 137 | `size` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 138 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 139 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;141 |
| 140 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;151 |
| 141 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;159 |
| 142 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;165 |
| 143 | `delete_max` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;178 |
| 144 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;185 |
| 145 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;189 |
| 146 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;197 |
| 147 | `is_sorted` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;202 |
| 148 | `default` |  | Y |  |  | Y |  | Y |  | 1016 |
| 149 | `eq` |  | Y |  |  | Y |  |  | unknown | 1043&#8209;1044 |

### Chap45/UnsortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 150 | `_unsorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 57 |
| 151 | `empty` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 152 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 153 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;83 |
| 154 | `insert` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;91 |
| 155 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;111 |
| 156 | `meld` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;119 |
| 157 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;123 |
| 158 | `size` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;126 |
| 159 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 160 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 161 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;141 |
| 162 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;149 |
| 163 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;153 |
| 164 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;157 |
| 165 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;165 |
| 166 | `default` |  | Y |  |  | Y |  | Y |  | 622 |
| 167 | `eq` |  | Y |  |  | Y |  |  | unknown | 649&#8209;650 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
