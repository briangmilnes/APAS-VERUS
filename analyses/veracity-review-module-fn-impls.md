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
| 1 | Chap37 | AVLTreeSeq | 20 | 23 | 0 | 13 | 35 | 1 | 31 | 4 | 1 |
| 2 | Chap37 | AVLTreeSeqMtPer | 11 | 14 | 0 | 13 | 25 | 2 | 11 | 12 | 4 |
| 3 | Chap37 | AVLTreeSeqStEph | 18 | 21 | 0 | 12 | 30 | 3 | 17 | 13 | 3 |
| 4 | Chap37 | AVLTreeSeqStPer | 13 | 16 | 0 | 14 | 27 | 3 | 11 | 14 | 5 |
| 5 | Chap37 | BSTAVLMtEph | 0 | 0 | 6 | 8 | 14 | 0 | 6 | 0 | 8 |
| 6 | Chap37 | BSTAVLStEph | 0 | 0 | 0 | 17 | 17 | 0 | 15 | 0 | 2 |
| 7 | Chap37 | BSTBBAlphaMtEph | 0 | 0 | 6 | 5 | 11 | 0 | 3 | 0 | 8 |
| 8 | Chap37 | BSTBBAlphaStEph | 0 | 0 | 0 | 12 | 12 | 0 | 10 | 0 | 2 |
| 9 | Chap37 | BSTPlainMtEph | 0 | 0 | 6 | 5 | 11 | 0 | 3 | 0 | 8 |
| 10 | Chap37 | BSTPlainStEph | 0 | 0 | 0 | 12 | 12 | 0 | 10 | 0 | 2 |
| 11 | Chap37 | BSTRBMtEph | 14 | 16 | 0 | 20 | 1 | 35 | 0 | 1 | 35 |
| 12 | Chap37 | BSTRBStEph | 0 | 0 | 0 | 15 | 15 | 0 | 13 | 0 | 2 |
| 13 | Chap37 | BSTSetAVLMtEph | 20 | 20 | 0 | 3 | 23 | 0 | 0 | 0 | 23 |
| 14 | Chap37 | BSTSetBBAlphaMtEph | 20 | 20 | 0 | 3 | 23 | 0 | 0 | 0 | 23 |
| 15 | Chap37 | BSTSetPlainMtEph | 20 | 20 | 0 | 3 | 23 | 0 | 0 | 0 | 23 |
| 16 | Chap37 | BSTSetRBMtEph | 20 | 20 | 0 | 2 | 22 | 0 | 0 | 0 | 22 |
| 17 | Chap37 | BSTSetSplayMtEph | 20 | 20 | 0 | 2 | 22 | 0 | 0 | 0 | 22 |
| 18 | Chap37 | BSTSplayMtEph | 14 | 16 | 0 | 17 | 1 | 32 | 0 | 1 | 32 |
| 19 | Chap37 | BSTSplayStEph | 11 | 12 | 0 | 12 | 24 | 0 | 3 | 1 | 20 |

## Function-by-Function Detail

### Chap37/AVLTreeSeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 141&#8209;144 |
| 2 | `lemma_inorder_values_maps_to_inorder` |  |  |  | Y | Y |  |  | unknown | 167&#8209;169 |
| 3 | `empty` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;187 |
| 4 | `new` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;190 |
| 5 | `length` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;194 |
| 6 | `nth` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;198 |
| 7 | `set` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;205 |
| 8 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;211 |
| 9 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;215 |
| 10 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;219 |
| 11 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;224 |
| 12 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;227 |
| 13 | `update` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;235 |
| 14 | `from_vec` | Y | Y |  |  | Y |  |  | hole | 237&#8209;241 |
| 15 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 243&#8209;248 |
| 16 | `iter` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;255 |
| 17 | `push_back` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;259 |
| 18 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;264 |
| 19 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;268 |
| 20 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 270&#8209;281 |
| 21 | `is_tree_empty` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;285 |
| 22 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 287&#8209;289 |
| 23 | `cached_height` |  |  |  | Y | Y |  |  | unknown | 294&#8209;295 |
| 24 | `cached_size` |  |  |  | Y | Y |  |  | unknown | 303&#8209;305 |
| 25 | `update_size_height` |  |  |  | Y | Y |  |  | unknown | 315&#8209;336 |
| 26 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 345&#8209;355 |
| 27 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 404&#8209;414 |
| 28 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 456&#8209;471 |
| 29 | `insert_at_link` |  |  |  | Y | Y |  |  | hole | 511&#8209;522 |
| 30 | `nth_link` |  |  |  | Y | Y |  |  | unknown | 591&#8209;594 |
| 31 | `set_link` |  |  |  | Y | Y |  |  | unknown | 609&#8209;619 |
| 32 | `push_inorder` |  |  |  | Y | Y |  |  | unknown | 636&#8209;641 |
| 33 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 671&#8209;676 |
| 34 | `next` |  | Y |  |  | Y |  |  | hole | 1066&#8209;1082 |
| 35 | `eq` |  | Y |  |  | Y |  |  | hole | 1174&#8209;1175 |
| 36 | `default` |  | Y |  |  |  | Y | Y |  | 1213 |

