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
| 1 | Chap40 | BSTKeyValueStEph | 27 | 29 | 0 | 16 | 45 | 0 | 43 | 2 | 0 |
| 2 | Chap40 | BSTReducedStEph | 37 | 39 | 0 | 10 | 49 | 0 | 45 | 4 | 0 |
| 3 | Chap40 | BSTSizeStEph | 32 | 34 | 0 | 10 | 44 | 0 | 42 | 2 | 0 |

## Function-by-Function Detail

### Chap40/BSTKeyValueStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_rotate_left_content_eq` |  |  |  | Y | Y |  |  | unknown | 137&#8209;148 |
| 2 | `lemma_rotate_right_content_eq` |  |  |  | Y | Y |  |  | unknown | 167&#8209;178 |
| 3 | `lemma_insert_left_commutes` |  |  |  | Y | Y |  |  | unknown | 197&#8209;205 |
| 4 | `lemma_insert_right_commutes` |  |  |  | Y | Y |  |  | unknown | 210&#8209;218 |
| 5 | `lemma_content_left_contains_key` |  |  |  | Y | Y |  |  | unknown | 221&#8209;225 |
| 6 | `lemma_content_right_contains_key` |  |  |  | Y | Y |  |  | unknown | 229&#8209;233 |
| 7 | `lemma_rotate_left_preserves_keys` |  |  |  | Y | Y |  |  | unknown | 238&#8209;249 |
| 8 | `lemma_rotate_right_preserves_keys` |  |  |  | Y | Y |  |  | unknown | 254&#8209;265 |
| 9 | `lemma_left_key_in_link` |  |  |  | Y | Y |  |  | unknown | 270&#8209;279 |
| 10 | `lemma_right_key_in_link` |  |  |  | Y | Y |  |  | unknown | 284&#8209;293 |
| 11 | `lemma_node_key_in_link` |  |  |  | Y | Y |  |  | unknown | 298&#8209;306 |
| 12 | `lemma_ordered_assemble_kv` |  |  |  | Y | Y |  |  | unknown | 310&#8209;323 |
| 13 | `lemma_strict_lt_transitive` |  |  |  | Y | Y |  |  | unknown | 328&#8209;333 |
| 14 | `lemma_strict_gt_transitive` |  |  |  | Y | Y |  |  | unknown | 342&#8209;347 |
| 15 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 381&#8209;385 |
| 16 | `size` | Y | Y |  |  | Y |  |  | unknown | 387&#8209;388 |
| 17 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 390&#8209;391 |
| 18 | `height` | Y | Y |  |  | Y |  |  | unknown | 393&#8209;395 |
| 19 | `insert` | Y | Y |  |  | Y |  |  | unknown | 397&#8209;406 |
| 20 | `delete` | Y | Y |  |  | Y |  |  | unknown | 408&#8209;413 |
| 21 | `find` | Y | Y |  |  | Y |  |  | unknown | 415&#8209;419 |
| 22 | `contains` | Y | Y |  |  | Y |  |  | unknown | 421&#8209;423 |
| 23 | `get` | Y | Y |  |  | Y |  |  | unknown | 425&#8209;429 |
| 24 | `keys` | Y | Y |  |  | Y |  |  | unknown | 431&#8209;433 |
| 25 | `values` | Y | Y |  |  | Y |  |  | unknown | 435&#8209;437 |
| 26 | `minimum_key` | Y | Y |  |  | Y |  |  | unknown | 440&#8209;449 |
| 27 | `maximum_key` | Y | Y |  |  | Y |  |  | unknown | 452&#8209;461 |
| 28 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 465&#8209;468 |
| 29 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 469&#8209;480 |
| 30 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 481&#8209;492 |
| 31 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 493&#8209;500 |
| 32 | `delete_link` | Y | Y |  |  | Y |  |  | unknown | 501&#8209;507 |
| 33 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 508&#8209;516 |
| 34 | `min_key_link` | Y | Y |  |  | Y |  |  | unknown | 517&#8209;526 |
| 35 | `max_key_link` | Y | Y |  |  | Y |  |  | unknown | 527&#8209;536 |
| 36 | `collect_keys` | Y | Y |  |  | Y |  |  | unknown | 537&#8209;539 |
| 37 | `collect_values` | Y | Y |  |  | Y |  |  | unknown | 540&#8209;542 |
| 38 | `collect_in_order_kvp` | Y | Y |  |  | Y |  |  | unknown | 543&#8209;545 |
| 39 | `find_min_priority_idx_kvp` | Y | Y |  |  | Y |  |  | unknown | 546&#8209;550 |
| 40 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 551&#8209;556 |
| 41 | `filter_by_key_kvp` | Y | Y |  |  | Y |  |  | unknown | 557&#8209;560 |
| 42 | `clone_link` |  |  |  | Y | Y |  |  | hole | 565&#8209;570 |
| 43 | `compare_kv_links` |  |  |  | Y | Y |  |  | unknown | 687&#8209;693 |
| 44 | `default` |  | Y |  |  | Y |  |  | unknown | 1498&#8209;1499 |
| 45 | `eq` |  | Y |  |  | Y |  |  | hole | 1541&#8209;1542 |

