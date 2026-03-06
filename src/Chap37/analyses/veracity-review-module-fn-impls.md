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
| 2 | Chap37 | AVLTreeSeqMtPer | 11 | 14 | 0 | 13 | 27 | 0 | 11 | 14 | 2 |
| 3 | Chap37 | AVLTreeSeqStEph | 18 | 21 | 0 | 12 | 33 | 0 | 24 | 8 | 1 |
| 4 | Chap37 | AVLTreeSeqStPer | 13 | 16 | 0 | 14 | 30 | 0 | 14 | 14 | 2 |
| 5 | Chap37 | BSTAVLMtEph | 6 | 6 | 0 | 8 | 14 | 0 | 12 | 0 | 2 |
| 6 | Chap37 | BSTAVLStEph | 7 | 7 | 0 | 10 | 17 | 0 | 17 | 0 | 0 |
| 7 | Chap37 | BSTBBAlphaMtEph | 6 | 6 | 0 | 5 | 11 | 0 | 9 | 0 | 2 |
| 8 | Chap37 | BSTBBAlphaStEph | 10 | 10 | 0 | 7 | 17 | 0 | 17 | 0 | 0 |
| 9 | Chap37 | BSTPlainMtEph | 6 | 6 | 0 | 5 | 11 | 0 | 9 | 0 | 2 |
| 10 | Chap37 | BSTPlainStEph | 10 | 10 | 0 | 10 | 20 | 0 | 20 | 0 | 0 |
| 11 | Chap37 | BSTRBMtEph | 14 | 15 | 0 | 21 | 36 | 0 | 22 | 0 | 14 |
| 12 | Chap37 | BSTRBStEph | 7 | 7 | 0 | 8 | 15 | 0 | 15 | 0 | 0 |
| 13 | Chap37 | BSTSetAVLMtEph | 21 | 22 | 0 | 3 | 25 | 0 | 21 | 1 | 3 |
| 14 | Chap37 | BSTSetBBAlphaMtEph | 21 | 22 | 0 | 3 | 25 | 0 | 21 | 1 | 3 |
| 15 | Chap37 | BSTSetPlainMtEph | 21 | 22 | 0 | 3 | 25 | 0 | 21 | 1 | 3 |
| 16 | Chap37 | BSTSetRBMtEph | 21 | 22 | 0 | 2 | 24 | 0 | 21 | 1 | 2 |
| 17 | Chap37 | BSTSetSplayMtEph | 21 | 22 | 0 | 2 | 24 | 0 | 21 | 1 | 2 |
| 18 | Chap37 | BSTSplayMtEph | 14 | 15 | 0 | 18 | 33 | 0 | 20 | 0 | 13 |
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
| 47 | `from_vec` | Y | Y |  |  | Y |  |  | hole | 220&#8209;223 |
| 48 | `values_in_order` | Y | Y |  |  | Y |  |  | hole | 225&#8209;226 |
| 49 | `height_fn` |  |  |  | Y | Y |  |  | unknown | 234&#8209;235 |
| 50 | `size_fn` |  |  |  | Y | Y |  |  | unknown | 243&#8209;244 |
| 51 | `mk` |  |  |  | Y | Y |  |  | hole | 253&#8209;262 |
| 52 | `rotate_right` |  |  |  | Y | Y |  |  | hole | 272&#8209;276 |
| 53 | `rotate_left` |  |  |  | Y | Y |  |  | hole | 285&#8209;289 |
| 54 | `rebalance` |  |  |  | Y | Y |  |  | hole | 298&#8209;302 |
| 55 | `nth_ref` |  |  |  | Y | Y |  |  | hole | 326&#8209;328 |
| 56 | `set_rec` |  |  |  | Y | Y |  |  | hole | 347&#8209;349 |
| 57 | `inorder_collect` |  |  |  | Y | Y |  |  | hole | 375 |
| 58 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | hole | 384&#8209;385 |
| 59 | `rec` |  |  |  | Y | Y |  | Y |  | 387 |
| 60 | `compare_trees` |  |  |  | Y | Y |  |  | hole | 402 |
| 61 | `default` |  | Y |  |  | Y |  | Y |  | 512 |
| 62 | `next` |  | Y |  |  | Y |  |  | hole | 520&#8209;521 |
| 63 | `eq` |  | Y |  |  | Y |  |  | hole | 557&#8209;558 |

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
| 107 | `from_vec` | Y | Y |  |  | Y |  |  | hole | 181&#8209;184 |
| 108 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;187 |
| 109 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;190 |
| 110 | `iter` | Y | Y |  |  | Y |  |  | hole | 192&#8209;193 |
| 111 | `height_fn` |  |  |  | Y | Y |  |  | unknown | 198&#8209;199 |
| 112 | `size_fn` |  |  |  | Y | Y |  |  | unknown | 207&#8209;208 |
| 113 | `mk` |  |  |  | Y | Y |  |  | hole | 217&#8209;226 |
| 114 | `rotate_right` |  |  |  | Y | Y |  |  | hole | 236&#8209;240 |
| 115 | `rotate_left` |  |  |  | Y | Y |  |  | hole | 249&#8209;253 |
| 116 | `rebalance` |  |  |  | Y | Y |  |  | hole | 262&#8209;266 |
| 117 | `nth_ref` |  |  |  | Y | Y |  |  | hole | 290&#8209;292 |
| 118 | `set_rec` |  |  |  | Y | Y |  |  | hole | 311&#8209;313 |
| 119 | `inorder_collect` |  |  |  | Y | Y |  |  | hole | 339 |
| 120 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | hole | 348&#8209;349 |
| 121 | `rec` |  |  |  | Y | Y |  | Y |  | 351 |
| 122 | `compare_trees` |  |  |  | Y | Y |  |  | hole | 364 |
| 123 | `default` |  | Y |  |  | Y |  | Y |  | 479 |
| 124 | `push_left_iter_stper` |  |  |  | Y | Y |  |  | hole | 493 |
| 125 | `next` |  | Y |  |  | Y |  |  | hole | 513&#8209;514 |
| 126 | `eq` |  | Y |  |  | Y |  |  | hole | 532&#8209;533 |

