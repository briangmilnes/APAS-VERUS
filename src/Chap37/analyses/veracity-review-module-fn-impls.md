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
| 1 | Chap37 | AVLTreeSeq | 20 | 23 | 0 | 13 | 35 | 1 | 32 | 3 | 1 |
| 2 | Chap37 | AVLTreeSeqMtPer | 11 | 14 | 0 | 13 | 25 | 2 | 11 | 12 | 4 |
| 3 | Chap37 | AVLTreeSeqStEph | 18 | 21 | 0 | 12 | 30 | 3 | 17 | 13 | 3 |
| 4 | Chap37 | AVLTreeSeqStPer | 13 | 16 | 0 | 14 | 27 | 3 | 11 | 14 | 5 |
| 5 | Chap37 | BSTAVLMtEph | 6 | 6 | 0 | 8 | 14 | 0 | 6 | 0 | 8 |
| 6 | Chap37 | BSTAVLStEph | 0 | 0 | 0 | 17 | 17 | 0 | 15 | 0 | 2 |
| 7 | Chap37 | BSTBBAlphaMtEph | 6 | 6 | 0 | 5 | 11 | 0 | 3 | 0 | 8 |
| 8 | Chap37 | BSTBBAlphaStEph | 10 | 10 | 0 | 7 | 17 | 0 | 17 | 0 | 0 |
| 9 | Chap37 | BSTPlainMtEph | 6 | 6 | 0 | 5 | 11 | 0 | 3 | 0 | 8 |
| 10 | Chap37 | BSTPlainStEph | 10 | 10 | 0 | 7 | 17 | 0 | 17 | 0 | 0 |
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
| 14 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;243 |
| 15 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;250 |
| 16 | `iter` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;257 |
| 17 | `push_back` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;261 |
| 18 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;266 |
| 19 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;270 |
| 20 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;283 |
| 21 | `is_tree_empty` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;287 |
| 22 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;291 |
| 23 | `cached_height` |  |  |  | Y | Y |  |  | unknown | 296&#8209;297 |
| 24 | `cached_size` |  |  |  | Y | Y |  |  | unknown | 305&#8209;307 |
| 25 | `update_size_height` |  |  |  | Y | Y |  |  | unknown | 317&#8209;338 |
| 26 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 347&#8209;357 |
| 27 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 406&#8209;416 |
| 28 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 458&#8209;473 |
| 29 | `insert_at_link` |  |  |  | Y | Y |  |  | hole | 512&#8209;523 |
| 30 | `nth_link` |  |  |  | Y | Y |  |  | unknown | 592&#8209;595 |
| 31 | `set_link` |  |  |  | Y | Y |  |  | unknown | 610&#8209;620 |
| 32 | `push_inorder` |  |  |  | Y | Y |  |  | unknown | 637&#8209;642 |
| 33 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 672&#8209;677 |
| 34 | `next` |  | Y |  |  | Y |  |  | hole | 1073&#8209;1089 |
| 35 | `eq` |  | Y |  |  | Y |  |  | hole | 1181&#8209;1182 |
| 36 | `default` |  | Y |  |  |  | Y | Y |  | 1219 |

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
| 127 | `new` | Y | Y |  |  | Y |  | Y |  | 52 |
| 128 | `insert` | Y | Y |  |  | Y |  | Y |  | 53 |
| 129 | `contains` | Y | Y |  |  | Y |  | Y |  | 54 |
| 130 | `size` | Y | Y |  |  | Y |  | Y |  | 55 |
| 131 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 56 |
| 132 | `height` | Y | Y |  |  | Y |  | Y |  | 57 |
| 133 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 62&#8209;94 |
| 134 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 109&#8209;113 |
| 135 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 175&#8209;179 |
| 136 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 242&#8209;251 |
| 137 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 334&#8209;337 |
| 138 | `find_node` |  |  |  | Y | Y |  |  | unknown | 359&#8209;364 |
| 139 | `min_node` |  |  |  | Y | Y |  | Y |  | 386&#8209;387 |
| 140 | `max_node` |  |  |  | Y | Y |  | Y |  | 398&#8209;399 |

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
| 158 | `new` | Y | Y |  |  | Y |  | Y |  | 51 |
| 159 | `insert` | Y | Y |  |  | Y |  | Y |  | 52 |
| 160 | `contains` | Y | Y |  |  | Y |  | Y |  | 53 |
| 161 | `size` | Y | Y |  |  | Y |  | Y |  | 54 |
| 162 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 55 |
| 163 | `height` | Y | Y |  |  | Y |  | Y |  | 56 |
| 164 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 72&#8209;81 |
| 165 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 164&#8209;167 |
| 166 | `find_node` |  |  |  | Y | Y |  |  | unknown | 189&#8209;194 |
| 167 | `min_node` |  |  |  | Y | Y |  | Y |  | 216&#8209;217 |
| 168 | `max_node` |  |  |  | Y | Y |  | Y |  | 228&#8209;229 |

