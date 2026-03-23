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
| 1 | `new_param_bst` |  |  |  | Y | Y |  |  | unknown | 94&#8209;103 |
| 2 | `clone_elem` |  |  |  | Y | Y |  |  | unknown | 153&#8209;155 |
| 3 | `lemma_cmp_antisymmetry` |  |  |  | Y | Y |  |  | unknown | 163&#8209;168 |
| 4 | `lemma_cmp_transitivity` |  |  |  | Y | Y |  |  | unknown | 175&#8209;181 |
| 5 | `lemma_cmp_eq_subst` |  |  |  | Y | Y |  |  | unknown | 188&#8209;195 |
| 6 | `lemma_cmp_equal_congruent` |  |  |  | Y | Y |  |  | unknown | 202&#8209;208 |
| 7 | `lemma_cmp_equal_congruent_right` |  |  |  | Y | Y |  |  | unknown | 215&#8209;221 |
| 8 | `lemma_cmp_order_axioms` |  |  |  | Y | Y |  |  | unknown | 229&#8209;235 |
| 9 | `new` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;246 |
| 10 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;252 |
| 11 | `expose` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;256 |
| 12 | `join_mid` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;261 |
| 13 | `size` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;276 |
| 14 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 279&#8209;280 |
| 15 | `insert` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;288 |
| 16 | `delete` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;293 |
| 17 | `find` | Y | Y |  |  | Y |  |  | unknown | 296&#8209;298 |
| 18 | `split` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;306 |
| 19 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;314 |
| 20 | `join_pair_inner` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;325 |
| 21 | `union` | Y | Y |  |  | Y |  |  | unknown | 328&#8209;333 |
| 22 | `intersect` | Y | Y |  |  | Y |  |  | unknown | 336&#8209;338 |
| 23 | `difference` | Y | Y |  |  | Y |  |  | unknown | 341&#8209;343 |
| 24 | `filter` | Y | Y |  |  | Y |  |  | unknown | 346&#8209;363 |
| 25 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 367&#8209;368 |
| 26 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 371&#8209;372 |
| 27 | `new_leaf` |  |  |  | Y | Y |  |  | unknown | 685&#8209;686 |
| 28 | `expose_internal` |  |  |  | Y | Y |  |  | unknown | 691&#8209;711 |
| 29 | `split_inner` |  |  |  | Y | Y |  |  | unknown | 750&#8209;767 |
| 30 | `find_recursive` |  |  |  | Y | Y |  |  | unknown | 916&#8209;922 |
| 31 | `min_key` |  |  |  | Y | Y |  |  | unknown | 944&#8209;950 |
| 32 | `union_inner` |  |  |  | Y | Y |  |  | unknown | 965&#8209;971 |
| 33 | `intersect_inner` |  |  |  | Y | Y |  |  | unknown | 1079&#8209;1085 |
| 34 | `difference_inner` |  |  |  | Y | Y |  |  | unknown | 1251&#8209;1257 |
| 35 | `filter_inner` |  |  |  | Y | Y |  |  | unknown | 1422&#8209;1441 |
| 36 | `filter_parallel` |  |  |  | Y | Y |  |  | unknown | 1541&#8209;1558 |
| 37 | `reduce_inner` |  |  |  | Y | Y |  |  | unknown | 1565&#8209;1574 |
| 38 | `reduce_parallel` |  |  |  | Y | Y |  |  | unknown | 1603&#8209;1610 |
| 39 | `collect_in_order` |  |  |  | Y | Y |  |  | unknown | 1617&#8209;1621 |

### Chap38/BSTParaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 40 | `new_param_bst` |  |  |  | Y | Y |  |  | unknown | 95&#8209;104 |
| 41 | `clone_elem` |  |  |  | Y | Y |  |  | unknown | 154&#8209;156 |
| 42 | `lemma_cmp_antisymmetry` |  |  |  | Y | Y |  |  | unknown | 164&#8209;169 |
| 43 | `lemma_cmp_transitivity` |  |  |  | Y | Y |  |  | unknown | 176&#8209;182 |
| 44 | `lemma_cmp_eq_subst` |  |  |  | Y | Y |  |  | unknown | 189&#8209;196 |
| 45 | `lemma_cmp_equal_congruent` |  |  |  | Y | Y |  |  | unknown | 207&#8209;213 |
| 46 | `lemma_cmp_equal_congruent_right` |  |  |  | Y | Y |  |  | unknown | 227&#8209;233 |
| 47 | `new` | Y | Y |  |  | Y |  |  | unknown | 247&#8209;248 |
| 48 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;255 |
| 49 | `expose` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;262 |
| 50 | `join_mid` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;277 |
| 51 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;300 |
| 52 | `size` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;304 |
| 53 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;308 |
| 54 | `insert` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;319 |
| 55 | `delete` | Y | Y |  |  | Y |  |  | unknown | 322&#8209;330 |
| 56 | `find` | Y | Y |  |  | Y |  |  | unknown | 333&#8209;337 |
| 57 | `split` | Y | Y |  |  | Y |  |  | unknown | 340&#8209;353 |
| 58 | `min_key` | Y | Y |  |  | Y |  |  | unknown | 356&#8209;364 |
| 59 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 367&#8209;375 |
| 60 | `union` | Y | Y |  |  | Y |  |  | unknown | 378&#8209;383 |
| 61 | `intersect` | Y | Y |  |  | Y |  |  | unknown | 386&#8209;390 |
| 62 | `difference` | Y | Y |  |  | Y |  |  | unknown | 393&#8209;397 |
| 63 | `filter` | Y | Y |  |  | Y |  |  | unknown | 400&#8209;418 |
| 64 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 422&#8209;426 |
| 65 | `collect_in_order` | Y | Y |  |  | Y |  |  | unknown | 429&#8209;432 |
| 66 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 435&#8209;438 |
| 67 | `filter_inner` |  |  |  | Y | Y |  |  | unknown | 1484&#8209;1503 |
| 68 | `reduce_inner` |  |  |  | Y | Y |  |  | unknown | 1558&#8209;1567 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
