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
| 3 | Chap37 | AVLTreeSeqStEph | 18 | 21 | 0 | 16 | 37 | 0 | 34 | 2 | 1 |
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
| 66 | `lemma_size_lt_usize_max` |  |  |  | Y | Y |  |  | unknown | 194&#8209;197 |
| 67 | `lemma_wf_implies_len_bound_steph` |  |  |  | Y | Y |  |  | unknown | 209&#8209;211 |
| 68 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 218&#8209;221 |
| 69 | `empty` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;242 |
| 70 | `new` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;245 |
| 71 | `length` | Y | Y |  |  | Y |  |  | unknown | 247&#8209;249 |
| 72 | `nth` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;253 |
| 73 | `set` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;260 |
| 74 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;266 |
| 75 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;270 |
| 76 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;274 |
| 77 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 276&#8209;278 |
| 78 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;281 |
| 79 | `update` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;289 |
| 80 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;298 |
| 81 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;305 |
| 82 | `iter` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;308 |
| 83 | `push_back` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;316 |
| 84 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 318&#8209;321 |
| 85 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;329 |
| 86 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;342 |
| 87 | `h_fn` |  |  |  | Y | Y |  |  | unknown | 350&#8209;352 |
| 88 | `size_link_fn` |  |  |  | Y | Y |  |  | unknown | 360&#8209;362 |
| 89 | `update_meta` |  |  |  | Y | Y |  |  | unknown | 372&#8209;386 |
| 90 | `rotate_right_fn` |  |  |  | Y | Y |  |  | unknown | 403&#8209;410 |
| 91 | `rotate_left_fn` |  |  |  | Y | Y |  |  | unknown | 441&#8209;448 |
| 92 | `rebalance_fn` |  |  |  | Y | Y |  |  | unknown | 479&#8209;488 |
| 93 | `insert_at_link` |  |  |  | Y | Y |  |  | unknown | 537&#8209;548 |
| 94 | `nth_link` |  |  |  | Y | Y |  |  | unknown | 601&#8209;604 |
| 95 | `set_link` |  |  |  | Y | Y |  |  | unknown | 619&#8209;629 |
| 96 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 656&#8209;661 |
| 97 | `clone_link` |  |  |  | Y | Y |  |  | hole | 699&#8209;705 |
| 98 | `default` |  | Y |  |  | Y |  | Y |  | 1059 |
| 99 | `push_left_iter` |  |  |  | Y | Y |  |  | unknown | 1065&#8209;1067 |
| 100 | `next` |  | Y |  |  | Y |  |  | unknown | 1087&#8209;1088 |
| 101 | `eq` |  | Y |  |  | Y |  |  | hole | 1127&#8209;1128 |

### Chap37/AVLTreeSeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 102 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 138&#8209;141 |
| 103 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 153&#8209;156 |
| 104 | `lemma_size_lt_usize_max` |  |  |  | Y | Y |  |  | unknown | 169&#8209;172 |
| 105 | `lemma_wf_implies_len_bound_stper` |  |  |  | Y | Y |  |  | unknown | 187&#8209;189 |
| 106 | `empty` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;206 |
| 107 | `new` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;209 |
| 108 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;212 |
| 109 | `length` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;216 |
| 110 | `nth` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;220 |
| 111 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;224 |
| 112 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;228 |
| 113 | `set` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;238 |
| 114 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;242 |
| 115 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;248 |
| 116 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;252 |
| 117 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;257 |
| 118 | `iter` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;260 |
| 119 | `height_fn` |  |  |  | Y | Y |  |  | unknown | 265&#8209;267 |
| 120 | `size_fn` |  |  |  | Y | Y |  |  | unknown | 275&#8209;277 |
| 121 | `mk` |  |  |  | Y | Y |  |  | unknown | 285&#8209;296 |
| 122 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 305&#8209;310 |
| 123 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 350&#8209;355 |
| 124 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 392&#8209;397 |
| 125 | `nth_ref` |  |  |  | Y | Y |  |  | unknown | 463&#8209;466 |
| 126 | `set_rec` |  |  |  | Y | Y |  |  | unknown | 481&#8209;491 |
| 127 | `inorder_collect` |  |  |  | Y | Y |  |  | unknown | 532&#8209;535 |
| 128 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | unknown | 544&#8209;549 |
| 129 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 591&#8209;596 |
| 130 | `default` |  | Y |  |  | Y |  | Y |  | 780 |
| 131 | `push_left_iter_stper` |  |  |  | Y | Y |  |  | unknown | 794&#8209;796 |
| 132 | `next` |  | Y |  |  | Y |  |  | unknown | 820&#8209;821 |
| 133 | `eq` |  | Y |  |  | Y |  |  | hole | 843&#8209;844 |

