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
| 1 | Chap62 | StarContractionMtEph | 2 | 0 | 0 | 4 | 2 | 2 | 2 | 0 | 2 |
| 2 | Chap62 | StarContractionStEph | 2 | 0 | 0 | 3 | 2 | 1 | 2 | 0 | 1 |
| 3 | Chap62 | StarPartitionMtEph | 1 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 4 | Chap62 | StarPartitionStEph | 1 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |

## Function-by-Function Detail

### Chap62/StarContractionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `star_contract_mt` | Y |  |  | Y | Y |  |  | unknown | 44&#8209;49 |
| 2 | `contract_to_vertices_mt` | Y |  |  | Y | Y |  |  | unknown | 53&#8209;54 |
| 3 | `build_quotient_graph_parallel` |  |  |  | Y |  | Y | Y |  | 99&#8209;120 |
| 4 | `route_edges_parallel` |  |  |  | Y |  | Y | Y |  | 122&#8209;172 |

### Chap62/StarContractionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `star_contract` | Y |  |  | Y | Y |  |  | unknown | 39&#8209;44 |
| 6 | `contract_to_vertices` | Y |  |  | Y | Y |  |  | unknown | 48&#8209;49 |
| 7 | `build_quotient_graph` |  |  |  | Y |  | Y | Y |  | 93&#8209;124 |

### Chap62/StarPartitionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 8 | `parallel_star_partition` | Y |  |  | Y | Y |  |  | unknown | 39&#8209;43 |

### Chap62/StarPartitionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 9 | `sequential_star_partition` | Y |  |  | Y | Y |  |  | unknown | 38&#8209;39 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
