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
| 1 | Chap12 | Exercise12_1 | 4 | 5 | 0 | 1 | 6 | 0 | 0 | 5 | 1 |
| 2 | Chap12 | Exercise12_2 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| 3 | Chap12 | Exercise12_5 | 5 | 7 | 0 | 0 | 7 | 0 | 0 | 6 | 1 |

## Function-by-Function Detail

### Chap12/Exercise12_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` | Y | Y |  |  | Y |  |  | hole | 56&#8209;57 |
| 2 | `lock` | Y | Y |  |  | Y |  |  | hole | 61&#8209;62 |
| 3 | `unlock` | Y | Y |  |  | Y |  |  | hole | 66&#8209;68 |
| 4 | `with_lock` | Y | Y |  |  | Y |  |  | hole | 74 |
| 5 | `parallel_increment` |  |  |  | Y | Y |  |  | hole | 120&#8209;121 |
| 6 | `default` |  | Y |  |  | Y |  | Y |  | 150 |

### Chap12/Exercise12_2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `fetch_add_cas` | Y | Y |  |  | Y |  | Y |  | 39 |

### Chap12/Exercise12_5.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 8 | `new` | Y | Y |  |  | Y |  |  | hole | 72&#8209;73 |
| 9 | `push` | Y | Y |  |  | Y |  |  | hole | 77&#8209;78 |
| 10 | `pop` | Y | Y |  |  | Y |  |  | hole | 84&#8209;85 |
| 11 | `is_empty` | Y | Y |  |  | Y |  |  | hole | 90&#8209;91 |
| 12 | `drain` | Y | Y |  |  | Y |  |  | hole | 97&#8209;98 |
| 13 | `drop` |  | Y |  |  | Y |  |  | hole | 164&#8209;166 |
| 14 | `default` |  | Y |  |  | Y |  | Y |  | 182 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
