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
| 4 | Chap37 | AVLTreeSeqStPer | 13 | 16 | 0 | 14 | 27 | 3 | 14 | 11 | 5 |
| 5 | Chap37 | BSTAVLMtEph | 6 | 6 | 0 | 8 | 14 | 0 | 12 | 0 | 2 |
| 6 | Chap37 | BSTAVLStEph | 7 | 7 | 0 | 10 | 17 | 0 | 17 | 0 | 0 |
| 7 | Chap37 | BSTBBAlphaMtEph | 6 | 6 | 0 | 5 | 11 | 0 | 9 | 0 | 2 |
| 8 | Chap37 | BSTBBAlphaStEph | 10 | 10 | 0 | 7 | 17 | 0 | 17 | 0 | 0 |
| 9 | Chap37 | BSTPlainMtEph | 6 | 6 | 0 | 5 | 11 | 0 | 9 | 0 | 2 |
| 10 | Chap37 | BSTPlainStEph | 10 | 10 | 0 | 7 | 17 | 0 | 17 | 0 | 0 |
| 11 | Chap37 | BSTRBMtEph | 14 | 16 | 0 | 20 | 34 | 2 | 14 | 1 | 21 |
| 12 | Chap37 | BSTRBStEph | 7 | 7 | 0 | 8 | 15 | 0 | 15 | 0 | 0 |
| 13 | Chap37 | BSTSetAVLMtEph | 20 | 20 | 0 | 3 | 23 | 0 | 20 | 0 | 3 |
| 14 | Chap37 | BSTSetBBAlphaMtEph | 20 | 20 | 0 | 3 | 23 | 0 | 20 | 0 | 3 |
| 15 | Chap37 | BSTSetPlainMtEph | 20 | 20 | 0 | 3 | 23 | 0 | 20 | 0 | 3 |
| 16 | Chap37 | BSTSetRBMtEph | 20 | 20 | 0 | 2 | 22 | 0 | 20 | 0 | 2 |
| 17 | Chap37 | BSTSetSplayMtEph | 20 | 20 | 0 | 2 | 22 | 0 | 20 | 0 | 2 |
| 18 | Chap37 | BSTSplayMtEph | 14 | 16 | 0 | 17 | 31 | 2 | 14 | 1 | 18 |
| 19 | Chap37 | BSTSplayStEph | 11 | 12 | 0 | 12 | 24 | 0 | 18 | 0 | 6 |

## Function-by-Function Detail

