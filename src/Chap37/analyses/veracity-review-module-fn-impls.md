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
| 3 | Chap37 | AVLTreeSeqStEph | 18 | 21 | 0 | 12 | 30 | 3 | 23 | 7 | 3 |
| 4 | Chap37 | AVLTreeSeqStPer | 13 | 16 | 0 | 14 | 27 | 3 | 11 | 14 | 5 |
| 5 | Chap37 | BSTAVLMtEph | 6 | 6 | 0 | 8 | 14 | 0 | 6 | 0 | 8 |
| 6 | Chap37 | BSTAVLStEph | 7 | 7 | 0 | 10 | 17 | 0 | 17 | 0 | 0 |
| 7 | Chap37 | BSTBBAlphaMtEph | 6 | 6 | 0 | 5 | 11 | 0 | 3 | 0 | 8 |
| 8 | Chap37 | BSTBBAlphaStEph | 10 | 10 | 0 | 7 | 17 | 0 | 17 | 0 | 0 |
| 9 | Chap37 | BSTPlainMtEph | 6 | 6 | 0 | 5 | 11 | 0 | 3 | 0 | 8 |
| 10 | Chap37 | BSTPlainStEph | 10 | 10 | 0 | 7 | 17 | 0 | 17 | 0 | 0 |
| 11 | Chap37 | BSTRBMtEph | 14 | 16 | 0 | 20 | 34 | 2 | 0 | 2 | 34 |
| 12 | Chap37 | BSTRBStEph | 7 | 7 | 0 | 8 | 15 | 0 | 15 | 0 | 0 |
| 13 | Chap37 | BSTSetAVLMtEph | 20 | 20 | 0 | 3 | 23 | 0 | 0 | 0 | 23 |
| 14 | Chap37 | BSTSetBBAlphaMtEph | 20 | 20 | 0 | 3 | 23 | 0 | 0 | 0 | 23 |
| 15 | Chap37 | BSTSetPlainMtEph | 20 | 20 | 0 | 3 | 23 | 0 | 0 | 0 | 23 |
| 16 | Chap37 | BSTSetRBMtEph | 20 | 20 | 0 | 2 | 22 | 0 | 0 | 0 | 22 |
| 17 | Chap37 | BSTSetSplayMtEph | 20 | 20 | 0 | 2 | 22 | 0 | 0 | 0 | 22 |
| 18 | Chap37 | BSTSplayMtEph | 14 | 16 | 0 | 17 | 31 | 2 | 0 | 2 | 31 |
| 19 | Chap37 | BSTSplayStEph | 11 | 12 | 0 | 12 | 24 | 0 | 13 | 1 | 10 |

## Function-by-Function Detail

### Chap37/AVLTreeSeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 162&#8209;165 |
| 2 | `lemma_inorder_values_maps_to_inorder` |  |  |  | Y | Y |  |  | unknown | 176&#8209;178 |
| 3 | `empty` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;199 |
| 4 | `new` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;202 |
| 5 | `length` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;206 |
| 6 | `nth` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;210 |
| 7 | `set` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;217 |
| 8 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;223 |
| 9 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;227 |
| 10 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;231 |
| 11 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;236 |
| 12 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;239 |
| 13 | `update` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;247 |
| 14 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;255 |
| 15 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;262 |
| 16 | `iter` | Y | Y |  |  | Y |  |  | unknown | 264&#8209;269 |
| 17 | `push_back` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;273 |
| 18 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;278 |
| 19 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;282 |
| 20 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;295 |
| 21 | `is_tree_empty` | Y | Y |  |  | Y |  |  | unknown | 297&#8209;299 |
| 22 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;303 |
| 23 | `cached_height` |  |  |  | Y | Y |  |  | unknown | 311&#8209;312 |
| 24 | `cached_size` |  |  |  | Y | Y |  |  | unknown | 320&#8209;322 |
| 25 | `update_size_height` |  |  |  | Y | Y |  |  | unknown | 332&#8209;353 |
| 26 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 362&#8209;372 |
| 27 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 421&#8209;431 |
| 28 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 473&#8209;488 |
| 29 | `insert_at_link` |  |  |  | Y | Y |  |  | hole | 527&#8209;538 |
| 30 | `nth_link` |  |  |  | Y | Y |  |  | unknown | 607&#8209;610 |
| 31 | `set_link` |  |  |  | Y | Y |  |  | unknown | 625&#8209;635 |
| 32 | `push_inorder` |  |  |  | Y | Y |  |  | unknown | 652&#8209;657 |
| 33 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 687&#8209;692 |
| 34 | `next` |  | Y |  |  | Y |  |  | hole | 1115&#8209;1131 |
| 35 | `eq` |  | Y |  |  | Y |  |  | hole | 1220&#8209;1221 |
| 36 | `default` |  | Y |  |  |  | Y | Y |  | 1258 |

