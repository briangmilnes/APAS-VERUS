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
| 1 | Chap36 | QuickSortMtEph | 3 | 3 | 0 | 7 | 10 | 0 | 10 | 0 | 0 |
| 2 | Chap36 | QuickSortMtEphSlice | 6 | 6 | 0 | 0 | 6 | 0 | 3 | 0 | 3 |
| 3 | Chap36 | QuickSortStEph | 3 | 3 | 0 | 7 | 10 | 0 | 10 | 0 | 0 |

## Function-by-Function Detail

### Chap36/QuickSortMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 64&#8209;65 |
| 2 | `quick_sort_first` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 3 | `quick_sort_median3` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 4 | `quick_sort_random` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 5 | `sort_vec` |  |  |  | Y | Y |  |  | unknown | 116&#8209;119 |
| 6 | `median_of_three` |  |  |  | Y | Y |  |  | unknown | 154&#8209;157 |
| 7 | `median3_pivot_idx` |  |  |  | Y | Y |  |  | unknown | 207&#8209;212 |
| 8 | `sort_vec_random` |  |  |  | Y | Y |  |  | unknown | 227&#8209;229 |
| 9 | `sort_vec_median3` |  |  |  | Y | Y |  |  | unknown | 239&#8209;241 |
| 10 | `sort_vec_with_idx` |  |  |  | Y | Y |  |  | unknown | 253&#8209;259 |

### Chap36/QuickSortMtEphSlice.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `pivot_mt_first` | Y | Y |  |  | Y |  |  | unknown | 20&#8209;21 |
| 12 | `pivot_mt_median3` | Y | Y |  |  | Y |  |  | unknown | 25&#8209;26 |
| 13 | `pivot_mt_random` | Y | Y |  |  | Y |  |  | unknown | 30&#8209;31 |
| 14 | `quick_sort_mt_first` | Y | Y |  |  | Y |  | Y |  | 35 |
| 15 | `quick_sort_mt_median3` | Y | Y |  |  | Y |  | Y |  | 39 |
| 16 | `quick_sort_mt_random` | Y | Y |  |  | Y |  | Y |  | 43 |

### Chap36/QuickSortStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 17 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 62&#8209;63 |
| 18 | `quick_sort_first` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 19 | `quick_sort_median3` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |
| 20 | `quick_sort_random` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;108 |
| 21 | `sort_vec` |  |  |  | Y | Y |  |  | unknown | 114&#8209;117 |
| 22 | `median_of_three` |  |  |  | Y | Y |  |  | unknown | 152&#8209;155 |
| 23 | `median3_pivot_idx` |  |  |  | Y | Y |  |  | unknown | 207&#8209;212 |
| 24 | `sort_vec_random` |  |  |  | Y | Y |  |  | unknown | 227&#8209;229 |
| 25 | `sort_vec_median3` |  |  |  | Y | Y |  |  | unknown | 239&#8209;241 |
| 26 | `sort_vec_with_idx` |  |  |  | Y | Y |  |  | unknown | 253&#8209;259 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