### Chap37/AVLTreeSeqMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 37 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 124&#8209;127 |
| 38 | `empty` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;145 |
| 39 | `new` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;148 |
| 40 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;151 |
| 41 | `length` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;155 |
| 42 | `nth` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;159 |
| 43 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;163 |
| 44 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;167 |
| 45 | `set` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;170 |
| 46 | `subseq_copy` | Y | Y |  |  | Y |  |  | hole | 172&#8209;173 |
| 47 | `from_vec` | Y | Y |  |  | Y |  | Y |  | 175 |
| 48 | `values_in_order` | Y | Y |  |  | Y |  |  | hole | 177 |
| 49 | `height_fn` |  |  |  | Y | Y |  |  | unknown | 182&#8209;183 |
| 50 | `size_fn` |  |  |  | Y | Y |  |  | unknown | 191&#8209;192 |
| 51 | `mk` |  |  |  | Y | Y |  |  | hole | 201&#8209;210 |
| 52 | `rotate_right` |  |  |  | Y | Y |  |  | hole | 220&#8209;224 |
| 53 | `rotate_left` |  |  |  | Y | Y |  |  | hole | 233&#8209;237 |
| 54 | `rebalance` |  |  |  | Y | Y |  |  | hole | 246&#8209;250 |
| 55 | `nth_ref` |  |  |  | Y | Y |  |  | hole | 274&#8209;276 |
| 56 | `set_rec` |  |  |  | Y | Y |  |  | hole | 295&#8209;297 |
| 57 | `inorder_collect` |  |  |  | Y | Y |  |  | hole | 323 |
| 58 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | hole | 332&#8209;333 |
| 59 | `rec` |  |  |  | Y | Y |  | Y |  | 335 |
| 60 | `compare_trees` |  |  |  | Y | Y |  |  | hole | 350 |
| 61 | `eq` |  | Y |  |  | Y |  |  | hole | 468&#8209;469 |
| 62 | `default` |  | Y |  |  |  | Y | Y |  | 499 |
| 63 | `next` |  | Y |  |  |  | Y | Y |  | 518&#8209;526 |

### Chap37/AVLTreeSeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 64 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 130&#8209;133 |
| 65 | `empty` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;151 |
| 66 | `new` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;154 |
| 67 | `length` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;158 |
| 68 | `nth` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;162 |
| 69 | `set` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;165 |
| 70 | `singleton` | Y | Y |  |  | Y |  |  | hole | 167&#8209;168 |
| 71 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;172 |
| 72 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;176 |
| 73 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;179 |
| 74 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;182 |
| 75 | `update` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;187 |
| 76 | `from_vec` | Y | Y |  |  | Y |  |  | hole | 189 |
| 77 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;192 |
| 78 | `iter` | Y | Y |  |  | Y |  |  | hole | 194 |
| 79 | `push_back` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;197 |
| 80 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;200 |
| 81 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;203 |
| 82 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;206 |
| 83 | `h_fn` |  |  |  | Y | Y |  |  | unknown | 211&#8209;212 |
| 84 | `size_link_fn` |  |  |  | Y | Y |  |  | hole | 220&#8209;221 |
| 85 | `update_meta` |  |  |  | Y | Y |  |  | hole | 233 |
| 86 | `rotate_right_fn` |  |  |  | Y | Y |  |  | hole | 242&#8209;246 |
| 87 | `rotate_left_fn` |  |  |  | Y | Y |  |  | hole | 259&#8209;263 |
| 88 | `rebalance_fn` |  |  |  | Y | Y |  |  | hole | 276&#8209;280 |
| 89 | `insert_at_link` |  |  |  | Y | Y |  |  | hole | 303 |
| 90 | `nth_link` |  |  |  | Y | Y |  |  | hole | 332&#8209;334 |
| 91 | `set_link` |  |  |  | Y | Y |  |  | hole | 348 |
| 92 | `compare_trees` |  |  |  | Y | Y |  |  | hole | 366 |
| 93 | `eq` |  | Y |  |  | Y |  |  | hole | 621&#8209;622 |
| 94 | `default` |  | Y |  |  |  | Y | Y |  | 647 |
| 95 | `push_left_iter` |  |  |  | Y |  | Y | Y |  | 652&#8209;658 |
| 96 | `next` |  | Y |  |  |  | Y | Y |  | 662&#8209;667 |

