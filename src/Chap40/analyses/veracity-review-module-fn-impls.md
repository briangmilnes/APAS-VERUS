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
| 1 | Chap40 | BSTKeyValueStEph | 27 | 29 | 1 | 16 | 46 | 0 | 46 | 0 | 0 |
| 2 | Chap40 | BSTReducedStEph | 37 | 39 | 1 | 10 | 50 | 0 | 48 | 2 | 0 |
| 3 | Chap40 | BSTSizeStEph | 32 | 34 | 1 | 10 | 45 | 0 | 45 | 0 | 0 |

## Function-by-Function Detail

### Chap40/BSTKeyValueStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 185&#8209;189 |
| 2 | `size` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;193 |
| 3 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;197 |
| 4 | `height` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;202 |
| 5 | `insert` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;214 |
| 6 | `delete` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;222 |
| 7 | `find` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;229 |
| 8 | `contains` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;234 |
| 9 | `get` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;241 |
| 10 | `keys` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;246 |
| 11 | `values` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;251 |
| 12 | `minimum_key` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;263 |
| 13 | `maximum_key` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;275 |
| 14 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;283 |
| 15 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;296 |
| 16 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;309 |
| 17 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;318 |
| 18 | `delete_link` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;326 |
| 19 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 328&#8209;336 |
| 20 | `min_key_link` | Y | Y |  |  | Y |  |  | unknown | 338&#8209;347 |
| 21 | `max_key_link` | Y | Y |  |  | Y |  |  | unknown | 349&#8209;358 |
| 22 | `collect_keys` | Y | Y |  |  | Y |  |  | unknown | 360&#8209;362 |
| 23 | `collect_values` | Y | Y |  |  | Y |  |  | unknown | 364&#8209;366 |
| 24 | `collect_in_order_kvp` | Y | Y |  |  | Y |  |  | unknown | 368&#8209;370 |
| 25 | `find_min_priority_idx_kvp` | Y | Y |  |  | Y |  |  | unknown | 372&#8209;376 |
| 26 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 378&#8209;383 |
| 27 | `filter_by_key_kvp` | Y | Y |  |  | Y |  |  | unknown | 385&#8209;388 |
| 28 | `lemma_rotate_left_content_eq` |  |  |  | Y | Y |  |  | unknown | 1166&#8209;1177 |
| 29 | `lemma_rotate_right_content_eq` |  |  |  | Y | Y |  |  | unknown | 1188&#8209;1199 |
| 30 | `lemma_insert_left_commutes` |  |  |  | Y | Y |  |  | unknown | 1210&#8209;1218 |
| 31 | `lemma_insert_right_commutes` |  |  |  | Y | Y |  |  | unknown | 1223&#8209;1231 |
| 32 | `lemma_content_left_contains_key` |  |  |  | Y | Y |  |  | unknown | 1234&#8209;1238 |
| 33 | `lemma_content_right_contains_key` |  |  |  | Y | Y |  |  | unknown | 1242&#8209;1246 |
| 34 | `lemma_rotate_left_preserves_keys` |  |  |  | Y | Y |  |  | unknown | 1251&#8209;1262 |
| 35 | `lemma_rotate_right_preserves_keys` |  |  |  | Y | Y |  |  | unknown | 1267&#8209;1278 |
| 36 | `lemma_left_key_in_link` |  |  |  | Y | Y |  |  | unknown | 1283&#8209;1292 |
| 37 | `lemma_right_key_in_link` |  |  |  | Y | Y |  |  | unknown | 1297&#8209;1306 |
| 38 | `lemma_node_key_in_link` |  |  |  | Y | Y |  |  | unknown | 1311&#8209;1319 |
| 39 | `lemma_ordered_assemble_kv` |  |  |  | Y | Y |  |  | unknown | 1323&#8209;1336 |
| 40 | `lemma_strict_lt_transitive` |  |  |  | Y | Y |  |  | unknown | 1341&#8209;1346 |
| 41 | `lemma_strict_gt_transitive` |  |  |  | Y | Y |  |  | unknown | 1355&#8209;1360 |
| 42 | `clone_link` |  |  |  | Y | Y |  |  | unknown | 1381&#8209;1386 |
| 43 | `compare_kv_links` |  |  |  | Y | Y |  |  | unknown | 1450&#8209;1456 |
| 44 | `iter` |  |  | Y |  | Y |  |  | unknown | 1497&#8209;1502 |
| 45 | `default` |  | Y |  |  | Y |  |  | unknown | 1528&#8209;1529 |
| 46 | `eq` |  | Y |  |  | Y |  |  | unknown | 1560&#8209;1561 |

