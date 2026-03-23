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
| 1 | Chap18 | ArraySeq | 23 | 25 | 3 | 13 | 41 | 0 | 39 | 1 | 1 |
| 2 | Chap18 | ArraySeqMtEph | 22 | 24 | 7 | 3 | 34 | 0 | 33 | 1 | 0 |
| 3 | Chap18 | ArraySeqMtPer | 19 | 21 | 6 | 0 | 27 | 0 | 26 | 1 | 0 |
| 4 | Chap18 | ArraySeqStEph | 21 | 23 | 2 | 0 | 25 | 0 | 24 | 1 | 0 |
| 5 | Chap18 | ArraySeqStPer | 20 | 22 | 2 | 0 | 24 | 0 | 23 | 1 | 0 |
| 6 | Chap18 | LinkedListStEph | 19 | 21 | 2 | 0 | 23 | 0 | 22 | 1 | 0 |
| 7 | Chap18 | LinkedListStPer | 18 | 20 | 2 | 0 | 22 | 0 | 21 | 1 | 0 |

## Function-by-Function Detail

### Chap18/ArraySeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_deep_view_len` |  |  |  | Y | Y |  |  | unknown | 161&#8209;163 |
| 2 | `lemma_deep_view_key` |  |  |  | Y | Y |  |  | unknown | 168&#8209;173 |
| 3 | `lemma_find_key_index_bounds` |  |  |  | Y | Y |  |  | unknown | 178&#8209;183 |
| 4 | `lemma_find_key_index_found` |  |  |  | Y | Y |  |  | unknown | 193&#8209;204 |
| 5 | `lemma_find_key_index_not_found` |  |  |  | Y | Y |  |  | unknown | 213&#8209;221 |
| 6 | `lemma_spec_collect_step_some` |  |  |  | Y | Y |  |  | unknown | 230&#8209;242 |
| 7 | `lemma_spec_collect_step_none` |  |  |  | Y | Y |  |  | unknown | 253&#8209;264 |
| 8 | `lemma_find_key_some` |  |  |  | Y | Y |  |  | unknown | 274&#8209;281 |
| 9 | `lemma_find_key_none` |  |  |  | Y | Y |  |  | unknown | 292&#8209;297 |
| 10 | `new` | Y | Y |  |  | Y |  |  | unknown | 319&#8209;326 |
| 11 | `set` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;336 |
| 12 | `length` | Y | Y |  |  | Y |  |  | unknown | 341&#8209;342 |
| 13 | `nth` | Y | Y |  |  | Y |  |  | unknown | 347&#8209;349 |
| 14 | `empty` | Y | Y |  |  | Y |  |  | unknown | 354&#8209;355 |
| 15 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 360&#8209;363 |
| 16 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 368&#8209;376 |
| 17 | `append` | Y | Y |  |  | Y |  |  | unknown | 381&#8209;389 |
| 18 | `filter` | Y | Y |  |  | Y |  |  | unknown | 396&#8209;411 |
| 19 | `update` | Y | Y |  |  | Y |  |  | unknown | 416&#8209;424 |
| 20 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 429&#8209;430 |
| 21 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 435&#8209;436 |
| 22 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 442&#8209;447 |
| 23 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 453&#8209;461 |
| 24 | `scan` | Y | Y |  |  | Y |  |  | unknown | 467&#8209;481 |
| 25 | `inject` | Y | Y |  |  | Y |  |  | unknown | 487&#8209;496 |
| 26 | `scan_inclusive` | Y | Y |  |  | Y |  |  | unknown | 502&#8209;512 |
| 27 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 517&#8209;525 |
| 28 | `remove` | Y | Y |  |  | Y |  |  | unknown | 530&#8209;537 |
| 29 | `insert` | Y | Y |  |  | Y |  |  | unknown | 542&#8209;549 |
| 30 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 554&#8209;557 |
| 31 | `find_key` | Y | Y |  |  | Y |  |  | unknown | 560&#8209;572 |
| 32 | `collect` | Y | Y |  |  | Y |  |  | unknown | 578&#8209;590 |
| 33 | `map` |  |  |  | Y | Y |  |  | unknown | 1253&#8209;1257 |
| 34 | `tabulate` |  |  |  | Y | Y |  |  | unknown | 1284&#8209;1290 |
| 35 | `flatten` |  |  |  | Y | Y |  |  | unknown | 1311&#8209;1315 |
| 36 | `iterate_prefixes` |  |  |  | Y | Y |  |  | unknown | 1369&#8209;1384 |
| 37 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1446&#8209;1448 |
| 38 | `iter` |  |  | Y |  | Y |  |  | unknown | 1452&#8209;1456 |
| 39 | `iter_mut` |  |  | Y |  | Y |  | Y |  | 1464 |
| 40 | `next` |  | Y |  |  | Y |  |  | unknown | 1513&#8209;1529 |
| 41 | `eq` |  | Y |  |  | Y |  |  | hole | 1617&#8209;1618 |

