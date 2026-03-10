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
| 1 | Chap39 | BSTParaTreapMtEph | 17 | 17 | 0 | 16 | 18 | 15 | 1 | 17 | 15 |
| 2 | Chap39 | BSTSetTreapMtEph | 20 | 20 | 0 | 2 | 22 | 0 | 12 | 10 | 0 |
| 3 | Chap39 | BSTTreapMtEph | 12 | 13 | 0 | 20 | 33 | 0 | 22 | 9 | 2 |
| 4 | Chap39 | BSTTreapStEph | 34 | 35 | 0 | 0 | 35 | 0 | 35 | 0 | 0 |

## Function-by-Function Detail

### Chap39/BSTParaTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_param_treap_arc` |  |  |  | Y | Y |  |  | unknown | 84&#8209;88 |
| 2 | `new` | Y | Y |  |  | Y |  |  | hole | 366&#8209;367 |
| 3 | `expose` | Y | Y |  |  | Y |  |  | hole | 370&#8209;371 |
| 4 | `expose_with_priority` | Y | Y |  |  | Y |  |  | hole | 374&#8209;375 |
| 5 | `join_mid` | Y | Y |  |  | Y |  |  | hole | 378&#8209;379 |
| 6 | `size` | Y | Y |  |  | Y |  |  | hole | 382&#8209;383 |
| 7 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 386&#8209;387 |
| 8 | `insert` | Y | Y |  |  | Y |  |  | hole | 390&#8209;391 |
| 9 | `delete` | Y | Y |  |  | Y |  |  | hole | 394&#8209;395 |
| 10 | `find` | Y | Y |  |  | Y |  |  | hole | 398&#8209;399 |
| 11 | `split` | Y | Y |  |  | Y |  |  | hole | 402&#8209;405 |
| 12 | `join_pair` | Y | Y |  |  | Y |  |  | hole | 408&#8209;409 |
| 13 | `union` | Y | Y |  |  | Y |  |  | hole | 412&#8209;413 |
| 14 | `intersect` | Y | Y |  |  | Y |  |  | hole | 416&#8209;417 |
| 15 | `difference` | Y | Y |  |  | Y |  |  | hole | 420&#8209;421 |
| 16 | `filter` | Y | Y |  |  | Y |  |  | hole | 424&#8209;425 |
| 17 | `reduce` | Y | Y |  |  | Y |  |  | hole | 428&#8209;431 |
| 18 | `in_order` | Y | Y |  |  | Y |  |  | hole | 434&#8209;435 |
| 19 | `priority_for` |  |  |  | Y |  | Y | Y |  | 132&#8209;140 |
| 20 | `tree_priority` |  |  |  | Y |  | Y | Y |  | 142&#8209;149 |
| 21 | `tree_size` |  |  |  | Y |  | Y | Y |  | 151&#8209;158 |
| 22 | `make_node` |  |  |  | Y |  | Y | Y |  | 160&#8209;169 |
| 23 | `join_with_priority` |  |  |  | Y |  | Y | Y |  | 171&#8209;193 |
| 24 | `split_inner` |  |  |  | Y |  | Y | Y |  | 195&#8209;215 |
| 25 | `join_pair_inner` |  |  |  | Y |  | Y | Y |  | 217&#8209;230 |
| 26 | `union_inner` |  |  |  | Y |  | Y | Y |  | 232&#8209;245 |
| 27 | `intersect_inner` |  |  |  | Y |  | Y | Y |  | 247&#8209;264 |
| 28 | `difference_inner` |  |  |  | Y |  | Y | Y |  | 266&#8209;283 |
| 29 | `filter_inner` |  |  |  | Y |  | Y | Y |  | 285&#8209;303 |
| 30 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 305&#8209;311 |
| 31 | `reduce_inner` |  |  |  | Y |  | Y | Y |  | 313&#8209;334 |
| 32 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 336&#8209;343 |
| 33 | `collect_in_order` |  |  |  | Y |  | Y | Y |  | 345&#8209;357 |

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
| 56 | `lemma_bst_decompose` |  |  |  | Y | Y |  |  | unknown | 107&#8209;117 |
| 57 | `lemma_contains_left` |  |  |  | Y | Y |  |  | unknown | 121&#8209;123 |
| 58 | `lemma_contains_right` |  |  |  | Y | Y |  |  | unknown | 127&#8209;129 |
| 59 | `lemma_contains_root` |  |  |  | Y | Y |  |  | unknown | 133&#8209;134 |
| 60 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 138&#8209;143 |
| 61 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 155&#8209;168 |
| 62 | `lemma_wf_assemble_node` |  |  |  | Y | Y |  |  | unknown | 179&#8209;184 |
| 63 | `new` | Y | Y |  |  | Y |  |  | hole | 206&#8209;207 |
| 64 | `insert` | Y | Y |  |  | Y |  |  | hole | 209&#8209;211 |
| 65 | `delete` | Y | Y |  |  | Y |  |  | hole | 213&#8209;215 |
| 66 | `find` | Y | Y |  |  | Y |  |  | hole | 217&#8209;218 |
| 67 | `contains` | Y | Y |  |  | Y |  |  | unknown | 220&#8209;221 |
| 68 | `size` | Y | Y |  |  | Y |  |  | hole | 223&#8209;224 |
| 69 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;227 |
| 70 | `height` | Y | Y |  |  | Y |  | Y |  | 229 |
| 71 | `minimum` | Y | Y |  |  | Y |  |  | hole | 231&#8209;232 |
| 72 | `maximum` | Y | Y |  |  | Y |  |  | hole | 234&#8209;235 |
| 73 | `in_order` | Y | Y |  |  | Y |  |  | hole | 237&#8209;238 |
| 74 | `pre_order` | Y | Y |  |  | Y |  |  | hole | 240&#8209;241 |
| 75 | `clone_link` |  |  |  | Y | Y |  |  | unknown | 298&#8209;302 |
| 76 | `size_link` |  |  |  | Y | Y |  |  | unknown | 333&#8209;334 |
| 77 | `update` |  |  |  | Y | Y |  |  | unknown | 342&#8209;348 |
| 78 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 357&#8209;365 |
| 79 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 431&#8209;439 |
| 80 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 507&#8209;519 |
| 81 | `delete_link` |  |  |  | Y | Y |  |  | unknown | 612&#8209;622 |
| 82 | `find_link` |  |  |  | Y | Y |  |  | unknown | 796&#8209;798 |
| 83 | `min_link` |  |  |  | Y | Y |  |  | unknown | 825&#8209;827 |
| 84 | `max_link` |  |  |  | Y | Y |  |  | unknown | 849&#8209;851 |
| 85 | `height_link` |  |  |  | Y | Y |  |  | unknown | 871&#8209;876 |
| 86 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 900&#8209;903 |
| 87 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 914&#8209;917 |
| 88 | `default` |  | Y |  |  | Y |  | Y |  | 1041 |

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
