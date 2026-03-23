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
| 1 | Chap37 | AVLTreeSeq | 20 | 23 | 0 | 14 | 37 | 0 | 34 | 2 | 1 |
| 2 | Chap37 | AVLTreeSeqMtPer | 11 | 14 | 0 | 13 | 27 | 0 | 24 | 2 | 1 |
| 3 | Chap37 | AVLTreeSeqStEph | 18 | 21 | 0 | 14 | 35 | 0 | 32 | 2 | 1 |
| 4 | Chap37 | AVLTreeSeqStPer | 13 | 16 | 0 | 16 | 32 | 0 | 30 | 1 | 1 |
| 5 | Chap37 | BSTAVLMtEph | 11 | 11 | 0 | 8 | 19 | 0 | 14 | 5 | 0 |
| 6 | Chap37 | BSTAVLStEph | 7 | 7 | 0 | 10 | 17 | 0 | 17 | 0 | 0 |
| 7 | Chap37 | BSTBBAlphaMtEph | 11 | 11 | 0 | 5 | 16 | 0 | 11 | 5 | 0 |
| 8 | Chap37 | BSTBBAlphaStEph | 10 | 10 | 0 | 7 | 17 | 0 | 17 | 0 | 0 |
| 9 | Chap37 | BSTPlainMtEph | 11 | 11 | 0 | 5 | 16 | 0 | 11 | 5 | 0 |
| 10 | Chap37 | BSTPlainStEph | 10 | 10 | 0 | 10 | 20 | 0 | 20 | 0 | 0 |
| 11 | Chap37 | BSTRBMtEph | 14 | 15 | 0 | 21 | 36 | 0 | 30 | 5 | 1 |
| 12 | Chap37 | BSTRBStEph | 7 | 7 | 0 | 8 | 15 | 0 | 15 | 0 | 0 |
| 13 | Chap37 | BSTSetAVLMtEph | 21 | 22 | 0 | 4 | 26 | 0 | 25 | 1 | 0 |
| 14 | Chap37 | BSTSetBBAlphaMtEph | 21 | 22 | 0 | 4 | 26 | 0 | 25 | 1 | 0 |
| 15 | Chap37 | BSTSetPlainMtEph | 21 | 22 | 0 | 4 | 26 | 0 | 25 | 1 | 0 |
| 16 | Chap37 | BSTSetRBMtEph | 21 | 22 | 0 | 3 | 25 | 0 | 24 | 1 | 0 |
| 17 | Chap37 | BSTSetSplayMtEph | 21 | 22 | 0 | 4 | 26 | 0 | 25 | 1 | 0 |
| 18 | Chap37 | BSTSplayMtEph | 14 | 15 | 0 | 18 | 33 | 0 | 27 | 5 | 1 |
| 19 | Chap37 | BSTSplayStEph | 11 | 12 | 0 | 13 | 25 | 0 | 24 | 0 | 1 |

## Function-by-Function Detail

### Chap37/AVLTreeSeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 183&#8209;186 |
| 2 | `lemma_inorder_values_maps_to_inorder` |  |  |  | Y | Y |  |  | unknown | 197&#8209;199 |
| 3 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 210&#8209;213 |
| 4 | `empty` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;232 |
| 5 | `new` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;235 |
| 6 | `length` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;239 |
| 7 | `nth` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;243 |
| 8 | `set` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;250 |
| 9 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;256 |
| 10 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;260 |
| 11 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;264 |
| 12 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;269 |
| 13 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;272 |
| 14 | `update` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;280 |
| 15 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;288 |
| 16 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;295 |
| 17 | `iter` | Y | Y |  |  | Y |  |  | unknown | 297&#8209;302 |
| 18 | `push_back` | Y | Y |  |  | Y |  |  | unknown | 304&#8209;306 |
| 19 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 308&#8209;311 |
| 20 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;315 |
| 21 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 317&#8209;328 |
| 22 | `is_tree_empty` | Y | Y |  |  | Y |  |  | unknown | 330&#8209;332 |
| 23 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 334&#8209;336 |
| 24 | `cached_height` |  |  |  | Y | Y |  |  | unknown | 344&#8209;346 |
| 25 | `cached_size` |  |  |  | Y | Y |  |  | unknown | 354&#8209;356 |
| 26 | `update_size_height` |  |  |  | Y | Y |  |  | unknown | 366&#8209;384 |
| 27 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 400&#8209;407 |
| 28 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 449&#8209;456 |
| 29 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 498&#8209;509 |
| 30 | `insert_at_link` |  |  |  | Y | Y |  |  | unknown | 553&#8209;564 |
| 31 | `nth_link` |  |  |  | Y | Y |  |  | unknown | 629&#8209;632 |
| 32 | `set_link` |  |  |  | Y | Y |  |  | unknown | 647&#8209;657 |
| 33 | `push_inorder` |  |  |  | Y | Y |  |  | unknown | 674&#8209;679 |
| 34 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 709&#8209;714 |
| 35 | `next` |  | Y |  |  | Y |  |  | hole | 1117&#8209;1133 |
| 36 | `default` |  | Y |  |  | Y |  | Y |  | 1188 |
| 37 | `eq` |  | Y |  |  | Y |  |  | hole | 1221&#8209;1222 |

