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
| 1 | Chap23 | BalBinTreeStEph | 8 | 10 | 3 | 3 | 16 | 0 | 16 | 0 | 0 |
| 2 | Chap23 | PrimTreeSeqStPer | 17 | 19 | 1 | 0 | 20 | 0 | 20 | 0 | 0 |

## Function-by-Function Detail

### Chap23/BalBinTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_in_order_pre_order_permutation` |  |  |  | Y | Y |  |  | unknown | 96&#8209;98 |
| 2 | `lemma_pre_order_post_order_permutation` |  |  |  | Y | Y |  |  | unknown | 141&#8209;143 |
| 3 | `leaf` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;200 |
| 4 | `node` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;211 |
| 5 | `is_leaf` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;217 |
| 6 | `size` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;224 |
| 7 | `height` | Y | Y |  |  | Y |  |  | unknown | 228&#8209;231 |
| 8 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;241 |
| 9 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;251 |
| 10 | `post_order` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;261 |
| 11 | `iter_in_order` |  |  | Y |  | Y |  |  | unknown | 445&#8209;451 |
| 12 | `iter_pre_order` |  |  | Y |  | Y |  |  | unknown | 459&#8209;465 |
| 13 | `iter_post_order` |  |  | Y |  | Y |  |  | unknown | 473&#8209;479 |
| 14 | `next` x3 |  | Y |  |  | Y |  |  | unknown | 563&#8209;579 |
| 15 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 778&#8209;780 |
| 16 | `clone_tree` |  |  |  | Y | Y |  |  | unknown | 807&#8209;810 |

### Chap23/PrimTreeSeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 17 | `empty` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;136 |
| 18 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;145 |
| 19 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;154 |
| 20 | `length` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;161 |
| 21 | `nth` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;169 |
| 22 | `expose` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;182 |
| 23 | `join` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;202 |
| 24 | `append` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;218 |
| 25 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;233 |
| 26 | `update` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;248 |
| 27 | `map` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;258 |
| 28 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;270 |
| 29 | `filter` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;287 |
| 30 | `drop` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;302 |
| 31 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;312 |
| 32 | `as_slice` | Y | Y |  |  | Y |  |  | unknown | 317&#8209;319 |
| 33 | `into_vec` | Y | Y |  |  | Y |  |  | unknown | 324&#8209;326 |
| 34 | `iter` |  |  | Y |  | Y |  |  | unknown | 336&#8209;340 |
| 35 | `next` |  | Y |  |  | Y |  |  | unknown | 783&#8209;799 |
| 36 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 885&#8209;886 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
