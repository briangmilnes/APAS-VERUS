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
| 1 | Chap37 | AVLTreeSeq | 31 | 34 | 0 | 3 | 37 | 0 | 35 | 1 | 1 |
| 2 | Chap37 | AVLTreeSeqMtPer | 21 | 24 | 0 | 5 | 29 | 0 | 26 | 2 | 1 |
| 3 | Chap37 | AVLTreeSeqStEph | 29 | 32 | 0 | 6 | 38 | 0 | 36 | 1 | 1 |
| 4 | Chap37 | AVLTreeSeqStPer | 22 | 25 | 0 | 8 | 33 | 0 | 31 | 1 | 1 |
| 5 | Chap37 | BSTAVLMtEph | 20 | 20 | 0 | 0 | 20 | 0 | 14 | 6 | 0 |
| 6 | Chap37 | BSTAVLStEph | 15 | 15 | 1 | 0 | 16 | 0 | 16 | 0 | 0 |
| 7 | Chap37 | BSTBBAlphaMtEph | 20 | 20 | 0 | 0 | 20 | 0 | 11 | 9 | 0 |
| 8 | Chap37 | BSTBBAlphaStEph | 17 | 17 | 1 | 0 | 18 | 0 | 18 | 0 | 0 |
| 9 | Chap37 | BSTPlainMtEph | 20 | 20 | 0 | 0 | 20 | 0 | 11 | 9 | 0 |
| 10 | Chap37 | BSTPlainStEph | 17 | 17 | 1 | 0 | 18 | 0 | 18 | 0 | 0 |
| 11 | Chap37 | BSTRBMtEph | 33 | 34 | 0 | 8 | 42 | 0 | 34 | 7 | 1 |
| 12 | Chap37 | BSTRBStEph | 14 | 14 | 1 | 0 | 15 | 0 | 15 | 0 | 0 |
| 13 | Chap37 | BSTSetAVLMtEph | 22 | 22 | 0 | 3 | 25 | 0 | 25 | 0 | 0 |
| 14 | Chap37 | BSTSetBBAlphaMtEph | 22 | 22 | 0 | 3 | 25 | 0 | 25 | 0 | 0 |
| 15 | Chap37 | BSTSetPlainMtEph | 22 | 22 | 0 | 3 | 25 | 0 | 25 | 0 | 0 |
| 16 | Chap37 | BSTSetRBMtEph | 22 | 22 | 0 | 3 | 25 | 0 | 25 | 0 | 0 |
| 17 | Chap37 | BSTSetSplayMtEph | 25 | 25 | 0 | 0 | 25 | 0 | 25 | 0 | 0 |
| 18 | Chap37 | BSTSpecsAndLemmas | 0 | 0 | 0 | 11 | 11 | 0 | 11 | 0 | 0 |
| 19 | Chap37 | BSTSplayMtEph | 33 | 34 | 0 | 3 | 37 | 0 | 28 | 8 | 1 |
| 20 | Chap37 | BSTSplayStEph | 23 | 24 | 1 | 3 | 28 | 0 | 27 | 0 | 1 |

## Function-by-Function Detail

### Chap37/AVLTreeSeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 178&#8209;181 |
| 2 | `lemma_inorder_values_maps_to_inorder` |  |  |  | Y | Y |  |  | unknown | 192&#8209;194 |
| 3 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 205&#8209;208 |
| 4 | `update_size_height` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;249 |
| 5 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;257 |
| 6 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;265 |
| 7 | `rebalance` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;279 |
| 8 | `cached_height_fn` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;296 |
| 9 | `cached_size_fn` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;301 |
| 10 | `insert_at_link` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;314 |
| 11 | `nth_link` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;319 |
| 12 | `set_link` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;331 |
| 13 | `push_inorder` | Y | Y |  |  | Y |  |  | unknown | 333&#8209;338 |
| 14 | `compare_trees` | Y | Y |  |  | Y |  |  | unknown | 340&#8209;346 |
| 15 | `empty` | Y | Y |  |  | Y |  |  | unknown | 354&#8209;355 |
| 16 | `new` | Y | Y |  |  | Y |  |  | unknown | 358&#8209;359 |
| 17 | `length` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;364 |
| 18 | `nth` | Y | Y |  |  | Y |  |  | unknown | 368&#8209;370 |
| 19 | `set` | Y | Y |  |  | Y |  |  | unknown | 373&#8209;378 |
| 20 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 381&#8209;385 |
| 21 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 388&#8209;390 |
| 22 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 393&#8209;395 |
| 23 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 398&#8209;401 |
| 24 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 404&#8209;405 |
| 25 | `update` | Y | Y |  |  | Y |  |  | unknown | 409&#8209;415 |
| 26 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 418&#8209;424 |
| 27 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 427&#8209;432 |
| 28 | `iter` | Y | Y |  |  | Y |  |  | unknown | 434&#8209;440 |
| 29 | `push_back` | Y | Y |  |  | Y |  |  | unknown | 443&#8209;445 |
| 30 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 448&#8209;451 |
| 31 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 454&#8209;456 |
| 32 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 459&#8209;470 |
| 33 | `is_tree_empty` | Y | Y |  |  | Y |  |  | unknown | 473&#8209;475 |
| 34 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 478&#8209;480 |
| 35 | `next` |  | Y |  |  | Y |  |  | hole | 1256&#8209;1272 |
| 36 | `default` |  | Y |  |  | Y |  | Y |  | 1344 |
| 37 | `eq` |  | Y |  |  | Y |  |  | unknown | 1350&#8209;1351 |

