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
| 1 | Chap59 | JohnsonMtEphI64 | 1 | 0 | 0 | 5 | 5 | 0 | 5 | 0 | 0 |
| 2 | Chap59 | JohnsonStEphI64 | 1 | 0 | 0 | 7 | 7 | 0 | 7 | 0 | 0 |

## Function-by-Function Detail

### Chap59/JohnsonMtEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `johnson_apsp` | Y |  |  | Y | Y |  |  | unknown | 33 |
| 2 | `parallel_dijkstra_all` |  |  |  | Y | Y |  |  | unknown | 93&#8209;117 |
| 3 | `add_dummy_source` |  |  |  | Y | Y |  |  | unknown | 204&#8209;216 |
| 4 | `reweight_graph` |  |  |  | Y | Y |  |  | unknown | 296&#8209;314 |
| 5 | `create_negative_cycle_result` |  |  |  | Y | Y |  |  | unknown | 410&#8209;416 |

### Chap59/JohnsonStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `johnson_apsp` | Y |  |  | Y | Y |  |  | unknown | 54&#8209;64 |
| 7 | `adjust_distance` |  |  |  | Y | Y |  |  | unknown | 74&#8209;76 |
| 8 | `reweight_edge` |  |  |  | Y | Y |  |  | unknown | 91&#8209;92 |
| 9 | `build_vertex_set` |  |  |  | Y | Y |  |  | unknown | 105&#8209;112 |
| 10 | `add_dummy_source` |  |  |  | Y | Y |  |  | unknown | 147&#8209;161 |
| 11 | `reweight_graph` |  |  |  | Y | Y |  |  | unknown | 242&#8209;260 |
| 12 | `create_negative_cycle_result` |  |  |  | Y | Y |  |  | unknown | 342&#8209;347 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
