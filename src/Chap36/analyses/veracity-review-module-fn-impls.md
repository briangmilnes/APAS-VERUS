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
| 1 | Chap36 | QuickSortMtEph | 6 | 6 | 0 | 2 | 8 | 0 | 8 | 0 | 0 |
| 2 | Chap36 | QuickSortMtEphSlice | 6 | 6 | 0 | 3 | 9 | 0 | 9 | 0 | 0 |
| 3 | Chap36 | QuickSortStEph | 6 | 6 | 0 | 2 | 8 | 0 | 8 | 0 | 0 |

## Function-by-Function Detail

### Chap36/QuickSortMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 64&#8209;65 |
| 2 | `lemma_partition_sort_concat` |  |  |  | Y | Y |  |  | unknown | 90&#8209;113 |
| 3 | `quick_sort_first` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;190 |
| 4 | `quick_sort_median3` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;200 |
| 5 | `quick_sort_random` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;210 |
| 6 | `median_of_three` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;216 |
| 7 | `median3_pivot_idx` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;224 |
| 8 | `concat_three` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;233 |

### Chap36/QuickSortMtEphSlice.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 9 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 71&#8209;72 |
| 10 | `lemma_partition_sort_concat` |  |  |  | Y | Y |  |  | unknown | 97&#8209;120 |
| 11 | `lemma_elements_from_vec` |  |  |  | Y | Y |  |  | unknown | 187&#8209;193 |
| 12 | `quick_sort_first` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;218 |
| 13 | `quick_sort_median3` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;232 |
| 14 | `quick_sort_random` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;246 |
| 15 | `median_of_three` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;252 |
| 16 | `median3_pivot_idx` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;264 |
| 17 | `concat_three_vecs` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;273 |

### Chap36/QuickSortStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 18 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 62&#8209;63 |
| 19 | `lemma_partition_sort_concat` |  |  |  | Y | Y |  |  | unknown | 88&#8209;111 |
| 20 | `quick_sort_first` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;188 |
| 21 | `quick_sort_median3` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;198 |
| 22 | `quick_sort_random` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;208 |
| 23 | `median_of_three` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;214 |
| 24 | `median3_pivot_idx` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;222 |
| 25 | `concat_three` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;231 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
