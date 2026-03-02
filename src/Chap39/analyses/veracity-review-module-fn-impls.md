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
| 1 | Chap39 | BSTParaTreapMtEph | 17 | 17 | 0 | 16 | 1 | 32 | 0 | 1 | 32 |
| 2 | Chap39 | BSTSetTreapMtEph | 20 | 20 | 0 | 2 | 0 | 22 | 0 | 0 | 22 |
| 3 | Chap39 | BSTTreapMtEph | 11 | 12 | 0 | 16 | 28 | 0 | 9 | 1 | 18 |
| 4 | Chap39 | BSTTreapStEph | 24 | 20 | 0 | 8 | 33 | 0 | 32 | 0 | 1 |

## Function-by-Function Detail

### Chap39/BSTParaTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_treap_lock` |  |  |  | Y | Y |  |  | hole | 77&#8209;78 |
| 2 | `priority_for` |  |  |  | Y |  | Y | Y |  | 116&#8209;124 |
| 3 | `tree_priority` |  |  |  | Y |  | Y | Y |  | 126&#8209;133 |
| 4 | `tree_size` |  |  |  | Y |  | Y | Y |  | 135&#8209;142 |
| 5 | `make_node` |  |  |  | Y |  | Y | Y |  | 144&#8209;153 |
| 6 | `join_with_priority` |  |  |  | Y |  | Y | Y |  | 155&#8209;177 |
| 7 | `split_inner` |  |  |  | Y |  | Y | Y |  | 179&#8209;199 |
| 8 | `join_pair_inner` |  |  |  | Y |  | Y | Y |  | 201&#8209;214 |
| 9 | `union_inner` |  |  |  | Y |  | Y | Y |  | 216&#8209;229 |
| 10 | `intersect_inner` |  |  |  | Y |  | Y | Y |  | 231&#8209;248 |
| 11 | `difference_inner` |  |  |  | Y |  | Y | Y |  | 250&#8209;267 |
| 12 | `filter_inner` |  |  |  | Y |  | Y | Y |  | 269&#8209;287 |
| 13 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 289&#8209;295 |
| 14 | `reduce_inner` |  |  |  | Y |  | Y | Y |  | 297&#8209;318 |
| 15 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 320&#8209;327 |
| 16 | `collect_in_order` |  |  |  | Y |  | Y | Y |  | 329&#8209;341 |
| 17 | `new` | Y | Y |  |  |  | Y | Y |  | 346&#8209;348 |
| 18 | `expose` | Y | Y |  |  |  | Y | Y |  | 349&#8209;351 |
| 19 | `expose_with_priority` | Y | Y |  |  |  | Y | Y |  | 352&#8209;354 |
| 20 | `join_mid` | Y | Y |  |  |  | Y | Y |  | 355&#8209;357 |
| 21 | `size` | Y | Y |  |  |  | Y | Y |  | 358&#8209;360 |
| 22 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 361&#8209;363 |
| 23 | `insert` | Y | Y |  |  |  | Y | Y |  | 364&#8209;366 |
| 24 | `delete` | Y | Y |  |  |  | Y | Y |  | 367&#8209;369 |
| 25 | `find` | Y | Y |  |  |  | Y | Y |  | 370&#8209;372 |
| 26 | `split` | Y | Y |  |  |  | Y | Y |  | 373&#8209;375 |
| 27 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 376&#8209;378 |
| 28 | `union` | Y | Y |  |  |  | Y | Y |  | 379&#8209;381 |
| 29 | `intersect` | Y | Y |  |  |  | Y | Y |  | 382&#8209;384 |
| 30 | `difference` | Y | Y |  |  |  | Y | Y |  | 385&#8209;387 |
| 31 | `filter` | Y | Y |  |  |  | Y | Y |  | 388&#8209;390 |
| 32 | `reduce` | Y | Y |  |  |  | Y | Y |  | 391&#8209;395 |
| 33 | `in_order` | Y | Y |  |  |  | Y | Y |  | 396&#8209;398 |

### Chap39/BSTSetTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 34 | `minimum_inner` |  |  |  | Y |  | Y | Y |  | 35&#8209;43 |
| 35 | `maximum_inner` |  |  |  | Y |  | Y | Y |  | 45&#8209;53 |
| 36 | `empty` | Y | Y |  |  |  | Y | Y |  | 58&#8209;59 |
| 37 | `singleton` | Y | Y |  |  |  | Y | Y |  | 60&#8209;61 |
| 38 | `size` | Y | Y |  |  |  | Y | Y |  | 62&#8209;63 |
| 39 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 64&#8209;65 |
| 40 | `find` | Y | Y |  |  |  | Y | Y |  | 66&#8209;67 |
| 41 | `contains` | Y | Y |  |  |  | Y | Y |  | 68&#8209;69 |
| 42 | `minimum` | Y | Y |  |  |  | Y | Y |  | 70&#8209;71 |
| 43 | `maximum` | Y | Y |  |  |  | Y | Y |  | 72&#8209;73 |
| 44 | `insert` | Y | Y |  |  |  | Y | Y |  | 74&#8209;75 |
| 45 | `delete` | Y | Y |  |  |  | Y | Y |  | 76&#8209;77 |
| 46 | `union` | Y | Y |  |  |  | Y | Y |  | 78&#8209;79 |
| 47 | `intersection` | Y | Y |  |  |  | Y | Y |  | 80&#8209;81 |
| 48 | `difference` | Y | Y |  |  |  | Y | Y |  | 82&#8209;83 |
| 49 | `split` | Y | Y |  |  |  | Y | Y |  | 84&#8209;85 |
| 50 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 86&#8209;87 |
| 51 | `join_m` | Y | Y |  |  |  | Y | Y |  | 88&#8209;89 |
| 52 | `filter` | Y | Y |  |  |  | Y | Y |  | 90&#8209;91 |
| 53 | `reduce` | Y | Y |  |  |  | Y | Y |  | 92&#8209;95 |
| 54 | `iter_in_order` | Y | Y |  |  |  | Y | Y |  | 96&#8209;97 |
| 55 | `as_tree` | Y | Y |  |  |  | Y | Y |  | 98&#8209;99 |