### Chap37/BSTAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 134 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 39&#8209;71 |
| 135 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 92&#8209;96 |
| 136 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 158&#8209;162 |
| 137 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 225&#8209;234 |
| 138 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 317&#8209;320 |
| 139 | `find_node` |  |  |  | Y | Y |  |  | unknown | 342&#8209;347 |
| 140 | `min_node` |  |  |  | Y | Y |  |  | unknown | 369&#8209;374 |
| 141 | `max_node` |  |  |  | Y | Y |  |  | unknown | 385&#8209;390 |
| 142 | `new` | Y | Y |  |  | Y |  |  | unknown | 443&#8209;445 |
| 143 | `insert` | Y | Y |  |  | Y |  |  | hole | 447&#8209;455 |
| 144 | `contains` | Y | Y |  |  | Y |  |  | hole | 457&#8209;459 |
| 145 | `size` | Y | Y |  |  | Y |  |  | hole | 461&#8209;463 |
| 146 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 465&#8209;467 |
| 147 | `height` | Y | Y |  |  | Y |  |  | hole | 469&#8209;471 |
| 148 | `find` | Y | Y |  |  | Y |  |  | unknown | 473&#8209;474 |
| 149 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 475&#8209;476 |
| 150 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 477&#8209;478 |
| 151 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 479&#8209;480 |
| 152 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 481&#8209;482 |

### Chap37/BSTAVLStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 153 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 67&#8209;99 |
| 154 | `lemma_max_plus_one` |  |  |  | Y | Y |  |  | unknown | 117&#8209;119 |
| 155 | `new` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;133 |
| 156 | `size` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 157 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;141 |
| 158 | `height` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;146 |
| 159 | `insert` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;157 |
| 160 | `contains` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;162 |
| 161 | `find` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;169 |
| 162 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 176&#8209;214 |
| 163 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 334&#8209;372 |
| 164 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 497&#8209;525 |
| 165 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 785&#8209;796 |
| 166 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 941&#8209;944 |
| 167 | `find_node` |  |  |  | Y | Y |  |  | unknown | 976&#8209;981 |
| 168 | `min_node` |  |  |  | Y | Y |  |  | unknown | 1013&#8209;1019 |
| 169 | `max_node` |  |  |  | Y | Y |  |  | unknown | 1035&#8209;1041 |

### Chap37/BSTBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 170 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 40&#8209;49 |
| 171 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 132&#8209;135 |
| 172 | `find_node` |  |  |  | Y | Y |  |  | unknown | 157&#8209;162 |
| 173 | `min_node` |  |  |  | Y | Y |  |  | unknown | 184&#8209;189 |
| 174 | `max_node` |  |  |  | Y | Y |  |  | unknown | 200&#8209;205 |
| 175 | `new` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;260 |
| 176 | `insert` | Y | Y |  |  | Y |  |  | hole | 262&#8209;270 |
| 177 | `contains` | Y | Y |  |  | Y |  |  | hole | 272&#8209;274 |
| 178 | `size` | Y | Y |  |  | Y |  |  | hole | 276&#8209;278 |
| 179 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 280&#8209;282 |
| 180 | `height` | Y | Y |  |  | Y |  |  | hole | 284&#8209;286 |
| 181 | `find` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;289 |
| 182 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;291 |
| 183 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;293 |
| 184 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;295 |
| 185 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 296&#8209;297 |

