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
| 1 | Chap49 | MinEditDistMtEph | 11 | 12 | 0 | 2 | 1 | 13 | 0 | 1 | 13 |
| 2 | Chap49 | MinEditDistMtPer | 6 | 7 | 0 | 2 | 1 | 8 | 0 | 1 | 8 |
| 3 | Chap49 | MinEditDistStEph | 11 | 11 | 0 | 2 | 1 | 12 | 0 | 0 | 13 |
| 4 | Chap49 | MinEditDistStPer | 6 | 6 | 0 | 2 | 1 | 7 | 0 | 0 | 8 |
| 5 | Chap49 | SubsetSumMtEph | 8 | 9 | 0 | 2 | 1 | 10 | 0 | 1 | 10 |
| 6 | Chap49 | SubsetSumMtPer | 5 | 6 | 0 | 2 | 1 | 7 | 0 | 1 | 7 |
| 7 | Chap49 | SubsetSumStEph | 8 | 8 | 0 | 2 | 1 | 9 | 0 | 0 | 10 |
| 8 | Chap49 | SubsetSumStPer | 5 | 5 | 0 | 2 | 1 | 6 | 0 | 0 | 7 |

## Function-by-Function Detail

### Chap49/MinEditDistMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_min_edit_dist_eph_lock` |  |  |  | Y | Y |  |  | hole | 30 |
| 2 | `new` | Y | Y |  |  |  | Y | Y |  | 48&#8209;53 |
| 3 | `from_sequences` | Y | Y |  |  |  | Y | Y |  | 55&#8209;58 |
| 4 | `min_edit_distance` | Y | Y |  |  |  | Y | Y |  | 60&#8209;64 |
| 5 | `source` | Y | Y |  |  |  | Y | Y |  | 66&#8209;69 |
| 6 | `target` | Y | Y |  |  |  | Y | Y |  | 71&#8209;74 |
| 7 | `source_mut` | Y | Y |  |  |  | Y | Y |  | 76&#8209;79 |
| 8 | `target_mut` | Y | Y |  |  |  | Y | Y |  | 81&#8209;84 |
| 9 | `set_source` | Y | Y |  |  |  | Y | Y |  | 86&#8209;89 |
| 10 | `set_target` | Y | Y |  |  |  | Y | Y |  | 91&#8209;94 |
| 11 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 96&#8209;99 |
| 12 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 101&#8209;104 |
| 13 | `min_edit_distance_rec` |  |  |  | Y |  | Y | Y |  | 109&#8209;160 |
| 14 | `eq` |  | Y |  |  |  | Y | Y |  | 237 |

### Chap49/MinEditDistMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 15 | `new_min_edit_dist_per_lock` |  |  |  | Y | Y |  |  | hole | 29 |
| 16 | `new` | Y | Y |  |  |  | Y | Y |  | 47&#8209;52 |
| 17 | `from_sequences` | Y | Y |  |  |  | Y | Y |  | 54&#8209;57 |
| 18 | `min_edit_distance` | Y | Y |  |  |  | Y | Y |  | 59&#8209;63 |
| 19 | `source` | Y | Y |  |  |  | Y | Y |  | 65&#8209;68 |
| 20 | `target` | Y | Y |  |  |  | Y | Y |  | 70&#8209;73 |
| 21 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 75&#8209;78 |
| 22 | `min_edit_distance_rec` |  |  |  | Y |  | Y | Y |  | 83&#8209;130 |
| 23 | `eq` |  | Y |  |  |  | Y | Y |  | 183 |

### Chap49/MinEditDistStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 24 | `_min_edit_dist_st_eph_verified` |  |  |  | Y | Y |  | Y |  | 23 |
| 25 | `new` | Y | Y |  |  |  | Y | Y |  | 39&#8209;44 |
| 26 | `from_sequences` | Y | Y |  |  |  | Y | Y |  | 46&#8209;49 |
| 27 | `min_edit_distance` | Y | Y |  |  |  | Y | Y |  | 51&#8209;53 |
| 28 | `source` | Y | Y |  |  |  | Y | Y |  | 55&#8209;58 |
| 29 | `target` | Y | Y |  |  |  | Y | Y |  | 60&#8209;63 |
| 30 | `source_mut` | Y | Y |  |  |  | Y | Y |  | 65&#8209;68 |
| 31 | `target_mut` | Y | Y |  |  |  | Y | Y |  | 70&#8209;73 |
| 32 | `set_source` | Y | Y |  |  |  | Y | Y |  | 75&#8209;78 |
| 33 | `set_target` | Y | Y |  |  |  | Y | Y |  | 80&#8209;83 |
| 34 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 85&#8209;88 |
| 35 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 90&#8209;93 |
| 36 | `min_edit_distance_rec` |  |  |  | Y |  | Y | Y |  | 98&#8209;125 |

