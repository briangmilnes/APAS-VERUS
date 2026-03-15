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
| 1 | Chap39 | BSTParaTreapMtEph | 17 | 17 | 0 | 16 | 18 | 15 | 9 | 9 | 15 |
| 2 | Chap39 | BSTSetTreapMtEph | 20 | 20 | 0 | 2 | 22 | 0 | 22 | 0 | 0 |
| 3 | Chap39 | BSTTreapMtEph | 12 | 13 | 0 | 20 | 33 | 0 | 24 | 7 | 2 |
| 4 | Chap39 | BSTTreapStEph | 34 | 35 | 0 | 0 | 35 | 0 | 35 | 0 | 0 |

## Function-by-Function Detail

### Chap39/BSTParaTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_param_treap_arc` |  |  |  | Y | Y |  |  | unknown | 86&#8209;90 |
| 2 | `new` | Y | Y |  |  | Y |  |  | unknown | 370&#8209;371 |
| 3 | `expose` | Y | Y |  |  | Y |  |  | unknown | 374&#8209;385 |
| 4 | `expose_with_priority` | Y | Y |  |  | Y |  |  | unknown | 388&#8209;399 |
| 5 | `join_mid` | Y | Y |  |  | Y |  |  | hole | 402&#8209;406 |
| 6 | `size` | Y | Y |  |  | Y |  |  | unknown | 409&#8209;410 |
| 7 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 413&#8209;414 |
| 8 | `insert` | Y | Y |  |  | Y |  |  | unknown | 417&#8209;418 |
| 9 | `delete` | Y | Y |  |  | Y |  |  | unknown | 421&#8209;422 |
| 10 | `find` | Y | Y |  |  | Y |  |  | unknown | 425&#8209;428 |
| 11 | `split` | Y | Y |  |  | Y |  |  | hole | 431&#8209;439 |
| 12 | `join_pair` | Y | Y |  |  | Y |  |  | hole | 442&#8209;443 |
| 13 | `union` | Y | Y |  |  | Y |  |  | hole | 446&#8209;447 |
| 14 | `intersect` | Y | Y |  |  | Y |  |  | hole | 450&#8209;451 |
| 15 | `difference` | Y | Y |  |  | Y |  |  | hole | 454&#8209;455 |
| 16 | `filter` | Y | Y |  |  | Y |  |  | hole | 458&#8209;459 |
| 17 | `reduce` | Y | Y |  |  | Y |  |  | hole | 462&#8209;465 |
| 18 | `in_order` | Y | Y |  |  | Y |  |  | hole | 468&#8209;469 |
| 19 | `priority_for` |  |  |  | Y |  | Y | Y |  | 134&#8209;142 |
| 20 | `tree_priority` |  |  |  | Y |  | Y | Y |  | 144&#8209;151 |
| 21 | `tree_size` |  |  |  | Y |  | Y | Y |  | 153&#8209;160 |
| 22 | `make_node` |  |  |  | Y |  | Y | Y |  | 162&#8209;171 |
| 23 | `join_with_priority` |  |  |  | Y |  | Y | Y |  | 173&#8209;195 |
| 24 | `split_inner` |  |  |  | Y |  | Y | Y |  | 197&#8209;217 |
| 25 | `join_pair_inner` |  |  |  | Y |  | Y | Y |  | 219&#8209;232 |
| 26 | `union_inner` |  |  |  | Y |  | Y | Y |  | 234&#8209;247 |
| 27 | `intersect_inner` |  |  |  | Y |  | Y | Y |  | 249&#8209;266 |
| 28 | `difference_inner` |  |  |  | Y |  | Y | Y |  | 268&#8209;285 |
| 29 | `filter_inner` |  |  |  | Y |  | Y | Y |  | 287&#8209;305 |
| 30 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 307&#8209;313 |
| 31 | `reduce_inner` |  |  |  | Y |  | Y | Y |  | 315&#8209;336 |
| 32 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 338&#8209;345 |
| 33 | `collect_in_order` |  |  |  | Y |  | Y | Y |  | 347&#8209;359 |

### Chap39/BSTSetTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 34 | `empty` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;55 |
| 35 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;58 |
| 36 | `size` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;61 |
| 37 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 38 | `find` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 39 | `contains` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 40 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;78 |
| 41 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;84 |
| 42 | `insert` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;87 |
| 43 | `delete` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 44 | `union` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 45 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 46 | `difference` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 47 | `split` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;109 |
| 48 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 49 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 50 | `filter` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 51 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;123 |
| 52 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;126 |
| 53 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 54 | `minimum_inner` |  |  |  | Y | Y |  |  | unknown | 135&#8209;139 |
| 55 | `maximum_inner` |  |  |  | Y | Y |  |  | unknown | 157&#8209;161 |

