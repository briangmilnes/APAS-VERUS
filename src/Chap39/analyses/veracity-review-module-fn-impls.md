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
| 1 | Chap39 | BSTParaTreapMtEph | 16 | 16 | 0 | 28 | 44 | 0 | 43 | 1 | 0 |
| 2 | Chap39 | BSTSetTreapMtEph | 20 | 20 | 0 | 2 | 22 | 0 | 22 | 0 | 0 |
| 3 | Chap39 | BSTTreapMtEph | 12 | 13 | 0 | 22 | 35 | 0 | 26 | 7 | 2 |
| 4 | Chap39 | BSTTreapStEph | 34 | 35 | 0 | 0 | 35 | 0 | 35 | 0 | 0 |

## Function-by-Function Detail

### Chap39/BSTParaTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `clone_elem` |  |  |  | Y | Y |  |  | unknown | 125&#8209;126 |
| 2 | `lemma_cmp_antisymmetry` |  |  |  | Y | Y |  |  | unknown | 132&#8209;136 |
| 3 | `lemma_cmp_antisymmetry_less` |  |  |  | Y | Y |  |  | unknown | 143&#8209;147 |
| 4 | `lemma_cmp_transitivity` |  |  |  | Y | Y |  |  | unknown | 154&#8209;159 |
| 5 | `lemma_cmp_eq_subst` |  |  |  | Y | Y |  |  | unknown | 166&#8209;172 |
| 6 | `lemma_cmp_equal_congruent` |  |  |  | Y | Y |  |  | unknown | 179&#8209;184 |
| 7 | `lemma_cmp_equal_congruent_right` |  |  |  | Y | Y |  |  | unknown | 192&#8209;197 |
| 8 | `param_treap_assert_finite` |  |  |  | Y | Y |  |  | unknown | 207&#8209;208 |
| 9 | `lemma_joined_right_gt_lk` |  |  |  | Y | Y |  |  | unknown | 215&#8209;232 |
| 10 | `lemma_joined_left_lt_rk` |  |  |  | Y | Y |  |  | unknown | 253&#8209;270 |
| 11 | `new_param_treap` |  |  |  | Y | Y |  |  | unknown | 291&#8209;299 |
| 12 | `new_leaf` |  |  |  | Y | Y |  |  | unknown | 308&#8209;309 |
| 13 | `expose_internal` |  |  |  | Y | Y |  |  | unknown | 314&#8209;319 |
| 14 | `expose_with_priority_internal` |  |  |  | Y | Y |  |  | unknown | 375&#8209;380 |
| 15 | `priority_for` |  |  |  | Y | Y |  |  | hole | 412 |
| 16 | `tree_priority_internal` |  |  |  | Y | Y |  |  | unknown | 420&#8209;422 |
| 17 | `make_node` |  |  |  | Y | Y |  |  | unknown | 434&#8209;448 |
| 18 | `join_with_priority` |  |  |  | Y | Y |  |  | unknown | 480&#8209;491 |
| 19 | `split_inner` |  |  |  | Y | Y |  |  | unknown | 635&#8209;646 |
| 20 | `join_pair_inner` |  |  |  | Y | Y |  |  | unknown | 805&#8209;814 |
| 21 | `union_inner` |  |  |  | Y | Y |  |  | unknown | 1040&#8209;1046 |
| 22 | `intersect_inner` |  |  |  | Y | Y |  |  | unknown | 1221&#8209;1227 |
| 23 | `difference_inner` |  |  |  | Y | Y |  |  | unknown | 1395&#8209;1401 |
| 24 | `filter_inner` |  |  |  | Y | Y |  |  | unknown | 1574&#8209;1593 |
| 25 | `filter_parallel` |  |  |  | Y | Y |  |  | unknown | 1757&#8209;1775 |
| 26 | `reduce_inner` |  |  |  | Y | Y |  |  | unknown | 1781&#8209;1790 |
| 27 | `reduce_parallel` |  |  |  | Y | Y |  |  | unknown | 1815&#8209;1822 |
| 28 | `collect_in_order` |  |  |  | Y | Y |  |  | unknown | 1829&#8209;1835 |
| 29 | `new` | Y | Y |  |  | Y |  |  | unknown | 1857&#8209;1858 |
| 30 | `expose` | Y | Y |  |  | Y |  |  | unknown | 1860&#8209;1874 |
| 31 | `join_mid` | Y | Y |  |  | Y |  |  | unknown | 1876&#8209;1892 |
| 32 | `size` | Y | Y |  |  | Y |  |  | unknown | 1894&#8209;1895 |
| 33 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 1897&#8209;1898 |
| 34 | `insert` | Y | Y |  |  | Y |  |  | unknown | 1900&#8209;1904 |
| 35 | `delete` | Y | Y |  |  | Y |  |  | unknown | 1906&#8209;1911 |
| 36 | `find` | Y | Y |  |  | Y |  |  | unknown | 1913&#8209;1917 |
| 37 | `split` | Y | Y |  |  | Y |  |  | unknown | 1919&#8209;1929 |
| 38 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 1931&#8209;1940 |
| 39 | `union` | Y | Y |  |  | Y |  |  | unknown | 1942&#8209;1947 |
| 40 | `intersect` | Y | Y |  |  | Y |  |  | unknown | 1949&#8209;1954 |
| 41 | `difference` | Y | Y |  |  | Y |  |  | unknown | 1956&#8209;1961 |
| 42 | `filter` | Y | Y |  |  | Y |  |  | unknown | 1963&#8209;1981 |
| 43 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 1983&#8209;1990 |
| 44 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 1992&#8209;1994 |

