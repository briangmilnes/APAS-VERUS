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
| 1 | Chap38 | BSTParaMtEph | 18 | 18 | 0 | 21 | 39 | 0 | 39 | 0 | 0 |
| 2 | Chap38 | BSTParaStEph | 20 | 20 | 0 | 9 | 29 | 0 | 29 | 0 | 0 |

## Function-by-Function Detail

### Chap38/BSTParaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_param_bst` |  |  |  | Y | Y |  |  | unknown | 96&#8209;105 |
| 2 | `clone_elem` |  |  |  | Y | Y |  |  | unknown | 155&#8209;157 |
| 3 | `lemma_cmp_antisymmetry` |  |  |  | Y | Y |  |  | unknown | 165&#8209;170 |
| 4 | `lemma_cmp_transitivity` |  |  |  | Y | Y |  |  | unknown | 177&#8209;183 |
| 5 | `lemma_cmp_eq_subst` |  |  |  | Y | Y |  |  | unknown | 190&#8209;197 |
| 6 | `lemma_cmp_equal_congruent` |  |  |  | Y | Y |  |  | unknown | 204&#8209;210 |
| 7 | `lemma_cmp_equal_congruent_right` |  |  |  | Y | Y |  |  | unknown | 217&#8209;223 |
| 8 | `lemma_cmp_order_axioms` |  |  |  | Y | Y |  |  | unknown | 231&#8209;237 |
| 9 | `new` | Y | Y |  |  | Y |  |  | unknown | 247&#8209;248 |
| 10 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;254 |
| 11 | `expose` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;258 |
| 12 | `join_mid` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;263 |
| 13 | `size` | Y | Y |  |  | Y |  |  | unknown | 277&#8209;278 |
| 14 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;282 |
| 15 | `insert` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;290 |
| 16 | `delete` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;295 |
| 17 | `find` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;300 |
| 18 | `split` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;308 |
| 19 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;316 |
| 20 | `join_pair_inner` | Y | Y |  |  | Y |  |  | unknown | 318&#8209;327 |
| 21 | `union` | Y | Y |  |  | Y |  |  | unknown | 330&#8209;335 |
| 22 | `intersect` | Y | Y |  |  | Y |  |  | unknown | 338&#8209;340 |
| 23 | `difference` | Y | Y |  |  | Y |  |  | unknown | 343&#8209;345 |
| 24 | `filter` | Y | Y |  |  | Y |  |  | unknown | 348&#8209;365 |
| 25 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 369&#8209;370 |
| 26 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 373&#8209;374 |
| 27 | `new_leaf` |  |  |  | Y | Y |  |  | unknown | 691&#8209;692 |
| 28 | `expose_internal` |  |  |  | Y | Y |  |  | unknown | 697&#8209;717 |
| 29 | `split_inner` |  |  |  | Y | Y |  |  | unknown | 756&#8209;773 |
| 30 | `find_recursive` |  |  |  | Y | Y |  |  | unknown | 922&#8209;928 |
| 31 | `min_key` |  |  |  | Y | Y |  |  | unknown | 950&#8209;956 |
| 32 | `union_inner` |  |  |  | Y | Y |  |  | unknown | 970&#8209;976 |
| 33 | `intersect_inner` |  |  |  | Y | Y |  |  | unknown | 1084&#8209;1090 |
| 34 | `difference_inner` |  |  |  | Y | Y |  |  | unknown | 1256&#8209;1262 |
| 35 | `filter_inner` |  |  |  | Y | Y |  |  | unknown | 1427&#8209;1446 |
| 36 | `filter_parallel` |  |  |  | Y | Y |  |  | unknown | 1546&#8209;1563 |
| 37 | `reduce_inner` |  |  |  | Y | Y |  |  | unknown | 1570&#8209;1579 |
| 38 | `reduce_parallel` |  |  |  | Y | Y |  |  | unknown | 1608&#8209;1615 |
| 39 | `collect_in_order` |  |  |  | Y | Y |  |  | unknown | 1622&#8209;1626 |

### Chap38/BSTParaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 40 | `new_param_bst` |  |  |  | Y | Y |  |  | unknown | 97&#8209;106 |
| 41 | `clone_elem` |  |  |  | Y | Y |  |  | unknown | 156&#8209;158 |
| 42 | `lemma_cmp_antisymmetry` |  |  |  | Y | Y |  |  | unknown | 166&#8209;171 |
| 43 | `lemma_cmp_transitivity` |  |  |  | Y | Y |  |  | unknown | 178&#8209;184 |
| 44 | `lemma_cmp_eq_subst` |  |  |  | Y | Y |  |  | unknown | 191&#8209;198 |
| 45 | `lemma_cmp_equal_congruent` |  |  |  | Y | Y |  |  | unknown | 209&#8209;215 |
| 46 | `lemma_cmp_equal_congruent_right` |  |  |  | Y | Y |  |  | unknown | 229&#8209;235 |
| 47 | `new` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;250 |
| 48 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;257 |
| 49 | `expose` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;264 |
| 50 | `join_mid` | Y | Y |  |  | Y |  |  | unknown | 277&#8209;279 |
| 51 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;302 |
| 52 | `size` | Y | Y |  |  | Y |  |  | unknown | 305&#8209;306 |
| 53 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;310 |
| 54 | `insert` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;321 |
| 55 | `delete` | Y | Y |  |  | Y |  |  | unknown | 324&#8209;332 |
| 56 | `find` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;339 |
| 57 | `split` | Y | Y |  |  | Y |  |  | unknown | 342&#8209;355 |
| 58 | `min_key` | Y | Y |  |  | Y |  |  | unknown | 358&#8209;366 |
| 59 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 369&#8209;377 |
| 60 | `union` | Y | Y |  |  | Y |  |  | unknown | 380&#8209;385 |
| 61 | `intersect` | Y | Y |  |  | Y |  |  | unknown | 388&#8209;392 |
| 62 | `difference` | Y | Y |  |  | Y |  |  | unknown | 395&#8209;399 |
| 63 | `filter` | Y | Y |  |  | Y |  |  | unknown | 402&#8209;420 |
| 64 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 424&#8209;428 |
| 65 | `collect_in_order` | Y | Y |  |  | Y |  |  | unknown | 431&#8209;434 |
| 66 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 437&#8209;440 |
| 67 | `filter_inner` |  |  |  | Y | Y |  |  | unknown | 1490&#8209;1509 |
| 68 | `reduce_inner` |  |  |  | Y | Y |  |  | unknown | 1564&#8209;1573 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
