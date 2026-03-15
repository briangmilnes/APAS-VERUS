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
| 4 | Chap53 | PQMinStEph | 2 | 2 | 0 | 4 | 4 | 0 | 4 | 0 | 0 |
| 5 | Chap53 | PQMinStPer | 2 | 2 | 0 | 4 | 4 | 0 | 4 | 0 | 0 |

## Function-by-Function Detail

### Chap53/GraphSearchMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `select` x2 | Y | Y |  |  | Y |  |  | unknown | 36&#8209;38 |
| 2 | `graph_search` | Y | Y |  | Y | Y |  |  | unknown | 44&#8209;49 |
| 3 | `graph_search_multi` | Y | Y |  | Y | Y |  |  | unknown | 53&#8209;60 |
| 4 | `reachable` | Y | Y |  | Y | Y |  |  | unknown | 64&#8209;68 |
| 5 | `graph_search_explore` |  |  |  | Y | Y |  |  | unknown | 116&#8209;129 |

### Chap53/GraphSearchStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `select` x2 | Y | Y |  |  | Y |  |  | unknown | 28&#8209;32 |
| 7 | `graph_search` | Y | Y |  | Y | Y |  |  | unknown | 38&#8209;45 |
| 8 | `graph_search_multi` | Y | Y |  | Y | Y |  |  | unknown | 49&#8209;57 |
| 9 | `reachable` | Y | Y |  | Y | Y |  |  | unknown | 61&#8209;67 |
| 10 | `graph_search_explore` |  |  |  | Y | Y |  |  | unknown | 117&#8209;130 |

### Chap53/GraphSearchStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `select` x2 | Y | Y |  |  | Y |  |  | unknown | 37&#8209;41 |
| 12 | `graph_search` | Y | Y |  | Y | Y |  |  | unknown | 47&#8209;54 |
| 13 | `graph_search_multi` | Y | Y |  | Y | Y |  |  | unknown | 58&#8209;66 |
| 14 | `reachable` | Y | Y |  | Y | Y |  |  | unknown | 70&#8209;76 |
| 15 | `graph_search_explore` |  |  |  | Y | Y |  |  | unknown | 125&#8209;138 |

### Chap53/PQMinStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `pq_min` | Y | Y |  | Y | Y |  |  | unknown | 39&#8209;49 |
| 17 | `pq_min_multi` | Y | Y |  | Y | Y |  |  | unknown | 53&#8209;64 |
| 18 | `pq_find_min_priority` |  |  |  | Y | Y |  |  | unknown | 102&#8209;110 |
| 19 | `pq_explore` |  |  |  | Y | Y |  |  | unknown | 129&#8209;144 |

### Chap53/PQMinStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 20 | `pq_min` | Y | Y |  | Y | Y |  |  | unknown | 43&#8209;53 |
| 21 | `pq_min_multi` | Y | Y |  | Y | Y |  |  | unknown | 57&#8209;68 |
| 22 | `pq_find_min_priority` |  |  |  | Y | Y |  |  | unknown | 104&#8209;112 |
| 23 | `pq_explore` |  |  |  | Y | Y |  |  | unknown | 125&#8209;140 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
