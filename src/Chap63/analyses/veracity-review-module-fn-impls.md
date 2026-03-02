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
| 1 | Chap63 | ConnectivityMtEph | 4 | 0 | 0 | 7 | 4 | 3 | 0 | 0 | 7 |
| 2 | Chap63 | ConnectivityStEph | 4 | 0 | 0 | 5 | 4 | 1 | 0 | 0 | 5 |

## Function-by-Function Detail

### Chap63/ConnectivityMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `count_components_mt` | Y |  |  | Y | Y |  | Y |  | 36 |
| 2 | `connected_components_mt` | Y |  |  | Y | Y |  | Y |  | 40&#8209;43 |
| 3 | `count_components_hof` | Y |  |  | Y | Y |  | Y |  | 47 |
| 4 | `connected_components_hof` | Y |  |  | Y | Y |  | Y |  | 51&#8209;54 |
| 5 | `build_quotient_edges_parallel` |  |  |  | Y |  | Y | Y |  | 127&#8209;143 |
| 6 | `route_edges_parallel` |  |  |  | Y |  | Y | Y |  | 145&#8209;195 |
| 7 | `compose_maps_parallel` |  |  |  | Y |  | Y | Y |  | 197&#8209;212 |

### Chap63/ConnectivityStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 8 | `count_components` | Y |  |  | Y | Y |  | Y |  | 31 |
| 9 | `connected_components` | Y |  |  | Y | Y |  | Y |  | 35 |
| 10 | `count_components_hof` | Y |  |  | Y | Y |  | Y |  | 39 |
| 11 | `connected_components_hof` | Y |  |  | Y | Y |  | Y |  | 43 |
| 12 | `build_quotient_edges` |  |  |  | Y |  | Y | Y |  | 118&#8209;146 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
