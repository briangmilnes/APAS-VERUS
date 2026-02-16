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
| 1 | Chap23 | BalBinTreeStEph | 7 | 9 | 2 | 0 | 11 | 0 | 10 | 1 | 0 |
| 2 | Chap23 | PrimTreeSeqStPer | 15 | 17 | 3 | 0 | 18 | 2 | 17 | 1 | 2 |

## Function-by-Function Detail

### Chap23/BalBinTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `leaf` | Y | Y |  |  | Y |  |  | strong | 118&#8209;122 |
| 2 | `node` | Y | Y |  |  | Y |  |  | strong | 125&#8209;130 |
| 3 | `is_leaf` | Y | Y |  |  | Y |  |  | strong | 133&#8209;134 |
| 4 | `size` | Y | Y |  |  | Y |  |  | strong | 137&#8209;139 |
| 5 | `height` | Y | Y |  |  | Y |  |  | strong | 142&#8209;144 |
| 6 | `in_order` | Y | Y |  |  | Y |  |  | strong | 148&#8209;152 |
| 7 | `pre_order` | Y | Y |  |  | Y |  |  | strong | 156&#8209;160 |
| 8 | `next` x2 |  | Y |  |  | Y |  |  | strong | 323&#8209;339 |
| 9 | `iter_in_order` |  |  | Y |  | Y |  |  | strong | 456&#8209;462 |
| 10 | `iter_pre_order` |  |  | Y |  | Y |  |  | strong | 468&#8209;474 |
| 11 | `eq` x2 |  | Y |  |  | Y |  |  | strong | 491&#8209;493 |

### Chap23/PrimTreeSeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 12 | `iter` |  |  | Y |  | Y |  |  | strong | 117&#8209;121 |
| 13 | `empty` | Y | Y |  |  | Y |  |  | strong | 135&#8209;136 |
| 14 | `singleton` | Y | Y |  |  | Y |  |  | strong | 140&#8209;143 |
| 15 | `from_vec` | Y | Y |  |  | Y |  |  | strong | 147&#8209;150 |
| 16 | `length` | Y | Y |  |  | Y |  |  | strong | 154&#8209;155 |
| 17 | `nth` | Y | Y |  |  | Y |  |  | strong | 159&#8209;161 |
| 18 | `expose` | Y | Y |  |  | Y |  |  | strong | 165&#8209;172 |
| 19 | `join` | Y | Y |  |  | Y |  |  | none | 185 |
| 20 | `append` | Y | Y |  |  | Y |  |  | strong | 189&#8209;197 |
| 21 | `subseq` | Y | Y |  |  | Y |  |  | strong | 201&#8209;209 |
| 22 | `update` | Y | Y |  |  | Y |  |  | strong | 213&#8209;221 |
| 23 | `map` | Y | Y |  |  | Y |  |  | strong | 225&#8209;230 |
| 24 | `tabulate` | Y | Y |  |  | Y |  |  | strong | 234&#8209;240 |
| 25 | `filter` | Y | Y |  |  | Y |  |  | strong | 244&#8209;256 |
| 26 | `drop` | Y | Y |  |  | Y |  |  | strong | 260&#8209;268 |
| 27 | `flatten` | Y | Y |  |  | Y |  |  | strong | 272&#8209;277 |
| 28 | `next` |  | Y |  |  | Y |  |  | strong | 697&#8209;713 |
| 29 | `eq` x2 |  | Y |  |  | Y |  |  | strong | 799&#8209;800 |
| 30 | `as_slice` |  |  | Y |  |  | Y | Y | none | 861 |
| 31 | `into_vec` |  |  | Y |  |  | Y | Y | none | 862 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.