### Chap37/AVLTreeSeqMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 172&#8209;175 |
| 39 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 187&#8209;190 |
| 40 | `empty` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;211 |
| 41 | `new` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;214 |
| 42 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;217 |
| 43 | `length` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;221 |
| 44 | `nth` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;225 |
| 45 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;229 |
| 46 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;233 |
| 47 | `set` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;240 |
| 48 | `subseq_copy` | Y | Y |  |  | Y |  |  | hole | 242&#8209;243 |
| 49 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;248 |
| 50 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;251 |
| 51 | `height_fn` |  |  |  | Y | Y |  |  | unknown | 259&#8209;261 |
| 52 | `size_fn` |  |  |  | Y | Y |  |  | unknown | 269&#8209;271 |
| 53 | `mk` |  |  |  | Y | Y |  |  | unknown | 279&#8209;290 |
| 54 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 299&#8209;304 |
| 55 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 333&#8209;338 |
| 56 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 367&#8209;372 |
| 57 | `nth_ref` |  |  |  | Y | Y |  |  | unknown | 433&#8209;436 |
| 58 | `set_rec` |  |  |  | Y | Y |  |  | unknown | 451&#8209;460 |
| 59 | `inorder_collect` |  |  |  | Y | Y |  |  | unknown | 497&#8209;500 |
| 60 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | unknown | 509&#8209;514 |
| 61 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 546&#8209;551 |
| 62 | `default` |  | Y |  |  | Y |  | Y |  | 685 |
| 63 | `next` |  | Y |  |  | Y |  |  | unknown | 692&#8209;693 |
| 64 | `eq` |  | Y |  |  | Y |  |  | hole | 729&#8209;730 |

### Chap37/AVLTreeSeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 65 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 179&#8209;182 |
| 66 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 194&#8209;197 |
| 67 | `empty` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;218 |
| 68 | `new` | Y | Y |  |  | Y |  |  | unknown | 220&#8209;221 |
| 69 | `length` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;225 |
| 70 | `nth` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;229 |
| 71 | `set` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;236 |
| 72 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;242 |
| 73 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;246 |
| 74 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;250 |
| 75 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;254 |
| 76 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;257 |
| 77 | `update` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;265 |
| 78 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;274 |
| 79 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 276&#8209;281 |
| 80 | `iter` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;284 |
| 81 | `push_back` | Y | Y |  |  | Y |  |  | unknown | 286&#8209;292 |
| 82 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;297 |
| 83 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;305 |
| 84 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;318 |
| 85 | `h_fn` |  |  |  | Y | Y |  |  | unknown | 326&#8209;328 |
| 86 | `size_link_fn` |  |  |  | Y | Y |  |  | unknown | 336&#8209;338 |
| 87 | `update_meta` |  |  |  | Y | Y |  |  | unknown | 348&#8209;362 |
| 88 | `rotate_right_fn` |  |  |  | Y | Y |  |  | unknown | 379&#8209;386 |
| 89 | `rotate_left_fn` |  |  |  | Y | Y |  |  | unknown | 417&#8209;424 |
| 90 | `rebalance_fn` |  |  |  | Y | Y |  |  | unknown | 455&#8209;464 |
| 91 | `insert_at_link` |  |  |  | Y | Y |  |  | unknown | 513&#8209;524 |
| 92 | `nth_link` |  |  |  | Y | Y |  |  | unknown | 577&#8209;580 |
| 93 | `set_link` |  |  |  | Y | Y |  |  | unknown | 595&#8209;605 |
| 94 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 632&#8209;637 |
| 95 | `clone_link` |  |  |  | Y | Y |  |  | hole | 675&#8209;681 |
| 96 | `default` |  | Y |  |  | Y |  | Y |  | 1035 |
| 97 | `push_left_iter` |  |  |  | Y | Y |  |  | unknown | 1041&#8209;1043 |
| 98 | `next` |  | Y |  |  | Y |  |  | unknown | 1063&#8209;1064 |
| 99 | `eq` |  | Y |  |  | Y |  |  | hole | 1103&#8209;1104 |