### Chap37/AVLTreeSeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_size_eq_inorder_len` |  |  |  | Y | Y |  |  | unknown | 146&#8209;149 |
| 2 | `lemma_inorder_values_maps_to_inorder` |  |  |  | Y | Y |  |  | unknown | 160&#8209;162 |
| 3 | `empty` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;183 |
| 4 | `new` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;186 |
| 5 | `length` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;190 |
| 6 | `nth` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;194 |
| 7 | `set` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;201 |
| 8 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;207 |
| 9 | `isEmpty` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;211 |
| 10 | `isSingleton` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;215 |
| 11 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;220 |
| 12 | `new_root` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;223 |
| 13 | `update` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;231 |
| 14 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;239 |
| 15 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;246 |
| 16 | `iter` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;253 |
| 17 | `push_back` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;257 |
| 18 | `contains_value` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;262 |
| 19 | `insert_value` | Y | Y |  |  | Y |  |  | unknown | 264&#8209;266 |
| 20 | `delete_value` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;279 |
| 21 | `is_tree_empty` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;283 |
| 22 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;287 |
| 23 | `cached_height` |  |  |  | Y | Y |  |  | unknown | 295&#8209;296 |
| 24 | `cached_size` |  |  |  | Y | Y |  |  | unknown | 304&#8209;306 |
| 25 | `update_size_height` |  |  |  | Y | Y |  |  | unknown | 316&#8209;337 |
| 26 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 346&#8209;356 |
| 27 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 405&#8209;415 |
| 28 | `rebalance` |  |  |  | Y | Y |  |  | unknown | 457&#8209;472 |
| 29 | `insert_at_link` |  |  |  | Y | Y |  |  | hole | 511&#8209;522 |
| 30 | `nth_link` |  |  |  | Y | Y |  |  | unknown | 591&#8209;594 |
| 31 | `set_link` |  |  |  | Y | Y |  |  | unknown | 609&#8209;619 |
| 32 | `push_inorder` |  |  |  | Y | Y |  |  | unknown | 636&#8209;641 |
| 33 | `compare_trees` |  |  |  | Y | Y |  |  | unknown | 671&#8209;676 |
| 34 | `next` |  | Y |  |  | Y |  |  | hole | 1099&#8209;1115 |
| 35 | `eq` |  | Y |  |  | Y |  |  | hole | 1204&#8209;1205 |
| 36 | `default` |  | Y |  |  |  | Y | Y |  | 1242 |

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
| 106 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;179 |
| 107 | `from_vec` | Y | Y |  |  | Y |  | Y |  | 181 |
| 108 | `values_in_order` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;184 |
| 109 | `to_arrayseq` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;187 |
| 110 | `iter` | Y | Y |  |  | Y |  |  | hole | 189 |
| 111 | `height_fn` |  |  |  | Y | Y |  |  | unknown | 194&#8209;195 |
| 112 | `size_fn` |  |  |  | Y | Y |  |  | unknown | 203&#8209;204 |
| 113 | `mk` |  |  |  | Y | Y |  |  | hole | 213&#8209;222 |
| 114 | `rotate_right` |  |  |  | Y | Y |  |  | hole | 232&#8209;236 |
| 115 | `rotate_left` |  |  |  | Y | Y |  |  | hole | 245&#8209;249 |
| 116 | `rebalance` |  |  |  | Y | Y |  |  | hole | 258&#8209;262 |
| 117 | `nth_ref` |  |  |  | Y | Y |  |  | hole | 286&#8209;288 |
| 118 | `set_rec` |  |  |  | Y | Y |  |  | hole | 307&#8209;309 |
| 119 | `inorder_collect` |  |  |  | Y | Y |  |  | hole | 335 |
| 120 | `build_balanced_from_slice` |  |  |  | Y | Y |  |  | hole | 344&#8209;345 |
| 121 | `rec` |  |  |  | Y | Y |  | Y |  | 347 |
| 122 | `compare_trees` |  |  |  | Y | Y |  |  | hole | 360 |
| 123 | `eq` |  | Y |  |  | Y |  |  | hole | 483&#8209;484 |
| 124 | `default` |  | Y |  |  |  | Y | Y |  | 508 |
| 125 | `push_left_iter` |  |  |  | Y |  | Y | Y |  | 565&#8209;570 |
| 126 | `next` |  | Y |  |  |  | Y | Y |  | 574&#8209;583 |

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
| 214 | `new` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 215 | `from_sorted_slice` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 216 | `insert` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 217 | `find` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 218 | `contains` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 219 | `size` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 220 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 221 | `height` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 222 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;86 |
| 223 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 224 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 225 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 226 | `filter` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;96 |
| 227 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;100 |
| 228 | `new_rb_link_lock` |  |  |  | Y | Y |  |  | hole | 113 |
| 229 | `new_node` |  |  |  | Y | Y |  | Y |  | 117 |
| 230 | `is_red` |  |  |  | Y | Y |  | Y |  | 127 |
| 231 | `size_link` |  |  |  | Y | Y |  | Y |  | 134 |
| 232 | `update` |  |  |  | Y | Y |  | Y |  | 141 |
| 233 | `rotate_left` |  |  |  | Y | Y |  | Y |  | 149 |
| 234 | `rotate_right` |  |  |  | Y | Y |  | Y |  | 168 |
| 235 | `flip_colors` |  |  |  | Y | Y |  | Y |  | 187 |
| 236 | `fix_up` |  |  |  | Y | Y |  | Y |  | 208 |
| 237 | `insert_link` |  |  |  | Y | Y |  | Y |  | 244&#8209;245 |
| 238 | `find_link` |  |  |  | Y | Y |  | Y |  | 262&#8209;263 |
| 239 | `min_link` |  |  |  | Y | Y |  | Y |  | 279&#8209;280 |
| 240 | `max_link` |  |  |  | Y | Y |  | Y |  | 291&#8209;292 |
| 241 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 303&#8209;304 |
| 242 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 313&#8209;314 |
| 243 | `in_order_parallel` |  |  |  | Y | Y |  | Y |  | 323&#8209;324 |
| 244 | `pre_order_parallel` |  |  |  | Y | Y |  | Y |  | 342&#8209;343 |
| 245 | `build_balanced` |  |  |  | Y | Y |  | Y |  | 361&#8209;362 |
| 246 | `filter_parallel` |  |  |  | Y | Y |  | Y |  | 383&#8209;386 |
| 247 | `reduce_parallel` |  |  |  | Y | Y |  | Y |  | 410&#8209;413 |
| 248 | `height_rec` |  | Y |  |  |  | Y | Y |  | 530&#8209;535 |
| 249 | `default` |  | Y |  |  |  | Y | Y |  | 600 |

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
| 265 | `empty` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;43 |
| 266 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 44&#8209;45 |
| 267 | `size` | Y | Y |  |  | Y |  |  | unknown | 46&#8209;47 |
| 268 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 48&#8209;49 |
| 269 | `find` | Y | Y |  |  | Y |  |  | unknown | 50&#8209;51 |
| 270 | `contains` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;53 |
| 271 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;55 |
| 272 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 56&#8209;57 |
| 273 | `insert` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;59 |
| 274 | `delete` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;61 |
| 275 | `union` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 276 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 277 | `difference` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;67 |
| 278 | `split` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 279 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 280 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;73 |
| 281 | `filter` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 282 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 283 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 284 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;81 |
| 285 | `values_vec` |  |  |  | Y | Y |  | Y |  | 87 |
| 286 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 89 |
| 287 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 97&#8209;99 |

