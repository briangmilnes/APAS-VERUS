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
| 2 | Chap36 | QuickSortMtEphSlice | 6 | 6 | 0 | 5 | 11 | 0 | 11 | 0 | 0 |
| 3 | Chap36 | QuickSortStEph | 6 | 6 | 0 | 2 | 8 | 0 | 8 | 0 | 0 |

## Function-by-Function Detail

### Chap36/QuickSortMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 72&#8209;73 |
| 2 | `lemma_partition_sort_concat` |  |  |  | Y | Y |  |  | unknown | 101&#8209;124 |
| 3 | `quick_sort_first` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;195 |
| 4 | `quick_sort_median3` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;205 |
| 5 | `quick_sort_random` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;215 |
| 6 | `median_of_three` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;222 |
| 7 | `median3_pivot_idx` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;231 |
| 8 | `concat_three` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;241 |

### Chap36/QuickSortMtEphSlice.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 9 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 83&#8209;84 |
| 10 | `lemma_partition_sort_concat` |  |  |  | Y | Y |  |  | unknown | 112&#8209;135 |
| 11 | `lemma_elements_from_vec` |  |  |  | Y | Y |  |  | unknown | 195&#8209;201 |
| 12 | `quick_sort_first` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;226 |
| 13 | `quick_sort_median3` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;240 |
| 14 | `quick_sort_random` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;254 |
| 15 | `median_of_three` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;261 |
| 16 | `median3_pivot_idx` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;274 |
| 17 | `concat_three_vecs` | Y | Y |  |  | Y |  |  | unknown | 278&#8209;286 |
| 18 | `append_vec` |  |  |  | Y | Y |  |  | unknown | 293&#8209;295 |
| 19 | `partition_three_dc` |  |  |  | Y | Y |  |  | unknown | 336&#8209;353 |

### Chap36/QuickSortStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 20 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 70&#8209;71 |
| 21 | `lemma_partition_sort_concat` |  |  |  | Y | Y |  |  | unknown | 99&#8209;122 |
| 22 | `quick_sort_first` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;199 |
| 23 | `quick_sort_median3` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;209 |
| 24 | `quick_sort_random` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;219 |
| 25 | `median_of_three` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;226 |
| 26 | `median3_pivot_idx` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;235 |
| 27 | `concat_three` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;245 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