### Chap37/AVLTreeSeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 100 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 138&#8209;141 |
| 101 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 153&#8209;156 |
| 102 | `lemma_size_lt_usize_max` |  |  |  | Y | Y |  |  | unknown | 169&#8209;172 |
| 103 | `lemma_wf_implies_len_bound_stper` |  |  |  | Y | Y |  |  | unknown | 187&#8209;189 |
| 104 | `empty` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;206 |
| 105 | `new` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;209 |
| 106 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;212 |
| 107 | `length` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;216 |
| 108 | `nth` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;220 |
| 109 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;224 |
| 110 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;228 |
| 111 | `set` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;238 |
| 112 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;242 |
| 113 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;248 |
| 114 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;252 |
| 115 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;257 |
| 116 | `iter` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;260 |
| 117 | `height_fn` |  |  |  | Y | Y |  |  | unknown | 265&#8209;267 |
| 118 | `size_fn` |  |  |  | Y | Y |  |  | unknown | 275&#8209;277 |
| 119 | `mk` |  |  |  | Y | Y |  |  | unknown | 285&#8209;296 |
| 120 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 305&#8209;310 |
| 121 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 350&#8209;355 |
| 122 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 392&#8209;397 |
| 123 | `nth_ref` |  |  |  | Y | Y |  |  | unknown | 463&#8209;466 |
| 124 | `set_rec` |  |  |  | Y | Y |  |  | unknown | 481&#8209;491 |
| 125 | `inorder_collect` |  |  |  | Y | Y |  |  | unknown | 532&#8209;535 |
| 126 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | unknown | 544&#8209;549 |
| 127 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 591&#8209;596 |
| 128 | `default` |  | Y |  |  | Y |  | Y |  | 780 |
| 129 | `push_left_iter_stper` |  |  |  | Y | Y |  |  | unknown | 794&#8209;796 |
| 130 | `next` |  | Y |  |  | Y |  |  | unknown | 820&#8209;821 |
| 131 | `eq` |  | Y |  |  | Y |  |  | hole | 843&#8209;844 |

### Chap37/BSTAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 132 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 39&#8209;71 |
| 133 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 92&#8209;96 |
| 134 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 158&#8209;162 |
| 135 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 225&#8209;234 |
| 136 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 317&#8209;320 |
| 137 | `find_node` |  |  |  | Y | Y |  |  | unknown | 342&#8209;347 |
| 138 | `min_node` |  |  |  | Y | Y |  |  | unknown | 369&#8209;374 |
| 139 | `max_node` |  |  |  | Y | Y |  |  | unknown | 385&#8209;390 |
| 140 | `new` | Y | Y |  |  | Y |  |  | unknown | 443&#8209;445 |
| 141 | `insert` | Y | Y |  |  | Y |  |  | hole | 447&#8209;455 |
| 142 | `contains` | Y | Y |  |  | Y |  |  | hole | 457&#8209;459 |
| 143 | `size` | Y | Y |  |  | Y |  |  | hole | 461&#8209;463 |
| 144 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 465&#8209;467 |
| 145 | `height` | Y | Y |  |  | Y |  |  | hole | 469&#8209;471 |
| 146 | `find` | Y | Y |  |  | Y |  |  | unknown | 473&#8209;474 |
| 147 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 475&#8209;476 |
| 148 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 477&#8209;478 |
| 149 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 479&#8209;480 |
| 150 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 481&#8209;482 |