### Chap37/BSTBBAlphaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 169 | `new` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;72 |
| 170 | `size` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;75 |
| 171 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 172 | `height` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;80 |
| 173 | `insert` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;87 |
| 174 | `contains` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;90 |
| 175 | `find` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 176 | `delete` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;102 |
| 177 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;108 |
| 178 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;114 |
| 179 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 163&#8209;170 |
| 180 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 292&#8209;295 |
| 181 | `find_node` |  |  |  | Y | Y |  |  | unknown | 325&#8209;330 |
| 182 | `min_node` |  |  |  | Y | Y |  |  | unknown | 360&#8209;366 |
| 183 | `max_node` |  |  |  | Y | Y |  |  | unknown | 380&#8209;386 |
| 184 | `delete_min_node` |  |  |  | Y | Y |  |  | unknown | 401&#8209;412 |
| 185 | `delete_node` |  |  |  | Y | Y |  |  | unknown | 515&#8209;522 |

### Chap37/BSTPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 186 | `new` | Y | Y |  |  | Y |  | Y |  | 50 |
| 187 | `insert` | Y | Y |  |  | Y |  | Y |  | 51 |
| 188 | `contains` | Y | Y |  |  | Y |  | Y |  | 52 |
| 189 | `size` | Y | Y |  |  | Y |  | Y |  | 53 |
| 190 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 54 |
| 191 | `height` | Y | Y |  |  | Y |  | Y |  | 55 |
| 192 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 67&#8209;76 |
| 193 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 159&#8209;162 |
| 194 | `find_node` |  |  |  | Y | Y |  |  | unknown | 184&#8209;189 |
| 195 | `min_node` |  |  |  | Y | Y |  | Y |  | 211&#8209;212 |
| 196 | `max_node` |  |  |  | Y | Y |  | Y |  | 223&#8209;224 |

### Chap37/BSTPlainStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 197 | `new` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;76 |
| 198 | `size` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 199 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;81 |
| 200 | `height` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;84 |
| 201 | `insert` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;91 |
| 202 | `contains` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 203 | `find` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |
| 204 | `delete` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;106 |
| 205 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;112 |
| 206 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;118 |
| 207 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 167&#8209;174 |
| 208 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 296&#8209;299 |
| 209 | `find_node` |  |  |  | Y | Y |  |  | unknown | 329&#8209;334 |
| 210 | `min_node` |  |  |  | Y | Y |  |  | unknown | 364&#8209;370 |
| 211 | `max_node` |  |  |  | Y | Y |  |  | unknown | 384&#8209;390 |
| 212 | `delete_min_node` |  |  |  | Y | Y |  |  | unknown | 405&#8209;416 |
| 213 | `delete_node` |  |  |  | Y | Y |  |  | unknown | 523&#8209;530 |

