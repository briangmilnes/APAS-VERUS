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
| 1 | Chap41 | AVLTreeSetMtEph | 12 | 13 | 0 | 1 | 13 | 1 | 0 | 13 | 1 |
| 2 | Chap41 | AVLTreeSetMtPer | 12 | 16 | 0 | 0 | 12 | 4 | 0 | 12 | 4 |
| 3 | Chap41 | AVLTreeSetStEph | 12 | 14 | 0 | 0 | 12 | 2 | 0 | 12 | 2 |
| 4 | Chap41 | AVLTreeSetStPer | 12 | 14 | 0 | 0 | 14 | 0 | 4 | 9 | 1 |
| 5 | Chap41 | ArraySetEnumMtEph | 13 | 14 | 0 | 0 | 13 | 1 | 0 | 13 | 1 |
| 6 | Chap41 | ArraySetStEph | 12 | 14 | 0 | 3 | 15 | 2 | 7 | 8 | 2 |
| 7 | Chap41 | Example41_3 | 3 | 3 | 0 | 8 | 9 | 0 | 0 | 4 | 5 |

## Function-by-Function Detail

### Chap41/AVLTreeSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `size` | Y | Y |  |  | Y |  |  | hole | 95&#8209;96 |
| 2 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 99&#8209;100 |
| 3 | `empty` | Y | Y |  |  | Y |  |  | hole | 103&#8209;104 |
| 4 | `singleton` | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 5 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 110&#8209;111 |
| 6 | `filter` | Y | Y |  |  | Y |  |  | hole | 114&#8209;115 |
| 7 | `intersection` | Y | Y |  |  | Y |  |  | hole | 118&#8209;119 |
| 8 | `difference` | Y | Y |  |  | Y |  |  | hole | 122&#8209;123 |
| 9 | `union` | Y | Y |  |  | Y |  |  | hole | 126&#8209;127 |
| 10 | `find` | Y | Y |  |  | Y |  |  | hole | 130&#8209;131 |
| 11 | `delete` | Y | Y |  |  | Y |  |  | hole | 134&#8209;135 |
| 12 | `insert` | Y | Y |  |  | Y |  |  | hole | 138&#8209;139 |
| 13 | `new_set_mt_lock` |  |  |  | Y | Y |  |  | hole | 150 |
| 14 | `default` |  | Y |  |  |  | Y | Y |  | 443 |

### Chap41/AVLTreeSetMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 15 | `size` | Y | Y |  |  | Y |  |  | hole | 94&#8209;95 |
| 16 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 98&#8209;99 |
| 17 | `empty` | Y | Y |  |  | Y |  |  | hole | 102&#8209;103 |
| 18 | `singleton` | Y | Y |  |  | Y |  |  | hole | 106&#8209;107 |
| 19 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 109&#8209;110 |
| 20 | `filter` | Y | Y |  |  | Y |  |  | hole | 113&#8209;114 |
| 21 | `intersection` | Y | Y |  |  | Y |  |  | hole | 117&#8209;118 |
| 22 | `difference` | Y | Y |  |  | Y |  |  | hole | 121&#8209;122 |
| 23 | `union` | Y | Y |  |  | Y |  |  | hole | 125&#8209;126 |
| 24 | `find` | Y | Y |  |  | Y |  |  | hole | 129&#8209;130 |
| 25 | `delete` | Y | Y |  |  | Y |  |  | hole | 133&#8209;134 |
| 26 | `insert` | Y | Y |  |  | Y |  |  | hole | 137&#8209;138 |
| 27 | `default` |  | Y |  |  |  | Y | Y |  | 509 |
| 28 | `eq` |  | Y |  |  |  | Y | Y |  | 516&#8209;525 |
| 29 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 531&#8209;533 |
| 30 | `cmp` |  | Y |  |  |  | Y | Y |  | 537&#8209;555 |

### Chap41/AVLTreeSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 31 | `size` | Y | Y |  |  | Y |  |  | hole | 82&#8209;83 |
| 32 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 86&#8209;87 |
| 33 | `empty` | Y | Y |  |  | Y |  |  | hole | 90&#8209;91 |
| 34 | `singleton` | Y | Y |  |  | Y |  |  | hole | 94&#8209;95 |
| 35 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 97&#8209;98 |
| 36 | `filter` | Y | Y |  |  | Y |  |  | hole | 101&#8209;102 |
| 37 | `intersection` | Y | Y |  |  | Y |  |  | hole | 105&#8209;106 |
| 38 | `difference` | Y | Y |  |  | Y |  |  | hole | 109&#8209;110 |
| 39 | `union` | Y | Y |  |  | Y |  |  | hole | 113&#8209;114 |
| 40 | `find` | Y | Y |  |  | Y |  |  | hole | 117&#8209;118 |
| 41 | `delete` | Y | Y |  |  | Y |  |  | hole | 121&#8209;122 |
| 42 | `insert` | Y | Y |  |  | Y |  |  | hole | 125&#8209;126 |
| 43 | `default` |  | Y |  |  |  | Y | Y |  | 475 |
| 44 | `eq` |  | Y |  |  |  | Y | Y |  | 482&#8209;491 |