### Chap37/AVLTreeSeqMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `lemma_inorder_values_maps_to_inorder` |  |  |  | Y | Y |  |  | unknown | 125&#8209;127 |
| 39 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 179&#8209;182 |
| 40 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 194&#8209;197 |
| 41 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;229 |
| 42 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;237 |
| 43 | `rebalance` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;245 |
| 44 | `height_fn` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;262 |
| 45 | `size_fn` | Y | Y |  |  | Y |  |  | unknown | 264&#8209;267 |
| 46 | `nth_ref` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;272 |
| 47 | `set_rec` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;284 |
| 48 | `inorder_collect` | Y | Y |  |  | Y |  |  | unknown | 286&#8209;291 |
| 49 | `compare_trees` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;299 |
| 50 | `empty` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;308 |
| 51 | `new` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;312 |
| 52 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;316 |
| 53 | `length` | Y | Y |  |  | Y |  |  | unknown | 319&#8209;321 |
| 54 | `nth` | Y | Y |  |  | Y |  |  | unknown | 325&#8209;327 |
| 55 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 330&#8209;332 |
| 56 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;337 |
| 57 | `set` | Y | Y |  |  | Y |  |  | unknown | 340&#8209;347 |
| 58 | `subseq_copy` | Y | Y |  |  | Y |  |  | hole | 350&#8209;352 |
| 59 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 355&#8209;359 |
| 60 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;364 |
| 61 | `iter` | Y | Y |  |  | Y |  |  | unknown | 366&#8209;373 |
| 62 | `mk` |  |  |  | Y | Y |  |  | unknown | 396&#8209;407 |
| 63 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | unknown | 543&#8209;548 |
| 64 | `next` |  | Y |  |  | Y |  |  | hole | 920&#8209;936 |
| 65 | `default` |  | Y |  |  | Y |  | Y |  | 987 |
| 66 | `eq` |  | Y |  |  | Y |  |  | unknown | 1001&#8209;1002 |

