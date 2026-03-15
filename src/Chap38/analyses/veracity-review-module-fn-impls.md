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
| 1 | Chap38 | BSTParaMtEph | 17 | 17 | 0 | 16 | 18 | 14 | 9 | 7 | 16 |
| 2 | Chap38 | BSTParaStEph | 20 | 20 | 0 | 8 | 28 | 0 | 25 | 3 | 0 |

## Function-by-Function Detail

### Chap38/BSTParaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_param_bst_arc` |  |  |  | Y | Y |  |  | unknown | 68&#8209;72 |
| 2 | `new` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 3 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;117 |
| 4 | `expose` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 5 | `join_mid` | Y | Y |  | Y | Y |  |  | unknown | 122&#8209;123 |
| 6 | `size` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;126 |
| 7 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 8 | `insert` | Y | Y |  |  | Y |  | Y |  | 132 |
| 9 | `delete` | Y | Y |  |  | Y |  | Y |  | 135 |
| 10 | `find` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;138 |
| 11 | `split` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;144 |
| 12 | `join_pair` | Y | Y |  |  | Y |  |  | hole | 146&#8209;147 |
| 13 | `union` | Y | Y |  |  | Y |  |  | hole | 149&#8209;150 |
| 14 | `intersect` | Y | Y |  |  | Y |  |  | hole | 152&#8209;153 |
| 15 | `difference` | Y | Y |  |  | Y |  |  | hole | 155&#8209;156 |
| 16 | `filter` | Y | Y |  |  | Y |  |  | hole | 158&#8209;159 |
| 17 | `reduce` | Y | Y |  |  | Y |  |  | hole | 162 |
| 18 | `in_order` | Y | Y |  |  | Y |  |  | hole | 164&#8209;165 |
| 19 | `new_leaf` |  |  |  | Y |  | Y | Y |  | 415&#8209;417 |
| 20 | `expose_internal` |  |  |  | Y |  | Y | Y |  | 419&#8209;427 |
| 21 | `split_inner` |  |  |  | Y |  | Y | Y |  | 441&#8209;458 |
| 22 | `join_m` |  |  |  | Y |  | Y | Y |  | 460&#8209;462 |
| 23 | `min_key` |  |  |  | Y |  | Y | Y |  | 464&#8209;472 |
| 24 | `join_pair_inner` |  |  |  | Y |  | Y | Y |  | 474&#8209;483 |
| 25 | `union_inner` |  |  |  | Y |  | Y | Y |  | 485&#8209;496 |
| 26 | `intersect_inner` |  |  |  | Y |  | Y | Y |  | 498&#8209;512 |
| 27 | `difference_inner` |  |  |  | Y |  | Y | Y |  | 514&#8209;530 |
| 28 | `filter_inner` |  |  |  | Y |  | Y | Y |  | 532&#8209;552 |
| 29 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 554&#8209;560 |
| 30 | `reduce_inner` |  |  |  | Y |  | Y | Y |  | 562&#8209;583 |
| 31 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 585&#8209;592 |
| 32 | `collect_in_order` |  |  |  | Y |  | Y | Y |  | 594&#8209;603 |

### Chap38/BSTParaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 33 | `new_param_bst` |  |  |  | Y | Y |  |  | unknown | 87&#8209;96 |
| 34 | `lemma_cmp_antisymmetry` |  |  |  | Y | Y |  |  | unknown | 146&#8209;151 |
| 35 | `lemma_cmp_transitivity` |  |  |  | Y | Y |  |  | unknown | 158&#8209;164 |
| 36 | `lemma_cmp_eq_subst` |  |  |  | Y | Y |  |  | unknown | 171&#8209;178 |
| 37 | `lemma_cmp_equal_congruent` |  |  |  | Y | Y |  |  | unknown | 189&#8209;195 |
| 38 | `lemma_cmp_equal_congruent_right` |  |  |  | Y | Y |  |  | unknown | 209&#8209;215 |
| 39 | `new` | Y | Y |  |  | Y |  |  | unknown | 228&#8209;229 |
| 40 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;235 |
| 41 | `expose` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;241 |
| 42 | `join_mid` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;255 |
| 43 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;277 |
| 44 | `size` | Y | Y |  |  | Y |  |  | unknown | 279&#8209;280 |
| 45 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;283 |
| 46 | `insert` | Y | Y |  |  | Y |  |  | hole | 285&#8209;292 |
| 47 | `delete` | Y | Y |  |  | Y |  |  | hole | 294&#8209;301 |
| 48 | `find` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;307 |
| 49 | `split` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;322 |
| 50 | `min_key` | Y | Y |  |  | Y |  |  | unknown | 324&#8209;332 |
| 51 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 334&#8209;342 |
| 52 | `union` | Y | Y |  |  | Y |  |  | hole | 344&#8209;348 |
| 53 | `intersect` | Y | Y |  |  | Y |  |  | unknown | 350&#8209;354 |
| 54 | `difference` | Y | Y |  |  | Y |  |  | unknown | 356&#8209;360 |
| 55 | `filter` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;368 |
| 56 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 371&#8209;372 |
| 57 | `collect_in_order` | Y | Y |  |  | Y |  |  | unknown | 374&#8209;376 |
| 58 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 378&#8209;379 |
| 59 | `filter_inner` |  |  |  | Y | Y |  |  | unknown | 1293&#8209;1303 |
| 60 | `reduce_inner` |  |  |  | Y | Y |  |  | unknown | 1345&#8209;1354 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