### Chap37/BSTAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 127 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 49&#8209;81 |
| 128 | `new` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;103 |
| 129 | `insert` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 130 | `contains` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;107 |
| 131 | `size` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 132 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 133 | `height` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 134 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 128&#8209;132 |
| 135 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 194&#8209;198 |
| 136 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 261&#8209;270 |
| 137 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 353&#8209;356 |
| 138 | `find_node` |  |  |  | Y | Y |  |  | unknown | 378&#8209;383 |
| 139 | `min_node` |  |  |  | Y | Y |  | Y |  | 405&#8209;406 |
| 140 | `max_node` |  |  |  | Y | Y |  | Y |  | 417&#8209;418 |

### Chap37/BSTAVLStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 141 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 67&#8209;99 |
| 142 | `lemma_max_plus_one` |  |  |  | Y | Y |  |  | unknown | 117&#8209;119 |
| 143 | `new` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;131 |
| 144 | `size` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 145 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;136 |
| 146 | `height` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 147 | `insert` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;148 |
| 148 | `contains` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;151 |
| 149 | `find` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;156 |
| 150 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 161&#8209;199 |
| 151 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 317&#8209;355 |
| 152 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 478&#8209;506 |
| 153 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 764&#8209;775 |
| 154 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 918&#8209;921 |
| 155 | `find_node` |  |  |  | Y | Y |  |  | unknown | 951&#8209;956 |
| 156 | `min_node` |  |  |  | Y | Y |  |  | unknown | 986&#8209;992 |
| 157 | `max_node` |  |  |  | Y | Y |  |  | unknown | 1006&#8209;1012 |

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
| 169 | `new` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;74 |
| 170 | `size` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;77 |
| 171 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 172 | `height` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 173 | `insert` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;89 |
| 174 | `contains` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 175 | `find` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;97 |
| 176 | `delete` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;104 |
| 177 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;110 |
| 178 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;116 |
| 179 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 165&#8209;172 |
| 180 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 300&#8209;303 |
| 181 | `find_node` |  |  |  | Y | Y |  |  | unknown | 333&#8209;338 |
| 182 | `min_node` |  |  |  | Y | Y |  |  | unknown | 368&#8209;374 |
| 183 | `max_node` |  |  |  | Y | Y |  |  | unknown | 388&#8209;394 |
| 184 | `delete_min_node` |  |  |  | Y | Y |  |  | unknown | 409&#8209;420 |
| 185 | `delete_node` |  |  |  | Y | Y |  |  | unknown | 525&#8209;532 |

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
| 197 | `lemma_node_contains` |  |  |  | Y | Y |  |  | unknown | 40&#8209;46 |
| 198 | `lemma_bst_left` |  |  |  | Y | Y |  |  | unknown | 50&#8209;59 |
| 199 | `lemma_bst_right` |  |  |  | Y | Y |  |  | unknown | 63&#8209;72 |
| 200 | `new` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;89 |
| 201 | `size` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 202 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 203 | `height` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 204 | `insert` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;104 |
| 205 | `contains` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 206 | `find` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;112 |
| 207 | `delete` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;119 |
| 208 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;125 |
| 209 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;131 |
| 210 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 221&#8209;228 |
| 211 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 356&#8209;359 |
| 212 | `find_node` |  |  |  | Y | Y |  |  | unknown | 389&#8209;394 |
| 213 | `min_node` |  |  |  | Y | Y |  |  | unknown | 424&#8209;430 |
| 214 | `max_node` |  |  |  | Y | Y |  |  | unknown | 444&#8209;450 |
| 215 | `delete_min_node` |  |  |  | Y | Y |  |  | unknown | 465&#8209;476 |
| 216 | `delete_node` |  |  |  | Y | Y |  |  | unknown | 583&#8209;590 |

