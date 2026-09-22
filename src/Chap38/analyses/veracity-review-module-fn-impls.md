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
| 1 | Chap38 | BSTParaMtEph | 21 | 21 | 0 | 15 | 36 | 0 | 36 | 0 | 0 |
| 2 | Chap38 | BSTParaSpecsAndLemmas | 0 | 0 | 0 | 6 | 6 | 0 | 6 | 0 | 0 |
| 3 | Chap38 | BSTParaStEph | 21 | 21 | 1 | 5 | 27 | 0 | 27 | 0 | 0 |

## Function-by-Function Detail

### Chap38/BSTParaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_param_bst` |  |  |  | Y | Y |  |  | unknown | 85&#8209;93 |
| 2 | `clone_elem` |  |  |  | Y | Y |  |  | unknown | 105&#8209;107 |
| 3 | `new` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;202 |
| 4 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;208 |
| 5 | `expose` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;215 |
| 6 | `join_mid` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;228 |
| 7 | `size` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;242 |
| 8 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;246 |
| 9 | `insert` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;257 |
| 10 | `delete` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;267 |
| 11 | `find` | Y | Y |  |  | Y |  |  | unknown | 270&#8209;272 |
| 12 | `split` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;284 |
| 13 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 287&#8209;295 |
| 14 | `join_pair_inner` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;307 |
| 15 | `union` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;315 |
| 16 | `intersect` | Y | Y |  |  | Y |  |  | unknown | 318&#8209;320 |
| 17 | `difference` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;325 |
| 18 | `filter` | Y | Y |  |  | Y |  |  | unknown | 328&#8209;344 |
| 19 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 348&#8209;350 |
| 20 | `min_key` | Y | Y |  |  | Y |  |  | unknown | 353&#8209;361 |
| 21 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 364&#8209;372 |
| 22 | `collect_in_order` | Y | Y |  |  | Y |  |  | unknown | 374&#8209;381 |
| 23 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 384&#8209;388 |
| 24 | `new_leaf` |  |  |  | Y | Y |  |  | unknown | 791&#8209;792 |
| 25 | `expose_internal` |  |  |  | Y | Y |  |  | unknown | 798&#8209;815 |
| 26 | `split_inner` |  |  |  | Y | Y |  |  | unknown | 849&#8209;864 |
| 27 | `find_recursive` |  |  |  | Y | Y |  |  | unknown | 1009&#8209;1014 |
| 28 | `min_key_inner` |  |  |  | Y | Y |  |  | unknown | 1041&#8209;1051 |
| 29 | `union_inner` |  |  |  | Y | Y |  |  | unknown | 1100&#8209;1106 |
| 30 | `intersect_inner` |  |  |  | Y | Y |  |  | unknown | 1177&#8209;1182 |
| 31 | `difference_inner` |  |  |  | Y | Y |  |  | unknown | 1298&#8209;1303 |
| 32 | `filter_inner` |  |  |  | Y | Y |  |  | unknown | 1419&#8209;1438 |
| 33 | `filter_parallel` |  |  |  | Y | Y |  |  | unknown | 1505&#8209;1521 |
| 34 | `reduce_inner` |  |  |  | Y | Y |  |  | unknown | 1530&#8209;1538 |
| 35 | `reduce_parallel` |  |  |  | Y | Y |  |  | unknown | 1571&#8209;1578 |
| 36 | `collect_in_order_inner` |  |  |  | Y | Y |  |  | unknown | 1586&#8209;1594 |

### Chap38/BSTParaSpecsAndLemmas.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 37 | `lemma_cmp_antisymmetry` |  |  |  | Y | Y |  |  | unknown | 42&#8209;47 |
| 38 | `lemma_cmp_transitivity` |  |  |  | Y | Y |  |  | unknown | 54&#8209;60 |
| 39 | `lemma_cmp_eq_subst` |  |  |  | Y | Y |  |  | unknown | 67&#8209;74 |
| 40 | `lemma_cmp_equal_congruent` |  |  |  | Y | Y |  |  | unknown | 81&#8209;87 |
| 41 | `lemma_cmp_equal_congruent_right` |  |  |  | Y | Y |  |  | unknown | 94&#8209;100 |
| 42 | `lemma_cmp_order_axioms` |  |  |  | Y | Y |  |  | unknown | 107&#8209;113 |

### Chap38/BSTParaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 43 | `new_param_bst` |  |  |  | Y | Y |  |  | unknown | 87&#8209;95 |
| 44 | `reveal_param_bst_backings` |  |  |  | Y | Y |  |  | unknown | 108&#8209;109 |
| 45 | `clone_elem` |  |  |  | Y | Y |  |  | unknown | 118&#8209;120 |
| 46 | `filter_inner` |  |  |  | Y | Y |  |  | unknown | 133&#8209;150 |
| 47 | `reduce_inner` |  |  |  | Y | Y |  |  | unknown | 196&#8209;204 |
| 48 | `new` | Y | Y |  |  | Y |  |  | unknown | 296&#8209;297 |
| 49 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;303 |
| 50 | `expose` | Y | Y |  |  | Y |  |  | unknown | 306&#8209;310 |
| 51 | `join_mid` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;323 |
| 52 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 336&#8209;344 |
| 53 | `size` | Y | Y |  |  | Y |  |  | unknown | 347&#8209;348 |
| 54 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 351&#8209;352 |
| 55 | `insert` | Y | Y |  |  | Y |  |  | unknown | 355&#8209;363 |
| 56 | `delete` | Y | Y |  |  | Y |  |  | unknown | 366&#8209;374 |
| 57 | `find` | Y | Y |  |  | Y |  |  | unknown | 377&#8209;381 |
| 58 | `split` | Y | Y |  |  | Y |  |  | unknown | 384&#8209;395 |
| 59 | `min_key` | Y | Y |  |  | Y |  |  | unknown | 398&#8209;406 |
| 60 | `max_key` | Y | Y |  |  | Y |  |  | unknown | 409&#8209;417 |
| 61 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 420&#8209;427 |
| 62 | `union` | Y | Y |  |  | Y |  |  | unknown | 430&#8209;435 |
| 63 | `intersect` | Y | Y |  |  | Y |  |  | unknown | 438&#8209;442 |
| 64 | `difference` | Y | Y |  |  | Y |  |  | unknown | 445&#8209;449 |
| 65 | `filter` | Y | Y |  |  | Y |  |  | unknown | 452&#8209;468 |
| 66 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 472&#8209;475 |
| 67 | `collect_in_order` | Y | Y |  |  | Y |  |  | unknown | 477&#8209;484 |
| 68 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 487&#8209;491 |
| 69 | `iter` |  |  | Y |  | Y |  |  | unknown | 1571&#8209;1576 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
