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
| 1 | Chap41 | AVLTreeSetMtEph | 13 | 15 | 0 | 0 | 15 | 0 | 0 | 14 | 1 |
| 2 | Chap41 | AVLTreeSetMtPer | 12 | 16 | 0 | 0 | 16 | 0 | 3 | 10 | 3 |
| 3 | Chap41 | AVLTreeSetStEph | 12 | 14 | 0 | 0 | 14 | 0 | 1 | 12 | 1 |
| 4 | Chap41 | AVLTreeSetStPer | 12 | 14 | 0 | 0 | 14 | 0 | 4 | 9 | 1 |
| 5 | Chap41 | ArraySetEnumMtEph | 13 | 14 | 0 | 6 | 20 | 0 | 14 | 6 | 0 |
| 6 | Chap41 | ArraySetStEph | 12 | 14 | 0 | 7 | 21 | 0 | 11 | 9 | 1 |
| 7 | Chap41 | Example41_3 | 3 | 3 | 0 | 8 | 9 | 0 | 3 | 4 | 2 |

## Function-by-Function Detail

### Chap41/AVLTreeSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `size` | Y | Y |  |  | Y |  |  | hole | 114&#8209;116 |
| 2 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 119&#8209;124 |
| 3 | `empty` | Y | Y |  |  | Y |  |  | hole | 127&#8209;130 |
| 4 | `singleton` | Y | Y |  |  | Y |  |  | hole | 133&#8209;137 |
| 5 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 139&#8209;142 |
| 6 | `filter` | Y | Y |  |  | Y |  |  | hole | 145&#8209;150 |
| 7 | `intersection` | Y | Y |  |  | Y |  |  | hole | 153&#8209;158 |
| 8 | `difference` | Y | Y |  |  | Y |  |  | hole | 161&#8209;166 |
| 9 | `union` | Y | Y |  |  | Y |  |  | hole | 169&#8209;174 |
| 10 | `find` | Y | Y |  |  | Y |  |  | hole | 177&#8209;179 |
| 11 | `delete` | Y | Y |  |  | Y |  |  | hole | 182&#8209;187 |
| 12 | `insert` | Y | Y |  |  | Y |  |  | hole | 190&#8209;195 |
| 13 | `iter` | Y | Y |  |  | Y |  |  | hole | 196&#8209;198 |
| 14 | `default` |  | Y |  |  | Y |  | Y |  | 466 |
| 15 | `next` |  | Y |  |  | Y |  |  | hole | 475&#8209;491 |

### Chap41/AVLTreeSetMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `size` | Y | Y |  |  | Y |  |  | hole | 81&#8209;82 |
| 17 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 18 | `empty` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 19 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;97 |
| 20 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 99&#8209;100 |
| 21 | `filter` | Y | Y |  |  | Y |  |  | hole | 103&#8209;104 |
| 22 | `intersection` | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 23 | `difference` | Y | Y |  |  | Y |  |  | hole | 111&#8209;112 |
| 24 | `union` | Y | Y |  |  | Y |  |  | hole | 115&#8209;116 |
| 25 | `find` | Y | Y |  |  | Y |  |  | hole | 119&#8209;120 |
| 26 | `delete` | Y | Y |  |  | Y |  |  | hole | 123&#8209;124 |
| 27 | `insert` | Y | Y |  |  | Y |  |  | hole | 127&#8209;128 |
| 28 | `default` |  | Y |  |  | Y |  | Y |  | 488 |
| 29 | `eq` |  | Y |  |  | Y |  |  | hole | 500&#8209;501 |
| 30 | `partial_cmp` |  | Y |  |  | Y |  | Y |  | 529 |
| 31 | `cmp` |  | Y |  |  | Y |  | Y |  | 536 |

### Chap41/AVLTreeSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 32 | `size` | Y | Y |  |  | Y |  |  | hole | 72&#8209;73 |
| 33 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 76&#8209;81 |
| 34 | `empty` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 35 | `singleton` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 36 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 91&#8209;92 |
| 37 | `filter` | Y | Y |  |  | Y |  |  | hole | 95&#8209;96 |
| 38 | `intersection` | Y | Y |  |  | Y |  |  | hole | 99&#8209;100 |
| 39 | `difference` | Y | Y |  |  | Y |  |  | hole | 103&#8209;104 |
| 40 | `union` | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 41 | `find` | Y | Y |  |  | Y |  |  | hole | 111&#8209;112 |
| 42 | `delete` | Y | Y |  |  | Y |  |  | hole | 115&#8209;116 |
| 43 | `insert` | Y | Y |  |  | Y |  |  | hole | 119&#8209;120 |
| 44 | `default` |  | Y |  |  | Y |  | Y |  | 450 |
| 45 | `eq` |  | Y |  |  | Y |  |  | hole | 462&#8209;463 |