### Chap37/BSTSetBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 288 | `empty` | Y | Y |  |  | Y |  |  | unknown | 25&#8209;26 |
| 289 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 27&#8209;28 |
| 290 | `size` | Y | Y |  |  | Y |  |  | unknown | 29&#8209;30 |
| 291 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 31&#8209;32 |
| 292 | `find` | Y | Y |  |  | Y |  |  | unknown | 33&#8209;34 |
| 293 | `contains` | Y | Y |  |  | Y |  |  | unknown | 35&#8209;36 |
| 294 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 37&#8209;38 |
| 295 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 39&#8209;40 |
| 296 | `insert` | Y | Y |  |  | Y |  |  | unknown | 41&#8209;42 |
| 297 | `delete` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;44 |
| 298 | `union` | Y | Y |  |  | Y |  |  | unknown | 45&#8209;46 |
| 299 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 47&#8209;48 |
| 300 | `difference` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;50 |
| 301 | `split` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;52 |
| 302 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 53&#8209;54 |
| 303 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;56 |
| 304 | `filter` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;58 |
| 305 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 306 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 307 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 308 | `values_vec` |  |  |  | Y | Y |  | Y |  | 67 |
| 309 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 70 |
| 310 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 75 |

### Chap37/BSTSetPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 311 | `empty` | Y | Y |  |  | Y |  |  | unknown | 25&#8209;26 |
| 312 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 27&#8209;28 |
| 313 | `size` | Y | Y |  |  | Y |  |  | unknown | 29&#8209;30 |
| 314 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 31&#8209;32 |
| 315 | `find` | Y | Y |  |  | Y |  |  | unknown | 33&#8209;34 |
| 316 | `contains` | Y | Y |  |  | Y |  |  | unknown | 35&#8209;36 |
| 317 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 37&#8209;38 |
| 318 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 39&#8209;40 |
| 319 | `insert` | Y | Y |  |  | Y |  |  | unknown | 41&#8209;42 |
| 320 | `delete` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;44 |
| 321 | `union` | Y | Y |  |  | Y |  |  | unknown | 45&#8209;46 |
| 322 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 47&#8209;48 |
| 323 | `difference` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;50 |
| 324 | `split` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;52 |
| 325 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 53&#8209;54 |
| 326 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;56 |
| 327 | `filter` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;58 |
| 328 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 329 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 330 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 331 | `values_vec` |  |  |  | Y | Y |  | Y |  | 67 |
| 332 | `rebuild_from_vec` |  |  |  | Y | Y |  | Y |  | 70 |
| 333 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 75 |

### Chap37/BSTSetRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 334 | `empty` | Y | Y |  |  | Y |  |  | unknown | 25&#8209;26 |
| 335 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 27&#8209;28 |
| 336 | `size` | Y | Y |  |  | Y |  |  | unknown | 29&#8209;30 |
| 337 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 31&#8209;32 |
| 338 | `find` | Y | Y |  |  | Y |  |  | unknown | 33&#8209;34 |
| 339 | `contains` | Y | Y |  |  | Y |  |  | unknown | 35&#8209;36 |
| 340 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 37&#8209;38 |
| 341 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 39&#8209;40 |
| 342 | `insert` | Y | Y |  |  | Y |  |  | unknown | 41&#8209;42 |
| 343 | `delete` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;44 |
| 344 | `union` | Y | Y |  |  | Y |  |  | unknown | 45&#8209;46 |
| 345 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 47&#8209;48 |
| 346 | `difference` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;50 |
| 347 | `split` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;52 |
| 348 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 53&#8209;54 |
| 349 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;56 |
| 350 | `filter` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;58 |
| 351 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 352 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 353 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 354 | `values_vec` |  |  |  | Y | Y |  | Y |  | 67 |
| 355 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 70 |

