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
| 1 | Chap39 | BSTParaTreapMtEph | 16 | 16 | 1 | 19 | 36 | 0 | 35 | 1 | 0 |
| 2 | Chap39 | BSTSetTreapMtEph | 20 | 20 | 1 | 2 | 23 | 0 | 23 | 0 | 0 |
| 3 | Chap39 | BSTTreapMtEph | 25 | 26 | 0 | 21 | 35 | 0 | 28 | 6 | 1 |
| 4 | Chap39 | BSTTreapSpecsAndLemmas | 0 | 0 | 0 | 11 | 11 | 0 | 11 | 0 | 0 |
| 5 | Chap39 | BSTTreapStEph | 51 | 52 | 1 | 19 | 72 | 0 | 71 | 1 | 0 |

## Function-by-Function Detail

### Chap39/BSTParaTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `clone_elem` |  |  |  | Y | Y |  |  | unknown | 80&#8209;81 |
| 2 | `new_param_treap` |  |  |  | Y | Y |  |  | unknown | 103&#8209;110 |
| 3 | `new_leaf` |  |  |  | Y | Y |  |  | unknown | 120&#8209;121 |
| 4 | `expose_internal` |  |  |  | Y | Y |  |  | unknown | 127&#8209;132 |
| 5 | `expose_with_priority_internal` |  |  |  | Y | Y |  |  | unknown | 184&#8209;189 |
| 6 | `priority_for` |  |  |  | Y | Y |  |  | hole | 220 |
| 7 | `tree_priority_internal` |  |  |  | Y | Y |  |  | unknown | 229&#8209;231 |
| 8 | `make_node` |  |  |  | Y | Y |  |  | unknown | 244&#8209;256 |
| 9 | `join_with_priority` |  |  |  | Y | Y |  |  | unknown | 281&#8209;291 |
| 10 | `split_inner` |  |  |  | Y | Y |  |  | unknown | 415&#8209;425 |
| 11 | `join_pair_inner` |  |  |  | Y | Y |  |  | unknown | 572&#8209;581 |
| 12 | `union_inner` |  |  |  | Y | Y |  |  | unknown | 710&#8209;716 |
| 13 | `intersect_inner` |  |  |  | Y | Y |  |  | unknown | 803&#8209;810 |
| 14 | `difference_inner` |  |  |  | Y | Y |  |  | unknown | 924&#8209;930 |
| 15 | `filter_inner` |  |  |  | Y | Y |  |  | unknown | 1037&#8209;1057 |
| 16 | `filter_parallel` |  |  |  | Y | Y |  |  | unknown | 1121&#8209;1139 |
| 17 | `reduce_inner` |  |  |  | Y | Y |  |  | unknown | 1146&#8209;1155 |
| 18 | `reduce_parallel` |  |  |  | Y | Y |  |  | unknown | 1179&#8209;1186 |
| 19 | `collect_in_order` |  |  |  | Y | Y |  |  | unknown | 1195&#8209;1200 |
| 20 | `new` | Y | Y |  |  | Y |  |  | unknown | 1262&#8209;1263 |
| 21 | `expose` | Y | Y |  |  | Y |  |  | unknown | 1266&#8209;1278 |
| 22 | `join_mid` | Y | Y |  |  | Y |  |  | unknown | 1281&#8209;1295 |
| 23 | `size` | Y | Y |  |  | Y |  |  | unknown | 1298&#8209;1299 |
| 24 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 1302&#8209;1303 |
| 25 | `insert` | Y | Y |  |  | Y |  |  | unknown | 1306&#8209;1310 |
| 26 | `delete` | Y | Y |  |  | Y |  |  | unknown | 1313&#8209;1318 |
| 27 | `find` | Y | Y |  |  | Y |  |  | unknown | 1321&#8209;1325 |
| 28 | `split` | Y | Y |  |  | Y |  |  | unknown | 1328&#8209;1336 |
| 29 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 1339&#8209;1347 |
| 30 | `union` | Y | Y |  |  | Y |  |  | unknown | 1350&#8209;1355 |
| 31 | `intersect` | Y | Y |  |  | Y |  |  | unknown | 1358&#8209;1363 |
| 32 | `difference` | Y | Y |  |  | Y |  |  | unknown | 1366&#8209;1371 |
| 33 | `filter` | Y | Y |  |  | Y |  |  | unknown | 1374&#8209;1391 |
| 34 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 1394&#8209;1402 |
| 35 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 1405&#8209;1407 |
| 36 | `iter` |  |  | Y |  | Y |  |  | unknown | 1639&#8209;1644 |