### Chap18/ArraySeqMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 42 | `lemma_spec_inject_element` |  |  |  | Y | Y |  |  | unknown | 122&#8209;131 |
| 43 | `lemma_spec_inject_len` |  |  |  | Y | Y |  |  | unknown | 162&#8209;164 |
| 44 | `apply_ninject_updates` |  |  |  | Y | Y |  |  | unknown | 196&#8209;204 |
| 45 | `new` | Y | Y |  |  | Y |  |  | unknown | 270&#8209;278 |
| 46 | `set` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;288 |
| 47 | `length` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;294 |
| 48 | `nth` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;301 |
| 49 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 306&#8209;315 |
| 50 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;329 |
| 51 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 334&#8209;338 |
| 52 | `empty` | Y | Y |  |  | Y |  |  | unknown | 347&#8209;348 |
| 53 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 353&#8209;357 |
| 54 | `append` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;371 |
| 55 | `filter` | Y | Y |  |  | Y |  |  | unknown | 378&#8209;393 |
| 56 | `update` | Y | Y |  |  | Y |  |  | unknown | 398&#8209;407 |
| 57 | `inject` | Y | Y |  |  | Y |  |  | unknown | 413&#8209;423 |
| 58 | `ninject` | Y | Y |  |  | Y |  |  | unknown | 429&#8209;438 |
| 59 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 443&#8209;444 |
| 60 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 449&#8209;450 |
| 61 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 455&#8209;460 |
| 62 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 465&#8209;473 |
| 63 | `scan` | Y | Y |  |  | Y |  |  | unknown | 478&#8209;490 |
| 64 | `map` | Y | Y |  |  | Y |  |  | unknown | 495&#8209;500 |
| 65 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 505&#8209;511 |
| 66 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 516&#8209;521 |
| 67 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1086&#8209;1088 |
| 68 | `iter` |  |  | Y |  | Y |  |  | unknown | 1092&#8209;1096 |
| 69 | `map_par` |  |  | Y |  | Y |  |  | unknown | 1103&#8209;1114 |
| 70 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 1155&#8209;1165 |
| 71 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 1209&#8209;1212 |
| 72 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 1237&#8209;1252 |
| 73 | `ninject_par` |  |  | Y |  | Y |  |  | unknown | 1325&#8209;1335 |
| 74 | `next` |  | Y |  |  | Y |  |  | unknown | 1462&#8209;1478 |
| 75 | `eq` |  | Y |  |  | Y |  |  | hole | 1581&#8209;1582 |

### Chap18/ArraySeqMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 76 | `new` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;113 |
| 77 | `length` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;119 |
| 78 | `nth` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 79 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;140 |
| 80 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;154 |
| 81 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;163 |
| 82 | `empty` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;173 |
| 83 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;182 |
| 84 | `append` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;196 |
| 85 | `filter` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;218 |
| 86 | `update` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;232 |
| 87 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;238 |
| 88 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 243&#8209;244 |
| 89 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;254 |
| 90 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;267 |
| 91 | `scan` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;284 |
| 92 | `map` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;294 |
| 93 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;305 |
| 94 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;315 |
| 95 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 775&#8209;777 |
| 96 | `iter` |  |  | Y |  | Y |  |  | unknown | 781&#8209;785 |
| 97 | `map_par` |  |  | Y |  | Y |  |  | unknown | 793&#8209;803 |
| 98 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 851&#8209;860 |
| 99 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 908&#8209;911 |
| 100 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 948&#8209;963 |
| 101 | `next` |  | Y |  |  | Y |  |  | unknown | 1067&#8209;1083 |
| 102 | `eq` |  | Y |  |  | Y |  |  | hole | 1186&#8209;1187 |

