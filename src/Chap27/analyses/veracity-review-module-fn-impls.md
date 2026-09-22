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
| 1 | Chap27 | ContractSpecsAndLemmas | 0 | 0 | 0 | 8 | 8 | 0 | 8 | 0 | 0 |
| 2 | Chap27 | ReduceContractMtEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 3 | Chap27 | ReduceContractStEph | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 0 | 0 |
| 4 | Chap27 | ScanContractMtEph | 2 | 2 | 0 | 0 | 2 | 0 | 2 | 0 | 0 |
| 5 | Chap27 | ScanContractStEph | 2 | 2 | 0 | 1 | 3 | 0 | 3 | 0 | 0 |

## Function-by-Function Detail

### Chap27/ContractSpecsAndLemmas.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 34&#8209;37 |
| 2 | `lemma_fold_left_pair` |  |  |  | Y | Y |  |  | unknown | 48&#8209;50 |
| 3 | `lemma_fold_left_singleton` |  |  |  | Y | Y |  |  | unknown | 57&#8209;59 |
| 4 | `lemma_contraction_even` |  |  |  | Y | Y |  |  | unknown | 66&#8209;76 |
| 5 | `lemma_prefix_contraction` |  |  |  | Y | Y |  |  | unknown | 128&#8209;136 |
| 6 | `lemma_expand_even` |  |  |  | Y | Y |  |  | unknown | 150&#8209;158 |
| 7 | `lemma_expand_odd` |  |  |  | Y | Y |  |  | unknown | 168&#8209;174 |
| 8 | `lemma_expand_odd_tail` |  |  |  | Y | Y |  |  | unknown | 185&#8209;196 |

### Chap27/ReduceContractMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 9 | `reduce_contract_parallel` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;71 |
| 10 | `contract_parallel` |  |  |  | Y | Y |  |  | unknown | 80&#8209;95 |

### Chap27/ReduceContractStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `reduce_contract` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;64 |

### Chap27/ScanContractMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 12 | `scan_contract_parallel` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;74 |
| 13 | `expand_scan_parallel` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;99 |

### Chap27/ScanContractStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `lemma_scan_prefix_unfold` |  |  |  | Y | Y |  |  | unknown | 54&#8209;55 |
| 15 | `scan_contract` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;83 |
| 16 | `expand_scan` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;108 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
