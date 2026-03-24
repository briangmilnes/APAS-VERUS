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
| 3 | Chap41 | AVLTreeSetStEph | 25 | 27 | 0 | 5 | 32 | 0 | 30 | 1 | 1 |
| 4 | Chap41 | AVLTreeSetStPer | 20 | 22 | 0 | 4 | 26 | 0 | 24 | 1 | 1 |
| 5 | Chap41 | ArraySetEnumMtEph | 13 | 14 | 0 | 7 | 21 | 0 | 20 | 1 | 0 |
| 6 | Chap41 | ArraySetStEph | 12 | 14 | 0 | 7 | 21 | 0 | 19 | 1 | 1 |
| 7 | Chap41 | Example41_3 | 3 | 3 | 0 | 8 | 9 | 0 | 0 | 0 | 9 |

## Function-by-Function Detail

### Chap41/AVLTreeSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `size` | Y | Y |  |  | Y |  |  | hole | 115&#8209;117 |
| 2 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 120&#8209;125 |
| 3 | `empty` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;131 |
| 4 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 5 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;144 |
| 6 | `filter` | Y | Y |  |  | Y |  |  | hole | 147&#8209;164 |
| 7 | `intersection` | Y | Y |  |  | Y |  |  | hole | 167&#8209;172 |
| 8 | `difference` | Y | Y |  |  | Y |  |  | hole | 175&#8209;180 |
| 9 | `union` | Y | Y |  |  | Y |  |  | hole | 183&#8209;191 |
| 10 | `find` | Y | Y |  |  | Y |  |  | hole | 194&#8209;196 |
| 11 | `delete` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;204 |
| 12 | `insert` | Y | Y |  |  | Y |  |  | hole | 207&#8209;214 |
| 13 | `iter` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;217 |
| 14 | `default` |  | Y |  |  | Y |  | Y |  | 453 |
| 15 | `next` |  | Y |  |  | Y |  |  | hole | 461&#8209;477 |

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
| 32 | `lemma_wf_implies_len_bound` |  |  |  | Y | Y |  |  | unknown | 96&#8209;101 |
| 33 | `lemma_inorder_values_maps_to_views` |  |  |  | Y | Y |  |  | unknown | 114&#8209;116 |
| 34 | `lemma_empty_set_is_sorted` |  |  |  | Y | Y |  |  | unknown | 134&#8209;139 |
| 35 | `lemma_push_sorted` |  |  |  | Y | Y |  |  | unknown | 143&#8209;148 |
| 36 | `lemma_subseq_sorted` |  |  |  | Y | Y |  |  | unknown | 170&#8209;175 |
| 37 | `size` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;194 |
| 38 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;202 |
| 39 | `empty` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;208 |
| 40 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;214 |
| 41 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;220 |
| 42 | `filter` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;238 |
| 43 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;244 |
| 44 | `difference` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;250 |
| 45 | `union` | Y | Y |  |  | Y |  |  | unknown | 252&#8209;259 |
| 46 | `find` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;263 |
| 47 | `delete` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;269 |
| 48 | `insert` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;277 |
| 49 | `find_iter` | Y | Y |  |  | Y |  |  | unknown | 279&#8209;281 |
| 50 | `insert_iter` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;289 |
| 51 | `delete_iter` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;295 |
| 52 | `filter_iter` | Y | Y |  |  | Y |  |  | unknown | 297&#8209;313 |
| 53 | `intersection_iter` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;319 |
| 54 | `union_iter` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;328 |
| 55 | `difference_iter` | Y | Y |  |  | Y |  |  | unknown | 330&#8209;334 |
| 56 | `insert_sorted` | Y | Y |  |  | Y |  |  | unknown | 343&#8209;351 |
| 57 | `delete_sorted` | Y | Y |  |  | Y |  |  | unknown | 353&#8209;360 |
| 58 | `filter_sorted` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;380 |
| 59 | `intersection_sorted` | Y | Y |  |  | Y |  |  | unknown | 382&#8209;390 |
| 60 | `difference_sorted` | Y | Y |  |  | Y |  |  | unknown | 392&#8209;400 |
| 61 | `union_sorted` | Y | Y |  |  | Y |  |  | unknown | 402&#8209;411 |
| 62 | `default` |  | Y |  |  | Y |  | Y |  | 697 |
| 63 | `eq` |  | Y |  |  | Y |  |  | hole | 709&#8209;710 |

