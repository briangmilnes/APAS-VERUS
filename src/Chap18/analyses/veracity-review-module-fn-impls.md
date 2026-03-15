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
| 1 | Chap18 | ArraySeq | 23 | 25 | 3 | 13 | 41 | 0 | 40 | 0 | 1 |
| 2 | Chap18 | ArraySeqMtEph | 22 | 24 | 7 | 3 | 34 | 0 | 34 | 0 | 0 |
| 3 | Chap18 | ArraySeqMtPer | 19 | 21 | 6 | 0 | 27 | 0 | 27 | 0 | 0 |
| 4 | Chap18 | ArraySeqStEph | 21 | 23 | 2 | 0 | 25 | 0 | 25 | 0 | 0 |
| 5 | Chap18 | ArraySeqStPer | 20 | 22 | 2 | 0 | 24 | 0 | 24 | 0 | 0 |
| 6 | Chap18 | LinkedListStEph | 19 | 21 | 2 | 0 | 23 | 0 | 23 | 0 | 0 |
| 7 | Chap18 | LinkedListStPer | 18 | 20 | 2 | 0 | 22 | 0 | 22 | 0 | 0 |

## Function-by-Function Detail

### Chap18/ArraySeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_deep_view_len` |  |  |  | Y | Y |  |  | unknown | 162&#8209;164 |
| 2 | `lemma_deep_view_key` |  |  |  | Y | Y |  |  | unknown | 169&#8209;174 |
| 3 | `lemma_find_key_index_bounds` |  |  |  | Y | Y |  |  | unknown | 179&#8209;184 |
| 4 | `lemma_find_key_index_found` |  |  |  | Y | Y |  |  | unknown | 194&#8209;205 |
| 5 | `lemma_find_key_index_not_found` |  |  |  | Y | Y |  |  | unknown | 214&#8209;222 |
| 6 | `lemma_spec_collect_step_some` |  |  |  | Y | Y |  |  | unknown | 231&#8209;243 |
| 7 | `lemma_spec_collect_step_none` |  |  |  | Y | Y |  |  | unknown | 254&#8209;265 |
| 8 | `lemma_find_key_some` |  |  |  | Y | Y |  |  | unknown | 275&#8209;282 |
| 9 | `lemma_find_key_none` |  |  |  | Y | Y |  |  | unknown | 293&#8209;298 |
| 10 | `new` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;327 |
| 11 | `set` | Y | Y |  |  | Y |  |  | unknown | 332&#8209;337 |
| 12 | `length` | Y | Y |  |  | Y |  |  | unknown | 342&#8209;343 |
| 13 | `nth` | Y | Y |  |  | Y |  |  | unknown | 348&#8209;350 |
| 14 | `empty` | Y | Y |  |  | Y |  |  | unknown | 355&#8209;356 |
| 15 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 361&#8209;364 |
| 16 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 369&#8209;377 |
| 17 | `append` | Y | Y |  |  | Y |  |  | unknown | 382&#8209;390 |
| 18 | `filter` | Y | Y |  |  | Y |  |  | unknown | 397&#8209;412 |
| 19 | `update` | Y | Y |  |  | Y |  |  | unknown | 417&#8209;425 |
| 20 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 430&#8209;431 |
| 21 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 436&#8209;437 |
| 22 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 443&#8209;448 |
| 23 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 454&#8209;462 |
| 24 | `scan` | Y | Y |  |  | Y |  |  | unknown | 468&#8209;482 |
| 25 | `inject` | Y | Y |  |  | Y |  |  | unknown | 488&#8209;497 |
| 26 | `scan_inclusive` | Y | Y |  |  | Y |  |  | unknown | 503&#8209;513 |
| 27 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 518&#8209;526 |
| 28 | `remove` | Y | Y |  |  | Y |  |  | unknown | 531&#8209;538 |
| 29 | `insert` | Y | Y |  |  | Y |  |  | unknown | 543&#8209;550 |
| 30 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 555&#8209;558 |
| 31 | `find_key` | Y | Y |  |  | Y |  |  | unknown | 561&#8209;573 |
| 32 | `collect` | Y | Y |  |  | Y |  |  | unknown | 579&#8209;591 |
| 33 | `map` |  |  |  | Y | Y |  |  | unknown | 1254&#8209;1258 |
| 34 | `tabulate` |  |  |  | Y | Y |  |  | unknown | 1285&#8209;1291 |
| 35 | `flatten` |  |  |  | Y | Y |  |  | unknown | 1312&#8209;1316 |
| 36 | `iterate_prefixes` |  |  |  | Y | Y |  |  | unknown | 1370&#8209;1385 |
| 37 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1447&#8209;1449 |
| 38 | `iter` |  |  | Y |  | Y |  |  | unknown | 1453&#8209;1457 |
| 39 | `iter_mut` |  |  | Y |  | Y |  | Y |  | 1465 |
| 40 | `next` |  | Y |  |  | Y |  |  | unknown | 1514&#8209;1530 |
| 41 | `eq` |  | Y |  |  | Y |  |  | unknown | 1618&#8209;1619 |

