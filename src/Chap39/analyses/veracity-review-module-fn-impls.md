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
| 2 | Chap39 | BSTSetTreapMtEph | 20 | 20 | 0 | 2 | 22 | 0 | 12 | 10 | 0 |
| 3 | Chap39 | BSTTreapMtEph | 12 | 13 | 0 | 21 | 34 | 0 | 22 | 12 | 0 |
| 4 | Chap39 | BSTTreapStEph | 34 | 35 | 0 | 0 | 35 | 0 | 35 | 0 | 0 |

## Function-by-Function Detail

### Chap39/BSTParaTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_treap_lock` |  |  |  | Y | Y |  |  | hole | 84&#8209;85 |
| 2 | `new` | Y | Y |  |  | Y |  |  | hole | 363&#8209;364 |
| 3 | `expose` | Y | Y |  |  | Y |  |  | hole | 367&#8209;368 |
| 4 | `expose_with_priority` | Y | Y |  |  | Y |  |  | hole | 371&#8209;372 |
| 5 | `join_mid` | Y | Y |  |  | Y |  |  | hole | 375&#8209;376 |
| 6 | `size` | Y | Y |  |  | Y |  |  | hole | 379&#8209;380 |
| 7 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 383&#8209;384 |
| 8 | `insert` | Y | Y |  |  | Y |  |  | hole | 387&#8209;388 |
| 9 | `delete` | Y | Y |  |  | Y |  |  | hole | 391&#8209;392 |
| 10 | `find` | Y | Y |  |  | Y |  |  | hole | 395&#8209;396 |
| 11 | `split` | Y | Y |  |  | Y |  |  | hole | 399&#8209;402 |
| 12 | `join_pair` | Y | Y |  |  | Y |  |  | hole | 405&#8209;406 |
| 13 | `union` | Y | Y |  |  | Y |  |  | hole | 409&#8209;410 |
| 14 | `intersect` | Y | Y |  |  | Y |  |  | hole | 413&#8209;414 |
| 15 | `difference` | Y | Y |  |  | Y |  |  | hole | 417&#8209;418 |
| 16 | `filter` | Y | Y |  |  | Y |  |  | hole | 421&#8209;422 |
| 17 | `reduce` | Y | Y |  |  | Y |  |  | hole | 425&#8209;428 |
| 18 | `in_order` | Y | Y |  |  | Y |  |  | hole | 431&#8209;432 |
| 19 | `priority_for` |  |  |  | Y |  | Y | Y |  | 129&#8209;137 |
| 20 | `tree_priority` |  |  |  | Y |  | Y | Y |  | 139&#8209;146 |
| 21 | `tree_size` |  |  |  | Y |  | Y | Y |  | 148&#8209;155 |
| 22 | `make_node` |  |  |  | Y |  | Y | Y |  | 157&#8209;166 |
| 23 | `join_with_priority` |  |  |  | Y |  | Y | Y |  | 168&#8209;190 |
| 24 | `split_inner` |  |  |  | Y |  | Y | Y |  | 192&#8209;212 |
| 25 | `join_pair_inner` |  |  |  | Y |  | Y | Y |  | 214&#8209;227 |
| 26 | `union_inner` |  |  |  | Y |  | Y | Y |  | 229&#8209;242 |
| 27 | `intersect_inner` |  |  |  | Y |  | Y | Y |  | 244&#8209;261 |
| 28 | `difference_inner` |  |  |  | Y |  | Y | Y |  | 263&#8209;280 |
| 29 | `filter_inner` |  |  |  | Y |  | Y | Y |  | 282&#8209;300 |
| 30 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 302&#8209;308 |
| 31 | `reduce_inner` |  |  |  | Y |  | Y | Y |  | 310&#8209;331 |
| 32 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 333&#8209;340 |
| 33 | `collect_in_order` |  |  |  | Y |  | Y | Y |  | 342&#8209;354 |

