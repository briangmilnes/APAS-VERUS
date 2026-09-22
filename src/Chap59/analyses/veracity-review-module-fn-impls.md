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
| 1 | Chap59 | JohnsonMtEphF64 | 1 | 0 | 0 | 6 | 6 | 0 | 6 | 0 | 0 |
| 2 | Chap59 | JohnsonMtEphI64 | 1 | 0 | 0 | 6 | 6 | 0 | 6 | 0 | 0 |
| 3 | Chap59 | JohnsonStEphF64 | 1 | 0 | 0 | 7 | 7 | 0 | 7 | 0 | 0 |
| 4 | Chap59 | JohnsonStEphI64 | 1 | 0 | 0 | 7 | 7 | 0 | 7 | 0 | 0 |

## Function-by-Function Detail

### Chap59/JohnsonMtEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `johnson_apsp` | Y |  |  | Y | Y |  |  | unknown | 75&#8209;86 |
| 2 | `adjust_distance` |  |  |  | Y | Y |  |  | unknown | 97&#8209;99 |
| 3 | `parallel_dijkstra_all` |  |  |  | Y | Y |  |  | unknown | 162&#8209;188 |
| 4 | `add_dummy_source` |  |  |  | Y | Y |  |  | unknown | 279&#8209;291 |
| 5 | `reweight_graph` |  |  |  | Y | Y |  |  | unknown | 371&#8209;390 |
| 6 | `create_negative_cycle_result` |  |  |  | Y | Y |  |  | unknown | 470&#8209;476 |

### Chap59/JohnsonMtEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `johnson_apsp` | Y |  |  | Y | Y |  |  | unknown | 63&#8209;74 |
| 8 | `adjust_distance` |  |  |  | Y | Y |  |  | unknown | 85&#8209;87 |
| 9 | `parallel_dijkstra_all` |  |  |  | Y | Y |  |  | unknown | 151&#8209;177 |
| 10 | `add_dummy_source` |  |  |  | Y | Y |  |  | unknown | 269&#8209;281 |
| 11 | `reweight_graph` |  |  |  | Y | Y |  |  | unknown | 360&#8209;379 |
| 12 | `create_negative_cycle_result` |  |  |  | Y | Y |  |  | unknown | 467&#8209;473 |

### Chap59/JohnsonStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 13 | `johnson_apsp` | Y |  |  | Y | Y |  |  | unknown | 69&#8209;79 |
| 14 | `adjust_distance` |  |  |  | Y | Y |  |  | unknown | 89&#8209;91 |
| 15 | `reweight_edge` |  |  |  | Y | Y |  |  | unknown | 102&#8209;103 |
| 16 | `build_vertex_set` |  |  |  | Y | Y |  |  | unknown | 110&#8209;117 |
| 17 | `add_dummy_source` |  |  |  | Y | Y |  |  | unknown | 147&#8209;161 |
| 18 | `reweight_graph` |  |  |  | Y | Y |  |  | unknown | 245&#8209;264 |
| 19 | `create_negative_cycle_result` |  |  |  | Y | Y |  |  | unknown | 338&#8209;341 |

### Chap59/JohnsonStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 20 | `johnson_apsp` | Y |  |  | Y | Y |  |  | unknown | 66&#8209;76 |
| 21 | `adjust_distance` |  |  |  | Y | Y |  |  | unknown | 86&#8209;88 |
| 22 | `reweight_edge` |  |  |  | Y | Y |  |  | unknown | 102&#8209;103 |
| 23 | `build_vertex_set` |  |  |  | Y | Y |  |  | unknown | 115&#8209;122 |
| 24 | `add_dummy_source` |  |  |  | Y | Y |  |  | unknown | 151&#8209;165 |
| 25 | `reweight_graph` |  |  |  | Y | Y |  |  | unknown | 248&#8209;266 |
| 26 | `create_negative_cycle_result` |  |  |  | Y | Y |  |  | unknown | 351&#8209;356 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
