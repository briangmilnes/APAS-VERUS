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
| 1 | Chap37 | AVLTreeSeq | 20 | 23 | 0 | 13 | 36 | 0 | 32 | 3 | 1 |
| 2 | Chap37 | AVLTreeSeqMtPer | 11 | 14 | 0 | 13 | 27 | 0 | 12 | 13 | 2 |
| 3 | Chap37 | AVLTreeSeqStEph | 18 | 21 | 0 | 12 | 33 | 0 | 24 | 8 | 1 |
| 4 | Chap37 | AVLTreeSeqStPer | 13 | 16 | 0 | 14 | 30 | 0 | 15 | 13 | 2 |
| 5 | Chap37 | BSTAVLMtEph | 6 | 6 | 0 | 8 | 14 | 0 | 12 | 0 | 2 |
| 6 | Chap37 | BSTAVLStEph | 7 | 7 | 0 | 10 | 17 | 0 | 17 | 0 | 0 |
| 7 | Chap37 | BSTBBAlphaMtEph | 6 | 6 | 0 | 5 | 11 | 0 | 9 | 0 | 2 |
| 8 | Chap37 | BSTBBAlphaStEph | 10 | 10 | 0 | 7 | 17 | 0 | 17 | 0 | 0 |
| 9 | Chap37 | BSTPlainMtEph | 6 | 6 | 0 | 5 | 11 | 0 | 9 | 0 | 2 |
| 10 | Chap37 | BSTPlainStEph | 10 | 10 | 0 | 7 | 17 | 0 | 17 | 0 | 0 |
| 11 | Chap37 | BSTRBMtEph | 14 | 15 | 0 | 21 | 36 | 0 | 14 | 1 | 21 |
| 12 | Chap37 | BSTRBStEph | 7 | 7 | 0 | 8 | 15 | 0 | 15 | 0 | 0 |
| 13 | Chap37 | BSTSetAVLMtEph | 21 | 22 | 0 | 3 | 25 | 0 | 21 | 1 | 3 |
| 14 | Chap37 | BSTSetBBAlphaMtEph | 21 | 22 | 0 | 3 | 25 | 0 | 21 | 1 | 3 |
| 15 | Chap37 | BSTSetPlainMtEph | 21 | 22 | 0 | 3 | 25 | 0 | 21 | 1 | 3 |
| 16 | Chap37 | BSTSetRBMtEph | 21 | 22 | 0 | 2 | 24 | 0 | 21 | 1 | 2 |
| 17 | Chap37 | BSTSetSplayMtEph | 21 | 22 | 0 | 2 | 24 | 0 | 21 | 1 | 2 |
| 18 | Chap37 | BSTSplayMtEph | 14 | 15 | 0 | 18 | 33 | 0 | 14 | 1 | 18 |
| 19 | Chap37 | BSTSplayStEph | 11 | 12 | 0 | 12 | 24 | 0 | 18 | 0 | 6 |

## Function-by-Function Detail

### Chap37/AVLTreeSeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 167&#8209;170 |
| 2 | `lemma_inorder_values_maps_to_inorder` |  |  |  | Y | Y |  |  | unknown | 181&#8209;183 |
| 3 | `empty` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;202 |
| 4 | `new` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;205 |
| 5 | `length` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;209 |
| 6 | `nth` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;213 |
| 7 | `set` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;220 |
| 8 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;226 |
| 9 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 228&#8209;230 |
| 10 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;234 |
| 11 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;239 |
| 12 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;242 |
| 13 | `update` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;250 |
| 14 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;258 |
| 15 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;265 |
| 16 | `iter` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;272 |
| 17 | `push_back` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;276 |
| 18 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 278&#8209;281 |
| 19 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;285 |
| 20 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 287&#8209;298 |
| 21 | `is_tree_empty` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;302 |
| 22 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 304&#8209;306 |
| 23 | `cached_height` |  |  |  | Y | Y |  |  | unknown | 314&#8209;315 |
| 24 | `cached_size` |  |  |  | Y | Y |  |  | unknown | 323&#8209;325 |
| 25 | `update_size_height` |  |  |  | Y | Y |  |  | unknown | 335&#8209;356 |
| 26 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 365&#8209;375 |
| 27 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 424&#8209;434 |
| 28 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 476&#8209;491 |
| 29 | `insert_at_link` |  |  |  | Y | Y |  |  | hole | 530&#8209;541 |
| 30 | `nth_link` |  |  |  | Y | Y |  |  | unknown | 610&#8209;613 |
| 31 | `set_link` |  |  |  | Y | Y |  |  | unknown | 628&#8209;638 |
| 32 | `push_inorder` |  |  |  | Y | Y |  |  | unknown | 655&#8209;660 |
| 33 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 690&#8209;695 |
| 34 | `next` |  | Y |  |  | Y |  |  | hole | 1098&#8209;1114 |
| 35 | `default` |  | Y |  |  | Y |  | Y |  | 1169 |
| 36 | `eq` |  | Y |  |  | Y |  |  | hole | 1192&#8209;1193 |

