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
| 1 | Chap53 | GraphSearchMtPer | 4 | 1 | 0 | 4 | 5 | 0 | 0 | 4 | 1 |
| 2 | Chap53 | GraphSearchStEph | 4 | 1 | 0 | 4 | 5 | 0 | 0 | 0 | 5 |
| 3 | Chap53 | GraphSearchStPer | 4 | 1 | 0 | 4 | 5 | 0 | 0 | 4 | 1 |
| 4 | Chap53 | PQMinStEph | 4 | 0 | 2 | 4 | 6 | 0 | 0 | 1 | 5 |
| 5 | Chap53 | PQMinStPer | 4 | 0 | 2 | 4 | 6 | 0 | 0 | 4 | 2 |

## Function-by-Function Detail

### Chap53/GraphSearchMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `select` x2 | Y | Y |  |  | Y |  |  | hole | 30 |
| 2 | `graph_search` | Y |  |  | Y | Y |  |  | hole | 36&#8209;39 |
| 3 | `graph_search_multi` | Y |  |  | Y | Y |  |  | hole | 43&#8209;46 |
| 4 | `reachable` | Y |  |  | Y | Y |  |  | hole | 50&#8209;52 |
| 5 | `explore` |  |  |  | Y | Y |  | Y |  | 99&#8209;108 |

### Chap53/GraphSearchStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `select` x2 | Y | Y |  |  | Y |  | Y |  | 25 |
| 7 | `graph_search` | Y |  |  | Y | Y |  | Y |  | 31&#8209;34 |
| 8 | `graph_search_multi` | Y |  |  | Y | Y |  | Y |  | 38&#8209;41 |
| 9 | `reachable` | Y |  |  | Y | Y |  | Y |  | 45&#8209;47 |
| 10 | `graph_search_explore` |  |  |  | Y | Y |  | Y |  | 77&#8209;82 |

### Chap53/GraphSearchStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `select` x2 | Y | Y |  |  | Y |  |  | hole | 33 |
| 12 | `graph_search` | Y |  |  | Y | Y |  |  | hole | 39&#8209;42 |
| 13 | `graph_search_multi` | Y |  |  | Y | Y |  |  | hole | 46&#8209;49 |
| 14 | `reachable` | Y |  |  | Y | Y |  |  | hole | 53&#8209;55 |
| 15 | `explore` |  |  |  | Y | Y |  | Y |  | 103&#8209;112 |

### Chap53/PQMinStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `priority` | Y |  | Y |  | Y |  |  | hole | 30 |
| 17 | `pq_min` | Y |  |  | Y | Y |  | Y |  | 36&#8209;39 |
| 18 | `pq_min_multi` | Y |  |  | Y | Y |  | Y |  | 43&#8209;46 |
| 19 | `new` | Y |  | Y |  | Y |  | Y |  | 50 |
| 20 | `pq_find_min_priority` |  |  |  | Y | Y |  | Y |  | 80&#8209;82 |
| 21 | `pq_explore` |  |  |  | Y | Y |  | Y |  | 91&#8209;96 |

### Chap53/PQMinStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 22 | `priority` | Y |  | Y |  | Y |  |  | hole | 29 |
| 23 | `pq_min` | Y |  |  | Y | Y |  |  | hole | 42&#8209;45 |
| 24 | `pq_min_multi` | Y |  |  | Y | Y |  |  | hole | 49&#8209;52 |
| 25 | `new` | Y |  | Y |  | Y |  |  | hole | 56 |
| 26 | `find_min_priority` |  |  |  | Y | Y |  | Y |  | 101 |
| 27 | `explore` |  |  |  | Y | Y |  | Y |  | 112&#8209;122 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
