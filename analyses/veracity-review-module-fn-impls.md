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
| 1 | Chap26 | DivConReduceMtPer | 5 | 5 | 0 | 0 | 5 | 0 | 3 | 2 | 0 |
| 2 | Chap26 | DivConReduceStPer | 5 | 5 | 0 | 0 | 5 | 0 | 3 | 2 | 0 |
| 3 | Chap26 | ETSPStPer | 1 | 1 | 1 | 1 | 1 | 2 | 0 | 1 | 2 |
| 4 | Chap26 | MergeSortMtPer | 2 | 2 | 0 | 0 | 2 | 0 | 0 | 2 | 0 |
| 5 | Chap26 | MergeSortStPer | 2 | 2 | 0 | 1 | 3 | 0 | 3 | 0 | 0 |
| 6 | Chap26 | ScanDCMtPer | 1 | 1 | 0 | 1 | 1 | 1 | 0 | 1 | 1 |
| 7 | Chap26 | ScanDCStPer | 2 | 2 | 0 | 1 | 3 | 0 | 2 | 1 | 0 |

## Function-by-Function Detail

### Chap26/DivConReduceMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `max_element_parallel` | Y | Y |  |  | Y |  |  | strong | 47&#8209;51 |
| 2 | `sum_parallel` | Y | Y |  |  | Y |  |  | strong | 62&#8209;68 |
| 3 | `product_parallel` | Y | Y |  |  | Y |  |  | strong | 73&#8209;79 |
| 4 | `any_parallel` | Y | Y |  |  | Y |  |  | strong | 84&#8209;90 |
| 5 | `all_parallel` | Y | Y |  |  | Y |  |  | strong | 95&#8209;101 |

### Chap26/DivConReduceStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `max_element` | Y | Y |  |  | Y |  |  | strong | 49&#8209;53 |
| 7 | `sum` | Y | Y |  |  | Y |  |  | strong | 64&#8209;70 |
| 8 | `product` | Y | Y |  |  | Y |  |  | strong | 75&#8209;81 |
| 9 | `any` | Y | Y |  |  | Y |  |  | strong | 86&#8209;92 |
| 10 | `all` | Y | Y |  |  | Y |  |  | strong | 97&#8209;103 |

### Chap26/ETSPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `etsp` | Y | Y |  |  | Y |  |  | weak | 58&#8209;60 |
| 12 | `distance` |  |  | Y |  |  | Y | Y | none | 85&#8209;89 |
| 13 | `etsp_inner` |  |  |  | Y |  | Y | Y | none | 92&#8209;186 |

### Chap26/MergeSortMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `merge_parallel` | Y | Y |  |  | Y |  |  | strong | 45&#8209;54 |
| 15 | `merge_sort_parallel` | Y | Y |  |  | Y |  |  | strong | 58&#8209;63 |

### Chap26/MergeSortStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `merge` | Y | Y |  |  | Y |  |  | strong | 69&#8209;78 |
| 17 | `merge_sort` | Y | Y |  |  | Y |  |  | strong | 82&#8209;87 |
| 18 | `lemma_push_sorted` |  |  |  | Y | Y |  |  | strong | 93&#8209;98 |

### Chap26/ScanDCMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 19 | `prefix_sums_dc_parallel` | Y | Y |  |  | Y |  |  | strong | 46&#8209;53 |
| 20 | `prefix_sums_dc_inner` |  |  |  | Y |  | Y | Y | none | 67&#8209;106 |

### Chap26/ScanDCStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 21 | `scan_dc` | Y | Y |  |  | Y |  |  | strong | 61&#8209;72 |
| 22 | `prefix_sums_dc` | Y | Y |  |  | Y |  |  | strong | 76&#8209;83 |
| 23 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | strong | 90&#8209;93 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.