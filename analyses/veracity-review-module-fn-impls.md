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
| 1 | Chap64 | SpanTreeMtEph | 2 | 0 | 0 | 2 | 0 | 2 | 0 | 0 | 2 |
| 2 | Chap64 | SpanTreeStEph | 2 | 0 | 0 | 2 | 0 | 2 | 0 | 0 | 2 |
| 3 | Chap64 | TSPApproxStEph | 4 | 0 | 0 | 7 | 0 | 7 | 0 | 0 | 7 |

## Function-by-Function Detail

### Chap64/SpanTreeMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `spanning_tree_star_contraction_mt` | Y |  |  | Y |  | Y | Y | none | 22&#8209;26 |
| 2 | `verify_spanning_tree` | Y |  |  | Y |  | Y | Y | none | 28&#8209;30 |

### Chap64/SpanTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3 | `spanning_tree_star_contraction` | Y |  |  | Y |  | Y | Y | none | 19&#8209;21 |
| 4 | `verify_spanning_tree` | Y |  |  | Y |  | Y | Y | none | 23&#8209;25 |

### Chap64/TSPApproxStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `euler_tour` | Y |  |  | Y |  | Y | Y | none | 24&#8209;26 |
| 6 | `shortcut_tour` | Y |  |  | Y |  | Y | Y | none | 28&#8209;30 |
| 7 | `tour_weight` | Y |  |  | Y |  | Y | Y | none | 32&#8209;37 |
| 8 | `approx_metric_tsp` | Y |  |  | Y |  | Y | Y | none | 39&#8209;44 |
| 9 | `euler_tour_dfs` |  |  |  | Y |  | Y | Y | none | 75&#8209;122 |
| 10 | `get_neighbors` |  |  |  | Y |  | Y | Y | none | 185&#8209;196 |
| 11 | `get_edge_weight` |  |  |  | Y |  | Y | Y | none | 198&#8209;210 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.