### Chap37/AVLTreeSeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 97 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 130&#8209;133 |
| 98 | `empty` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;151 |
| 99 | `new` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;154 |
| 100 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;157 |
| 101 | `length` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;161 |
| 102 | `nth` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;165 |
| 103 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;169 |
| 104 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;173 |
| 105 | `set` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;176 |
| 106 | `subseq_copy` | Y | Y |  |  | Y |  |  | hole | 178&#8209;179 |
| 107 | `from_vec` | Y | Y |  |  | Y |  | Y |  | 181 |
| 108 | `values_in_order` | Y | Y |  |  | Y |  |  | hole | 183 |
| 109 | `to_arrayseq` | Y | Y |  |  | Y |  |  | hole | 185 |
| 110 | `iter` | Y | Y |  |  | Y |  |  | hole | 187 |
| 111 | `height_fn` |  |  |  | Y | Y |  |  | unknown | 192&#8209;193 |
| 112 | `size_fn` |  |  |  | Y | Y |  |  | unknown | 201&#8209;202 |
| 113 | `mk` |  |  |  | Y | Y |  |  | hole | 211&#8209;220 |
| 114 | `rotate_right` |  |  |  | Y | Y |  |  | hole | 230&#8209;234 |
| 115 | `rotate_left` |  |  |  | Y | Y |  |  | hole | 243&#8209;247 |
| 116 | `rebalance` |  |  |  | Y | Y |  |  | hole | 256&#8209;260 |
| 117 | `nth_ref` |  |  |  | Y | Y |  |  | hole | 284&#8209;286 |
| 118 | `set_rec` |  |  |  | Y | Y |  |  | hole | 305&#8209;307 |
| 119 | `inorder_collect` |  |  |  | Y | Y |  |  | hole | 333 |
| 120 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | hole | 342&#8209;343 |
| 121 | `rec` |  |  |  | Y | Y |  | Y |  | 345 |
| 122 | `compare_trees` |  |  |  | Y | Y |  |  | hole | 358 |
| 123 | `eq` |  | Y |  |  | Y |  |  | hole | 484&#8209;485 |
| 124 | `default` |  | Y |  |  |  | Y | Y |  | 509 |
| 125 | `push_left_iter` |  |  |  | Y |  | Y | Y |  | 526&#8209;531 |
| 126 | `next` |  | Y |  |  |  | Y | Y |  | 535&#8209;544 |

### Chap37/BSTAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 127 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 53&#8209;85 |
| 128 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 100&#8209;104 |
| 129 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 166&#8209;170 |
| 130 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 233&#8209;242 |
| 131 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 325&#8209;328 |
| 132 | `find_node` |  |  |  | Y | Y |  |  | unknown | 350&#8209;355 |
| 133 | `min_node` |  |  |  | Y | Y |  | Y |  | 377&#8209;378 |
| 134 | `max_node` |  |  |  | Y | Y |  | Y |  | 389&#8209;390 |
| 135 | `new` |  |  | Y |  | Y |  | Y |  | 404 |
| 136 | `insert` |  |  | Y |  | Y |  | Y |  | 414 |
| 137 | `contains` |  |  | Y |  | Y |  | Y |  | 437 |
| 138 | `size` |  |  | Y |  | Y |  | Y |  | 446 |
| 139 | `is_empty` |  |  | Y |  | Y |  | Y |  | 456 |
| 140 | `height` |  |  | Y |  | Y |  | Y |  | 465 |

### Chap37/BSTAVLStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 141 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 56&#8209;88 |
| 142 | `lemma_max_plus_one` |  |  |  | Y | Y |  |  | unknown | 95&#8209;97 |
| 143 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 108&#8209;146 |
| 144 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 260&#8209;298 |
| 145 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 419&#8209;447 |
| 146 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 705&#8209;716 |
| 147 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 859&#8209;862 |
| 148 | `find_node` |  |  |  | Y | Y |  |  | unknown | 892&#8209;897 |
| 149 | `min_node` |  |  |  | Y | Y |  | Y |  | 927&#8209;928 |
| 150 | `max_node` |  |  |  | Y | Y |  | Y |  | 942&#8209;943 |
| 151 | `avl_new` |  |  |  | Y | Y |  |  | unknown | 957&#8209;960 |
| 152 | `avl_size` |  |  |  | Y | Y |  |  | unknown | 965&#8209;967 |
| 153 | `avl_is_empty` |  |  |  | Y | Y |  |  | unknown | 972&#8209;973 |
| 154 | `avl_height` |  |  |  | Y | Y |  |  | unknown | 978&#8209;980 |
| 155 | `avl_insert` |  |  |  | Y | Y |  |  | unknown | 985&#8209;993 |
| 156 | `avl_contains` |  |  |  | Y | Y |  |  | unknown | 998&#8209;1000 |
| 157 | `avl_find` |  |  |  | Y | Y |  |  | unknown | 1005&#8209;1009 |