### Chap37/AVLTreeSeqMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 37 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 160&#8209;163 |
| 38 | `empty` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;184 |
| 39 | `new` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;187 |
| 40 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;190 |
| 41 | `length` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;194 |
| 42 | `nth` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;198 |
| 43 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;202 |
| 44 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;206 |
| 45 | `set` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;209 |
| 46 | `subseq_copy` | Y | Y |  |  | Y |  |  | hole | 211&#8209;212 |
| 47 | `from_vec` | Y | Y |  |  | Y |  | Y |  | 214 |
| 48 | `values_in_order` | Y | Y |  |  | Y |  |  | hole | 216 |
| 49 | `height_fn` |  |  |  | Y | Y |  |  | unknown | 224&#8209;225 |
| 50 | `size_fn` |  |  |  | Y | Y |  |  | unknown | 233&#8209;234 |
| 51 | `mk` |  |  |  | Y | Y |  |  | hole | 243&#8209;252 |
| 52 | `rotate_right` |  |  |  | Y | Y |  |  | hole | 262&#8209;266 |
| 53 | `rotate_left` |  |  |  | Y | Y |  |  | hole | 275&#8209;279 |
| 54 | `rebalance` |  |  |  | Y | Y |  |  | hole | 288&#8209;292 |
| 55 | `nth_ref` |  |  |  | Y | Y |  |  | hole | 316&#8209;318 |
| 56 | `set_rec` |  |  |  | Y | Y |  |  | hole | 337&#8209;339 |
| 57 | `inorder_collect` |  |  |  | Y | Y |  |  | hole | 365 |
| 58 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | hole | 374&#8209;375 |
| 59 | `rec` |  |  |  | Y | Y |  | Y |  | 377 |
| 60 | `compare_trees` |  |  |  | Y | Y |  |  | hole | 392 |
| 61 | `eq` |  | Y |  |  | Y |  |  | hole | 522&#8209;523 |
| 62 | `default` |  | Y |  |  |  | Y | Y |  | 547 |
| 63 | `next` |  | Y |  |  |  | Y | Y |  | 611&#8209;619 |

