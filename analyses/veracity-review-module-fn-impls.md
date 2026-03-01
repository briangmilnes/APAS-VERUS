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
| 1 | Chap40 | BSTKeyValueStEph | 13 | 14 | 0 | 15 | 29 | 0 | 8 | 0 | 21 |
| 2 | Chap40 | BSTReducedStEph | 18 | 19 | 0 | 20 | 39 | 0 | 16 | 1 | 22 |
| 3 | Chap40 | BSTSizeStEph | 14 | 15 | 0 | 21 | 36 | 0 | 20 | 0 | 16 |

## Function-by-Function Detail

### Chap40/BSTKeyValueStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `clone_link` |  |  |  | Y | Y |  | Y |  | 37&#8209;38 |
| 2 | `new_node` |  |  |  | Y | Y |  | Y |  | 67 |
| 3 | `new` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;106 |
| 4 | `size` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 5 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 6 | `height` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 7 | `insert` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;119 |
| 8 | `delete` | Y | Y |  |  | Y |  | Y |  | 121 |
| 9 | `find` | Y | Y |  |  | Y |  | Y |  | 123 |
| 10 | `contains` | Y | Y |  |  | Y |  | Y |  | 125 |
| 11 | `get` | Y | Y |  |  | Y |  | Y |  | 127 |
| 12 | `keys` | Y | Y |  |  | Y |  | Y |  | 129 |
| 13 | `values` | Y | Y |  |  | Y |  | Y |  | 131 |
| 14 | `minimum_key` | Y | Y |  |  | Y |  | Y |  | 134 |
| 15 | `maximum_key` | Y | Y |  |  | Y |  | Y |  | 137 |
| 16 | `height_link` |  |  |  | Y | Y |  |  | unknown | 142&#8209;145 |
| 17 | `rotate_left` |  |  |  | Y | Y |  | Y |  | 172 |
| 18 | `rotate_right` |  |  |  | Y | Y |  | Y |  | 186 |
| 19 | `insert_link` |  |  |  | Y | Y |  | Y |  | 198&#8209;199 |
| 20 | `find_link` |  |  |  | Y | Y |  | Y |  | 235&#8209;236 |
| 21 | `min_key_link` |  |  |  | Y | Y |  | Y |  | 254&#8209;255 |
| 22 | `max_key_link` |  |  |  | Y | Y |  | Y |  | 268&#8209;269 |
| 23 | `collect_keys` |  |  |  | Y | Y |  | Y |  | 282&#8209;283 |
| 24 | `collect_values` |  |  |  | Y | Y |  | Y |  | 294&#8209;295 |
| 25 | `collect_in_order_kvp` |  |  |  | Y | Y |  | Y |  | 305&#8209;306 |
| 26 | `find_min_priority_idx_kvp` |  |  |  | Y | Y |  |  | unknown | 315&#8209;319 |
| 27 | `build_treap_from_vec` |  |  |  | Y | Y |  |  | unknown | 337&#8209;341 |
| 28 | `filter_by_key_kvp` |  |  |  | Y | Y |  | Y |  | 358&#8209;360 |
| 29 | `default` |  | Y |  |  | Y |  | Y |  | 430 |