### Chap39/BSTTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 56 | `lemma_bst_decompose` |  |  |  | Y | Y |  |  | unknown | 108&#8209;118 |
| 57 | `lemma_contains_left` |  |  |  | Y | Y |  |  | unknown | 122&#8209;124 |
| 58 | `lemma_contains_right` |  |  |  | Y | Y |  |  | unknown | 128&#8209;130 |
| 59 | `lemma_contains_root` |  |  |  | Y | Y |  |  | unknown | 134&#8209;135 |
| 60 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 139&#8209;144 |
| 61 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 156&#8209;169 |
| 62 | `lemma_wf_assemble_node` |  |  |  | Y | Y |  |  | unknown | 180&#8209;185 |
| 63 | `new` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;210 |
| 64 | `insert` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;214 |
| 65 | `delete` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;218 |
| 66 | `find` | Y | Y |  |  | Y |  |  | hole | 220&#8209;221 |
| 67 | `contains` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;224 |
| 68 | `size` | Y | Y |  |  | Y |  |  | hole | 226&#8209;227 |
| 69 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;230 |
| 70 | `height` | Y | Y |  |  | Y |  | Y |  | 232 |
| 71 | `minimum` | Y | Y |  |  | Y |  |  | hole | 234&#8209;235 |
| 72 | `maximum` | Y | Y |  |  | Y |  |  | hole | 237&#8209;238 |
| 73 | `in_order` | Y | Y |  |  | Y |  |  | hole | 240&#8209;241 |
| 74 | `pre_order` | Y | Y |  |  | Y |  |  | hole | 243&#8209;244 |
| 75 | `clone_link` |  |  |  | Y | Y |  |  | hole | 301&#8209;306 |
| 76 | `size_link` |  |  |  | Y | Y |  |  | unknown | 337&#8209;339 |
| 77 | `update` |  |  |  | Y | Y |  |  | unknown | 347&#8209;353 |
| 78 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 362&#8209;370 |
| 79 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 436&#8209;444 |
| 80 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 512&#8209;524 |
| 81 | `delete_link` |  |  |  | Y | Y |  |  | unknown | 617&#8209;627 |
| 82 | `find_link` |  |  |  | Y | Y |  |  | unknown | 801&#8209;804 |
| 83 | `min_link` |  |  |  | Y | Y |  |  | unknown | 831&#8209;834 |
| 84 | `max_link` |  |  |  | Y | Y |  |  | unknown | 856&#8209;859 |
| 85 | `height_link` |  |  |  | Y | Y |  |  | unknown | 879&#8209;884 |
| 86 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 908&#8209;911 |
| 87 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 922&#8209;925 |
| 88 | `default` |  | Y |  |  | Y |  | Y |  | 1061 |

### Chap39/BSTTreapStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 89 | `lemma_height_le_size` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 90 | `lemma_size_wf_child_bounded` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;109 |
| 91 | `lemma_wf_decompose` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;120 |
| 92 | `lemma_wf_assemble_node` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;127 |
| 93 | `lemma_contains_left` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 94 | `lemma_contains_right` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;135 |
| 95 | `lemma_bst_decompose` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;147 |
| 96 | `lemma_contains_root` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;150 |
| 97 | `new` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;160 |
| 98 | `size` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 99 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;168 |
| 100 | `height` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;175 |
| 101 | `insert` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;188 |
| 102 | `delete` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;199 |
| 103 | `find` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;203 |
| 104 | `contains` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;207 |
| 105 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;215 |
| 106 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;223 |
| 107 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;227 |
| 108 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;231 |
| 109 | `new_node` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;237 |
| 110 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;241 |
| 111 | `update_size` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;250 |
| 112 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;260 |
| 113 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;270 |
| 114 | `clone_link` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;275 |
| 115 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 277&#8209;281 |
| 116 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;294 |
| 117 | `delete_link` | Y | Y |  |  | Y |  |  | unknown | 296&#8209;304 |
| 118 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 306&#8209;307 |
| 119 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;314 |
| 120 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;321 |
| 121 | `in_order_vec` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;324 |
| 122 | `pre_order_vec` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;327 |
| 123 | `default` |  | Y |  |  | Y |  |  | unknown | 1092&#8209;1093 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
