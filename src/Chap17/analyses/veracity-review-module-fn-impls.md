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
| 1 | Chap17 | MathSeq | 18 | 20 | 1 | 0 | 20 | 1 | 20 | 0 | 1 |

## Function-by-Function Detail

### Chap17/MathSeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;147 |
| 2 | `set` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;157 |
| 3 | `length` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;162 |
| 4 | `nth` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;168 |
| 5 | `empty` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;173 |
| 6 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;180 |
| 7 | `add_last` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;188 |
| 8 | `delete_last` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;198 |
| 9 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;203 |
| 10 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;208 |
| 11 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;213 |
| 12 | `with_len` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;220 |
| 13 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 224&#8209;231 |
| 14 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;240 |
| 15 | `domain` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;245 |
| 16 | `range` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;253 |
| 17 | `multiset_range` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;262 |
| 18 | `iter` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;271 |
| 19 | `next` |  | Y |  |  | Y |  |  | unknown | 617&#8209;633 |
| 20 | `eq` |  | Y |  |  | Y |  |  | unknown | 724&#8209;725 |
| 21 | `iter_mut` |  |  | Y |  |  | Y | Y |  | 738&#8209;743 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
