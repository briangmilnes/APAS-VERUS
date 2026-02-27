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
| 1 | Chap27 | ReduceContractMtEph | 1 | 1 | 0 | 2 | 3 | 0 | 3 | 0 | 0 |
| 2 | Chap27 | ReduceContractStEph | 1 | 1 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 3 | Chap27 | ScanContractMtEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 4 | Chap27 | ScanContractStEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |

## Function-by-Function Detail

### Chap27/ReduceContractMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `reduce_contract_parallel` | Y | Y |  |  | Y |  |  | unknown | 53&#8209;66 |
| 2 | `contract_parallel` |  |  |  | Y | Y |  |  | unknown | 78&#8209;93 |
| 3 | `reduce_contract_verified` |  |  |  | Y | Y |  |  | unknown | 248&#8209;262 |

### Chap27/ReduceContractStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 4 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 44&#8209;47 |
| 5 | `lemma_fold_left_pair` |  |  |  | Y | Y |  |  | unknown | 60&#8209;62 |
| 6 | `lemma_fold_left_singleton` |  |  |  | Y | Y |  |  | unknown | 76&#8209;78 |
| 7 | `lemma_contraction_even` |  |  |  | Y | Y |  |  | unknown | 88&#8209;98 |
| 8 | `reduce_contract` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;180 |

### Chap27/ScanContractMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 9 | `scan_contract_parallel` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;70 |
| 10 | `scan_contract_verified` |  |  |  | Y | Y |  |  | unknown | 79&#8209;96 |

### Chap27/ScanContractStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `lemma_prefix_contraction` |  |  |  | Y | Y |  |  | unknown | 46&#8209;54 |
| 12 | `scan_contract` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;87 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
