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
| 1 | Chap45 | BalancedTreePQ | 26 | 28 | 0 | 1 | 29 | 0 | 27 | 1 | 1 |
| 2 | Chap45 | BinaryHeapPQ | 18 | 20 | 0 | 17 | 37 | 0 | 35 | 1 | 1 |
| 3 | Chap45 | Example45_2 | 8 | 8 | 0 | 9 | 9 | 0 | 0 | 0 | 9 |
| 4 | Chap45 | HeapsortExample | 2 | 3 | 0 | 22 | 5 | 20 | 0 | 1 | 24 |
| 5 | Chap45 | LeftistHeapPQ | 24 | 27 | 0 | 7 | 31 | 3 | 28 | 1 | 5 |
| 6 | Chap45 | SortedListPQ | 19 | 21 | 0 | 1 | 22 | 0 | 20 | 0 | 2 |
| 7 | Chap45 | UnsortedListPQ | 15 | 17 | 0 | 1 | 18 | 0 | 15 | 1 | 2 |

## Function-by-Function Detail

### Chap45/BalancedTreePQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `_balanced_tree_pq_verified` |  |  |  | Y | Y |  | Y |  | 54 |
| 2 | `empty` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 3 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 4 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;72 |
| 5 | `insert` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;81 |
| 6 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;90 |
| 7 | `meld` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;97 |
| 8 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |
| 9 | `size` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;105 |
| 10 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 11 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;113 |
| 12 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;120 |
| 13 | `delete_max` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;129 |
| 14 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;136 |
| 15 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;140 |
| 16 | `contains` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;144 |
| 17 | `remove` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;151 |
| 18 | `range` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;155 |
| 19 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;159 |
| 20 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;163 |
| 21 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;167 |
| 22 | `is_sorted` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;171 |
| 23 | `height` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;175 |
| 24 | `split` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;182 |
| 25 | `join` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;189 |
| 26 | `filter` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;198 |
| 27 | `map` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;203 |
| 28 | `default` |  | Y |  |  | Y |  |  | unknown | 673&#8209;674 |
| 29 | `eq` |  | Y |  |  | Y |  |  | hole | 761&#8209;762 |

### Chap45/BinaryHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 30 | `lemma_log2_bound` |  |  |  | Y | Y |  |  | unknown | 78&#8209;85 |
| 31 | `lemma_heap_root_le_all` |  |  |  | Y | Y |  |  | unknown | 99&#8209;106 |
| 32 | `lemma_heap_parent_le` |  |  |  | Y | Y |  |  | unknown | 129&#8209;135 |
| 33 | `empty` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;178 |
| 34 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;186 |
| 35 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;196 |
| 36 | `insert` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;206 |
| 37 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;225 |
| 38 | `meld` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;234 |
| 39 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;241 |
| 40 | `size` | Y | Y |  |  | Y |  |  | unknown | 243&#8209;244 |
| 41 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;247 |
| 42 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;251 |
| 43 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;258 |
| 44 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;267 |
| 45 | `is_valid_heap` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;270 |
| 46 | `height` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;274 |
| 47 | `level_elements` | Y | Y |  |  | Y |  |  | unknown | 276&#8209;280 |
| 48 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;286 |
| 49 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;290 |
| 50 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;299 |
| 51 | `left_child` |  |  |  | Y | Y |  |  | unknown | 311&#8209;313 |
| 52 | `right_child` |  |  |  | Y | Y |  |  | unknown | 318&#8209;320 |
| 53 | `parent` |  |  |  | Y | Y |  |  | unknown | 325&#8209;327 |
| 54 | `lemma_swap_preserves_multiset` |  |  |  | Y | Y |  |  | unknown | 332&#8209;334 |
| 55 | `lemma_le_preserved_by_multiset_eq` |  |  |  | Y | Y |  |  | unknown | 361&#8209;370 |
| 56 | `swap_elements` |  |  |  | Y | Y |  |  | unknown | 381&#8209;389 |
| 57 | `bubble_up` |  |  |  | Y | Y |  |  | unknown | 449&#8209;456 |
| 58 | `bubble_down` |  |  |  | Y | Y |  |  | unknown | 501&#8209;509 |
| 59 | `bubble_down_heap` |  |  |  | Y | Y |  |  | unknown | 564&#8209;573 |
| 60 | `bubble_up_heap` |  |  |  | Y | Y |  |  | unknown | 739&#8209;763 |
| 61 | `heapify` |  |  |  | Y | Y |  |  | unknown | 923&#8209;930 |
| 62 | `is_heap` |  |  |  | Y | Y |  |  | unknown | 973&#8209;975 |
| 63 | `exec_pow2` |  |  |  | Y | Y |  |  | unknown | 1001&#8209;1003 |
| 64 | `exec_log2` |  |  |  | Y | Y |  |  | unknown | 1028&#8209;1031 |
| 65 | `default` |  | Y |  |  | Y |  | Y |  | 1758 |
| 66 | `eq` |  | Y |  |  | Y |  |  | hole | 1793&#8209;1794 |