### Chap39/BSTTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 56 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 95&#8209;100 |
| 57 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 112&#8209;125 |
| 58 | `lemma_wf_assemble_node` |  |  |  | Y | Y |  |  | unknown | 136&#8209;141 |
| 59 | `new` | Y | Y |  |  | Y |  | Y |  | 157 |
| 60 | `insert` | Y | Y |  |  | Y |  | Y |  | 160 |
| 61 | `find` | Y | Y |  |  | Y |  | Y |  | 163 |
| 62 | `contains` | Y | Y |  |  | Y |  | Y |  | 166 |
| 63 | `size` | Y | Y |  |  | Y |  | Y |  | 169 |
| 64 | `is_empty` | Y | Y |  |  | Y |  | Y |  | 172 |
| 65 | `height` | Y | Y |  |  | Y |  | Y |  | 175 |
| 66 | `minimum` | Y | Y |  |  | Y |  | Y |  | 178 |
| 67 | `maximum` | Y | Y |  |  | Y |  | Y |  | 181 |
| 68 | `in_order` | Y | Y |  |  | Y |  | Y |  | 184 |
| 69 | `pre_order` | Y | Y |  |  | Y |  | Y |  | 187 |
| 70 | `clone_link` |  |  |  | Y | Y |  | Y |  | 193&#8209;194 |
| 71 | `new_treap_link_lock` |  |  |  | Y | Y |  |  | hole | 225&#8209;226 |
| 72 | `size_link` |  |  |  | Y | Y |  |  | unknown | 232&#8209;233 |
| 73 | `update` |  |  |  | Y | Y |  |  | unknown | 241&#8209;247 |
| 74 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 256&#8209;262 |
| 75 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 286&#8209;292 |
| 76 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 318&#8209;326 |
| 77 | `find_link` |  |  |  | Y | Y |  | Y |  | 367&#8209;368 |
| 78 | `min_link` |  |  |  | Y | Y |  | Y |  | 386&#8209;387 |
| 79 | `max_link` |  |  |  | Y | Y |  | Y |  | 400&#8209;401 |
| 80 | `height_link` |  |  |  | Y | Y |  |  | unknown | 412&#8209;417 |
| 81 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 441&#8209;442 |
| 82 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 453&#8209;454 |
| 83 | `default` |  | Y |  |  | Y |  | Y |  | 537 |

### Chap39/BSTTreapStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 84 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 166&#8209;171 |
| 85 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 184&#8209;197 |
| 86 | `lemma_wf_decompose` |  |  |  | Y | Y |  |  | unknown | 210&#8209;219 |
| 87 | `lemma_wf_assemble_node` |  |  |  | Y | Y |  |  | unknown | 223&#8209;228 |
| 88 | `lemma_contains_left` |  |  |  | Y | Y |  |  | unknown | 232&#8209;234 |
| 89 | `lemma_contains_right` |  |  |  | Y | Y |  |  | unknown | 238&#8209;240 |
| 90 | `lemma_bst_decompose` |  |  |  | Y | Y |  |  | unknown | 244&#8209;254 |
| 91 | `lemma_contains_root` |  |  |  | Y | Y |  |  | unknown | 258&#8209;259 |
| 92 | `new` | Y | Y |  |  | Y |  |  | unknown | 278&#8209;283 |
| 93 | `size` | Y | Y |  |  | Y |  |  | unknown | 286&#8209;287 |
| 94 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;291 |
| 95 | `height` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;298 |
| 96 | `insert` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;306 |
| 97 | `find` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;310 |
| 98 | `contains` | Y | Y |  |  | Y |  | Y |  | 313 |
| 99 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;321 |
| 100 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 324&#8209;329 |
| 101 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 332&#8209;333 |
| 102 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 336&#8209;337 |
| 103 | `new_node` | Y |  |  |  | Y |  |  | unknown | 340&#8209;343 |
| 104 | `size_link` | Y |  |  |  | Y |  |  | unknown | 358&#8209;359 |
| 105 | `update_size` | Y |  |  |  | Y |  |  | unknown | 368&#8209;374 |
| 106 | `rotate_left` | Y |  |  |  | Y |  |  | unknown | 381&#8209;388 |
| 107 | `rotate_right` | Y |  |  |  | Y |  |  | unknown | 457&#8209;463 |
| 108 | `clone_link` | Y | Y |  |  | Y |  |  | unknown | 485&#8209;486 |
| 109 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 488&#8209;492 |
| 110 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 494&#8209;501 |
| 111 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 503&#8209;504 |
| 112 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 506&#8209;511 |
| 113 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 513&#8209;518 |
| 114 | `in_order_vec` | Y | Y |  |  | Y |  |  | unknown | 520&#8209;521 |
| 115 | `pre_order_vec` | Y | Y |  |  | Y |  |  | unknown | 523&#8209;524 |
| 116 | `default` |  | Y |  |  | Y |  |  | unknown | 739&#8209;740 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
