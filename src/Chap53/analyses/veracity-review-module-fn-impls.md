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
| 1 | Chap53 | GraphSearchMtPer | 4 | 1 | 0 | 4 | 5 | 0 | 0 | 2 | 3 |
| 2 | Chap53 | GraphSearchStEph | 4 | 1 | 0 | 4 | 5 | 0 | 0 | 2 | 3 |
| 3 | Chap53 | GraphSearchStPer | 4 | 1 | 0 | 4 | 5 | 0 | 0 | 2 | 3 |
| 4 | Chap53 | PQMinStEph | 2 | 0 | 0 | 4 | 4 | 0 | 0 | 3 | 1 |
| 5 | Chap53 | PQMinStPer | 2 | 0 | 0 | 4 | 4 | 0 | 0 | 3 | 1 |

## Function-by-Function Detail

### Chap53/GraphSearchMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `select` x2 | Y | Y |  |  | Y |  |  | hole | 31 |
| 2 | `graph_search` | Y |  |  | Y | Y |  | Y |  | 37&#8209;40 |
| 3 | `graph_search_multi` | Y |  |  | Y | Y |  | Y |  | 44&#8209;47 |
| 4 | `reachable` | Y |  |  | Y | Y |  | Y |  | 51&#8209;53 |
| 5 | `graph_search_explore` |  |  |  | Y | Y |  |  | hole | 85&#8209;90 |

### Chap53/GraphSearchStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `select` x2 | Y | Y |  |  | Y |  |  | hole | 24 |
| 7 | `graph_search` | Y |  |  | Y | Y |  | Y |  | 30&#8209;33 |
| 8 | `graph_search_multi` | Y |  |  | Y | Y |  | Y |  | 37&#8209;40 |
| 9 | `reachable` | Y |  |  | Y | Y |  | Y |  | 44&#8209;46 |
| 10 | `graph_search_explore` |  |  |  | Y | Y |  |  | hole | 78&#8209;83 |

### Chap53/GraphSearchStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `select` x2 | Y | Y |  |  | Y |  |  | hole | 33 |
| 12 | `graph_search` | Y |  |  | Y | Y |  | Y |  | 39&#8209;42 |
| 13 | `graph_search_multi` | Y |  |  | Y | Y |  | Y |  | 46&#8209;49 |
| 14 | `reachable` | Y |  |  | Y | Y |  | Y |  | 53&#8209;55 |
| 15 | `graph_search_explore` |  |  |  | Y | Y |  |  | hole | 87&#8209;92 |

### Chap53/PQMinStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `pq_min` | Y |  |  | Y | Y |  | Y |  | 24&#8209;27 |
| 17 | `pq_min_multi` | Y |  |  | Y | Y |  |  | hole | 31&#8209;34 |
| 18 | `pq_find_min_priority` |  |  |  | Y | Y |  |  | hole | 52&#8209;54 |
| 19 | `pq_explore` |  |  |  | Y | Y |  |  | hole | 64&#8209;69 |

### Chap53/PQMinStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 20 | `pq_min` | Y |  |  | Y | Y |  | Y |  | 29&#8209;32 |
| 21 | `pq_min_multi` | Y |  |  | Y | Y |  |  | hole | 36&#8209;39 |
| 22 | `pq_find_min_priority` |  |  |  | Y | Y |  |  | hole | 57&#8209;59 |
| 23 | `pq_explore` |  |  |  | Y | Y |  |  | hole | 69&#8209;74 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
