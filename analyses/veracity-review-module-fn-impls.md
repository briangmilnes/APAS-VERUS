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
| 1 | Chap40 | BSTKeyValueStEph | 27 | 28 | 0 | 1 | 29 | 0 | 28 | 1 | 0 |
| 2 | Chap40 | BSTReducedStEph | 36 | 37 | 0 | 2 | 39 | 0 | 36 | 3 | 0 |
| 3 | Chap40 | BSTSizeStEph | 31 | 32 | 0 | 4 | 36 | 0 | 35 | 1 | 0 |

## Function-by-Function Detail

### Chap40/BSTKeyValueStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `clone_link` |  |  |  | Y | Y |  |  | hole | 38&#8209;42 |
| 2 | `new` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;147 |
| 3 | `size` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;150 |
| 4 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;153 |
| 5 | `height` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;157 |
| 6 | `insert` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;163 |
| 7 | `delete` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;167 |
| 8 | `find` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;171 |
| 9 | `contains` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;175 |
| 10 | `get` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;179 |
| 11 | `keys` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;183 |
| 12 | `values` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;187 |
| 13 | `minimum_key` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;194 |
| 14 | `maximum_key` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;201 |
| 15 | `new_node` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;211 |
| 16 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;215 |
| 17 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;217 |
| 18 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;219 |
| 19 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 220&#8209;222 |
| 20 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;225 |
| 21 | `min_key_link` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;230 |
| 22 | `max_key_link` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;235 |
| 23 | `collect_keys` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;238 |
| 24 | `collect_values` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;241 |
| 25 | `collect_in_order_kvp` | Y | Y |  |  | Y |  |  | unknown | 242&#8209;244 |
| 26 | `find_min_priority_idx_kvp` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;249 |
| 27 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;255 |
| 28 | `filter_by_key_kvp` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;259 |
| 29 | `default` |  | Y |  |  | Y |  |  | unknown | 529&#8209;530 |

### Chap40/BSTReducedStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 30 | `clone_link` |  |  |  | Y | Y |  |  | hole | 43&#8209;48 |
| 31 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 246&#8209;250 |
| 32 | `identity` x2 | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 33 | `combine` x2 | Y | Y |  |  | Y |  |  | hole | 109&#8209;110 |
| 34 | `lift` x2 | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 35 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 226&#8209;235 |
| 36 | `size` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;253 |
| 37 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;256 |
| 38 | `height` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;260 |
| 39 | `insert` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;269 |
| 40 | `delete` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;275 |
| 41 | `find` | Y | Y |  |  | Y |  |  | unknown | 277&#8209;279 |
| 42 | `contains` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;283 |
| 43 | `get` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;287 |
| 44 | `keys` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;291 |
| 45 | `values` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;295 |
| 46 | `minimum_key` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;302 |
| 47 | `maximum_key` | Y | Y |  |  | Y |  |  | unknown | 305&#8209;309 |
| 48 | `reduced_value` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;314 |
| 49 | `range_reduce` | Y | Y |  |  | Y |  |  | unknown | 317&#8209;319 |
| 50 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;324 |
| 51 | `reduced_value_link` | Y | Y |  |  | Y |  |  | unknown | 327&#8209;328 |
| 52 | `update_node` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;342 |
| 53 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 345&#8209;355 |
| 54 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 357&#8209;363 |
| 55 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 365&#8209;371 |
| 56 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 373&#8209;381 |
| 57 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 382&#8209;384 |
| 58 | `min_key_link` | Y | Y |  |  | Y |  |  | unknown | 385&#8209;389 |
| 59 | `max_key_link` | Y | Y |  |  | Y |  |  | unknown | 390&#8209;394 |
| 60 | `collect_keys` | Y | Y |  |  | Y |  |  | unknown | 395&#8209;398 |
| 61 | `collect_values` | Y | Y |  |  | Y |  |  | unknown | 399&#8209;402 |
| 62 | `collect_in_order_kvp` | Y | Y |  |  | Y |  |  | unknown | 403&#8209;406 |
| 63 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 407&#8209;410 |
| 64 | `filter_by_key_kvp` | Y | Y |  |  | Y |  |  | unknown | 411&#8209;414 |
| 65 | `find_min_priority_idx_kvp` | Y | Y |  |  | Y |  |  | unknown | 415&#8209;419 |
| 66 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 420&#8209;427 |
| 67 | `range_reduce_link` | Y | Y |  |  | Y |  |  | unknown | 430&#8209;432 |
| 68 | `default` |  | Y |  |  | Y |  |  | unknown | 807&#8209;808 |

### Chap40/BSTSizeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 69 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 108&#8209;113 |
| 70 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 125&#8209;138 |
| 71 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 148&#8209;158 |
| 72 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 173&#8209;177 |
| 73 | `size` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;180 |
| 74 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;183 |
| 75 | `height` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;190 |
| 76 | `insert` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;199 |
| 77 | `delete` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;205 |
| 78 | `find` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;209 |
| 79 | `contains` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;213 |
| 80 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;219 |
| 81 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;225 |
| 82 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;229 |
| 83 | `rank` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;236 |
| 84 | `select` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;239 |
| 85 | `split_rank` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;245 |
| 86 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;250 |
| 87 | `update_size` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;259 |
| 88 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;265 |
| 89 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;272 |
| 90 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;279 |
| 91 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;288 |
| 92 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;291 |
| 93 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;296 |
| 94 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 297&#8209;301 |
| 95 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 302&#8209;307 |
| 96 | `in_order_collect` | Y | Y |  |  | Y |  |  | unknown | 308&#8209;311 |
| 97 | `in_order_collect_with_priority` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;315 |
| 98 | `find_min_priority_idx` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;318 |
| 99 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 319&#8209;324 |
| 100 | `filter_by_key` | Y | Y |  |  | Y |  |  | unknown | 325&#8209;326 |
| 101 | `rank_link` | Y | Y |  |  | Y |  |  | unknown | 327&#8209;332 |
| 102 | `select_link` | Y | Y |  |  | Y |  |  | unknown | 333&#8209;335 |
| 103 | `clone_link` |  |  |  | Y | Y |  |  | hole | 733&#8209;738 |
| 104 | `default` |  | Y |  |  | Y |  |  | unknown | 780&#8209;781 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
