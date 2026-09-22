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
| 1 | Chap49 | MinEditDistMtEph | 11 | 12 | 0 | 3 | 12 | 3 | 11 | 0 | 4 |
| 2 | Chap49 | MinEditDistMtPer | 6 | 7 | 0 | 3 | 9 | 1 | 8 | 0 | 2 |
| 3 | Chap49 | MinEditDistStEph | 11 | 11 | 0 | 1 | 10 | 2 | 9 | 0 | 3 |
| 4 | Chap49 | MinEditDistStPer | 6 | 7 | 0 | 1 | 7 | 1 | 6 | 0 | 2 |
| 5 | Chap49 | SubsetSumMtEph | 8 | 9 | 0 | 3 | 10 | 2 | 9 | 0 | 3 |
| 6 | Chap49 | SubsetSumMtPer | 5 | 6 | 0 | 3 | 8 | 1 | 7 | 0 | 2 |
| 7 | Chap49 | SubsetSumStEph | 8 | 8 | 0 | 1 | 8 | 1 | 7 | 0 | 2 |
| 8 | Chap49 | SubsetSumStPer | 5 | 6 | 0 | 1 | 6 | 1 | 4 | 0 | 3 |

## Function-by-Function Detail

### Chap49/MinEditDistMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;110 |
| 2 | `from_sequences` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 3 | `min_edit_distance` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;129 |
| 4 | `source` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 5 | `target` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;139 |
| 6 | `set_source` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;149 |
| 7 | `set_target` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;159 |
| 8 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;167 |
| 9 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 171 |
| 10 | `new_arc_memo` |  |  |  | Y | Y |  |  | unknown | 179&#8209;183 |
| 11 | `clone_arc_memo` |  |  |  | Y | Y |  |  | unknown | 190&#8209;194 |
| 12 | `min_edit_distance_rec` |  |  |  | Y | Y |  |  | unknown | 202&#8209;216 |
| 13 | `source_mut` | Y | Y |  |  |  | Y | Y |  | 429&#8209;431 |
| 14 | `target_mut` | Y | Y |  |  |  | Y | Y |  | 433&#8209;435 |
| 15 | `eq` |  | Y |  |  |  | Y | Y |  | 460 |

### Chap49/MinEditDistMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `new` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;108 |
| 17 | `from_sequences` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;116 |
| 18 | `min_edit_distance` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;124 |
| 19 | `source` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 20 | `target` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 21 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 138 |
| 22 | `new_arc_memo` |  |  |  | Y | Y |  |  | unknown | 146&#8209;150 |
| 23 | `clone_arc_memo` |  |  |  | Y | Y |  |  | unknown | 157&#8209;161 |
| 24 | `min_edit_distance_rec` |  |  |  | Y | Y |  |  | unknown | 169&#8209;183 |
| 25 | `eq` |  | Y |  |  |  | Y | Y |  | 382 |

### Chap49/MinEditDistStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 26 | `new` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;102 |
| 27 | `from_sequences` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;109 |
| 28 | `min_edit_distance` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 29 | `source` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 30 | `target` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;128 |
| 31 | `set_source` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;136 |
| 32 | `set_target` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;144 |
| 33 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;151 |
| 34 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 155 |
| 35 | `min_edit_distance_rec` |  |  |  | Y | Y |  |  | unknown | 164&#8209;179 |
| 36 | `source_mut` | Y | Y |  |  |  | Y | Y |  | 300&#8209;302 |
| 37 | `target_mut` | Y | Y |  |  |  | Y | Y |  | 304&#8209;306 |

### Chap49/MinEditDistStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `new` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;102 |
| 39 | `from_sequences` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;109 |
| 40 | `min_edit_distance` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 41 | `source` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 42 | `target` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 43 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 129 |
| 44 | `min_edit_distance_rec` |  |  |  | Y | Y |  |  | unknown | 138&#8209;153 |
| 45 | `eq` |  | Y |  |  |  | Y | Y |  | 262&#8209;266 |

### Chap49/SubsetSumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 46 | `new` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;102 |
| 47 | `from_multiset` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;109 |
| 48 | `subset_sum` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;117 |
| 49 | `multiset` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;122 |
| 50 | `set` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;130 |
| 51 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;136 |
| 52 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 140 |
| 53 | `new_arc_memo` |  |  |  | Y | Y |  |  | unknown | 148&#8209;151 |
| 54 | `clone_arc_memo` |  |  |  | Y | Y |  |  | unknown | 158&#8209;162 |
| 55 | `subset_sum_rec` |  |  |  | Y | Y |  |  | unknown | 170&#8209;180 |
| 56 | `multiset_mut` | Y | Y |  |  |  | Y | Y |  | 362&#8209;364 |
| 57 | `eq` |  | Y |  |  |  | Y | Y |  | 388 |

### Chap49/SubsetSumMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 58 | `new` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;100 |
| 59 | `from_multiset` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;107 |
| 60 | `subset_sum` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;114 |
| 61 | `multiset` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;119 |
| 62 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 123 |
| 63 | `new_arc_memo` |  |  |  | Y | Y |  |  | unknown | 131&#8209;134 |
| 64 | `clone_arc_memo` |  |  |  | Y | Y |  |  | unknown | 141&#8209;145 |
| 65 | `subset_sum_rec` |  |  |  | Y | Y |  |  | unknown | 153&#8209;163 |
| 66 | `eq` |  | Y |  |  |  | Y | Y |  | 341 |

### Chap49/SubsetSumStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 67 | `new` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 68 | `from_multiset` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;97 |
| 69 | `subset_sum` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;105 |
| 70 | `multiset` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 71 | `set` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 72 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 73 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 125 |
| 74 | `subset_sum_rec` |  |  |  | Y | Y |  |  | unknown | 134&#8209;143 |
| 75 | `multiset_mut` | Y | Y |  |  |  | Y | Y |  | 246&#8209;248 |

### Chap49/SubsetSumStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 76 | `new` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 77 | `from_multiset` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;97 |
| 78 | `subset_sum` | Y | Y |  |  | Y |  | Y |  | 102&#8209;104 |
| 79 | `multiset` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 80 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 113 |
| 81 | `subset_sum_rec` |  |  |  | Y | Y |  |  | unknown | 122&#8209;131 |
| 82 | `eq` |  | Y |  |  |  | Y | Y |  | 227&#8209;229 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
