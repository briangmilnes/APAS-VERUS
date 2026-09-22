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
| 3 | Chap62 | StarPartitionMtEph | 1 | 0 | 0 | 10 | 10 | 0 | 9 | 1 | 0 |
| 4 | Chap62 | StarPartitionStEph | 1 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |

## Function-by-Function Detail

### Chap62/StarContractionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `star_contract_mt` | Y |  |  | Y | Y |  |  | unknown | 94&#8209;112 |
| 2 | `contract_to_vertices_mt` | Y |  |  | Y | Y |  |  | unknown | 117&#8209;120 |
| 3 | `star_contract_mt_fuel` |  |  |  | Y | Y |  |  | unknown | 128&#8209;150 |
| 4 | `build_quotient_graph_parallel` |  |  |  | Y | Y |  |  | unknown | 261&#8209;272 |
| 5 | `route_edges_parallel` |  |  |  | Y | Y |  |  | unknown | 329&#8209;350 |

### Chap62/StarContractionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `star_contract` | Y |  |  | Y | Y |  |  | unknown | 74&#8209;92 |
| 7 | `contract_to_vertices` | Y |  |  | Y | Y |  |  | unknown | 97&#8209;100 |
| 8 | `star_contract_fuel` |  |  |  | Y | Y |  |  | unknown | 108&#8209;130 |
| 9 | `build_quotient_graph` |  |  |  | Y | Y |  |  | unknown | 240&#8209;251 |

### Chap62/StarPartitionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 10 | `parallel_star_partition` | Y |  |  | Y | Y |  |  | unknown | 116&#8209;120 |
| 11 | `hash_coin` |  |  |  | Y | Y |  |  | hole | 132 |
| 12 | `hash_coin_flips_mt` |  |  |  | Y | Y |  |  | unknown | 145&#8209;159 |
| 13 | `build_th_edges_mt` |  |  |  | Y | Y |  |  | unknown | 281&#8209;309 |
| 14 | `build_p_vec_mt` |  |  |  | Y | Y |  |  | unknown | 470&#8209;482 |
| 15 | `build_vertex_to_index_mt` |  |  |  | Y | Y |  |  | unknown | 577&#8209;593 |
| 16 | `build_satellite_map_mt` |  |  |  | Y | Y |  |  | unknown | 758&#8209;790 |
| 17 | `build_p_vec_with_inject_mt` |  |  |  | Y | Y |  |  | unknown | 977&#8209;1027 |
| 18 | `build_partition_map_mt` |  |  |  | Y | Y |  |  | unknown | 1245&#8209;1265 |
| 19 | `build_centers_mt` |  |  |  | Y | Y |  |  | unknown | 1453&#8209;1469 |

### Chap62/StarPartitionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 20 | `sequential_star_partition` | Y |  |  | Y | Y |  |  | unknown | 90&#8209;91 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
