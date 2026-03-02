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
| 1 | Chap30 | Probability | 4 | 14 | 0 | 0 | 14 | 0 | 0 | 10 | 4 |

## Function-by-Function Detail

### Chap30/Probability.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` | Y | Y |  |  | Y |  | Y |  | 30 |
| 2 | `value` | Y | Y |  |  | Y |  | Y |  | 34 |
| 3 | `infinity` | Y | Y |  |  | Y |  |  | hole | 38 |
| 4 | `zero` | Y | Y |  |  | Y |  | Y |  | 42 |
| 5 | `default` |  | Y |  |  | Y |  | Y |  | 56 |
| 6 | `eq` |  | Y |  |  | Y |  |  | hole | 64 |
| 7 | `partial_cmp` |  | Y |  |  | Y |  |  | hole | 73 |
| 8 | `cmp` |  | Y |  |  | Y |  |  | hole | 81 |
| 9 | `hash` |  | Y |  |  | Y |  |  | hole | 104 |
| 10 | `from` x2 |  | Y |  |  | Y |  |  | hole | 111 |
| 11 | `add` |  | Y |  |  | Y |  |  | hole | 125 |
| 12 | `sub` |  | Y |  |  | Y |  |  | hole | 132 |
| 13 | `mul` |  | Y |  |  | Y |  |  | hole | 139 |
| 14 | `div` |  | Y |  |  | Y |  |  | hole | 146 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
