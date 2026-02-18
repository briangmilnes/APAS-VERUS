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
| 1 | Chap57 | DijkstraStEphFloat | 1 | 2 | 0 | 2 | 4 | 0 | 0 | 4 | 0 |
| 2 | Chap57 | DijkstraStEphInt | 1 | 2 | 0 | 2 | 4 | 0 | 0 | 4 | 0 |
| 3 | Chap57 | StackStEph | 4 | 1 | 6 | 0 | 7 | 0 | 0 | 7 | 0 |

## Function-by-Function Detail

### Chap57/DijkstraStEphFloat.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `dijkstra` | Y |  |  | Y | Y |  |  | hole | 62 |
| 2 | `pq_entry_new` |  |  |  | Y | Y |  |  | hole | 71 |
| 3 | `cmp` |  | Y |  |  | Y |  |  | hole | 77 |
| 4 | `partial_cmp` |  | Y |  |  | Y |  |  | hole | 87 |

### Chap57/DijkstraStEphInt.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `dijkstra` | Y |  |  | Y | Y |  |  | hole | 62 |
| 6 | `pq_entry_new` |  |  |  | Y | Y |  |  | hole | 71 |
| 7 | `cmp` |  | Y |  |  | Y |  |  | hole | 77 |
| 8 | `partial_cmp` |  | Y |  |  | Y |  |  | hole | 87 |

### Chap57/StackStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 9 | `new` | Y |  | Y |  | Y |  |  | hole | 56 |
| 10 | `push` | Y |  | Y |  | Y |  |  | hole | 61 |
| 11 | `pop` | Y |  | Y |  | Y |  |  | hole | 66 |
| 12 | `is_empty` | Y |  | Y |  | Y |  |  | hole | 71 |
| 13 | `peek` |  |  | Y |  | Y |  |  | hole | 101 |
| 14 | `size` |  |  | Y |  | Y |  |  | hole | 113 |
| 15 | `default` |  | Y |  |  | Y |  |  | hole | 122 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
