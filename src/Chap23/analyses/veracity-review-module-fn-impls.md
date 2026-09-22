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
| 1 | Chap23 | BalBinTreeStEph | 8 | 9 | 3 | 3 | 15 | 0 | 15 | 0 | 0 |
| 2 | Chap23 | PrimTreeSeqStPer | 17 | 18 | 1 | 0 | 19 | 0 | 19 | 0 | 0 |

## Function-by-Function Detail

### Chap23/BalBinTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `leaf` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;93 |
| 2 | `node` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;104 |
| 3 | `is_leaf` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 4 | `size` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;117 |
| 5 | `height` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;124 |
| 6 | `in_order` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;134 |
| 7 | `pre_order` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;144 |
| 8 | `post_order` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;154 |
| 9 | `iter_in_order` |  |  | Y |  | Y |  |  | unknown | 345&#8209;351 |
| 10 | `iter_pre_order` |  |  | Y |  | Y |  |  | unknown | 359&#8209;365 |
| 11 | `iter_post_order` |  |  | Y |  | Y |  |  | unknown | 373&#8209;379 |
| 12 | `lemma_in_order_pre_order_permutation` |  |  |  | Y | Y |  |  | unknown | 406&#8209;408 |
| 13 | `lemma_pre_order_post_order_permutation` |  |  |  | Y | Y |  |  | unknown | 442&#8209;444 |
| 14 | `clone_tree` |  |  |  | Y | Y |  |  | unknown | 479&#8209;481 |
| 15 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 512&#8209;514 |

### Chap23/PrimTreeSeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `iter` |  |  | Y |  | Y |  |  | unknown | 114&#8209;118 |
| 17 | `empty` | Y | Y |  |  | Y |  |  | unknown | 655&#8209;657 |
| 18 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 662&#8209;666 |
| 19 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 670&#8209;674 |
| 20 | `length` | Y | Y |  |  | Y |  |  | unknown | 679&#8209;681 |
| 21 | `nth` | Y | Y |  |  | Y |  |  | unknown | 687&#8209;690 |
| 22 | `expose` | Y | Y |  |  | Y |  |  | unknown | 695&#8209;703 |
| 23 | `join` | Y | Y |  |  | Y |  |  | unknown | 718&#8209;723 |
| 24 | `append` | Y | Y |  |  | Y |  |  | unknown | 728&#8209;739 |
| 25 | `subseq` | Y | Y |  |  | Y |  |  | unknown | 744&#8209;754 |
| 26 | `update` | Y | Y |  |  | Y |  |  | unknown | 760&#8209;770 |
| 27 | `map` | Y | Y |  |  | Y |  |  | unknown | 775&#8209;780 |
| 28 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 785&#8209;792 |
| 29 | `filter` | Y | Y |  |  | Y |  |  | unknown | 797&#8209;809 |
| 30 | `drop` | Y | Y |  |  | Y |  |  | unknown | 814&#8209;824 |
| 31 | `flatten` | Y | Y |  |  | Y |  |  | unknown | 829&#8209;834 |
| 32 | `as_slice` | Y | Y |  |  | Y |  |  | unknown | 838&#8209;840 |
| 33 | `into_vec` | Y | Y |  |  | Y |  |  | unknown | 844&#8209;846 |
| 34 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 875&#8209;876 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
