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
| 1 | Chap18 | ArraySeq | 23 | 24 | 3 | 13 | 40 | 0 | 39 | 0 | 1 |
| 2 | Chap18 | ArraySeqMtEph | 22 | 23 | 10 | 1 | 34 | 0 | 34 | 0 | 0 |
| 3 | Chap18 | ArraySeqMtEphSlice | 20 | 21 | 0 | 12 | 33 | 0 | 33 | 0 | 0 |
| 4 | Chap18 | ArraySeqMtPer | 20 | 21 | 11 | 0 | 32 | 0 | 32 | 0 | 0 |
| 5 | Chap18 | ArraySeqSpecsAndLemmas | 0 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 6 | Chap18 | ArraySeqStEph | 21 | 22 | 2 | 1 | 25 | 0 | 25 | 0 | 0 |
| 7 | Chap18 | ArraySeqStPer | 20 | 21 | 2 | 1 | 24 | 0 | 24 | 0 | 0 |
| 8 | Chap18 | LinkedListStEph | 19 | 20 | 2 | 1 | 23 | 0 | 23 | 0 | 0 |
| 9 | Chap18 | LinkedListStPer | 18 | 19 | 2 | 1 | 22 | 0 | 22 | 0 | 0 |

## Function-by-Function Detail

### Chap18/ArraySeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_deep_view_len` |  |  |  | Y | Y |  |  | unknown | 160&#8209;162 |
| 2 | `lemma_deep_view_key` |  |  |  | Y | Y |  |  | unknown | 167&#8209;172 |
| 3 | `lemma_find_key_index_bounds` |  |  |  | Y | Y |  |  | unknown | 177&#8209;182 |
| 4 | `lemma_find_key_index_found` |  |  |  | Y | Y |  |  | unknown | 192&#8209;203 |
| 5 | `lemma_find_key_index_not_found` |  |  |  | Y | Y |  |  | unknown | 212&#8209;220 |
| 6 | `lemma_spec_collect_step_some` |  |  |  | Y | Y |  |  | unknown | 229&#8209;241 |
| 7 | `lemma_spec_collect_step_none` |  |  |  | Y | Y |  |  | unknown | 251&#8209;262 |
| 8 | `lemma_find_key_some` |  |  |  | Y | Y |  |  | unknown | 271&#8209;278 |
| 9 | `lemma_find_key_none` |  |  |  | Y | Y |  |  | unknown | 290&#8209;295 |
| 10 | `new` | Y | Y |  |  | Y |  |  | unknown | 318&#8209;325 |
| 11 | `set` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;334 |
| 12 | `length` | Y | Y |  |  | Y |  |  | unknown | 339&#8209;340 |
| 13 | `nth` | Y | Y |  |  | Y |  |  | unknown | 346&#8209;348 |
| 14 | `empty` | Y | Y |  |  | Y |  |  | unknown | 353&#8209;354 |
| 15 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 359&#8209;362 |
| 16 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 367&#8209;375 |
| 17 | `append` | Y | Y |  |  | Y |  |  | unknown | 380&#8209;388 |
| 18 | `filter` | Y | Y |  |  | Y |  |  | unknown | 395&#8209;410 |
| 19 | `update` | Y | Y |  |  | Y |  |  | unknown | 416&#8209;424 |
| 20 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 429&#8209;430 |
| 21 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 435&#8209;436 |
| 22 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 442&#8209;447 |
| 23 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 453&#8209;461 |
| 24 | `scan` | Y | Y |  |  | Y |  |  | unknown | 467&#8209;481 |
| 25 | `inject` | Y | Y |  |  | Y |  |  | unknown | 488&#8209;497 |
| 26 | `scan_inclusive` | Y | Y |  |  | Y |  |  | unknown | 502&#8209;512 |
| 27 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 516&#8209;524 |
| 28 | `remove` | Y | Y |  |  | Y |  |  | unknown | 528&#8209;535 |
| 29 | `insert` | Y | Y |  |  | Y |  |  | unknown | 539&#8209;546 |
| 30 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 550&#8209;553 |
| 31 | `find_key` | Y | Y |  |  | Y |  |  | unknown | 557&#8209;569 |
| 32 | `collect` | Y | Y |  |  | Y |  |  | unknown | 577&#8209;589 |
| 33 | `map` |  |  |  | Y | Y |  |  | unknown | 1301&#8209;1305 |
| 34 | `tabulate` |  |  |  | Y | Y |  |  | unknown | 1332&#8209;1338 |
| 35 | `flatten` |  |  |  | Y | Y |  |  | unknown | 1363&#8209;1367 |
| 36 | `iterate_prefixes` |  |  |  | Y | Y |  |  | unknown | 1423&#8209;1442 |
| 37 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1505&#8209;1507 |
| 38 | `iter` |  |  | Y |  | Y |  |  | unknown | 1511&#8209;1515 |
| 39 | `iter_mut` |  |  | Y |  | Y |  | Y |  | 1523 |
| 40 | `eq` |  | Y |  |  | Y |  |  | unknown | 1586&#8209;1587 |