### Chap40/BSTReducedStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 46 | `lemma_ordered_assemble_reduced` |  |  |  | Y | Y |  |  | unknown | 83&#8209;96 |
| 47 | `lemma_cmp_antisymmetry_reduced` |  |  |  | Y | Y |  |  | unknown | 100&#8209;105 |
| 48 | `lemma_cmp_antisymmetry_lt_reduced` |  |  |  | Y | Y |  |  | unknown | 112&#8209;117 |
| 49 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 123&#8209;132 |
| 50 | `lemma_cmp_transitivity_lt_reduced` |  |  |  | Y | Y |  |  | unknown | 136&#8209;142 |
| 51 | `lemma_cmp_transitivity_gt_reduced` |  |  |  | Y | Y |  |  | unknown | 149&#8209;155 |
| 52 | `lemma_rotate_left_content_eq_reduced` |  |  |  | Y | Y |  |  | unknown | 162&#8209;173 |
| 53 | `lemma_rotate_right_content_eq_reduced` |  |  |  | Y | Y |  |  | unknown | 186&#8209;197 |
| 54 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 276&#8209;280 |
| 55 | `identity` x2 | Y | Y |  |  | Y |  |  | hole | 262&#8209;263 |
| 56 | `combine` x2 | Y | Y |  |  | Y |  |  | hole | 264&#8209;265 |
| 57 | `lift` x2 | Y | Y |  |  | Y |  |  | unknown | 266&#8209;267 |
| 58 | `size` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;283 |
| 59 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;286 |
| 60 | `height` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;290 |
| 61 | `insert` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;302 |
| 62 | `delete` | Y | Y |  |  | Y |  |  | unknown | 304&#8209;312 |
| 63 | `find` | Y | Y |  |  | Y |  |  | unknown | 314&#8209;321 |
| 64 | `contains` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;328 |
| 65 | `get` | Y | Y |  |  | Y |  |  | unknown | 330&#8209;337 |
| 66 | `keys` | Y | Y |  |  | Y |  |  | unknown | 339&#8209;341 |
| 67 | `values` | Y | Y |  |  | Y |  |  | unknown | 343&#8209;345 |
| 68 | `minimum_key` | Y | Y |  |  | Y |  |  | unknown | 348&#8209;352 |
| 69 | `maximum_key` | Y | Y |  |  | Y |  |  | unknown | 355&#8209;359 |
| 70 | `reduced_value` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;364 |
| 71 | `range_reduce` | Y | Y |  |  | Y |  |  | unknown | 367&#8209;369 |
| 72 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 373&#8209;374 |
| 73 | `reduced_value_link` | Y | Y |  |  | Y |  |  | unknown | 377&#8209;378 |
| 74 | `update_node` | Y | Y |  |  | Y |  |  | unknown | 381&#8209;393 |
| 75 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 396&#8209;406 |
| 76 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 408&#8209;424 |
| 77 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 426&#8209;442 |
| 78 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 444&#8209;457 |
| 79 | `delete_link` | Y | Y |  |  | Y |  |  | unknown | 458&#8209;470 |
| 80 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 471&#8209;481 |
| 81 | `min_key_link` | Y | Y |  |  | Y |  |  | unknown | 482&#8209;486 |
| 82 | `max_key_link` | Y | Y |  |  | Y |  |  | unknown | 487&#8209;491 |
| 83 | `collect_keys` | Y | Y |  |  | Y |  |  | unknown | 492&#8209;495 |
| 84 | `collect_values` | Y | Y |  |  | Y |  |  | unknown | 496&#8209;499 |
| 85 | `collect_in_order_kvp` | Y | Y |  |  | Y |  |  | unknown | 500&#8209;503 |
| 86 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 504&#8209;507 |
| 87 | `filter_by_key_kvp` | Y | Y |  |  | Y |  |  | unknown | 508&#8209;511 |
| 88 | `find_min_priority_idx_kvp` | Y | Y |  |  | Y |  |  | unknown | 512&#8209;516 |
| 89 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 517&#8209;524 |
| 90 | `range_reduce_link` | Y | Y |  |  | Y |  |  | unknown | 527&#8209;529 |
| 91 | `clone_link` |  |  |  | Y | Y |  |  | hole | 598&#8209;604 |
| 92 | `compare_reduced_links` |  |  |  | Y | Y |  |  | unknown | 699&#8209;705 |
| 93 | `default` |  | Y |  |  | Y |  |  | unknown | 1599&#8209;1600 |
| 94 | `eq` |  | Y |  |  | Y |  |  | hole | 1646&#8209;1647 |