### Chap49/MinEditDistStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 37 | `_min_edit_dist_st_per_verified` |  |  |  | Y | Y |  | Y |  | 22 |
| 38 | `new` | Y | Y |  |  |  | Y | Y |  | 38&#8209;43 |
| 39 | `from_sequences` | Y | Y |  |  |  | Y | Y |  | 45&#8209;48 |
| 40 | `min_edit_distance` | Y | Y |  |  |  | Y | Y |  | 50&#8209;52 |
| 41 | `source` | Y | Y |  |  |  | Y | Y |  | 54&#8209;57 |
| 42 | `target` | Y | Y |  |  |  | Y | Y |  | 59&#8209;62 |
| 43 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 64&#8209;67 |
| 44 | `min_edit_distance_rec` |  |  |  | Y |  | Y | Y |  | 72&#8209;99 |

### Chap49/SubsetSumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 45 | `new_subset_sum_eph_lock` |  |  |  | Y | Y |  |  | hole | 30 |
| 46 | `new` | Y | Y |  |  |  | Y | Y |  | 47&#8209;52 |
| 47 | `from_multiset` | Y | Y |  |  |  | Y | Y |  | 54&#8209;57 |
| 48 | `subset_sum` | Y | Y |  |  |  | Y | Y |  | 59&#8209;63 |
| 49 | `multiset` | Y | Y |  |  |  | Y | Y |  | 65&#8209;68 |
| 50 | `multiset_mut` | Y | Y |  |  |  | Y | Y |  | 70&#8209;73 |
| 51 | `set` | Y | Y |  |  |  | Y | Y |  | 75&#8209;78 |
| 52 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 80&#8209;83 |
| 53 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 85&#8209;88 |
| 54 | `subset_sum_rec` |  |  |  | Y |  | Y | Y |  | 93&#8209;138 |
| 55 | `eq` |  | Y |  |  |  | Y | Y |  | 204 |

### Chap49/SubsetSumMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 56 | `new_subset_sum_per_lock` |  |  |  | Y | Y |  |  | hole | 29 |
| 57 | `new` | Y | Y |  |  |  | Y | Y |  | 46&#8209;51 |
| 58 | `from_multiset` | Y | Y |  |  |  | Y | Y |  | 53&#8209;56 |
| 59 | `subset_sum` | Y | Y |  |  |  | Y | Y |  | 58&#8209;62 |
| 60 | `multiset` | Y | Y |  |  |  | Y | Y |  | 64&#8209;67 |
| 61 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 69&#8209;72 |
| 62 | `subset_sum_rec` |  |  |  | Y |  | Y | Y |  | 77&#8209;122 |
| 63 | `eq` |  | Y |  |  |  | Y | Y |  | 173 |

### Chap49/SubsetSumStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 64 | `_subset_sum_st_eph_verified` |  |  |  | Y | Y |  | Y |  | 21 |
| 65 | `new` | Y | Y |  |  |  | Y | Y |  | 36&#8209;41 |
| 66 | `from_multiset` | Y | Y |  |  |  | Y | Y |  | 43&#8209;46 |
| 67 | `subset_sum` | Y | Y |  |  |  | Y | Y |  | 48&#8209;52 |
| 68 | `multiset` | Y | Y |  |  |  | Y | Y |  | 54&#8209;57 |
| 69 | `multiset_mut` | Y | Y |  |  |  | Y | Y |  | 59&#8209;62 |
| 70 | `set` | Y | Y |  |  |  | Y | Y |  | 64&#8209;67 |
| 71 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 69&#8209;72 |
| 72 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 74&#8209;77 |
| 73 | `subset_sum_rec` |  |  |  | Y |  | Y | Y |  | 82&#8209;104 |

### Chap49/SubsetSumStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 74 | `_subset_sum_st_per_verified` |  |  |  | Y | Y |  | Y |  | 21 |
| 75 | `new` | Y | Y |  |  |  | Y | Y |  | 36&#8209;41 |
| 76 | `from_multiset` | Y | Y |  |  |  | Y | Y |  | 43&#8209;46 |
| 77 | `subset_sum` | Y | Y |  |  |  | Y | Y |  | 48&#8209;52 |
| 78 | `multiset` | Y | Y |  |  |  | Y | Y |  | 54&#8209;57 |
| 79 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 59&#8209;62 |
| 80 | `subset_sum_rec` |  |  |  | Y |  | Y | Y |  | 67&#8209;89 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