### Chap41/AVLTreeSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 45 | `size` | Y | Y |  |  | Y |  |  | hole | 69&#8209;71 |
| 46 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;76 |
| 47 | `empty` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;82 |
| 48 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 49 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 50 | `filter` | Y | Y |  |  | Y |  |  | hole | 98&#8209;103 |
| 51 | `intersection` | Y | Y |  |  | Y |  |  | hole | 106&#8209;113 |
| 52 | `difference` | Y | Y |  |  | Y |  |  | hole | 116&#8209;123 |
| 53 | `union` | Y | Y |  |  | Y |  |  | hole | 126&#8209;133 |
| 54 | `find` | Y | Y |  |  | Y |  |  | hole | 136&#8209;138 |
| 55 | `delete` | Y | Y |  |  | Y |  |  | hole | 141&#8209;146 |
| 56 | `insert` | Y | Y |  |  | Y |  |  | hole | 149&#8209;154 |
| 57 | `default` |  | Y |  |  | Y |  | Y |  | 488 |
| 58 | `eq` |  | Y |  |  | Y |  |  | hole | 498&#8209;499 |

### Chap41/ArraySetEnumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 59 | `new` | Y | Y |  |  | Y |  |  | hole | 55&#8209;56 |
| 60 | `size` | Y | Y |  |  | Y |  |  | hole | 59&#8209;60 |
| 61 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 63&#8209;64 |
| 62 | `empty` | Y | Y |  |  | Y |  |  | hole | 66&#8209;67 |
| 63 | `singleton` | Y | Y |  |  | Y |  |  | hole | 70&#8209;74 |
| 64 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 76&#8209;77 |
| 65 | `filter` | Y | Y |  |  | Y |  |  | hole | 80&#8209;81 |
| 66 | `intersection` | Y | Y |  |  | Y |  |  | hole | 84&#8209;85 |
| 67 | `difference` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 68 | `union` | Y | Y |  |  | Y |  |  | hole | 92&#8209;93 |
| 69 | `find` | Y | Y |  |  | Y |  |  | hole | 96&#8209;97 |
| 70 | `delete` | Y | Y |  |  | Y |  |  | hole | 100&#8209;101 |
| 71 | `insert` | Y | Y |  |  | Y |  |  | hole | 104&#8209;105 |
| 72 | `eq` |  | Y |  |  |  | Y | Y |  | 299&#8209;308 |

### Chap41/ArraySetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 73 | `lemma_filter_remove` |  |  |  | Y | Y |  |  | unknown | 101&#8209;104 |
| 74 | `lemma_push_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 140&#8209;145 |
| 75 | `lemma_subseq_no_dups_subset` |  |  |  | Y | Y |  |  | unknown | 166&#8209;172 |
| 76 | `size` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;194 |
| 77 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 196&#8209;201 |
| 78 | `empty` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;204 |
| 79 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;207 |
| 80 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;210 |
| 81 | `filter` | Y | Y |  |  | Y |  |  | hole | 212&#8209;219 |
| 82 | `intersection` | Y | Y |  |  | Y |  |  | hole | 221&#8209;230 |
| 83 | `difference` | Y | Y |  |  | Y |  |  | hole | 232&#8209;241 |
| 84 | `union` | Y | Y |  |  | Y |  |  | hole | 243&#8209;252 |
| 85 | `find` | Y | Y |  |  | Y |  |  | hole | 254&#8209;256 |
| 86 | `delete` | Y | Y |  |  | Y |  |  | hole | 258&#8209;265 |
| 87 | `insert` | Y | Y |  |  | Y |  |  | hole | 267&#8209;274 |
| 88 | `default` |  | Y |  |  |  | Y | Y |  | 631 |
| 89 | `eq` |  | Y |  |  |  | Y | Y |  | 638&#8209;647 |

### Chap41/Example41_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 90 | `example_41_1_array_set` | Y | Y |  | Y | Y |  | Y |  | 21 |
| 91 | `example_41_1_avl_set` | Y | Y |  | Y | Y |  | Y |  | 25 |
| 92 | `demonstrate_set_operations` | Y | Y |  |  | Y |  | Y |  | 29 |
| 93 | `example_41_1_array_set_impl` |  |  |  | Y | Y |  |  | hole | 33&#8209;34 |
| 94 | `example_41_1_avl_set_impl` |  |  |  | Y | Y |  |  | hole | 86&#8209;87 |
| 95 | `example_41_3_from_seq_demonstration_impl` |  |  |  | Y | Y |  |  | hole | 139&#8209;140 |
| 96 | `additional_set_operations_impl` |  |  |  | Y | Y |  |  | hole | 188&#8209;189 |
| 97 | `example_41_3_from_seq_demonstration` |  |  |  | Y | Y |  | Y |  | 246 |
| 98 | `additional_set_operations` |  |  |  | Y | Y |  | Y |  | 247 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