### Chap39/BSTSetTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 45 | `empty` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 46 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;64 |
| 47 | `size` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;67 |
| 48 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 49 | `find` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 50 | `contains` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;80 |
| 51 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;87 |
| 52 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;94 |
| 53 | `insert` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;101 |
| 54 | `delete` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;108 |
| 55 | `union` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;115 |
| 56 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;122 |
| 57 | `difference` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;129 |
| 58 | `split` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;144 |
| 59 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;155 |
| 60 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;167 |
| 61 | `filter` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;181 |
| 62 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;190 |
| 63 | `iter_in_order` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;194 |
| 64 | `as_tree` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;197 |
| 65 | `minimum_inner` |  |  |  | Y | Y |  |  | unknown | 203&#8209;208 |
| 66 | `maximum_inner` |  |  |  | Y | Y |  |  | unknown | 226&#8209;231 |

### Chap39/BSTTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 67 | `lemma_bst_decompose` |  |  |  | Y | Y |  |  | unknown | 122&#8209;132 |
| 68 | `lemma_contains_left` |  |  |  | Y | Y |  |  | unknown | 136&#8209;138 |
| 69 | `lemma_contains_right` |  |  |  | Y | Y |  |  | unknown | 142&#8209;144 |
| 70 | `lemma_contains_root` |  |  |  | Y | Y |  |  | unknown | 148&#8209;149 |
| 71 | `lemma_contains_implies_in_set` |  |  |  | Y | Y |  |  | unknown | 154&#8209;157 |
| 72 | `lemma_set_of_link_finite` |  |  |  | Y | Y |  |  | unknown | 175&#8209;177 |
| 73 | `lemma_height_le_size` |  |  |  | Y | Y |  |  | unknown | 188&#8209;193 |
| 74 | `lemma_size_wf_child_bounded` |  |  |  | Y | Y |  |  | unknown | 205&#8209;218 |
| 75 | `lemma_wf_assemble_node` |  |  |  | Y | Y |  |  | unknown | 229&#8209;234 |
| 76 | `new` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;259 |
| 77 | `insert` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;263 |
| 78 | `delete` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;267 |
| 79 | `find` | Y | Y |  |  | Y |  |  | hole | 269&#8209;271 |
| 80 | `contains` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;275 |
| 81 | `size` | Y | Y |  |  | Y |  |  | hole | 277&#8209;278 |
| 82 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;281 |
| 83 | `height` | Y | Y |  |  | Y |  | Y |  | 283 |
| 84 | `minimum` | Y | Y |  |  | Y |  |  | hole | 285&#8209;286 |
| 85 | `maximum` | Y | Y |  |  | Y |  |  | hole | 288&#8209;289 |
| 86 | `in_order` | Y | Y |  |  | Y |  |  | hole | 291&#8209;292 |
| 87 | `pre_order` | Y | Y |  |  | Y |  |  | hole | 294&#8209;295 |
| 88 | `clone_link` |  |  |  | Y | Y |  |  | hole | 352&#8209;357 |
| 89 | `size_link` |  |  |  | Y | Y |  |  | unknown | 388&#8209;390 |
| 90 | `update` |  |  |  | Y | Y |  |  | unknown | 398&#8209;407 |
| 91 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 416&#8209;424 |
| 92 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 490&#8209;498 |
| 93 | `insert_link` |  |  |  | Y | Y |  |  | unknown | 566&#8209;578 |
| 94 | `delete_link` |  |  |  | Y | Y |  |  | unknown | 671&#8209;681 |
| 95 | `find_link` |  |  |  | Y | Y |  |  | unknown | 855&#8209;862 |
| 96 | `min_link` |  |  |  | Y | Y |  |  | unknown | 919&#8209;922 |
| 97 | `max_link` |  |  |  | Y | Y |  |  | unknown | 944&#8209;947 |
| 98 | `height_link` |  |  |  | Y | Y |  |  | unknown | 967&#8209;972 |
| 99 | `in_order_collect` |  |  |  | Y | Y |  |  | unknown | 996&#8209;999 |
| 100 | `pre_order_collect` |  |  |  | Y | Y |  |  | unknown | 1010&#8209;1013 |
| 101 | `default` |  | Y |  |  | Y |  | Y |  | 1150 |