### Chap37/AVLTreeSeqMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 37 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 166&#8209;169 |
| 38 | `empty` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;190 |
| 39 | `new` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;193 |
| 40 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;196 |
| 41 | `length` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;200 |
| 42 | `nth` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;204 |
| 43 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;208 |
| 44 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;212 |
| 45 | `set` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;215 |
| 46 | `subseq_copy` | Y | Y |  |  | Y |  |  | hole | 217&#8209;218 |
| 47 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 220&#8209;221 |
| 48 | `values_in_order` | Y | Y |  |  | Y |  |  | hole | 223&#8209;224 |
| 49 | `height_fn` |  |  |  | Y | Y |  |  | unknown | 232&#8209;233 |
| 50 | `size_fn` |  |  |  | Y | Y |  |  | unknown | 241&#8209;242 |
| 51 | `mk` |  |  |  | Y | Y |  |  | hole | 251&#8209;260 |
| 52 | `rotate_right` |  |  |  | Y | Y |  |  | hole | 270&#8209;274 |
| 53 | `rotate_left` |  |  |  | Y | Y |  |  | hole | 283&#8209;287 |
| 54 | `rebalance` |  |  |  | Y | Y |  |  | hole | 296&#8209;300 |
| 55 | `nth_ref` |  |  |  | Y | Y |  |  | hole | 324&#8209;326 |
| 56 | `set_rec` |  |  |  | Y | Y |  |  | hole | 345&#8209;347 |
| 57 | `inorder_collect` |  |  |  | Y | Y |  |  | hole | 373 |
| 58 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | hole | 382&#8209;383 |
| 59 | `rec` |  |  |  | Y | Y |  | Y |  | 385 |
| 60 | `compare_trees` |  |  |  | Y | Y |  |  | hole | 400 |
| 61 | `default` |  | Y |  |  | Y |  | Y |  | 508 |
| 62 | `next` |  | Y |  |  | Y |  |  | hole | 516&#8209;517 |
| 63 | `eq` |  | Y |  |  | Y |  |  | hole | 553&#8209;554 |

