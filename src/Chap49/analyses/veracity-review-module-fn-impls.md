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
| 6 | Chap49 | SubsetSumMtPer | 5 | 6 | 0 | 3 | 8 | 1 | 6 | 0 | 3 |
| 7 | Chap49 | SubsetSumStEph | 8 | 8 | 0 | 1 | 8 | 1 | 7 | 0 | 2 |
| 8 | Chap49 | SubsetSumStPer | 5 | 6 | 0 | 1 | 6 | 1 | 4 | 0 | 3 |

## Function-by-Function Detail

### Chap49/MinEditDistMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;96 |
| 2 | `from_sequences` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;103 |
| 3 | `min_edit_distance` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;113 |
| 4 | `source` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 5 | `target` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 6 | `set_source` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;131 |
| 7 | `set_target` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;139 |
| 8 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;146 |
| 9 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 150 |
| 10 | `new_arc_memo` |  |  |  | Y | Y |  |  | unknown | 156&#8209;160 |
| 11 | `clone_arc_memo` |  |  |  | Y | Y |  |  | unknown | 166&#8209;170 |
| 12 | `min_edit_distance_rec` |  |  |  | Y | Y |  |  | unknown | 177&#8209;191 |
| 13 | `source_mut` | Y | Y |  |  |  | Y | Y |  | 363&#8209;365 |
| 14 | `target_mut` | Y | Y |  |  |  | Y | Y |  | 367&#8209;369 |
| 15 | `eq` |  | Y |  |  |  | Y | Y |  | 380 |

### Chap49/MinEditDistMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `new` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 17 | `from_sequences` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 18 | `min_edit_distance` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;109 |
| 19 | `source` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;114 |
| 20 | `target` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;119 |
| 21 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 123 |
| 22 | `new_arc_memo` |  |  |  | Y | Y |  |  | unknown | 129&#8209;133 |
| 23 | `clone_arc_memo` |  |  |  | Y | Y |  |  | unknown | 139&#8209;143 |
| 24 | `min_edit_distance_rec` |  |  |  | Y | Y |  |  | unknown | 150&#8209;164 |
| 25 | `eq` |  | Y |  |  |  | Y | Y |  | 313 |

### Chap49/MinEditDistStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 26 | `new` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 27 | `from_sequences` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;99 |
| 28 | `min_edit_distance` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;107 |
| 29 | `source` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 30 | `target` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 31 | `set_source` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;125 |
| 32 | `set_target` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;133 |
| 33 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;140 |
| 34 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 144 |
| 35 | `min_edit_distance_rec` |  |  |  | Y | Y |  |  | unknown | 151&#8209;166 |
| 36 | `source_mut` | Y | Y |  |  |  | Y | Y |  | 270&#8209;272 |
| 37 | `target_mut` | Y | Y |  |  |  | Y | Y |  | 274&#8209;276 |

### Chap49/MinEditDistStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `new` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 39 | `from_sequences` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;99 |
| 40 | `min_edit_distance` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 41 | `source` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 42 | `target` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;114 |
| 43 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 118 |
| 44 | `min_edit_distance_rec` |  |  |  | Y | Y |  |  | unknown | 125&#8209;140 |
| 45 | `eq` |  | Y |  |  |  | Y | Y |  | 236&#8209;240 |

### Chap49/SubsetSumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 46 | `new` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 47 | `from_multiset` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 48 | `subset_sum` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;106 |
| 49 | `multiset` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 50 | `set` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;117 |
| 51 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;122 |
| 52 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 126 |
| 53 | `new_arc_memo` |  |  |  | Y | Y |  |  | unknown | 132&#8209;136 |
| 54 | `clone_arc_memo` |  |  |  | Y | Y |  |  | unknown | 142&#8209;146 |
| 55 | `subset_sum_rec` |  |  |  | Y | Y |  |  | unknown | 153&#8209;163 |
| 56 | `multiset_mut` | Y | Y |  |  |  | Y | Y |  | 305&#8209;307 |
| 57 | `eq` |  | Y |  |  |  | Y | Y |  | 317 |

### Chap49/SubsetSumMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 58 | `new` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 59 | `from_multiset` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |
| 60 | `subset_sum` | Y | Y |  |  | Y |  | Y |  | 102&#8209;104 |
| 61 | `multiset` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 62 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 113 |
| 63 | `new_arc_memo` |  |  |  | Y | Y |  |  | unknown | 119&#8209;123 |
| 64 | `clone_arc_memo` |  |  |  | Y | Y |  |  | unknown | 129&#8209;133 |
| 65 | `subset_sum_rec` |  |  |  | Y | Y |  |  | unknown | 140&#8209;150 |
| 66 | `eq` |  | Y |  |  |  | Y | Y |  | 278 |

### Chap49/SubsetSumStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 67 | `new` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 68 | `from_multiset` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;87 |
| 69 | `subset_sum` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;94 |
| 70 | `multiset` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 71 | `set` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;105 |
| 72 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 73 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 114 |
| 74 | `subset_sum_rec` |  |  |  | Y | Y |  |  | unknown | 121&#8209;130 |
| 75 | `multiset_mut` | Y | Y |  |  |  | Y | Y |  | 220&#8209;222 |

### Chap49/SubsetSumStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 76 | `new` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 77 | `from_multiset` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;87 |
| 78 | `subset_sum` | Y | Y |  |  | Y |  | Y |  | 91&#8209;93 |
| 79 | `multiset` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;98 |
| 80 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 102 |
| 81 | `subset_sum_rec` |  |  |  | Y | Y |  |  | unknown | 109&#8209;118 |
| 82 | `eq` |  | Y |  |  |  | Y | Y |  | 204&#8209;206 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
