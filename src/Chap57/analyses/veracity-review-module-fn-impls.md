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
| 1 | Chap57 | DijkstraStEphF64 | 0 | 2 | 0 | 0 | 0 | 2 | 0 | 0 | 2 |
| 2 | Chap57 | DijkstraStEphI64 | 1 | 2 | 0 | 2 | 4 | 0 | 2 | 0 | 2 |
| 3 | Chap57 | StackStEph | 6 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |

## Function-by-Function Detail

### Chap57/DijkstraStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 33&#8209;35 |
| 2 | `cmp` |  | Y |  |  |  | Y | Y |  | 39&#8209;42 |

### Chap57/DijkstraStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3 | `dijkstra` | Y |  |  | Y | Y |  |  | unknown | 63 |
| 4 | `pq_entry_new` |  |  |  | Y | Y |  |  | unknown | 68&#8209;70 |
| 5 | `cmp` |  | Y |  |  | Y |  | Y |  | 76 |
| 6 | `partial_cmp` |  | Y |  |  | Y |  | Y |  | 88 |

### Chap57/StackStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `new` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 8 | `push` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;72 |
| 9 | `pop` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;76 |
| 10 | `peek` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 11 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 12 | `size` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 13 | `default` |  | Y |  |  | Y |  |  | unknown | 140&#8209;141 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
