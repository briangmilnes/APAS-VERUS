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
| 1 | Chap36 | QuickSortMtEph | 0 | 0 | 0 | 9 | 9 | 0 | 9 | 0 | 0 |
| 2 | Chap36 | QuickSortMtEphSlice | 6 | 6 | 0 | 0 | 6 | 0 | 0 | 0 | 6 |
| 3 | Chap36 | QuickSortStEph | 0 | 0 | 0 | 9 | 9 | 0 | 9 | 0 | 0 |

## Function-by-Function Detail

### Chap36/QuickSortMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `sort_vec` |  |  |  | Y | Y |  |  | unknown | 46&#8209;49 |
| 2 | `median_of_three` |  |  |  | Y | Y |  |  | unknown | 371&#8209;372 |
| 3 | `median3_pivot_idx` |  |  |  | Y | Y |  |  | unknown | 400&#8209;402 |
| 4 | `sort_vec_random` |  |  |  | Y | Y |  |  | unknown | 417&#8209;420 |
| 5 | `sort_vec_median3` |  |  |  | Y | Y |  |  | unknown | 430&#8209;433 |
| 6 | `sort_vec_with_idx` |  |  |  | Y | Y |  |  | unknown | 445&#8209;451 |
| 7 | `quick_sort_first` |  |  |  | Y | Y |  |  | unknown | 737&#8209;739 |
| 8 | `quick_sort_median3` |  |  |  | Y | Y |  |  | unknown | 746&#8209;748 |
| 9 | `quick_sort_random` |  |  |  | Y | Y |  |  | unknown | 755&#8209;757 |

### Chap36/QuickSortMtEphSlice.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 10 | `pivot_mt_first` | Y | Y |  |  | Y |  | Y |  | 20 |
| 11 | `pivot_mt_median3` | Y | Y |  |  | Y |  | Y |  | 23 |
| 12 | `pivot_mt_random` | Y | Y |  |  | Y |  | Y |  | 26 |
| 13 | `quick_sort_mt_first` | Y | Y |  |  | Y |  | Y |  | 29 |
| 14 | `quick_sort_mt_median3` | Y | Y |  |  | Y |  | Y |  | 32 |
| 15 | `quick_sort_mt_random` | Y | Y |  |  | Y |  | Y |  | 35 |

### Chap36/QuickSortStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `sort_vec` |  |  |  | Y | Y |  |  | unknown | 44&#8209;47 |
| 17 | `quick_sort_first` |  |  |  | Y | Y |  |  | unknown | 367&#8209;369 |
| 18 | `median_of_three` |  |  |  | Y | Y |  |  | unknown | 388&#8209;389 |
| 19 | `median3_pivot_idx` |  |  |  | Y | Y |  |  | unknown | 419&#8209;421 |
| 20 | `sort_vec_random` |  |  |  | Y | Y |  |  | unknown | 436&#8209;439 |
| 21 | `sort_vec_median3` |  |  |  | Y | Y |  |  | unknown | 449&#8209;452 |
| 22 | `sort_vec_with_idx` |  |  |  | Y | Y |  |  | unknown | 464&#8209;470 |
| 23 | `quick_sort_median3` |  |  |  | Y | Y |  |  | unknown | 735&#8209;737 |
| 24 | `quick_sort_random` |  |  |  | Y | Y |  |  | unknown | 744&#8209;746 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