### Chap40/BSTReducedStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 30 | `clone_link` |  |  |  | Y | Y |  | Y |  | 42&#8209;43 |
| 31 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 153&#8209;158 |
| 32 | `identity` x2 | Y | Y |  |  | Y |  | Y |  | 94 |
| 33 | `combine` x2 | Y | Y |  |  | Y |  |  | hole | 96 |
| 34 | `lift` x2 | Y | Y |  |  | Y |  | Y |  | 98 |
| 35 | `size` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;161 |
| 36 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 37 | `height` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;168 |
| 38 | `insert` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;177 |
| 39 | `delete` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;180 |
| 40 | `find` | Y | Y |  |  | Y |  | Y |  | 182 |
| 41 | `contains` | Y | Y |  |  | Y |  | Y |  | 184 |
| 42 | `get` | Y | Y |  |  | Y |  | Y |  | 186 |
| 43 | `keys` | Y | Y |  |  | Y |  | Y |  | 188 |
| 44 | `values` | Y | Y |  |  | Y |  | Y |  | 190 |
| 45 | `minimum_key` | Y | Y |  |  | Y |  | Y |  | 193 |
| 46 | `maximum_key` | Y | Y |  |  | Y |  | Y |  | 196 |
| 47 | `reduced_value` | Y | Y |  |  | Y |  | Y |  | 199 |
| 48 | `range_reduce` | Y | Y |  |  | Y |  | Y |  | 202 |
| 49 | `default` |  | Y |  |  | Y |  | Y |  | 206 |
| 50 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 233&#8209;242 |
| 51 | `size_link` |  |  |  | Y | Y |  |  | unknown | 245&#8209;246 |
| 52 | `reduced_value_link` |  |  |  | Y | Y |  | Y |  | 256 |
| 53 | `update_node` |  |  |  | Y | Y |  |  | unknown | 265&#8209;276 |
| 54 | `make_node` |  |  |  | Y | Y |  |  | unknown | 290&#8209;303 |
| 55 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 319&#8209;325 |
| 56 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 354&#8209;360 |
| 57 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 389&#8209;402 |
| 58 | `find_link` |  |  |  | Y | Y |  | Y |  | 454&#8209;458 |
| 59 | `min_key_link` |  |  |  | Y | Y |  | Y |  | 476&#8209;477 |
| 60 | `max_key_link` |  |  |  | Y | Y |  | Y |  | 490&#8209;491 |
| 61 | `collect_keys` |  |  |  | Y | Y |  | Y |  | 504&#8209;505 |
| 62 | `collect_values` |  |  |  | Y | Y |  | Y |  | 516&#8209;517 |
| 63 | `collect_in_order_kvp` |  |  |  | Y | Y |  | Y |  | 527&#8209;531 |
| 64 | `height_link` |  |  |  | Y | Y |  |  | unknown | 553&#8209;556 |
| 65 | `filter_by_key_kvp_r` |  |  |  | Y | Y |  | Y |  | 569&#8209;571 |
| 66 | `find_min_priority_idx_kvp_r` |  |  |  | Y | Y |  |  | unknown | 586&#8209;590 |
| 67 | `build_treap_from_vec_r` |  |  |  | Y | Y |  |  | unknown | 608&#8209;615 |
| 68 | `range_reduce_link` |  |  |  | Y | Y |  | Y |  | 631&#8209;636 |

### Chap40/BSTSizeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 69 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 85&#8209;90 |
| 70 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 102&#8209;115 |
| 71 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 125&#8209;135 |
| 72 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 149&#8209;154 |
| 73 | `size` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;157 |
| 74 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;160 |
| 75 | `height` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;167 |
| 76 | `insert` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;176 |
| 77 | `delete` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;179 |
| 78 | `find` | Y | Y |  |  | Y |  | Y |  | 181 |
| 79 | `contains` | Y | Y |  |  | Y |  | Y |  | 183 |
| 80 | `minimum` | Y | Y |  |  | Y |  | Y |  | 185 |
| 81 | `maximum` | Y | Y |  |  | Y |  | Y |  | 187 |
| 82 | `in_order` | Y | Y |  |  | Y |  | Y |  | 189 |
| 83 | `rank` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;196 |
| 84 | `select` | Y | Y |  |  | Y |  | Y |  | 198 |
| 85 | `split_rank` | Y | Y |  |  | Y |  | Y |  | 200 |
| 86 | `size_link` |  |  |  | Y | Y |  |  | unknown | 217&#8209;218 |
| 87 | `update_size` |  |  |  | Y | Y |  |  | unknown | 226&#8209;234 |
| 88 | `make_node` |  |  |  | Y | Y |  |  | unknown | 241&#8209;246 |
| 89 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 255&#8209;261 |
| 90 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 293&#8209;299 |
| 91 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 331&#8209;339 |
| 92 | `find_link` |  |  |  | Y | Y |  | Y |  | 392&#8209;393 |
| 93 | `min_link` |  |  |  | Y | Y |  | Y |  | 409&#8209;410 |
| 94 | `max_link` |  |  |  | Y | Y |  | Y |  | 421&#8209;422 |
| 95 | `height_link` |  |  |  | Y | Y |  |  | unknown | 433&#8209;438 |
| 96 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 460&#8209;461 |
| 97 | `in_order_collect_with_priority` |  |  |  | Y | Y |  | Y |  | 470&#8209;474 |
| 98 | `find_min_priority_idx` |  |  |  | Y | Y |  |  | unknown | 483&#8209;488 |
| 99 | `build_treap_from_vec` |  |  |  | Y | Y |  |  | unknown | 509&#8209;516 |
| 100 | `filter_by_key` |  |  |  | Y | Y |  | Y |  | 529 |
| 101 | `rank_link` |  |  |  | Y | Y |  |  | unknown | 544&#8209;549 |
| 102 | `select_link` |  |  |  | Y | Y |  | Y |  | 568&#8209;569 |
| 103 | `clone_link` |  |  |  | Y | Y |  | Y |  | 655&#8209;656 |
| 104 | `default` |  | Y |  |  | Y |  | Y |  | 691 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