### Chap37/AVLTreeSeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 64 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 160&#8209;163 |
| 65 | `empty` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;184 |
| 66 | `new` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;187 |
| 67 | `length` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;191 |
| 68 | `nth` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;195 |
| 69 | `set` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;198 |
| 70 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;201 |
| 71 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;205 |
| 72 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;209 |
| 73 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;212 |
| 74 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;215 |
| 75 | `update` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;220 |
| 76 | `from_vec` | Y | Y |  |  | Y |  |  | hole | 222 |
| 77 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 224&#8209;225 |
| 78 | `iter` | Y | Y |  |  | Y |  |  | hole | 227 |
| 79 | `push_back` | Y | Y |  |  | Y |  |  | hole | 229&#8209;230 |
| 80 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;233 |
| 81 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;236 |
| 82 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;239 |
| 83 | `h_fn` |  |  |  | Y | Y |  |  | unknown | 247&#8209;248 |
| 84 | `size_link_fn` |  |  |  | Y | Y |  |  | hole | 256&#8209;257 |
| 85 | `update_meta` |  |  |  | Y | Y |  |  | hole | 268&#8209;281 |
| 86 | `rotate_right_fn` |  |  |  | Y | Y |  |  | unknown | 291&#8209;298 |
| 87 | `rotate_left_fn` |  |  |  | Y | Y |  |  | unknown | 329&#8209;336 |
| 88 | `rebalance_fn` |  |  |  | Y | Y |  |  | unknown | 367&#8209;375 |
| 89 | `insert_at_link` |  |  |  | Y | Y |  |  | unknown | 424&#8209;434 |
| 90 | `nth_link` |  |  |  | Y | Y |  |  | unknown | 476&#8209;479 |
| 91 | `set_link` |  |  |  | Y | Y |  |  | unknown | 494&#8209;503 |
| 92 | `compare_trees` |  |  |  | Y | Y |  |  | hole | 530 |
| 93 | `eq` |  | Y |  |  | Y |  |  | hole | 801&#8209;802 |
| 94 | `default` |  | Y |  |  |  | Y | Y |  | 827 |
| 95 | `push_left_iter` |  |  |  | Y |  | Y | Y |  | 884&#8209;890 |
| 96 | `next` |  | Y |  |  |  | Y | Y |  | 894&#8209;899 |

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
| 125 | `push_left_iter` |  |  |  | Y |  | Y | Y |  | 566&#8209;571 |
| 126 | `next` |  | Y |  |  |  | Y | Y |  | 575&#8209;584 |

### Chap37/BSTAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 127 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 72&#8209;104 |
| 128 | `new` | Y | Y |  |  | Y |  | Y |  | 114 |
| 129 | `insert` | Y | Y |  |  | Y |  | Y |  | 115 |
| 130 | `contains` | Y | Y |  |  | Y |  | Y |  | 116 |
| 131 | `size` | Y | Y |  |  | Y |  | Y |  | 117 |
| 132 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 118 |
| 133 | `height` | Y | Y |  |  | Y |  | Y |  | 119 |
| 134 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 137&#8209;141 |
| 135 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 203&#8209;207 |
| 136 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 270&#8209;279 |
| 137 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 362&#8209;365 |
| 138 | `find_node` |  |  |  | Y | Y |  |  | unknown | 387&#8209;392 |
| 139 | `min_node` |  |  |  | Y | Y |  | Y |  | 414&#8209;415 |
| 140 | `max_node` |  |  |  | Y | Y |  | Y |  | 426&#8209;427 |

### Chap37/BSTAVLStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 141 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 67&#8209;99 |
| 142 | `lemma_max_plus_one` |  |  |  | Y | Y |  |  | unknown | 106&#8209;108 |
| 143 | `new` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;120 |
| 144 | `size` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;123 |
| 145 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 146 | `height` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 147 | `insert` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;137 |
| 148 | `contains` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;140 |
| 149 | `find` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;145 |
| 150 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 150&#8209;188 |
| 151 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 302&#8209;340 |
| 152 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 461&#8209;489 |
| 153 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 747&#8209;758 |
| 154 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 901&#8209;904 |
| 155 | `find_node` |  |  |  | Y | Y |  |  | unknown | 934&#8209;939 |
| 156 | `min_node` |  |  |  | Y | Y |  |  | unknown | 969&#8209;975 |
| 157 | `max_node` |  |  |  | Y | Y |  |  | unknown | 989&#8209;995 |

