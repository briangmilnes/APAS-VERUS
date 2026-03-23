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
| 1 | Chap63 | ConnectivityMtEph | 4 | 0 | 0 | 5 | 5 | 0 | 5 | 0 | 0 |
| 2 | Chap63 | ConnectivityStEph | 4 | 0 | 0 | 5 | 5 | 0 | 5 | 0 | 0 |

## Function-by-Function Detail

### Chap63/ConnectivityMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `count_components_mt` | Y |  |  | Y | Y |  |  | unknown | 42&#8209;43 |
| 2 | `connected_components_mt` | Y |  |  | Y | Y |  |  | unknown | 47&#8209;51 |
| 3 | `count_components_hof` | Y |  |  | Y | Y |  |  | unknown | 55&#8209;56 |
| 4 | `connected_components_hof` | Y |  |  | Y | Y |  |  | unknown | 60&#8209;64 |
| 5 | `compose_maps_parallel` |  |  |  | Y | Y |  |  | unknown | 123&#8209;129 |

### Chap63/ConnectivityStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `count_components` | Y |  |  | Y | Y |  |  | unknown | 43&#8209;44 |
| 7 | `connected_components` | Y |  |  | Y | Y |  |  | unknown | 48&#8209;49 |
| 8 | `count_components_hof` | Y |  |  | Y | Y |  |  | unknown | 53&#8209;54 |
| 9 | `connected_components_hof` | Y |  |  | Y | Y |  |  | unknown | 58&#8209;59 |
| 10 | `build_quotient_edges` |  |  |  | Y | Y |  |  | unknown | 115&#8209;123 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
