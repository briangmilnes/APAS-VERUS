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
| 45 | `new` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;275 |
| 46 | `set` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;285 |
| 47 | `length` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;291 |
| 48 | `nth` | Y | Y |  |  | Y |  |  | unknown | 296&#8209;298 |
| 49 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;311 |
| 50 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;324 |
| 51 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;332 |
| 52 | `empty` | Y | Y |  |  | Y |  |  | unknown | 341&#8209;342 |
| 53 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 347&#8209;350 |
| 54 | `append` | Y | Y |  |  | Y |  |  | unknown | 355&#8209;363 |
| 55 | `filter` | Y | Y |  |  | Y |  |  | unknown | 370&#8209;384 |
| 56 | `update` | Y | Y |  |  | Y |  |  | unknown | 389&#8209;397 |
| 57 | `inject` | Y | Y |  |  | Y |  |  | unknown | 403&#8209;412 |
| 58 | `ninject` | Y | Y |  |  | Y |  |  | unknown | 418&#8209;426 |
| 59 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 431&#8209;432 |
| 60 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 437&#8209;438 |
| 61 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 443&#8209;448 |
| 62 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 453&#8209;461 |
| 63 | `scan` | Y | Y |  |  | Y |  |  | unknown | 466&#8209;478 |
| 64 | `map` | Y | Y |  |  | Y |  |  | unknown | 483&#8209;488 |
| 65 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 493&#8209;499 |
| 66 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 504&#8209;509 |
| 67 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1072&#8209;1074 |
| 68 | `iter` |  |  | Y |  | Y |  |  | unknown | 1078&#8209;1082 |
| 69 | `map_par` |  |  | Y |  | Y |  |  | unknown | 1089&#8209;1100 |
| 70 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 1141&#8209;1151 |
| 71 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 1195&#8209;1198 |
| 72 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 1223&#8209;1238 |
| 73 | `ninject_par` |  |  | Y |  | Y |  |  | unknown | 1311&#8209;1321 |
| 74 | `next` |  | Y |  |  | Y |  |  | unknown | 1448&#8209;1464 |
| 75 | `eq` |  | Y |  |  | Y |  |  | unknown | 1567&#8209;1568 |

### Chap18/ArraySeqMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 76 | `new` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;111 |
| 77 | `length` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 78 | `nth` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 79 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;137 |
| 80 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;150 |
| 81 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;158 |
| 82 | `empty` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;168 |
| 83 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;176 |
| 84 | `append` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;189 |
| 85 | `filter` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;210 |
| 86 | `update` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;223 |
| 87 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 228&#8209;229 |
| 88 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;235 |
| 89 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;245 |
| 90 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;258 |
| 91 | `scan` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;275 |
| 92 | `map` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;285 |
| 93 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;296 |
| 94 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;306 |
| 95 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 764&#8209;766 |
| 96 | `iter` |  |  | Y |  | Y |  |  | unknown | 770&#8209;774 |
| 97 | `map_par` |  |  | Y |  | Y |  |  | unknown | 782&#8209;792 |
| 98 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 840&#8209;849 |
| 99 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 897&#8209;900 |
| 100 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 937&#8209;952 |
| 101 | `next` |  | Y |  |  | Y |  |  | unknown | 1056&#8209;1072 |
| 102 | `eq` |  | Y |  |  | Y |  |  | unknown | 1175&#8209;1176 |

### Chap18/ArraySeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 103 | `new` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;120 |
| 104 | `set` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;130 |
| 105 | `length` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;136 |
| 106 | `nth` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;143 |
| 107 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;156 |
| 108 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;169 |
| 109 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;177 |
| 110 | `empty` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;187 |
| 111 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;195 |
| 112 | `append` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;208 |
| 113 | `filter` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;229 |
| 114 | `update` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;242 |
| 115 | `inject` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;257 |
| 116 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;263 |
| 117 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;269 |
| 118 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;279 |
| 119 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;292 |
| 120 | `scan` | Y | Y |  |  | Y |  |  | unknown | 297&#8209;309 |
| 121 | `map` | Y | Y |  |  | Y |  |  | unknown | 314&#8209;319 |
| 122 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 324&#8209;330 |
| 123 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;340 |
| 124 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 879&#8209;881 |
| 125 | `iter` |  |  | Y |  | Y |  |  | unknown | 885&#8209;889 |
| 126 | `next` |  | Y |  |  | Y |  |  | unknown | 936&#8209;952 |
| 127 | `eq` |  | Y |  |  | Y |  |  | unknown | 1042&#8209;1043 |

### Chap18/ArraySeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 128 | `new` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;120 |
| 129 | `length` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;126 |
| 130 | `nth` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 131 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;146 |
| 132 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;159 |
| 133 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;167 |
| 134 | `empty` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;177 |
| 135 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;185 |
| 136 | `append` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;198 |
| 137 | `filter` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;219 |
| 138 | `update` | Y | Y |  |  | Y |  |  | unknown | 224&#8209;232 |
| 139 | `inject` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;247 |
| 140 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;253 |
| 141 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;259 |
| 142 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 264&#8209;269 |
| 143 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;282 |
| 144 | `scan` | Y | Y |  |  | Y |  |  | unknown | 287&#8209;299 |
| 145 | `map` | Y | Y |  |  | Y |  |  | unknown | 304&#8209;309 |
| 146 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 314&#8209;320 |
| 147 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 325&#8209;330 |
| 148 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 860&#8209;862 |
| 149 | `iter` |  |  | Y |  | Y |  |  | unknown | 866&#8209;870 |
| 150 | `next` |  | Y |  |  | Y |  |  | unknown | 904&#8209;920 |
| 151 | `eq` |  | Y |  |  | Y |  |  | unknown | 1004&#8209;1005 |

### Chap18/LinkedListStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 152 | `new` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;107 |
| 153 | `set` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;116 |
| 154 | `length` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 155 | `nth` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;127 |
| 156 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;139 |
| 157 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;146 |
| 158 | `empty` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;155 |
| 159 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;162 |
| 160 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;172 |
| 161 | `map` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;181 |
| 162 | `append` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;193 |
| 163 | `filter` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;213 |
| 164 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;222 |
| 165 | `update` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;234 |
| 166 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;239 |
| 167 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 243&#8209;244 |
| 168 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;253 |
| 169 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;265 |
| 170 | `scan` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;281 |
| 171 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 721&#8209;723 |
| 172 | `iter` |  |  | Y |  | Y |  |  | unknown | 729&#8209;733 |
| 173 | `next` |  | Y |  |  | Y |  |  | unknown | 769&#8209;785 |
| 174 | `eq` |  | Y |  |  | Y |  |  | unknown | 877&#8209;878 |

### Chap18/LinkedListStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 175 | `new` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;108 |
| 176 | `length` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 177 | `nth` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 178 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;131 |
| 179 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;138 |
| 180 | `empty` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;147 |
| 181 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;154 |
| 182 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;164 |
| 183 | `map` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;173 |
| 184 | `append` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;185 |
| 185 | `filter` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;205 |
| 186 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;214 |
| 187 | `update` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;226 |
| 188 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;231 |
| 189 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;236 |
| 190 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;245 |
| 191 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;257 |
| 192 | `scan` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;273 |
| 193 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 704&#8209;706 |
| 194 | `iter` |  |  | Y |  | Y |  |  | unknown | 712&#8209;716 |
| 195 | `next` |  | Y |  |  | Y |  |  | unknown | 752&#8209;768 |
| 196 | `eq` |  | Y |  |  | Y |  |  | unknown | 860&#8209;861 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