### Chap37/BSTAVLStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 151 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 67&#8209;99 |
| 152 | `lemma_max_plus_one` |  |  |  | Y | Y |  |  | unknown | 117&#8209;119 |
| 153 | `new` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;133 |
| 154 | `size` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 155 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;141 |
| 156 | `height` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;146 |
| 157 | `insert` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;157 |
| 158 | `contains` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;162 |
| 159 | `find` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;169 |
| 160 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 176&#8209;214 |
| 161 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 334&#8209;372 |
| 162 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 497&#8209;525 |
| 163 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 785&#8209;796 |
| 164 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 941&#8209;944 |
| 165 | `find_node` |  |  |  | Y | Y |  |  | unknown | 976&#8209;981 |
| 166 | `min_node` |  |  |  | Y | Y |  |  | unknown | 1013&#8209;1019 |
| 167 | `max_node` |  |  |  | Y | Y |  |  | unknown | 1035&#8209;1041 |

### Chap37/BSTBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 168 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 40&#8209;49 |
| 169 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 132&#8209;135 |
| 170 | `find_node` |  |  |  | Y | Y |  |  | unknown | 157&#8209;162 |
| 171 | `min_node` |  |  |  | Y | Y |  |  | unknown | 184&#8209;189 |
| 172 | `max_node` |  |  |  | Y | Y |  |  | unknown | 200&#8209;205 |
| 173 | `new` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;260 |
| 174 | `insert` | Y | Y |  |  | Y |  |  | hole | 262&#8209;270 |
| 175 | `contains` | Y | Y |  |  | Y |  |  | hole | 272&#8209;274 |
| 176 | `size` | Y | Y |  |  | Y |  |  | hole | 276&#8209;278 |
| 177 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 280&#8209;282 |
| 178 | `height` | Y | Y |  |  | Y |  |  | hole | 284&#8209;286 |
| 179 | `find` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;289 |
| 180 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;291 |
| 181 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;293 |
| 182 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;295 |
| 183 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 296&#8209;297 |

### Chap37/BSTBBAlphaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 184 | `new` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 185 | `size` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 186 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;84 |
| 187 | `height` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 188 | `insert` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;99 |
| 189 | `contains` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;104 |
| 190 | `find` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;111 |
| 191 | `delete` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;121 |
| 192 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;129 |
| 193 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;137 |
| 194 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 209&#8209;216 |
| 195 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 346&#8209;349 |
| 196 | `find_node` |  |  |  | Y | Y |  |  | unknown | 381&#8209;386 |
| 197 | `min_node` |  |  |  | Y | Y |  |  | unknown | 418&#8209;424 |
| 198 | `max_node` |  |  |  | Y | Y |  |  | unknown | 440&#8209;446 |
| 199 | `delete_min_node` |  |  |  | Y | Y |  |  | unknown | 463&#8209;474 |
| 200 | `delete_node` |  |  |  | Y | Y |  |  | unknown | 581&#8209;588 |

### Chap37/BSTPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 201 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 39&#8209;48 |
| 202 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 131&#8209;134 |
| 203 | `find_node` |  |  |  | Y | Y |  |  | unknown | 156&#8209;161 |
| 204 | `min_node` |  |  |  | Y | Y |  |  | unknown | 183&#8209;188 |
| 205 | `max_node` |  |  |  | Y | Y |  |  | unknown | 199&#8209;204 |
| 206 | `new` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;259 |
| 207 | `insert` | Y | Y |  |  | Y |  |  | hole | 261&#8209;269 |
| 208 | `contains` | Y | Y |  |  | Y |  |  | hole | 271&#8209;273 |
| 209 | `size` | Y | Y |  |  | Y |  |  | hole | 275&#8209;277 |
| 210 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 279&#8209;281 |
| 211 | `height` | Y | Y |  |  | Y |  |  | hole | 283&#8209;285 |
| 212 | `find` | Y | Y |  |  | Y |  |  | unknown | 287&#8209;288 |
| 213 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;290 |
| 214 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;292 |
| 215 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;294 |
| 216 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;296 |

### Chap37/BSTPlainStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 217 | `lemma_node_contains` |  |  |  | Y | Y |  |  | unknown | 40&#8209;46 |
| 218 | `lemma_bst_left` |  |  |  | Y | Y |  |  | unknown | 50&#8209;59 |
| 219 | `lemma_bst_right` |  |  |  | Y | Y |  |  | unknown | 63&#8209;72 |
| 220 | `new` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;91 |
| 221 | `size` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;96 |
| 222 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 223 | `height` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;104 |
| 224 | `insert` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;114 |
| 225 | `contains` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;119 |
| 226 | `find` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;126 |
| 227 | `delete` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;136 |
| 228 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;144 |
| 229 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;152 |
| 230 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 265&#8209;272 |
| 231 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 402&#8209;405 |
| 232 | `find_node` |  |  |  | Y | Y |  |  | unknown | 437&#8209;442 |
| 233 | `min_node` |  |  |  | Y | Y |  |  | unknown | 474&#8209;480 |
| 234 | `max_node` |  |  |  | Y | Y |  |  | unknown | 496&#8209;502 |
| 235 | `delete_min_node` |  |  |  | Y | Y |  |  | unknown | 519&#8209;530 |
| 236 | `delete_node` |  |  |  | Y | Y |  |  | unknown | 639&#8209;646 |