### Chap37/BSTRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 214 | `new_rb_link_lock` |  |  |  | Y | Y |  |  | hole | 333 |
| 215 | `new_node` |  |  |  | Y |  | Y | Y |  | 34&#8209;42 |
| 216 | `is_red` |  |  |  | Y |  | Y | Y |  | 44 |
| 217 | `size_link` |  |  |  | Y |  | Y | Y |  | 46 |
| 218 | `update` |  |  |  | Y |  | Y | Y |  | 48 |
| 219 | `rotate_left` |  |  |  | Y |  | Y | Y |  | 50&#8209;67 |
| 220 | `rotate_right` |  |  |  | Y |  | Y | Y |  | 69&#8209;86 |
| 221 | `flip_colors` |  |  |  | Y |  | Y | Y |  | 88&#8209;107 |
| 222 | `fix_up` |  |  |  | Y |  | Y | Y |  | 109&#8209;143 |
| 223 | `insert_link` |  |  |  | Y |  | Y | Y |  | 145&#8209;159 |
| 224 | `find_link` |  |  |  | Y |  | Y | Y |  | 161&#8209;174 |
| 225 | `min_link` |  |  |  | Y |  | Y | Y |  | 176&#8209;184 |
| 226 | `max_link` |  |  |  | Y |  | Y | Y |  | 186&#8209;194 |
| 227 | `in_order_collect` |  |  |  | Y |  | Y | Y |  | 196&#8209;202 |
| 228 | `pre_order_collect` |  |  |  | Y |  | Y | Y |  | 204&#8209;210 |
| 229 | `in_order_parallel` |  |  |  | Y |  | Y | Y |  | 214&#8209;229 |
| 230 | `pre_order_parallel` |  |  |  | Y |  | Y | Y |  | 231&#8209;246 |
| 231 | `build_balanced` |  |  |  | Y |  | Y | Y |  | 248&#8209;267 |
| 232 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 269&#8209;294 |
| 233 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 296&#8209;318 |
| 234 | `new` | Y | Y |  |  |  | Y | Y |  | 353&#8209;354 |
| 235 | `from_sorted_slice` | Y | Y |  |  |  | Y | Y |  | 355&#8209;356 |
| 236 | `insert` | Y | Y |  |  |  | Y | Y |  | 357&#8209;358 |
| 237 | `find` | Y | Y |  |  |  | Y | Y |  | 359&#8209;360 |
| 238 | `contains` | Y | Y |  |  |  | Y | Y |  | 361 |
| 239 | `size` | Y | Y |  |  |  | Y | Y |  | 362 |
| 240 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 363 |
| 241 | `height` | Y | Y |  |  |  | Y | Y |  | 364 |
| 242 | `minimum` | Y | Y |  |  |  | Y | Y |  | 365 |
| 243 | `maximum` | Y | Y |  |  |  | Y | Y |  | 366 |
| 244 | `in_order` | Y | Y |  |  |  | Y | Y |  | 367&#8209;368 |
| 245 | `pre_order` | Y | Y |  |  |  | Y | Y |  | 369&#8209;370 |
| 246 | `filter` | Y | Y |  |  |  | Y | Y |  | 371&#8209;374 |
| 247 | `reduce` | Y | Y |  |  |  | Y | Y |  | 375&#8209;378 |
| 248 | `height_rec` |  | Y |  |  |  | Y | Y |  | 416&#8209;421 |
| 249 | `default` |  | Y |  |  |  | Y | Y |  | 486 |

### Chap37/BSTRBStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 250 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 41&#8209;73 |
| 251 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 87&#8209;93 |
| 252 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 185&#8209;191 |
| 253 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 282&#8209;289 |
| 254 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 411&#8209;414 |
| 255 | `find_node` |  |  |  | Y | Y |  |  | unknown | 444&#8209;449 |
| 256 | `min_node` |  |  |  | Y | Y |  | Y |  | 479&#8209;480 |
| 257 | `max_node` |  |  |  | Y | Y |  | Y |  | 494&#8209;495 |
| 258 | `rb_new` |  |  |  | Y | Y |  |  | unknown | 509&#8209;512 |
| 259 | `rb_size` |  |  |  | Y | Y |  |  | unknown | 517&#8209;519 |
| 260 | `rb_is_empty` |  |  |  | Y | Y |  |  | unknown | 524&#8209;525 |
| 261 | `rb_height` |  |  |  | Y | Y |  |  | unknown | 530&#8209;532 |
| 262 | `rb_insert` |  |  |  | Y | Y |  |  | unknown | 537&#8209;543 |
| 263 | `rb_contains` |  |  |  | Y | Y |  |  | unknown | 548&#8209;550 |
| 264 | `rb_find` |  |  |  | Y | Y |  |  | unknown | 555&#8209;559 |

