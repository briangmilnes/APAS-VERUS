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
| 3 | Chap39 | BSTTreapMtEph | 11 | 12 | 0 | 16 | 28 | 0 | 10 | 11 | 7 |
| 4 | Chap39 | BSTTreapStEph | 32 | 33 | 0 | 0 | 33 | 0 | 33 | 0 | 0 |

## Function-by-Function Detail

### Chap39/BSTParaTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_treap_lock` |  |  |  | Y | Y |  |  | hole | 79&#8209;80 |
| 2 | `priority_for` |  |  |  | Y |  | Y | Y |  | 124&#8209;132 |
| 3 | `tree_priority` |  |  |  | Y |  | Y | Y |  | 134&#8209;141 |
| 4 | `tree_size` |  |  |  | Y |  | Y | Y |  | 143&#8209;150 |
| 5 | `make_node` |  |  |  | Y |  | Y | Y |  | 152&#8209;161 |
| 6 | `join_with_priority` |  |  |  | Y |  | Y | Y |  | 163&#8209;185 |
| 7 | `split_inner` |  |  |  | Y |  | Y | Y |  | 187&#8209;207 |
| 8 | `join_pair_inner` |  |  |  | Y |  | Y | Y |  | 209&#8209;222 |
| 9 | `union_inner` |  |  |  | Y |  | Y | Y |  | 224&#8209;237 |
| 10 | `intersect_inner` |  |  |  | Y |  | Y | Y |  | 239&#8209;256 |
| 11 | `difference_inner` |  |  |  | Y |  | Y | Y |  | 258&#8209;275 |
| 12 | `filter_inner` |  |  |  | Y |  | Y | Y |  | 277&#8209;295 |
| 13 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 297&#8209;303 |
| 14 | `reduce_inner` |  |  |  | Y |  | Y | Y |  | 305&#8209;326 |
| 15 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 328&#8209;335 |
| 16 | `collect_in_order` |  |  |  | Y |  | Y | Y |  | 337&#8209;349 |
| 17 | `new` | Y | Y |  |  |  | Y | Y |  | 354&#8209;356 |
| 18 | `expose` | Y | Y |  |  |  | Y | Y |  | 357&#8209;359 |
| 19 | `expose_with_priority` | Y | Y |  |  |  | Y | Y |  | 360&#8209;362 |
| 20 | `join_mid` | Y | Y |  |  |  | Y | Y |  | 363&#8209;365 |
| 21 | `size` | Y | Y |  |  |  | Y | Y |  | 366&#8209;368 |
| 22 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 369&#8209;371 |
| 23 | `insert` | Y | Y |  |  |  | Y | Y |  | 372&#8209;374 |
| 24 | `delete` | Y | Y |  |  |  | Y | Y |  | 375&#8209;377 |
| 25 | `find` | Y | Y |  |  |  | Y | Y |  | 378&#8209;380 |
| 26 | `split` | Y | Y |  |  |  | Y | Y |  | 381&#8209;383 |
| 27 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 384&#8209;386 |
| 28 | `union` | Y | Y |  |  |  | Y | Y |  | 387&#8209;389 |
| 29 | `intersect` | Y | Y |  |  |  | Y | Y |  | 390&#8209;392 |
| 30 | `difference` | Y | Y |  |  |  | Y | Y |  | 393&#8209;395 |
| 31 | `filter` | Y | Y |  |  |  | Y | Y |  | 396&#8209;398 |
| 32 | `reduce` | Y | Y |  |  |  | Y | Y |  | 399&#8209;403 |
| 33 | `in_order` | Y | Y |  |  |  | Y | Y |  | 404&#8209;406 |

### Chap39/BSTSetTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 34 | `minimum_inner` |  |  |  | Y |  | Y | Y |  | 50&#8209;58 |
| 35 | `maximum_inner` |  |  |  | Y |  | Y | Y |  | 60&#8209;68 |
| 36 | `empty` | Y | Y |  |  |  | Y | Y |  | 73&#8209;74 |
| 37 | `singleton` | Y | Y |  |  |  | Y | Y |  | 75&#8209;76 |
| 38 | `size` | Y | Y |  |  |  | Y | Y |  | 77&#8209;78 |
| 39 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 79&#8209;80 |
| 40 | `find` | Y | Y |  |  |  | Y | Y |  | 81&#8209;82 |
| 41 | `contains` | Y | Y |  |  |  | Y | Y |  | 83&#8209;84 |
| 42 | `minimum` | Y | Y |  |  |  | Y | Y |  | 85&#8209;86 |
| 43 | `maximum` | Y | Y |  |  |  | Y | Y |  | 87&#8209;88 |
| 44 | `insert` | Y | Y |  |  |  | Y | Y |  | 89&#8209;90 |
| 45 | `delete` | Y | Y |  |  |  | Y | Y |  | 91&#8209;92 |
| 46 | `union` | Y | Y |  |  |  | Y | Y |  | 93&#8209;94 |
| 47 | `intersection` | Y | Y |  |  |  | Y | Y |  | 95&#8209;96 |
| 48 | `difference` | Y | Y |  |  |  | Y | Y |  | 97&#8209;98 |
| 49 | `split` | Y | Y |  |  |  | Y | Y |  | 99&#8209;100 |
| 50 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 101&#8209;102 |
| 51 | `join_m` | Y | Y |  |  |  | Y | Y |  | 103&#8209;104 |
| 52 | `filter` | Y | Y |  |  |  | Y | Y |  | 105&#8209;106 |
| 53 | `reduce` | Y | Y |  |  |  | Y | Y |  | 107&#8209;110 |
| 54 | `iter_in_order` | Y | Y |  |  |  | Y | Y |  | 111&#8209;112 |
| 55 | `as_tree` | Y | Y |  |  |  | Y | Y |  | 113&#8209;114 |