### Chap37/AVLTreeSeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 67 | `empty` | Y | Y |  |  | Y |  |  | unknown | 840&#8209;841 |
| 68 | `new` | Y | Y |  |  | Y |  |  | unknown | 844&#8209;845 |
| 69 | `length` | Y | Y |  |  | Y |  |  | unknown | 848&#8209;850 |
| 70 | `nth` | Y | Y |  |  | Y |  |  | unknown | 854&#8209;856 |
| 71 | `set` | Y | Y |  |  | Y |  |  | unknown | 859&#8209;864 |
| 72 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 867&#8209;871 |
| 73 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 874&#8209;876 |
| 74 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 879&#8209;881 |
| 75 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 884&#8209;886 |
| 76 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 889&#8209;890 |
| 77 | `update` | Y | Y |  |  | Y |  |  | unknown | 894&#8209;900 |
| 78 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 903&#8209;910 |
| 79 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 913&#8209;918 |
| 80 | `iter` | Y | Y |  |  | Y |  |  | unknown | 920&#8209;928 |
| 81 | `push_back` | Y | Y |  |  | Y |  |  | unknown | 931&#8209;937 |
| 82 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 940&#8209;943 |
| 83 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 946&#8209;952 |
| 84 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 955&#8209;966 |
| 85 | `push_left_iter` |  |  |  | Y | Y |  |  | unknown | 449&#8209;454 |
| 86 | `lemma_inorder_values_maps_to_inorder` |  |  |  | Y | Y |  |  | unknown | 579&#8209;581 |
| 87 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 642&#8209;645 |
| 88 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 657&#8209;660 |
| 89 | `lemma_size_lt_usize_max` |  |  |  | Y | Y |  |  | unknown | 672&#8209;675 |
| 90 | `lemma_wf_implies_len_bound_steph` |  |  |  | Y | Y |  |  | unknown | 687&#8209;689 |
| 91 | `update_meta` | Y | Y |  |  | Y |  |  | unknown | 716&#8209;729 |
| 92 | `rotate_right_fn` | Y | Y |  |  | Y |  |  | unknown | 731&#8209;739 |
| 93 | `rotate_left_fn` | Y | Y |  |  | Y |  |  | unknown | 741&#8209;749 |
| 94 | `rebalance_fn` | Y | Y |  |  | Y |  |  | unknown | 751&#8209;763 |
| 95 | `h_fn` | Y | Y |  |  | Y |  |  | unknown | 777&#8209;780 |
| 96 | `size_link_fn` | Y | Y |  |  | Y |  |  | unknown | 782&#8209;785 |
| 97 | `insert_at_link` | Y | Y |  |  | Y |  |  | unknown | 787&#8209;798 |
| 98 | `nth_link` | Y | Y |  |  | Y |  |  | unknown | 800&#8209;803 |
| 99 | `set_link` | Y | Y |  |  | Y |  |  | unknown | 805&#8209;815 |
| 100 | `compare_trees` | Y | Y |  |  | Y |  |  | unknown | 817&#8209;823 |
| 101 | `clone_link` | Y | Y |  |  | Y |  |  | unknown | 826&#8209;832 |
| 102 | `next` |  | Y |  |  | Y |  |  | hole | 1327&#8209;1344 |
| 103 | `default` |  | Y |  |  | Y |  | Y |  | 1392 |
| 104 | `eq` |  | Y |  |  | Y |  |  | unknown | 1398&#8209;1399 |

### Chap37/AVLTreeSeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 105 | `lemma_inorder_values_maps_to_inorder` |  |  |  | Y | Y |  |  | unknown | 126&#8209;128 |
| 106 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 183&#8209;186 |
| 107 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 198&#8209;201 |
| 108 | `lemma_size_lt_usize_max` |  |  |  | Y | Y |  |  | unknown | 214&#8209;217 |
| 109 | `lemma_wf_implies_len_bound_stper` |  |  |  | Y | Y |  |  | unknown | 232&#8209;234 |
| 110 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;265 |
| 111 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;273 |
| 112 | `rebalance` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;281 |
| 113 | `height_fn` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;298 |
| 114 | `size_fn` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;303 |
| 115 | `nth_ref` | Y | Y |  |  | Y |  |  | unknown | 305&#8209;308 |
| 116 | `set_rec` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;320 |
| 117 | `inorder_collect` | Y | Y |  |  | Y |  |  | unknown | 322&#8209;325 |
| 118 | `compare_trees` | Y | Y |  |  | Y |  |  | unknown | 327&#8209;333 |
| 119 | `empty` | Y | Y |  |  | Y |  |  | unknown | 341&#8209;342 |
| 120 | `new` | Y | Y |  |  | Y |  |  | unknown | 345&#8209;346 |
| 121 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 349&#8209;350 |
| 122 | `length` | Y | Y |  |  | Y |  |  | unknown | 353&#8209;355 |
| 123 | `nth` | Y | Y |  |  | Y |  |  | unknown | 359&#8209;361 |
| 124 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 364&#8209;366 |
| 125 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 369&#8209;371 |
| 126 | `set` | Y | Y |  |  | Y |  |  | unknown | 374&#8209;382 |
| 127 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 385&#8209;387 |
| 128 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 390&#8209;394 |
| 129 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 397&#8209;399 |
| 130 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 402&#8209;405 |
| 131 | `iter` | Y | Y |  |  | Y |  |  | unknown | 407&#8209;415 |
| 132 | `mk` |  |  |  | Y | Y |  |  | unknown | 438&#8209;449 |
| 133 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | unknown | 609&#8209;614 |
| 134 | `push_left_iter_stper` |  |  |  | Y | Y |  |  | unknown | 963&#8209;967 |
| 135 | `next` |  | Y |  |  | Y |  |  | hole | 1046&#8209;1063 |
| 136 | `default` |  | Y |  |  | Y |  | Y |  | 1107 |
| 137 | `eq` |  | Y |  |  | Y |  |  | unknown | 1120&#8209;1121 |

