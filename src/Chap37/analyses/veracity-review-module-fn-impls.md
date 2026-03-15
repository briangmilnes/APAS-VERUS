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
| 2 | Chap37 | AVLTreeSeqMtPer | 11 | 14 | 0 | 14 | 28 | 0 | 23 | 3 | 2 |
| 3 | Chap37 | AVLTreeSeqStEph | 18 | 21 | 0 | 14 | 35 | 0 | 32 | 2 | 1 |
| 4 | Chap37 | AVLTreeSeqStPer | 13 | 16 | 0 | 14 | 30 | 0 | 28 | 1 | 1 |
| 5 | Chap37 | BSTAVLMtEph | 11 | 11 | 0 | 8 | 19 | 0 | 19 | 0 | 0 |
| 6 | Chap37 | BSTAVLStEph | 7 | 7 | 0 | 10 | 17 | 0 | 17 | 0 | 0 |
| 7 | Chap37 | BSTBBAlphaMtEph | 11 | 11 | 0 | 5 | 16 | 0 | 16 | 0 | 0 |
| 8 | Chap37 | BSTBBAlphaStEph | 10 | 10 | 0 | 7 | 17 | 0 | 17 | 0 | 0 |
| 9 | Chap37 | BSTPlainMtEph | 11 | 11 | 0 | 5 | 16 | 0 | 16 | 0 | 0 |
| 10 | Chap37 | BSTPlainStEph | 10 | 10 | 0 | 10 | 20 | 0 | 20 | 0 | 0 |
| 11 | Chap37 | BSTRBMtEph | 14 | 15 | 0 | 21 | 36 | 0 | 35 | 0 | 1 |
| 12 | Chap37 | BSTRBStEph | 7 | 7 | 0 | 8 | 15 | 0 | 15 | 0 | 0 |
| 13 | Chap37 | BSTSetAVLMtEph | 21 | 22 | 0 | 4 | 26 | 0 | 26 | 0 | 0 |
| 14 | Chap37 | BSTSetBBAlphaMtEph | 21 | 22 | 0 | 4 | 26 | 0 | 26 | 0 | 0 |
| 15 | Chap37 | BSTSetPlainMtEph | 21 | 22 | 0 | 4 | 26 | 0 | 26 | 0 | 0 |
| 16 | Chap37 | BSTSetRBMtEph | 21 | 22 | 0 | 3 | 25 | 0 | 25 | 0 | 0 |
| 17 | Chap37 | BSTSetSplayMtEph | 21 | 22 | 0 | 4 | 26 | 0 | 26 | 0 | 0 |
| 18 | Chap37 | BSTSplayMtEph | 14 | 15 | 0 | 18 | 33 | 0 | 32 | 0 | 1 |
| 19 | Chap37 | BSTSplayStEph | 11 | 12 | 0 | 12 | 24 | 0 | 23 | 0 | 1 |

## Function-by-Function Detail