### Chap40/BSTReducedStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 47 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 239&#8209;243 |
| 48 | `identity` x2 | Y | Y |  |  | Y |  |  | hole | 1477&#8209;1478 |
| 49 | `combine` x2 | Y | Y |  |  | Y |  |  | hole | 1480&#8209;1481 |
| 50 | `lift` x2 | Y | Y |  |  | Y |  |  | unknown | 1483&#8209;1484 |
| 51 | `size` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;247 |
| 52 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;251 |
| 53 | `height` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;256 |
| 54 | `insert` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;269 |
| 55 | `delete` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;280 |
| 56 | `find` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;290 |
| 57 | `contains` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;298 |
| 58 | `get` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;308 |
| 59 | `keys` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;313 |
| 60 | `values` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;318 |
| 61 | `minimum_key` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;325 |
| 62 | `maximum_key` | Y | Y |  |  | Y |  |  | unknown | 328&#8209;332 |
| 63 | `reduced_value` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;337 |
| 64 | `range_reduce` | Y | Y |  |  | Y |  |  | unknown | 340&#8209;342 |
| 65 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 347&#8209;348 |
| 66 | `reduced_value_link` | Y | Y |  |  | Y |  |  | unknown | 351&#8209;352 |
| 67 | `update_node` | Y | Y |  |  | Y |  |  | unknown | 355&#8209;367 |
| 68 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 370&#8209;380 |
| 69 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 383&#8209;399 |
| 70 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 402&#8209;418 |
| 71 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 421&#8209;434 |
| 72 | `delete_link` | Y | Y |  |  | Y |  |  | unknown | 436&#8209;448 |
| 73 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 450&#8209;460 |
| 74 | `min_key_link` | Y | Y |  |  | Y |  |  | unknown | 462&#8209;466 |
| 75 | `max_key_link` | Y | Y |  |  | Y |  |  | unknown | 468&#8209;472 |
| 76 | `collect_keys` | Y | Y |  |  | Y |  |  | unknown | 474&#8209;477 |
| 77 | `collect_values` | Y | Y |  |  | Y |  |  | unknown | 479&#8209;482 |
| 78 | `collect_in_order_kvp` | Y | Y |  |  | Y |  |  | unknown | 484&#8209;487 |
| 79 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 489&#8209;492 |
| 80 | `filter_by_key_kvp` | Y | Y |  |  | Y |  |  | unknown | 494&#8209;497 |
| 81 | `find_min_priority_idx_kvp` | Y | Y |  |  | Y |  |  | unknown | 499&#8209;503 |
| 82 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 505&#8209;512 |
| 83 | `range_reduce_link` | Y | Y |  |  | Y |  |  | unknown | 515&#8209;517 |
| 84 | `lemma_ordered_assemble_reduced` |  |  |  | Y | Y |  |  | unknown | 1349&#8209;1362 |
| 85 | `lemma_cmp_antisymmetry_reduced` |  |  |  | Y | Y |  |  | unknown | 1366&#8209;1371 |
| 86 | `lemma_cmp_antisymmetry_lt_reduced` |  |  |  | Y | Y |  |  | unknown | 1378&#8209;1383 |
| 87 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 1389&#8209;1398 |
| 88 | `lemma_cmp_transitivity_lt_reduced` |  |  |  | Y | Y |  |  | unknown | 1402&#8209;1408 |
| 89 | `lemma_cmp_transitivity_gt_reduced` |  |  |  | Y | Y |  |  | unknown | 1415&#8209;1421 |
| 90 | `lemma_rotate_left_content_eq_reduced` |  |  |  | Y | Y |  |  | unknown | 1428&#8209;1439 |
| 91 | `lemma_rotate_right_content_eq_reduced` |  |  |  | Y | Y |  |  | unknown | 1444&#8209;1455 |
| 92 | `clone_link` |  |  |  | Y | Y |  |  | unknown | 1554&#8209;1560 |
| 93 | `compare_reduced_links` |  |  |  | Y | Y |  |  | unknown | 1583&#8209;1589 |
| 94 | `iter` |  |  | Y |  | Y |  |  | unknown | 1628&#8209;1633 |
| 95 | `default` |  | Y |  |  | Y |  |  | unknown | 1661&#8209;1662 |
| 96 | `eq` |  | Y |  |  | Y |  |  | unknown | 1701&#8209;1702 |