### Chap39/BSTTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 56 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 106&#8209;111 |
| 57 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 123&#8209;136 |
| 58 | `lemma_wf_assemble_node` |  |  |  | Y | Y |  |  | unknown | 147&#8209;152 |
| 59 | `new` | Y | Y |  |  | Y |  |  | hole | 167&#8209;168 |
| 60 | `insert` | Y | Y |  |  | Y |  |  | hole | 171&#8209;172 |
| 61 | `find` | Y | Y |  |  | Y |  |  | hole | 175&#8209;178 |
| 62 | `contains` | Y | Y |  |  | Y |  |  | hole | 181&#8209;182 |
| 63 | `size` | Y | Y |  |  | Y |  |  | hole | 185&#8209;186 |
| 64 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;190 |
| 65 | `height` | Y | Y |  |  | Y |  |  | hole | 193&#8209;194 |
| 66 | `minimum` | Y | Y |  |  | Y |  |  | hole | 197&#8209;200 |
| 67 | `maximum` | Y | Y |  |  | Y |  |  | hole | 203&#8209;206 |
| 68 | `in_order` | Y | Y |  |  | Y |  |  | hole | 209&#8209;210 |
| 69 | `pre_order` | Y | Y |  |  | Y |  |  | hole | 213&#8209;214 |
| 70 | `clone_link` |  |  |  | Y | Y |  | Y |  | 220&#8209;221 |
| 71 | `new_treap_link_lock` |  |  |  | Y | Y |  |  | hole | 252&#8209;253 |
| 72 | `size_link` |  |  |  | Y | Y |  |  | unknown | 259&#8209;260 |
| 73 | `update` |  |  |  | Y | Y |  |  | unknown | 268&#8209;274 |
| 74 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 283&#8209;289 |
| 75 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 313&#8209;319 |
| 76 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 345&#8209;353 |
| 77 | `find_link` |  |  |  | Y | Y |  | Y |  | 394&#8209;395 |
| 78 | `min_link` |  |  |  | Y | Y |  | Y |  | 413&#8209;414 |
| 79 | `max_link` |  |  |  | Y | Y |  | Y |  | 427&#8209;428 |
| 80 | `height_link` |  |  |  | Y | Y |  |  | unknown | 439&#8209;444 |
| 81 | `in_order_collect` |  |  |  | Y | Y |  | Y |  | 468&#8209;469 |
| 82 | `pre_order_collect` |  |  |  | Y | Y |  | Y |  | 480&#8209;481 |
| 83 | `default` |  | Y |  |  | Y |  | Y |  | 598 |

### Chap39/BSTTreapStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 84 | `lemma_height_le_size` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 85 | `lemma_size_wf_child_bounded` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;109 |
| 86 | `lemma_wf_decompose` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;120 |
| 87 | `lemma_wf_assemble_node` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;127 |
| 88 | `lemma_contains_left` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 89 | `lemma_contains_right` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;135 |
| 90 | `lemma_bst_decompose` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;147 |
| 91 | `lemma_contains_root` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;150 |
| 92 | `new` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;160 |
| 93 | `size` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 94 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;168 |
| 95 | `height` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;175 |
| 96 | `insert` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;188 |
| 97 | `find` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;192 |
| 98 | `contains` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;196 |
| 99 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;204 |
| 100 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;212 |
| 101 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;216 |
| 102 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;220 |
| 103 | `new_node` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;226 |
| 104 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;230 |
| 105 | `update_size` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;239 |
| 106 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;249 |
| 107 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;259 |
| 108 | `clone_link` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;264 |
| 109 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;270 |
| 110 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;283 |
| 111 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;286 |
| 112 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;293 |
| 113 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;300 |
| 114 | `in_order_vec` | Y | Y |  |  | Y |  |  | unknown | 302&#8209;303 |
| 115 | `pre_order_vec` | Y | Y |  |  | Y |  |  | unknown | 305&#8209;306 |
| 116 | `default` |  | Y |  |  | Y |  |  | unknown | 885&#8209;886 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