### Chap18/ArraySeqMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 42 | `lemma_spec_inject_element` |  |  |  | Y | Y |  |  | unknown | 123&#8209;132 |
| 43 | `lemma_spec_inject_len` |  |  |  | Y | Y |  |  | unknown | 163&#8209;165 |
| 44 | `apply_ninject_updates` |  |  |  | Y | Y |  |  | unknown | 197&#8209;205 |
| 45 | `new` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;279 |
| 46 | `set` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;289 |
| 47 | `length` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;295 |
| 48 | `nth` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;302 |
| 49 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;316 |
| 50 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;330 |
| 51 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;339 |
| 52 | `empty` | Y | Y |  |  | Y |  |  | unknown | 348&#8209;349 |
| 53 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 354&#8209;358 |
| 54 | `append` | Y | Y |  |  | Y |  |  | unknown | 363&#8209;372 |
| 55 | `filter` | Y | Y |  |  | Y |  |  | unknown | 379&#8209;394 |
| 56 | `update` | Y | Y |  |  | Y |  |  | unknown | 399&#8209;408 |
| 57 | `inject` | Y | Y |  |  | Y |  |  | unknown | 414&#8209;424 |
| 58 | `ninject` | Y | Y |  |  | Y |  |  | unknown | 430&#8209;439 |
| 59 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 444&#8209;445 |
| 60 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 450&#8209;451 |
| 61 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 456&#8209;461 |
| 62 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 466&#8209;474 |
| 63 | `scan` | Y | Y |  |  | Y |  |  | unknown | 479&#8209;491 |
| 64 | `map` | Y | Y |  |  | Y |  |  | unknown | 496&#8209;501 |
| 65 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 506&#8209;512 |
| 66 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 517&#8209;522 |
| 67 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1087&#8209;1089 |
| 68 | `iter` |  |  | Y |  | Y |  |  | unknown | 1093&#8209;1097 |
| 69 | `map_par` |  |  | Y |  | Y |  |  | unknown | 1104&#8209;1115 |
| 70 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 1156&#8209;1166 |
| 71 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 1210&#8209;1213 |
| 72 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 1238&#8209;1253 |
| 73 | `ninject_par` |  |  | Y |  | Y |  |  | unknown | 1326&#8209;1336 |
| 74 | `next` |  | Y |  |  | Y |  |  | unknown | 1463&#8209;1479 |
| 75 | `eq` |  | Y |  |  | Y |  |  | unknown | 1582&#8209;1583 |

### Chap18/ArraySeqMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 76 | `new` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;114 |
| 77 | `length` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 78 | `nth` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;127 |
| 79 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;141 |
| 80 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;155 |
| 81 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;164 |
| 82 | `empty` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;174 |
| 83 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;183 |
| 84 | `append` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;197 |
| 85 | `filter` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;219 |
| 86 | `update` | Y | Y |  |  | Y |  |  | unknown | 224&#8209;233 |
| 87 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;239 |
| 88 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;245 |
| 89 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;255 |
| 90 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;268 |
| 91 | `scan` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;285 |
| 92 | `map` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;295 |
| 93 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;306 |
| 94 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;316 |
| 95 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 776&#8209;778 |
| 96 | `iter` |  |  | Y |  | Y |  |  | unknown | 782&#8209;786 |
| 97 | `map_par` |  |  | Y |  | Y |  |  | unknown | 794&#8209;804 |
| 98 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 852&#8209;861 |
| 99 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 909&#8209;912 |
| 100 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 949&#8209;964 |
| 101 | `next` |  | Y |  |  | Y |  |  | unknown | 1068&#8209;1084 |
| 102 | `eq` |  | Y |  |  | Y |  |  | unknown | 1187&#8209;1188 |