### Chap41/AVLTreeSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 64 | `lemma_inorder_values_maps_to_views_per` |  |  |  | Y | Y |  |  | unknown | 82&#8209;84 |
| 65 | `lemma_push_sorted_per` |  |  |  | Y | Y |  |  | unknown | 104&#8209;109 |
| 66 | `lemma_map_view_feq_implies_ext_eq_per` |  |  |  | Y | Y |  |  | unknown | 132&#8209;137 |
| 67 | `lemma_subseq_sorted_per` |  |  |  | Y | Y |  |  | unknown | 153&#8209;158 |
| 68 | `size` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;177 |
| 69 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;183 |
| 70 | `empty` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;189 |
| 71 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;195 |
| 72 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;201 |
| 73 | `filter` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;220 |
| 74 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;229 |
| 75 | `difference` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;238 |
| 76 | `union` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;248 |
| 77 | `find` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;253 |
| 78 | `delete` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;260 |
| 79 | `insert` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;269 |
| 80 | `find_iter` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;273 |
| 81 | `insert_iter` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;281 |
| 82 | `delete_iter` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;287 |
| 83 | `filter_iter` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;305 |
| 84 | `intersection_iter` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;313 |
| 85 | `union_iter` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;322 |
| 86 | `difference_iter` | Y | Y |  |  | Y |  |  | unknown | 324&#8209;330 |
| 87 | `insert_sorted_per` | Y | Y |  |  | Y |  |  | unknown | 339&#8209;347 |
| 88 | `default` |  | Y |  |  | Y |  | Y |  | 1656 |
| 89 | `eq` |  | Y |  |  | Y |  |  | hole | 1666&#8209;1667 |

### Chap41/ArraySetEnumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 90 | `zero_bit_false` |  |  |  | Y | Y |  |  | unknown | 102&#8209;104 |
| 91 | `set_bit64_proof` |  |  |  | Y | Y |  |  | unknown | 108&#8209;115 |
| 92 | `bit_or_64_proof` |  |  |  | Y | Y |  |  | unknown | 119&#8209;122 |
| 93 | `bit_and_64_proof` |  |  |  | Y | Y |  |  | unknown | 126&#8209;129 |
| 94 | `bit_andnot_64_proof` |  |  |  | Y | Y |  |  | unknown | 133&#8209;136 |
| 95 | `lemma_bounded_usize_set_finite` |  |  |  | Y | Y |  |  | unknown | 140&#8209;142 |
| 96 | `lemma_view_finite` |  |  |  | Y | Y |  |  | unknown | 154&#8209;159 |
| 97 | `new` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;182 |
| 98 | `size` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;188 |
| 99 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;197 |
| 100 | `empty` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;204 |
| 101 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;214 |
| 102 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;221 |
| 103 | `filter` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;233 |
| 104 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;246 |
| 105 | `difference` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;259 |
| 106 | `union` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;272 |
| 107 | `find` | Y | Y |  |  | Y |  |  | unknown | 276&#8209;278 |
| 108 | `delete` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;288 |
| 109 | `insert` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;299 |
| 110 | `eq` |  | Y |  |  | Y |  |  | hole | 946&#8209;947 |

### Chap41/ArraySetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 111 | `lemma_filter_remove` |  |  |  | Y | Y |  |  | unknown | 77&#8209;80 |
| 112 | `lemma_push_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 116&#8209;121 |
| 113 | `lemma_filter_in_original` |  |  |  | Y | Y |  |  | unknown | 141&#8209;143 |
| 114 | `lemma_filter_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 177&#8209;180 |
| 115 | `lemma_filter_to_set_intersect` |  |  |  | Y | Y |  |  | unknown | 211&#8209;214 |
| 116 | `lemma_filter_to_set_difference` |  |  |  | Y | Y |  |  | unknown | 246&#8209;249 |
| 117 | `lemma_subseq_no_dups_subset` |  |  |  | Y | Y |  |  | unknown | 282&#8209;288 |
| 118 | `size` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;309 |
| 119 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;318 |
| 120 | `empty` | Y | Y |  |  | Y |  |  | unknown | 322&#8209;323 |
| 121 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 327&#8209;328 |
| 122 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 332&#8209;336 |
| 123 | `filter` | Y | Y |  |  | Y |  |  | unknown | 340&#8209;358 |
| 124 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;371 |
| 125 | `difference` | Y | Y |  |  | Y |  |  | unknown | 375&#8209;384 |
| 126 | `union` | Y | Y |  |  | Y |  |  | unknown | 388&#8209;397 |
| 127 | `find` | Y | Y |  |  | Y |  |  | unknown | 401&#8209;403 |
| 128 | `delete` | Y | Y |  |  | Y |  |  | unknown | 407&#8209;414 |
| 129 | `insert` | Y | Y |  |  | Y |  |  | unknown | 418&#8209;425 |
| 130 | `default` |  | Y |  |  | Y |  | Y |  | 1219 |
| 131 | `eq` |  | Y |  |  | Y |  |  | hole | 1231&#8209;1232 |

### Chap41/Example41_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 132 | `example_41_1_array_set` | Y | Y |  | Y | Y |  | Y |  | 18 |
| 133 | `example_41_1_avl_set` | Y | Y |  | Y | Y |  | Y |  | 21 |
| 134 | `demonstrate_set_operations` | Y | Y |  |  | Y |  | Y |  | 24 |
| 135 | `example_41_1_array_set_impl` |  |  |  | Y | Y |  | Y |  | 27 |
| 136 | `example_41_1_avl_set_impl` |  |  |  | Y | Y |  | Y |  | 70 |
| 137 | `example_41_3_from_seq_demonstration_impl` |  |  |  | Y | Y |  | Y |  | 113 |
| 138 | `additional_set_operations_impl` |  |  |  | Y | Y |  | Y |  | 141 |
| 139 | `example_41_3_from_seq_demonstration` |  |  |  | Y | Y |  | Y |  | 191 |
| 140 | `additional_set_operations` |  |  |  | Y | Y |  | Y |  | 193 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
