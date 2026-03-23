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
| 1 | Chap53 | GraphSearchMtPer | 4 | 4 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 2 | Chap53 | GraphSearchStEph | 4 | 4 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 3 | Chap53 | GraphSearchStPer | 4 | 4 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 4 | Chap53 | PQMinStEph | 2 | 2 | 0 | 4 | 4 | 0 | 3 | 1 | 0 |
| 5 | Chap53 | PQMinStPer | 2 | 2 | 0 | 4 | 4 | 0 | 3 | 1 | 0 |

## Function-by-Function Detail

### Chap53/GraphSearchMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `select` x2 | Y | Y |  |  | Y |  |  | unknown | 35&#8209;37 |
| 2 | `graph_search` | Y | Y |  | Y | Y |  |  | unknown | 43&#8209;48 |
| 3 | `graph_search_multi` | Y | Y |  | Y | Y |  |  | unknown | 52&#8209;59 |
| 4 | `reachable` | Y | Y |  | Y | Y |  |  | unknown | 63&#8209;67 |
| 5 | `graph_search_explore` |  |  |  | Y | Y |  |  | unknown | 115&#8209;128 |

### Chap53/GraphSearchStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `select` x2 | Y | Y |  |  | Y |  |  | unknown | 28&#8209;32 |
| 7 | `graph_search` | Y | Y |  | Y | Y |  |  | unknown | 38&#8209;49 |
| 8 | `graph_search_multi` | Y | Y |  | Y | Y |  |  | unknown | 53&#8209;65 |
| 9 | `reachable` | Y | Y |  | Y | Y |  |  | unknown | 69&#8209;79 |
| 10 | `graph_search_explore` |  |  |  | Y | Y |  |  | unknown | 143&#8209;162 |

### Chap53/GraphSearchStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `select` x2 | Y | Y |  |  | Y |  |  | unknown | 37&#8209;41 |
| 12 | `graph_search` | Y | Y |  | Y | Y |  |  | unknown | 47&#8209;58 |
| 13 | `graph_search_multi` | Y | Y |  | Y | Y |  |  | unknown | 62&#8209;74 |
| 14 | `reachable` | Y | Y |  | Y | Y |  |  | unknown | 78&#8209;88 |
| 15 | `graph_search_explore` |  |  |  | Y | Y |  |  | unknown | 151&#8209;170 |

### Chap53/PQMinStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `pq_min` | Y | Y |  | Y | Y |  |  | unknown | 40&#8209;54 |
| 17 | `pq_min_multi` | Y | Y |  | Y | Y |  |  | unknown | 58&#8209;73 |
| 18 | `pq_find_min_priority` |  |  |  | Y | Y |  |  | unknown | 123&#8209;131 |
| 19 | `pq_explore` |  |  |  | Y | Y |  |  | hole | 150&#8209;166 |

### Chap53/PQMinStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 20 | `pq_min` | Y | Y |  | Y | Y |  |  | unknown | 45&#8209;59 |
| 21 | `pq_min_multi` | Y | Y |  | Y | Y |  |  | unknown | 63&#8209;78 |
| 22 | `pq_find_min_priority` |  |  |  | Y | Y |  |  | unknown | 126&#8209;134 |
| 23 | `pq_explore` |  |  |  | Y | Y |  |  | hole | 147&#8209;163 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
