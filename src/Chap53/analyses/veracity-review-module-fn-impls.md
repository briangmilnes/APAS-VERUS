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
| 1 | `graph_search` | Y | Y |  | Y | Y |  |  | unknown | 118&#8209;130 |
| 2 | `graph_search_multi` | Y | Y |  | Y | Y |  |  | unknown | 134&#8209;147 |
| 3 | `reachable` | Y | Y |  | Y | Y |  |  | unknown | 151&#8209;162 |
| 4 | `select` x2 | Y | Y |  |  | Y |  |  | unknown | 107&#8209;112 |
| 5 | `graph_search_explore` |  |  |  | Y | Y |  |  | unknown | 218&#8209;239 |

### Chap53/GraphSearchStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `graph_search` | Y | Y |  | Y | Y |  |  | unknown | 110&#8209;122 |
| 7 | `graph_search_multi` | Y | Y |  | Y | Y |  |  | unknown | 126&#8209;139 |
| 8 | `reachable` | Y | Y |  | Y | Y |  |  | unknown | 143&#8209;154 |
| 9 | `select` x2 | Y | Y |  |  | Y |  |  | unknown | 100&#8209;104 |
| 10 | `graph_search_explore` |  |  |  | Y | Y |  |  | unknown | 213&#8209;233 |

### Chap53/GraphSearchStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `graph_search` | Y | Y |  | Y | Y |  |  | unknown | 117&#8209;129 |
| 12 | `graph_search_multi` | Y | Y |  | Y | Y |  |  | unknown | 133&#8209;146 |
| 13 | `reachable` | Y | Y |  | Y | Y |  |  | unknown | 150&#8209;161 |
| 14 | `select` x2 | Y | Y |  |  | Y |  |  | unknown | 107&#8209;111 |
| 15 | `graph_search_explore` |  |  |  | Y | Y |  |  | unknown | 221&#8209;241 |

### Chap53/PQMinStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `pq_min` | Y | Y |  | Y | Y |  |  | unknown | 74&#8209;99 |
| 17 | `pq_min_multi` | Y | Y |  | Y | Y |  |  | unknown | 103&#8209;128 |
| 18 | `pq_find_min_priority` |  |  |  | Y | Y |  |  | unknown | 189&#8209;197 |
| 19 | `pq_explore` |  |  |  | Y | Y |  |  | unknown | 218&#8209;259 |

### Chap53/PQMinStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 20 | `pq_min` | Y | Y |  | Y | Y |  |  | unknown | 77&#8209;97 |
| 21 | `pq_min_multi` | Y | Y |  | Y | Y |  |  | unknown | 101&#8209;122 |
| 22 | `pq_find_min_priority` |  |  |  | Y | Y |  |  | unknown | 179&#8209;187 |
| 23 | `pq_explore` |  |  |  | Y | Y |  |  | unknown | 208&#8209;242 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
