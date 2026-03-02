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
| 1 | Chap61 | EdgeContractionMtEph | 2 | 0 | 0 | 3 | 2 | 1 | 0 | 0 | 3 |
| 2 | Chap61 | EdgeContractionStEph | 2 | 0 | 0 | 2 | 2 | 0 | 0 | 0 | 2 |
| 3 | Chap61 | VertexMatchingMtEph | 1 | 0 | 0 | 5 | 1 | 4 | 0 | 0 | 5 |
| 4 | Chap61 | VertexMatchingStEph | 2 | 0 | 0 | 2 | 2 | 0 | 0 | 0 | 2 |

## Function-by-Function Detail

### Chap61/EdgeContractionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `edge_contract_mt` | Y |  |  | Y | Y |  | Y |  | 32&#8209;35 |
| 2 | `contract_round_mt` | Y |  |  | Y | Y |  | Y |  | 39&#8209;42 |
| 3 | `build_edges_parallel` |  |  |  | Y |  | Y | Y |  | 109&#8209;162 |

### Chap61/EdgeContractionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 4 | `edge_contract` | Y |  |  | Y | Y |  | Y |  | 30&#8209;33 |
| 5 | `contract_round` | Y |  |  | Y | Y |  | Y |  | 37 |

### Chap61/VertexMatchingMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `parallel_matching_mt` | Y |  |  | Y | Y |  | Y |  | 30 |
| 7 | `flip_coins_parallel` |  |  |  | Y |  | Y | Y |  | 78&#8209;100 |
| 8 | `select_edges_parallel` |  |  |  | Y |  | Y | Y |  | 102&#8209;133 |
| 9 | `select_edges_recursive` |  |  |  | Y |  | Y | Y |  | 135&#8209;178 |
| 10 | `should_select_edge` |  |  |  | Y |  | Y | Y |  | 180&#8209;211 |

### Chap61/VertexMatchingStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `greedy_matching` | Y |  |  | Y | Y |  | Y |  | 26 |
| 12 | `parallel_matching_st` | Y |  |  | Y | Y |  | Y |  | 30 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
