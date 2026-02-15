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
| 1 | Chap23 | BalBinTreeStEph | 7 | 9 | 2 | 0 | 10 | 1 | 10 | 0 | 1 |
| 2 | Chap23 | PrimTreeSeqStPer | 15 | 17 | 3 | 0 | 18 | 2 | 17 | 1 | 2 |

## Function-by-Function Detail

### Chap23/BalBinTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `leaf` | Y | Y |  |  | Y |  |  | strong | 115&#8209;119 |
| 2 | `node` | Y | Y |  |  | Y |  |  | strong | 122&#8209;127 |
| 3 | `is_leaf` | Y | Y |  |  | Y |  |  | strong | 130&#8209;131 |
| 4 | `size` | Y | Y |  |  | Y |  |  | strong | 134&#8209;136 |
| 5 | `height` | Y | Y |  |  | Y |  |  | strong | 139&#8209;141 |
| 6 | `in_order` | Y | Y |  |  | Y |  |  | strong | 145&#8209;149 |
| 7 | `pre_order` | Y | Y |  |  | Y |  |  | strong | 153&#8209;157 |
| 8 | `next` x2 |  | Y |  |  | Y |  |  | strong | 320&#8209;336 |
| 9 | `iter_in_order` |  |  | Y |  | Y |  |  | strong | 453&#8209;459 |
| 10 | `iter_pre_order` |  |  | Y |  | Y |  |  | strong | 465&#8209;471 |
| 11 | `eq` x2 |  | Y |  |  |  | Y | Y | none | 497&#8209;504 |

### Chap23/PrimTreeSeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 12 | `iter` |  |  | Y |  | Y |  |  | strong | 115&#8209;119 |
| 13 | `empty` | Y | Y |  |  | Y |  |  | strong | 133&#8209;134 |
| 14 | `singleton` | Y | Y |  |  | Y |  |  | strong | 138&#8209;141 |
| 15 | `from_vec` | Y | Y |  |  | Y |  |  | strong | 145&#8209;148 |
| 16 | `length` | Y | Y |  |  | Y |  |  | strong | 152&#8209;153 |
| 17 | `nth` | Y | Y |  |  | Y |  |  | strong | 157&#8209;159 |
| 18 | `expose` | Y | Y |  |  | Y |  |  | strong | 163&#8209;170 |
| 19 | `join` | Y | Y |  |  | Y |  |  | none | 183 |
| 20 | `append` | Y | Y |  |  | Y |  |  | strong | 187&#8209;195 |
| 21 | `subseq` | Y | Y |  |  | Y |  |  | strong | 199&#8209;207 |
| 22 | `update` | Y | Y |  |  | Y |  |  | strong | 211&#8209;219 |
| 23 | `map` | Y | Y |  |  | Y |  |  | strong | 223&#8209;228 |
| 24 | `tabulate` | Y | Y |  |  | Y |  |  | strong | 232&#8209;238 |
| 25 | `filter` | Y | Y |  |  | Y |  |  | strong | 242&#8209;254 |
| 26 | `drop` | Y | Y |  |  | Y |  |  | strong | 258&#8209;266 |
| 27 | `flatten` | Y | Y |  |  | Y |  |  | strong | 270&#8209;275 |
| 28 | `next` |  | Y |  |  | Y |  |  | strong | 695&#8209;711 |
| 29 | `eq` x2 |  | Y |  |  | Y |  |  | strong | 797&#8209;798 |
| 30 | `as_slice` |  |  | Y |  |  | Y | Y | none | 859 |
| 31 | `into_vec` |  |  | Y |  |  | Y | Y | none | 860 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.