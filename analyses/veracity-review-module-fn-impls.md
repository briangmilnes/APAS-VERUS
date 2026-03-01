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
| 1 | Chap40 | BSTKeyValueStEph | 27 | 28 | 0 | 1 | 29 | 0 | 8 | 0 | 21 |
| 2 | Chap40 | BSTReducedStEph | 36 | 37 | 0 | 2 | 39 | 0 | 16 | 1 | 22 |
| 3 | Chap40 | BSTSizeStEph | 31 | 32 | 0 | 4 | 36 | 0 | 20 | 0 | 16 |

## Function-by-Function Detail

### Chap40/BSTKeyValueStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `clone_link` |  |  |  | Y | Y |  | Y |  | 37&#8209;38 |
| 2 | `new` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;107 |
| 3 | `size` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 4 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 5 | `height` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;117 |
| 6 | `insert` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 7 | `delete` | Y | Y |  |  | Y |  | Y |  | 122 |
| 8 | `find` | Y | Y |  |  | Y |  | Y |  | 124 |
| 9 | `contains` | Y | Y |  |  | Y |  | Y |  | 126 |
| 10 | `get` | Y | Y |  |  | Y |  | Y |  | 128 |
| 11 | `keys` | Y | Y |  |  | Y |  | Y |  | 130 |
| 12 | `values` | Y | Y |  |  | Y |  | Y |  | 132 |
| 13 | `minimum_key` | Y | Y |  |  | Y |  | Y |  | 135 |
| 14 | `maximum_key` | Y | Y |  |  | Y |  | Y |  | 138 |
| 15 | `new_node` | Y | Y |  |  | Y |  | Y |  | 142 |
| 16 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;146 |
| 17 | `rotate_left` | Y | Y |  |  | Y |  | Y |  | 147 |
| 18 | `rotate_right` | Y | Y |  |  | Y |  | Y |  | 148 |
| 19 | `insert_link` | Y | Y |  |  | Y |  | Y |  | 149&#8209;150 |
| 20 | `find_link` | Y | Y |  |  | Y |  | Y |  | 151&#8209;152 |
| 21 | `min_key_link` | Y | Y |  |  | Y |  | Y |  | 153&#8209;154 |
| 22 | `max_key_link` | Y | Y |  |  | Y |  | Y |  | 155&#8209;156 |
| 23 | `collect_keys` | Y | Y |  |  | Y |  | Y |  | 157&#8209;158 |
| 24 | `collect_values` | Y | Y |  |  | Y |  | Y |  | 159&#8209;160 |
| 25 | `collect_in_order_kvp` | Y | Y |  |  | Y |  | Y |  | 161&#8209;162 |
| 26 | `find_min_priority_idx_kvp` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;167 |
| 27 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;172 |
| 28 | `filter_by_key_kvp` | Y | Y |  |  | Y |  | Y |  | 173&#8209;175 |
| 29 | `default` |  | Y |  |  | Y |  | Y |  | 440 |

### Chap40/BSTReducedStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 30 | `clone_link` |  |  |  | Y | Y |  | Y |  | 42&#8209;43 |
| 31 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 202&#8209;205 |
| 32 | `identity` x2 | Y | Y |  |  | Y |  | Y |  | 94 |
| 33 | `combine` x2 | Y | Y |  |  | Y |  |  | hole | 96 |
| 34 | `lift` x2 | Y | Y |  |  | Y |  | Y |  | 98 |
| 35 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 182&#8209;191 |
| 36 | `size` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;208 |
| 37 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;211 |
| 38 | `height` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;215 |
| 39 | `insert` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;224 |
| 40 | `delete` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;227 |
| 41 | `find` | Y | Y |  |  | Y |  | Y |  | 229 |
| 42 | `contains` | Y | Y |  |  | Y |  | Y |  | 231 |
| 43 | `get` | Y | Y |  |  | Y |  | Y |  | 233 |
| 44 | `keys` | Y | Y |  |  | Y |  | Y |  | 235 |
| 45 | `values` | Y | Y |  |  | Y |  | Y |  | 237 |
| 46 | `minimum_key` | Y | Y |  |  | Y |  | Y |  | 240 |
| 47 | `maximum_key` | Y | Y |  |  | Y |  | Y |  | 243 |
| 48 | `reduced_value` | Y | Y |  |  | Y |  | Y |  | 246 |
| 49 | `range_reduce` | Y | Y |  |  | Y |  | Y |  | 249 |
| 50 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;254 |
| 51 | `reduced_value_link` | Y | Y |  |  | Y |  | Y |  | 257 |
| 52 | `update_node` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;271 |
| 53 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;284 |
| 54 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 286&#8209;292 |
| 55 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;300 |
| 56 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 302&#8209;310 |
| 57 | `find_link` | Y | Y |  |  | Y |  | Y |  | 311&#8209;312 |
| 58 | `min_key_link` | Y | Y |  |  | Y |  | Y |  | 313&#8209;314 |
| 59 | `max_key_link` | Y | Y |  |  | Y |  | Y |  | 315&#8209;316 |
| 60 | `collect_keys` | Y | Y |  |  | Y |  | Y |  | 317&#8209;318 |
| 61 | `collect_values` | Y | Y |  |  | Y |  | Y |  | 319&#8209;320 |
| 62 | `collect_in_order_kvp` | Y | Y |  |  | Y |  | Y |  | 321&#8209;322 |
| 63 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;326 |
| 64 | `filter_by_key_kvp` | Y | Y |  |  | Y |  | Y |  | 327&#8209;329 |
| 65 | `find_min_priority_idx_kvp` | Y | Y |  |  | Y |  |  | unknown | 330&#8209;334 |
| 66 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;342 |
| 67 | `range_reduce_link` | Y | Y |  |  | Y |  | Y |  | 345&#8209;346 |
| 68 | `default` |  | Y |  |  | Y |  | Y |  | 719 |

### Chap40/BSTSizeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 69 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 86&#8209;91 |
| 70 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 103&#8209;116 |
| 71 | `lemma_wf_assemble` |  |  |  | Y | Y |  |  | unknown | 126&#8209;136 |
| 72 | `new` x3 | Y | Y |  |  | Y |  |  | unknown | 151&#8209;154 |
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
| 86 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;205 |
| 87 | `update_size` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;214 |
| 88 | `make_node` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;220 |
| 89 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;227 |
| 90 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 228&#8209;234 |
| 91 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;243 |
| 92 | `find_link` | Y | Y |  |  | Y |  | Y |  | 244&#8209;245 |
| 93 | `min_link` | Y | Y |  |  | Y |  | Y |  | 246&#8209;247 |
| 94 | `max_link` | Y | Y |  |  | Y |  | Y |  | 248&#8209;249 |
| 95 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;255 |
| 96 | `in_order_collect` | Y | Y |  |  | Y |  | Y |  | 256&#8209;257 |
| 97 | `in_order_collect_with_priority` | Y | Y |  |  | Y |  | Y |  | 258&#8209;259 |
| 98 | `find_min_priority_idx` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;262 |
| 99 | `build_treap_from_vec` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;268 |
| 100 | `filter_by_key` | Y | Y |  |  | Y |  | Y |  | 269 |
| 101 | `rank_link` | Y | Y |  |  | Y |  |  | unknown | 270&#8209;275 |
| 102 | `select_link` | Y | Y |  |  | Y |  | Y |  | 276&#8209;277 |
| 103 | `clone_link` |  |  |  | Y | Y |  | Y |  | 673&#8209;674 |
| 104 | `default` |  | Y |  |  | Y |  | Y |  | 709 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