### Chap41/AVLTreeSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 46 | `size` | Y | Y |  |  | Y |  |  | hole | 66&#8209;68 |
| 47 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 48 | `empty` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;81 |
| 49 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;88 |
| 50 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 51 | `filter` | Y | Y |  |  | Y |  |  | hole | 97&#8209;102 |
| 52 | `intersection` | Y | Y |  |  | Y |  |  | hole | 105&#8209;112 |
| 53 | `difference` | Y | Y |  |  | Y |  |  | hole | 115&#8209;122 |
| 54 | `union` | Y | Y |  |  | Y |  |  | hole | 125&#8209;132 |
| 55 | `find` | Y | Y |  |  | Y |  |  | hole | 135&#8209;137 |
| 56 | `delete` | Y | Y |  |  | Y |  |  | hole | 140&#8209;145 |
| 57 | `insert` | Y | Y |  |  | Y |  |  | hole | 148&#8209;153 |
| 58 | `default` |  | Y |  |  | Y |  | Y |  | 480 |
| 59 | `eq` |  | Y |  |  | Y |  |  | hole | 490&#8209;491 |

### Chap41/ArraySetEnumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 60 | `zero_bit_false` |  |  |  | Y | Y |  |  | unknown | 102&#8209;104 |
| 61 | `set_bit64_proof` |  |  |  | Y | Y |  |  | unknown | 108&#8209;115 |
| 62 | `bit_or_64_proof` |  |  |  | Y | Y |  |  | unknown | 119&#8209;122 |
| 63 | `bit_and_64_proof` |  |  |  | Y | Y |  |  | unknown | 126&#8209;129 |
| 64 | `bit_andnot_64_proof` |  |  |  | Y | Y |  |  | unknown | 133&#8209;136 |
| 65 | `lemma_view_finite` |  |  |  | Y | Y |  |  | hole | 139&#8209;144 |
| 66 | `new` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;167 |
| 67 | `size` | Y | Y |  |  | Y |  |  | hole | 170&#8209;172 |
| 68 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 175&#8209;180 |
| 69 | `empty` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;187 |
| 70 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;196 |
| 71 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 199&#8209;203 |
| 72 | `filter` | Y | Y |  |  | Y |  |  | hole | 206&#8209;212 |
| 73 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;224 |
| 74 | `difference` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;236 |
| 75 | `union` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;248 |
| 76 | `find` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;253 |
| 77 | `delete` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;262 |
| 78 | `insert` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;272 |
| 79 | `eq` |  | Y |  |  | Y |  |  | hole | 691&#8209;692 |

### Chap41/ArraySetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 80 | `lemma_filter_remove` |  |  |  | Y | Y |  |  | unknown | 80&#8209;83 |
| 81 | `lemma_push_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 119&#8209;124 |
| 82 | `lemma_filter_in_original` |  |  |  | Y | Y |  |  | unknown | 144&#8209;146 |
| 83 | `lemma_filter_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 180&#8209;183 |
| 84 | `lemma_filter_to_set_intersect` |  |  |  | Y | Y |  |  | unknown | 214&#8209;217 |
| 85 | `lemma_filter_to_set_difference` |  |  |  | Y | Y |  |  | unknown | 249&#8209;252 |
| 86 | `lemma_subseq_no_dups_subset` |  |  |  | Y | Y |  |  | unknown | 285&#8209;291 |
| 87 | `size` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;311 |
| 88 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 313&#8209;318 |
| 89 | `empty` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;321 |
| 90 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;324 |
| 91 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;327 |
| 92 | `filter` | Y | Y |  |  | Y |  |  | hole | 329&#8209;336 |
| 93 | `intersection` | Y | Y |  |  | Y |  |  | hole | 338&#8209;347 |
| 94 | `difference` | Y | Y |  |  | Y |  |  | hole | 349&#8209;358 |
| 95 | `union` | Y | Y |  |  | Y |  |  | hole | 360&#8209;369 |
| 96 | `find` | Y | Y |  |  | Y |  |  | hole | 371&#8209;373 |
| 97 | `delete` | Y | Y |  |  | Y |  |  | hole | 375&#8209;382 |
| 98 | `insert` | Y | Y |  |  | Y |  |  | hole | 384&#8209;391 |
| 99 | `default` |  | Y |  |  | Y |  | Y |  | 1086 |
| 100 | `eq` |  | Y |  |  | Y |  |  | hole | 1098&#8209;1099 |

### Chap41/Example41_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 101 | `example_41_1_array_set` | Y | Y |  | Y | Y |  |  | unknown | 20&#8209;21 |
| 102 | `example_41_1_avl_set` | Y | Y |  | Y | Y |  |  | unknown | 24&#8209;25 |
| 103 | `demonstrate_set_operations` | Y | Y |  |  | Y |  |  | unknown | 28&#8209;29 |
| 104 | `example_41_1_array_set_impl` |  |  |  | Y | Y |  |  | hole | 33&#8209;34 |
| 105 | `example_41_1_avl_set_impl` |  |  |  | Y | Y |  |  | hole | 86&#8209;87 |
| 106 | `example_41_3_from_seq_demonstration_impl` |  |  |  | Y | Y |  |  | hole | 139&#8209;140 |
| 107 | `additional_set_operations_impl` |  |  |  | Y | Y |  |  | hole | 188&#8209;189 |
| 108 | `example_41_3_from_seq_demonstration` |  |  |  | Y | Y |  | Y |  | 246 |
| 109 | `additional_set_operations` |  |  |  | Y | Y |  | Y |  | 247 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