### Chap37/BSTAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 138 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;100 |
| 139 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;133 |
| 140 | `rebalance` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;141 |
| 141 | `insert_node` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;172 |
| 142 | `contains_node` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;177 |
| 143 | `find_node` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;184 |
| 144 | `min_node` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;192 |
| 145 | `max_node` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;200 |
| 146 | `new` | Y | Y |  |  | Y |  |  | unknown | 738&#8209;743 |
| 147 | `insert` | Y | Y |  |  | Y |  |  | hole | 746&#8209;755 |
| 148 | `contains` | Y | Y |  |  | Y |  |  | hole | 758&#8209;760 |
| 149 | `size` | Y | Y |  |  | Y |  |  | hole | 763&#8209;765 |
| 150 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 768&#8209;770 |
| 151 | `height` | Y | Y |  |  | Y |  |  | hole | 773&#8209;775 |
| 152 | `find` | Y | Y |  |  | Y |  |  | hole | 779&#8209;783 |
| 153 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 785&#8209;786 |
| 154 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 788&#8209;789 |
| 155 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 791&#8209;793 |
| 156 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 795&#8209;797 |
| 157 | `iter` | Y | Y |  |  | Y |  |  | unknown | 799&#8209;803 |

### Chap37/BSTAVLStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 158 | `new` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 159 | `size` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;101 |
| 160 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;105 |
| 161 | `height` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;111 |
| 162 | `insert` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;123 |
| 163 | `contains` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;129 |
| 164 | `find` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;138 |
| 165 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;182 |
| 166 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;216 |
| 167 | `rebalance` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;224 |
| 168 | `insert_node` | Y | Y |  |  | Y |  |  | unknown | 242&#8209;253 |
| 169 | `contains_node` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;258 |
| 170 | `find_node` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;265 |
| 171 | `min_node` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;272 |
| 172 | `max_node` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;279 |
| 173 | `iter` |  |  | Y |  | Y |  |  | unknown | 973&#8209;981 |

### Chap37/BSTBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 174 | `insert_node` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;71 |
| 175 | `contains_node` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;76 |
| 176 | `find_node` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;83 |
| 177 | `min_node` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;91 |
| 178 | `max_node` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;99 |
| 179 | `delete_min_node` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;115 |
| 180 | `delete_node` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;127 |
| 181 | `new` | Y | Y |  |  | Y |  |  | unknown | 514&#8209;518 |
| 182 | `insert` | Y | Y |  |  | Y |  |  | hole | 521&#8209;529 |
| 183 | `delete` | Y | Y |  |  | Y |  |  | hole | 532&#8209;540 |
| 184 | `contains` | Y | Y |  |  | Y |  |  | hole | 543&#8209;545 |
| 185 | `size` | Y | Y |  |  | Y |  |  | hole | 548&#8209;550 |
| 186 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 553&#8209;555 |
| 187 | `height` | Y | Y |  |  | Y |  |  | hole | 558&#8209;560 |
| 188 | `find` | Y | Y |  |  | Y |  |  | hole | 564&#8209;568 |
| 189 | `minimum` | Y | Y |  |  | Y |  |  | hole | 570&#8209;575 |
| 190 | `maximum` | Y | Y |  |  | Y |  |  | hole | 577&#8209;582 |
| 191 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 584&#8209;586 |
| 192 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 588&#8209;590 |
| 193 | `iter` | Y | Y |  |  | Y |  |  | unknown | 592&#8209;596 |

### Chap37/BSTBBAlphaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 194 | `new` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |
| 195 | `size` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;106 |
| 196 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 197 | `height` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;116 |
| 198 | `insert` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;127 |
| 199 | `contains` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;133 |
| 200 | `find` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;142 |
| 201 | `delete` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;153 |
| 202 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;162 |
| 203 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;171 |
| 204 | `insert_node` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;185 |
| 205 | `contains_node` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;191 |
| 206 | `find_node` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;199 |
| 207 | `min_node` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;207 |
| 208 | `max_node` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;215 |
| 209 | `delete_min_node` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;229 |
| 210 | `delete_node` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;240 |
| 211 | `iter` |  |  | Y |  | Y |  |  | unknown | 709&#8209;717 |