### Chap37/BSTRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 237 | `new_node` |  |  |  | Y | Y |  |  | unknown | 116&#8209;122 |
| 238 | `is_red` |  |  |  | Y | Y |  |  | unknown | 133&#8209;136 |
| 239 | `size_link` |  |  |  | Y | Y |  |  | unknown | 144&#8209;147 |
| 240 | `update` |  |  |  | Y | Y |  |  | unknown | 155&#8209;161 |
| 241 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 170&#8209;174 |
| 242 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 253&#8209;257 |
| 243 | `flip_colors` |  |  |  | Y | Y |  |  | unknown | 336&#8209;340 |
| 244 | `fix_up` |  |  |  | Y | Y |  |  | unknown | 362&#8209;366 |
| 245 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 403&#8209;410 |
| 246 | `find_link` |  |  |  | Y | Y |  |  | unknown | 525&#8209;530 |
| 247 | `min_link` |  |  |  | Y | Y |  |  | unknown | 562&#8209;568 |
| 248 | `max_link` |  |  |  | Y | Y |  |  | unknown | 610&#8209;616 |
| 249 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 658&#8209;661 |
| 250 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 670&#8209;673 |
| 251 | `in_order_parallel` |  |  |  | Y | Y |  |  | unknown | 682&#8209;685 |
| 252 | `pre_order_parallel` |  |  |  | Y | Y |  |  | unknown | 703&#8209;706 |
| 253 | `build_balanced` |  |  |  | Y | Y |  |  | unknown | 725&#8209;727 |
| 254 | `filter_parallel` |  |  |  | Y | Y |  |  | unknown | 753&#8209;758 |
| 255 | `reduce_parallel` |  |  |  | Y | Y |  |  | unknown | 782&#8209;787 |
| 256 | `height_rec` |  |  |  | Y | Y |  |  | unknown | 808&#8209;811 |
| 257 | `compute_link_spec_size` |  |  |  | Y | Y |  |  | unknown | 820&#8209;823 |
| 258 | `new` | Y | Y |  |  | Y |  |  | unknown | 873&#8209;875 |
| 259 | `from_sorted_slice` | Y | Y |  |  | Y |  |  | unknown | 877&#8209;878 |
| 260 | `insert` | Y | Y |  |  | Y |  |  | hole | 880&#8209;886 |
| 261 | `contains` | Y | Y |  |  | Y |  |  | hole | 888&#8209;890 |
| 262 | `size` | Y | Y |  |  | Y |  |  | hole | 892&#8209;894 |
| 263 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 896&#8209;898 |
| 264 | `height` | Y | Y |  |  | Y |  |  | hole | 900&#8209;902 |
| 265 | `find` | Y | Y |  |  | Y |  |  | unknown | 904&#8209;905 |
| 266 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 906&#8209;907 |
| 267 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 908&#8209;909 |
| 268 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 910&#8209;911 |
| 269 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 912&#8209;913 |
| 270 | `filter` | Y | Y |  |  | Y |  |  | unknown | 914&#8209;917 |
| 271 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 918&#8209;921 |
| 272 | `default` |  | Y |  |  | Y |  | Y |  | 1061 |

### Chap37/BSTRBStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 273 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 51&#8209;83 |
| 274 | `new` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;110 |
| 275 | `size` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;115 |
| 276 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 277 | `height` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;123 |
| 278 | `insert` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;133 |
| 279 | `contains` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 280 | `find` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;145 |
| 281 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 153&#8209;159 |
| 282 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 253&#8209;259 |
| 283 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 359&#8209;366 |
| 284 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 490&#8209;493 |
| 285 | `find_node` |  |  |  | Y | Y |  |  | unknown | 525&#8209;530 |
| 286 | `min_node` |  |  |  | Y | Y |  |  | unknown | 562&#8209;568 |
| 287 | `max_node` |  |  |  | Y | Y |  |  | unknown | 584&#8209;590 |

