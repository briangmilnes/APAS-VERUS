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
| 1 | `new` | Y | Y |  |  | Y |  | Y |  | 32 |
| 2 | `value` | Y | Y |  |  | Y |  | Y |  | 36 |
| 3 | `infinity` | Y | Y |  |  | Y |  |  | hole | 40 |
| 4 | `zero` | Y | Y |  |  | Y |  | Y |  | 44 |
| 5 | `default` |  | Y |  |  | Y |  | Y |  | 60 |
| 6 | `eq` |  | Y |  |  | Y |  |  | hole | 68 |
| 7 | `partial_cmp` |  | Y |  |  | Y |  |  | hole | 79 |
| 8 | `cmp` |  | Y |  |  | Y |  |  | hole | 87 |
| 9 | `hash` |  | Y |  |  | Y |  |  | hole | 110 |
| 10 | `from` x2 |  | Y |  |  | Y |  |  | hole | 117 |
| 11 | `add` |  | Y |  |  | Y |  |  | hole | 133 |
| 12 | `sub` |  | Y |  |  | Y |  |  | hole | 142 |
| 13 | `mul` |  | Y |  |  | Y |  |  | hole | 151 |
| 14 | `div` |  | Y |  |  | Y |  |  | hole | 160 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