### Chap37/BSTPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 212 | `insert_node` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;68 |
| 213 | `contains_node` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 214 | `find_node` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;80 |
| 215 | `min_node` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;88 |
| 216 | `max_node` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;96 |
| 217 | `delete_min_node` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;112 |
| 218 | `delete_node` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;124 |
| 219 | `new` | Y | Y |  |  | Y |  |  | unknown | 494&#8209;498 |
| 220 | `insert` | Y | Y |  |  | Y |  |  | hole | 501&#8209;509 |
| 221 | `delete` | Y | Y |  |  | Y |  |  | hole | 512&#8209;520 |
| 222 | `contains` | Y | Y |  |  | Y |  |  | hole | 523&#8209;525 |
| 223 | `size` | Y | Y |  |  | Y |  |  | hole | 528&#8209;530 |
| 224 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 533&#8209;535 |
| 225 | `height` | Y | Y |  |  | Y |  |  | hole | 538&#8209;540 |
| 226 | `find` | Y | Y |  |  | Y |  |  | hole | 544&#8209;548 |
| 227 | `minimum` | Y | Y |  |  | Y |  |  | hole | 550&#8209;555 |
| 228 | `maximum` | Y | Y |  |  | Y |  |  | hole | 557&#8209;562 |
| 229 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 564&#8209;566 |
| 230 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 568&#8209;570 |
| 231 | `iter` | Y | Y |  |  | Y |  |  | unknown | 572&#8209;576 |

### Chap37/BSTPlainStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 232 | `insert_node` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;80 |
| 233 | `contains_node` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;86 |
| 234 | `find_node` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;94 |
| 235 | `min_node` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;102 |
| 236 | `max_node` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;110 |
| 237 | `delete_min_node` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;124 |
| 238 | `delete_node` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;135 |
| 239 | `new` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;147 |
| 240 | `size` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;153 |
| 241 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;157 |
| 242 | `height` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;163 |
| 243 | `insert` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;174 |
| 244 | `contains` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;180 |
| 245 | `find` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;189 |
| 246 | `delete` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;200 |
| 247 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;209 |
| 248 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;218 |
| 249 | `iter` |  |  | Y |  | Y |  |  | unknown | 721&#8209;729 |

### Chap37/BSTRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 250 | `lemma_link_to_bbt_size` |  |  |  | Y | Y |  |  | unknown | 158&#8209;160 |
| 251 | `lemma_link_to_bbt_contains` |  |  |  | Y | Y |  |  | unknown | 172&#8209;174 |
| 252 | `lemma_link_to_bbt_height` |  |  |  | Y | Y |  |  | unknown | 186&#8209;188 |
| 253 | `lemma_link_to_bbt_is_bst` |  |  |  | Y | Y |  |  | unknown | 200&#8209;203 |
| 254 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 228&#8209;230 |
| 255 | `is_red` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;255 |
| 256 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;259 |
| 257 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;266 |
| 258 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;273 |
| 259 | `flip_colors` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;280 |
| 260 | `fix_up` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;287 |
| 261 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;296 |
| 262 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;302 |
| 263 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 304&#8209;309 |
| 264 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;316 |
| 265 | `in_order_collect` | Y | Y |  |  | Y |  |  | unknown | 318&#8209;320 |
| 266 | `pre_order_collect` | Y | Y |  |  | Y |  |  | unknown | 322&#8209;324 |
| 267 | `in_order_parallel` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;328 |
| 268 | `pre_order_parallel` | Y | Y |  |  | Y |  |  | unknown | 330&#8209;332 |
| 269 | `filter_parallel` | Y | Y |  |  | Y |  |  | unknown | 334&#8209;340 |
| 270 | `reduce_parallel` | Y | Y |  |  | Y |  |  | unknown | 342&#8209;348 |
| 271 | `height_rec` | Y | Y |  |  | Y |  |  | unknown | 350&#8209;352 |
| 272 | `compute_link_spec_size` | Y | Y |  |  | Y |  |  | unknown | 355&#8209;357 |
| 273 | `new_node` |  |  |  | Y | Y |  |  | unknown | 366&#8209;372 |
| 274 | `update` |  |  |  | Y | Y |  |  | unknown | 386&#8209;391 |
| 275 | `build_balanced` |  |  |  | Y | Y |  |  | unknown | 1122&#8209;1124 |
| 276 | `new` | Y | Y |  |  | Y |  |  | unknown | 1177&#8209;1181 |
| 277 | `from_sorted_slice` | Y | Y |  |  | Y |  |  | hole | 1184&#8209;1185 |
| 278 | `insert` | Y | Y |  |  | Y |  |  | hole | 1188&#8209;1197 |
| 279 | `contains` | Y | Y |  |  | Y |  |  | hole | 1200&#8209;1202 |
| 280 | `size` | Y | Y |  |  | Y |  |  | hole | 1205&#8209;1207 |
| 281 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 1210&#8209;1212 |
| 282 | `height` | Y | Y |  |  | Y |  |  | hole | 1215&#8209;1217 |
| 283 | `find` | Y | Y |  |  | Y |  |  | hole | 1221&#8209;1225 |
| 284 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 1227&#8209;1228 |
| 285 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 1230&#8209;1231 |
| 286 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 1233&#8209;1234 |
| 287 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 1236&#8209;1237 |
| 288 | `filter` | Y | Y |  |  | Y |  |  | unknown | 1239&#8209;1246 |
| 289 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 1248&#8209;1254 |
| 290 | `iter` | Y | Y |  |  | Y |  |  | unknown | 1256&#8209;1260 |
| 291 | `default` |  | Y |  |  | Y |  | Y |  | 1558 |