### Chap37/BSTBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 158 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 63&#8209;72 |
| 159 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 155&#8209;158 |
| 160 | `find_node` |  |  |  | Y | Y |  |  | unknown | 180&#8209;185 |
| 161 | `min_node` |  |  |  | Y | Y |  | Y |  | 207&#8209;208 |
| 162 | `max_node` |  |  |  | Y | Y |  | Y |  | 219&#8209;220 |
| 163 | `new` |  |  | Y |  | Y |  | Y |  | 234 |
| 164 | `insert` |  |  | Y |  | Y |  | Y |  | 244 |
| 165 | `contains` |  |  | Y |  | Y |  | Y |  | 267 |
| 166 | `size` |  |  | Y |  | Y |  | Y |  | 276 |
| 167 | `is_empty` |  |  | Y |  | Y |  | Y |  | 286 |
| 168 | `height` |  |  | Y |  | Y |  | Y |  | 295 |

### Chap37/BSTBBAlphaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 169 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 60&#8209;67 |
| 170 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 189&#8209;192 |
| 171 | `find_node` |  |  |  | Y | Y |  |  | unknown | 222&#8209;227 |
| 172 | `min_node` |  |  |  | Y | Y |  | Y |  | 257&#8209;258 |
| 173 | `max_node` |  |  |  | Y | Y |  | Y |  | 272&#8209;273 |
| 174 | `bb_new` |  |  |  | Y | Y |  |  | unknown | 287&#8209;290 |
| 175 | `bb_size` |  |  |  | Y | Y |  |  | unknown | 295&#8209;297 |
| 176 | `bb_is_empty` |  |  |  | Y | Y |  |  | unknown | 302&#8209;303 |
| 177 | `bb_height` |  |  |  | Y | Y |  |  | unknown | 308&#8209;310 |
| 178 | `bb_insert` |  |  |  | Y | Y |  |  | unknown | 315&#8209;321 |
| 179 | `bb_contains` |  |  |  | Y | Y |  |  | unknown | 326&#8209;328 |
| 180 | `bb_find` |  |  |  | Y | Y |  |  | unknown | 333&#8209;337 |

### Chap37/BSTPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 181 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 58&#8209;67 |
| 182 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 150&#8209;153 |
| 183 | `find_node` |  |  |  | Y | Y |  |  | unknown | 175&#8209;180 |
| 184 | `min_node` |  |  |  | Y | Y |  | Y |  | 202&#8209;203 |
| 185 | `max_node` |  |  |  | Y | Y |  | Y |  | 214&#8209;215 |
| 186 | `new` |  |  | Y |  | Y |  | Y |  | 229 |
| 187 | `insert` |  |  | Y |  | Y |  | Y |  | 239 |
| 188 | `contains` |  |  | Y |  | Y |  | Y |  | 262 |
| 189 | `is_empty` |  |  | Y |  | Y |  | Y |  | 271 |
| 190 | `size` |  |  | Y |  | Y |  | Y |  | 280 |
| 191 | `height` |  |  | Y |  | Y |  | Y |  | 290 |

### Chap37/BSTPlainStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 192 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 64&#8209;71 |
| 193 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 193&#8209;196 |
| 194 | `find_node` |  |  |  | Y | Y |  |  | unknown | 226&#8209;231 |
| 195 | `min_node` |  |  |  | Y | Y |  | Y |  | 261&#8209;262 |
| 196 | `max_node` |  |  |  | Y | Y |  | Y |  | 276&#8209;277 |
| 197 | `bst_new` |  |  |  | Y | Y |  |  | unknown | 291&#8209;294 |
| 198 | `bst_size` |  |  |  | Y | Y |  |  | unknown | 299&#8209;301 |
| 199 | `bst_is_empty` |  |  |  | Y | Y |  |  | unknown | 306&#8209;307 |
| 200 | `bst_height` |  |  |  | Y | Y |  |  | unknown | 312&#8209;314 |
| 201 | `bst_insert` |  |  |  | Y | Y |  |  | unknown | 319&#8209;325 |
| 202 | `bst_contains` |  |  |  | Y | Y |  |  | unknown | 330&#8209;332 |
| 203 | `bst_find` |  |  |  | Y | Y |  |  | unknown | 337&#8209;341 |

