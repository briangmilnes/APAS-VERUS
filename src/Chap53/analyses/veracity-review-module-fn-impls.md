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
| 1 | Chap53 | GraphSearchMtPer | 4 | 4 | 0 | 4 | 5 | 0 | 3 | 2 | 0 |
| 2 | Chap53 | GraphSearchStEph | 4 | 4 | 0 | 4 | 5 | 0 | 3 | 2 | 0 |
| 3 | Chap53 | GraphSearchStPer | 4 | 4 | 0 | 4 | 5 | 0 | 3 | 2 | 0 |
| 4 | Chap53 | PQMinStEph | 2 | 2 | 0 | 4 | 4 | 0 | 1 | 3 | 0 |
| 5 | Chap53 | PQMinStPer | 2 | 2 | 0 | 4 | 4 | 0 | 1 | 3 | 0 |

## Function-by-Function Detail

### Chap53/GraphSearchMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `select` x2 | Y | Y |  |  | Y |  |  | hole | 31&#8209;32 |
| 2 | `graph_search` | Y | Y |  | Y | Y |  |  | unknown | 38&#8209;42 |
| 3 | `graph_search_multi` | Y | Y |  | Y | Y |  |  | unknown | 46&#8209;50 |
| 4 | `reachable` | Y | Y |  | Y | Y |  |  | unknown | 54&#8209;57 |
| 5 | `graph_search_explore` |  |  |  | Y | Y |  |  | hole | 90&#8209;96 |

### Chap53/GraphSearchStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `select` x2 | Y | Y |  |  | Y |  |  | hole | 24&#8209;25 |
| 7 | `graph_search` | Y | Y |  | Y | Y |  |  | unknown | 31&#8209;35 |
| 8 | `graph_search_multi` | Y | Y |  | Y | Y |  |  | unknown | 39&#8209;43 |
| 9 | `reachable` | Y | Y |  | Y | Y |  |  | unknown | 47&#8209;50 |
| 10 | `graph_search_explore` |  |  |  | Y | Y |  |  | hole | 83&#8209;89 |

### Chap53/GraphSearchStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `select` x2 | Y | Y |  |  | Y |  |  | hole | 33&#8209;34 |
| 12 | `graph_search` | Y | Y |  | Y | Y |  |  | unknown | 40&#8209;44 |
| 13 | `graph_search_multi` | Y | Y |  | Y | Y |  |  | unknown | 48&#8209;52 |
| 14 | `reachable` | Y | Y |  | Y | Y |  |  | unknown | 56&#8209;59 |
| 15 | `graph_search_explore` |  |  |  | Y | Y |  |  | hole | 92&#8209;98 |

### Chap53/PQMinStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `pq_min` | Y | Y |  | Y | Y |  |  | unknown | 24&#8209;28 |
| 17 | `pq_min_multi` | Y | Y |  | Y | Y |  |  | hole | 32&#8209;36 |
| 18 | `pq_find_min_priority` |  |  |  | Y | Y |  |  | hole | 65&#8209;67 |
| 19 | `pq_explore` |  |  |  | Y | Y |  |  | hole | 77&#8209;82 |

### Chap53/PQMinStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 20 | `pq_min` | Y | Y |  | Y | Y |  |  | unknown | 29&#8209;33 |
| 21 | `pq_min_multi` | Y | Y |  | Y | Y |  |  | hole | 37&#8209;41 |
| 22 | `pq_find_min_priority` |  |  |  | Y | Y |  |  | hole | 70&#8209;72 |
| 23 | `pq_explore` |  |  |  | Y | Y |  |  | hole | 82&#8209;87 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
