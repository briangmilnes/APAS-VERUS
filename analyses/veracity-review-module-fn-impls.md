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
| 1 | Chap38 | BSTParaMtEph | 17 | 17 | 0 | 16 | 18 | 14 | 0 | 18 | 14 |
| 2 | Chap38 | BSTParaStEph | 17 | 17 | 0 | 14 | 30 | 0 | 0 | 30 | 0 |

## Function-by-Function Detail

### Chap38/BSTParaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_bst_para_lock` |  |  |  | Y | Y |  |  | hole | 67 |
| 2 | `new` | Y | Y |  |  | Y |  |  | hole | 99&#8209;100 |
| 3 | `singleton` | Y | Y |  |  | Y |  |  | hole | 102&#8209;105 |
| 4 | `expose` | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 5 | `join_mid` | Y | Y |  | Y | Y |  |  | hole | 110&#8209;111 |
| 6 | `size` | Y | Y |  |  | Y |  |  | hole | 113&#8209;114 |
| 7 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 116&#8209;117 |
| 8 | `insert` | Y | Y |  |  | Y |  |  | hole | 120 |
| 9 | `delete` | Y | Y |  |  | Y |  |  | hole | 123 |
| 10 | `find` | Y | Y |  |  | Y |  |  | hole | 125&#8209;126 |
| 11 | `split` | Y | Y |  |  | Y |  |  | hole | 128&#8209;132 |
| 12 | `join_pair` | Y | Y |  |  | Y |  |  | hole | 134&#8209;135 |
| 13 | `union` | Y | Y |  |  | Y |  |  | hole | 137&#8209;138 |
| 14 | `intersect` | Y | Y |  |  | Y |  |  | hole | 140&#8209;141 |
| 15 | `difference` | Y | Y |  |  | Y |  |  | hole | 143&#8209;144 |
| 16 | `filter` | Y | Y |  |  | Y |  |  | hole | 146&#8209;147 |
| 17 | `reduce` | Y | Y |  |  | Y |  |  | hole | 150 |
| 18 | `in_order` | Y | Y |  |  | Y |  |  | hole | 152&#8209;153 |
| 19 | `new_leaf` |  |  |  | Y |  | Y | Y |  | 325&#8209;327 |
| 20 | `expose_internal` |  |  |  | Y |  | Y | Y |  | 329&#8209;337 |
| 21 | `split_inner` |  |  |  | Y |  | Y | Y |  | 351&#8209;368 |
| 22 | `join_m` |  |  |  | Y |  | Y | Y |  | 370&#8209;372 |
| 23 | `min_key` |  |  |  | Y |  | Y | Y |  | 374&#8209;382 |
| 24 | `join_pair_inner` |  |  |  | Y |  | Y | Y |  | 384&#8209;393 |
| 25 | `union_inner` |  |  |  | Y |  | Y | Y |  | 395&#8209;406 |
| 26 | `intersect_inner` |  |  |  | Y |  | Y | Y |  | 408&#8209;422 |
| 27 | `difference_inner` |  |  |  | Y |  | Y | Y |  | 424&#8209;440 |
| 28 | `filter_inner` |  |  |  | Y |  | Y | Y |  | 442&#8209;462 |
| 29 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 464&#8209;470 |
| 30 | `reduce_inner` |  |  |  | Y |  | Y | Y |  | 472&#8209;493 |
| 31 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 495&#8209;502 |
| 32 | `collect_in_order` |  |  |  | Y |  | Y | Y |  | 504&#8209;513 |

### Chap38/BSTParaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 33 | `new_bst_para_lock` |  |  |  | Y | Y |  |  | hole | 73 |
| 34 | `new` | Y | Y |  |  | Y |  |  | hole | 105&#8209;106 |
| 35 | `singleton` | Y | Y |  |  | Y |  |  | hole | 108&#8209;111 |
| 36 | `expose` | Y | Y |  |  | Y |  |  | hole | 113&#8209;114 |
| 37 | `join_mid` | Y | Y |  | Y | Y |  |  | hole | 116&#8209;117 |
| 38 | `size` | Y | Y |  |  | Y |  |  | hole | 119&#8209;120 |
| 39 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 122&#8209;123 |
| 40 | `insert` | Y | Y |  |  | Y |  |  | hole | 126 |
| 41 | `delete` | Y | Y |  |  | Y |  |  | hole | 129 |
| 42 | `find` | Y | Y |  |  | Y |  |  | hole | 131&#8209;132 |
| 43 | `split` | Y | Y |  |  | Y |  |  | hole | 134&#8209;138 |
| 44 | `join_pair` | Y | Y |  |  | Y |  |  | hole | 140&#8209;141 |
| 45 | `union` | Y | Y |  |  | Y |  |  | hole | 143&#8209;144 |
| 46 | `intersect` | Y | Y |  |  | Y |  |  | hole | 146&#8209;147 |
| 47 | `difference` | Y | Y |  |  | Y |  |  | hole | 149&#8209;150 |
| 48 | `filter` | Y | Y |  |  | Y |  |  | hole | 152&#8209;153 |
| 49 | `reduce` | Y | Y |  |  | Y |  |  | hole | 156 |
| 50 | `in_order` | Y | Y |  |  | Y |  |  | hole | 158&#8209;159 |
| 51 | `new_leaf` |  |  |  | Y | Y |  |  | hole | 293 |
| 52 | `expose_internal` |  |  |  | Y | Y |  |  | hole | 298 |
| 53 | `split_inner` |  |  |  | Y | Y |  |  | hole | 322 |
| 54 | `join_m` |  |  |  | Y | Y |  |  | hole | 342 |
| 55 | `min_key` |  |  |  | Y | Y |  |  | hole | 347 |
| 56 | `join_pair_inner` |  |  |  | Y | Y |  |  | hole | 358 |
| 57 | `union_inner` |  |  |  | Y | Y |  |  | hole | 371 |
| 58 | `intersect_inner` |  |  |  | Y | Y |  |  | hole | 386 |
| 59 | `difference_inner` |  |  |  | Y | Y |  |  | hole | 404 |
| 60 | `filter_inner` |  |  |  | Y | Y |  |  | hole | 423&#8209;426 |
| 61 | `reduce_inner` |  |  |  | Y | Y |  |  | hole | 443&#8209;447 |
| 62 | `collect_in_order` |  |  |  | Y | Y |  |  | hole | 460 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