### Chap37/BSTBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 158 | `new` | Y | Y |  |  | Y |  | Y |  | 73 |
| 159 | `insert` | Y | Y |  |  | Y |  | Y |  | 74 |
| 160 | `contains` | Y | Y |  |  | Y |  | Y |  | 75 |
| 161 | `size` | Y | Y |  |  | Y |  | Y |  | 76 |
| 162 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 77 |
| 163 | `height` | Y | Y |  |  | Y |  | Y |  | 78 |
| 164 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 96&#8209;105 |
| 165 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 188&#8209;191 |
| 166 | `find_node` |  |  |  | Y | Y |  |  | unknown | 213&#8209;218 |
| 167 | `min_node` |  |  |  | Y | Y |  | Y |  | 240&#8209;241 |
| 168 | `max_node` |  |  |  | Y | Y |  | Y |  | 252&#8209;253 |

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
| 186 | `new` | Y | Y |  |  | Y |  | Y |  | 68 |
| 187 | `insert` | Y | Y |  |  | Y |  | Y |  | 69 |
| 188 | `contains` | Y | Y |  |  | Y |  | Y |  | 70 |
| 189 | `size` | Y | Y |  |  | Y |  | Y |  | 71 |
| 190 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 72 |
| 191 | `height` | Y | Y |  |  | Y |  | Y |  | 73 |
| 192 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 91&#8209;100 |
| 193 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 183&#8209;186 |
| 194 | `find_node` |  |  |  | Y | Y |  |  | unknown | 208&#8209;213 |
| 195 | `min_node` |  |  |  | Y | Y |  | Y |  | 235&#8209;236 |
| 196 | `max_node` |  |  |  | Y | Y |  | Y |  | 247&#8209;248 |

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
| 214 | `new` | Y | Y |  |  | Y |  | Y |  | 73 |
| 215 | `from_sorted_slice` | Y | Y |  |  | Y |  | Y |  | 74 |
| 216 | `insert` | Y | Y |  |  | Y |  | Y |  | 75 |
| 217 | `find` | Y | Y |  |  | Y |  | Y |  | 76 |
| 218 | `contains` | Y | Y |  |  | Y |  | Y |  | 77 |
| 219 | `size` | Y | Y |  |  | Y |  | Y |  | 78 |
| 220 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 79 |
| 221 | `height` | Y | Y |  |  | Y |  | Y |  | 80 |
| 222 | `minimum` | Y | Y |  |  | Y |  | Y |  | 81 |
| 223 | `maximum` | Y | Y |  |  | Y |  | Y |  | 82 |
| 224 | `in_order` | Y | Y |  |  | Y |  | Y |  | 83 |
| 225 | `pre_order` | Y | Y |  |  | Y |  | Y |  | 84 |
| 226 | `filter` | Y | Y |  |  | Y |  | Y |  | 85&#8209;87 |
| 227 | `reduce` | Y | Y |  |  | Y |  | Y |  | 88&#8209;90 |
| 228 | `new_rb_link_lock` |  |  |  | Y | Y |  |  | hole | 103 |
| 229 | `new_node` |  |  |  | Y | Y |  | Y |  | 109 |
| 230 | `is_red` |  |  |  | Y | Y |  | Y |  | 119 |
| 231 | `size_link` |  |  |  | Y | Y |  | Y |  | 126 |
| 232 | `update` |  |  |  | Y | Y |  |  | hole | 133 |
| 233 | `rotate_left` |  |  |  | Y | Y |  | Y |  | 140 |
| 234 | `rotate_right` |  |  |  | Y | Y |  | Y |  | 159 |
| 235 | `flip_colors` |  |  |  | Y | Y |  | Y |  | 178 |
| 236 | `fix_up` |  |  |  | Y | Y |  | Y |  | 199 |
| 237 | `insert_link` |  |  |  | Y | Y |  | Y |  | 235&#8209;236 |
| 238 | `find_link` |  |  |  | Y | Y |  | Y |  | 253&#8209;254 |
| 239 | `min_link` |  |  |  | Y | Y |  | Y |  | 270&#8209;271 |
| 240 | `max_link` |  |  |  | Y | Y |  | Y |  | 282&#8209;283 |
| 241 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 294&#8209;295 |
| 242 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 304&#8209;305 |
| 243 | `in_order_parallel` |  |  |  | Y | Y |  | Y |  | 314&#8209;315 |
| 244 | `pre_order_parallel` |  |  |  | Y | Y |  | Y |  | 333&#8209;334 |
| 245 | `build_balanced` |  |  |  | Y | Y |  | Y |  | 352&#8209;353 |
| 246 | `filter_parallel` |  |  |  | Y | Y |  | Y |  | 374&#8209;377 |
| 247 | `reduce_parallel` |  |  |  | Y | Y |  | Y |  | 401&#8209;404 |
| 248 | `height_rec` |  | Y |  |  |  | Y | Y |  | 524&#8209;529 |
| 249 | `default` |  | Y |  |  |  | Y | Y |  | 594 |