### Chap37/BSTRBStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 292 | `new` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;78 |
| 293 | `size` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;84 |
| 294 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 295 | `height` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 296 | `insert` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;105 |
| 297 | `contains` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;111 |
| 298 | `find` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;120 |
| 299 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;132 |
| 300 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;141 |
| 301 | `insert_node` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;150 |
| 302 | `contains_node` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;155 |
| 303 | `find_node` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;162 |
| 304 | `min_node` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;169 |
| 305 | `max_node` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;176 |
| 306 | `iter` |  |  | Y |  | Y |  |  | unknown | 620&#8209;628 |

### Chap37/BSTSetAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 307 | `empty` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;62 |
| 308 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;66 |
| 309 | `size` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;70 |
| 310 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;74 |
| 311 | `find` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 312 | `contains` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 313 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 314 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;91 |
| 315 | `insert` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 316 | `delete` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 317 | `union` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 318 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 319 | `difference` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;111 |
| 320 | `split` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 321 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 322 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;123 |
| 323 | `filter` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;129 |
| 324 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;135 |
| 325 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 326 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;143 |
| 327 | `iter` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;148 |
| 328 | `copy_set` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;152 |
| 329 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 159&#8209;161 |
| 330 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 181&#8209;182 |
| 331 | `build_from_vec` |  |  |  | Y | Y |  |  | unknown | 201&#8209;203 |

### Chap37/BSTSetBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 332 | `empty` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;62 |
| 333 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;66 |
| 334 | `size` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;70 |
| 335 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;74 |
| 336 | `find` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 337 | `contains` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 338 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 339 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;91 |
| 340 | `insert` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 341 | `delete` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 342 | `union` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 343 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 344 | `difference` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;111 |
| 345 | `split` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 346 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 347 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;123 |
| 348 | `filter` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;129 |
| 349 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;135 |
| 350 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 351 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;143 |
| 352 | `iter` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;148 |
| 353 | `copy_set` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;152 |
| 354 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 159&#8209;161 |
| 355 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 180&#8209;181 |
| 356 | `from_vec` |  |  |  | Y | Y |  |  | unknown | 198&#8209;200 |

### Chap37/BSTSetPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 357 | `empty` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;62 |
| 358 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;66 |
| 359 | `size` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;70 |
| 360 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;74 |
| 361 | `find` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 362 | `contains` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 363 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 364 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;91 |
| 365 | `insert` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 366 | `delete` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 367 | `union` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 368 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 369 | `difference` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;111 |
| 370 | `split` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 371 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 372 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;123 |
| 373 | `filter` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;129 |
| 374 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;135 |
| 375 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 376 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;143 |
| 377 | `iter` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;148 |
| 378 | `copy_set` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;152 |
| 379 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 159&#8209;161 |
| 380 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 180&#8209;181 |
| 381 | `from_vec` |  |  |  | Y | Y |  |  | unknown | 198&#8209;200 |

