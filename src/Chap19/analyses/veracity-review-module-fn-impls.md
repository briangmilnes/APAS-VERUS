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
| 1 | Chap19 | ArraySeqMtEph | 25 | 27 | 6 | 4 | 37 | 0 | 36 | 1 | 0 |
| 2 | Chap19 | ArraySeqMtEphSlice | 8 | 8 | 1 | 0 | 9 | 0 | 9 | 0 | 0 |
| 3 | Chap19 | ArraySeqStEph | 24 | 26 | 3 | 2 | 31 | 0 | 30 | 1 | 0 |
| 4 | Chap19 | ArraySeqStPer | 23 | 25 | 3 | 2 | 30 | 0 | 29 | 1 | 0 |

## Function-by-Function Detail

### Chap19/ArraySeqMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_spec_inject_element` |  |  |  | Y | Y |  |  | unknown | 122&#8209;131 |
| 2 | `lemma_spec_inject_len` |  |  |  | Y | Y |  |  | unknown | 162&#8209;164 |
| 3 | `new` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;196 |
| 4 | `set` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;207 |
| 5 | `length` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;213 |
| 6 | `nth` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;220 |
| 7 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;234 |
| 8 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;248 |
| 9 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;257 |
| 10 | `empty` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;263 |
| 11 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;274 |
| 12 | `append` | Y | Y |  |  | Y |  |  | unknown | 279&#8209;288 |
| 13 | `filter` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;306 |
| 14 | `update` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;320 |
| 15 | `inject` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;336 |
| 16 | `ninject` | Y | Y |  |  | Y |  |  | unknown | 342&#8209;351 |
| 17 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 356&#8209;357 |
| 18 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;363 |
| 19 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 368&#8209;373 |
| 20 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 378&#8209;385 |
| 21 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 390&#8209;397 |
| 22 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 402&#8209;410 |
| 23 | `scan` | Y | Y |  |  | Y |  |  | unknown | 415&#8209;425 |
| 24 | `map` | Y | Y |  |  | Y |  |  | unknown | 430&#8209;435 |
| 25 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 440&#8209;446 |
| 26 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 451&#8209;456 |
| 27 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 461&#8209;470 |
| 28 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 1059&#8209;1062 |
| 29 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 1074&#8209;1079 |
| 30 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1092&#8209;1094 |
| 31 | `iter` |  |  | Y |  | Y |  |  | unknown | 1098&#8209;1102 |
| 32 | `map_par` |  |  | Y |  | Y |  |  | unknown | 1110&#8209;1121 |
| 33 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 1163&#8209;1173 |
| 34 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 1217&#8209;1220 |
| 35 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 1246&#8209;1261 |
| 36 | `next` |  | Y |  |  | Y |  |  | unknown | 1357&#8209;1373 |
| 37 | `eq` |  | Y |  |  | Y |  |  | hole | 1476&#8209;1477 |

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
| 47 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 125&#8209;128 |
| 48 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 140&#8209;145 |
| 49 | `new` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;184 |
| 50 | `set` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;195 |
| 51 | `length` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;201 |
| 52 | `nth` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;208 |
| 53 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;222 |
| 54 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;236 |
| 55 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;245 |
| 56 | `empty` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;251 |
| 57 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;262 |
| 58 | `append` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;276 |
| 59 | `filter` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;294 |
| 60 | `update` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;308 |
| 61 | `inject` | Y | Y |  |  | Y |  |  | unknown | 314&#8209;324 |
| 62 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;330 |
| 63 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;336 |
| 64 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 341&#8209;346 |
| 65 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 351&#8209;358 |
| 66 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 363&#8209;370 |
| 67 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 375&#8209;383 |
| 68 | `scan` | Y | Y |  |  | Y |  |  | unknown | 388&#8209;398 |
| 69 | `map` | Y | Y |  |  | Y |  |  | unknown | 403&#8209;408 |
| 70 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 413&#8209;419 |
| 71 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 424&#8209;429 |
| 72 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 434&#8209;443 |
| 73 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1019&#8209;1021 |
| 74 | `iter` |  |  | Y |  | Y |  |  | unknown | 1025&#8209;1029 |
| 75 | `lemma_view_index` |  |  | Y |  | Y |  |  | unknown | 1037&#8209;1039 |
| 76 | `next` |  | Y |  |  | Y |  |  | unknown | 1073&#8209;1089 |
| 77 | `eq` |  | Y |  |  | Y |  |  | hole | 1197&#8209;1198 |

### Chap19/ArraySeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 78 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 124&#8209;127 |
| 79 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 139&#8209;144 |
| 80 | `new` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;181 |
| 81 | `length` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;187 |
| 82 | `nth` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;194 |
| 83 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;208 |
| 84 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;222 |
| 85 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;231 |
| 86 | `empty` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;237 |
| 87 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 242&#8209;248 |
| 88 | `append` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;262 |
| 89 | `filter` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;280 |
| 90 | `update` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;294 |
| 91 | `inject` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;310 |
| 92 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;316 |
| 93 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;322 |
| 94 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 327&#8209;332 |
| 95 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 337&#8209;344 |
| 96 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 349&#8209;356 |
| 97 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 361&#8209;369 |
| 98 | `scan` | Y | Y |  |  | Y |  |  | unknown | 374&#8209;384 |
| 99 | `map` | Y | Y |  |  | Y |  |  | unknown | 389&#8209;394 |
| 100 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 399&#8209;405 |
| 101 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 410&#8209;415 |
| 102 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 420&#8209;429 |
| 103 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1027&#8209;1029 |
| 104 | `lemma_view_index` |  |  | Y |  | Y |  |  | unknown | 1035&#8209;1037 |
| 105 | `iter` |  |  | Y |  | Y |  |  | unknown | 1041&#8209;1045 |
| 106 | `next` |  | Y |  |  | Y |  |  | unknown | 1081&#8209;1097 |
| 107 | `eq` |  | Y |  |  | Y |  |  | hole | 1205&#8209;1206 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