### Chap18/ArraySeqMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 41 | `new` | Y | Y |  |  | Y |  |  | unknown | 1476&#8209;1484 |
| 42 | `set` | Y | Y |  |  | Y |  |  | unknown | 1488&#8209;1493 |
| 43 | `length` | Y | Y |  |  | Y |  |  | unknown | 1498&#8209;1499 |
| 44 | `nth` | Y | Y |  |  | Y |  |  | unknown | 1505&#8209;1507 |
| 45 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 1511&#8209;1520 |
| 46 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 1525&#8209;1534 |
| 47 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 1538&#8209;1542 |
| 48 | `empty` | Y | Y |  |  | Y |  |  | unknown | 1551&#8209;1552 |
| 49 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 1557&#8209;1561 |
| 50 | `append` | Y | Y |  |  | Y |  |  | unknown | 1566&#8209;1575 |
| 51 | `filter` | Y | Y |  |  | Y |  |  | unknown | 1582&#8209;1597 |
| 52 | `update` | Y | Y |  |  | Y |  |  | unknown | 1603&#8209;1612 |
| 53 | `inject` | Y | Y |  |  | Y |  |  | unknown | 1619&#8209;1629 |
| 54 | `ninject` | Y | Y |  |  | Y |  |  | unknown | 1635&#8209;1644 |
| 55 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 1649&#8209;1650 |
| 56 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 1655&#8209;1656 |
| 57 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 1661&#8209;1666 |
| 58 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 1671&#8209;1680 |
| 59 | `scan` | Y | Y |  |  | Y |  |  | unknown | 1685&#8209;1697 |
| 60 | `map` | Y | Y |  |  | Y |  |  | unknown | 1702&#8209;1710 |
| 61 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 1715&#8209;1722 |
| 62 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 1728&#8209;1733 |
| 63 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 637&#8209;639 |
| 64 | `iter` |  |  | Y |  | Y |  |  | unknown | 643&#8209;647 |
| 65 | `map_par` |  |  | Y |  | Y |  |  | unknown | 653&#8209;666 |
| 66 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 708&#8209;719 |
| 67 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 766&#8209;769 |
| 68 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 793&#8209;811 |
| 69 | `reduce_dc` |  |  | Y |  | Y |  |  | unknown | 890&#8209;906 |
| 70 | `map_dc` |  |  | Y |  | Y |  |  | unknown | 983&#8209;996 |
| 71 | `filter_dc` |  |  | Y |  | Y |  |  | unknown | 1110&#8209;1129 |
| 72 | `ninject_par` |  |  | Y |  | Y |  |  | unknown | 1312&#8209;1322 |
| 73 | `apply_ninject_updates` |  |  |  | Y | Y |  |  | unknown | 1742&#8209;1751 |
| 74 | `eq` |  | Y |  |  | Y |  |  | unknown | 1841&#8209;1842 |