### Chap39/BSTSetTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 37 | `empty` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 38 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;81 |
| 39 | `size` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 40 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;89 |
| 41 | `find` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;96 |
| 42 | `contains` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |
| 43 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;108 |
| 44 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;115 |
| 45 | `insert` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;123 |
| 46 | `delete` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;131 |
| 47 | `union` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;139 |
| 48 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;147 |
| 49 | `difference` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;155 |
| 50 | `split` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;169 |
| 51 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;180 |
| 52 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;193 |
| 53 | `filter` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;213 |
| 54 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;223 |
| 55 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;228 |
| 56 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;232 |
| 57 | `minimum_inner` |  |  |  | Y | Y |  |  | unknown | 240&#8209;244 |
| 58 | `maximum_inner` |  |  |  | Y | Y |  |  | unknown | 263&#8209;267 |
| 59 | `iter` |  |  | Y |  | Y |  |  | unknown | 448&#8209;453 |

### Chap39/BSTTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 60 | `size_link` | Y | Y |  | Y | Y |  |  | unknown | 980&#8209;982 |
| 61 | `update` |  |  |  | Y | Y |  |  | unknown | 100&#8209;109 |
| 62 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 118&#8209;126 |
| 63 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 202&#8209;210 |
| 64 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 287&#8209;299 |
| 65 | `delete_link` |  |  |  | Y | Y |  |  | unknown | 373&#8209;385 |
| 66 | `find_link` | Y | Y |  | Y | Y |  |  | unknown | 986&#8209;992 |
| 67 | `min_link` | Y | Y |  | Y | Y |  |  | unknown | 996&#8209;1004 |
| 68 | `max_link` | Y | Y |  | Y | Y |  |  | unknown | 1008&#8209;1016 |
| 69 | `height_link` | Y | Y |  | Y | Y |  |  | unknown | 1020&#8209;1024 |
| 70 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 688&#8209;691 |
| 71 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 702&#8209;705 |
| 72 | `lemma_bst_decompose` | Y | Y |  | Y | Y |  |  | unknown | 915&#8209;925 |
| 73 | `lemma_contains_left` | Y | Y |  | Y | Y |  |  | unknown | 927&#8209;929 |
| 74 | `lemma_contains_right` | Y | Y |  | Y | Y |  |  | unknown | 931&#8209;933 |
| 75 | `lemma_contains_root` | Y | Y |  | Y | Y |  |  | unknown | 935&#8209;936 |
| 76 | `lemma_contains_implies_in_set` |  |  |  | Y | Y |  |  | unknown | 805&#8209;808 |
| 77 | `lemma_height_le_size` | Y | Y |  | Y | Y |  |  | unknown | 938&#8209;942 |
| 78 | `lemma_size_wf_child_bounded` | Y | Y |  | Y | Y |  |  | unknown | 944&#8209;956 |
| 79 | `lemma_wf_assemble_node` | Y | Y |  | Y | Y |  |  | unknown | 971&#8209;976 |
| 80 | `lemma_wf_decompose` | Y | Y |  |  | Y |  |  | unknown | 958&#8209;967 |
| 81 | `new` | Y | Y |  |  | Y |  |  | unknown | 1028&#8209;1029 |
| 82 | `insert` | Y | Y |  |  | Y |  |  | unknown | 1032&#8209;1041 |
| 83 | `delete` | Y | Y |  |  | Y |  |  | unknown | 1044&#8209;1052 |
| 84 | `find` | Y | Y |  |  | Y |  |  | hole | 1055&#8209;1061 |
| 85 | `contains` | Y | Y |  |  | Y |  |  | unknown | 1064&#8209;1068 |
| 86 | `size` | Y | Y |  |  | Y |  |  | hole | 1071&#8209;1072 |
| 87 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 1075&#8209;1076 |
| 88 | `height` | Y | Y |  |  | Y |  |  | unknown | 1079&#8209;1080 |
| 89 | `minimum` | Y | Y |  |  | Y |  |  | hole | 1083&#8209;1084 |
| 90 | `maximum` | Y | Y |  |  | Y |  |  | hole | 1087&#8209;1088 |
| 91 | `in_order` | Y | Y |  |  | Y |  |  | hole | 1091&#8209;1092 |
| 92 | `pre_order` | Y | Y |  |  | Y |  |  | hole | 1095&#8209;1096 |
| 93 | `clone_link` |  |  |  | Y | Y |  |  | unknown | 1459&#8209;1464 |
| 94 | `default` |  | Y |  |  | Y |  | Y |  | 1547 |