### Chap37/BSTRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 204 | `new_rb_link_lock` |  |  |  | Y | Y |  |  | hole | 333 |
| 205 | `new_node` |  |  |  | Y |  | Y | Y |  | 34&#8209;42 |
| 206 | `is_red` |  |  |  | Y |  | Y | Y |  | 44 |
| 207 | `size_link` |  |  |  | Y |  | Y | Y |  | 46 |
| 208 | `update` |  |  |  | Y |  | Y | Y |  | 48 |
| 209 | `rotate_left` |  |  |  | Y |  | Y | Y |  | 50&#8209;67 |
| 210 | `rotate_right` |  |  |  | Y |  | Y | Y |  | 69&#8209;86 |
| 211 | `flip_colors` |  |  |  | Y |  | Y | Y |  | 88&#8209;107 |
| 212 | `fix_up` |  |  |  | Y |  | Y | Y |  | 109&#8209;143 |
| 213 | `insert_link` |  |  |  | Y |  | Y | Y |  | 145&#8209;159 |
| 214 | `find_link` |  |  |  | Y |  | Y | Y |  | 161&#8209;174 |
| 215 | `min_link` |  |  |  | Y |  | Y | Y |  | 176&#8209;184 |
| 216 | `max_link` |  |  |  | Y |  | Y | Y |  | 186&#8209;194 |
| 217 | `in_order_collect` |  |  |  | Y |  | Y | Y |  | 196&#8209;202 |
| 218 | `pre_order_collect` |  |  |  | Y |  | Y | Y |  | 204&#8209;210 |
| 219 | `in_order_parallel` |  |  |  | Y |  | Y | Y |  | 214&#8209;229 |
| 220 | `pre_order_parallel` |  |  |  | Y |  | Y | Y |  | 231&#8209;246 |
| 221 | `build_balanced` |  |  |  | Y |  | Y | Y |  | 248&#8209;267 |
| 222 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 269&#8209;294 |
| 223 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 296&#8209;318 |
| 224 | `new` | Y | Y |  |  |  | Y | Y |  | 353&#8209;354 |
| 225 | `from_sorted_slice` | Y | Y |  |  |  | Y | Y |  | 355&#8209;356 |
| 226 | `insert` | Y | Y |  |  |  | Y | Y |  | 357&#8209;358 |
| 227 | `find` | Y | Y |  |  |  | Y | Y |  | 359&#8209;360 |
| 228 | `contains` | Y | Y |  |  |  | Y | Y |  | 361 |
| 229 | `size` | Y | Y |  |  |  | Y | Y |  | 362 |
| 230 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 363 |
| 231 | `height` | Y | Y |  |  |  | Y | Y |  | 364 |
| 232 | `minimum` | Y | Y |  |  |  | Y | Y |  | 365 |
| 233 | `maximum` | Y | Y |  |  |  | Y | Y |  | 366 |
| 234 | `in_order` | Y | Y |  |  |  | Y | Y |  | 367&#8209;368 |
| 235 | `pre_order` | Y | Y |  |  |  | Y | Y |  | 369&#8209;370 |
| 236 | `filter` | Y | Y |  |  |  | Y | Y |  | 371&#8209;374 |
| 237 | `reduce` | Y | Y |  |  |  | Y | Y |  | 375&#8209;378 |
| 238 | `height_rec` |  | Y |  |  |  | Y | Y |  | 416&#8209;421 |
| 239 | `default` |  | Y |  |  |  | Y | Y |  | 486 |

### Chap37/BSTRBStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 240 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 41&#8209;73 |
| 241 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 87&#8209;93 |
| 242 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 185&#8209;191 |
| 243 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 282&#8209;289 |
| 244 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 411&#8209;414 |
| 245 | `find_node` |  |  |  | Y | Y |  |  | unknown | 444&#8209;449 |
| 246 | `min_node` |  |  |  | Y | Y |  | Y |  | 479&#8209;480 |
| 247 | `max_node` |  |  |  | Y | Y |  | Y |  | 494&#8209;495 |
| 248 | `rb_new` |  |  |  | Y | Y |  |  | unknown | 509&#8209;512 |
| 249 | `rb_size` |  |  |  | Y | Y |  |  | unknown | 517&#8209;519 |
| 250 | `rb_is_empty` |  |  |  | Y | Y |  |  | unknown | 524&#8209;525 |
| 251 | `rb_height` |  |  |  | Y | Y |  |  | unknown | 530&#8209;532 |
| 252 | `rb_insert` |  |  |  | Y | Y |  |  | unknown | 537&#8209;543 |
| 253 | `rb_contains` |  |  |  | Y | Y |  |  | unknown | 548&#8209;550 |
| 254 | `rb_find` |  |  |  | Y | Y |  |  | unknown | 555&#8209;559 |