### Chap37/BSTRBStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 250 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 51&#8209;83 |
| 251 | `new` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;97 |
| 252 | `size` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 253 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 254 | `height` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;105 |
| 255 | `insert` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;112 |
| 256 | `contains` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 257 | `find` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;120 |
| 258 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 126&#8209;132 |
| 259 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 224&#8209;230 |
| 260 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 321&#8209;328 |
| 261 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 450&#8209;453 |
| 262 | `find_node` |  |  |  | Y | Y |  |  | unknown | 483&#8209;488 |
| 263 | `min_node` |  |  |  | Y | Y |  |  | unknown | 518&#8209;524 |
| 264 | `max_node` |  |  |  | Y | Y |  |  | unknown | 538&#8209;544 |

### Chap37/BSTSetAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 265 | `empty` | Y | Y |  |  | Y |  | Y |  | 43 |
| 266 | `singleton` | Y | Y |  |  | Y |  | Y |  | 45 |
| 267 | `size` | Y | Y |  |  | Y |  | Y |  | 47 |
| 268 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 49 |
| 269 | `find` | Y | Y |  |  | Y |  | Y |  | 51 |
| 270 | `contains` | Y | Y |  |  | Y |  | Y |  | 53 |
| 271 | `minimum` | Y | Y |  |  | Y |  | Y |  | 55 |
| 272 | `maximum` | Y | Y |  |  | Y |  | Y |  | 57 |
| 273 | `insert` | Y | Y |  |  | Y |  | Y |  | 59 |
| 274 | `delete` | Y | Y |  |  | Y |  | Y |  | 61 |
| 275 | `union` | Y | Y |  |  | Y |  | Y |  | 63 |
| 276 | `intersection` | Y | Y |  |  | Y |  | Y |  | 65 |
| 277 | `difference` | Y | Y |  |  | Y |  | Y |  | 67 |
| 278 | `split` | Y | Y |  |  | Y |  | Y |  | 69 |
| 279 | `join_pair` | Y | Y |  |  | Y |  | Y |  | 71 |
| 280 | `join_m` | Y | Y |  |  | Y |  | Y |  | 73 |
| 281 | `filter` | Y | Y |  |  | Y |  | Y |  | 75 |
| 282 | `reduce` | Y | Y |  |  | Y |  | Y |  | 77 |
| 283 | `iter_in_order` | Y | Y |  |  | Y |  | Y |  | 79 |
| 284 | `as_tree` | Y | Y |  |  | Y |  | Y |  | 81 |
| 285 | `values_vec` |  |  |  | Y | Y |  | Y |  | 87 |
| 286 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 89 |
| 287 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 97&#8209;99 |

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
| 378 | `new` | Y | Y |  |  | Y |  | Y |  | 66 |
| 379 | `from_sorted_slice` | Y | Y |  |  | Y |  | Y |  | 67 |
| 380 | `insert` | Y | Y |  |  | Y |  | Y |  | 68 |
| 381 | `find` | Y | Y |  |  | Y |  | Y |  | 69 |
| 382 | `contains` | Y | Y |  |  | Y |  | Y |  | 70 |
| 383 | `size` | Y | Y |  |  | Y |  | Y |  | 71 |
| 384 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 72 |
| 385 | `height` | Y | Y |  |  | Y |  | Y |  | 73 |
| 386 | `minimum` | Y | Y |  |  | Y |  | Y |  | 74 |
| 387 | `maximum` | Y | Y |  |  | Y |  | Y |  | 75 |
| 388 | `in_order` | Y | Y |  |  | Y |  | Y |  | 76 |
| 389 | `pre_order` | Y | Y |  |  | Y |  | Y |  | 77 |
| 390 | `filter` | Y | Y |  |  | Y |  | Y |  | 78&#8209;80 |
| 391 | `reduce` | Y | Y |  |  | Y |  | Y |  | 81&#8209;83 |
| 392 | `new_splay_link_lock` |  |  |  | Y | Y |  |  | hole | 96 |
| 393 | `new_node` |  |  |  | Y | Y |  | Y |  | 102 |
| 394 | `size_link` |  |  |  | Y | Y |  | Y |  | 111 |
| 395 | `update` |  |  |  | Y | Y |  |  | hole | 118 |
| 396 | `splay` |  |  |  | Y | Y |  | Y |  | 127&#8209;128 |
| 397 | `bst_insert` |  |  |  | Y | Y |  | Y |  | 242&#8209;243 |
| 398 | `insert_link` |  |  |  | Y | Y |  | Y |  | 264 |
| 399 | `find_link` |  |  |  | Y | Y |  | Y |  | 275&#8209;276 |
| 400 | `min_link` |  |  |  | Y | Y |  | Y |  | 292&#8209;293 |
| 401 | `max_link` |  |  |  | Y | Y |  | Y |  | 304&#8209;305 |
| 402 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 316&#8209;317 |
| 403 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 326&#8209;327 |
| 404 | `in_order_parallel` |  |  |  | Y | Y |  | Y |  | 336&#8209;337 |
| 405 | `pre_order_parallel` |  |  |  | Y | Y |  | Y |  | 355&#8209;356 |
| 406 | `build_balanced` |  |  |  | Y | Y |  | Y |  | 374&#8209;375 |
| 407 | `filter_parallel` |  |  |  | Y | Y |  | Y |  | 395&#8209;398 |
| 408 | `reduce_parallel` |  |  |  | Y | Y |  | Y |  | 422&#8209;425 |
| 409 | `height_rec` |  | Y |  |  |  | Y | Y |  | 526&#8209;531 |
| 410 | `default` |  | Y |  |  |  | Y | Y |  | 596 |

