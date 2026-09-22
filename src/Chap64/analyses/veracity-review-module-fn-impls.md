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
| 1 | Chap64 | SpanTreeMtEph | 2 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 2 | Chap64 | SpanTreeStEph | 2 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 3 | Chap64 | TSPApproxStEph | 4 | 0 | 0 | 8 | 8 | 0 | 8 | 0 | 0 |

## Function-by-Function Detail

### Chap64/SpanTreeMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `spanning_tree_star_contraction_mt` | Y |  |  | Y | Y |  |  | unknown | 67&#8209;70 |
| 2 | `verify_spanning_tree` | Y |  |  | Y | Y |  |  | unknown | 75&#8209;76 |

### Chap64/SpanTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3 | `spanning_tree_star_contraction` | Y |  |  | Y | Y |  |  | unknown | 66&#8209;67 |
| 4 | `verify_spanning_tree` | Y |  |  | Y | Y |  |  | unknown | 72&#8209;73 |

### Chap64/TSPApproxStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `euler_tour` | Y |  |  | Y | Y |  |  | unknown | 65&#8209;73 |
| 6 | `shortcut_tour` | Y |  |  | Y | Y |  |  | unknown | 78 |
| 7 | `tour_weight` | Y |  |  | Y | Y |  |  | unknown | 83&#8209;89 |
| 8 | `approx_metric_tsp` | Y |  |  | Y | Y |  |  | unknown | 94&#8209;103 |
| 9 | `vec_contains_pair` |  |  |  | Y | Y |  |  | unknown | 112&#8209;113 |
| 10 | `euler_tour_dfs` |  |  |  | Y | Y |  |  | unknown | 167&#8209;181 |
| 11 | `get_neighbors` |  |  |  | Y | Y |  |  | unknown | 348&#8209;352 |
| 12 | `get_edge_weight` |  |  |  | Y | Y |  |  | unknown | 358&#8209;370 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
