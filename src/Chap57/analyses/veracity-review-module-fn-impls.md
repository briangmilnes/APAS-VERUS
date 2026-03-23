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
| 2 | Chap57 | DijkstraStEphU64 | 1 | 6 | 0 | 2 | 8 | 0 | 2 | 2 | 4 |
| 3 | Chap57 | StackStEph | 6 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |

## Function-by-Function Detail

### Chap57/DijkstraStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 42&#8209;44 |
| 2 | `cmp` |  | Y |  |  |  | Y | Y |  | 48&#8209;51 |

### Chap57/DijkstraStEphU64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3 | `dijkstra` | Y |  |  | Y | Y |  |  | unknown | 88&#8209;97 |
| 4 | `pq_entry_new` |  |  |  | Y | Y |  |  | unknown | 105&#8209;106 |
| 5 | `cmp` x2 |  | Y |  |  | Y |  |  | hole | 113 |
| 6 | `partial_cmp` |  | Y |  |  | Y |  |  | hole | 130 |
| 7 | `reflexive` |  | Y |  |  | Y |  | Y |  | 140 |
| 8 | `transitive` |  | Y |  |  | Y |  | Y |  | 141 |
| 9 | `antisymmetric` |  | Y |  |  | Y |  | Y |  | 142 |
| 10 | `total` |  | Y |  |  | Y |  | Y |  | 143 |

### Chap57/StackStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `new` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 12 | `push` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 13 | `pop` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 14 | `peek` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;87 |
| 15 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 16 | `size` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;97 |
| 17 | `default` |  | Y |  |  | Y |  |  | unknown | 152&#8209;153 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
