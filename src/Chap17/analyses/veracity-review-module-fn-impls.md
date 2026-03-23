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
| 1 | Chap17 | MathSeq | 18 | 20 | 1 | 0 | 20 | 1 | 19 | 1 | 1 |

## Function-by-Function Detail

### Chap17/MathSeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;141 |
| 2 | `set` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;151 |
| 3 | `length` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;156 |
| 4 | `nth` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;162 |
| 5 | `empty` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;167 |
| 6 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;174 |
| 7 | `add_last` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;182 |
| 8 | `delete_last` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;192 |
| 9 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;197 |
| 10 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;202 |
| 11 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;207 |
| 12 | `with_len` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;214 |
| 13 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;225 |
| 14 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;234 |
| 15 | `domain` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;241 |
| 16 | `range` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;249 |
| 17 | `multiset_range` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;258 |
| 18 | `iter` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;267 |
| 19 | `next` |  | Y |  |  | Y |  |  | unknown | 614&#8209;630 |
| 20 | `eq` |  | Y |  |  | Y |  |  | hole | 721&#8209;722 |
| 21 | `iter_mut` |  |  | Y |  |  | Y | Y |  | 735&#8209;740 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