### Chap37/BSTSetAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 265 | `values_vec` |  |  |  | Y | Y |  | Y |  | 24 |
| 266 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 26 |
| 267 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 34&#8209;36 |
| 268 | `empty` | Y | Y |  |  | Y |  | Y |  | 47 |
| 269 | `singleton` | Y | Y |  |  | Y |  | Y |  | 49 |
| 270 | `size` | Y | Y |  |  | Y |  | Y |  | 51 |
| 271 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 53 |
| 272 | `find` | Y | Y |  |  | Y |  | Y |  | 55 |
| 273 | `contains` | Y | Y |  |  | Y |  | Y |  | 57 |
| 274 | `minimum` | Y | Y |  |  | Y |  | Y |  | 59 |
| 275 | `maximum` | Y | Y |  |  | Y |  | Y |  | 61 |
| 276 | `insert` | Y | Y |  |  | Y |  | Y |  | 63 |
| 277 | `delete` | Y | Y |  |  | Y |  | Y |  | 65 |
| 278 | `union` | Y | Y |  |  | Y |  | Y |  | 67 |
| 279 | `intersection` | Y | Y |  |  | Y |  | Y |  | 69 |
| 280 | `difference` | Y | Y |  |  | Y |  | Y |  | 71 |
| 281 | `split` | Y | Y |  |  | Y |  | Y |  | 73 |
| 282 | `join_pair` | Y | Y |  |  | Y |  | Y |  | 75 |
| 283 | `join_m` | Y | Y |  |  | Y |  | Y |  | 77 |
| 284 | `filter` | Y | Y |  |  | Y |  | Y |  | 79 |
| 285 | `reduce` | Y | Y |  |  | Y |  | Y |  | 81 |
| 286 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 83 |
| 287 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 85 |

### Chap37/BSTSetBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 288 | `empty` | Y | Y |  |  | Y |  | Y |  | 26 |
| 289 | `singleton` | Y | Y |  |  | Y |  | Y |  | 28 |
| 290 | `size` | Y | Y |  |  | Y |  | Y |  | 30 |
| 291 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 32 |
| 292 | `find` | Y | Y |  |  | Y |  | Y |  | 34 |
| 293 | `contains` | Y | Y |  |  | Y |  | Y |  | 36 |
| 294 | `minimum` | Y | Y |  |  | Y |  | Y |  | 38 |
| 295 | `maximum` | Y | Y |  |  | Y |  | Y |  | 40 |
| 296 | `insert` | Y | Y |  |  | Y |  | Y |  | 42 |
| 297 | `delete` | Y | Y |  |  | Y |  | Y |  | 44 |
| 298 | `union` | Y | Y |  |  | Y |  | Y |  | 46 |
| 299 | `intersection` | Y | Y |  |  | Y |  | Y |  | 48 |
| 300 | `difference` | Y | Y |  |  | Y |  | Y |  | 50 |
| 301 | `split` | Y | Y |  |  | Y |  | Y |  | 52 |
| 302 | `join_pair` | Y | Y |  |  | Y |  | Y |  | 54 |
| 303 | `join_m` | Y | Y |  |  | Y |  | Y |  | 56 |
| 304 | `filter` | Y | Y |  |  | Y |  | Y |  | 58 |
| 305 | `reduce` | Y | Y |  |  | Y |  | Y |  | 60 |
| 306 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 62 |
| 307 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 64 |
| 308 | `values_vec` |  |  |  | Y | Y |  | Y |  | 67 |
| 309 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 70 |
| 310 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 75 |

### Chap37/BSTSetPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 311 | `empty` | Y | Y |  |  | Y |  | Y |  | 26 |
| 312 | `singleton` | Y | Y |  |  | Y |  | Y |  | 28 |
| 313 | `size` | Y | Y |  |  | Y |  | Y |  | 30 |
| 314 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 32 |
| 315 | `find` | Y | Y |  |  | Y |  | Y |  | 34 |
| 316 | `contains` | Y | Y |  |  | Y |  | Y |  | 36 |
| 317 | `minimum` | Y | Y |  |  | Y |  | Y |  | 38 |
| 318 | `maximum` | Y | Y |  |  | Y |  | Y |  | 40 |
| 319 | `insert` | Y | Y |  |  | Y |  | Y |  | 42 |
| 320 | `delete` | Y | Y |  |  | Y |  | Y |  | 44 |
| 321 | `union` | Y | Y |  |  | Y |  | Y |  | 46 |
| 322 | `intersection` | Y | Y |  |  | Y |  | Y |  | 48 |
| 323 | `difference` | Y | Y |  |  | Y |  | Y |  | 50 |
| 324 | `split` | Y | Y |  |  | Y |  | Y |  | 52 |
| 325 | `join_pair` | Y | Y |  |  | Y |  | Y |  | 54 |
| 326 | `join_m` | Y | Y |  |  | Y |  | Y |  | 56 |
| 327 | `filter` | Y | Y |  |  | Y |  | Y |  | 58 |
| 328 | `reduce` | Y | Y |  |  | Y |  | Y |  | 60 |
| 329 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 62 |
| 330 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 64 |
| 331 | `values_vec` |  |  |  | Y | Y |  | Y |  | 67 |
| 332 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 70 |
| 333 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 75 |

