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
| 4 | Chap41 | AVLTreeSetStPer | 12 | 14 | 0 | 0 | 12 | 2 | 0 | 12 | 2 |
| 5 | Chap41 | ArraySetEnumMtEph | 13 | 14 | 0 | 0 | 13 | 1 | 0 | 13 | 1 |
| 6 | Chap41 | ArraySetStEph | 12 | 14 | 0 | 3 | 15 | 2 | 7 | 8 | 2 |
| 7 | Chap41 | Example41_3 | 3 | 3 | 0 | 8 | 9 | 0 | 0 | 4 | 5 |

## Function-by-Function Detail

### Chap41/AVLTreeSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_set_mt_lock` |  |  |  | Y | Y |  |  | hole | 60 |
| 2 | `size` | Y | Y |  |  | Y |  |  | hole | 87&#8209;88 |
| 3 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 91&#8209;92 |
| 4 | `empty` | Y | Y |  |  | Y |  |  | hole | 95&#8209;96 |
| 5 | `singleton` | Y | Y |  |  | Y |  |  | hole | 99&#8209;100 |
| 6 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 102&#8209;103 |
| 7 | `filter` | Y | Y |  |  | Y |  |  | hole | 106&#8209;107 |
| 8 | `intersection` | Y | Y |  |  | Y |  |  | hole | 110&#8209;111 |
| 9 | `difference` | Y | Y |  |  | Y |  |  | hole | 114&#8209;115 |
| 10 | `union` | Y | Y |  |  | Y |  |  | hole | 118&#8209;119 |
| 11 | `find` | Y | Y |  |  | Y |  |  | hole | 122&#8209;123 |
| 12 | `delete` | Y | Y |  |  | Y |  |  | hole | 126&#8209;127 |
| 13 | `insert` | Y | Y |  |  | Y |  |  | hole | 130&#8209;131 |
| 14 | `default` |  | Y |  |  |  | Y | Y |  | 408 |

### Chap41/AVLTreeSetMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 15 | `size` | Y | Y |  |  | Y |  |  | hole | 76&#8209;77 |
| 16 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 80&#8209;81 |
| 17 | `empty` | Y | Y |  |  | Y |  |  | hole | 84&#8209;85 |
| 18 | `singleton` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 19 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 91&#8209;92 |
| 20 | `filter` | Y | Y |  |  | Y |  |  | hole | 95&#8209;96 |
| 21 | `intersection` | Y | Y |  |  | Y |  |  | hole | 99&#8209;100 |
| 22 | `difference` | Y | Y |  |  | Y |  |  | hole | 103&#8209;104 |
| 23 | `union` | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 24 | `find` | Y | Y |  |  | Y |  |  | hole | 111&#8209;112 |
| 25 | `delete` | Y | Y |  |  | Y |  |  | hole | 115&#8209;116 |
| 26 | `insert` | Y | Y |  |  | Y |  |  | hole | 119&#8209;120 |
| 27 | `default` |  | Y |  |  |  | Y | Y |  | 474 |
| 28 | `eq` |  | Y |  |  |  | Y | Y |  | 478&#8209;487 |
| 29 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 493&#8209;495 |
| 30 | `cmp` |  | Y |  |  |  | Y | Y |  | 499&#8209;517 |

### Chap41/AVLTreeSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 31 | `size` | Y | Y |  |  | Y |  |  | hole | 64&#8209;65 |
| 32 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 68&#8209;69 |
| 33 | `empty` | Y | Y |  |  | Y |  |  | hole | 72&#8209;73 |
| 34 | `singleton` | Y | Y |  |  | Y |  |  | hole | 76&#8209;77 |
| 35 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 79&#8209;80 |
| 36 | `filter` | Y | Y |  |  | Y |  |  | hole | 83&#8209;84 |
| 37 | `intersection` | Y | Y |  |  | Y |  |  | hole | 87&#8209;88 |
| 38 | `difference` | Y | Y |  |  | Y |  |  | hole | 91&#8209;92 |
| 39 | `union` | Y | Y |  |  | Y |  |  | hole | 95&#8209;96 |
| 40 | `find` | Y | Y |  |  | Y |  |  | hole | 99&#8209;100 |
| 41 | `delete` | Y | Y |  |  | Y |  |  | hole | 103&#8209;104 |
| 42 | `insert` | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 43 | `default` |  | Y |  |  |  | Y | Y |  | 440 |
| 44 | `eq` |  | Y |  |  |  | Y | Y |  | 444&#8209;453 |

### Chap41/AVLTreeSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 45 | `size` | Y | Y |  |  | Y |  |  | hole | 64&#8209;65 |
| 46 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 68&#8209;69 |
| 47 | `empty` | Y | Y |  |  | Y |  |  | hole | 72&#8209;73 |
| 48 | `singleton` | Y | Y |  |  | Y |  |  | hole | 76&#8209;77 |
| 49 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 79&#8209;80 |
| 50 | `filter` | Y | Y |  |  | Y |  |  | hole | 83&#8209;84 |
| 51 | `intersection` | Y | Y |  |  | Y |  |  | hole | 87&#8209;88 |
| 52 | `difference` | Y | Y |  |  | Y |  |  | hole | 91&#8209;92 |
| 53 | `union` | Y | Y |  |  | Y |  |  | hole | 95&#8209;96 |
| 54 | `find` | Y | Y |  |  | Y |  |  | hole | 99&#8209;100 |
| 55 | `delete` | Y | Y |  |  | Y |  |  | hole | 103&#8209;104 |
| 56 | `insert` | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 57 | `default` |  | Y |  |  |  | Y | Y |  | 448 |
| 58 | `eq` |  | Y |  |  |  | Y | Y |  | 452&#8209;461 |

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
| 73 | `lemma_filter_remove` |  |  |  | Y | Y |  |  | unknown | 82&#8209;85 |
| 74 | `lemma_push_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 121&#8209;126 |
| 75 | `lemma_subseq_no_dups_subset` |  |  |  | Y | Y |  |  | unknown | 147&#8209;153 |
| 76 | `size` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;172 |
| 77 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 174&#8209;179 |
| 78 | `empty` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;182 |
| 79 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;185 |
| 80 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;188 |
| 81 | `filter` | Y | Y |  |  | Y |  |  | hole | 190&#8209;197 |
| 82 | `intersection` | Y | Y |  |  | Y |  |  | hole | 199&#8209;208 |
| 83 | `difference` | Y | Y |  |  | Y |  |  | hole | 210&#8209;219 |
| 84 | `union` | Y | Y |  |  | Y |  |  | hole | 221&#8209;230 |
| 85 | `find` | Y | Y |  |  | Y |  |  | hole | 232&#8209;234 |
| 86 | `delete` | Y | Y |  |  | Y |  |  | hole | 236&#8209;243 |
| 87 | `insert` | Y | Y |  |  | Y |  |  | hole | 245&#8209;252 |
| 88 | `default` |  | Y |  |  |  | Y | Y |  | 592 |
| 89 | `eq` |  | Y |  |  |  | Y | Y |  | 596&#8209;605 |

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