### Chap39/BSTTreapStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 102 | `lemma_height_le_size` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 103 | `lemma_size_wf_child_bounded` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;109 |
| 104 | `lemma_wf_decompose` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;120 |
| 105 | `lemma_wf_assemble_node` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;127 |
| 106 | `lemma_contains_left` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 107 | `lemma_contains_right` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;135 |
| 108 | `lemma_bst_decompose` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;147 |
| 109 | `lemma_contains_root` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;150 |
| 110 | `new` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;160 |
| 111 | `size` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 112 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;168 |
| 113 | `height` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;175 |
| 114 | `insert` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;189 |
| 115 | `delete` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;200 |
| 116 | `find` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;210 |
| 117 | `contains` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;218 |
| 118 | `minimum` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;226 |
| 119 | `maximum` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;234 |
| 120 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;238 |
| 121 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;242 |
| 122 | `new_node` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;248 |
| 123 | `size_link` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;252 |
| 124 | `update_size` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;261 |
| 125 | `rotate_left` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;271 |
| 126 | `rotate_right` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;281 |
| 127 | `clone_link` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;286 |
| 128 | `height_link` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;292 |
| 129 | `insert_link` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;306 |
| 130 | `delete_link` | Y | Y |  |  | Y |  |  | unknown | 308&#8209;316 |
| 131 | `find_link` | Y | Y |  |  | Y |  |  | unknown | 318&#8209;324 |
| 132 | `min_link` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;331 |
| 133 | `max_link` | Y | Y |  |  | Y |  |  | unknown | 333&#8209;338 |
| 134 | `in_order_vec` | Y | Y |  |  | Y |  |  | unknown | 340&#8209;341 |
| 135 | `pre_order_vec` | Y | Y |  |  | Y |  |  | unknown | 343&#8209;344 |
| 136 | `default` |  | Y |  |  | Y |  |  | unknown | 1165&#8209;1166 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