### Chap37/BSTBBAlphaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 186 | `new` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 187 | `size` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 188 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;84 |
| 189 | `height` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 190 | `insert` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;99 |
| 191 | `contains` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;104 |
| 192 | `find` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;111 |
| 193 | `delete` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;121 |
| 194 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;129 |
| 195 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;137 |
| 196 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 209&#8209;216 |
| 197 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 346&#8209;349 |
| 198 | `find_node` |  |  |  | Y | Y |  |  | unknown | 381&#8209;386 |
| 199 | `min_node` |  |  |  | Y | Y |  |  | unknown | 418&#8209;424 |
| 200 | `max_node` |  |  |  | Y | Y |  |  | unknown | 440&#8209;446 |
| 201 | `delete_min_node` |  |  |  | Y | Y |  |  | unknown | 463&#8209;474 |
| 202 | `delete_node` |  |  |  | Y | Y |  |  | unknown | 581&#8209;588 |

### Chap37/BSTPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 203 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 39&#8209;48 |
| 204 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 131&#8209;134 |
| 205 | `find_node` |  |  |  | Y | Y |  |  | unknown | 156&#8209;161 |
| 206 | `min_node` |  |  |  | Y | Y |  |  | unknown | 183&#8209;188 |
| 207 | `max_node` |  |  |  | Y | Y |  |  | unknown | 199&#8209;204 |
| 208 | `new` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;259 |
| 209 | `insert` | Y | Y |  |  | Y |  |  | hole | 261&#8209;269 |
| 210 | `contains` | Y | Y |  |  | Y |  |  | hole | 271&#8209;273 |
| 211 | `size` | Y | Y |  |  | Y |  |  | hole | 275&#8209;277 |
| 212 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 279&#8209;281 |
| 213 | `height` | Y | Y |  |  | Y |  |  | hole | 283&#8209;285 |
| 214 | `find` | Y | Y |  |  | Y |  |  | unknown | 287&#8209;288 |
| 215 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;290 |
| 216 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;292 |
| 217 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;294 |
| 218 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;296 |

### Chap37/BSTPlainStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 219 | `lemma_node_contains` |  |  |  | Y | Y |  |  | unknown | 40&#8209;46 |
| 220 | `lemma_bst_left` |  |  |  | Y | Y |  |  | unknown | 50&#8209;59 |
| 221 | `lemma_bst_right` |  |  |  | Y | Y |  |  | unknown | 63&#8209;72 |
| 222 | `new` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;91 |
| 223 | `size` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;96 |
| 224 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 225 | `height` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;104 |
| 226 | `insert` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;114 |
| 227 | `contains` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;119 |
| 228 | `find` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;126 |
| 229 | `delete` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;136 |
| 230 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;144 |
| 231 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;152 |
| 232 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 265&#8209;272 |
| 233 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 402&#8209;405 |
| 234 | `find_node` |  |  |  | Y | Y |  |  | unknown | 437&#8209;442 |
| 235 | `min_node` |  |  |  | Y | Y |  |  | unknown | 474&#8209;480 |
| 236 | `max_node` |  |  |  | Y | Y |  |  | unknown | 496&#8209;502 |
| 237 | `delete_min_node` |  |  |  | Y | Y |  |  | unknown | 519&#8209;530 |
| 238 | `delete_node` |  |  |  | Y | Y |  |  | unknown | 639&#8209;646 |