### Chap37/BSTSplayStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 411 | `new` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;100 |
| 412 | `size` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 413 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 414 | `height` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 415 | `insert` | Y | Y |  |  | Y |  | Y |  | 108 |
| 416 | `find` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 417 | `contains` | Y | Y |  |  | Y |  | Y |  | 111 |
| 418 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;115 |
| 419 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;119 |
| 420 | `in_order` | Y | Y |  |  | Y |  | Y |  | 120 |
| 421 | `pre_order` | Y | Y |  |  | Y |  | Y |  | 121 |
| 422 | `new_node` |  |  |  | Y | Y |  |  | unknown | 127&#8209;132 |
| 423 | `size_link` |  |  |  | Y | Y |  |  | unknown | 142&#8209;143 |
| 424 | `height_link` |  |  |  | Y | Y |  |  | unknown | 152&#8209;155 |
| 425 | `update` |  |  |  | Y | Y |  |  | hole | 169&#8209;174 |
| 426 | `splay` |  |  |  | Y | Y |  | Y |  | 185&#8209;186 |
| 427 | `bst_insert` |  |  |  | Y | Y |  | Y |  | 306&#8209;307 |
| 428 | `insert_link` |  |  |  | Y | Y |  | Y |  | 328 |
| 429 | `find_link` |  |  |  | Y | Y |  |  | unknown | 339&#8209;342 |
| 430 | `min_link` |  |  |  | Y | Y |  |  | unknown | 356&#8209;360 |
| 431 | `max_link` |  |  |  | Y | Y |  |  | unknown | 371&#8209;375 |
| 432 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 386&#8209;387 |
| 433 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 396&#8209;397 |
| 434 | `default` |  | Y |  |  | Y |  | Y |  | 445 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