### Chap37/BSTSetAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 255 | `values_vec` |  |  |  | Y | Y |  | Y |  | 24 |
| 256 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 26 |
| 257 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 34&#8209;36 |
| 258 | `empty` | Y | Y |  |  | Y |  | Y |  | 47 |
| 259 | `singleton` | Y | Y |  |  | Y |  | Y |  | 49 |
| 260 | `size` | Y | Y |  |  | Y |  | Y |  | 51 |
| 261 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 53 |
| 262 | `find` | Y | Y |  |  | Y |  | Y |  | 55 |
| 263 | `contains` | Y | Y |  |  | Y |  | Y |  | 57 |
| 264 | `minimum` | Y | Y |  |  | Y |  | Y |  | 59 |
| 265 | `maximum` | Y | Y |  |  | Y |  | Y |  | 61 |
| 266 | `insert` | Y | Y |  |  | Y |  | Y |  | 63 |
| 267 | `delete` | Y | Y |  |  | Y |  | Y |  | 65 |
| 268 | `union` | Y | Y |  |  | Y |  | Y |  | 67 |
| 269 | `intersection` | Y | Y |  |  | Y |  | Y |  | 69 |
| 270 | `difference` | Y | Y |  |  | Y |  | Y |  | 71 |
| 271 | `split` | Y | Y |  |  | Y |  | Y |  | 73 |
| 272 | `join_pair` | Y | Y |  |  | Y |  | Y |  | 75 |
| 273 | `join_m` | Y | Y |  |  | Y |  | Y |  | 77 |
| 274 | `filter` | Y | Y |  |  | Y |  | Y |  | 79 |
| 275 | `reduce` | Y | Y |  |  | Y |  | Y |  | 81 |
| 276 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 83 |
| 277 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 85 |

### Chap37/BSTSetBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 278 | `empty` | Y | Y |  |  | Y |  | Y |  | 26 |
| 279 | `singleton` | Y | Y |  |  | Y |  | Y |  | 28 |
| 280 | `size` | Y | Y |  |  | Y |  | Y |  | 30 |
| 281 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 32 |
| 282 | `find` | Y | Y |  |  | Y |  | Y |  | 34 |
| 283 | `contains` | Y | Y |  |  | Y |  | Y |  | 36 |
| 284 | `minimum` | Y | Y |  |  | Y |  | Y |  | 38 |
| 285 | `maximum` | Y | Y |  |  | Y |  | Y |  | 40 |
| 286 | `insert` | Y | Y |  |  | Y |  | Y |  | 42 |
| 287 | `delete` | Y | Y |  |  | Y |  | Y |  | 44 |
| 288 | `union` | Y | Y |  |  | Y |  | Y |  | 46 |
| 289 | `intersection` | Y | Y |  |  | Y |  | Y |  | 48 |
| 290 | `difference` | Y | Y |  |  | Y |  | Y |  | 50 |
| 291 | `split` | Y | Y |  |  | Y |  | Y |  | 52 |
| 292 | `join_pair` | Y | Y |  |  | Y |  | Y |  | 54 |
| 293 | `join_m` | Y | Y |  |  | Y |  | Y |  | 56 |
| 294 | `filter` | Y | Y |  |  | Y |  | Y |  | 58 |
| 295 | `reduce` | Y | Y |  |  | Y |  | Y |  | 60 |
| 296 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 62 |
| 297 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 64 |
| 298 | `values_vec` |  |  |  | Y | Y |  | Y |  | 67 |
| 299 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 70 |
| 300 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 75 |

### Chap37/BSTSetPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 301 | `empty` | Y | Y |  |  | Y |  | Y |  | 26 |
| 302 | `singleton` | Y | Y |  |  | Y |  | Y |  | 28 |
| 303 | `size` | Y | Y |  |  | Y |  | Y |  | 30 |
| 304 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 32 |
| 305 | `find` | Y | Y |  |  | Y |  | Y |  | 34 |
| 306 | `contains` | Y | Y |  |  | Y |  | Y |  | 36 |
| 307 | `minimum` | Y | Y |  |  | Y |  | Y |  | 38 |
| 308 | `maximum` | Y | Y |  |  | Y |  | Y |  | 40 |
| 309 | `insert` | Y | Y |  |  | Y |  | Y |  | 42 |
| 310 | `delete` | Y | Y |  |  | Y |  | Y |  | 44 |
| 311 | `union` | Y | Y |  |  | Y |  | Y |  | 46 |
| 312 | `intersection` | Y | Y |  |  | Y |  | Y |  | 48 |
| 313 | `difference` | Y | Y |  |  | Y |  | Y |  | 50 |
| 314 | `split` | Y | Y |  |  | Y |  | Y |  | 52 |
| 315 | `join_pair` | Y | Y |  |  | Y |  | Y |  | 54 |
| 316 | `join_m` | Y | Y |  |  | Y |  | Y |  | 56 |
| 317 | `filter` | Y | Y |  |  | Y |  | Y |  | 58 |
| 318 | `reduce` | Y | Y |  |  | Y |  | Y |  | 60 |
| 319 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 62 |
| 320 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 64 |
| 321 | `values_vec` |  |  |  | Y | Y |  | Y |  | 67 |
| 322 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 70 |
| 323 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 75 |