### Chap37/BSTSetAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 288 | `empty` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 289 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 290 | `size` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 291 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;85 |
| 292 | `find` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 293 | `contains` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;91 |
| 294 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 295 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 296 | `insert` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 297 | `delete` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 298 | `union` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 299 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 300 | `difference` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;112 |
| 301 | `split` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 302 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 303 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 304 | `filter` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 305 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;127 |
| 306 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;130 |
| 307 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 308 | `iter` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;136 |
| 309 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 141&#8209;143 |
| 310 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 149&#8209;150 |
| 311 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 160&#8209;163 |
| 312 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 172&#8209;174 |
| 313 | `next` |  | Y |  |  | Y |  |  | hole | 383&#8209;399 |

### Chap37/BSTSetBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 314 | `empty` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;61 |
| 315 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 316 | `size` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;66 |
| 317 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;69 |
| 318 | `find` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;72 |
| 319 | `contains` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;75 |
| 320 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;78 |
| 321 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;81 |
| 322 | `insert` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;84 |
| 323 | `delete` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 324 | `union` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;90 |
| 325 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;93 |
| 326 | `difference` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 327 | `split` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 328 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;102 |
| 329 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;105 |
| 330 | `filter` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;108 |
| 331 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;111 |
| 332 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;114 |
| 333 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;117 |
| 334 | `iter` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;120 |
| 335 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 123&#8209;125 |
| 336 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 130&#8209;131 |
| 337 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 141&#8209;144 |
| 338 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 153&#8209;155 |
| 339 | `next` |  | Y |  |  | Y |  |  | hole | 309&#8209;325 |

### Chap37/BSTSetPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 340 | `empty` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;61 |
| 341 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 342 | `size` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;66 |
| 343 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;69 |
| 344 | `find` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;72 |
| 345 | `contains` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;75 |
| 346 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;78 |
| 347 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;81 |
| 348 | `insert` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;84 |
| 349 | `delete` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 350 | `union` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;90 |
| 351 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;93 |
| 352 | `difference` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 353 | `split` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 354 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;102 |
| 355 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;105 |
| 356 | `filter` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;108 |
| 357 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;111 |
| 358 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;114 |
| 359 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;117 |
| 360 | `iter` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;120 |
| 361 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 123&#8209;125 |
| 362 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 130&#8209;131 |
| 363 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 141&#8209;144 |
| 364 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 153&#8209;155 |
| 365 | `next` |  | Y |  |  | Y |  |  | hole | 365&#8209;381 |

### Chap37/BSTSetRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 366 | `empty` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 367 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 368 | `size` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 369 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;70 |
| 370 | `find` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;73 |
| 371 | `contains` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;76 |
| 372 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 373 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 374 | `insert` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;85 |
| 375 | `delete` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 376 | `union` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;91 |
| 377 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 378 | `difference` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 379 | `split` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 380 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 381 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 382 | `filter` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 383 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;112 |
| 384 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 385 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 386 | `iter` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 387 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 124&#8209;126 |
| 388 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 131&#8209;133 |
| 389 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 140&#8209;141 |
| 390 | `next` |  | Y |  |  | Y |  |  | hole | 386&#8209;402 |

### Chap37/BSTSetSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 391 | `empty` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 392 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 393 | `size` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 394 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;86 |
| 395 | `find` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;89 |
| 396 | `contains` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 397 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 398 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;98 |
| 399 | `insert` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |
| 400 | `delete` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;104 |
| 401 | `union` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 402 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 403 | `difference` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;113 |
| 404 | `split` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 405 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 406 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 407 | `filter` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 408 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 409 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 410 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 411 | `iter` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;137 |
| 412 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 142&#8209;144 |
| 413 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 150&#8209;151 |
| 414 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 161&#8209;164 |
| 415 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 173&#8209;175 |
| 416 | `next` |  | Y |  |  | Y |  |  | hole | 384&#8209;400 |

