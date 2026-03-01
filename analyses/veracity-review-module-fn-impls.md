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
| 2 | Chap38 | BSTParaStEph | 20 | 20 | 0 | 3 | 23 | 0 | 12 | 10 | 1 |

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
| 33 | `new_param_bst` |  |  |  | Y | Y |  |  | hole | 78&#8209;83 |
| 34 | `new` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 35 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;122 |
| 36 | `expose` | Y | Y |  |  | Y |  |  | hole | 124&#8209;128 |
| 37 | `join_mid` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;140 |
| 38 | `join_m` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;158 |
| 39 | `size` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;161 |
| 40 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 41 | `insert` | Y | Y |  |  | Y |  |  | hole | 167 |
| 42 | `delete` | Y | Y |  |  | Y |  |  | hole | 170 |
| 43 | `find` | Y | Y |  |  | Y |  |  | hole | 172&#8209;173 |
| 44 | `split` | Y | Y |  |  | Y |  |  | hole | 175&#8209;183 |
| 45 | `min_key` | Y | Y |  |  | Y |  | Y |  | 185 |
| 46 | `join_pair` | Y | Y |  |  | Y |  |  | hole | 187&#8209;188 |
| 47 | `union` | Y | Y |  |  | Y |  |  | hole | 190&#8209;191 |
| 48 | `intersect` | Y | Y |  |  | Y |  |  | hole | 193&#8209;194 |
| 49 | `difference` | Y | Y |  |  | Y |  |  | hole | 196&#8209;197 |
| 50 | `filter` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;201 |
| 51 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;205 |
| 52 | `collect_in_order` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;209 |
| 53 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;212 |
| 54 | `filter_inner` |  |  |  | Y | Y |  |  | unknown | 508&#8209;514 |
| 55 | `reduce_inner` |  |  |  | Y | Y |  |  | unknown | 540&#8209;548 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
