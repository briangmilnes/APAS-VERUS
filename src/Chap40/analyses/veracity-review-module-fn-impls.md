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
| 1 | Chap40 | BSTKeyValueStEph | 26 | 28 | 0 | 9 | 37 | 0 | 34 | 2 | 1 |
| 2 | Chap40 | BSTReducedStEph | 36 | 38 | 0 | 3 | 41 | 0 | 36 | 4 | 1 |
| 3 | Chap40 | BSTSizeStEph | 31 | 33 | 0 | 5 | 38 | 0 | 35 | 2 | 1 |

## Function-by-Function Detail

### Chap40/BSTKeyValueStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_content_left_contains_key` |  |  |  | Y | Y |  |  | unknown | 138&#8209;142 |
| 2 | `lemma_content_right_contains_key` |  |  |  | Y | Y |  |  | unknown | 146&#8209;150 |
| 3 | `lemma_rotate_left_preserves_keys` |  |  |  | Y | Y |  |  | unknown | 155&#8209;166 |
| 4 | `lemma_rotate_right_preserves_keys` |  |  |  | Y | Y |  |  | unknown | 171&#8209;182 |
| 5 | `lemma_left_key_in_link` |  |  |  | Y | Y |  |  | unknown | 187&#8209;196 |
| 6 | `lemma_right_key_in_link` |  |  |  | Y | Y |  |  | unknown | 201&#8209;210 |
| 7 | `lemma_node_key_in_link` |  |  |  | Y | Y |  |  | unknown | 215&#8209;223 |
| 8 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 249&#8209;252 |
| 9 | `size` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;255 |
| 10 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;258 |
| 11 | `height` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;262 |
| 12 | `insert` | Y | Y |  |  | Y |  |  | unknown | 264&#8209;269 |
| 13 | `delete` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;273 |
| 14 | `find` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;279 |
| 15 | `contains` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;285 |
| 16 | `get` | Y | Y |  |  | Y |  |  | unknown | 287&#8209;291 |
| 17 | `keys` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;295 |
| 18 | `values` | Y | Y |  |  | Y |  |  | unknown | 297&#8209;299 |
| 19 | `minimum_key` | Y | Y |  |  | Y |  |  | unknown | 302&#8209;311 |
| 20 | `maximum_key` | Y | Y |  |  | Y |  |  | unknown | 314&#8209;323 |
| 21 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 327&#8209;330 |
| 22 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;334 |
| 23 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;338 |
| 24 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 339&#8209;343 |
| 25 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 344&#8209;348 |
| 26 | `min_key_link` | Y | Y |  |  | Y |  |  | unknown | 349&#8209;358 |
| 27 | `max_key_link` | Y | Y |  |  | Y |  |  | unknown | 359&#8209;368 |
| 28 | `collect_keys` | Y | Y |  |  | Y |  |  | unknown | 369&#8209;371 |
| 29 | `collect_values` | Y | Y |  |  | Y |  |  | unknown | 372&#8209;374 |
| 30 | `collect_in_order_kvp` | Y | Y |  |  | Y |  |  | unknown | 375&#8209;377 |
| 31 | `find_min_priority_idx_kvp` | Y | Y |  |  | Y |  |  | unknown | 378&#8209;382 |
| 32 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 383&#8209;388 |
| 33 | `filter_by_key_kvp` | Y | Y |  |  | Y |  |  | unknown | 389&#8209;392 |
| 34 | `clone_link` |  |  |  | Y | Y |  |  | hole | 397&#8209;401 |
| 35 | `compare_kv_links` |  |  |  | Y | Y |  | Y |  | 477&#8209;478 |
| 36 | `default` |  | Y |  |  | Y |  |  | unknown | 805&#8209;806 |
| 37 | `eq` |  | Y |  |  | Y |  |  | hole | 846&#8209;847 |

