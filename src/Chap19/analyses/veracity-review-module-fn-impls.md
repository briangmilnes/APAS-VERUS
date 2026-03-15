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
| 3 | Chap19 | ArraySeqStEph | 24 | 26 | 3 | 2 | 31 | 0 | 31 | 0 | 0 |
| 4 | Chap19 | ArraySeqStPer | 23 | 25 | 2 | 2 | 29 | 0 | 29 | 0 | 0 |

## Function-by-Function Detail

### Chap19/ArraySeqMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_spec_inject_element` |  |  |  | Y | Y |  |  | unknown | 123&#8209;132 |
| 2 | `lemma_spec_inject_len` |  |  |  | Y | Y |  |  | unknown | 163&#8209;165 |
| 3 | `new` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;197 |
| 4 | `set` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;208 |
| 5 | `length` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;214 |
| 6 | `nth` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;221 |
| 7 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;235 |
| 8 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;249 |
| 9 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;258 |
| 10 | `empty` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;264 |
| 11 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;275 |
| 12 | `append` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;289 |
| 13 | `filter` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;307 |
| 14 | `update` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;321 |
| 15 | `inject` | Y | Y |  |  | Y |  |  | unknown | 327&#8209;337 |
| 16 | `ninject` | Y | Y |  |  | Y |  |  | unknown | 343&#8209;352 |
| 17 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 357&#8209;358 |
| 18 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 363&#8209;364 |
| 19 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 369&#8209;370 |
| 20 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 375&#8209;379 |
| 21 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 384&#8209;386 |
| 22 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 391&#8209;395 |
| 23 | `scan` | Y | Y |  |  | Y |  |  | unknown | 400&#8209;403 |
| 24 | `map` | Y | Y |  |  | Y |  |  | unknown | 408&#8209;413 |
| 25 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 418&#8209;424 |
| 26 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 429&#8209;434 |
| 27 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 439&#8209;448 |
| 28 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 1008&#8209;1011 |
| 29 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 1023&#8209;1028 |
| 30 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 1041&#8209;1043 |
| 31 | `iter` |  |  | Y |  | Y |  |  | unknown | 1047&#8209;1051 |
| 32 | `map_par` |  |  | Y |  | Y |  |  | unknown | 1059&#8209;1070 |
| 33 | `filter_par` |  |  | Y |  | Y |  |  | unknown | 1112&#8209;1122 |
| 34 | `lemma_monoid_fold_left` |  |  | Y |  | Y |  |  | unknown | 1166&#8209;1169 |
| 35 | `reduce_par` |  |  | Y |  | Y |  |  | unknown | 1195&#8209;1210 |
| 36 | `next` |  | Y |  |  | Y |  |  | unknown | 1306&#8209;1322 |
| 37 | `eq` |  | Y |  |  | Y |  |  | unknown | 1425&#8209;1426 |

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
| 49 | `new` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;177 |
| 50 | `set` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;188 |
| 51 | `length` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;194 |
| 52 | `nth` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;201 |
| 53 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;215 |
| 54 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 220&#8209;229 |
| 55 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;238 |
| 56 | `empty` | Y | Y |  |  | Y |  |  | unknown | 243&#8209;244 |
| 57 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;255 |
| 58 | `append` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;269 |
| 59 | `filter` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;287 |
| 60 | `update` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;301 |
| 61 | `inject` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;317 |
| 62 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 322&#8209;323 |
| 63 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 328&#8209;329 |
| 64 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 334&#8209;335 |
| 65 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 340&#8209;344 |
| 66 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 349&#8209;351 |
| 67 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 356&#8209;360 |
| 68 | `scan` | Y | Y |  |  | Y |  |  | unknown | 365&#8209;368 |
| 69 | `map` | Y | Y |  |  | Y |  |  | unknown | 373&#8209;378 |
| 70 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 383&#8209;389 |
| 71 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 394&#8209;399 |
| 72 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 404&#8209;413 |
| 73 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 960&#8209;962 |
| 74 | `iter` |  |  | Y |  | Y |  |  | unknown | 966&#8209;970 |
| 75 | `lemma_view_index` |  |  | Y |  | Y |  |  | unknown | 978&#8209;980 |
| 76 | `next` |  | Y |  |  | Y |  |  | unknown | 1014&#8209;1030 |
| 77 | `eq` |  | Y |  |  | Y |  |  | unknown | 1138&#8209;1139 |

### Chap19/ArraySeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 78 | `lemma_flatten_bounded_by_outer_len` |  |  |  | Y | Y |  |  | unknown | 118&#8209;121 |
| 79 | `lemma_flatten_all_satisfy` |  |  |  | Y | Y |  |  | unknown | 133&#8209;138 |
| 80 | `new` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;175 |
| 81 | `length` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;181 |
| 82 | `nth` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;188 |
| 83 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;202 |
| 84 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;216 |
| 85 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;225 |
| 86 | `empty` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;231 |
| 87 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;242 |
| 88 | `append` | Y | Y |  |  | Y |  |  | unknown | 247&#8209;256 |
| 89 | `filter` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;274 |
| 90 | `update` | Y | Y |  |  | Y |  |  | unknown | 279&#8209;288 |
| 91 | `inject` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;304 |
| 92 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;310 |
| 93 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;316 |
| 94 | `iterate_iter` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;322 |
| 95 | `iterate` | Y | Y |  |  | Y |  |  | unknown | 327&#8209;331 |
| 96 | `reduce_iter` | Y | Y |  |  | Y |  |  | unknown | 336&#8209;338 |
| 97 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 343&#8209;347 |
| 98 | `scan` | Y | Y |  |  | Y |  |  | unknown | 352&#8209;355 |
| 99 | `map` | Y | Y |  |  | Y |  |  | unknown | 360&#8209;365 |
| 100 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 370&#8209;376 |
| 101 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 381&#8209;386 |
| 102 | `deflate` | Y | Y |  |  | Y |  |  | unknown | 391&#8209;400 |
| 103 | `lemma_spec_index` |  |  | Y |  | Y |  |  | unknown | 969&#8209;971 |
| 104 | `iter` |  |  | Y |  | Y |  |  | unknown | 975&#8209;979 |
| 105 | `next` |  | Y |  |  | Y |  |  | unknown | 1015&#8209;1031 |
| 106 | `eq` |  | Y |  |  | Y |  |  | unknown | 1139&#8209;1140 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