### Chap18/ArraySeqMtEphSlice.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 75 | `lemma_monoid_fold_left` |  |  |  | Y | Y |  |  | unknown | 138&#8209;141 |
| 76 | `lemma_prefix_fold_matching` |  |  |  | Y | Y |  |  | unknown | 161&#8209;170 |
| 77 | `lemma_prefix_fold_split` |  |  |  | Y | Y |  |  | unknown | 178&#8209;190 |
| 78 | `lemma_prefix_fold_eq_fold_left` |  |  |  | Y | Y |  |  | unknown | 210&#8209;218 |
| 79 | `lemma_sum_inner_lens_split` |  |  |  | Y | Y |  |  | unknown | 231&#8209;248 |
| 80 | `length` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;277 |
| 81 | `nth_cloned` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;285 |
| 82 | `slice` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;298 |
| 83 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;306 |
| 84 | `empty` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;313 |
| 85 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 317&#8209;322 |
| 86 | `new` | Y | Y |  |  | Y |  |  | unknown | 325&#8209;333 |
| 87 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 337&#8209;344 |
| 88 | `iter` | Y | Y |  |  | Y |  |  | unknown | 346&#8209;351 |
| 89 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 355&#8209;365 |
| 90 | `map` | Y | Y |  |  | Y |  |  | unknown | 369&#8209;380 |
| 91 | `filter` | Y | Y |  |  | Y |  |  | unknown | 384&#8209;394 |
| 92 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 399&#8209;407 |
| 93 | `scan` | Y | Y |  |  | Y |  |  | unknown | 412&#8209;426 |
| 94 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 430&#8209;432 |
| 95 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 436&#8209;438 |
| 96 | `append` | Y | Y |  |  | Y |  |  | unknown | 442&#8209;454 |
| 97 | `update` | Y | Y |  |  | Y |  |  | unknown | 458&#8209;468 |
| 98 | `inject` | Y | Y |  |  | Y |  |  | unknown | 472&#8209;482 |
| 99 | `ninject` | Y | Y |  |  | Y |  |  | unknown | 486&#8209;495 |
| 100 | `reduce_dc` |  |  |  | Y | Y |  |  | unknown | 867&#8209;882 |
| 101 | `map_dc_vec` |  |  |  | Y | Y |  |  | unknown | 979&#8209;989 |
| 102 | `filter_dc_vec` |  |  |  | Y | Y |  |  | unknown | 1052&#8209;1063 |
| 103 | `tabulate_dc_vec` |  |  |  | Y | Y |  |  | unknown | 1129&#8209;1140 |
| 104 | `scan_dc_vec` |  |  |  | Y | Y |  |  | unknown | 1228&#8209;1248 |
| 105 | `flatten` |  |  |  | Y | Y |  |  | unknown | 1418&#8209;1426 |
| 106 | `flatten_dc_vec` |  |  |  | Y | Y |  |  | unknown | 1435&#8209;1444 |
| 107 | `eq` |  | Y |  |  | Y |  |  | unknown | 1608&#8209;1609 |

### Chap18/ArraySeqMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 108 | `new` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;116 |
| 109 | `length` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;122 |
| 110 | `nth` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;130 |
| 111 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;143 |
| 112 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;157 |
| 113 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;165 |
| 114 | `empty` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;175 |
| 115 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;184 |
| 116 | `append` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;198 |
| 117 | `filter` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;220 |
| 118 | `update` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;235 |
| 119 | `inject` | Y | Y |  |  | Y |  |  | unknown | 242&#8209;252 |
| 120 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;258 |
| 121 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;264 |
| 122 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;274 |
| 123 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 279&#8209;287 |
| 124 | `scan` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;304 |
| 125 | `map` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;314 |
| 126 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 319&#8209;325 |
| 127 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 330&#8209;335 |
| 128 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 886&#8209;888 |
| 129 | `iter` |  |  | Y |  | Y |  |  | unknown | 892&#8209;896 |
| 130 | `map_par` |  |  | Y |  | Y |  |  | unknown | 904&#8209;915 |
| 131 | `filter_dc` |  |  | Y |  | Y |  |  | unknown | 965&#8209;986 |
| 132 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 1147&#8209;1157 |
| 133 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 1207&#8209;1210 |
| 134 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 1246&#8209;1262 |
| 135 | `map_inner` |  |  | Y |  | Y |  |  | unknown | 1347&#8209;1360 |
| 136 | `filter_inner` |  |  | Y |  | Y |  |  | unknown | 1452&#8209;1468 |
| 137 | `reduce_inner` |  |  | Y |  | Y |  |  | unknown | 1563&#8209;1578 |
| 138 | `tabulate_inner` |  |  | Y |  | Y |  |  | unknown | 1653&#8209;1668 |
| 139 | `eq` |  | Y |  |  | Y |  |  | unknown | 1788&#8209;1789 |

