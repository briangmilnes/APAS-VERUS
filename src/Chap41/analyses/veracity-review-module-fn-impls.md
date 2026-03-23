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
| 1 | Chap41 | AVLTreeSetMtEph | 13 | 15 | 0 | 0 | 15 | 0 | 5 | 9 | 1 |
| 2 | Chap41 | AVLTreeSetMtPer | 12 | 16 | 0 | 0 | 16 | 0 | 3 | 11 | 2 |
| 3 | Chap41 | AVLTreeSetStEph | 18 | 20 | 0 | 5 | 25 | 0 | 23 | 1 | 1 |
| 4 | Chap41 | AVLTreeSetStPer | 12 | 14 | 0 | 0 | 14 | 0 | 12 | 1 | 1 |
| 5 | Chap41 | ArraySetEnumMtEph | 13 | 14 | 0 | 7 | 21 | 0 | 20 | 1 | 0 |
| 6 | Chap41 | ArraySetStEph | 12 | 14 | 0 | 7 | 21 | 0 | 19 | 1 | 1 |
| 7 | Chap41 | Example41_3 | 3 | 3 | 0 | 8 | 9 | 0 | 0 | 0 | 9 |

## Function-by-Function Detail

### Chap41/AVLTreeSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `size` | Y | Y |  |  | Y |  |  | hole | 114&#8209;116 |
| 2 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 119&#8209;124 |
| 3 | `empty` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;130 |
| 4 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;137 |
| 5 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;143 |
| 6 | `filter` | Y | Y |  |  | Y |  |  | hole | 146&#8209;163 |
| 7 | `intersection` | Y | Y |  |  | Y |  |  | hole | 166&#8209;171 |
| 8 | `difference` | Y | Y |  |  | Y |  |  | hole | 174&#8209;179 |
| 9 | `union` | Y | Y |  |  | Y |  |  | hole | 182&#8209;190 |
| 10 | `find` | Y | Y |  |  | Y |  |  | hole | 193&#8209;195 |
| 11 | `delete` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;203 |
| 12 | `insert` | Y | Y |  |  | Y |  |  | hole | 206&#8209;213 |
| 13 | `iter` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;216 |
| 14 | `default` |  | Y |  |  | Y |  | Y |  | 452 |
| 15 | `next` |  | Y |  |  | Y |  |  | hole | 460&#8209;476 |

### Chap41/AVLTreeSetMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `size` | Y | Y |  |  | Y |  |  | hole | 93&#8209;95 |
| 17 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 98&#8209;104 |
| 18 | `empty` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;108 |
| 19 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 20 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 21 | `filter` | Y | Y |  |  | Y |  |  | hole | 118&#8209;134 |
| 22 | `intersection` | Y | Y |  |  | Y |  |  | hole | 137&#8209;138 |
| 23 | `difference` | Y | Y |  |  | Y |  |  | hole | 141&#8209;142 |
| 24 | `union` | Y | Y |  |  | Y |  |  | hole | 145&#8209;146 |
| 25 | `find` | Y | Y |  |  | Y |  |  | hole | 149&#8209;151 |
| 26 | `delete` | Y | Y |  |  | Y |  |  | hole | 154&#8209;155 |
| 27 | `insert` | Y | Y |  |  | Y |  |  | hole | 158&#8209;159 |
| 28 | `default` |  | Y |  |  | Y |  | Y |  | 382 |
| 29 | `eq` |  | Y |  |  | Y |  |  | hole | 394&#8209;395 |
| 30 | `partial_cmp` |  | Y |  |  | Y |  | Y |  | 408 |
| 31 | `cmp` |  | Y |  |  | Y |  |  | hole | 416 |

### Chap41/AVLTreeSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 32 | `lemma_wf_implies_len_bound` |  |  |  | Y | Y |  |  | unknown | 89&#8209;94 |
| 33 | `lemma_inorder_values_maps_to_views` |  |  |  | Y | Y |  |  | unknown | 107&#8209;109 |
| 34 | `lemma_empty_set_is_sorted` |  |  |  | Y | Y |  |  | unknown | 129&#8209;134 |
| 35 | `lemma_push_sorted` |  |  |  | Y | Y |  |  | unknown | 152&#8209;157 |
| 36 | `lemma_subseq_sorted` |  |  |  | Y | Y |  |  | unknown | 184&#8209;189 |
| 37 | `size` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;209 |
| 38 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;217 |
| 39 | `empty` | Y | Y |  |  | Y |  |  | unknown | 220&#8209;223 |
| 40 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;229 |
| 41 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;235 |
| 42 | `filter` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;254 |
| 43 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;261 |
| 44 | `difference` | Y | Y |  |  | Y |  |  | unknown | 264&#8209;268 |
| 45 | `union` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;278 |
| 46 | `find` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;283 |
| 47 | `delete` | Y | Y |  |  | Y |  |  | unknown | 286&#8209;290 |
| 48 | `insert` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;299 |
| 49 | `insert_sorted` | Y | Y |  |  | Y |  |  | unknown | 308&#8209;317 |
| 50 | `delete_sorted` | Y | Y |  |  | Y |  |  | unknown | 319&#8209;327 |
| 51 | `filter_sorted` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;348 |
| 52 | `intersection_sorted` | Y | Y |  |  | Y |  |  | unknown | 350&#8209;358 |
| 53 | `difference_sorted` | Y | Y |  |  | Y |  |  | unknown | 360&#8209;368 |
| 54 | `union_sorted` | Y | Y |  |  | Y |  |  | unknown | 370&#8209;379 |
| 55 | `default` |  | Y |  |  | Y |  | Y |  | 2116 |
| 56 | `eq` |  | Y |  |  | Y |  |  | hole | 2128&#8209;2129 |