### Chap40/BSTSizeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 97 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 173&#8209;177 |
| 98 | `size` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;181 |
| 99 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;185 |
| 100 | `height` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;193 |
| 101 | `insert` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;206 |
| 102 | `delete` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;217 |
| 103 | `find` | Y | Y |  |  | Y |  |  | unknown | 220&#8209;227 |
| 104 | `contains` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;235 |
| 105 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;242 |
| 106 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;249 |
| 107 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;254 |
| 108 | `rank` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;262 |
| 109 | `select` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;266 |
| 110 | `split_rank` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;273 |
| 111 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 278&#8209;279 |
| 112 | `update_size` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;289 |
| 113 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;297 |
| 114 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;315 |
| 115 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 317&#8209;333 |
| 116 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;348 |
| 117 | `delete_link` | Y | Y |  |  | Y |  |  | unknown | 350&#8209;362 |
| 118 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 364&#8209;374 |
| 119 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 376&#8209;380 |
| 120 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 382&#8209;386 |
| 121 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 388&#8209;393 |
| 122 | `in_order_collect` | Y | Y |  |  | Y |  |  | unknown | 395&#8209;398 |
| 123 | `in_order_collect_with_priority` | Y | Y |  |  | Y |  |  | unknown | 400&#8209;403 |
| 124 | `find_min_priority_idx` | Y | Y |  |  | Y |  |  | unknown | 405&#8209;407 |
| 125 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 409&#8209;414 |
| 126 | `filter_by_key` | Y | Y |  |  | Y |  |  | unknown | 416&#8209;417 |
| 127 | `rank_link` | Y | Y |  |  | Y |  |  | unknown | 419&#8209;424 |
| 128 | `select_link` | Y | Y |  |  | Y |  |  | unknown | 426&#8209;428 |
| 129 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 1215&#8209;1220 |
| 130 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 1232&#8209;1245 |
| 131 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 1254&#8209;1264 |
| 132 | `lemma_ordered_assemble` |  |  |  | Y | Y |  |  | unknown | 1267&#8209;1280 |
| 133 | `lemma_cmp_antisymmetry` |  |  |  | Y | Y |  |  | unknown | 1284&#8209;1289 |
| 134 | `lemma_cmp_antisymmetry_lt` |  |  |  | Y | Y |  |  | unknown | 1296&#8209;1301 |
| 135 | `lemma_cmp_transitivity_lt` |  |  |  | Y | Y |  |  | unknown | 1308&#8209;1314 |
| 136 | `lemma_cmp_transitivity_gt` |  |  |  | Y | Y |  |  | unknown | 1321&#8209;1327 |
| 137 | `compare_links` |  |  |  | Y | Y |  |  | unknown | 1413&#8209;1419 |
| 138 | `clone_link` |  |  |  | Y | Y |  |  | unknown | 1436&#8209;1442 |
| 139 | `iter` |  |  | Y |  | Y |  |  | unknown | 1487&#8209;1492 |
| 140 | `default` |  | Y |  |  | Y |  |  | unknown | 1517&#8209;1518 |
| 141 | `eq` |  | Y |  |  | Y |  |  | unknown | 1547&#8209;1548 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