### Chap37/BSTRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 217 | `new` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 218 | `from_sorted_slice` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 219 | `insert` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 220 | `find` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;89 |
| 221 | `contains` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;91 |
| 222 | `size` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 223 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;95 |
| 224 | `height` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;97 |
| 225 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 226 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;101 |
| 227 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;103 |
| 228 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 229 | `filter` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;109 |
| 230 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;113 |
| 231 | `new_node` |  |  |  | Y | Y |  | Y |  | 125 |
| 232 | `is_red` |  |  |  | Y | Y |  | Y |  | 135 |
| 233 | `size_link` |  |  |  | Y | Y |  | Y |  | 142 |
| 234 | `update` |  |  |  | Y | Y |  |  | unknown | 149&#8209;154 |
| 235 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 163&#8209;164 |
| 236 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 188&#8209;189 |
| 237 | `flip_colors` |  |  |  | Y | Y |  |  | unknown | 213&#8209;214 |
| 238 | `fix_up` |  |  |  | Y | Y |  |  | unknown | 236&#8209;237 |
| 239 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 274&#8209;276 |
| 240 | `find_link` |  |  |  | Y | Y |  | Y |  | 293&#8209;294 |
| 241 | `min_link` |  |  |  | Y | Y |  | Y |  | 310&#8209;311 |
| 242 | `max_link` |  |  |  | Y | Y |  | Y |  | 322&#8209;323 |
| 243 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 334&#8209;335 |
| 244 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 344&#8209;345 |
| 245 | `in_order_parallel` |  |  |  | Y | Y |  | Y |  | 354&#8209;355 |
| 246 | `pre_order_parallel` |  |  |  | Y | Y |  | Y |  | 373&#8209;374 |
| 247 | `build_balanced` |  |  |  | Y | Y |  |  | unknown | 392&#8209;394 |
| 248 | `filter_parallel` |  |  |  | Y | Y |  | Y |  | 420&#8209;423 |
| 249 | `reduce_parallel` |  |  |  | Y | Y |  | Y |  | 447&#8209;450 |
| 250 | `height_rec` |  |  |  | Y | Y |  | Y |  | 471&#8209;472 |
| 251 | `compute_link_spec_size` |  |  |  | Y | Y |  |  | unknown | 481&#8209;484 |
| 252 | `default` |  | Y |  |  | Y |  | Y |  | 603 |