### Chap37/BSTRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 239 | `new_node` |  |  |  | Y | Y |  |  | unknown | 116&#8209;122 |
| 240 | `is_red` |  |  |  | Y | Y |  |  | unknown | 133&#8209;136 |
| 241 | `size_link` |  |  |  | Y | Y |  |  | unknown | 144&#8209;147 |
| 242 | `update` |  |  |  | Y | Y |  |  | unknown | 155&#8209;161 |
| 243 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 170&#8209;174 |
| 244 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 253&#8209;257 |
| 245 | `flip_colors` |  |  |  | Y | Y |  |  | unknown | 336&#8209;340 |
| 246 | `fix_up` |  |  |  | Y | Y |  |  | unknown | 362&#8209;366 |
| 247 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 403&#8209;410 |
| 248 | `find_link` |  |  |  | Y | Y |  |  | unknown | 525&#8209;530 |
| 249 | `min_link` |  |  |  | Y | Y |  |  | unknown | 562&#8209;568 |
| 250 | `max_link` |  |  |  | Y | Y |  |  | unknown | 610&#8209;616 |
| 251 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 658&#8209;661 |
| 252 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 670&#8209;673 |
| 253 | `in_order_parallel` |  |  |  | Y | Y |  |  | unknown | 682&#8209;685 |
| 254 | `pre_order_parallel` |  |  |  | Y | Y |  |  | unknown | 703&#8209;706 |
| 255 | `build_balanced` |  |  |  | Y | Y |  |  | unknown | 725&#8209;727 |
| 256 | `filter_parallel` |  |  |  | Y | Y |  |  | unknown | 753&#8209;758 |
| 257 | `reduce_parallel` |  |  |  | Y | Y |  |  | unknown | 782&#8209;787 |
| 258 | `height_rec` |  |  |  | Y | Y |  |  | unknown | 808&#8209;811 |
| 259 | `compute_link_spec_size` |  |  |  | Y | Y |  |  | unknown | 820&#8209;823 |
| 260 | `new` | Y | Y |  |  | Y |  |  | unknown | 873&#8209;875 |
| 261 | `from_sorted_slice` | Y | Y |  |  | Y |  |  | unknown | 877&#8209;878 |
| 262 | `insert` | Y | Y |  |  | Y |  |  | hole | 880&#8209;886 |
| 263 | `contains` | Y | Y |  |  | Y |  |  | hole | 888&#8209;890 |
| 264 | `size` | Y | Y |  |  | Y |  |  | hole | 892&#8209;894 |
| 265 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 896&#8209;898 |
| 266 | `height` | Y | Y |  |  | Y |  |  | hole | 900&#8209;902 |
| 267 | `find` | Y | Y |  |  | Y |  |  | unknown | 904&#8209;905 |
| 268 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 906&#8209;907 |
| 269 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 908&#8209;909 |
| 270 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 910&#8209;911 |
| 271 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 912&#8209;913 |
| 272 | `filter` | Y | Y |  |  | Y |  |  | unknown | 914&#8209;917 |
| 273 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 918&#8209;921 |
| 274 | `default` |  | Y |  |  | Y |  | Y |  | 1061 |

### Chap37/BSTRBStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 275 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 51&#8209;83 |
| 276 | `new` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;110 |
| 277 | `size` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;115 |
| 278 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 279 | `height` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;123 |
| 280 | `insert` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;133 |
| 281 | `contains` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 282 | `find` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;145 |
| 283 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 153&#8209;159 |
| 284 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 253&#8209;259 |
| 285 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 359&#8209;366 |
| 286 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 490&#8209;493 |
| 287 | `find_node` |  |  |  | Y | Y |  |  | unknown | 525&#8209;530 |
| 288 | `min_node` |  |  |  | Y | Y |  |  | unknown | 562&#8209;568 |
| 289 | `max_node` |  |  |  | Y | Y |  |  | unknown | 584&#8209;590 |

### Chap37/BSTSetAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 290 | `empty` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 291 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 292 | `size` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 293 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;85 |
| 294 | `find` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 295 | `contains` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;91 |
| 296 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 297 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 298 | `insert` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 299 | `delete` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 300 | `union` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 301 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 302 | `difference` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;112 |
| 303 | `split` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 304 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 305 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 306 | `filter` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 307 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;127 |
| 308 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;130 |
| 309 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 310 | `iter` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;136 |
| 311 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 141&#8209;143 |
| 312 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 149&#8209;150 |
| 313 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 160&#8209;163 |
| 314 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 172&#8209;174 |
| 315 | `next` |  | Y |  |  | Y |  |  | hole | 383&#8209;399 |

