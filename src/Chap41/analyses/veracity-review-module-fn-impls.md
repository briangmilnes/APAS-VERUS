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
| 1 | Chap41 | AVLTreeSetMtEph | 13 | 15 | 0 | 0 | 15 | 0 | 7 | 7 | 1 |
| 2 | Chap41 | AVLTreeSetMtPer | 12 | 16 | 0 | 0 | 16 | 0 | 4 | 10 | 2 |
| 3 | Chap41 | AVLTreeSetStEph | 12 | 14 | 0 | 1 | 15 | 0 | 12 | 2 | 1 |
| 4 | Chap41 | AVLTreeSetStPer | 12 | 14 | 0 | 0 | 14 | 0 | 12 | 1 | 1 |
| 5 | Chap41 | ArraySetEnumMtEph | 13 | 14 | 0 | 7 | 21 | 0 | 20 | 1 | 0 |
| 6 | Chap41 | ArraySetStEph | 12 | 14 | 0 | 7 | 21 | 0 | 19 | 1 | 1 |
| 7 | Chap41 | Example41_3 | 3 | 3 | 0 | 8 | 9 | 0 | 5 | 4 | 0 |

## Function-by-Function Detail

### Chap41/AVLTreeSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `size` | Y | Y |  |  | Y |  |  | hole | 116&#8209;118 |
| 2 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 121&#8209;126 |
| 3 | `empty` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;132 |
| 4 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;139 |
| 5 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;145 |
| 6 | `filter` | Y | Y |  |  | Y |  |  | hole | 148&#8209;153 |
| 7 | `intersection` | Y | Y |  |  | Y |  |  | hole | 156&#8209;161 |
| 8 | `difference` | Y | Y |  |  | Y |  |  | hole | 164&#8209;169 |
| 9 | `union` | Y | Y |  |  | Y |  |  | hole | 172&#8209;177 |
| 10 | `find` | Y | Y |  |  | Y |  |  | hole | 180&#8209;182 |
| 11 | `delete` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;190 |
| 12 | `insert` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;198 |
| 13 | `iter` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;201 |
| 14 | `default` |  | Y |  |  | Y |  | Y |  | 494 |
| 15 | `next` |  | Y |  |  | Y |  |  | unknown | 502&#8209;518 |

### Chap41/AVLTreeSetMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `size` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 17 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;101 |
| 18 | `empty` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 19 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 20 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 111&#8209;112 |
| 21 | `filter` | Y | Y |  |  | Y |  |  | hole | 115&#8209;116 |
| 22 | `intersection` | Y | Y |  |  | Y |  |  | hole | 119&#8209;120 |
| 23 | `difference` | Y | Y |  |  | Y |  |  | hole | 123&#8209;124 |
| 24 | `union` | Y | Y |  |  | Y |  |  | hole | 127&#8209;128 |
| 25 | `find` | Y | Y |  |  | Y |  |  | hole | 131&#8209;133 |
| 26 | `delete` | Y | Y |  |  | Y |  |  | hole | 136&#8209;137 |
| 27 | `insert` | Y | Y |  |  | Y |  |  | hole | 140&#8209;141 |
| 28 | `default` |  | Y |  |  | Y |  | Y |  | 502 |
| 29 | `eq` |  | Y |  |  | Y |  |  | hole | 514&#8209;515 |
| 30 | `partial_cmp` |  | Y |  |  | Y |  | Y |  | 547 |
| 31 | `cmp` |  | Y |  |  | Y |  |  | hole | 555 |

### Chap41/AVLTreeSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 32 | `lemma_wf_implies_len_bound` |  |  |  | Y | Y |  |  | unknown | 67&#8209;72 |
| 33 | `size` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 34 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;103 |
| 35 | `empty` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;109 |
| 36 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;115 |
| 37 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;120 |
| 38 | `filter` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;129 |
| 39 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;136 |
| 40 | `difference` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;143 |
| 41 | `union` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;150 |
| 42 | `find` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;155 |
| 43 | `delete` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;162 |
| 44 | `insert` | Y | Y |  |  | Y |  |  | hole | 165&#8209;169 |
| 45 | `default` |  | Y |  |  | Y |  | Y |  | 927 |
| 46 | `eq` |  | Y |  |  | Y |  |  | hole | 939&#8209;940 |

### Chap41/AVLTreeSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 47 | `size` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 48 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 49 | `empty` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;79 |
| 50 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;85 |
| 51 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;90 |
| 52 | `filter` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;99 |
| 53 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;108 |
| 54 | `difference` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;117 |
| 55 | `union` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;126 |
| 56 | `find` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 57 | `delete` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 58 | `insert` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;145 |
| 59 | `default` |  | Y |  |  | Y |  | Y |  | 884 |
| 60 | `eq` |  | Y |  |  | Y |  |  | hole | 894&#8209;895 |