### Chap37/BSTSetRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 382 | `empty` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;61 |
| 383 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;65 |
| 384 | `size` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;69 |
| 385 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;73 |
| 386 | `find` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;78 |
| 387 | `contains` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 388 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;86 |
| 389 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;90 |
| 390 | `insert` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 391 | `delete` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;98 |
| 392 | `union` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;102 |
| 393 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 394 | `difference` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 395 | `split` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;114 |
| 396 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 397 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 398 | `filter` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;128 |
| 399 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;134 |
| 400 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |
| 401 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;142 |
| 402 | `iter` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;147 |
| 403 | `copy_set` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;151 |
| 404 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 158&#8209;160 |
| 405 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 180&#8209;181 |
| 406 | `build_from_vec` |  |  |  | Y | Y |  |  | unknown | 200&#8209;202 |

### Chap37/BSTSetSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 407 | `empty` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;62 |
| 408 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;66 |
| 409 | `size` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;70 |
| 410 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;74 |
| 411 | `find` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 412 | `contains` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 413 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 414 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;91 |
| 415 | `insert` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 416 | `delete` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 417 | `union` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 418 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 419 | `difference` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;111 |
| 420 | `split` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 421 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 422 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;123 |
| 423 | `filter` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;129 |
| 424 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;135 |
| 425 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 426 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;143 |
| 427 | `iter` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;148 |
| 428 | `values_vec` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;156 |
| 429 | `rebuild_from_vec` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;160 |
| 430 | `build_from_vec` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;164 |
| 431 | `copy_set` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;168 |

### Chap37/BSTSpecsAndLemmas.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 432 | `lemma_node_contains` |  |  |  | Y | Y |  |  | unknown | 30&#8209;36 |
| 433 | `lemma_bst_left` |  |  |  | Y | Y |  |  | unknown | 40&#8209;49 |
| 434 | `lemma_bst_right` |  |  |  | Y | Y |  |  | unknown | 53&#8209;62 |
| 435 | `lemma_modified_left_preserves_bst` |  |  |  | Y | Y |  |  | unknown | 67&#8209;79 |
| 436 | `lemma_modified_right_preserves_bst` |  |  |  | Y | Y |  |  | unknown | 83&#8209;95 |
| 437 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 100&#8209;132 |
| 438 | `lemma_max_plus_one` |  |  |  | Y | Y |  |  | unknown | 156&#8209;158 |
| 439 | `lemma_bst_insert_left` |  |  |  | Y | Y |  |  | unknown | 164&#8209;190 |
| 440 | `lemma_bst_insert_right` |  |  |  | Y | Y |  |  | unknown | 220&#8209;246 |
| 441 | `lemma_bst_delete_left` |  |  |  | Y | Y |  |  | unknown | 276&#8209;303 |
| 442 | `lemma_bst_delete_right` |  |  |  | Y | Y |  |  | unknown | 333&#8209;360 |

