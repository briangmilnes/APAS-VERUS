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
| 1 | `new` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;142 |
| 2 | `set` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;152 |
| 3 | `length` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;157 |
| 4 | `nth` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;163 |
| 5 | `empty` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;168 |
| 6 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;175 |
| 7 | `add_last` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;183 |
| 8 | `delete_last` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;193 |
| 9 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;198 |
| 10 | `is_singleton` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;203 |
| 11 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;208 |
| 12 | `with_len` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;215 |
| 13 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;226 |
| 14 | `subseq_copy` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;235 |
| 15 | `domain` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;240 |
| 16 | `range` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;248 |
| 17 | `multiset_range` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;257 |
| 18 | `iter` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;266 |
| 19 | `next` |  | Y |  |  | Y |  |  | unknown | 612&#8209;628 |
| 20 | `eq` |  | Y |  |  | Y |  |  | unknown | 719&#8209;720 |
| 21 | `iter_mut` |  |  | Y |  |  | Y | Y |  | 733&#8209;738 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