### Chap37/AVLTreeSeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 64 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 178&#8209;181 |
| 65 | `empty` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;202 |
| 66 | `new` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;205 |
| 67 | `length` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;209 |
| 68 | `nth` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;213 |
| 69 | `set` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;216 |
| 70 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;219 |
| 71 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;223 |
| 72 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;227 |
| 73 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;231 |
| 74 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;234 |
| 75 | `update` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;239 |
| 76 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;247 |
| 77 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;254 |
| 78 | `iter` | Y | Y |  |  | Y |  |  | hole | 256&#8209;257 |
| 79 | `push_back` | Y | Y |  |  | Y |  |  | hole | 259&#8209;263 |
| 80 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;268 |
| 81 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 270&#8209;274 |
| 82 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 276&#8209;287 |
| 83 | `h_fn` |  |  |  | Y | Y |  |  | unknown | 295&#8209;296 |
| 84 | `size_link_fn` |  |  |  | Y | Y |  |  | hole | 304&#8209;305 |
| 85 | `update_meta` |  |  |  | Y | Y |  |  | hole | 316&#8209;329 |
| 86 | `rotate_right_fn` |  |  |  | Y | Y |  |  | unknown | 339&#8209;346 |
| 87 | `rotate_left_fn` |  |  |  | Y | Y |  |  | unknown | 377&#8209;384 |
| 88 | `rebalance_fn` |  |  |  | Y | Y |  |  | unknown | 415&#8209;423 |
| 89 | `insert_at_link` |  |  |  | Y | Y |  |  | unknown | 472&#8209;483 |
| 90 | `nth_link` |  |  |  | Y | Y |  |  | unknown | 536&#8209;539 |
| 91 | `set_link` |  |  |  | Y | Y |  |  | unknown | 554&#8209;563 |
| 92 | `compare_trees` |  |  |  | Y | Y |  |  | hole | 590 |
| 93 | `default` |  | Y |  |  | Y |  | Y |  | 932 |
| 94 | `push_left_iter` |  |  |  | Y | Y |  |  | hole | 938 |
| 95 | `next` |  | Y |  |  | Y |  |  | hole | 959&#8209;960 |
| 96 | `eq` |  | Y |  |  | Y |  |  | hole | 989&#8209;990 |

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
| 106 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;179 |
| 107 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;182 |
| 108 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;185 |
| 109 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;188 |
| 110 | `iter` | Y | Y |  |  | Y |  |  | hole | 190&#8209;191 |
| 111 | `height_fn` |  |  |  | Y | Y |  |  | unknown | 196&#8209;197 |
| 112 | `size_fn` |  |  |  | Y | Y |  |  | unknown | 205&#8209;206 |
| 113 | `mk` |  |  |  | Y | Y |  |  | hole | 215&#8209;224 |
| 114 | `rotate_right` |  |  |  | Y | Y |  |  | hole | 234&#8209;238 |
| 115 | `rotate_left` |  |  |  | Y | Y |  |  | hole | 247&#8209;251 |
| 116 | `rebalance` |  |  |  | Y | Y |  |  | hole | 260&#8209;264 |
| 117 | `nth_ref` |  |  |  | Y | Y |  |  | hole | 288&#8209;290 |
| 118 | `set_rec` |  |  |  | Y | Y |  |  | hole | 309&#8209;311 |
| 119 | `inorder_collect` |  |  |  | Y | Y |  |  | hole | 337 |
| 120 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | hole | 346&#8209;347 |
| 121 | `rec` |  |  |  | Y | Y |  | Y |  | 349 |
| 122 | `compare_trees` |  |  |  | Y | Y |  |  | hole | 362 |
| 123 | `default` |  | Y |  |  | Y |  | Y |  | 475 |
| 124 | `push_left_iter_stper` |  |  |  | Y | Y |  |  | hole | 489 |
| 125 | `next` |  | Y |  |  | Y |  |  | hole | 509&#8209;510 |
| 126 | `eq` |  | Y |  |  | Y |  |  | hole | 528&#8209;529 |