### Chap45/Example45_2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 67 | `_example_45_2_verified` |  |  |  | Y | Y |  | Y |  | 27 |
| 68 | `example_45_2_textbook_example` | Y | Y |  | Y | Y |  | Y |  | 32 |
| 69 | `example_45_2_reverse_sorted` | Y | Y |  | Y | Y |  | Y |  | 33 |
| 70 | `example_45_2_already_sorted` | Y | Y |  | Y | Y |  | Y |  | 34 |
| 71 | `example_45_2_duplicates` | Y | Y |  | Y | Y |  | Y |  | 35 |
| 72 | `example_45_2_single_element` | Y | Y |  | Y | Y |  | Y |  | 36 |
| 73 | `example_45_2_empty` | Y | Y |  | Y | Y |  | Y |  | 37 |
| 74 | `example_45_2_efficiency_demonstration` | Y | Y |  | Y | Y |  | Y |  | 38 |
| 75 | `run_example_45_2` | Y | Y |  | Y | Y |  | Y |  | 39 |

### Chap45/HeapsortExample.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 76 | `eq` |  | Y |  |  | Y |  |  | hole | 103&#8209;104 |
| 77 | `_heapsort_example_verified` |  |  |  | Y | Y |  | Y |  | 125 |
| 78 | `is_vec_sorted_exec` |  |  |  | Y | Y |  | Y |  | 130 |
| 79 | `all_results_match` | Y | Y |  |  | Y |  | Y |  | 151 |
| 80 | `all_results_sorted` | Y | Y |  |  | Y |  | Y |  | 153 |
| 81 | `heapsort_unsorted_list` |  |  |  | Y |  | Y | Y |  | 177&#8209;193 |
| 82 | `heapsort_sorted_list` |  |  |  | Y |  | Y | Y |  | 195&#8209;211 |
| 83 | `heapsort_balanced_tree` |  |  |  | Y |  | Y | Y |  | 213&#8209;229 |
| 84 | `heapsort_binary_heap` |  |  |  | Y |  | Y | Y |  | 231&#8209;247 |
| 85 | `heapsort_leftist_heap` |  |  |  | Y |  | Y | Y |  | 249&#8209;265 |
| 86 | `compare_all_heapsorts` |  |  |  | Y |  | Y | Y |  | 267&#8209;277 |
| 87 | `textbook_example` |  |  |  | Y |  | Y | Y |  | 304&#8209;308 |
| 88 | `reverse_sorted_example` |  |  |  | Y |  | Y | Y |  | 310&#8209;314 |
| 89 | `already_sorted_example` |  |  |  | Y |  | Y | Y |  | 316&#8209;320 |
| 90 | `duplicates_example` |  |  |  | Y |  | Y | Y |  | 322&#8209;326 |
| 91 | `single_element_example` |  |  |  | Y |  | Y | Y |  | 328&#8209;332 |
| 92 | `empty_example` |  |  |  | Y |  | Y | Y |  | 334&#8209;338 |
| 93 | `large_example` |  |  |  | Y |  | Y | Y |  | 340&#8209;350 |
| 94 | `efficiency_demonstration` |  |  |  | Y |  | Y | Y |  | 352&#8209;361 |
| 95 | `complexity_analysis` |  |  |  | Y |  | Y | Y |  | 363&#8209;392 |
| 96 | `correctness_verification` |  |  |  | Y |  | Y | Y |  | 394&#8209;408 |
| 97 | `vec_to_array_seq` |  |  |  | Y |  | Y | Y |  | 410&#8209;418 |
| 98 | `vec_to_avl_seq` |  |  |  | Y |  | Y | Y |  | 420&#8209;421 |
| 99 | `is_sorted` |  |  |  | Y |  | Y | Y |  | 423&#8209;424 |
| 100 | `generate_test_sequences` |  |  |  | Y |  | Y | Y |  | 426&#8209;441 |