### Chap39/BSTTreapSpecsAndLemmas.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 95 | `lemma_cmp_antisymmetry` |  |  |  | Y | Y |  |  | unknown | 47&#8209;51 |
| 96 | `lemma_cmp_antisymmetry_less` |  |  |  | Y | Y |  |  | unknown | 58&#8209;62 |
| 97 | `lemma_cmp_transitivity` |  |  |  | Y | Y |  |  | unknown | 69&#8209;74 |
| 98 | `lemma_cmp_eq_subst` |  |  |  | Y | Y |  |  | unknown | 81&#8209;87 |
| 99 | `lemma_cmp_equal_congruent` |  |  |  | Y | Y |  |  | unknown | 94&#8209;99 |
| 100 | `lemma_cmp_equal_congruent_right` |  |  |  | Y | Y |  |  | unknown | 106&#8209;111 |
| 101 | `lemma_joined_right_gt_lk` |  |  |  | Y | Y |  |  | unknown | 119&#8209;136 |
| 102 | `lemma_joined_left_lt_rk` |  |  |  | Y | Y |  |  | unknown | 155&#8209;172 |
| 103 | `lemma_split_result_subset` |  |  |  | Y | Y |  |  | unknown | 191&#8209;198 |
| 104 | `lemma_union_part_subset` |  |  |  | Y | Y |  |  | unknown | 217&#8209;219 |
| 105 | `lemma_halves_cross_ordered` |  |  |  | Y | Y |  |  | unknown | 225&#8209;235 |