### Chap18/ArraySeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 103 | `new` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;122 |
| 104 | `set` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;132 |
| 105 | `length` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;138 |
| 106 | `nth` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |
| 107 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;159 |
| 108 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;173 |
| 109 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;182 |
| 110 | `empty` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;192 |
| 111 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;201 |
| 112 | `append` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;214 |
| 113 | `filter` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;235 |
| 114 | `update` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;248 |
| 115 | `inject` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;263 |
| 116 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;269 |
| 117 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;275 |
| 118 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;285 |
| 119 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;298 |
| 120 | `scan` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;315 |
| 121 | `map` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;325 |
| 122 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 330&#8209;336 |
| 123 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 341&#8209;346 |
| 124 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 887&#8209;889 |
| 125 | `iter` |  |  | Y |  | Y |  |  | unknown | 893&#8209;897 |
| 126 | `next` |  | Y |  |  | Y |  |  | unknown | 944&#8209;960 |
| 127 | `eq` |  | Y |  |  | Y |  |  | hole | 1050&#8209;1051 |

### Chap18/ArraySeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 128 | `new` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;122 |
| 129 | `length` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;128 |
| 130 | `nth` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;135 |
| 131 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;149 |
| 132 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;163 |
| 133 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;172 |
| 134 | `empty` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;182 |
| 135 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;191 |
| 136 | `append` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;205 |
| 137 | `filter` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;227 |
| 138 | `update` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;241 |
| 139 | `inject` | Y | Y |  |  | Y |  |  | unknown | 247&#8209;257 |
| 140 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;263 |
| 141 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;269 |
| 142 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;279 |
| 143 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;292 |
| 144 | `scan` | Y | Y |  |  | Y |  |  | unknown | 297&#8209;309 |
| 145 | `map` | Y | Y |  |  | Y |  |  | unknown | 314&#8209;319 |
| 146 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 324&#8209;330 |
| 147 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;340 |
| 148 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 872&#8209;874 |
| 149 | `iter` |  |  | Y |  | Y |  |  | unknown | 878&#8209;882 |
| 150 | `next` |  | Y |  |  | Y |  |  | unknown | 916&#8209;932 |
| 151 | `eq` |  | Y |  |  | Y |  |  | hole | 1016&#8209;1017 |

### Chap18/LinkedListStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 152 | `new` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;109 |
| 153 | `set` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;118 |
| 154 | `length` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 155 | `nth` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;129 |
| 156 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;142 |
| 157 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;150 |
| 158 | `empty` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;159 |
| 159 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;167 |
| 160 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;177 |
| 161 | `map` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;186 |
| 162 | `append` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;199 |
| 163 | `filter` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;220 |
| 164 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 224&#8209;229 |
| 165 | `update` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;242 |
| 166 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;247 |
| 167 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;252 |
| 168 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;261 |
| 169 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;273 |
| 170 | `scan` | Y | Y |  |  | Y |  |  | unknown | 277&#8209;289 |
| 171 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 731&#8209;733 |
| 172 | `iter` |  |  | Y |  | Y |  |  | unknown | 739&#8209;743 |
| 173 | `next` |  | Y |  |  | Y |  |  | unknown | 779&#8209;795 |
| 174 | `eq` |  | Y |  |  | Y |  |  | hole | 887&#8209;888 |

### Chap18/LinkedListStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 175 | `new` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;110 |
| 176 | `length` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 177 | `nth` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 178 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;134 |
| 179 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;142 |
| 180 | `empty` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;151 |
| 181 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;159 |
| 182 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;169 |
| 183 | `map` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;178 |
| 184 | `append` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;191 |
| 185 | `filter` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;212 |
| 186 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;221 |
| 187 | `update` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;234 |
| 188 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;239 |
| 189 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 243&#8209;244 |
| 190 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;253 |
| 191 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;265 |
| 192 | `scan` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;281 |
| 193 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 714&#8209;716 |
| 194 | `iter` |  |  | Y |  | Y |  |  | unknown | 722&#8209;726 |
| 195 | `next` |  | Y |  |  | Y |  |  | unknown | 762&#8209;778 |
| 196 | `eq` |  | Y |  |  | Y |  |  | hole | 870&#8209;871 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
