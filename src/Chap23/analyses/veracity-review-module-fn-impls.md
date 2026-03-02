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
| 3 | `leaf` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;198 |
| 4 | `node` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;208 |
| 5 | `is_leaf` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;213 |
| 6 | `size` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;219 |
| 7 | `height` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;225 |
| 8 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;234 |
| 9 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;243 |
| 10 | `post_order` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;252 |
| 11 | `iter_in_order` |  |  | Y |  | Y |  |  | unknown | 432&#8209;438 |
| 12 | `iter_pre_order` |  |  | Y |  | Y |  |  | unknown | 446&#8209;452 |
| 13 | `iter_post_order` |  |  | Y |  | Y |  |  | unknown | 460&#8209;466 |
| 14 | `next` x3 |  | Y |  |  | Y |  |  | unknown | 550&#8209;566 |
| 15 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 765&#8209;767 |
| 16 | `clone_tree` |  |  |  | Y | Y |  |  | unknown | 794&#8209;796 |

### Chap23/PrimTreeSeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 17 | `empty` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 18 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;137 |
| 19 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;145 |
| 20 | `length` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;151 |
| 21 | `nth` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;158 |
| 22 | `expose` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;170 |
| 23 | `join` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;189 |
| 24 | `append` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;202 |
| 25 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;215 |
| 26 | `update` | Y | Y |  |  | Y |  |  | unknown | 220&#8209;228 |
| 27 | `map` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;238 |
| 28 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 243&#8209;249 |
| 29 | `filter` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;266 |
| 30 | `drop` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;279 |
| 31 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;289 |
| 32 | `as_slice` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;295 |
| 33 | `into_vec` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;301 |
| 34 | `iter` |  |  | Y |  | Y |  |  | unknown | 321&#8209;325 |
| 35 | `next` |  | Y |  |  | Y |  |  | unknown | 760&#8209;776 |
| 36 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 862&#8209;863 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