### Chap37/BSTAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 127 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 49&#8209;81 |
| 128 | `new` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 129 | `insert` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 130 | `contains` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 131 | `size` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |
| 132 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;100 |
| 133 | `height` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 134 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 117&#8209;121 |
| 135 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 183&#8209;187 |
| 136 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 250&#8209;259 |
| 137 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 342&#8209;345 |
| 138 | `find_node` |  |  |  | Y | Y |  |  | unknown | 367&#8209;372 |
| 139 | `min_node` |  |  |  | Y | Y |  | Y |  | 394&#8209;395 |
| 140 | `max_node` |  |  |  | Y | Y |  | Y |  | 406&#8209;407 |

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
| 158 | `new` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;50 |
| 159 | `insert` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;52 |
| 160 | `contains` | Y | Y |  |  | Y |  |  | unknown | 53&#8209;54 |
| 161 | `size` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;56 |
| 162 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;58 |
| 163 | `height` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 164 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 75&#8209;84 |
| 165 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 167&#8209;170 |
| 166 | `find_node` |  |  |  | Y | Y |  |  | unknown | 192&#8209;197 |
| 167 | `min_node` |  |  |  | Y | Y |  | Y |  | 219&#8209;220 |
| 168 | `max_node` |  |  |  | Y | Y |  | Y |  | 231&#8209;232 |

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
| 186 | `new` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;50 |
| 187 | `insert` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;52 |
| 188 | `contains` | Y | Y |  |  | Y |  |  | unknown | 53&#8209;54 |
| 189 | `size` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;56 |
| 190 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;58 |
| 191 | `height` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 192 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 75&#8209;84 |
| 193 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 167&#8209;170 |
| 194 | `find_node` |  |  |  | Y | Y |  |  | unknown | 192&#8209;197 |
| 195 | `min_node` |  |  |  | Y | Y |  | Y |  | 219&#8209;220 |
| 196 | `max_node` |  |  |  | Y | Y |  | Y |  | 231&#8209;232 |

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
| 214 | `new` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 215 | `from_sorted_slice` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;73 |
| 216 | `insert` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 217 | `find` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 218 | `contains` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 219 | `size` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;81 |
| 220 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;83 |
| 221 | `height` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 222 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;87 |
| 223 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;89 |
| 224 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;91 |
| 225 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 226 | `filter` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;97 |
| 227 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;101 |
| 228 | `new_rb_link_lock` |  |  |  | Y | Y |  |  | hole | 114 |
| 229 | `new_node` |  |  |  | Y | Y |  | Y |  | 118 |
| 230 | `is_red` |  |  |  | Y | Y |  | Y |  | 128 |
| 231 | `size_link` |  |  |  | Y | Y |  | Y |  | 135 |
| 232 | `update` |  |  |  | Y | Y |  | Y |  | 142 |
| 233 | `rotate_left` |  |  |  | Y | Y |  | Y |  | 150 |
| 234 | `rotate_right` |  |  |  | Y | Y |  | Y |  | 169 |
| 235 | `flip_colors` |  |  |  | Y | Y |  | Y |  | 188 |
| 236 | `fix_up` |  |  |  | Y | Y |  | Y |  | 209 |
| 237 | `insert_link` |  |  |  | Y | Y |  | Y |  | 245&#8209;246 |
| 238 | `find_link` |  |  |  | Y | Y |  | Y |  | 263&#8209;264 |
| 239 | `min_link` |  |  |  | Y | Y |  | Y |  | 280&#8209;281 |
| 240 | `max_link` |  |  |  | Y | Y |  | Y |  | 292&#8209;293 |
| 241 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 304&#8209;305 |
| 242 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 314&#8209;315 |
| 243 | `in_order_parallel` |  |  |  | Y | Y |  | Y |  | 324&#8209;325 |
| 244 | `pre_order_parallel` |  |  |  | Y | Y |  | Y |  | 343&#8209;344 |
| 245 | `build_balanced` |  |  |  | Y | Y |  | Y |  | 362&#8209;363 |
| 246 | `filter_parallel` |  |  |  | Y | Y |  | Y |  | 384&#8209;387 |
| 247 | `reduce_parallel` |  |  |  | Y | Y |  | Y |  | 411&#8209;414 |
| 248 | `height_rec` |  |  |  | Y | Y |  | Y |  | 435&#8209;436 |
| 249 | `default` |  | Y |  |  | Y |  | Y |  | 543 |

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
| 265 | `empty` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 266 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 267 | `size` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;81 |
| 268 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;83 |
| 269 | `find` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 270 | `contains` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;87 |
| 271 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;89 |
| 272 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;91 |
| 273 | `insert` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 274 | `delete` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;95 |
| 275 | `union` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;97 |
| 276 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 277 | `difference` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;101 |
| 278 | `split` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;103 |
| 279 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 280 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;107 |
| 281 | `filter` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 282 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 283 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 284 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 285 | `iter` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 286 | `values_vec` |  |  |  | Y | Y |  | Y |  | 123 |
| 287 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 125 |
| 288 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 133&#8209;135 |
| 289 | `next` |  | Y |  |  | Y |  |  | hole | 377&#8209;393 |

### Chap37/BSTSetBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 290 | `empty` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 291 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 292 | `size` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 293 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 294 | `find` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 295 | `contains` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 296 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 297 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 298 | `insert` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 299 | `delete` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 300 | `union` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 301 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 302 | `difference` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 303 | `split` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;86 |
| 304 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 305 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 306 | `filter` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 307 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 308 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 309 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |
| 310 | `iter` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;100 |
| 311 | `values_vec` |  |  |  | Y | Y |  | Y |  | 103 |
| 312 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 106 |
| 313 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 111 |
| 314 | `next` |  | Y |  |  | Y |  |  | hole | 263&#8209;279 |

### Chap37/BSTSetPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 315 | `empty` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 316 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 317 | `size` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 318 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 319 | `find` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 320 | `contains` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 321 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 322 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 323 | `insert` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 324 | `delete` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 325 | `union` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 326 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 327 | `difference` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 328 | `split` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;86 |
| 329 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 330 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 331 | `filter` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 332 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 333 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 334 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |
| 335 | `iter` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;100 |
| 336 | `values_vec` |  |  |  | Y | Y |  | Y |  | 103 |
| 337 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 106 |
| 338 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 111 |
| 339 | `next` |  | Y |  |  | Y |  |  | hole | 319&#8209;335 |

### Chap37/BSTSetRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 340 | `empty` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 341 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 342 | `size` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 343 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 344 | `find` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 345 | `contains` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 346 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 347 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 348 | `insert` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 349 | `delete` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 350 | `union` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 351 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 352 | `difference` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 353 | `split` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;86 |
| 354 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 355 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 356 | `filter` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 357 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 358 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 359 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |
| 360 | `iter` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;100 |
| 361 | `values_vec` |  |  |  | Y | Y |  | Y |  | 103 |
| 362 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 106 |
| 363 | `next` |  | Y |  |  | Y |  |  | hole | 347&#8209;363 |

