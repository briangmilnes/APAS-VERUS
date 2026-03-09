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
| 17 | `empty` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 18 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;142 |
| 19 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;150 |
| 20 | `length` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;156 |
| 21 | `nth` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;163 |
| 22 | `expose` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;175 |
| 23 | `join` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;194 |
| 24 | `append` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;207 |
| 25 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;220 |
| 26 | `update` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;233 |
| 27 | `map` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;243 |
| 28 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;254 |
| 29 | `filter` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;271 |
| 30 | `drop` | Y | Y |  |  | Y |  |  | unknown | 276&#8209;284 |
| 31 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;294 |
| 32 | `as_slice` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;300 |
| 33 | `into_vec` | Y | Y |  |  | Y |  |  | unknown | 305&#8209;306 |
| 34 | `iter` |  |  | Y |  | Y |  |  | unknown | 316&#8209;320 |
| 35 | `next` |  | Y |  |  | Y |  |  | unknown | 757&#8209;773 |
| 36 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 859&#8209;860 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