### Chap37/BSTSetRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 334 | `empty` | Y | Y |  |  | Y |  | Y |  | 26 |
| 335 | `singleton` | Y | Y |  |  | Y |  | Y |  | 28 |
| 336 | `size` | Y | Y |  |  | Y |  | Y |  | 30 |
| 337 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 32 |
| 338 | `find` | Y | Y |  |  | Y |  | Y |  | 34 |
| 339 | `contains` | Y | Y |  |  | Y |  | Y |  | 36 |
| 340 | `minimum` | Y | Y |  |  | Y |  | Y |  | 38 |
| 341 | `maximum` | Y | Y |  |  | Y |  | Y |  | 40 |
| 342 | `insert` | Y | Y |  |  | Y |  | Y |  | 42 |
| 343 | `delete` | Y | Y |  |  | Y |  | Y |  | 44 |
| 344 | `union` | Y | Y |  |  | Y |  | Y |  | 46 |
| 345 | `intersection` | Y | Y |  |  | Y |  | Y |  | 48 |
| 346 | `difference` | Y | Y |  |  | Y |  | Y |  | 50 |
| 347 | `split` | Y | Y |  |  | Y |  | Y |  | 52 |
| 348 | `join_pair` | Y | Y |  |  | Y |  | Y |  | 54 |
| 349 | `join_m` | Y | Y |  |  | Y |  | Y |  | 56 |
| 350 | `filter` | Y | Y |  |  | Y |  | Y |  | 58 |
| 351 | `reduce` | Y | Y |  |  | Y |  | Y |  | 60 |
| 352 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 62 |
| 353 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 64 |
| 354 | `values_vec` |  |  |  | Y | Y |  | Y |  | 67 |
| 355 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 70 |

### Chap37/BSTSetSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 356 | `empty` | Y | Y |  |  | Y |  | Y |  | 26 |
| 357 | `singleton` | Y | Y |  |  | Y |  | Y |  | 28 |
| 358 | `size` | Y | Y |  |  | Y |  | Y |  | 30 |
| 359 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 32 |
| 360 | `find` | Y | Y |  |  | Y |  | Y |  | 34 |
| 361 | `contains` | Y | Y |  |  | Y |  | Y |  | 36 |
| 362 | `minimum` | Y | Y |  |  | Y |  | Y |  | 38 |
| 363 | `maximum` | Y | Y |  |  | Y |  | Y |  | 40 |
| 364 | `insert` | Y | Y |  |  | Y |  | Y |  | 42 |
| 365 | `delete` | Y | Y |  |  | Y |  | Y |  | 44 |
| 366 | `union` | Y | Y |  |  | Y |  | Y |  | 46 |
| 367 | `intersection` | Y | Y |  |  | Y |  | Y |  | 48 |
| 368 | `difference` | Y | Y |  |  | Y |  | Y |  | 50 |
| 369 | `split` | Y | Y |  |  | Y |  | Y |  | 52 |
| 370 | `join_pair` | Y | Y |  |  | Y |  | Y |  | 54 |
| 371 | `join_m` | Y | Y |  |  | Y |  | Y |  | 56 |
| 372 | `filter` | Y | Y |  |  | Y |  | Y |  | 58 |
| 373 | `reduce` | Y | Y |  |  | Y |  | Y |  | 60 |
| 374 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 62 |
| 375 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 64 |
| 376 | `values_vec` |  |  |  | Y | Y |  | Y |  | 67 |
| 377 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 71 |

