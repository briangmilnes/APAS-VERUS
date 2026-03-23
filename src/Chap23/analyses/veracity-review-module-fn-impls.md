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
| 1 | Chap23 | BalBinTreeStEph | 8 | 10 | 3 | 3 | 16 | 0 | 14 | 2 | 0 |
| 2 | Chap23 | PrimTreeSeqStPer | 17 | 19 | 1 | 0 | 20 | 0 | 19 | 1 | 0 |

## Function-by-Function Detail

### Chap23/BalBinTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_in_order_pre_order_permutation` |  |  |  | Y | Y |  |  | unknown | 95&#8209;97 |
| 2 | `lemma_pre_order_post_order_permutation` |  |  |  | Y | Y |  |  | unknown | 140&#8209;142 |
| 3 | `leaf` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;199 |
| 4 | `node` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;210 |
| 5 | `is_leaf` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;216 |
| 6 | `size` | Y | Y |  |  | Y |  |  | unknown | 220&#8209;223 |
| 7 | `height` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;230 |
| 8 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;240 |
| 9 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;250 |
| 10 | `post_order` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;260 |
| 11 | `iter_in_order` |  |  | Y |  | Y |  |  | unknown | 444&#8209;450 |
| 12 | `iter_pre_order` |  |  | Y |  | Y |  |  | unknown | 458&#8209;464 |
| 13 | `iter_post_order` |  |  | Y |  | Y |  |  | unknown | 472&#8209;478 |
| 14 | `next` x3 |  | Y |  |  | Y |  |  | unknown | 564&#8209;580 |
| 15 | `eq` x2 |  | Y |  |  | Y |  |  | hole | 785&#8209;787 |
| 16 | `clone_tree` |  |  |  | Y | Y |  |  | hole | 819&#8209;821 |

### Chap23/PrimTreeSeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 17 | `empty` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;135 |
| 18 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;144 |
| 19 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;153 |
| 20 | `length` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;160 |
| 21 | `nth` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;168 |
| 22 | `expose` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;181 |
| 23 | `join` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;201 |
| 24 | `append` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;217 |
| 25 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;232 |
| 26 | `update` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;247 |
| 27 | `map` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;257 |
| 28 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;269 |
| 29 | `filter` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;286 |
| 30 | `drop` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;301 |
| 31 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 306&#8209;311 |
| 32 | `as_slice` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;318 |
| 33 | `into_vec` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;325 |
| 34 | `iter` |  |  | Y |  | Y |  |  | unknown | 335&#8209;339 |
| 35 | `next` |  | Y |  |  | Y |  |  | unknown | 784&#8209;800 |
| 36 | `eq` x2 |  | Y |  |  | Y |  |  | hole | 894&#8209;895 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