### Chap37/AVLTreeSeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 184&#8209;187 |
| 2 | `lemma_inorder_values_maps_to_inorder` |  |  |  | Y | Y |  |  | unknown | 198&#8209;200 |
| 3 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 211&#8209;214 |
| 4 | `empty` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;233 |
| 5 | `new` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;236 |
| 6 | `length` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;240 |
| 7 | `nth` | Y | Y |  |  | Y |  |  | unknown | 242&#8209;244 |
| 8 | `set` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;251 |
| 9 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;257 |
| 10 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;261 |
| 11 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;265 |
| 12 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;270 |
| 13 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;273 |
| 14 | `update` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;281 |
| 15 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;289 |
| 16 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;296 |
| 17 | `iter` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;303 |
| 18 | `push_back` | Y | Y |  |  | Y |  |  | unknown | 305&#8209;307 |
| 19 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;312 |
| 20 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 314&#8209;316 |
| 21 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 318&#8209;329 |
| 22 | `is_tree_empty` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;333 |
| 23 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;337 |
| 24 | `cached_height` |  |  |  | Y | Y |  |  | unknown | 345&#8209;347 |
| 25 | `cached_size` |  |  |  | Y | Y |  |  | unknown | 355&#8209;357 |
| 26 | `update_size_height` |  |  |  | Y | Y |  |  | unknown | 367&#8209;385 |
| 27 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 401&#8209;408 |
| 28 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 450&#8209;457 |
| 29 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 499&#8209;510 |
| 30 | `insert_at_link` |  |  |  | Y | Y |  |  | unknown | 554&#8209;565 |
| 31 | `nth_link` |  |  |  | Y | Y |  |  | unknown | 630&#8209;633 |
| 32 | `set_link` |  |  |  | Y | Y |  |  | unknown | 648&#8209;658 |
| 33 | `push_inorder` |  |  |  | Y | Y |  |  | unknown | 675&#8209;680 |
| 34 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 710&#8209;715 |
| 35 | `next` |  | Y |  |  | Y |  |  | hole | 1118&#8209;1134 |
| 36 | `default` |  | Y |  |  | Y |  | Y |  | 1189 |
| 37 | `eq` |  | Y |  |  | Y |  |  | hole | 1222&#8209;1223 |

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
| 59 | `inorder_collect` |  |  |  | Y | Y |  |  | unknown | 497&#8209;499 |
| 60 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | hole | 509&#8209;512 |
| 61 | `rec` |  |  |  | Y | Y |  | Y |  | 514 |
| 62 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 528&#8209;533 |
| 63 | `default` |  | Y |  |  | Y |  | Y |  | 667 |
| 64 | `next` |  | Y |  |  | Y |  |  | unknown | 674&#8209;675 |
| 65 | `eq` |  | Y |  |  | Y |  |  | hole | 711&#8209;712 |

### Chap37/AVLTreeSeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 66 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 180&#8209;183 |
| 67 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 195&#8209;198 |
| 68 | `empty` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;219 |
| 69 | `new` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;222 |
| 70 | `length` | Y | Y |  |  | Y |  |  | unknown | 224&#8209;226 |
| 71 | `nth` | Y | Y |  |  | Y |  |  | unknown | 228&#8209;230 |
| 72 | `set` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;233 |
| 73 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;236 |
| 74 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;240 |
| 75 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 242&#8209;244 |
| 76 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;248 |
| 77 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;251 |
| 78 | `update` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;256 |
| 79 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;265 |
| 80 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;272 |
| 81 | `iter` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;275 |
| 82 | `push_back` | Y | Y |  |  | Y |  |  | unknown | 277&#8209;283 |
| 83 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;288 |
| 84 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;294 |
| 85 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 296&#8209;307 |
| 86 | `h_fn` |  |  |  | Y | Y |  |  | unknown | 315&#8209;317 |
| 87 | `size_link_fn` |  |  |  | Y | Y |  |  | unknown | 325&#8209;327 |
| 88 | `update_meta` |  |  |  | Y | Y |  |  | unknown | 337&#8209;351 |
| 89 | `rotate_right_fn` |  |  |  | Y | Y |  |  | unknown | 368&#8209;375 |
| 90 | `rotate_left_fn` |  |  |  | Y | Y |  |  | unknown | 406&#8209;413 |
| 91 | `rebalance_fn` |  |  |  | Y | Y |  |  | unknown | 444&#8209;453 |
| 92 | `insert_at_link` |  |  |  | Y | Y |  |  | unknown | 502&#8209;513 |
| 93 | `nth_link` |  |  |  | Y | Y |  |  | unknown | 566&#8209;569 |
| 94 | `set_link` |  |  |  | Y | Y |  |  | unknown | 584&#8209;593 |
| 95 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 619&#8209;624 |
| 96 | `clone_link` |  |  |  | Y | Y |  |  | hole | 661&#8209;667 |
| 97 | `default` |  | Y |  |  | Y |  | Y |  | 1020 |
| 98 | `push_left_iter` |  |  |  | Y | Y |  |  | unknown | 1025&#8209;1027 |
| 99 | `next` |  | Y |  |  | Y |  |  | unknown | 1047&#8209;1048 |
| 100 | `eq` |  | Y |  |  | Y |  |  | hole | 1087&#8209;1088 |

