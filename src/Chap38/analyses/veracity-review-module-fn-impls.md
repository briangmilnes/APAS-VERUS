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
| 1 | Chap38 | BSTParaMtEph | 17 | 17 | 0 | 16 | 18 | 14 | 1 | 17 | 14 |
| 2 | Chap38 | BSTParaStEph | 20 | 20 | 0 | 8 | 28 | 0 | 18 | 10 | 0 |

## Function-by-Function Detail

### Chap38/BSTParaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_param_bst_arc` |  |  |  | Y | Y |  |  | unknown | 67&#8209;71 |
| 2 | `new` | Y | Y |  |  | Y |  |  | hole | 98&#8209;99 |
| 3 | `singleton` | Y | Y |  |  | Y |  |  | hole | 101&#8209;104 |
| 4 | `expose` | Y | Y |  |  | Y |  |  | hole | 106&#8209;107 |
| 5 | `join_mid` | Y | Y |  | Y | Y |  |  | hole | 109&#8209;110 |
| 6 | `size` | Y | Y |  |  | Y |  |  | hole | 112&#8209;113 |
| 7 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 115&#8209;116 |
| 8 | `insert` | Y | Y |  |  | Y |  |  | hole | 119 |
| 9 | `delete` | Y | Y |  |  | Y |  |  | hole | 122 |
| 10 | `find` | Y | Y |  |  | Y |  |  | hole | 124&#8209;125 |
| 11 | `split` | Y | Y |  |  | Y |  |  | hole | 127&#8209;131 |
| 12 | `join_pair` | Y | Y |  |  | Y |  |  | hole | 133&#8209;134 |
| 13 | `union` | Y | Y |  |  | Y |  |  | hole | 136&#8209;137 |
| 14 | `intersect` | Y | Y |  |  | Y |  |  | hole | 139&#8209;140 |
| 15 | `difference` | Y | Y |  |  | Y |  |  | hole | 142&#8209;143 |
| 16 | `filter` | Y | Y |  |  | Y |  |  | hole | 145&#8209;146 |
| 17 | `reduce` | Y | Y |  |  | Y |  |  | hole | 149 |
| 18 | `in_order` | Y | Y |  |  | Y |  |  | hole | 151&#8209;152 |
| 19 | `new_leaf` |  |  |  | Y |  | Y | Y |  | 324&#8209;326 |
| 20 | `expose_internal` |  |  |  | Y |  | Y | Y |  | 328&#8209;336 |
| 21 | `split_inner` |  |  |  | Y |  | Y | Y |  | 350&#8209;367 |
| 22 | `join_m` |  |  |  | Y |  | Y | Y |  | 369&#8209;371 |
| 23 | `min_key` |  |  |  | Y |  | Y | Y |  | 373&#8209;381 |
| 24 | `join_pair_inner` |  |  |  | Y |  | Y | Y |  | 383&#8209;392 |
| 25 | `union_inner` |  |  |  | Y |  | Y | Y |  | 394&#8209;405 |
| 26 | `intersect_inner` |  |  |  | Y |  | Y | Y |  | 407&#8209;421 |
| 27 | `difference_inner` |  |  |  | Y |  | Y | Y |  | 423&#8209;439 |
| 28 | `filter_inner` |  |  |  | Y |  | Y | Y |  | 441&#8209;461 |
| 29 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 463&#8209;469 |
| 30 | `reduce_inner` |  |  |  | Y |  | Y | Y |  | 471&#8209;492 |
| 31 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 494&#8209;501 |
| 32 | `collect_in_order` |  |  |  | Y |  | Y | Y |  | 503&#8209;512 |

### Chap38/BSTParaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 33 | `new_param_bst` |  |  |  | Y | Y |  |  | unknown | 80&#8209;85 |
| 34 | `lemma_cmp_antisymmetry` |  |  |  | Y | Y |  |  | unknown | 118&#8209;123 |
| 35 | `lemma_cmp_transitivity` |  |  |  | Y | Y |  |  | unknown | 130&#8209;136 |
| 36 | `lemma_cmp_eq_subst` |  |  |  | Y | Y |  |  | hole | 145&#8209;151 |
| 37 | `lemma_cmp_equal_congruent` |  |  |  | Y | Y |  |  | hole | 162&#8209;167 |
| 38 | `lemma_cmp_equal_congruent_right` |  |  |  | Y | Y |  |  | hole | 176&#8209;181 |
| 39 | `new` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;193 |
| 40 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;198 |
| 41 | `expose` | Y | Y |  |  | Y |  |  | hole | 200&#8209;204 |
| 42 | `join_mid` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;218 |
| 43 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;236 |
| 44 | `size` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;239 |
| 45 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;242 |
| 46 | `insert` | Y | Y |  |  | Y |  |  | hole | 245 |
| 47 | `delete` | Y | Y |  |  | Y |  |  | hole | 248 |
| 48 | `find` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;254 |
| 49 | `split` | Y | Y |  |  | Y |  |  | hole | 256&#8209;269 |
| 50 | `min_key` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;279 |
| 51 | `join_pair` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;286 |
| 52 | `union` | Y | Y |  |  | Y |  |  | hole | 288&#8209;289 |
| 53 | `intersect` | Y | Y |  |  | Y |  |  | hole | 291&#8209;292 |
| 54 | `difference` | Y | Y |  |  | Y |  |  | hole | 294&#8209;295 |
| 55 | `filter` | Y | Y |  |  | Y |  |  | unknown | 297&#8209;299 |
| 56 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 302&#8209;303 |
| 57 | `collect_in_order` | Y | Y |  |  | Y |  |  | unknown | 305&#8209;307 |
| 58 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;310 |
| 59 | `filter_inner` |  |  |  | Y | Y |  |  | unknown | 777&#8209;783 |
| 60 | `reduce_inner` |  |  |  | Y | Y |  |  | unknown | 816&#8209;824 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
