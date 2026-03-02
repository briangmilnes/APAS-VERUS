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
| 1 | Chap62 | StarContractionMtEph | 2 | 0 | 0 | 4 | 2 | 2 | 0 | 0 | 4 |
| 2 | Chap62 | StarContractionStEph | 2 | 0 | 0 | 3 | 2 | 1 | 0 | 0 | 3 |
| 3 | Chap62 | StarPartitionMtEph | 1 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 1 |
| 4 | Chap62 | StarPartitionStEph | 1 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 1 |

## Function-by-Function Detail

### Chap62/StarContractionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `star_contract_mt` | Y |  |  | Y | Y |  | Y |  | 31&#8209;35 |
| 2 | `contract_to_vertices_mt` | Y |  |  | Y | Y |  | Y |  | 39 |
| 3 | `build_quotient_graph_parallel` |  |  |  | Y |  | Y | Y |  | 83&#8209;104 |
| 4 | `route_edges_parallel` |  |  |  | Y |  | Y | Y |  | 106&#8209;156 |

### Chap62/StarContractionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `star_contract` | Y |  |  | Y | Y |  | Y |  | 26&#8209;30 |
| 6 | `contract_to_vertices` | Y |  |  | Y | Y |  | Y |  | 34 |
| 7 | `build_quotient_graph` |  |  |  | Y |  | Y | Y |  | 77&#8209;108 |

### Chap62/StarPartitionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 8 | `parallel_star_partition` | Y |  |  | Y | Y |  | Y |  | 26&#8209;29 |

### Chap62/StarPartitionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 9 | `sequential_star_partition` | Y |  |  | Y | Y |  | Y |  | 25 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