### Chap37/BSTSetSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 364 | `empty` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 365 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 366 | `size` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 367 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 368 | `find` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 369 | `contains` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 370 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 371 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 372 | `insert` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 373 | `delete` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 374 | `union` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 375 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 376 | `difference` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 377 | `split` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;86 |
| 378 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 379 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 380 | `filter` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 381 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 382 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 383 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |
| 384 | `iter` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;100 |
| 385 | `values_vec` |  |  |  | Y | Y |  | Y |  | 103 |
| 386 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 107 |
| 387 | `next` |  | Y |  |  | Y |  |  | hole | 348&#8209;364 |

### Chap37/BSTSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 388 | `new` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 389 | `from_sorted_slice` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 390 | `insert` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 391 | `find` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 392 | `contains` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 393 | `size` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 394 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 395 | `height` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 396 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 397 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 398 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 399 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;86 |
| 400 | `filter` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;90 |
| 401 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;94 |
| 402 | `new_splay_link_lock` |  |  |  | Y | Y |  |  | hole | 107 |
| 403 | `new_node` |  |  |  | Y | Y |  | Y |  | 111 |
| 404 | `size_link` |  |  |  | Y | Y |  | Y |  | 120 |
| 405 | `update` |  |  |  | Y | Y |  | Y |  | 127 |
| 406 | `splay` |  |  |  | Y | Y |  | Y |  | 137&#8209;138 |
| 407 | `bst_insert` |  |  |  | Y | Y |  | Y |  | 252&#8209;253 |
| 408 | `insert_link` |  |  |  | Y | Y |  | Y |  | 274 |
| 409 | `find_link` |  |  |  | Y | Y |  | Y |  | 285&#8209;286 |
| 410 | `min_link` |  |  |  | Y | Y |  | Y |  | 302&#8209;303 |
| 411 | `max_link` |  |  |  | Y | Y |  | Y |  | 314&#8209;315 |
| 412 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 326&#8209;327 |
| 413 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 336&#8209;337 |
| 414 | `in_order_parallel` |  |  |  | Y | Y |  | Y |  | 346&#8209;347 |
| 415 | `pre_order_parallel` |  |  |  | Y | Y |  | Y |  | 365&#8209;366 |
| 416 | `build_balanced` |  |  |  | Y | Y |  | Y |  | 384&#8209;385 |
| 417 | `filter_parallel` |  |  |  | Y | Y |  | Y |  | 405&#8209;408 |
| 418 | `reduce_parallel` |  |  |  | Y | Y |  | Y |  | 432&#8209;435 |
| 419 | `height_rec` |  |  |  | Y | Y |  | Y |  | 456&#8209;457 |
| 420 | `default` |  | Y |  |  | Y |  | Y |  | 561 |

### Chap37/BSTSplayStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 421 | `new` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;100 |
| 422 | `size` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 423 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 424 | `height` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 425 | `insert` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 426 | `find` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 427 | `contains` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 428 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;117 |
| 429 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;121 |
| 430 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 431 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 432 | `new_node` |  |  |  | Y | Y |  |  | unknown | 131&#8209;136 |
| 433 | `size_link` |  |  |  | Y | Y |  |  | unknown | 146&#8209;147 |
| 434 | `height_link` |  |  |  | Y | Y |  |  | unknown | 156&#8209;159 |
| 435 | `update` |  |  |  | Y | Y |  |  | unknown | 173&#8209;177 |
| 436 | `splay` |  |  |  | Y | Y |  | Y |  | 189&#8209;190 |
| 437 | `bst_insert` |  |  |  | Y | Y |  | Y |  | 310&#8209;311 |
| 438 | `insert_link` |  |  |  | Y | Y |  | Y |  | 332 |
| 439 | `find_link` |  |  |  | Y | Y |  |  | unknown | 343&#8209;346 |
| 440 | `min_link` |  |  |  | Y | Y |  |  | unknown | 360&#8209;364 |
| 441 | `max_link` |  |  |  | Y | Y |  |  | unknown | 375&#8209;379 |
| 442 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 390&#8209;391 |
| 443 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 400&#8209;401 |
| 444 | `default` |  | Y |  |  | Y |  | Y |  | 449 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