### Chap45/LeftistHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 101 | `_leftist_heap_pq_verified` |  |  |  | Y | Y |  | Y |  | 80 |
| 102 | `total_order_le` |  |  |  | Y | Y |  |  | unknown | 84&#8209;85 |
| 103 | `lemma_total_size_monotone` |  |  |  | Y | Y |  |  | unknown | 103&#8209;106 |
| 104 | `lemma_heap_root_is_min` |  |  |  | Y | Y |  |  | unknown | 114&#8209;120 |
| 105 | `lemma_rank_le_size` |  |  |  | Y | Y |  |  | unknown | 175&#8209;177 |
| 106 | `rank` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;205 |
| 107 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;221 |
| 108 | `meld_nodes` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;235 |
| 109 | `size` x3 | Y | Y |  |  | Y |  |  | unknown | 317&#8209;319 |
| 110 | `height` x3 | Y | Y |  |  | Y |  |  | unknown | 329&#8209;331 |
| 111 | `is_leftist` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;245 |
| 112 | `is_heap` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;247 |
| 113 | `is_rank_bounded` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;250 |
| 114 | `to_vec` x3 | Y | Y |  |  | Y |  |  | unknown | 342&#8209;344 |
| 115 | `empty` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;267 |
| 116 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;272 |
| 117 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;280 |
| 118 | `insert` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;288 |
| 119 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;302 |
| 120 | `meld` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;311 |
| 121 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;316 |
| 122 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;321 |
| 123 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 322&#8209;328 |
| 124 | `root_rank` | Y | Y |  |  | Y |  |  | unknown | 332&#8209;333 |
| 125 | `is_valid_leftist_heap` | Y | Y |  |  | Y |  |  | unknown | 334&#8209;336 |
| 126 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 337&#8209;341 |
| 127 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 345&#8209;351 |
| 128 | `meld_multiple` | Y | Y |  |  | Y |  |  | unknown | 354&#8209;361 |
| 129 | `split` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;366 |
| 130 | `default` |  | Y |  |  | Y |  | Y |  | 1114 |
| 131 | `eq` x2 |  | Y |  |  | Y |  |  | hole | 1140&#8209;1142 |
| 132 | `format_node` x2 |  | Y |  |  |  | Y | Y |  | 1211&#8209;1221 |
| 133 | `efficient_multi_way_merge` |  |  |  | Y |  | Y | Y |  | 1252&#8209;1259 |
| 134 | `parallel_heap_construction` |  |  |  | Y |  | Y | Y |  | 1261&#8209;1264 |

### Chap45/SortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 135 | `_sorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 56 |
| 136 | `lemma_push_preserves_sorted` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;71 |
| 137 | `empty` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;78 |
| 138 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;86 |
| 139 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 140 | `insert` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;102 |
| 141 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;115 |
| 142 | `meld` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;126 |
| 143 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;133 |
| 144 | `size` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;136 |
| 145 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;139 |
| 146 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;143 |
| 147 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;153 |
| 148 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;161 |
| 149 | `find_max` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;167 |
| 150 | `delete_max` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;180 |
| 151 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;187 |
| 152 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;191 |
| 153 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;199 |
| 154 | `is_sorted` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;204 |
| 155 | `default` |  | Y |  |  | Y |  | Y |  | 1029 |
| 156 | `eq` |  | Y |  |  | Y |  |  | unknown | 1056&#8209;1057 |

### Chap45/UnsortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 157 | `_unsorted_list_pq_verified` |  |  |  | Y | Y |  | Y |  | 56 |
| 158 | `empty` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;70 |
| 159 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;77 |
| 160 | `find_min` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;85 |
| 161 | `insert` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;93 |
| 162 | `delete_min` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;113 |
| 163 | `meld` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;121 |
| 164 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 165 | `size` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;128 |
| 166 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 167 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;135 |
| 168 | `insert_all` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;143 |
| 169 | `extract_all_sorted` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;151 |
| 170 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;155 |
| 171 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;159 |
| 172 | `to_sorted_vec` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;167 |
| 173 | `default` |  | Y |  |  | Y |  | Y |  | 635 |
| 174 | `eq` |  | Y |  |  | Y |  |  | hole | 662&#8209;663 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