### Chap18/ArraySeqSpecsAndLemmas.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 140 | `lemma_spec_inject_len` |  |  |  | Y | Y |  |  | unknown | 64&#8209;66 |
| 141 | `lemma_spec_inject_element` |  |  |  | Y | Y |  |  | unknown | 75&#8209;84 |

### Chap18/ArraySeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 142 | `new` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;110 |
| 143 | `set` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;119 |
| 144 | `length` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 145 | `nth` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 146 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;146 |
| 147 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;160 |
| 148 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;168 |
| 149 | `empty` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;178 |
| 150 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;187 |
| 151 | `append` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;201 |
| 152 | `filter` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;223 |
| 153 | `update` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;238 |
| 154 | `inject` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;255 |
| 155 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;261 |
| 156 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;267 |
| 157 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;277 |
| 158 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;290 |
| 159 | `scan` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;307 |
| 160 | `map` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;317 |
| 161 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 322&#8209;328 |
| 162 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 333&#8209;338 |
| 163 | `lemma_take_fold_left_step` |  |  |  | Y | Y |  |  | unknown | 938&#8209;940 |
| 164 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 948&#8209;950 |
| 165 | `iter` |  |  | Y |  | Y |  |  | unknown | 954&#8209;958 |
| 166 | `eq` |  | Y |  |  | Y |  |  | unknown | 1017&#8209;1018 |

### Chap18/ArraySeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 167 | `new` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;110 |
| 168 | `length` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;116 |
| 169 | `nth` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 170 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;137 |
| 171 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;151 |
| 172 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;159 |
| 173 | `empty` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;169 |
| 174 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;178 |
| 175 | `append` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;192 |
| 176 | `filter` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;214 |
| 177 | `update` | Y | Y |  |  | Y |  |  | unknown | 220&#8209;229 |
| 178 | `inject` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;246 |
| 179 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;252 |
| 180 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;258 |
| 181 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;268 |
| 182 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;281 |
| 183 | `scan` | Y | Y |  |  | Y |  |  | unknown | 286&#8209;298 |
| 184 | `map` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;308 |
| 185 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;319 |
| 186 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 324&#8209;329 |
| 187 | `lemma_take_fold_left_step` |  |  |  | Y | Y |  |  | unknown | 915&#8209;917 |
| 188 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 925&#8209;927 |
| 189 | `iter` |  |  | Y |  | Y |  |  | unknown | 931&#8209;935 |
| 190 | `eq` |  | Y |  |  | Y |  |  | unknown | 995&#8209;996 |

### Chap18/LinkedListStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 191 | `new` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;108 |
| 192 | `set` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;116 |
| 193 | `length` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 194 | `nth` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;127 |
| 195 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;140 |
| 196 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;147 |
| 197 | `empty` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;155 |
| 198 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;162 |
| 199 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;172 |
| 200 | `map` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;181 |
| 201 | `append` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;194 |
| 202 | `filter` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;215 |
| 203 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;223 |
| 204 | `update` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;236 |
| 205 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;240 |
| 206 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 243&#8209;244 |
| 207 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 247&#8209;252 |
| 208 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;264 |
| 209 | `scan` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;280 |
| 210 | `lemma_take_fold_left_step` |  |  |  | Y | Y |  |  | unknown | 763&#8209;765 |
| 211 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 773&#8209;775 |
| 212 | `iter` |  |  | Y |  | Y |  |  | unknown | 780&#8209;784 |
| 213 | `eq` |  | Y |  |  | Y |  |  | unknown | 847&#8209;848 |

### Chap18/LinkedListStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 214 | `new` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;108 |
| 215 | `length` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 216 | `nth` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 217 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;132 |
| 218 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;139 |
| 219 | `empty` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;147 |
| 220 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;154 |
| 221 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;164 |
| 222 | `map` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;173 |
| 223 | `append` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;186 |
| 224 | `filter` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;207 |
| 225 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;215 |
| 226 | `update` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;228 |
| 227 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;232 |
| 228 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;236 |
| 229 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;244 |
| 230 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;256 |
| 231 | `scan` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;272 |
| 232 | `lemma_take_fold_left_step` |  |  |  | Y | Y |  |  | unknown | 744&#8209;746 |
| 233 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 754&#8209;756 |
| 234 | `iter` |  |  | Y |  | Y |  |  | unknown | 761&#8209;765 |
| 235 | `eq` |  | Y |  |  | Y |  |  | unknown | 828&#8209;829 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
