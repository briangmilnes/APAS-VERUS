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
| 1 | Chap40 | BSTKeyValueStEph | 27 | 28 | 0 | 8 | 36 | 0 | 35 | 1 | 0 |
| 2 | Chap40 | BSTReducedStEph | 36 | 37 | 0 | 2 | 39 | 0 | 36 | 3 | 0 |
| 3 | Chap40 | BSTSizeStEph | 31 | 32 | 0 | 4 | 36 | 0 | 35 | 1 | 0 |

## Function-by-Function Detail

### Chap40/BSTKeyValueStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `clone_link` |  |  |  | Y | Y |  |  | hole | 46&#8209;50 |
| 2 | `lemma_content_left_contains_key` |  |  |  | Y | Y |  |  | unknown | 170&#8209;174 |
| 3 | `lemma_content_right_contains_key` |  |  |  | Y | Y |  |  | unknown | 178&#8209;182 |
| 4 | `lemma_rotate_left_preserves_keys` |  |  |  | Y | Y |  |  | unknown | 187&#8209;198 |
| 5 | `lemma_rotate_right_preserves_keys` |  |  |  | Y | Y |  |  | unknown | 203&#8209;214 |
| 6 | `lemma_left_key_in_link` |  |  |  | Y | Y |  |  | unknown | 219&#8209;228 |
| 7 | `lemma_right_key_in_link` |  |  |  | Y | Y |  |  | unknown | 233&#8209;242 |
| 8 | `lemma_node_key_in_link` |  |  |  | Y | Y |  |  | unknown | 247&#8209;255 |
| 9 | `new` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;272 |
| 10 | `size` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;275 |
| 11 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 277&#8209;278 |
| 12 | `height` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;282 |
| 13 | `insert` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;289 |
| 14 | `delete` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;293 |
| 15 | `find` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;299 |
| 16 | `contains` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;305 |
| 17 | `get` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;311 |
| 18 | `keys` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;315 |
| 19 | `values` | Y | Y |  |  | Y |  |  | unknown | 317&#8209;319 |
| 20 | `minimum_key` | Y | Y |  |  | Y |  |  | unknown | 322&#8209;331 |
| 21 | `maximum_key` | Y | Y |  |  | Y |  |  | unknown | 334&#8209;343 |
| 22 | `new_node` | Y | Y |  |  | Y |  |  | unknown | 347&#8209;353 |
| 23 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 354&#8209;357 |
| 24 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 358&#8209;361 |
| 25 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;365 |
| 26 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 366&#8209;370 |
| 27 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 371&#8209;375 |
| 28 | `min_key_link` | Y | Y |  |  | Y |  |  | unknown | 376&#8209;385 |
| 29 | `max_key_link` | Y | Y |  |  | Y |  |  | unknown | 386&#8209;395 |
| 30 | `collect_keys` | Y | Y |  |  | Y |  |  | unknown | 396&#8209;398 |
| 31 | `collect_values` | Y | Y |  |  | Y |  |  | unknown | 399&#8209;401 |
| 32 | `collect_in_order_kvp` | Y | Y |  |  | Y |  |  | unknown | 402&#8209;404 |
| 33 | `find_min_priority_idx_kvp` | Y | Y |  |  | Y |  |  | unknown | 405&#8209;409 |
| 34 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 410&#8209;415 |
| 35 | `filter_by_key_kvp` | Y | Y |  |  | Y |  |  | unknown | 416&#8209;419 |
| 36 | `default` |  | Y |  |  | Y |  |  | unknown | 742&#8209;743 |

