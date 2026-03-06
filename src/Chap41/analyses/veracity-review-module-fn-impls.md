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
| 1 | Chap41 | AVLTreeSetMtEph | 12 | 13 | 0 | 1 | 14 | 0 | 0 | 13 | 1 |
| 2 | Chap41 | AVLTreeSetMtPer | 12 | 16 | 0 | 0 | 16 | 0 | 3 | 10 | 3 |
| 3 | Chap41 | AVLTreeSetStEph | 12 | 14 | 0 | 0 | 14 | 0 | 1 | 12 | 1 |
| 4 | Chap41 | AVLTreeSetStPer | 12 | 14 | 0 | 0 | 14 | 0 | 4 | 9 | 1 |
| 5 | Chap41 | ArraySetEnumMtEph | 13 | 14 | 0 | 0 | 14 | 0 | 0 | 14 | 0 |
| 6 | Chap41 | ArraySetStEph | 12 | 14 | 0 | 7 | 21 | 0 | 11 | 9 | 1 |
| 7 | Chap41 | Example41_3 | 3 | 3 | 0 | 8 | 9 | 0 | 3 | 4 | 2 |

## Function-by-Function Detail

### Chap41/AVLTreeSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `size` | Y | Y |  |  | Y |  |  | hole | 77&#8209;78 |
| 2 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 81&#8209;82 |
| 3 | `empty` | Y | Y |  |  | Y |  |  | hole | 85&#8209;86 |
| 4 | `singleton` | Y | Y |  |  | Y |  |  | hole | 89&#8209;90 |
| 5 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 92&#8209;93 |
| 6 | `filter` | Y | Y |  |  | Y |  |  | hole | 96&#8209;97 |
| 7 | `intersection` | Y | Y |  |  | Y |  |  | hole | 100&#8209;101 |
| 8 | `difference` | Y | Y |  |  | Y |  |  | hole | 104&#8209;105 |
| 9 | `union` | Y | Y |  |  | Y |  |  | hole | 108&#8209;109 |
| 10 | `find` | Y | Y |  |  | Y |  |  | hole | 112&#8209;113 |
| 11 | `delete` | Y | Y |  |  | Y |  |  | hole | 116&#8209;117 |
| 12 | `insert` | Y | Y |  |  | Y |  |  | hole | 120&#8209;121 |
| 13 | `new_set_mt_lock` |  |  |  | Y | Y |  |  | hole | 134 |
| 14 | `default` |  | Y |  |  | Y |  | Y |  | 390 |

### Chap41/AVLTreeSetMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 15 | `size` | Y | Y |  |  | Y |  |  | hole | 77&#8209;78 |
| 16 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 17 | `empty` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;86 |
| 18 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 19 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 92&#8209;93 |
| 20 | `filter` | Y | Y |  |  | Y |  |  | hole | 96&#8209;97 |
| 21 | `intersection` | Y | Y |  |  | Y |  |  | hole | 100&#8209;101 |
| 22 | `difference` | Y | Y |  |  | Y |  |  | hole | 104&#8209;105 |
| 23 | `union` | Y | Y |  |  | Y |  |  | hole | 108&#8209;109 |
| 24 | `find` | Y | Y |  |  | Y |  |  | hole | 112&#8209;113 |
| 25 | `delete` | Y | Y |  |  | Y |  |  | hole | 116&#8209;117 |
| 26 | `insert` | Y | Y |  |  | Y |  |  | hole | 120&#8209;121 |
| 27 | `default` |  | Y |  |  | Y |  | Y |  | 473 |
| 28 | `eq` |  | Y |  |  | Y |  |  | hole | 485&#8209;486 |
| 29 | `partial_cmp` |  | Y |  |  | Y |  | Y |  | 514 |
| 30 | `cmp` |  | Y |  |  | Y |  | Y |  | 521 |

### Chap41/AVLTreeSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 31 | `size` | Y | Y |  |  | Y |  |  | hole | 70&#8209;71 |
| 32 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 74&#8209;75 |
| 33 | `empty` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 34 | `singleton` | Y | Y |  |  | Y |  |  | hole | 82&#8209;83 |
| 35 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 85&#8209;86 |
| 36 | `filter` | Y | Y |  |  | Y |  |  | hole | 89&#8209;90 |
| 37 | `intersection` | Y | Y |  |  | Y |  |  | hole | 93&#8209;94 |
| 38 | `difference` | Y | Y |  |  | Y |  |  | hole | 97&#8209;98 |
| 39 | `union` | Y | Y |  |  | Y |  |  | hole | 101&#8209;102 |
| 40 | `find` | Y | Y |  |  | Y |  |  | hole | 105&#8209;106 |
| 41 | `delete` | Y | Y |  |  | Y |  |  | hole | 109&#8209;110 |
| 42 | `insert` | Y | Y |  |  | Y |  |  | hole | 113&#8209;114 |
| 43 | `default` |  | Y |  |  | Y |  | Y |  | 441 |
| 44 | `eq` |  | Y |  |  | Y |  |  | hole | 453&#8209;454 |

