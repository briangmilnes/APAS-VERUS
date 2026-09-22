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
| 1 | Chap57 | DijkstraStEphF64 | 1 | 8 | 0 | 2 | 10 | 0 | 2 | 8 | 0 |
| 2 | Chap57 | DijkstraStEphU64 | 1 | 6 | 0 | 2 | 8 | 0 | 2 | 2 | 4 |
| 3 | Chap57 | StackStEph | 6 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |

## Function-by-Function Detail

### Chap57/DijkstraStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `dijkstra` | Y |  |  | Y | Y |  |  | unknown | 93&#8209;102 |
| 2 | `pq_entry_new` |  |  |  | Y | Y |  |  | unknown | 110&#8209;111 |
| 3 | `cmp` x2 |  | Y |  |  | Y |  |  | hole | 118 |
| 4 | `partial_cmp` |  | Y |  |  | Y |  |  | hole | 137 |
| 5 | `reflexive` |  | Y |  |  | Y |  |  | hole | 148 |
| 6 | `transitive` |  | Y |  |  | Y |  |  | hole | 150 |
| 7 | `antisymmetric` |  | Y |  |  | Y |  |  | hole | 152 |
| 8 | `total` |  | Y |  |  | Y |  |  | hole | 154 |
| 9 | `cmp_spec_less_implies_le` |  | Y |  |  | Y |  |  | hole | 174 |
| 10 | `cmp_spec_greater_implies_le` |  | Y |  |  | Y |  |  | hole | 176 |

### Chap57/DijkstraStEphU64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `dijkstra` | Y |  |  | Y | Y |  |  | unknown | 89&#8209;98 |
| 12 | `pq_entry_new` |  |  |  | Y | Y |  |  | unknown | 106&#8209;107 |
| 13 | `cmp` x2 |  | Y |  |  | Y |  |  | hole | 114 |
| 14 | `partial_cmp` |  | Y |  |  | Y |  |  | hole | 131 |
| 15 | `reflexive` |  | Y |  |  | Y |  | Y |  | 141 |
| 16 | `transitive` |  | Y |  |  | Y |  | Y |  | 142 |
| 17 | `antisymmetric` |  | Y |  |  | Y |  | Y |  | 143 |
| 18 | `total` |  | Y |  |  | Y |  | Y |  | 144 |

### Chap57/StackStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 19 | `new` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 20 | `push` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;87 |
| 21 | `pop` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 22 | `peek` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 23 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;100 |
| 24 | `size` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 25 | `default` |  | Y |  |  | Y |  |  | unknown | 167&#8209;168 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
