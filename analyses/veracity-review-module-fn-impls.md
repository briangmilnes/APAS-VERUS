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
| 1 | Chap41 | AVLTreeSetMtEph | 12 | 15 | 0 | 0 | 0 | 15 | 0 | 0 | 15 |
| 2 | Chap41 | AVLTreeSetMtPer | 12 | 16 | 0 | 0 | 0 | 16 | 0 | 0 | 16 |
| 3 | Chap41 | AVLTreeSetStEph | 12 | 13 | 0 | 0 | 0 | 13 | 0 | 0 | 13 |
| 4 | Chap41 | AVLTreeSetStPer | 12 | 13 | 0 | 0 | 0 | 13 | 0 | 0 | 13 |
| 5 | Chap41 | ArraySetEnumMtEph | 13 | 13 | 0 | 0 | 0 | 13 | 0 | 0 | 13 |
| 6 | Chap41 | ArraySetStEph | 12 | 13 | 0 | 0 | 0 | 13 | 0 | 0 | 13 |
| 7 | Chap41 | Example41_3 | 3 | 0 | 0 | 4 | 0 | 5 | 0 | 0 | 5 |

## Function-by-Function Detail

### Chap41/AVLTreeSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `size` | Y | Y |  |  |  | Y | Y |  | 33&#8209;35 |
| 2 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 36&#8209;38 |
| 3 | `empty` | Y | Y |  |  |  | Y | Y |  | 39&#8209;41 |
| 4 | `singleton` | Y | Y |  |  |  | Y | Y |  | 42&#8209;44 |
| 5 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 45&#8209;46 |
| 6 | `filter` | Y | Y |  |  |  | Y | Y |  | 47&#8209;49 |
| 7 | `intersection` | Y | Y |  |  |  | Y | Y |  | 50&#8209;52 |
| 8 | `difference` | Y | Y |  |  |  | Y | Y |  | 53&#8209;55 |
| 9 | `union` | Y | Y |  |  |  | Y | Y |  | 56&#8209;58 |
| 10 | `find` | Y | Y |  |  |  | Y | Y |  | 59&#8209;61 |
| 11 | `delete` | Y | Y |  |  |  | Y | Y |  | 62&#8209;64 |
| 12 | `insert` | Y | Y |  |  |  | Y | Y |  | 65&#8209;67 |
| 13 | `parallel_filter` |  | Y |  |  |  | Y | Y |  | 114&#8209;140 |
| 14 | `parallel_intersect` |  | Y |  |  |  | Y | Y |  | 171&#8209;202 |
| 15 | `default` |  | Y |  |  |  | Y | Y |  | 266 |

### Chap41/AVLTreeSetMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 34&#8209;36 |
| 17 | `cmp` |  | Y |  |  |  | Y | Y |  | 40&#8209;58 |
| 18 | `size` | Y | Y |  |  |  | Y | Y |  | 62&#8209;64 |
| 19 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 65&#8209;67 |
| 20 | `empty` | Y | Y |  |  |  | Y | Y |  | 68&#8209;70 |
| 21 | `singleton` | Y | Y |  |  |  | Y | Y |  | 71&#8209;73 |
| 22 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 74&#8209;75 |
| 23 | `filter` | Y | Y |  |  |  | Y | Y |  | 76&#8209;78 |
| 24 | `intersection` | Y | Y |  |  |  | Y | Y |  | 79&#8209;81 |
| 25 | `difference` | Y | Y |  |  |  | Y | Y |  | 82&#8209;84 |
| 26 | `union` | Y | Y |  |  |  | Y | Y |  | 85&#8209;87 |
| 27 | `find` | Y | Y |  |  |  | Y | Y |  | 88&#8209;90 |
| 28 | `delete` | Y | Y |  |  |  | Y | Y |  | 91&#8209;93 |
| 29 | `insert` | Y | Y |  |  |  | Y | Y |  | 94&#8209;96 |
| 30 | `parallel_sort` |  | Y |  |  |  | Y | Y |  | 122&#8209;155 |
| 31 | `default` |  | Y |  |  |  | Y | Y |  | 372 |

### Chap41/AVLTreeSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 32 | `size` | Y | Y |  |  |  | Y | Y |  | 25&#8209;27 |
| 33 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 28&#8209;30 |
| 34 | `empty` | Y | Y |  |  |  | Y | Y |  | 31&#8209;33 |
| 35 | `singleton` | Y | Y |  |  |  | Y | Y |  | 34&#8209;36 |
| 36 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 37&#8209;38 |
| 37 | `filter` | Y | Y |  |  |  | Y | Y |  | 39&#8209;41 |
| 38 | `intersection` | Y | Y |  |  |  | Y | Y |  | 42&#8209;44 |
| 39 | `difference` | Y | Y |  |  |  | Y | Y |  | 45&#8209;47 |
| 40 | `union` | Y | Y |  |  |  | Y | Y |  | 48&#8209;50 |
| 41 | `find` | Y | Y |  |  |  | Y | Y |  | 51&#8209;53 |
| 42 | `delete` | Y | Y |  |  |  | Y | Y |  | 54&#8209;56 |
| 43 | `insert` | Y | Y |  |  |  | Y | Y |  | 57&#8209;59 |
| 44 | `default` |  | Y |  |  |  | Y | Y |  | 212 |

