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
| 1 | Chap62 | StarContractionMtEph | 2 | 0 | 0 | 5 | 5 | 0 | 5 | 0 | 0 |
| 2 | Chap62 | StarContractionStEph | 2 | 0 | 0 | 4 | 4 | 0 | 4 | 0 | 0 |
| 3 | Chap62 | StarPartitionMtEph | 1 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 4 | Chap62 | StarPartitionStEph | 1 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |

## Function-by-Function Detail

### Chap62/StarContractionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `star_contract_mt` | Y |  |  | Y | Y |  |  | unknown | 45&#8209;55 |
| 2 | `contract_to_vertices_mt` | Y |  |  | Y | Y |  |  | unknown | 59&#8209;62 |
| 3 | `star_contract_mt_fuel` |  |  |  | Y | Y |  |  | unknown | 86&#8209;102 |
| 4 | `build_quotient_graph_parallel` |  |  |  | Y | Y |  |  | unknown | 181&#8209;192 |
| 5 | `route_edges_parallel` |  |  |  | Y | Y |  |  | unknown | 249&#8209;270 |

### Chap62/StarContractionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `star_contract` | Y |  |  | Y | Y |  |  | unknown | 44&#8209;54 |
| 7 | `contract_to_vertices` | Y |  |  | Y | Y |  |  | unknown | 58&#8209;61 |
| 8 | `star_contract_fuel` |  |  |  | Y | Y |  |  | unknown | 67&#8209;83 |
| 9 | `build_quotient_graph` |  |  |  | Y | Y |  |  | unknown | 162&#8209;173 |

### Chap62/StarPartitionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 10 | `parallel_star_partition` | Y |  |  | Y | Y |  |  | unknown | 43&#8209;47 |

### Chap62/StarPartitionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `sequential_star_partition` | Y |  |  | Y | Y |  |  | unknown | 41&#8209;42 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