### Chap37/BSTSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 443 | `lemma_height_le_node_count` |  |  |  | Y | Y |  |  | unknown | 171&#8209;173 |
| 444 | `lemma_zig_child_ordering` |  |  |  | Y | Y |  |  | unknown | 186&#8209;199 |
| 445 | `lemma_zag_child_ordering` |  |  |  | Y | Y |  |  | unknown | 216&#8209;229 |
| 446 | `new_node` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;256 |
| 447 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;261 |
| 448 | `update` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;272 |
| 449 | `splay` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;279 |
| 450 | `bst_insert` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;288 |
| 451 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;297 |
| 452 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;304 |
| 453 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;312 |
| 454 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;320 |
| 455 | `in_order_collect` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;325 |
| 456 | `pre_order_collect` | Y | Y |  |  | Y |  |  | unknown | 328&#8209;330 |
| 457 | `in_order_parallel` | Y | Y |  |  | Y |  |  | unknown | 333&#8209;335 |
| 458 | `pre_order_parallel` | Y | Y |  |  | Y |  |  | unknown | 338&#8209;340 |
| 459 | `clone_link` | Y | Y |  |  | Y |  |  | unknown | 344&#8209;345 |
| 460 | `build_balanced` | Y | Y |  |  | Y |  |  | unknown | 348&#8209;359 |
| 461 | `filter_parallel` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;368 |
| 462 | `reduce_parallel` | Y | Y |  |  | Y |  |  | unknown | 371&#8209;377 |
| 463 | `height_rec` | Y | Y |  |  | Y |  |  | unknown | 380&#8209;382 |
| 464 | `compute_link_spec_size` | Y | Y |  |  | Y |  |  | unknown | 385&#8209;387 |
| 465 | `new` | Y | Y |  |  | Y |  |  | unknown | 1775&#8209;1779 |
| 466 | `from_sorted_slice` | Y | Y |  |  | Y |  |  | unknown | 1782&#8209;1788 |
| 467 | `insert` | Y | Y |  |  | Y |  |  | hole | 1791&#8209;1802 |
| 468 | `contains` | Y | Y |  |  | Y |  |  | hole | 1805&#8209;1807 |
| 469 | `size` | Y | Y |  |  | Y |  |  | hole | 1810&#8209;1812 |
| 470 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 1815&#8209;1817 |
| 471 | `height` | Y | Y |  |  | Y |  |  | hole | 1820&#8209;1822 |
| 472 | `find` | Y | Y |  |  | Y |  |  | hole | 1826&#8209;1830 |
| 473 | `minimum` | Y | Y |  |  | Y |  |  | hole | 1832&#8209;1838 |
| 474 | `maximum` | Y | Y |  |  | Y |  |  | hole | 1840&#8209;1846 |
| 475 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 1848&#8209;1850 |
| 476 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 1852&#8209;1854 |
| 477 | `filter` | Y | Y |  |  | Y |  |  | unknown | 1856&#8209;1862 |
| 478 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 1864&#8209;1870 |
| 479 | `default` |  | Y |  |  | Y |  | Y |  | 2110 |

### Chap37/BSTSplayStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 480 | `new_node` | Y | Y |  |  | Y |  |  | unknown | 1029&#8209;1034 |
| 481 | `update` | Y | Y |  |  | Y |  |  | unknown | 1039&#8209;1043 |
| 482 | `splay` | Y | Y |  |  | Y |  |  | unknown | 1045&#8209;1049 |
| 483 | `lemma_bst_deep_link` |  |  |  | Y | Y |  |  | unknown | 906&#8209;938 |
| 484 | `lemma_zig_child_ordering` |  |  |  | Y | Y |  |  | unknown | 960&#8209;973 |
| 485 | `lemma_zag_child_ordering` |  |  |  | Y | Y |  |  | unknown | 990&#8209;1003 |
| 486 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 1063&#8209;1064 |
| 487 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 1065&#8209;1067 |
| 488 | `bst_insert` | Y | Y |  |  | Y |  |  | unknown | 1068&#8209;1074 |
| 489 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 1075&#8209;1081 |
| 490 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 1082&#8209;1086 |
| 491 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 1087&#8209;1092 |
| 492 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 1093&#8209;1098 |
| 493 | `in_order_collect` | Y | Y |  |  | Y |  |  | unknown | 1100&#8209;1102 |
| 494 | `pre_order_collect` | Y | Y |  |  | Y |  |  | unknown | 1104&#8209;1106 |
| 495 | `new` | Y | Y |  |  | Y |  |  | unknown | 1119&#8209;1125 |
| 496 | `size` | Y | Y |  |  | Y |  |  | unknown | 1127&#8209;1129 |
| 497 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 1131&#8209;1133 |
| 498 | `height` | Y | Y |  |  | Y |  |  | unknown | 1135&#8209;1139 |
| 499 | `insert` | Y | Y |  |  | Y |  |  | unknown | 1141&#8209;1146 |
| 500 | `find` | Y | Y |  |  | Y |  |  | unknown | 1149&#8209;1153 |
| 501 | `contains` | Y | Y |  |  | Y |  |  | unknown | 1155&#8209;1157 |
| 502 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 1159&#8209;1164 |
| 503 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 1166&#8209;1171 |
| 504 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 1173&#8209;1175 |
| 505 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 1177&#8209;1179 |
| 506 | `iter` |  |  | Y |  | Y |  |  | unknown | 1634&#8209;1639 |
| 507 | `default` |  | Y |  |  | Y |  | Y |  | 1675 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