### Chap37/AVLTreeSeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 101 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 136&#8209;139 |
| 102 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 151&#8209;154 |
| 103 | `empty` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;172 |
| 104 | `new` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;175 |
| 105 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;178 |
| 106 | `length` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;182 |
| 107 | `nth` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;186 |
| 108 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;190 |
| 109 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;194 |
| 110 | `set` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;201 |
| 111 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;204 |
| 112 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;209 |
| 113 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;212 |
| 114 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;215 |
| 115 | `iter` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;218 |
| 116 | `height_fn` |  |  |  | Y | Y |  |  | unknown | 223&#8209;225 |
| 117 | `size_fn` |  |  |  | Y | Y |  |  | unknown | 233&#8209;235 |
| 118 | `mk` |  |  |  | Y | Y |  |  | unknown | 243&#8209;254 |
| 119 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 263&#8209;268 |
| 120 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 305&#8209;310 |
| 121 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 342&#8209;347 |
| 122 | `nth_ref` |  |  |  | Y | Y |  |  | unknown | 410&#8209;413 |
| 123 | `set_rec` |  |  |  | Y | Y |  |  | unknown | 428&#8209;437 |
| 124 | `inorder_collect` |  |  |  | Y | Y |  |  | unknown | 474&#8209;476 |
| 125 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | unknown | 485&#8209;489 |
| 126 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 529&#8209;534 |
| 127 | `default` |  | Y |  |  | Y |  | Y |  | 680 |
| 128 | `push_left_iter_stper` |  |  |  | Y | Y |  |  | unknown | 693&#8209;695 |
| 129 | `next` |  | Y |  |  | Y |  |  | unknown | 719&#8209;720 |
| 130 | `eq` |  | Y |  |  | Y |  |  | hole | 742&#8209;743 |

### Chap37/BSTAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 131 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 40&#8209;72 |
| 132 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 93&#8209;97 |
| 133 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 159&#8209;163 |
| 134 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 226&#8209;235 |
| 135 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 318&#8209;321 |
| 136 | `find_node` |  |  |  | Y | Y |  |  | unknown | 343&#8209;348 |
| 137 | `min_node` |  |  |  | Y | Y |  |  | unknown | 370&#8209;375 |
| 138 | `max_node` |  |  |  | Y | Y |  |  | unknown | 386&#8209;391 |
| 139 | `new` | Y | Y |  |  | Y |  |  | unknown | 444&#8209;446 |
| 140 | `insert` | Y | Y |  |  | Y |  |  | unknown | 448&#8209;456 |
| 141 | `contains` | Y | Y |  |  | Y |  |  | unknown | 458&#8209;460 |
| 142 | `size` | Y | Y |  |  | Y |  |  | unknown | 462&#8209;464 |
| 143 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 466&#8209;468 |
| 144 | `height` | Y | Y |  |  | Y |  |  | unknown | 470&#8209;472 |
| 145 | `find` | Y | Y |  |  | Y |  |  | unknown | 474&#8209;475 |
| 146 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 476&#8209;477 |
| 147 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 478&#8209;479 |
| 148 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 480&#8209;481 |
| 149 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 482&#8209;483 |

### Chap37/BSTAVLStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 150 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 67&#8209;99 |
| 151 | `lemma_max_plus_one` |  |  |  | Y | Y |  |  | unknown | 117&#8209;119 |
| 152 | `new` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;133 |
| 153 | `size` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 154 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;141 |
| 155 | `height` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;146 |
| 156 | `insert` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;157 |
| 157 | `contains` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;162 |
| 158 | `find` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;169 |
| 159 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 174&#8209;212 |
| 160 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 330&#8209;368 |
| 161 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 491&#8209;519 |
| 162 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 777&#8209;788 |
| 163 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 931&#8209;934 |
| 164 | `find_node` |  |  |  | Y | Y |  |  | unknown | 964&#8209;969 |
| 165 | `min_node` |  |  |  | Y | Y |  |  | unknown | 999&#8209;1005 |
| 166 | `max_node` |  |  |  | Y | Y |  |  | unknown | 1019&#8209;1025 |