### Chap37/BSTSetBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 316 | `empty` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;61 |
| 317 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 318 | `size` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;66 |
| 319 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;69 |
| 320 | `find` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;72 |
| 321 | `contains` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;75 |
| 322 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;78 |
| 323 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;81 |
| 324 | `insert` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;84 |
| 325 | `delete` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 326 | `union` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;90 |
| 327 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;93 |
| 328 | `difference` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 329 | `split` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 330 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;102 |
| 331 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;105 |
| 332 | `filter` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;108 |
| 333 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;111 |
| 334 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;114 |
| 335 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;117 |
| 336 | `iter` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;120 |
| 337 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 123&#8209;125 |
| 338 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 130&#8209;131 |
| 339 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 141&#8209;144 |
| 340 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 153&#8209;155 |
| 341 | `next` |  | Y |  |  | Y |  |  | hole | 309&#8209;325 |

### Chap37/BSTSetPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 342 | `empty` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;61 |
| 343 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 344 | `size` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;66 |
| 345 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;69 |
| 346 | `find` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;72 |
| 347 | `contains` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;75 |
| 348 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;78 |
| 349 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;81 |
| 350 | `insert` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;84 |
| 351 | `delete` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 352 | `union` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;90 |
| 353 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;93 |
| 354 | `difference` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 355 | `split` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 356 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;102 |
| 357 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;105 |
| 358 | `filter` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;108 |
| 359 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;111 |
| 360 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;114 |
| 361 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;117 |
| 362 | `iter` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;120 |
| 363 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 123&#8209;125 |
| 364 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 130&#8209;131 |
| 365 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 141&#8209;144 |
| 366 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 153&#8209;155 |
| 367 | `next` |  | Y |  |  | Y |  |  | hole | 365&#8209;381 |

### Chap37/BSTSetRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 368 | `empty` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 369 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 370 | `size` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 371 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;70 |
| 372 | `find` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;73 |
| 373 | `contains` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;76 |
| 374 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 375 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 376 | `insert` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;85 |
| 377 | `delete` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 378 | `union` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;91 |
| 379 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 380 | `difference` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 381 | `split` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 382 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 383 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 384 | `filter` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 385 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;112 |
| 386 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 387 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 388 | `iter` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 389 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 124&#8209;126 |
| 390 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 131&#8209;133 |
| 391 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 140&#8209;141 |
| 392 | `next` |  | Y |  |  | Y |  |  | hole | 386&#8209;402 |

### Chap37/BSTSetSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 393 | `empty` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 394 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 395 | `size` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 396 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;86 |
| 397 | `find` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;89 |
| 398 | `contains` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 399 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 400 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;98 |
| 401 | `insert` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |
| 402 | `delete` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;104 |
| 403 | `union` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 404 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 405 | `difference` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;113 |
| 406 | `split` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 407 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 408 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 409 | `filter` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 410 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 411 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 412 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 413 | `iter` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;137 |
| 414 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 142&#8209;144 |
| 415 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 150&#8209;151 |
| 416 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 161&#8209;164 |
| 417 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 173&#8209;175 |
| 418 | `next` |  | Y |  |  | Y |  |  | hole | 384&#8209;400 |

