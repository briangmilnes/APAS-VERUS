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
| 1 | Chap19 | ArraySeqMtEph | 25 | 27 | 6 | 4 | 37 | 0 | 37 | 0 | 0 |
| 2 | Chap19 | ArraySeqMtEphSlice | 8 | 8 | 1 | 0 | 9 | 0 | 9 | 0 | 0 |
| 3 | Chap19 | ArraySeqStEph | 24 | 26 | 2 | 2 | 30 | 0 | 30 | 0 | 0 |
| 4 | Chap19 | ArraySeqStPer | 23 | 25 | 2 | 2 | 29 | 0 | 29 | 0 | 0 |

## Function-by-Function Detail

### Chap19/ArraySeqMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_spec_inject_element` |  |  |  | Y | Y |  |  | unknown | 123&#8209;132 |
| 2 | `lemma_spec_inject_len` |  |  |  | Y | Y |  |  | unknown | 163&#8209;165 |
| 3 | `new` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;194 |
| 4 | `set` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;205 |
| 5 | `length` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;211 |
| 6 | `nth` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;218 |
| 7 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;231 |
| 8 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;244 |
| 9 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;252 |
| 10 | `empty` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;258 |
| 11 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;268 |
| 12 | `append` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;281 |
| 13 | `filter` | Y | Y |  |  | Y |  |  | unknown | 286&#8209;298 |
| 14 | `update` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;311 |
| 15 | `inject` | Y | Y |  |  | Y |  |  | unknown | 317&#8209;326 |
| 16 | `ninject` | Y | Y |  |  | Y |  |  | unknown | 332&#8209;340 |
| 17 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 345&#8209;346 |
| 18 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 351&#8209;352 |
| 19 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 357&#8209;358 |
| 20 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 363&#8209;367 |
| 21 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 372&#8209;374 |
| 22 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 379&#8209;383 |
| 23 | `scan` | Y | Y |  |  | Y |  |  | unknown | 388&#8209;391 |
| 24 | `map` | Y | Y |  |  | Y |  |  | unknown | 396&#8209;401 |
| 25 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 406&#8209;412 |
| 26 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 417&#8209;422 |
| 27 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 427&#8209;435 |
| 28 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 993&#8209;996 |
| 29 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 1008&#8209;1013 |
| 30 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1026&#8209;1028 |
| 31 | `iter` |  |  | Y |  | Y |  |  | unknown | 1032&#8209;1036 |
| 32 | `map_par` |  |  | Y |  | Y |  |  | unknown | 1044&#8209;1055 |
| 33 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 1097&#8209;1107 |
| 34 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 1151&#8209;1154 |
| 35 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 1180&#8209;1195 |
| 36 | `next` |  | Y |  |  | Y |  |  | unknown | 1291&#8209;1307 |
| 37 | `eq` |  | Y |  |  | Y |  |  | unknown | 1410&#8209;1411 |

### Chap19/ArraySeqMtEphSlice.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `length` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 39 | `nth_cloned` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;99 |
| 40 | `slice` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;111 |
| 41 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;123 |
| 42 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;130 |
| 43 | `empty` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;135 |
| 44 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;142 |
| 45 | `new` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;152 |
| 46 | `iter` |  |  | Y |  | Y |  |  | unknown | 249&#8209;255 |

### Chap19/ArraySeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 47 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 119&#8209;122 |
| 48 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 134&#8209;139 |
| 49 | `new` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;174 |
| 50 | `set` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;185 |
| 51 | `length` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;191 |
| 52 | `nth` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;198 |
| 53 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;211 |
| 54 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;224 |
| 55 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;232 |
| 56 | `empty` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;238 |
| 57 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 243&#8209;248 |
| 58 | `append` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;261 |
| 59 | `filter` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;278 |
| 60 | `update` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;291 |
| 61 | `inject` | Y | Y |  |  | Y |  |  | unknown | 297&#8209;306 |
| 62 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;312 |
| 63 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 317&#8209;318 |
| 64 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;324 |
| 65 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;333 |
| 66 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 338&#8209;340 |
| 67 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 345&#8209;349 |
| 68 | `scan` | Y | Y |  |  | Y |  |  | unknown | 354&#8209;357 |
| 69 | `map` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;367 |
| 70 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 372&#8209;378 |
| 71 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 383&#8209;388 |
| 72 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 393&#8209;401 |
| 73 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 946&#8209;948 |
| 74 | `iter` |  |  | Y |  | Y |  |  | unknown | 952&#8209;956 |
| 75 | `next` |  | Y |  |  | Y |  |  | unknown | 992&#8209;1008 |
| 76 | `eq` |  | Y |  |  | Y |  |  | unknown | 1116&#8209;1117 |

### Chap19/ArraySeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 77 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 118&#8209;121 |
| 78 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 133&#8209;138 |
| 79 | `new` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;172 |
| 80 | `length` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;178 |
| 81 | `nth` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;185 |
| 82 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;198 |
| 83 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;211 |
| 84 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;219 |
| 85 | `empty` | Y | Y |  |  | Y |  |  | unknown | 224&#8209;225 |
| 86 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;235 |
| 87 | `append` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;248 |
| 88 | `filter` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;265 |
| 89 | `update` | Y | Y |  |  | Y |  |  | unknown | 270&#8209;278 |
| 90 | `inject` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;293 |
| 91 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;299 |
| 92 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 304&#8209;305 |
| 93 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;311 |
| 94 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;320 |
| 95 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 325&#8209;327 |
| 96 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 332&#8209;336 |
| 97 | `scan` | Y | Y |  |  | Y |  |  | unknown | 341&#8209;344 |
| 98 | `map` | Y | Y |  |  | Y |  |  | unknown | 349&#8209;354 |
| 99 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 359&#8209;365 |
| 100 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 370&#8209;375 |
| 101 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 380&#8209;388 |
| 102 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 955&#8209;957 |
| 103 | `iter` |  |  | Y |  | Y |  |  | unknown | 961&#8209;965 |
| 104 | `next` |  | Y |  |  | Y |  |  | unknown | 1001&#8209;1017 |
| 105 | `eq` |  | Y |  |  | Y |  |  | unknown | 1125&#8209;1126 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