### Chap39/BSTTreapStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 106 | `lemma_height_le_size` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;157 |
| 107 | `lemma_size_wf_child_bounded` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;171 |
| 108 | `lemma_wf_decompose` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;182 |
| 109 | `lemma_wf_assemble_node` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;189 |
| 110 | `lemma_contains_left` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;193 |
| 111 | `lemma_contains_right` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;197 |
| 112 | `lemma_bst_decompose` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;209 |
| 113 | `lemma_contains_root` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;212 |
| 114 | `new` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;222 |
| 115 | `size` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;226 |
| 116 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;230 |
| 117 | `height` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;237 |
| 118 | `insert` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;251 |
| 119 | `delete` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;262 |
| 120 | `find` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;272 |
| 121 | `contains` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;280 |
| 122 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;288 |
| 123 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;296 |
| 124 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;300 |
| 125 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;304 |
| 126 | `new_node` | Y | Y |  |  | Y |  |  | unknown | 308&#8209;311 |
| 127 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;316 |
| 128 | `update_size` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;326 |
| 129 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;337 |
| 130 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 340&#8209;348 |
| 131 | `clone_link` | Y | Y |  |  | Y |  |  | unknown | 351&#8209;354 |
| 132 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 357&#8209;361 |
| 133 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 364&#8209;376 |
| 134 | `delete_link` | Y | Y |  |  | Y |  |  | unknown | 379&#8209;387 |
| 135 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 390&#8209;396 |
| 136 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 399&#8209;404 |
| 137 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 407&#8209;412 |
| 138 | `in_order_vec` | Y | Y |  |  | Y |  |  | unknown | 415&#8209;416 |
| 139 | `pre_order_vec` | Y | Y |  |  | Y |  |  | unknown | 419&#8209;420 |
| 140 | `param_new` | Y | Y |  |  | Y |  |  | unknown | 433&#8209;434 |
| 141 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 438&#8209;442 |
| 142 | `expose` | Y | Y |  |  | Y |  |  | unknown | 446&#8209;464 |
| 143 | `join_mid` | Y | Y |  |  | Y |  |  | unknown | 468&#8209;485 |
| 144 | `param_size` | Y | Y |  |  | Y |  |  | unknown | 489&#8209;491 |
| 145 | `param_is_empty` | Y | Y |  |  | Y |  |  | unknown | 495&#8209;497 |
| 146 | `param_insert` | Y | Y |  |  | Y |  |  | unknown | 501&#8209;508 |
| 147 | `param_delete` | Y | Y |  |  | Y |  |  | unknown | 512&#8209;520 |
| 148 | `param_find` | Y | Y |  |  | Y |  |  | unknown | 524&#8209;531 |
| 149 | `param_split` | Y | Y |  |  | Y |  |  | unknown | 535&#8209;548 |
| 150 | `param_join_pair` | Y | Y |  |  | Y |  |  | unknown | 552&#8209;564 |
| 151 | `param_union` | Y | Y |  |  | Y |  |  | unknown | 568&#8209;577 |
| 152 | `param_intersect` | Y | Y |  |  | Y |  |  | unknown | 581&#8209;590 |
| 153 | `param_difference` | Y | Y |  |  | Y |  |  | unknown | 594&#8209;603 |
| 154 | `param_filter` | Y | Y |  |  | Y |  |  | unknown | 607&#8209;626 |
| 155 | `param_reduce` | Y | Y |  |  | Y |  |  | unknown | 630&#8209;637 |
| 156 | `param_in_order` | Y | Y |  |  | Y |  |  | unknown | 641&#8209;646 |
| 157 | `lemma_wf_view_inhabited_st` |  |  |  | Y | Y |  |  | unknown | 1815&#8209;1820 |
| 158 | `lemma_wf_view_all_inhabited_st` |  |  |  | Y | Y |  |  | unknown | 1839&#8209;1844 |
| 159 | `lemma_wf_size_eq_view_len` |  |  |  | Y | Y |  |  | unknown | 1874&#8209;1877 |
| 160 | `lemma_param_wf_implies_size_wf` |  |  |  | Y | Y |  |  | unknown | 1894&#8209;1897 |
| 161 | `clone_elem_st` |  |  |  | Y | Y |  |  | unknown | 1916&#8209;1917 |
| 162 | `clone_with_view` |  |  |  | Y | Y |  |  | unknown | 1928&#8209;1930 |
| 163 | `priority_for_st` |  |  |  | Y | Y |  |  | hole | 1945 |
| 164 | `make_node_treap_st` |  |  |  | Y | Y |  |  | unknown | 1957&#8209;1973 |
| 165 | `tree_priority_st` |  |  |  | Y | Y |  |  | unknown | 2000&#8209;2002 |
| 166 | `expose_to_parts_st` |  |  |  | Y | Y |  |  | unknown | 2012&#8209;2021 |
| 167 | `join_with_priority_st` |  |  |  | Y | Y |  |  | unknown | 2053&#8209;2070 |
| 168 | `split_inner_st` |  |  |  | Y | Y |  |  | unknown | 2131&#8209;2148 |
| 169 | `join_pair_inner_st` |  |  |  | Y | Y |  |  | unknown | 2295&#8209;2310 |
| 170 | `union_inner_st` |  |  |  | Y | Y |  |  | unknown | 2406&#8209;2418 |
| 171 | `intersect_inner_st` |  |  |  | Y | Y |  |  | unknown | 2500&#8209;2512 |
| 172 | `difference_inner_st` |  |  |  | Y | Y |  |  | unknown | 2605&#8209;2618 |
| 173 | `filter_inner_st` |  |  |  | Y | Y |  |  | unknown | 2838&#8209;2860 |
| 174 | `reduce_inner_st` |  |  |  | Y | Y |  |  | unknown | 3019&#8209;3030 |
| 175 | `collect_in_order_st` |  |  |  | Y | Y |  |  | unknown | 3050&#8209;3059 |
| 176 | `iter` |  |  | Y |  | Y |  |  | unknown | 3101&#8209;3106 |
| 177 | `default` |  | Y |  |  | Y |  |  | unknown | 3133&#8209;3134 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
