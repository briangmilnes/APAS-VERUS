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
| 1 | Chap19 | ArraySeqMtEph | 25 | 26 | 9 | 2 | 37 | 0 | 37 | 0 | 0 |
| 2 | Chap19 | ArraySeqMtEphSlice | 22 | 23 | 0 | 12 | 35 | 0 | 35 | 0 | 0 |
| 3 | Chap19 | ArraySeqSpecsAndLemmas | 0 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 4 | Chap19 | ArraySeqStEph | 24 | 25 | 3 | 2 | 30 | 0 | 30 | 0 | 0 |
| 5 | Chap19 | ArraySeqStPer | 23 | 24 | 3 | 2 | 29 | 0 | 29 | 0 | 0 |

## Function-by-Function Detail

### Chap19/ArraySeqMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 97&#8209;100 |
| 2 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 112&#8209;117 |
| 3 | `new` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;152 |
| 4 | `set` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;162 |
| 5 | `length` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;168 |
| 6 | `nth` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;176 |
| 7 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;189 |
| 8 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;203 |
| 9 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;211 |
| 10 | `empty` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;217 |
| 11 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;228 |
| 12 | `append` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;242 |
| 13 | `filter` | Y | Y |  |  | Y |  |  | unknown | 247&#8209;260 |
| 14 | `update` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;275 |
| 15 | `inject` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;292 |
| 16 | `ninject` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;307 |
| 17 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;313 |
| 18 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 318&#8209;319 |
| 19 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 324&#8209;329 |
| 20 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 334&#8209;341 |
| 21 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 346&#8209;353 |
| 22 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 358&#8209;366 |
| 23 | `scan` | Y | Y |  |  | Y |  |  | unknown | 371&#8209;381 |
| 24 | `map` | Y | Y |  |  | Y |  |  | unknown | 386&#8209;393 |
| 25 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 398&#8209;404 |
| 26 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 410&#8209;415 |
| 27 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 420&#8209;429 |
| 28 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1022&#8209;1024 |
| 29 | `iter` |  |  | Y |  | Y |  |  | unknown | 1028&#8209;1032 |
| 30 | `map_par` |  |  | Y |  | Y |  |  | unknown | 1041&#8209;1052 |
| 31 | `filter_dc` |  |  | Y |  | Y |  |  | unknown | 1097&#8209;1116 |
| 32 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 1274&#8209;1284 |
| 33 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 1330&#8209;1333 |
| 34 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 1359&#8209;1374 |
| 35 | `map_dc` |  |  | Y |  | Y |  |  | unknown | 1451&#8209;1463 |
| 36 | `concat_seqs` |  |  | Y |  | Y |  |  | unknown | 1544&#8209;1550 |
| 37 | `eq` |  | Y |  |  | Y |  |  | unknown | 1635&#8209;1636 |

### Chap19/ArraySeqMtEphSlice.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `lemma_monoid_fold_left` |  |  |  | Y | Y |  |  | unknown | 137&#8209;140 |
| 39 | `lemma_prefix_fold_matching` |  |  |  | Y | Y |  |  | unknown | 160&#8209;169 |
| 40 | `lemma_prefix_fold_split` |  |  |  | Y | Y |  |  | unknown | 177&#8209;189 |
| 41 | `lemma_prefix_fold_eq_fold_left` |  |  |  | Y | Y |  |  | unknown | 209&#8209;217 |
| 42 | `lemma_sum_inner_lens_split` |  |  |  | Y | Y |  |  | unknown | 230&#8209;247 |
| 43 | `length` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;276 |
| 44 | `nth_cloned` | Y | Y |  |  | Y |  |  | unknown | 279&#8209;284 |
| 45 | `slice` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;297 |
| 46 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;310 |
| 47 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;318 |
| 48 | `empty` | Y | Y |  |  | Y |  |  | unknown | 322&#8209;325 |
| 49 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;334 |
| 50 | `new` | Y | Y |  |  | Y |  |  | unknown | 337&#8209;345 |
| 51 | `to_vec` | Y | Y |  |  | Y |  |  | unknown | 349&#8209;356 |
| 52 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 361&#8209;362 |
| 53 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 367&#8209;368 |
| 54 | `set` | Y | Y |  |  | Y |  |  | unknown | 372&#8209;383 |
| 55 | `append` | Y | Y |  |  | Y |  |  | unknown | 387&#8209;400 |
| 56 | `update` | Y | Y |  |  | Y |  |  | unknown | 404&#8209;415 |
| 57 | `inject` | Y | Y |  |  | Y |  |  | unknown | 420&#8209;430 |
| 58 | `ninject` | Y | Y |  |  | Y |  |  | unknown | 435&#8209;444 |
| 59 | `iter` | Y | Y |  |  | Y |  |  | unknown | 446&#8209;451 |
| 60 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 455&#8209;465 |
| 61 | `map` | Y | Y |  |  | Y |  |  | unknown | 469&#8209;480 |
| 62 | `filter` | Y | Y |  |  | Y |  |  | unknown | 484&#8209;494 |
| 63 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 499&#8209;507 |
| 64 | `scan` | Y | Y |  |  | Y |  |  | unknown | 512&#8209;526 |
| 65 | `reduce_dc` |  |  |  | Y | Y |  |  | unknown | 909&#8209;923 |
| 66 | `map_dc_vec` |  |  |  | Y | Y |  |  | unknown | 1017&#8209;1027 |
| 67 | `filter_dc_vec` |  |  |  | Y | Y |  |  | unknown | 1090&#8209;1101 |
| 68 | `tabulate_dc_vec` |  |  |  | Y | Y |  |  | unknown | 1167&#8209;1178 |
| 69 | `scan_dc_vec` |  |  |  | Y | Y |  |  | unknown | 1265&#8209;1284 |
| 70 | `flatten` |  |  |  | Y | Y |  |  | unknown | 1449&#8209;1457 |
| 71 | `flatten_dc_vec` |  |  |  | Y | Y |  |  | unknown | 1466&#8209;1474 |
| 72 | `eq` |  | Y |  |  | Y |  |  | unknown | 1635&#8209;1636 |