### Chap37/BSTBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 167 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 41&#8209;50 |
| 168 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 133&#8209;136 |
| 169 | `find_node` |  |  |  | Y | Y |  |  | unknown | 158&#8209;163 |
| 170 | `min_node` |  |  |  | Y | Y |  |  | unknown | 185&#8209;190 |
| 171 | `max_node` |  |  |  | Y | Y |  |  | unknown | 201&#8209;206 |
| 172 | `new` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;261 |
| 173 | `insert` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;271 |
| 174 | `contains` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;275 |
| 175 | `size` | Y | Y |  |  | Y |  |  | unknown | 277&#8209;279 |
| 176 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;283 |
| 177 | `height` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;287 |
| 178 | `find` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;290 |
| 179 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;292 |
| 180 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;294 |
| 181 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;296 |
| 182 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 297&#8209;298 |

### Chap37/BSTBBAlphaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 183 | `new` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 184 | `size` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 185 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;84 |
| 186 | `height` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 187 | `insert` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;99 |
| 188 | `contains` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;104 |
| 189 | `find` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;111 |
| 190 | `delete` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;121 |
| 191 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;129 |
| 192 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;137 |
| 193 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 187&#8209;194 |
| 194 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 322&#8209;325 |
| 195 | `find_node` |  |  |  | Y | Y |  |  | unknown | 355&#8209;360 |
| 196 | `min_node` |  |  |  | Y | Y |  |  | unknown | 390&#8209;396 |
| 197 | `max_node` |  |  |  | Y | Y |  |  | unknown | 410&#8209;416 |
| 198 | `delete_min_node` |  |  |  | Y | Y |  |  | unknown | 431&#8209;442 |
| 199 | `delete_node` |  |  |  | Y | Y |  |  | unknown | 547&#8209;554 |

### Chap37/BSTPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 200 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 40&#8209;49 |
| 201 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 132&#8209;135 |
| 202 | `find_node` |  |  |  | Y | Y |  |  | unknown | 157&#8209;162 |
| 203 | `min_node` |  |  |  | Y | Y |  |  | unknown | 184&#8209;189 |
| 204 | `max_node` |  |  |  | Y | Y |  |  | unknown | 200&#8209;205 |
| 205 | `new` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;260 |
| 206 | `insert` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;270 |
| 207 | `contains` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;274 |
| 208 | `size` | Y | Y |  |  | Y |  |  | unknown | 276&#8209;278 |
| 209 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;282 |
| 210 | `height` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;286 |
| 211 | `find` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;289 |
| 212 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;291 |
| 213 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;293 |
| 214 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;295 |
| 215 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 296&#8209;297 |

### Chap37/BSTPlainStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 216 | `lemma_node_contains` |  |  |  | Y | Y |  |  | unknown | 40&#8209;46 |
| 217 | `lemma_bst_left` |  |  |  | Y | Y |  |  | unknown | 50&#8209;59 |
| 218 | `lemma_bst_right` |  |  |  | Y | Y |  |  | unknown | 63&#8209;72 |
| 219 | `new` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;91 |
| 220 | `size` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;96 |
| 221 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 222 | `height` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;104 |
| 223 | `insert` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;114 |
| 224 | `contains` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;119 |
| 225 | `find` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;126 |
| 226 | `delete` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;136 |
| 227 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;144 |
| 228 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;152 |
| 229 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 243&#8209;250 |
| 230 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 378&#8209;381 |
| 231 | `find_node` |  |  |  | Y | Y |  |  | unknown | 411&#8209;416 |
| 232 | `min_node` |  |  |  | Y | Y |  |  | unknown | 446&#8209;452 |
| 233 | `max_node` |  |  |  | Y | Y |  |  | unknown | 466&#8209;472 |
| 234 | `delete_min_node` |  |  |  | Y | Y |  |  | unknown | 487&#8209;498 |
| 235 | `delete_node` |  |  |  | Y | Y |  |  | unknown | 605&#8209;612 |