### Chap37/BSTSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 419 | `new_node` |  |  |  | Y | Y |  |  | unknown | 109&#8209;115 |
| 420 | `size_link` |  |  |  | Y | Y |  |  | unknown | 125&#8209;128 |
| 421 | `update` |  |  |  | Y | Y |  |  | unknown | 136&#8209;141 |
| 422 | `splay` |  |  |  | Y | Y |  |  | unknown | 153&#8209;158 |
| 423 | `bst_insert` |  |  |  | Y | Y |  |  | unknown | 1130&#8209;1137 |
| 424 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 1263&#8209;1269 |
| 425 | `find_link` |  |  |  | Y | Y |  |  | unknown | 1281&#8209;1286 |
| 426 | `min_link` |  |  |  | Y | Y |  |  | unknown | 1318&#8209;1324 |
| 427 | `max_link` |  |  |  | Y | Y |  |  | unknown | 1366&#8209;1372 |
| 428 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 1414&#8209;1417 |
| 429 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 1426&#8209;1429 |
| 430 | `in_order_parallel` |  |  |  | Y | Y |  |  | unknown | 1438&#8209;1441 |
| 431 | `pre_order_parallel` |  |  |  | Y | Y |  |  | unknown | 1459&#8209;1462 |
| 432 | `build_balanced` |  |  |  | Y | Y |  |  | unknown | 1481&#8209;1483 |
| 433 | `filter_parallel` |  |  |  | Y | Y |  |  | unknown | 1508&#8209;1513 |
| 434 | `reduce_parallel` |  |  |  | Y | Y |  |  | unknown | 1537&#8209;1542 |
| 435 | `height_rec` |  |  |  | Y | Y |  |  | unknown | 1563&#8209;1566 |
| 436 | `compute_link_spec_size` |  |  |  | Y | Y |  |  | unknown | 1575&#8209;1578 |
| 437 | `new` | Y | Y |  |  | Y |  |  | unknown | 1628&#8209;1630 |
| 438 | `from_sorted_slice` | Y | Y |  |  | Y |  |  | unknown | 1632&#8209;1633 |
| 439 | `insert` | Y | Y |  |  | Y |  |  | hole | 1635&#8209;1641 |
| 440 | `contains` | Y | Y |  |  | Y |  |  | hole | 1643&#8209;1645 |
| 441 | `size` | Y | Y |  |  | Y |  |  | hole | 1647&#8209;1649 |
| 442 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 1651&#8209;1653 |
| 443 | `height` | Y | Y |  |  | Y |  |  | hole | 1655&#8209;1657 |
| 444 | `find` | Y | Y |  |  | Y |  |  | unknown | 1659&#8209;1660 |
| 445 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 1661&#8209;1662 |
| 446 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 1663&#8209;1664 |
| 447 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 1665&#8209;1666 |
| 448 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 1667&#8209;1668 |
| 449 | `filter` | Y | Y |  |  | Y |  |  | unknown | 1669&#8209;1672 |
| 450 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 1673&#8209;1676 |
| 451 | `default` |  | Y |  |  | Y |  | Y |  | 1813 |

### Chap37/BSTSplayStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 452 | `lemma_bst_deep_link` |  |  |  | Y | Y |  |  | unknown | 132&#8209;164 |
| 453 | `new` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;199 |
| 454 | `size` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;202 |
| 455 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;205 |
| 456 | `height` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;210 |
| 457 | `insert` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;216 |
| 458 | `find` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;221 |
| 459 | `contains` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;224 |
| 460 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;230 |
| 461 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;236 |
| 462 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;239 |
| 463 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;242 |
| 464 | `new_node` |  |  |  | Y | Y |  |  | unknown | 251&#8209;257 |
| 465 | `size_link` |  |  |  | Y | Y |  |  | unknown | 270&#8209;271 |
| 466 | `height_link` |  |  |  | Y | Y |  |  | unknown | 282&#8209;285 |
| 467 | `update` |  |  |  | Y | Y |  |  | unknown | 302&#8209;306 |
| 468 | `splay` |  |  |  | Y | Y |  |  | unknown | 320&#8209;325 |
| 469 | `bst_insert` |  |  |  | Y | Y |  |  | unknown | 1299&#8209;1306 |
| 470 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 1442&#8209;1448 |
| 471 | `find_link` |  |  |  | Y | Y |  |  | unknown | 1462&#8209;1467 |
| 472 | `min_link` |  |  |  | Y | Y |  |  | unknown | 1501&#8209;1507 |
| 473 | `max_link` |  |  |  | Y | Y |  |  | unknown | 1553&#8209;1559 |
| 474 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 1606&#8209;1609 |
| 475 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 1621&#8209;1624 |
| 476 | `default` |  | Y |  |  | Y |  | Y |  | 1703 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
