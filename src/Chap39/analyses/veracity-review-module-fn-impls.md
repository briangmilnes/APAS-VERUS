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
| 1 | Chap39 | BSTParaTreapMtEph | 17 | 17 | 0 | 16 | 18 | 15 | 0 | 18 | 15 |
| 2 | Chap39 | BSTSetTreapMtEph | 20 | 20 | 0 | 2 | 0 | 22 | 0 | 0 | 22 |
| 3 | Chap39 | BSTTreapMtEph | 12 | 13 | 0 | 21 | 34 | 0 | 22 | 12 | 0 |
| 4 | Chap39 | BSTTreapStEph | 34 | 35 | 0 | 0 | 35 | 0 | 35 | 0 | 0 |

## Function-by-Function Detail

### Chap39/BSTParaTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_treap_lock` |  |  |  | Y | Y |  |  | hole | 72&#8209;73 |
| 2 | `new` | Y | Y |  |  | Y |  |  | hole | 351 |
| 3 | `expose` | Y | Y |  |  | Y |  |  | hole | 354 |
| 4 | `expose_with_priority` | Y | Y |  |  | Y |  |  | hole | 357 |
| 5 | `join_mid` | Y | Y |  |  | Y |  |  | hole | 360 |
| 6 | `size` | Y | Y |  |  | Y |  |  | hole | 363 |
| 7 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 366 |
| 8 | `insert` | Y | Y |  |  | Y |  |  | hole | 369 |
| 9 | `delete` | Y | Y |  |  | Y |  |  | hole | 372 |
| 10 | `find` | Y | Y |  |  | Y |  |  | hole | 375 |
| 11 | `split` | Y | Y |  |  | Y |  |  | hole | 378 |
| 12 | `join_pair` | Y | Y |  |  | Y |  |  | hole | 381 |
| 13 | `union` | Y | Y |  |  | Y |  |  | hole | 384 |
| 14 | `intersect` | Y | Y |  |  | Y |  |  | hole | 387 |
| 15 | `difference` | Y | Y |  |  | Y |  |  | hole | 390 |
| 16 | `filter` | Y | Y |  |  | Y |  |  | hole | 393 |
| 17 | `reduce` | Y | Y |  |  | Y |  |  | hole | 396&#8209;398 |
| 18 | `in_order` | Y | Y |  |  | Y |  |  | hole | 401 |
| 19 | `priority_for` |  |  |  | Y |  | Y | Y |  | 117&#8209;125 |
| 20 | `tree_priority` |  |  |  | Y |  | Y | Y |  | 127&#8209;134 |
| 21 | `tree_size` |  |  |  | Y |  | Y | Y |  | 136&#8209;143 |
| 22 | `make_node` |  |  |  | Y |  | Y | Y |  | 145&#8209;154 |
| 23 | `join_with_priority` |  |  |  | Y |  | Y | Y |  | 156&#8209;178 |
| 24 | `split_inner` |  |  |  | Y |  | Y | Y |  | 180&#8209;200 |
| 25 | `join_pair_inner` |  |  |  | Y |  | Y | Y |  | 202&#8209;215 |
| 26 | `union_inner` |  |  |  | Y |  | Y | Y |  | 217&#8209;230 |
| 27 | `intersect_inner` |  |  |  | Y |  | Y | Y |  | 232&#8209;249 |
| 28 | `difference_inner` |  |  |  | Y |  | Y | Y |  | 251&#8209;268 |
| 29 | `filter_inner` |  |  |  | Y |  | Y | Y |  | 270&#8209;288 |
| 30 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 290&#8209;296 |
| 31 | `reduce_inner` |  |  |  | Y |  | Y | Y |  | 298&#8209;319 |
| 32 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 321&#8209;328 |
| 33 | `collect_in_order` |  |  |  | Y |  | Y | Y |  | 330&#8209;342 |

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
| 56 | `lemma_bst_decompose` |  |  |  | Y | Y |  |  | unknown | 139&#8209;149 |
| 57 | `lemma_contains_left` |  |  |  | Y | Y |  |  | unknown | 153&#8209;155 |
| 58 | `lemma_contains_right` |  |  |  | Y | Y |  |  | unknown | 159&#8209;161 |
| 59 | `lemma_contains_root` |  |  |  | Y | Y |  |  | unknown | 165&#8209;166 |
| 60 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 170&#8209;175 |
| 61 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 187&#8209;200 |
| 62 | `lemma_wf_assemble_node` |  |  |  | Y | Y |  |  | unknown | 211&#8209;216 |
| 63 | `new` | Y | Y |  |  | Y |  |  | hole | 231&#8209;232 |
| 64 | `insert` | Y | Y |  |  | Y |  |  | hole | 235&#8209;236 |
| 65 | `delete` | Y | Y |  |  | Y |  |  | hole | 239&#8209;240 |
| 66 | `find` | Y | Y |  |  | Y |  |  | hole | 243&#8209;246 |
| 67 | `contains` | Y | Y |  |  | Y |  |  | hole | 249&#8209;250 |
| 68 | `size` | Y | Y |  |  | Y |  |  | hole | 253&#8209;254 |
| 69 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;258 |
| 70 | `height` | Y | Y |  |  | Y |  |  | hole | 261&#8209;262 |
| 71 | `minimum` | Y | Y |  |  | Y |  |  | hole | 265&#8209;268 |
| 72 | `maximum` | Y | Y |  |  | Y |  |  | hole | 271&#8209;274 |
| 73 | `in_order` | Y | Y |  |  | Y |  |  | hole | 277&#8209;278 |
| 74 | `pre_order` | Y | Y |  |  | Y |  |  | hole | 281&#8209;282 |
| 75 | `clone_link` |  |  |  | Y | Y |  |  | unknown | 288&#8209;292 |
| 76 | `new_treap_link_lock` |  |  |  | Y | Y |  |  | hole | 323&#8209;324 |
| 77 | `size_link` |  |  |  | Y | Y |  |  | unknown | 330&#8209;331 |
| 78 | `update` |  |  |  | Y | Y |  |  | unknown | 339&#8209;345 |
| 79 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 354&#8209;362 |
| 80 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 428&#8209;436 |
| 81 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 504&#8209;516 |
| 82 | `delete_link` |  |  |  | Y | Y |  |  | unknown | 609&#8209;619 |
| 83 | `find_link` |  |  |  | Y | Y |  |  | unknown | 793&#8209;795 |
| 84 | `min_link` |  |  |  | Y | Y |  |  | unknown | 822&#8209;824 |
| 85 | `max_link` |  |  |  | Y | Y |  |  | unknown | 846&#8209;848 |
| 86 | `height_link` |  |  |  | Y | Y |  |  | unknown | 868&#8209;873 |
| 87 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 897&#8209;900 |
| 88 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 911&#8209;914 |
| 89 | `default` |  | Y |  |  | Y |  |  | unknown | 1038&#8209;1039 |