### Chap18/ArraySeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 103 | `new` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;123 |
| 104 | `set` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;133 |
| 105 | `length` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;139 |
| 106 | `nth` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;146 |
| 107 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;160 |
| 108 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;174 |
| 109 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;183 |
| 110 | `empty` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;193 |
| 111 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;202 |
| 112 | `append` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;215 |
| 113 | `filter` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;236 |
| 114 | `update` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;249 |
| 115 | `inject` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;264 |
| 116 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;270 |
| 117 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;276 |
| 118 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;286 |
| 119 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;299 |
| 120 | `scan` | Y | Y |  |  | Y |  |  | unknown | 304&#8209;316 |
| 121 | `map` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;326 |
| 122 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;337 |
| 123 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 342&#8209;347 |
| 124 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 888&#8209;890 |
| 125 | `iter` |  |  | Y |  | Y |  |  | unknown | 894&#8209;898 |
| 126 | `next` |  | Y |  |  | Y |  |  | unknown | 945&#8209;961 |
| 127 | `eq` |  | Y |  |  | Y |  |  | unknown | 1051&#8209;1052 |

### Chap18/ArraySeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 128 | `new` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;123 |
| 129 | `length` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 130 | `nth` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;136 |
| 131 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;150 |
| 132 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;164 |
| 133 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;173 |
| 134 | `empty` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;183 |
| 135 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;192 |
| 136 | `append` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;206 |
| 137 | `filter` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;228 |
| 138 | `update` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;242 |
| 139 | `inject` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;258 |
| 140 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;264 |
| 141 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;270 |
| 142 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;280 |
| 143 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;293 |
| 144 | `scan` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;310 |
| 145 | `map` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;320 |
| 146 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 325&#8209;331 |
| 147 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 336&#8209;341 |
| 148 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 873&#8209;875 |
| 149 | `iter` |  |  | Y |  | Y |  |  | unknown | 879&#8209;883 |
| 150 | `next` |  | Y |  |  | Y |  |  | unknown | 917&#8209;933 |
| 151 | `eq` |  | Y |  |  | Y |  |  | unknown | 1017&#8209;1018 |

### Chap18/LinkedListStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 152 | `new` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;110 |
| 153 | `set` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;119 |
| 154 | `length` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;124 |
| 155 | `nth` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;130 |
| 156 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;143 |
| 157 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;151 |
| 158 | `empty` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;160 |
| 159 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;168 |
| 160 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;178 |
| 161 | `map` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;187 |
| 162 | `append` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;200 |
| 163 | `filter` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;221 |
| 164 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;230 |
| 165 | `update` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;243 |
| 166 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 247&#8209;248 |
| 167 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;253 |
| 168 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;262 |
| 169 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;274 |
| 170 | `scan` | Y | Y |  |  | Y |  |  | unknown | 278&#8209;290 |
| 171 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 732&#8209;734 |
| 172 | `iter` |  |  | Y |  | Y |  |  | unknown | 740&#8209;744 |
| 173 | `next` |  | Y |  |  | Y |  |  | unknown | 780&#8209;796 |
| 174 | `eq` |  | Y |  |  | Y |  |  | unknown | 888&#8209;889 |

### Chap18/LinkedListStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 175 | `new` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;111 |
| 176 | `length` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;116 |
| 177 | `nth` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 178 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;135 |
| 179 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;143 |
| 180 | `empty` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;152 |
| 181 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;160 |
| 182 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;170 |
| 183 | `map` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;179 |
| 184 | `append` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;192 |
| 185 | `filter` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;213 |
| 186 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;222 |
| 187 | `update` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;235 |
| 188 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;240 |
| 189 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;245 |
| 190 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;254 |
| 191 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;266 |
| 192 | `scan` | Y | Y |  |  | Y |  |  | unknown | 270&#8209;282 |
| 193 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 715&#8209;717 |
| 194 | `iter` |  |  | Y |  | Y |  |  | unknown | 723&#8209;727 |
| 195 | `next` |  | Y |  |  | Y |  |  | unknown | 763&#8209;779 |
| 196 | `eq` |  | Y |  |  | Y |  |  | unknown | 871&#8209;872 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