### Chap37/BSTSetRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 324 | `empty` | Y | Y |  |  | Y |  | Y |  | 26 |
| 325 | `singleton` | Y | Y |  |  | Y |  | Y |  | 28 |
| 326 | `size` | Y | Y |  |  | Y |  | Y |  | 30 |
| 327 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 32 |
| 328 | `find` | Y | Y |  |  | Y |  | Y |  | 34 |
| 329 | `contains` | Y | Y |  |  | Y |  | Y |  | 36 |
| 330 | `minimum` | Y | Y |  |  | Y |  | Y |  | 38 |
| 331 | `maximum` | Y | Y |  |  | Y |  | Y |  | 40 |
| 332 | `insert` | Y | Y |  |  | Y |  | Y |  | 42 |
| 333 | `delete` | Y | Y |  |  | Y |  | Y |  | 44 |
| 334 | `union` | Y | Y |  |  | Y |  | Y |  | 46 |
| 335 | `intersection` | Y | Y |  |  | Y |  | Y |  | 48 |
| 336 | `difference` | Y | Y |  |  | Y |  | Y |  | 50 |
| 337 | `split` | Y | Y |  |  | Y |  | Y |  | 52 |
| 338 | `join_pair` | Y | Y |  |  | Y |  | Y |  | 54 |
| 339 | `join_m` | Y | Y |  |  | Y |  | Y |  | 56 |
| 340 | `filter` | Y | Y |  |  | Y |  | Y |  | 58 |
| 341 | `reduce` | Y | Y |  |  | Y |  | Y |  | 60 |
| 342 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 62 |
| 343 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 64 |
| 344 | `values_vec` |  |  |  | Y | Y |  | Y |  | 67 |
| 345 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 70 |

### Chap37/BSTSetSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 346 | `empty` | Y | Y |  |  | Y |  | Y |  | 26 |
| 347 | `singleton` | Y | Y |  |  | Y |  | Y |  | 28 |
| 348 | `size` | Y | Y |  |  | Y |  | Y |  | 30 |
| 349 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 32 |
| 350 | `find` | Y | Y |  |  | Y |  | Y |  | 34 |
| 351 | `contains` | Y | Y |  |  | Y |  | Y |  | 36 |
| 352 | `minimum` | Y | Y |  |  | Y |  | Y |  | 38 |
| 353 | `maximum` | Y | Y |  |  | Y |  | Y |  | 40 |
| 354 | `insert` | Y | Y |  |  | Y |  | Y |  | 42 |
| 355 | `delete` | Y | Y |  |  | Y |  | Y |  | 44 |
| 356 | `union` | Y | Y |  |  | Y |  | Y |  | 46 |
| 357 | `intersection` | Y | Y |  |  | Y |  | Y |  | 48 |
| 358 | `difference` | Y | Y |  |  | Y |  | Y |  | 50 |
| 359 | `split` | Y | Y |  |  | Y |  | Y |  | 52 |
| 360 | `join_pair` | Y | Y |  |  | Y |  | Y |  | 54 |
| 361 | `join_m` | Y | Y |  |  | Y |  | Y |  | 56 |
| 362 | `filter` | Y | Y |  |  | Y |  | Y |  | 58 |
| 363 | `reduce` | Y | Y |  |  | Y |  | Y |  | 60 |
| 364 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 62 |
| 365 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 64 |
| 366 | `values_vec` |  |  |  | Y | Y |  | Y |  | 67 |
| 367 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 71 |