### Chap37/BSTRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 236 | `new_node` |  |  |  | Y | Y |  |  | unknown | 99&#8209;105 |
| 237 | `is_red` |  |  |  | Y | Y |  |  | unknown | 116&#8209;119 |
| 238 | `size_link` |  |  |  | Y | Y |  |  | unknown | 127&#8209;130 |
| 239 | `update` |  |  |  | Y | Y |  |  | unknown | 138&#8209;144 |
| 240 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 153&#8209;155 |
| 241 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 179&#8209;181 |
| 242 | `flip_colors` |  |  |  | Y | Y |  |  | unknown | 205&#8209;207 |
| 243 | `fix_up` |  |  |  | Y | Y |  |  | unknown | 229&#8209;231 |
| 244 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 268&#8209;271 |
| 245 | `find_link` |  |  |  | Y | Y |  |  | unknown | 288&#8209;292 |
| 246 | `min_link` |  |  |  | Y | Y |  |  | unknown | 308&#8209;313 |
| 247 | `max_link` |  |  |  | Y | Y |  |  | unknown | 324&#8209;329 |
| 248 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 340&#8209;343 |
| 249 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 352&#8209;355 |
| 250 | `in_order_parallel` |  |  |  | Y | Y |  |  | unknown | 364&#8209;367 |
| 251 | `pre_order_parallel` |  |  |  | Y | Y |  |  | unknown | 385&#8209;388 |
| 252 | `build_balanced` |  |  |  | Y | Y |  |  | unknown | 406&#8209;409 |
| 253 | `filter_parallel` |  |  |  | Y | Y |  |  | unknown | 435&#8209;440 |
| 254 | `reduce_parallel` |  |  |  | Y | Y |  |  | unknown | 464&#8209;469 |
| 255 | `height_rec` |  |  |  | Y | Y |  |  | unknown | 490&#8209;494 |
| 256 | `compute_link_spec_size` |  |  |  | Y | Y |  |  | unknown | 503&#8209;506 |
| 257 | `new` | Y | Y |  |  | Y |  |  | unknown | 556&#8209;558 |
| 258 | `from_sorted_slice` | Y | Y |  |  | Y |  |  | unknown | 560&#8209;561 |
| 259 | `insert` | Y | Y |  |  | Y |  |  | unknown | 563&#8209;569 |
| 260 | `contains` | Y | Y |  |  | Y |  |  | unknown | 571&#8209;573 |
| 261 | `size` | Y | Y |  |  | Y |  |  | unknown | 575&#8209;577 |
| 262 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 579&#8209;581 |
| 263 | `height` | Y | Y |  |  | Y |  |  | unknown | 583&#8209;585 |
| 264 | `find` | Y | Y |  |  | Y |  |  | unknown | 587&#8209;588 |
| 265 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 589&#8209;590 |
| 266 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 591&#8209;592 |
| 267 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 593&#8209;594 |
| 268 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 595&#8209;596 |
| 269 | `filter` | Y | Y |  |  | Y |  |  | unknown | 597&#8209;600 |
| 270 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 601&#8209;604 |
| 271 | `default` |  | Y |  |  | Y |  | Y |  | 743 |

### Chap37/BSTRBStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 272 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 51&#8209;83 |
| 273 | `new` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;110 |
| 274 | `size` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;115 |
| 275 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 276 | `height` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;123 |
| 277 | `insert` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;133 |
| 278 | `contains` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 279 | `find` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;145 |
| 280 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 151&#8209;157 |
| 281 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 249&#8209;255 |
| 282 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 346&#8209;353 |
| 283 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 475&#8209;478 |
| 284 | `find_node` |  |  |  | Y | Y |  |  | unknown | 508&#8209;513 |
| 285 | `min_node` |  |  |  | Y | Y |  |  | unknown | 543&#8209;549 |
| 286 | `max_node` |  |  |  | Y | Y |  |  | unknown | 563&#8209;569 |