### Chap40/BSTSizeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 95 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 76&#8209;81 |
| 96 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 93&#8209;106 |
| 97 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 116&#8209;126 |
| 98 | `lemma_ordered_assemble` |  |  |  | Y | Y |  |  | unknown | 129&#8209;142 |
| 99 | `lemma_cmp_antisymmetry` |  |  |  | Y | Y |  |  | unknown | 146&#8209;151 |
| 100 | `lemma_cmp_antisymmetry_lt` |  |  |  | Y | Y |  |  | unknown | 158&#8209;163 |
| 101 | `lemma_cmp_transitivity_lt` |  |  |  | Y | Y |  |  | unknown | 170&#8209;176 |
| 102 | `lemma_cmp_transitivity_gt` |  |  |  | Y | Y |  |  | unknown | 183&#8209;189 |
| 103 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 247&#8209;251 |
| 104 | `size` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;254 |
| 105 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;257 |
| 106 | `height` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;264 |
| 107 | `insert` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;276 |
| 108 | `delete` | Y | Y |  |  | Y |  |  | unknown | 278&#8209;286 |
| 109 | `find` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;295 |
| 110 | `contains` | Y | Y |  |  | Y |  |  | unknown | 297&#8209;302 |
| 111 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 304&#8209;308 |
| 112 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;314 |
| 113 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;318 |
| 114 | `rank` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;325 |
| 115 | `select` | Y | Y |  |  | Y |  |  | unknown | 327&#8209;328 |
| 116 | `split_rank` | Y | Y |  |  | Y |  |  | unknown | 330&#8209;334 |
| 117 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 338&#8209;339 |
| 118 | `update_size` | Y | Y |  |  | Y |  |  | unknown | 340&#8209;348 |
| 119 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 349&#8209;355 |
| 120 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 356&#8209;372 |
| 121 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 373&#8209;389 |
| 122 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 390&#8209;403 |
| 123 | `delete_link` | Y | Y |  |  | Y |  |  | unknown | 404&#8209;416 |
| 124 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 417&#8209;427 |
| 125 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 428&#8209;432 |
| 126 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 433&#8209;437 |
| 127 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 438&#8209;443 |
| 128 | `in_order_collect` | Y | Y |  |  | Y |  |  | unknown | 444&#8209;447 |
| 129 | `in_order_collect_with_priority` | Y | Y |  |  | Y |  |  | unknown | 448&#8209;451 |
| 130 | `find_min_priority_idx` | Y | Y |  |  | Y |  |  | unknown | 452&#8209;454 |
| 131 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 455&#8209;460 |
| 132 | `filter_by_key` | Y | Y |  |  | Y |  |  | unknown | 461&#8209;462 |
| 133 | `rank_link` | Y | Y |  |  | Y |  |  | unknown | 463&#8209;468 |
| 134 | `select_link` | Y | Y |  |  | Y |  |  | unknown | 469&#8209;471 |
| 135 | `compare_links` |  |  |  | Y | Y |  |  | unknown | 589&#8209;595 |
| 136 | `clone_link` |  |  |  | Y | Y |  |  | hole | 1404&#8209;1410 |
| 137 | `default` |  | Y |  |  | Y |  |  | unknown | 1429&#8209;1430 |
| 138 | `eq` |  | Y |  |  | Y |  |  | hole | 1469&#8209;1470 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