### Chap37/BSTSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 378 | `new_splay_link_lock` |  |  |  | Y | Y |  |  | hole | 355 |
| 379 | `new_node` |  |  |  | Y |  | Y | Y |  | 27&#8209;34 |
| 380 | `size_link` |  |  |  | Y |  | Y | Y |  | 36 |
| 381 | `update` |  |  |  | Y |  | Y | Y |  | 38 |
| 382 | `splay` |  |  |  | Y |  | Y | Y |  | 40&#8209;151 |
| 383 | `bst_insert` |  |  |  | Y |  | Y | Y |  | 153&#8209;171 |
| 384 | `insert_link` |  |  |  | Y |  | Y | Y |  | 173&#8209;182 |
| 385 | `find_link` |  |  |  | Y |  | Y | Y |  | 184&#8209;197 |
| 386 | `min_link` |  |  |  | Y |  | Y | Y |  | 199&#8209;207 |
| 387 | `max_link` |  |  |  | Y |  | Y | Y |  | 209&#8209;217 |
| 388 | `in_order_collect` |  |  |  | Y |  | Y | Y |  | 219&#8209;225 |
| 389 | `pre_order_collect` |  |  |  | Y |  | Y | Y |  | 227&#8209;233 |
| 390 | `in_order_parallel` |  |  |  | Y |  | Y | Y |  | 237&#8209;252 |
| 391 | `pre_order_parallel` |  |  |  | Y |  | Y | Y |  | 254&#8209;269 |
| 392 | `build_balanced` |  |  |  | Y |  | Y | Y |  | 271&#8209;289 |
| 393 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 291&#8209;316 |
| 394 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 318&#8209;340 |
| 395 | `new` | Y | Y |  |  |  | Y | Y |  | 375&#8209;376 |
| 396 | `from_sorted_slice` | Y | Y |  |  |  | Y | Y |  | 377&#8209;378 |
| 397 | `insert` | Y | Y |  |  |  | Y | Y |  | 379&#8209;380 |
| 398 | `find` | Y | Y |  |  |  | Y | Y |  | 381&#8209;382 |
| 399 | `contains` | Y | Y |  |  |  | Y | Y |  | 383&#8209;384 |
| 400 | `size` | Y | Y |  |  |  | Y | Y |  | 385&#8209;386 |
| 401 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 387&#8209;388 |
| 402 | `height` | Y | Y |  |  |  | Y | Y |  | 389&#8209;390 |
| 403 | `minimum` | Y | Y |  |  |  | Y | Y |  | 391&#8209;392 |
| 404 | `maximum` | Y | Y |  |  |  | Y | Y |  | 393&#8209;394 |
| 405 | `in_order` | Y | Y |  |  |  | Y | Y |  | 395&#8209;396 |
| 406 | `pre_order` | Y | Y |  |  |  | Y | Y |  | 397&#8209;398 |
| 407 | `filter` | Y | Y |  |  |  | Y | Y |  | 399&#8209;402 |
| 408 | `reduce` | Y | Y |  |  |  | Y | Y |  | 403&#8209;406 |
| 409 | `height_rec` |  | Y |  |  |  | Y | Y |  | 441&#8209;446 |
| 410 | `default` |  | Y |  |  |  | Y | Y |  | 511 |

### Chap37/BSTSplayStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 411 | `new_node` |  |  |  | Y | Y |  | Y |  | 38 |
| 412 | `size_link` |  |  |  | Y | Y |  |  | unknown | 67&#8209;68 |
| 413 | `height_link` |  |  |  | Y | Y |  |  | unknown | 77&#8209;80 |
| 414 | `update` |  |  |  | Y | Y |  |  | hole | 94&#8209;99 |
| 415 | `splay` |  |  |  | Y | Y |  | Y |  | 110&#8209;111 |
| 416 | `bst_insert` |  |  |  | Y | Y |  | Y |  | 231&#8209;232 |
| 417 | `insert_link` |  |  |  | Y | Y |  | Y |  | 255 |
| 418 | `find_link` |  |  |  | Y | Y |  | Y |  | 266&#8209;267 |
| 419 | `min_link` |  |  |  | Y | Y |  | Y |  | 283&#8209;284 |
| 420 | `max_link` |  |  |  | Y | Y |  | Y |  | 295&#8209;296 |
| 421 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 307&#8209;308 |
| 422 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 317&#8209;318 |
| 423 | `new` | Y | Y |  |  | Y |  | Y |  | 346&#8209;348 |
| 424 | `size` | Y | Y |  |  | Y |  | Y |  | 350 |
| 425 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 352 |
| 426 | `height` | Y | Y |  |  | Y |  |  | unknown | 354&#8209;355 |
| 427 | `insert` | Y | Y |  |  | Y |  | Y |  | 357 |
| 428 | `find` | Y | Y |  |  | Y |  | Y |  | 359 |
| 429 | `contains` | Y | Y |  |  | Y |  | Y |  | 361 |
| 430 | `minimum` | Y | Y |  |  | Y |  | Y |  | 363 |
| 431 | `maximum` | Y | Y |  |  | Y |  | Y |  | 365 |
| 432 | `in_order` | Y | Y |  |  | Y |  | Y |  | 367 |
| 433 | `pre_order` | Y | Y |  |  | Y |  | Y |  | 369 |
| 434 | `default` |  | Y |  |  | Y |  | Y |  | 410 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