### Chap37/BSTRBStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 253 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 51&#8209;83 |
| 254 | `new` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;108 |
| 255 | `size` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;111 |
| 256 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 257 | `height` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 258 | `insert` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;123 |
| 259 | `contains` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 260 | `find` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;131 |
| 261 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 137&#8209;143 |
| 262 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 235&#8209;241 |
| 263 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 332&#8209;339 |
| 264 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 461&#8209;464 |
| 265 | `find_node` |  |  |  | Y | Y |  |  | unknown | 494&#8209;499 |
| 266 | `min_node` |  |  |  | Y | Y |  |  | unknown | 529&#8209;535 |
| 267 | `max_node` |  |  |  | Y | Y |  |  | unknown | 549&#8209;555 |

### Chap37/BSTSetAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 268 | `empty` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 269 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 270 | `size` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;81 |
| 271 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;83 |
| 272 | `find` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 273 | `contains` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;87 |
| 274 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;89 |
| 275 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;91 |
| 276 | `insert` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 277 | `delete` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;95 |
| 278 | `union` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;97 |
| 279 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 280 | `difference` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;101 |
| 281 | `split` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;103 |
| 282 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 283 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;107 |
| 284 | `filter` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 285 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 286 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 287 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 288 | `iter` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 289 | `values_vec` |  |  |  | Y | Y |  | Y |  | 123 |
| 290 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 125 |
| 291 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 133&#8209;135 |
| 292 | `next` |  | Y |  |  | Y |  |  | hole | 377&#8209;393 |

### Chap37/BSTSetBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 293 | `empty` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 294 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 295 | `size` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 296 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 297 | `find` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 298 | `contains` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 299 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 300 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 301 | `insert` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 302 | `delete` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 303 | `union` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 304 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 305 | `difference` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 306 | `split` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;86 |
| 307 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 308 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 309 | `filter` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 310 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 311 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 312 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |
| 313 | `iter` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;100 |
| 314 | `values_vec` |  |  |  | Y | Y |  | Y |  | 103 |
| 315 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 106 |
| 316 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 111 |
| 317 | `next` |  | Y |  |  | Y |  |  | hole | 263&#8209;279 |

### Chap37/BSTSetPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 318 | `empty` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 319 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 320 | `size` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 321 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 322 | `find` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 323 | `contains` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 324 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 325 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 326 | `insert` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 327 | `delete` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 328 | `union` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 329 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 330 | `difference` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 331 | `split` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;86 |
| 332 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 333 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 334 | `filter` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 335 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 336 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 337 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |
| 338 | `iter` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;100 |
| 339 | `values_vec` |  |  |  | Y | Y |  | Y |  | 103 |
| 340 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 106 |
| 341 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 111 |
| 342 | `next` |  | Y |  |  | Y |  |  | hole | 319&#8209;335 |

### Chap37/BSTSetRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 343 | `empty` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 344 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 345 | `size` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 346 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 347 | `find` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 348 | `contains` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 349 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 350 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 351 | `insert` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 352 | `delete` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 353 | `union` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 354 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 355 | `difference` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 356 | `split` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;86 |
| 357 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 358 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 359 | `filter` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 360 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 361 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 362 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |
| 363 | `iter` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;100 |
| 364 | `values_vec` |  |  |  | Y | Y |  | Y |  | 103 |
| 365 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 106 |
| 366 | `next` |  | Y |  |  | Y |  |  | hole | 347&#8209;363 |