### Chap41/AVLTreeSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 57 | `size` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 58 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 59 | `empty` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;79 |
| 60 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;85 |
| 61 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;91 |
| 62 | `filter` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;110 |
| 63 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;119 |
| 64 | `difference` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;128 |
| 65 | `union` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;138 |
| 66 | `find` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;143 |
| 67 | `delete` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;150 |
| 68 | `insert` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;159 |
| 69 | `default` |  | Y |  |  | Y |  | Y |  | 1081 |
| 70 | `eq` |  | Y |  |  | Y |  |  | hole | 1091&#8209;1092 |

### Chap41/ArraySetEnumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 71 | `zero_bit_false` |  |  |  | Y | Y |  |  | unknown | 102&#8209;104 |
| 72 | `set_bit64_proof` |  |  |  | Y | Y |  |  | unknown | 108&#8209;115 |
| 73 | `bit_or_64_proof` |  |  |  | Y | Y |  |  | unknown | 119&#8209;122 |
| 74 | `bit_and_64_proof` |  |  |  | Y | Y |  |  | unknown | 126&#8209;129 |
| 75 | `bit_andnot_64_proof` |  |  |  | Y | Y |  |  | unknown | 133&#8209;136 |
| 76 | `lemma_bounded_usize_set_finite` |  |  |  | Y | Y |  |  | unknown | 140&#8209;142 |
| 77 | `lemma_view_finite` |  |  |  | Y | Y |  |  | unknown | 154&#8209;159 |
| 78 | `new` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;182 |
| 79 | `size` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;188 |
| 80 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;197 |
| 81 | `empty` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;204 |
| 82 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;214 |
| 83 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;221 |
| 84 | `filter` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;233 |
| 85 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;246 |
| 86 | `difference` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;259 |
| 87 | `union` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;272 |
| 88 | `find` | Y | Y |  |  | Y |  |  | unknown | 276&#8209;278 |
| 89 | `delete` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;288 |
| 90 | `insert` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;299 |
| 91 | `eq` |  | Y |  |  | Y |  |  | hole | 946&#8209;947 |

### Chap41/ArraySetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 92 | `lemma_filter_remove` |  |  |  | Y | Y |  |  | unknown | 80&#8209;83 |
| 93 | `lemma_push_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 119&#8209;124 |
| 94 | `lemma_filter_in_original` |  |  |  | Y | Y |  |  | unknown | 144&#8209;146 |
| 95 | `lemma_filter_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 180&#8209;183 |
| 96 | `lemma_filter_to_set_intersect` |  |  |  | Y | Y |  |  | unknown | 214&#8209;217 |
| 97 | `lemma_filter_to_set_difference` |  |  |  | Y | Y |  |  | unknown | 249&#8209;252 |
| 98 | `lemma_subseq_no_dups_subset` |  |  |  | Y | Y |  |  | unknown | 285&#8209;291 |
| 99 | `size` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;313 |
| 100 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 317&#8209;322 |
| 101 | `empty` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;327 |
| 102 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;332 |
| 103 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 336&#8209;340 |
| 104 | `filter` | Y | Y |  |  | Y |  |  | unknown | 344&#8209;362 |
| 105 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 366&#8209;375 |
| 106 | `difference` | Y | Y |  |  | Y |  |  | unknown | 379&#8209;388 |
| 107 | `union` | Y | Y |  |  | Y |  |  | unknown | 392&#8209;401 |
| 108 | `find` | Y | Y |  |  | Y |  |  | unknown | 405&#8209;407 |
| 109 | `delete` | Y | Y |  |  | Y |  |  | unknown | 411&#8209;418 |
| 110 | `insert` | Y | Y |  |  | Y |  |  | unknown | 422&#8209;429 |
| 111 | `default` |  | Y |  |  | Y |  | Y |  | 1228 |
| 112 | `eq` |  | Y |  |  | Y |  |  | hole | 1240&#8209;1241 |

### Chap41/Example41_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 113 | `example_41_1_array_set` | Y | Y |  | Y | Y |  | Y |  | 18 |
| 114 | `example_41_1_avl_set` | Y | Y |  | Y | Y |  | Y |  | 21 |
| 115 | `demonstrate_set_operations` | Y | Y |  |  | Y |  | Y |  | 24 |
| 116 | `example_41_1_array_set_impl` |  |  |  | Y | Y |  | Y |  | 27 |
| 117 | `example_41_1_avl_set_impl` |  |  |  | Y | Y |  | Y |  | 70 |
| 118 | `example_41_3_from_seq_demonstration_impl` |  |  |  | Y | Y |  | Y |  | 113 |
| 119 | `additional_set_operations_impl` |  |  |  | Y | Y |  | Y |  | 141 |
| 120 | `example_41_3_from_seq_demonstration` |  |  |  | Y | Y |  | Y |  | 191 |
| 121 | `additional_set_operations` |  |  |  | Y | Y |  | Y |  | 193 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