### Chap37/BSTSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 368 | `new_splay_link_lock` |  |  |  | Y | Y |  |  | hole | 355 |
| 369 | `new_node` |  |  |  | Y |  | Y | Y |  | 27&#8209;34 |
| 370 | `size_link` |  |  |  | Y |  | Y | Y |  | 36 |
| 371 | `update` |  |  |  | Y |  | Y | Y |  | 38 |
| 372 | `splay` |  |  |  | Y |  | Y | Y |  | 40&#8209;151 |
| 373 | `bst_insert` |  |  |  | Y |  | Y | Y |  | 153&#8209;171 |
| 374 | `insert_link` |  |  |  | Y |  | Y | Y |  | 173&#8209;182 |
| 375 | `find_link` |  |  |  | Y |  | Y | Y |  | 184&#8209;197 |
| 376 | `min_link` |  |  |  | Y |  | Y | Y |  | 199&#8209;207 |
| 377 | `max_link` |  |  |  | Y |  | Y | Y |  | 209&#8209;217 |
| 378 | `in_order_collect` |  |  |  | Y |  | Y | Y |  | 219&#8209;225 |
| 379 | `pre_order_collect` |  |  |  | Y |  | Y | Y |  | 227&#8209;233 |
| 380 | `in_order_parallel` |  |  |  | Y |  | Y | Y |  | 237&#8209;252 |
| 381 | `pre_order_parallel` |  |  |  | Y |  | Y | Y |  | 254&#8209;269 |
| 382 | `build_balanced` |  |  |  | Y |  | Y | Y |  | 271&#8209;289 |
| 383 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 291&#8209;316 |
| 384 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 318&#8209;340 |
| 385 | `new` | Y | Y |  |  |  | Y | Y |  | 375&#8209;376 |
| 386 | `from_sorted_slice` | Y | Y |  |  |  | Y | Y |  | 377&#8209;378 |
| 387 | `insert` | Y | Y |  |  |  | Y | Y |  | 379&#8209;380 |
| 388 | `find` | Y | Y |  |  |  | Y | Y |  | 381&#8209;382 |
| 389 | `contains` | Y | Y |  |  |  | Y | Y |  | 383&#8209;384 |
| 390 | `size` | Y | Y |  |  |  | Y | Y |  | 385&#8209;386 |
| 391 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 387&#8209;388 |
| 392 | `height` | Y | Y |  |  |  | Y | Y |  | 389&#8209;390 |
| 393 | `minimum` | Y | Y |  |  |  | Y | Y |  | 391&#8209;392 |
| 394 | `maximum` | Y | Y |  |  |  | Y | Y |  | 393&#8209;394 |
| 395 | `in_order` | Y | Y |  |  |  | Y | Y |  | 395&#8209;396 |
| 396 | `pre_order` | Y | Y |  |  |  | Y | Y |  | 397&#8209;398 |
| 397 | `filter` | Y | Y |  |  |  | Y | Y |  | 399&#8209;402 |
| 398 | `reduce` | Y | Y |  |  |  | Y | Y |  | 403&#8209;406 |
| 399 | `height_rec` |  | Y |  |  |  | Y | Y |  | 441&#8209;446 |
| 400 | `default` |  | Y |  |  |  | Y | Y |  | 511 |

### Chap37/BSTSplayStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 401 | `new_node` |  |  |  | Y | Y |  | Y |  | 38 |
| 402 | `size_link` |  |  |  | Y | Y |  |  | unknown | 67&#8209;68 |
| 403 | `height_link` |  |  |  | Y | Y |  |  | unknown | 77&#8209;80 |
| 404 | `update` |  |  |  | Y | Y |  |  | hole | 94&#8209;99 |
| 405 | `splay` |  |  |  | Y | Y |  | Y |  | 110&#8209;111 |
| 406 | `bst_insert` |  |  |  | Y | Y |  | Y |  | 231&#8209;232 |
| 407 | `insert_link` |  |  |  | Y | Y |  | Y |  | 255 |
| 408 | `find_link` |  |  |  | Y | Y |  | Y |  | 266&#8209;267 |
| 409 | `min_link` |  |  |  | Y | Y |  | Y |  | 283&#8209;284 |
| 410 | `max_link` |  |  |  | Y | Y |  | Y |  | 295&#8209;296 |
| 411 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 307&#8209;308 |
| 412 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 317&#8209;318 |
| 413 | `new` | Y | Y |  |  | Y |  | Y |  | 346&#8209;348 |
| 414 | `size` | Y | Y |  |  | Y |  | Y |  | 350 |
| 415 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 352 |
| 416 | `height` | Y | Y |  |  | Y |  |  | unknown | 354&#8209;355 |
| 417 | `insert` | Y | Y |  |  | Y |  | Y |  | 357 |
| 418 | `find` | Y | Y |  |  | Y |  | Y |  | 359 |
| 419 | `contains` | Y | Y |  |  | Y |  | Y |  | 361 |
| 420 | `minimum` | Y | Y |  |  | Y |  | Y |  | 363 |
| 421 | `maximum` | Y | Y |  |  | Y |  | Y |  | 365 |
| 422 | `in_order` | Y | Y |  |  | Y |  | Y |  | 367 |
| 423 | `pre_order` | Y | Y |  |  | Y |  | Y |  | 369 |
| 424 | `default` |  | Y |  |  | Y |  | Y |  | 410 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