### Chap37/BSTSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 417 | `new_node` |  |  |  | Y | Y |  |  | unknown | 109&#8209;115 |
| 418 | `size_link` |  |  |  | Y | Y |  |  | unknown | 125&#8209;128 |
| 419 | `update` |  |  |  | Y | Y |  |  | unknown | 136&#8209;141 |
| 420 | `splay` |  |  |  | Y | Y |  |  | unknown | 153&#8209;158 |
| 421 | `bst_insert` |  |  |  | Y | Y |  |  | unknown | 1130&#8209;1137 |
| 422 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 1263&#8209;1269 |
| 423 | `find_link` |  |  |  | Y | Y |  |  | unknown | 1281&#8209;1286 |
| 424 | `min_link` |  |  |  | Y | Y |  |  | unknown | 1318&#8209;1324 |
| 425 | `max_link` |  |  |  | Y | Y |  |  | unknown | 1366&#8209;1372 |
| 426 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 1414&#8209;1417 |
| 427 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 1426&#8209;1429 |
| 428 | `in_order_parallel` |  |  |  | Y | Y |  |  | unknown | 1438&#8209;1441 |
| 429 | `pre_order_parallel` |  |  |  | Y | Y |  |  | unknown | 1459&#8209;1462 |
| 430 | `build_balanced` |  |  |  | Y | Y |  |  | unknown | 1481&#8209;1483 |
| 431 | `filter_parallel` |  |  |  | Y | Y |  |  | unknown | 1508&#8209;1513 |
| 432 | `reduce_parallel` |  |  |  | Y | Y |  |  | unknown | 1537&#8209;1542 |
| 433 | `height_rec` |  |  |  | Y | Y |  |  | unknown | 1563&#8209;1566 |
| 434 | `compute_link_spec_size` |  |  |  | Y | Y |  |  | unknown | 1575&#8209;1578 |
| 435 | `new` | Y | Y |  |  | Y |  |  | unknown | 1628&#8209;1630 |
| 436 | `from_sorted_slice` | Y | Y |  |  | Y |  |  | unknown | 1632&#8209;1633 |
| 437 | `insert` | Y | Y |  |  | Y |  |  | hole | 1635&#8209;1641 |
| 438 | `contains` | Y | Y |  |  | Y |  |  | hole | 1643&#8209;1645 |
| 439 | `size` | Y | Y |  |  | Y |  |  | hole | 1647&#8209;1649 |
| 440 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 1651&#8209;1653 |
| 441 | `height` | Y | Y |  |  | Y |  |  | hole | 1655&#8209;1657 |
| 442 | `find` | Y | Y |  |  | Y |  |  | unknown | 1659&#8209;1660 |
| 443 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 1661&#8209;1662 |
| 444 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 1663&#8209;1664 |
| 445 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 1665&#8209;1666 |
| 446 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 1667&#8209;1668 |
| 447 | `filter` | Y | Y |  |  | Y |  |  | unknown | 1669&#8209;1672 |
| 448 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 1673&#8209;1676 |
| 449 | `default` |  | Y |  |  | Y |  | Y |  | 1813 |

### Chap37/BSTSplayStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 450 | `lemma_bst_deep_link` |  |  |  | Y | Y |  |  | unknown | 132&#8209;164 |
| 451 | `new` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;199 |
| 452 | `size` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;202 |
| 453 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;205 |
| 454 | `height` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;210 |
| 455 | `insert` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;216 |
| 456 | `find` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;221 |
| 457 | `contains` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;224 |
| 458 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;230 |
| 459 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;236 |
| 460 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;239 |
| 461 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;242 |
| 462 | `new_node` |  |  |  | Y | Y |  |  | unknown | 251&#8209;257 |
| 463 | `size_link` |  |  |  | Y | Y |  |  | unknown | 270&#8209;271 |
| 464 | `height_link` |  |  |  | Y | Y |  |  | unknown | 282&#8209;285 |
| 465 | `update` |  |  |  | Y | Y |  |  | unknown | 302&#8209;306 |
| 466 | `splay` |  |  |  | Y | Y |  |  | unknown | 320&#8209;325 |
| 467 | `bst_insert` |  |  |  | Y | Y |  |  | unknown | 1299&#8209;1306 |
| 468 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 1442&#8209;1448 |
| 469 | `find_link` |  |  |  | Y | Y |  |  | unknown | 1462&#8209;1467 |
| 470 | `min_link` |  |  |  | Y | Y |  |  | unknown | 1501&#8209;1507 |
| 471 | `max_link` |  |  |  | Y | Y |  |  | unknown | 1553&#8209;1559 |
| 472 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 1606&#8209;1609 |
| 473 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 1621&#8209;1624 |
| 474 | `default` |  | Y |  |  | Y |  | Y |  | 1703 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
