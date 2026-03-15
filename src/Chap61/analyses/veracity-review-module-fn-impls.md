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
| 1 | Chap61 | EdgeContractionMtEph | 2 | 0 | 0 | 3 | 2 | 1 | 2 | 0 | 1 |
| 2 | Chap61 | EdgeContractionStEph | 2 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 3 | Chap61 | VertexMatchingMtEph | 1 | 0 | 0 | 5 | 1 | 4 | 1 | 0 | 4 |
| 4 | Chap61 | VertexMatchingStEph | 2 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |

## Function-by-Function Detail

### Chap61/EdgeContractionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `edge_contract_mt` | Y |  |  | Y | Y |  |  | unknown | 45&#8209;49 |
| 2 | `contract_round_mt` | Y |  |  | Y | Y |  |  | unknown | 53&#8209;57 |
| 3 | `build_edges_parallel` |  |  |  | Y |  | Y | Y |  | 125&#8209;178 |

### Chap61/EdgeContractionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 4 | `edge_contract` | Y |  |  | Y | Y |  |  | unknown | 43&#8209;47 |
| 5 | `contract_round` | Y |  |  | Y | Y |  |  | unknown | 51&#8209;52 |

### Chap61/VertexMatchingMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `parallel_matching_mt` | Y |  |  | Y | Y |  |  | unknown | 43&#8209;44 |
| 7 | `flip_coins_parallel` |  |  |  | Y |  | Y | Y |  | 93&#8209;115 |
| 8 | `select_edges_parallel` |  |  |  | Y |  | Y | Y |  | 117&#8209;148 |
| 9 | `select_edges_recursive` |  |  |  | Y |  | Y | Y |  | 150&#8209;193 |
| 10 | `should_select_edge` |  |  |  | Y |  | Y | Y |  | 195&#8209;226 |

### Chap61/VertexMatchingStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `greedy_matching` | Y |  |  | Y | Y |  |  | unknown | 39&#8209;40 |
| 12 | `parallel_matching_st` | Y |  |  | Y | Y |  |  | unknown | 44&#8209;45 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