### Chap39/BSTSetTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 34 | `empty` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;53 |
| 35 | `singleton` | Y | Y |  |  | Y |  |  | hole | 55&#8209;56 |
| 36 | `size` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;59 |
| 37 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 38 | `find` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 39 | `contains` | Y | Y |  |  | Y |  |  | hole | 67&#8209;68 |
| 40 | `minimum` | Y | Y |  |  | Y |  |  | hole | 70&#8209;74 |
| 41 | `maximum` | Y | Y |  |  | Y |  |  | hole | 76&#8209;80 |
| 42 | `insert` | Y | Y |  |  | Y |  |  | hole | 82&#8209;83 |
| 43 | `delete` | Y | Y |  |  | Y |  |  | hole | 85&#8209;86 |
| 44 | `union` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;89 |
| 45 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 46 | `difference` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;95 |
| 47 | `split` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;100 |
| 48 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;103 |
| 49 | `join_m` | Y | Y |  |  | Y |  |  | hole | 105&#8209;106 |
| 50 | `filter` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 51 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;114 |
| 52 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 53 | `as_tree` | Y | Y |  |  | Y |  |  | hole | 119&#8209;120 |
| 54 | `minimum_inner` |  |  |  | Y | Y |  |  | hole | 126&#8209;130 |
| 55 | `maximum_inner` |  |  |  | Y | Y |  |  | hole | 142&#8209;146 |

### Chap39/BSTTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 56 | `lemma_bst_decompose` |  |  |  | Y | Y |  |  | unknown | 92&#8209;102 |
| 57 | `lemma_contains_left` |  |  |  | Y | Y |  |  | unknown | 106&#8209;108 |
| 58 | `lemma_contains_right` |  |  |  | Y | Y |  |  | unknown | 112&#8209;114 |
| 59 | `lemma_contains_root` |  |  |  | Y | Y |  |  | unknown | 118&#8209;119 |
| 60 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 123&#8209;128 |
| 61 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 140&#8209;153 |
| 62 | `lemma_wf_assemble_node` |  |  |  | Y | Y |  |  | unknown | 164&#8209;169 |
| 63 | `new` | Y | Y |  |  | Y |  |  | hole | 191&#8209;192 |
| 64 | `insert` | Y | Y |  |  | Y |  |  | hole | 195&#8209;196 |
| 65 | `delete` | Y | Y |  |  | Y |  |  | hole | 199&#8209;200 |
| 66 | `find` | Y | Y |  |  | Y |  |  | hole | 203&#8209;206 |
| 67 | `contains` | Y | Y |  |  | Y |  |  | hole | 209&#8209;210 |
| 68 | `size` | Y | Y |  |  | Y |  |  | hole | 213&#8209;214 |
| 69 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;218 |
| 70 | `height` | Y | Y |  |  | Y |  |  | hole | 221&#8209;222 |
| 71 | `minimum` | Y | Y |  |  | Y |  |  | hole | 225&#8209;230 |
| 72 | `maximum` | Y | Y |  |  | Y |  |  | hole | 233&#8209;238 |
| 73 | `in_order` | Y | Y |  |  | Y |  |  | hole | 241&#8209;242 |
| 74 | `pre_order` | Y | Y |  |  | Y |  |  | hole | 245&#8209;246 |
| 75 | `clone_link` |  |  |  | Y | Y |  |  | unknown | 303&#8209;307 |
| 76 | `new_treap_link_lock` |  |  |  | Y | Y |  |  | hole | 338&#8209;339 |
| 77 | `size_link` |  |  |  | Y | Y |  |  | unknown | 345&#8209;346 |
| 78 | `update` |  |  |  | Y | Y |  |  | unknown | 354&#8209;360 |
| 79 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 369&#8209;377 |
| 80 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 443&#8209;451 |
| 81 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 519&#8209;531 |
| 82 | `delete_link` |  |  |  | Y | Y |  |  | unknown | 624&#8209;634 |
| 83 | `find_link` |  |  |  | Y | Y |  |  | unknown | 808&#8209;810 |
| 84 | `min_link` |  |  |  | Y | Y |  |  | unknown | 837&#8209;839 |
| 85 | `max_link` |  |  |  | Y | Y |  |  | unknown | 861&#8209;863 |
| 86 | `height_link` |  |  |  | Y | Y |  |  | unknown | 883&#8209;888 |
| 87 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 912&#8209;915 |
| 88 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 926&#8209;929 |
| 89 | `default` |  | Y |  |  | Y |  |  | unknown | 1057&#8209;1058 |

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