### Chap41/AVLTreeSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 45 | `size` | Y | Y |  |  | Y |  |  | hole | 63&#8209;65 |
| 46 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;70 |
| 47 | `empty` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;76 |
| 48 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 49 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 50 | `filter` | Y | Y |  |  | Y |  |  | hole | 92&#8209;97 |
| 51 | `intersection` | Y | Y |  |  | Y |  |  | hole | 100&#8209;107 |
| 52 | `difference` | Y | Y |  |  | Y |  |  | hole | 110&#8209;117 |
| 53 | `union` | Y | Y |  |  | Y |  |  | hole | 120&#8209;127 |
| 54 | `find` | Y | Y |  |  | Y |  |  | hole | 130&#8209;132 |
| 55 | `delete` | Y | Y |  |  | Y |  |  | hole | 135&#8209;140 |
| 56 | `insert` | Y | Y |  |  | Y |  |  | hole | 143&#8209;148 |
| 57 | `default` |  | Y |  |  | Y |  | Y |  | 482 |
| 58 | `eq` |  | Y |  |  | Y |  |  | hole | 492&#8209;493 |

### Chap41/ArraySetEnumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 59 | `new` | Y | Y |  |  | Y |  |  | hole | 59&#8209;60 |
| 60 | `size` | Y | Y |  |  | Y |  |  | hole | 63&#8209;64 |
| 61 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 67&#8209;68 |
| 62 | `empty` | Y | Y |  |  | Y |  |  | hole | 70&#8209;71 |
| 63 | `singleton` | Y | Y |  |  | Y |  |  | hole | 74&#8209;78 |
| 64 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 80&#8209;81 |
| 65 | `filter` | Y | Y |  |  | Y |  |  | hole | 84&#8209;85 |
| 66 | `intersection` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 67 | `difference` | Y | Y |  |  | Y |  |  | hole | 92&#8209;93 |
| 68 | `union` | Y | Y |  |  | Y |  |  | hole | 96&#8209;97 |
| 69 | `find` | Y | Y |  |  | Y |  |  | hole | 100&#8209;101 |
| 70 | `delete` | Y | Y |  |  | Y |  |  | hole | 104&#8209;105 |
| 71 | `insert` | Y | Y |  |  | Y |  |  | hole | 108&#8209;109 |
| 72 | `eq` |  | Y |  |  | Y |  |  | hole | 282&#8209;283 |

### Chap41/ArraySetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 73 | `lemma_filter_remove` |  |  |  | Y | Y |  |  | unknown | 80&#8209;83 |
| 74 | `lemma_push_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 119&#8209;124 |
| 75 | `lemma_filter_in_original` |  |  |  | Y | Y |  |  | unknown | 144&#8209;146 |
| 76 | `lemma_filter_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 180&#8209;183 |
| 77 | `lemma_filter_to_set_intersect` |  |  |  | Y | Y |  |  | unknown | 214&#8209;217 |
| 78 | `lemma_filter_to_set_difference` |  |  |  | Y | Y |  |  | unknown | 249&#8209;252 |
| 79 | `lemma_subseq_no_dups_subset` |  |  |  | Y | Y |  |  | unknown | 285&#8209;291 |
| 80 | `size` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;311 |
| 81 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 313&#8209;318 |
| 82 | `empty` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;321 |
| 83 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;324 |
| 84 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;327 |
| 85 | `filter` | Y | Y |  |  | Y |  |  | hole | 329&#8209;336 |
| 86 | `intersection` | Y | Y |  |  | Y |  |  | hole | 338&#8209;347 |
| 87 | `difference` | Y | Y |  |  | Y |  |  | hole | 349&#8209;358 |
| 88 | `union` | Y | Y |  |  | Y |  |  | hole | 360&#8209;369 |
| 89 | `find` | Y | Y |  |  | Y |  |  | hole | 371&#8209;373 |
| 90 | `delete` | Y | Y |  |  | Y |  |  | hole | 375&#8209;382 |
| 91 | `insert` | Y | Y |  |  | Y |  |  | hole | 384&#8209;391 |
| 92 | `default` |  | Y |  |  | Y |  | Y |  | 1086 |
| 93 | `eq` |  | Y |  |  | Y |  |  | hole | 1098&#8209;1099 |

### Chap41/Example41_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 94 | `example_41_1_array_set` | Y | Y |  | Y | Y |  |  | unknown | 20&#8209;21 |
| 95 | `example_41_1_avl_set` | Y | Y |  | Y | Y |  |  | unknown | 24&#8209;25 |
| 96 | `demonstrate_set_operations` | Y | Y |  |  | Y |  |  | unknown | 28&#8209;29 |
| 97 | `example_41_1_array_set_impl` |  |  |  | Y | Y |  |  | hole | 33&#8209;34 |
| 98 | `example_41_1_avl_set_impl` |  |  |  | Y | Y |  |  | hole | 86&#8209;87 |
| 99 | `example_41_3_from_seq_demonstration_impl` |  |  |  | Y | Y |  |  | hole | 139&#8209;140 |
| 100 | `additional_set_operations_impl` |  |  |  | Y | Y |  |  | hole | 188&#8209;189 |
| 101 | `example_41_3_from_seq_demonstration` |  |  |  | Y | Y |  | Y |  | 246 |
| 102 | `additional_set_operations` |  |  |  | Y | Y |  | Y |  | 247 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