### Chap40/BSTReducedStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 127&#8209;136 |
| 39 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 174&#8209;178 |
| 40 | `identity` x2 | Y | Y |  |  | Y |  |  | hole | 160&#8209;161 |
| 41 | `combine` x2 | Y | Y |  |  | Y |  |  | hole | 162&#8209;163 |
| 42 | `lift` x2 | Y | Y |  |  | Y |  |  | unknown | 164&#8209;165 |
| 43 | `size` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;181 |
| 44 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;184 |
| 45 | `height` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;188 |
| 46 | `insert` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;197 |
| 47 | `delete` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;203 |
| 48 | `find` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;207 |
| 49 | `contains` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;211 |
| 50 | `get` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;215 |
| 51 | `keys` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;219 |
| 52 | `values` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;223 |
| 53 | `minimum_key` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;230 |
| 54 | `maximum_key` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;237 |
| 55 | `reduced_value` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;242 |
| 56 | `range_reduce` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;247 |
| 57 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;252 |
| 58 | `reduced_value_link` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;256 |
| 59 | `update_node` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;270 |
| 60 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;283 |
| 61 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;291 |
| 62 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;299 |
| 63 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;309 |
| 64 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;312 |
| 65 | `min_key_link` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;317 |
| 66 | `max_key_link` | Y | Y |  |  | Y |  |  | unknown | 318&#8209;322 |
| 67 | `collect_keys` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;326 |
| 68 | `collect_values` | Y | Y |  |  | Y |  |  | unknown | 327&#8209;330 |
| 69 | `collect_in_order_kvp` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;334 |
| 70 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;338 |
| 71 | `filter_by_key_kvp` | Y | Y |  |  | Y |  |  | unknown | 339&#8209;342 |
| 72 | `find_min_priority_idx_kvp` | Y | Y |  |  | Y |  |  | unknown | 343&#8209;347 |
| 73 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 348&#8209;355 |
| 74 | `range_reduce_link` | Y | Y |  |  | Y |  |  | unknown | 358&#8209;360 |
| 75 | `clone_link` |  |  |  | Y | Y |  |  | hole | 366&#8209;371 |
| 76 | `compare_reduced_links` |  |  |  | Y | Y |  | Y |  | 466&#8209;467 |
| 77 | `default` |  | Y |  |  | Y |  |  | unknown | 846&#8209;847 |
| 78 | `eq` |  | Y |  |  | Y |  |  | hole | 892&#8209;893 |

### Chap40/BSTSizeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 79 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 117&#8209;122 |
| 80 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 134&#8209;147 |
| 81 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 157&#8209;167 |
| 82 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 191&#8209;195 |
| 83 | `size` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;198 |
| 84 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;201 |
| 85 | `height` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;208 |
| 86 | `insert` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;217 |
| 87 | `delete` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;223 |
| 88 | `find` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;227 |
| 89 | `contains` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;231 |
| 90 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;237 |
| 91 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;243 |
| 92 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;247 |
| 93 | `rank` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;254 |
| 94 | `select` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;257 |
| 95 | `split_rank` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;263 |
| 96 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;268 |
| 97 | `update_size` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;277 |
| 98 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 278&#8209;283 |
| 99 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;290 |
| 100 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;297 |
| 101 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;306 |
| 102 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;309 |
| 103 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;314 |
| 104 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;319 |
| 105 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;325 |
| 106 | `in_order_collect` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;329 |
| 107 | `in_order_collect_with_priority` | Y | Y |  |  | Y |  |  | unknown | 330&#8209;333 |
| 108 | `find_min_priority_idx` | Y | Y |  |  | Y |  |  | unknown | 334&#8209;336 |
| 109 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 337&#8209;342 |
| 110 | `filter_by_key` | Y | Y |  |  | Y |  |  | unknown | 343&#8209;344 |
| 111 | `rank_link` | Y | Y |  |  | Y |  |  | unknown | 345&#8209;350 |
| 112 | `select_link` | Y | Y |  |  | Y |  |  | unknown | 351&#8209;353 |
| 113 | `compare_links` |  |  |  | Y | Y |  | Y |  | 406&#8209;407 |
| 114 | `clone_link` |  |  |  | Y | Y |  |  | hole | 803&#8209;808 |
| 115 | `default` |  | Y |  |  | Y |  |  | unknown | 827&#8209;828 |
| 116 | `eq` |  | Y |  |  | Y |  |  | hole | 865&#8209;866 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