### Chap37/BSTSetSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 367 | `empty` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 368 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 369 | `size` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 370 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 371 | `find` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 372 | `contains` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 373 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 374 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 375 | `insert` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 376 | `delete` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 377 | `union` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 378 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 379 | `difference` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 380 | `split` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;86 |
| 381 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 382 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 383 | `filter` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 384 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 385 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 386 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |
| 387 | `iter` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;100 |
| 388 | `values_vec` |  |  |  | Y | Y |  | Y |  | 103 |
| 389 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 107 |
| 390 | `next` |  | Y |  |  | Y |  |  | hole | 348&#8209;364 |

### Chap37/BSTSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 391 | `new` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 392 | `from_sorted_slice` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 393 | `insert` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;80 |
| 394 | `find` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 395 | `contains` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 396 | `size` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;86 |
| 397 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 398 | `height` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 399 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 400 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 401 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 402 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |
| 403 | `filter` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 404 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;106 |
| 405 | `new_node` |  |  |  | Y | Y |  | Y |  | 118 |
| 406 | `size_link` |  |  |  | Y | Y |  | Y |  | 127 |
| 407 | `update` |  |  |  | Y | Y |  |  | unknown | 134&#8209;138 |
| 408 | `splay` |  |  |  | Y | Y |  |  | unknown | 149&#8209;151 |
| 409 | `bst_insert` |  |  |  | Y | Y |  |  | unknown | 265&#8209;267 |
| 410 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 288&#8209;289 |
| 411 | `find_link` |  |  |  | Y | Y |  | Y |  | 301&#8209;302 |
| 412 | `min_link` |  |  |  | Y | Y |  | Y |  | 318&#8209;319 |
| 413 | `max_link` |  |  |  | Y | Y |  | Y |  | 330&#8209;331 |
| 414 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 342&#8209;343 |
| 415 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 352&#8209;353 |
| 416 | `in_order_parallel` |  |  |  | Y | Y |  | Y |  | 362&#8209;363 |
| 417 | `pre_order_parallel` |  |  |  | Y | Y |  | Y |  | 381&#8209;382 |
| 418 | `build_balanced` |  |  |  | Y | Y |  |  | unknown | 400&#8209;402 |
| 419 | `filter_parallel` |  |  |  | Y | Y |  | Y |  | 427&#8209;430 |
| 420 | `reduce_parallel` |  |  |  | Y | Y |  | Y |  | 454&#8209;457 |
| 421 | `height_rec` |  |  |  | Y | Y |  | Y |  | 478&#8209;479 |
| 422 | `compute_link_spec_size` |  |  |  | Y | Y |  |  | unknown | 488&#8209;491 |
| 423 | `default` |  | Y |  |  | Y |  | Y |  | 607 |

### Chap37/BSTSplayStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 424 | `new` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;100 |
| 425 | `size` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 426 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 427 | `height` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 428 | `insert` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 429 | `find` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 430 | `contains` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 431 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;117 |
| 432 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;121 |
| 433 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 434 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 435 | `new_node` |  |  |  | Y | Y |  |  | unknown | 131&#8209;136 |
| 436 | `size_link` |  |  |  | Y | Y |  |  | unknown | 146&#8209;147 |
| 437 | `height_link` |  |  |  | Y | Y |  |  | unknown | 156&#8209;159 |
| 438 | `update` |  |  |  | Y | Y |  |  | unknown | 173&#8209;177 |
| 439 | `splay` |  |  |  | Y | Y |  | Y |  | 189&#8209;190 |
| 440 | `bst_insert` |  |  |  | Y | Y |  | Y |  | 310&#8209;311 |
| 441 | `insert_link` |  |  |  | Y | Y |  | Y |  | 332 |
| 442 | `find_link` |  |  |  | Y | Y |  |  | unknown | 343&#8209;346 |
| 443 | `min_link` |  |  |  | Y | Y |  |  | unknown | 360&#8209;364 |
| 444 | `max_link` |  |  |  | Y | Y |  |  | unknown | 375&#8209;379 |
| 445 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 390&#8209;391 |
| 446 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 400&#8209;401 |
| 447 | `default` |  | Y |  |  | Y |  | Y |  | 449 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
