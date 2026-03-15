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
| 1 | Chap64 | SpanTreeMtEph | 2 | 0 | 0 | 3 | 3 | 0 | 3 | 0 | 0 |
| 2 | Chap64 | SpanTreeStEph | 2 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 3 | Chap64 | TSPApproxStEph | 4 | 0 | 0 | 7 | 4 | 3 | 3 | 0 | 4 |

## Function-by-Function Detail

### Chap64/SpanTreeMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_spanning_edges_arc` |  |  |  | Y | Y |  |  | unknown | 41&#8209;45 |
| 2 | `spanning_tree_star_contraction_mt` | Y |  |  | Y | Y |  |  | unknown | 60&#8209;63 |
| 3 | `verify_spanning_tree` | Y |  |  | Y | Y |  |  | unknown | 67&#8209;68 |

### Chap64/SpanTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 4 | `spanning_tree_star_contraction` | Y |  |  | Y | Y |  |  | unknown | 40&#8209;41 |
| 5 | `verify_spanning_tree` | Y |  |  | Y | Y |  |  | unknown | 45&#8209;46 |

### Chap64/TSPApproxStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `euler_tour` | Y |  |  | Y | Y |  |  | unknown | 49&#8209;54 |
| 7 | `shortcut_tour` | Y |  |  | Y | Y |  | Y |  | 58 |
| 8 | `tour_weight` | Y |  |  | Y | Y |  |  | unknown | 62&#8209;66 |
| 9 | `approx_metric_tsp` | Y |  |  | Y | Y |  |  | unknown | 70&#8209;75 |
| 10 | `euler_tour_dfs` |  |  |  | Y |  | Y | Y |  | 109&#8209;160 |
| 11 | `get_neighbors` |  |  |  | Y |  | Y | Y |  | 225&#8209;239 |
| 12 | `get_edge_weight` |  |  |  | Y |  | Y | Y |  | 241&#8209;256 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
