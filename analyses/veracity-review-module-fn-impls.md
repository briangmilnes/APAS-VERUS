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
| 1 | Chap27 | ReduceContractMtEph | 1 | 1 | 0 | 5 | 6 | 0 | 6 | 0 | 0 |
| 2 | Chap27 | ReduceContractStEph | 1 | 1 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 3 | Chap27 | ScanContractMtEph | 2 | 2 | 0 | 8 | 10 | 0 | 10 | 0 | 0 |
| 4 | Chap27 | ScanContractStEph | 2 | 2 | 0 | 8 | 10 | 0 | 10 | 0 | 0 |

## Function-by-Function Detail

### Chap27/ReduceContractMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 49&#8209;52 |
| 2 | `lemma_fold_left_pair` |  |  |  | Y | Y |  |  | unknown | 62&#8209;64 |
| 3 | `lemma_fold_left_singleton` |  |  |  | Y | Y |  |  | unknown | 73&#8209;75 |
| 4 | `lemma_contraction_even` |  |  |  | Y | Y |  |  | unknown | 83&#8209;93 |
| 5 | `reduce_contract_parallel` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;165 |
| 6 | `contract_parallel` |  |  |  | Y | Y |  |  | unknown | 175&#8209;190 |

### Chap27/ReduceContractStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 44&#8209;47 |
| 8 | `lemma_fold_left_pair` |  |  |  | Y | Y |  |  | unknown | 60&#8209;62 |
| 9 | `lemma_fold_left_singleton` |  |  |  | Y | Y |  |  | unknown | 76&#8209;78 |
| 10 | `lemma_contraction_even` |  |  |  | Y | Y |  |  | unknown | 88&#8209;98 |
| 11 | `reduce_contract` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;180 |

### Chap27/ScanContractMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 12 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 48&#8209;51 |
| 13 | `lemma_fold_left_pair` |  |  |  | Y | Y |  |  | unknown | 62&#8209;64 |
| 14 | `lemma_fold_left_singleton` |  |  |  | Y | Y |  |  | unknown | 73&#8209;75 |
| 15 | `lemma_contraction_even` |  |  |  | Y | Y |  |  | unknown | 83&#8209;93 |
| 16 | `lemma_prefix_contraction` |  |  |  | Y | Y |  |  | unknown | 149&#8209;157 |
| 17 | `lemma_expand_even` |  |  |  | Y | Y |  |  | unknown | 169&#8209;177 |
| 18 | `lemma_expand_odd` |  |  |  | Y | Y |  |  | unknown | 189&#8209;195 |
| 19 | `lemma_expand_odd_tail` |  |  |  | Y | Y |  |  | unknown | 205&#8209;216 |
| 20 | `scan_contract_parallel` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;250 |
| 21 | `expand_scan_parallel` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;273 |

### Chap27/ScanContractStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 22 | `lemma_fold_left_monoid` |  |  |  | Y | Y |  |  | unknown | 42&#8209;45 |
| 23 | `lemma_fold_left_pair` |  |  |  | Y | Y |  |  | unknown | 56&#8209;58 |
| 24 | `lemma_fold_left_singleton` |  |  |  | Y | Y |  |  | unknown | 67&#8209;69 |
| 25 | `lemma_contraction_even` |  |  |  | Y | Y |  |  | unknown | 77&#8209;87 |
| 26 | `lemma_prefix_contraction` |  |  |  | Y | Y |  |  | unknown | 143&#8209;151 |
| 27 | `lemma_expand_even` |  |  |  | Y | Y |  |  | unknown | 163&#8209;171 |
| 28 | `lemma_expand_odd` |  |  |  | Y | Y |  |  | unknown | 183&#8209;189 |
| 29 | `lemma_expand_odd_tail` |  |  |  | Y | Y |  |  | unknown | 199&#8209;210 |
| 30 | `scan_contract` | Y | Y |  |  | Y |  |  | unknown | 228&#8209;243 |
| 31 | `expand_scan` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;266 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