### Chap39/BSTTreapStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 90 | `lemma_height_le_size` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 91 | `lemma_size_wf_child_bounded` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;109 |
| 92 | `lemma_wf_decompose` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;120 |
| 93 | `lemma_wf_assemble_node` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;127 |
| 94 | `lemma_contains_left` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 95 | `lemma_contains_right` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;135 |
| 96 | `lemma_bst_decompose` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;147 |
| 97 | `lemma_contains_root` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;150 |
| 98 | `new` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;160 |
| 99 | `size` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 100 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;168 |
| 101 | `height` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;175 |
| 102 | `insert` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;188 |
| 103 | `delete` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;199 |
| 104 | `find` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;203 |
| 105 | `contains` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;207 |
| 106 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;215 |
| 107 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;223 |
| 108 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;227 |
| 109 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;231 |
| 110 | `new_node` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;237 |
| 111 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;241 |
| 112 | `update_size` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;250 |
| 113 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;260 |
| 114 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;270 |
| 115 | `clone_link` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;275 |
| 116 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 277&#8209;281 |
| 117 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;294 |
| 118 | `delete_link` | Y | Y |  |  | Y |  |  | unknown | 296&#8209;304 |
| 119 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 306&#8209;307 |
| 120 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;314 |
| 121 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;321 |
| 122 | `in_order_vec` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;324 |
| 123 | `pre_order_vec` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;327 |
| 124 | `default` |  | Y |  |  | Y |  |  | unknown | 1092&#8209;1093 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