### Chap37/BSTSetAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 287 | `empty` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 288 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 289 | `size` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 290 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;86 |
| 291 | `find` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;89 |
| 292 | `contains` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 293 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 294 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;98 |
| 295 | `insert` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |
| 296 | `delete` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;104 |
| 297 | `union` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 298 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 299 | `difference` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;113 |
| 300 | `split` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 301 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 302 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 303 | `filter` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 304 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 305 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 306 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 307 | `iter` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;137 |
| 308 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 142&#8209;144 |
| 309 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 149&#8209;151 |
| 310 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 160&#8209;164 |
| 311 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 173&#8209;175 |
| 312 | `next` |  | Y |  |  | Y |  |  | unknown | 384&#8209;400 |

### Chap37/BSTSetBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 313 | `empty` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 314 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 315 | `size` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 316 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;70 |
| 317 | `find` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;73 |
| 318 | `contains` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;76 |
| 319 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 320 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 321 | `insert` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;85 |
| 322 | `delete` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 323 | `union` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;91 |
| 324 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 325 | `difference` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 326 | `split` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 327 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 328 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 329 | `filter` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 330 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;112 |
| 331 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 332 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 333 | `iter` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 334 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 124&#8209;126 |
| 335 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 130&#8209;132 |
| 336 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 141&#8209;145 |
| 337 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 154&#8209;156 |
| 338 | `next` |  | Y |  |  | Y |  |  | unknown | 310&#8209;326 |

### Chap37/BSTSetPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 339 | `empty` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 340 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 341 | `size` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 342 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;70 |
| 343 | `find` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;73 |
| 344 | `contains` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;76 |
| 345 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 346 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 347 | `insert` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;85 |
| 348 | `delete` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 349 | `union` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;91 |
| 350 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 351 | `difference` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 352 | `split` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 353 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 354 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 355 | `filter` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 356 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;112 |
| 357 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 358 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 359 | `iter` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 360 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 124&#8209;126 |
| 361 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 130&#8209;132 |
| 362 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 141&#8209;145 |
| 363 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 154&#8209;156 |
| 364 | `next` |  | Y |  |  | Y |  |  | unknown | 366&#8209;382 |

### Chap37/BSTSetRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 365 | `empty` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 366 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 367 | `size` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 368 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;70 |
| 369 | `find` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;73 |
| 370 | `contains` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;76 |
| 371 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 372 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 373 | `insert` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;85 |
| 374 | `delete` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 375 | `union` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;91 |
| 376 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 377 | `difference` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 378 | `split` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 379 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 380 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 381 | `filter` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 382 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;112 |
| 383 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 384 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 385 | `iter` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 386 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 124&#8209;126 |
| 387 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 131&#8209;133 |
| 388 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 139&#8209;141 |
| 389 | `next` |  | Y |  |  | Y |  |  | unknown | 386&#8209;402 |

### Chap37/BSTSetSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 390 | `empty` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 391 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 392 | `size` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 393 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;86 |
| 394 | `find` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;89 |
| 395 | `contains` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 396 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 397 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;98 |
| 398 | `insert` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |
| 399 | `delete` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;104 |
| 400 | `union` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 401 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 402 | `difference` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;113 |
| 403 | `split` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 404 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 405 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 406 | `filter` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 407 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 408 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 409 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 410 | `iter` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;137 |
| 411 | `values_vec` |  |  |  | Y | Y |  |  | unknown | 142&#8209;144 |
| 412 | `rebuild_from_vec` |  |  |  | Y | Y |  |  | unknown | 149&#8209;151 |
| 413 | `from_sorted_iter` |  |  |  | Y | Y |  |  | unknown | 160&#8209;164 |
| 414 | `copy_set` |  |  |  | Y | Y |  |  | unknown | 173&#8209;175 |
| 415 | `next` |  | Y |  |  | Y |  |  | unknown | 384&#8209;400 |

