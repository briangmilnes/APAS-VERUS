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
| 1 | Chap17 | MathSeq | 18 | 19 | 1 | 0 | 19 | 1 | 19 | 0 | 1 |

## Function-by-Function Detail

### Chap17/MathSeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;127 |
| 2 | `set` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;136 |
| 3 | `length` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;140 |
| 4 | `nth` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |
| 5 | `empty` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;149 |
| 6 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;155 |
| 7 | `add_last` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;162 |
| 8 | `delete_last` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;171 |
| 9 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;175 |
| 10 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;179 |
| 11 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;183 |
| 12 | `with_len` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;189 |
| 13 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;199 |
| 14 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;207 |
| 15 | `domain` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;213 |
| 16 | `range` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;220 |
| 17 | `multiset_range` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;228 |
| 18 | `iter` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;236 |
| 19 | `eq` |  | Y |  |  | Y |  |  | unknown | 586&#8209;587 |
| 20 | `iter_mut` |  |  | Y |  |  | Y | Y |  | 617&#8209;621 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