### Chap40/BSTReducedStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 37 | `clone_link` |  |  |  | Y | Y |  |  | hole | 43&#8209;48 |
| 38 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 246&#8209;250 |
| 39 | `identity` x2 | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 40 | `combine` x2 | Y | Y |  |  | Y |  |  | hole | 109&#8209;110 |
| 41 | `lift` x2 | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 42 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 226&#8209;235 |
| 43 | `size` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;253 |
| 44 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;256 |
| 45 | `height` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;260 |
| 46 | `insert` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;269 |
| 47 | `delete` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;275 |
| 48 | `find` | Y | Y |  |  | Y |  |  | unknown | 277&#8209;279 |
| 49 | `contains` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;283 |
| 50 | `get` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;287 |
| 51 | `keys` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;291 |
| 52 | `values` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;295 |
| 53 | `minimum_key` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;302 |
| 54 | `maximum_key` | Y | Y |  |  | Y |  |  | unknown | 305&#8209;309 |
| 55 | `reduced_value` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;314 |
| 56 | `range_reduce` | Y | Y |  |  | Y |  |  | unknown | 317&#8209;319 |
| 57 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;324 |
| 58 | `reduced_value_link` | Y | Y |  |  | Y |  |  | unknown | 327&#8209;328 |
| 59 | `update_node` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;342 |
| 60 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 345&#8209;355 |
| 61 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 357&#8209;363 |
| 62 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 365&#8209;371 |
| 63 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 373&#8209;381 |
| 64 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 382&#8209;384 |
| 65 | `min_key_link` | Y | Y |  |  | Y |  |  | unknown | 385&#8209;389 |
| 66 | `max_key_link` | Y | Y |  |  | Y |  |  | unknown | 390&#8209;394 |
| 67 | `collect_keys` | Y | Y |  |  | Y |  |  | unknown | 395&#8209;398 |
| 68 | `collect_values` | Y | Y |  |  | Y |  |  | unknown | 399&#8209;402 |
| 69 | `collect_in_order_kvp` | Y | Y |  |  | Y |  |  | unknown | 403&#8209;406 |
| 70 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 407&#8209;410 |
| 71 | `filter_by_key_kvp` | Y | Y |  |  | Y |  |  | unknown | 411&#8209;414 |
| 72 | `find_min_priority_idx_kvp` | Y | Y |  |  | Y |  |  | unknown | 415&#8209;419 |
| 73 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 420&#8209;427 |
| 74 | `range_reduce_link` | Y | Y |  |  | Y |  |  | unknown | 430&#8209;432 |
| 75 | `default` |  | Y |  |  | Y |  |  | unknown | 807&#8209;808 |

### Chap40/BSTSizeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 76 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 108&#8209;113 |
| 77 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 125&#8209;138 |
| 78 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 148&#8209;158 |
| 79 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 173&#8209;177 |
| 80 | `size` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;180 |
| 81 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;183 |
| 82 | `height` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;190 |
| 83 | `insert` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;199 |
| 84 | `delete` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;205 |
| 85 | `find` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;209 |
| 86 | `contains` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;213 |
| 87 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;219 |
| 88 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;225 |
| 89 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;229 |
| 90 | `rank` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;236 |
| 91 | `select` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;239 |
| 92 | `split_rank` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;245 |
| 93 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;250 |
| 94 | `update_size` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;259 |
| 95 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;265 |
| 96 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;272 |
| 97 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;279 |
| 98 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;288 |
| 99 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;291 |
| 100 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;296 |
| 101 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 297&#8209;301 |
| 102 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 302&#8209;307 |
| 103 | `in_order_collect` | Y | Y |  |  | Y |  |  | unknown | 308&#8209;311 |
| 104 | `in_order_collect_with_priority` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;315 |
| 105 | `find_min_priority_idx` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;318 |
| 106 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 319&#8209;324 |
| 107 | `filter_by_key` | Y | Y |  |  | Y |  |  | unknown | 325&#8209;326 |
| 108 | `rank_link` | Y | Y |  |  | Y |  |  | unknown | 327&#8209;332 |
| 109 | `select_link` | Y | Y |  |  | Y |  |  | unknown | 333&#8209;335 |
| 110 | `clone_link` |  |  |  | Y | Y |  |  | hole | 733&#8209;738 |
| 111 | `default` |  | Y |  |  | Y |  |  | unknown | 780&#8209;781 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