### Chap37/BSTSetSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 356 | `empty` | Y | Y |  |  | Y |  |  | unknown | 25&#8209;26 |
| 357 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 27&#8209;28 |
| 358 | `size` | Y | Y |  |  | Y |  |  | unknown | 29&#8209;30 |
| 359 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 31&#8209;32 |
| 360 | `find` | Y | Y |  |  | Y |  |  | unknown | 33&#8209;34 |
| 361 | `contains` | Y | Y |  |  | Y |  |  | unknown | 35&#8209;36 |
| 362 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 37&#8209;38 |
| 363 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 39&#8209;40 |
| 364 | `insert` | Y | Y |  |  | Y |  |  | unknown | 41&#8209;42 |
| 365 | `delete` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;44 |
| 366 | `union` | Y | Y |  |  | Y |  |  | unknown | 45&#8209;46 |
| 367 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 47&#8209;48 |
| 368 | `difference` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;50 |
| 369 | `split` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;52 |
| 370 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 53&#8209;54 |
| 371 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;56 |
| 372 | `filter` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;58 |
| 373 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 374 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 375 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 376 | `values_vec` |  |  |  | Y | Y |  | Y |  | 67 |
| 377 | `from_sorted_iter` |  |  |  | Y | Y |  | Y |  | 71 |

### Chap37/BSTSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 378 | `new` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 379 | `from_sorted_slice` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 380 | `insert` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;67 |
| 381 | `find` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 382 | `contains` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 383 | `size` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;73 |
| 384 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 385 | `height` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 386 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 387 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;81 |
| 388 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;83 |
| 389 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 390 | `filter` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;89 |
| 391 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;93 |
| 392 | `new_splay_link_lock` |  |  |  | Y | Y |  |  | hole | 106 |
| 393 | `new_node` |  |  |  | Y | Y |  | Y |  | 110 |
| 394 | `size_link` |  |  |  | Y | Y |  | Y |  | 119 |
| 395 | `update` |  |  |  | Y | Y |  | Y |  | 126 |
| 396 | `splay` |  |  |  | Y | Y |  | Y |  | 136&#8209;137 |
| 397 | `bst_insert` |  |  |  | Y | Y |  | Y |  | 251&#8209;252 |
| 398 | `insert_link` |  |  |  | Y | Y |  | Y |  | 273 |
| 399 | `find_link` |  |  |  | Y | Y |  | Y |  | 284&#8209;285 |
| 400 | `min_link` |  |  |  | Y | Y |  | Y |  | 301&#8209;302 |
| 401 | `max_link` |  |  |  | Y | Y |  | Y |  | 313&#8209;314 |
| 402 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 325&#8209;326 |
| 403 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 335&#8209;336 |
| 404 | `in_order_parallel` |  |  |  | Y | Y |  | Y |  | 345&#8209;346 |
| 405 | `pre_order_parallel` |  |  |  | Y | Y |  | Y |  | 364&#8209;365 |
| 406 | `build_balanced` |  |  |  | Y | Y |  | Y |  | 383&#8209;384 |
| 407 | `filter_parallel` |  |  |  | Y | Y |  | Y |  | 404&#8209;407 |
| 408 | `reduce_parallel` |  |  |  | Y | Y |  | Y |  | 431&#8209;434 |
| 409 | `height_rec` |  | Y |  |  |  | Y | Y |  | 532&#8209;537 |
| 410 | `default` |  | Y |  |  |  | Y | Y |  | 602 |

### Chap37/BSTSplayStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 411 | `new` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;100 |
| 412 | `size` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 413 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 414 | `height` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 415 | `insert` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 416 | `find` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 417 | `contains` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 418 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;117 |
| 419 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;121 |
| 420 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 421 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 422 | `new_node` |  |  |  | Y | Y |  |  | unknown | 131&#8209;136 |
| 423 | `size_link` |  |  |  | Y | Y |  |  | unknown | 146&#8209;147 |
| 424 | `height_link` |  |  |  | Y | Y |  |  | unknown | 156&#8209;159 |
| 425 | `update` |  |  |  | Y | Y |  |  | unknown | 173&#8209;177 |
| 426 | `splay` |  |  |  | Y | Y |  | Y |  | 189&#8209;190 |
| 427 | `bst_insert` |  |  |  | Y | Y |  | Y |  | 310&#8209;311 |
| 428 | `insert_link` |  |  |  | Y | Y |  | Y |  | 332 |
| 429 | `find_link` |  |  |  | Y | Y |  |  | unknown | 343&#8209;346 |
| 430 | `min_link` |  |  |  | Y | Y |  |  | unknown | 360&#8209;364 |
| 431 | `max_link` |  |  |  | Y | Y |  |  | unknown | 375&#8209;379 |
| 432 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 390&#8209;391 |
| 433 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 400&#8209;401 |
| 434 | `default` |  | Y |  |  | Y |  | Y |  | 449 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