### Chap41/ArraySetEnumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 61 | `zero_bit_false` |  |  |  | Y | Y |  |  | unknown | 102&#8209;104 |
| 62 | `set_bit64_proof` |  |  |  | Y | Y |  |  | unknown | 108&#8209;115 |
| 63 | `bit_or_64_proof` |  |  |  | Y | Y |  |  | unknown | 119&#8209;122 |
| 64 | `bit_and_64_proof` |  |  |  | Y | Y |  |  | unknown | 126&#8209;129 |
| 65 | `bit_andnot_64_proof` |  |  |  | Y | Y |  |  | unknown | 133&#8209;136 |
| 66 | `lemma_bounded_usize_set_finite` |  |  |  | Y | Y |  |  | unknown | 140&#8209;142 |
| 67 | `lemma_view_finite` |  |  |  | Y | Y |  |  | unknown | 154&#8209;159 |
| 68 | `new` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;182 |
| 69 | `size` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;187 |
| 70 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;195 |
| 71 | `empty` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;202 |
| 72 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;211 |
| 73 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;218 |
| 74 | `filter` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;229 |
| 75 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;241 |
| 76 | `difference` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;253 |
| 77 | `union` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;265 |
| 78 | `find` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;270 |
| 79 | `delete` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;279 |
| 80 | `insert` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;289 |
| 81 | `eq` |  | Y |  |  | Y |  |  | hole | 936&#8209;937 |

### Chap41/ArraySetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 82 | `lemma_filter_remove` |  |  |  | Y | Y |  |  | unknown | 80&#8209;83 |
| 83 | `lemma_push_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 119&#8209;124 |
| 84 | `lemma_filter_in_original` |  |  |  | Y | Y |  |  | unknown | 144&#8209;146 |
| 85 | `lemma_filter_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 180&#8209;183 |
| 86 | `lemma_filter_to_set_intersect` |  |  |  | Y | Y |  |  | unknown | 214&#8209;217 |
| 87 | `lemma_filter_to_set_difference` |  |  |  | Y | Y |  |  | unknown | 249&#8209;252 |
| 88 | `lemma_subseq_no_dups_subset` |  |  |  | Y | Y |  |  | unknown | 285&#8209;291 |
| 89 | `size` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;311 |
| 90 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;318 |
| 91 | `empty` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;321 |
| 92 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;324 |
| 93 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;327 |
| 94 | `filter` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;337 |
| 95 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 339&#8209;348 |
| 96 | `difference` | Y | Y |  |  | Y |  |  | unknown | 350&#8209;359 |
| 97 | `union` | Y | Y |  |  | Y |  |  | unknown | 361&#8209;370 |
| 98 | `find` | Y | Y |  |  | Y |  |  | unknown | 372&#8209;374 |
| 99 | `delete` | Y | Y |  |  | Y |  |  | unknown | 376&#8209;383 |
| 100 | `insert` | Y | Y |  |  | Y |  |  | unknown | 385&#8209;392 |
| 101 | `default` |  | Y |  |  | Y |  | Y |  | 1088 |
| 102 | `eq` |  | Y |  |  | Y |  |  | hole | 1100&#8209;1101 |

### Chap41/Example41_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 103 | `example_41_1_array_set` | Y | Y |  | Y | Y |  |  | unknown | 20&#8209;21 |
| 104 | `example_41_1_avl_set` | Y | Y |  | Y | Y |  |  | unknown | 24&#8209;25 |
| 105 | `demonstrate_set_operations` | Y | Y |  |  | Y |  |  | unknown | 28&#8209;29 |
| 106 | `example_41_1_array_set_impl` |  |  |  | Y | Y |  |  | hole | 33&#8209;34 |
| 107 | `example_41_1_avl_set_impl` |  |  |  | Y | Y |  |  | hole | 86&#8209;87 |
| 108 | `example_41_3_from_seq_demonstration_impl` |  |  |  | Y | Y |  |  | hole | 139&#8209;140 |
| 109 | `additional_set_operations_impl` |  |  |  | Y | Y |  |  | hole | 188&#8209;189 |
| 110 | `example_41_3_from_seq_demonstration` |  |  |  | Y | Y |  |  | unknown | 250&#8209;251 |
| 111 | `additional_set_operations` |  |  |  | Y | Y |  |  | unknown | 253&#8209;254 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