### Chap41/AVLTreeSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 45 | `size` | Y | Y |  |  |  | Y | Y |  | 24&#8209;26 |
| 46 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 27&#8209;29 |
| 47 | `empty` | Y | Y |  |  |  | Y | Y |  | 30&#8209;32 |
| 48 | `singleton` | Y | Y |  |  |  | Y | Y |  | 33&#8209;35 |
| 49 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 36&#8209;37 |
| 50 | `filter` | Y | Y |  |  |  | Y | Y |  | 38&#8209;40 |
| 51 | `intersection` | Y | Y |  |  |  | Y | Y |  | 41&#8209;43 |
| 52 | `difference` | Y | Y |  |  |  | Y | Y |  | 44&#8209;46 |
| 53 | `union` | Y | Y |  |  |  | Y | Y |  | 47&#8209;49 |
| 54 | `find` | Y | Y |  |  |  | Y | Y |  | 50&#8209;52 |
| 55 | `delete` | Y | Y |  |  |  | Y | Y |  | 53&#8209;55 |
| 56 | `insert` | Y | Y |  |  |  | Y | Y |  | 56&#8209;58 |
| 57 | `default` |  | Y |  |  |  | Y | Y |  | 248 |

### Chap41/ArraySetEnumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 58 | `new` | Y | Y |  |  |  | Y | Y |  | 21&#8209;22 |
| 59 | `size` | Y | Y |  |  |  | Y | Y |  | 23&#8209;25 |
| 60 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 26&#8209;28 |
| 61 | `empty` | Y | Y |  |  |  | Y | Y |  | 29&#8209;30 |
| 62 | `singleton` | Y | Y |  |  |  | Y | Y |  | 31&#8209;33 |
| 63 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 34&#8209;35 |
| 64 | `filter` | Y | Y |  |  |  | Y | Y |  | 36&#8209;38 |
| 65 | `intersection` | Y | Y |  |  |  | Y | Y |  | 39&#8209;41 |
| 66 | `difference` | Y | Y |  |  |  | Y | Y |  | 42&#8209;44 |
| 67 | `union` | Y | Y |  |  |  | Y | Y |  | 45&#8209;47 |
| 68 | `find` | Y | Y |  |  |  | Y | Y |  | 48&#8209;50 |
| 69 | `delete` | Y | Y |  |  |  | Y | Y |  | 51&#8209;53 |
| 70 | `insert` | Y | Y |  |  |  | Y | Y |  | 54&#8209;56 |

### Chap41/ArraySetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 71 | `size` | Y | Y |  |  |  | Y | Y |  | 17&#8209;18 |
| 72 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 19&#8209;20 |
| 73 | `empty` | Y | Y |  |  |  | Y | Y |  | 21&#8209;22 |
| 74 | `singleton` | Y | Y |  |  |  | Y | Y |  | 23&#8209;24 |
| 75 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 25&#8209;26 |
| 76 | `filter` | Y | Y |  |  |  | Y | Y |  | 27&#8209;28 |
| 77 | `intersection` | Y | Y |  |  |  | Y | Y |  | 29&#8209;30 |
| 78 | `difference` | Y | Y |  |  |  | Y | Y |  | 31&#8209;32 |
| 79 | `union` | Y | Y |  |  |  | Y | Y |  | 33&#8209;34 |
| 80 | `find` | Y | Y |  |  |  | Y | Y |  | 35&#8209;36 |
| 81 | `delete` | Y | Y |  |  |  | Y | Y |  | 37&#8209;38 |
| 82 | `insert` | Y | Y |  |  |  | Y | Y |  | 39&#8209;40 |
| 83 | `default` |  | Y |  |  |  | Y | Y |  | 176 |

### Chap41/Example41_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 84 | `example_41_1_array_set` | Y |  |  | Y |  | Y | Y |  | 15&#8209;17 |
| 85 | `example_41_1_avl_set` | Y |  |  | Y |  | Y | Y |  | 19&#8209;21 |
| 86 | `demonstrate_set_operations` | Y |  |  |  |  | Y | Y |  | 23&#8209;25 |
| 87 | `example_41_3_from_seq_demonstration` |  |  |  | Y |  | Y | Y |  | 128&#8209;172 |
| 88 | `additional_set_operations` |  |  |  | Y |  | Y | Y |  | 174&#8209;215 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