### Chap19/ArraySeqSpecsAndLemmas.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 73 | `lemma_spec_inject_len` |  |  |  | Y | Y |  |  | unknown | 64&#8209;66 |
| 74 | `lemma_spec_inject_element` |  |  |  | Y | Y |  |  | unknown | 75&#8209;84 |

### Chap19/ArraySeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 75 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 94&#8209;97 |
| 76 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 109&#8209;114 |
| 77 | `new` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;149 |
| 78 | `set` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;159 |
| 79 | `length` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;165 |
| 80 | `nth` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;173 |
| 81 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;186 |
| 82 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;200 |
| 83 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;208 |
| 84 | `empty` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;214 |
| 85 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;225 |
| 86 | `append` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;239 |
| 87 | `filter` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;257 |
| 88 | `update` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;272 |
| 89 | `inject` | Y | Y |  |  | Y |  |  | unknown | 279&#8209;289 |
| 90 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;295 |
| 91 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;301 |
| 92 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 306&#8209;311 |
| 93 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;323 |
| 94 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 328&#8209;335 |
| 95 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 340&#8209;348 |
| 96 | `scan` | Y | Y |  |  | Y |  |  | unknown | 353&#8209;363 |
| 97 | `map` | Y | Y |  |  | Y |  |  | unknown | 368&#8209;373 |
| 98 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 378&#8209;384 |
| 99 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 390&#8209;395 |
| 100 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 400&#8209;409 |
| 101 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1007&#8209;1009 |
| 102 | `iter` |  |  | Y |  | Y |  |  | unknown | 1013&#8209;1017 |
| 103 | `lemma_view_index` |  |  | Y |  | Y |  |  | unknown | 1025&#8209;1027 |
| 104 | `eq` |  | Y |  |  | Y |  |  | unknown | 1084&#8209;1085 |

### Chap19/ArraySeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 105 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 93&#8209;96 |
| 106 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 108&#8209;113 |
| 107 | `new` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;147 |
| 108 | `length` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;153 |
| 109 | `nth` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;161 |
| 110 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;174 |
| 111 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;188 |
| 112 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;196 |
| 113 | `empty` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;202 |
| 114 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;213 |
| 115 | `append` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;227 |
| 116 | `filter` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;245 |
| 117 | `update` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;260 |
| 118 | `inject` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;277 |
| 119 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;283 |
| 120 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;289 |
| 121 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;299 |
| 122 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 304&#8209;311 |
| 123 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;323 |
| 124 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 328&#8209;336 |
| 125 | `scan` | Y | Y |  |  | Y |  |  | unknown | 341&#8209;351 |
| 126 | `map` | Y | Y |  |  | Y |  |  | unknown | 356&#8209;361 |
| 127 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 366&#8209;372 |
| 128 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 378&#8209;383 |
| 129 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 388&#8209;397 |
| 130 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1003&#8209;1005 |
| 131 | `lemma_view_index` |  |  | Y |  | Y |  |  | unknown | 1011&#8209;1013 |
| 132 | `iter` |  |  | Y |  | Y |  |  | unknown | 1017&#8209;1021 |
| 133 | `eq` |  | Y |  |  | Y |  |  | unknown | 1079&#8209;1080 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
