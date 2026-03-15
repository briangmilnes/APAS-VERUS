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
| 1 | Chap40 | BSTKeyValueStEph | 26 | 28 | 0 | 9 | 37 | 0 | 37 | 0 | 0 |
| 2 | Chap40 | BSTReducedStEph | 36 | 38 | 0 | 3 | 41 | 0 | 39 | 2 | 0 |
| 3 | Chap40 | BSTSizeStEph | 31 | 33 | 0 | 5 | 38 | 0 | 38 | 0 | 0 |

## Function-by-Function Detail

### Chap40/BSTKeyValueStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_content_left_contains_key` |  |  |  | Y | Y |  |  | unknown | 99&#8209;103 |
| 2 | `lemma_content_right_contains_key` |  |  |  | Y | Y |  |  | unknown | 107&#8209;111 |
| 3 | `lemma_rotate_left_preserves_keys` |  |  |  | Y | Y |  |  | unknown | 116&#8209;127 |
| 4 | `lemma_rotate_right_preserves_keys` |  |  |  | Y | Y |  |  | unknown | 132&#8209;143 |
| 5 | `lemma_left_key_in_link` |  |  |  | Y | Y |  |  | unknown | 148&#8209;157 |
| 6 | `lemma_right_key_in_link` |  |  |  | Y | Y |  |  | unknown | 162&#8209;171 |
| 7 | `lemma_node_key_in_link` |  |  |  | Y | Y |  |  | unknown | 176&#8209;184 |
| 8 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 216&#8209;219 |
| 9 | `size` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;222 |
| 10 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 224&#8209;225 |
| 11 | `height` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;229 |
| 12 | `insert` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;236 |
| 13 | `delete` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;240 |
| 14 | `find` | Y | Y |  |  | Y |  |  | unknown | 242&#8209;246 |
| 15 | `contains` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;252 |
| 16 | `get` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;258 |
| 17 | `keys` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;262 |
| 18 | `values` | Y | Y |  |  | Y |  |  | unknown | 264&#8209;266 |
| 19 | `minimum_key` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;278 |
| 20 | `maximum_key` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;290 |
| 21 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;297 |
| 22 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;301 |
| 23 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 302&#8209;305 |
| 24 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 306&#8209;310 |
| 25 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;315 |
| 26 | `min_key_link` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;325 |
| 27 | `max_key_link` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;335 |
| 28 | `collect_keys` | Y | Y |  |  | Y |  |  | unknown | 336&#8209;338 |
| 29 | `collect_values` | Y | Y |  |  | Y |  |  | unknown | 339&#8209;341 |
| 30 | `collect_in_order_kvp` | Y | Y |  |  | Y |  |  | unknown | 342&#8209;344 |
| 31 | `find_min_priority_idx_kvp` | Y | Y |  |  | Y |  |  | unknown | 345&#8209;349 |
| 32 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 350&#8209;355 |
| 33 | `filter_by_key_kvp` | Y | Y |  |  | Y |  |  | unknown | 356&#8209;359 |
| 34 | `clone_link` |  |  |  | Y | Y |  |  | unknown | 364&#8209;369 |
| 35 | `compare_kv_links` |  |  |  | Y | Y |  |  | unknown | 485&#8209;491 |
| 36 | `default` |  | Y |  |  | Y |  |  | unknown | 818&#8209;819 |
| 37 | `eq` |  | Y |  |  | Y |  |  | unknown | 859&#8209;860 |

### Chap40/BSTReducedStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 80&#8209;89 |
| 39 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 40 | `identity` x2 | Y | Y |  |  | Y |  |  | hole | 120&#8209;121 |
| 41 | `combine` x2 | Y | Y |  |  | Y |  |  | hole | 122&#8209;123 |
| 42 | `lift` x2 | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 43 | `size` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;141 |
| 44 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;144 |
| 45 | `height` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;148 |
| 46 | `insert` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;157 |
| 47 | `delete` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;163 |
| 48 | `find` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;167 |
| 49 | `contains` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;171 |
| 50 | `get` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;175 |
| 51 | `keys` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;179 |
| 52 | `values` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;183 |
| 53 | `minimum_key` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;190 |
| 54 | `maximum_key` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;197 |
| 55 | `reduced_value` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;202 |
| 56 | `range_reduce` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;207 |
| 57 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;212 |
| 58 | `reduced_value_link` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;216 |
| 59 | `update_node` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;230 |
| 60 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;243 |
| 61 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;251 |
| 62 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;259 |
| 63 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;269 |
| 64 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 270&#8209;272 |
| 65 | `min_key_link` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;277 |
| 66 | `max_key_link` | Y | Y |  |  | Y |  |  | unknown | 278&#8209;282 |
| 67 | `collect_keys` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;286 |
| 68 | `collect_values` | Y | Y |  |  | Y |  |  | unknown | 287&#8209;290 |
| 69 | `collect_in_order_kvp` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;294 |
| 70 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;298 |
| 71 | `filter_by_key_kvp` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;302 |
| 72 | `find_min_priority_idx_kvp` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;307 |
| 73 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 308&#8209;315 |
| 74 | `range_reduce_link` | Y | Y |  |  | Y |  |  | unknown | 318&#8209;320 |
| 75 | `clone_link` |  |  |  | Y | Y |  |  | unknown | 373&#8209;379 |
| 76 | `compare_reduced_links` |  |  |  | Y | Y |  |  | unknown | 474&#8209;480 |
| 77 | `default` |  | Y |  |  | Y |  |  | unknown | 859&#8209;860 |
| 78 | `eq` |  | Y |  |  | Y |  |  | unknown | 905&#8209;906 |

### Chap40/BSTSizeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 79 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 68&#8209;73 |
| 80 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 85&#8209;98 |
| 81 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 108&#8209;118 |
| 82 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 149&#8209;153 |
| 83 | `size` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;156 |
| 84 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;159 |
| 85 | `height` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;166 |
| 86 | `insert` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;175 |
| 87 | `delete` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;181 |
| 88 | `find` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;185 |
| 89 | `contains` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;189 |
| 90 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;195 |
| 91 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;201 |
| 92 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;205 |
| 93 | `rank` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;212 |
| 94 | `select` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;215 |
| 95 | `split_rank` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;221 |
| 96 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;226 |
| 97 | `update_size` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;235 |
| 98 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;241 |
| 99 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 242&#8209;248 |
| 100 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;255 |
| 101 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;264 |
| 102 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;267 |
| 103 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;272 |
| 104 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;277 |
| 105 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 278&#8209;283 |
| 106 | `in_order_collect` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;287 |
| 107 | `in_order_collect_with_priority` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;291 |
| 108 | `find_min_priority_idx` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;294 |
| 109 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;300 |
| 110 | `filter_by_key` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;302 |
| 111 | `rank_link` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;308 |
| 112 | `select_link` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;311 |
| 113 | `compare_links` |  |  |  | Y | Y |  |  | unknown | 413&#8209;419 |
| 114 | `clone_link` |  |  |  | Y | Y |  |  | unknown | 815&#8209;821 |
| 115 | `default` |  | Y |  |  | Y |  |  | unknown | 840&#8209;841 |
| 116 | `eq` |  | Y |  |  | Y |  |  | unknown | 878&#8209;879 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