### Chap37/BSTSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 416 | `new_node` |  |  |  | Y | Y |  |  | unknown | 92&#8209;98 |
| 417 | `size_link` |  |  |  | Y | Y |  |  | unknown | 108&#8209;111 |
| 418 | `update` |  |  |  | Y | Y |  |  | unknown | 119&#8209;124 |
| 419 | `splay` |  |  |  | Y | Y |  |  | unknown | 135&#8209;138 |
| 420 | `bst_insert` |  |  |  | Y | Y |  |  | unknown | 252&#8209;255 |
| 421 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 276&#8209;278 |
| 422 | `find_link` |  |  |  | Y | Y |  |  | unknown | 290&#8209;294 |
| 423 | `min_link` |  |  |  | Y | Y |  |  | unknown | 310&#8209;315 |
| 424 | `max_link` |  |  |  | Y | Y |  |  | unknown | 326&#8209;331 |
| 425 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 342&#8209;345 |
| 426 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 354&#8209;357 |
| 427 | `in_order_parallel` |  |  |  | Y | Y |  |  | unknown | 366&#8209;369 |
| 428 | `pre_order_parallel` |  |  |  | Y | Y |  |  | unknown | 387&#8209;390 |
| 429 | `build_balanced` |  |  |  | Y | Y |  |  | unknown | 408&#8209;411 |
| 430 | `filter_parallel` |  |  |  | Y | Y |  |  | unknown | 436&#8209;441 |
| 431 | `reduce_parallel` |  |  |  | Y | Y |  |  | unknown | 465&#8209;470 |
| 432 | `height_rec` |  |  |  | Y | Y |  |  | unknown | 491&#8209;495 |
| 433 | `compute_link_spec_size` |  |  |  | Y | Y |  |  | unknown | 504&#8209;507 |
| 434 | `new` | Y | Y |  |  | Y |  |  | unknown | 557&#8209;559 |
| 435 | `from_sorted_slice` | Y | Y |  |  | Y |  |  | unknown | 561&#8209;562 |
| 436 | `insert` | Y | Y |  |  | Y |  |  | unknown | 564&#8209;570 |
| 437 | `contains` | Y | Y |  |  | Y |  |  | unknown | 572&#8209;574 |
| 438 | `size` | Y | Y |  |  | Y |  |  | unknown | 576&#8209;578 |
| 439 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 580&#8209;582 |
| 440 | `height` | Y | Y |  |  | Y |  |  | unknown | 584&#8209;586 |
| 441 | `find` | Y | Y |  |  | Y |  |  | unknown | 588&#8209;589 |
| 442 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 590&#8209;591 |
| 443 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 592&#8209;593 |
| 444 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 594&#8209;595 |
| 445 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 596&#8209;597 |
| 446 | `filter` | Y | Y |  |  | Y |  |  | unknown | 598&#8209;601 |
| 447 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 602&#8209;605 |
| 448 | `default` |  | Y |  |  | Y |  | Y |  | 741 |

### Chap37/BSTSplayStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 449 | `new` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;120 |
| 450 | `size` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;123 |
| 451 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 452 | `height` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;131 |
| 453 | `insert` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;135 |
| 454 | `find` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |
| 455 | `contains` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;141 |
| 456 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;146 |
| 457 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;151 |
| 458 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;154 |
| 459 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;157 |
| 460 | `new_node` |  |  |  | Y | Y |  |  | unknown | 163&#8209;169 |
| 461 | `size_link` |  |  |  | Y | Y |  |  | unknown | 179&#8209;181 |
| 462 | `height_link` |  |  |  | Y | Y |  |  | unknown | 190&#8209;193 |
| 463 | `update` |  |  |  | Y | Y |  |  | unknown | 207&#8209;212 |
| 464 | `splay` |  |  |  | Y | Y |  |  | unknown | 224&#8209;227 |
| 465 | `bst_insert` |  |  |  | Y | Y |  |  | unknown | 347&#8209;350 |
| 466 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 371&#8209;373 |
| 467 | `find_link` |  |  |  | Y | Y |  |  | unknown | 385&#8209;389 |
| 468 | `min_link` |  |  |  | Y | Y |  |  | unknown | 403&#8209;408 |
| 469 | `max_link` |  |  |  | Y | Y |  |  | unknown | 419&#8209;424 |
| 470 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 435&#8209;438 |
| 471 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 447&#8209;450 |
| 472 | `default` |  | Y |  |  | Y |  | Y |  | 500 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
