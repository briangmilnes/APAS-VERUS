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
| 1 | Chap39 | BSTSetTreapMtEph | 20 | 20 | 0 | 2 | 0 | 22 | 0 | 0 | 22 |

## Function-by-Function Detail

### Chap39/BSTSetTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `minimum_inner` |  |  |  | Y |  | Y | Y |  | 50&#8209;58 |
| 2 | `maximum_inner` |  |  |  | Y |  | Y | Y |  | 60&#8209;68 |
| 3 | `empty` | Y | Y |  |  |  | Y | Y |  | 73&#8209;74 |
| 4 | `singleton` | Y | Y |  |  |  | Y | Y |  | 75&#8209;76 |
| 5 | `size` | Y | Y |  |  |  | Y | Y |  | 77&#8209;78 |
| 6 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 79&#8209;80 |
| 7 | `find` | Y | Y |  |  |  | Y | Y |  | 81&#8209;82 |
| 8 | `contains` | Y | Y |  |  |  | Y | Y |  | 83&#8209;84 |
| 9 | `minimum` | Y | Y |  |  |  | Y | Y |  | 85&#8209;86 |
| 10 | `maximum` | Y | Y |  |  |  | Y | Y |  | 87&#8209;88 |
| 11 | `insert` | Y | Y |  |  |  | Y | Y |  | 89&#8209;90 |
| 12 | `delete` | Y | Y |  |  |  | Y | Y |  | 91&#8209;92 |
| 13 | `union` | Y | Y |  |  |  | Y | Y |  | 93&#8209;94 |
| 14 | `intersection` | Y | Y |  |  |  | Y | Y |  | 95&#8209;96 |
| 15 | `difference` | Y | Y |  |  |  | Y | Y |  | 97&#8209;98 |
| 16 | `split` | Y | Y |  |  |  | Y | Y |  | 99&#8209;100 |
| 17 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 101&#8209;102 |
| 18 | `join_m` | Y | Y |  |  |  | Y | Y |  | 103&#8209;104 |
| 19 | `filter` | Y | Y |  |  |  | Y | Y |  | 105&#8209;106 |
| 20 | `reduce` | Y | Y |  |  |  | Y | Y |  | 107&#8209;110 |
| 21 | `iter_in_order` | Y | Y |  |  |  | Y | Y |  | 111&#8209;112 |
| 22 | `as_tree` | Y | Y |  |  |  | Y | Y |  | 113&#8209;114 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
