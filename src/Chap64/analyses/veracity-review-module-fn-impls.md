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
| 1 | Chap64 | SpanTreeMtEph | 2 | 0 | 0 | 4 | 4 | 0 | 0 | 2 | 2 |
| 2 | Chap64 | SpanTreeStEph | 2 | 0 | 0 | 2 | 2 | 0 | 0 | 0 | 2 |
| 3 | Chap64 | TSPApproxStEph | 4 | 0 | 0 | 8 | 1 | 7 | 0 | 0 | 8 |

## Function-by-Function Detail

### Chap64/SpanTreeMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_spanning_edges_lock` |  |  |  | Y | Y |  |  | hole | 36&#8209;38 |
| 2 | `new_valid_lock` |  |  |  | Y | Y |  |  | hole | 47 |
| 3 | `spanning_tree_star_contraction_mt` | Y |  |  | Y | Y |  | Y |  | 54&#8209;56 |
| 4 | `verify_spanning_tree` | Y |  |  | Y | Y |  | Y |  | 60 |

### Chap64/SpanTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `spanning_tree_star_contraction` | Y |  |  | Y | Y |  | Y |  | 27 |
| 6 | `verify_spanning_tree` | Y |  |  | Y | Y |  | Y |  | 31 |

### Chap64/TSPApproxStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `_tsp_approx_st_eph_verified` |  |  |  | Y | Y |  | Y |  | 14 |
| 8 | `euler_tour` | Y |  |  | Y |  | Y | Y |  | 41&#8209;43 |
| 9 | `shortcut_tour` | Y |  |  | Y |  | Y | Y |  | 45&#8209;47 |
| 10 | `tour_weight` | Y |  |  | Y |  | Y | Y |  | 49&#8209;54 |
| 11 | `approx_metric_tsp` | Y |  |  | Y |  | Y | Y |  | 56&#8209;61 |
| 12 | `euler_tour_dfs` |  |  |  | Y |  | Y | Y |  | 93&#8209;144 |
| 13 | `get_neighbors` |  |  |  | Y |  | Y | Y |  | 209&#8209;223 |
| 14 | `get_edge_weight` |  |  |  | Y |  | Y | Y |  | 225&#8209;240 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
