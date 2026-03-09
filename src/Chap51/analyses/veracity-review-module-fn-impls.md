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
| 1 | Chap51 | BottomUpDPMtEph | 8 | 10 | 0 | 0 | 10 | 0 | 8 | 2 | 0 |
| 2 | Chap51 | BottomUpDPMtPer | 6 | 8 | 0 | 0 | 8 | 0 | 6 | 2 | 0 |
| 3 | Chap51 | BottomUpDPStEph | 10 | 12 | 0 | 0 | 12 | 0 | 10 | 2 | 0 |
| 4 | Chap51 | BottomUpDPStPer | 8 | 10 | 0 | 0 | 10 | 0 | 8 | 2 | 0 |
| 5 | Chap51 | TopDownDPMtEph | 9 | 11 | 0 | 2 | 11 | 2 | 8 | 3 | 2 |
| 6 | Chap51 | TopDownDPMtPer | 7 | 9 | 0 | 2 | 9 | 2 | 6 | 3 | 2 |
| 7 | Chap51 | TopDownDPStEph | 14 | 16 | 0 | 1 | 17 | 0 | 16 | 1 | 0 |
| 8 | Chap51 | TopDownDPStPer | 12 | 14 | 0 | 1 | 15 | 0 | 14 | 1 | 0 |

## Function-by-Function Detail

### Chap51/BottomUpDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 2 | `new` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;69 |
| 3 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 4 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 5 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 6 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;83 |
| 7 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;88 |
| 8 | `med_bottom_up_parallel` | Y | Y |  |  | Y |  |  | hole | 90&#8209;98 |
| 9 | `default` |  | Y |  |  | Y |  |  | unknown | 239&#8209;242 |
| 10 | `eq` |  | Y |  |  | Y |  |  | hole | 265&#8209;266 |

### Chap51/BottomUpDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 12 | `new` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;69 |
| 13 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 14 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 15 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 16 | `med_bottom_up_parallel` | Y | Y |  |  | Y |  |  | hole | 80&#8209;86 |
| 17 | `default` |  | Y |  |  | Y |  |  | unknown | 224&#8209;227 |
| 18 | `eq` |  | Y |  |  | Y |  |  | hole | 250&#8209;251 |

### Chap51/BottomUpDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 19 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 50&#8209;51 |
| 20 | `new` | Y | Y |  |  | Y |  |  | unknown | 53&#8209;58 |
| 21 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;61 |
| 22 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 23 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;67 |
| 24 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;72 |
| 25 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;77 |
| 26 | `med_bottom_up` | Y | Y |  |  | Y |  |  | hole | 79&#8209;87 |
| 27 | `initialize_base_cases` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;103 |
| 28 | `compute_cell_value` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;126 |
| 29 | `default` |  | Y |  |  | Y |  |  | unknown | 318&#8209;321 |
| 30 | `eq` |  | Y |  |  | Y |  |  | hole | 344&#8209;345 |

### Chap51/BottomUpDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 31 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;52 |
| 32 | `new` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;59 |
| 33 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 34 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 35 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 36 | `med_bottom_up` | Y | Y |  |  | Y |  |  | hole | 70&#8209;76 |
| 37 | `initialize_base_cases` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;92 |
| 38 | `compute_cell_value` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;115 |
| 39 | `default` |  | Y |  |  | Y |  |  | unknown | 304&#8209;307 |
| 40 | `eq` |  | Y |  |  | Y |  |  | hole | 330&#8209;331 |

### Chap51/TopDownDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 41 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 42 | `new` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;70 |
| 43 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;73 |
| 44 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 45 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 46 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;84 |
| 47 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;89 |
| 48 | `med_memoized_concurrent` | Y | Y |  |  | Y |  |  | hole | 91&#8209;99 |
| 49 | `med_memoized_parallel` | Y | Y |  |  | Y |  |  | hole | 101&#8209;109 |
| 50 | `default` |  | Y |  |  | Y |  |  | unknown | 197&#8209;200 |
| 51 | `eq` |  | Y |  |  | Y |  |  | hole | 223&#8209;224 |
| 52 | `med_recursive_concurrent` |  |  |  | Y |  | Y | Y |  | 238&#8209;269 |
| 53 | `med_recursive_parallel` |  |  |  | Y |  | Y | Y |  | 271&#8209;323 |

### Chap51/TopDownDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 54 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 55 | `new` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;70 |
| 56 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;73 |
| 57 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 58 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 59 | `med_memoized_concurrent` | Y | Y |  |  | Y |  |  | hole | 81&#8209;83 |
| 60 | `med_memoized_parallel` | Y | Y |  |  | Y |  |  | hole | 85&#8209;87 |
| 61 | `default` |  | Y |  |  | Y |  |  | unknown | 171&#8209;174 |
| 62 | `eq` |  | Y |  |  | Y |  |  | hole | 197&#8209;198 |
| 63 | `med_recursive_concurrent` |  |  |  | Y |  | Y | Y |  | 212&#8209;243 |
| 64 | `med_recursive_parallel` |  |  |  | Y |  | Y | Y |  | 245&#8209;297 |

### Chap51/TopDownDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 65 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 70&#8209;72 |
| 66 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 67 | `new` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;101 |
| 68 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 69 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;107 |
| 70 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 71 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 72 | `is_memoized` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;116 |
| 73 | `get_memoized` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;124 |
| 74 | `insert_memo` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;130 |
| 75 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;136 |
| 76 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;141 |
| 77 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;146 |
| 78 | `med_memoized` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;156 |
| 79 | `med_recursive` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;171 |
| 80 | `default` |  | Y |  |  | Y |  |  | unknown | 328&#8209;331 |
| 81 | `eq` |  | Y |  |  | Y |  |  | hole | 355&#8209;356 |

### Chap51/TopDownDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 82 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 70&#8209;72 |
| 83 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 84 | `new` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;101 |
| 85 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 86 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;107 |
| 87 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 88 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 89 | `is_memoized` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;116 |
| 90 | `get_memoized` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;124 |
| 91 | `with_memo_table` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;129 |
| 92 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;134 |
| 93 | `med_memoized` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |
| 94 | `med_recursive` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;154 |
| 95 | `default` |  | Y |  |  | Y |  |  | unknown | 302&#8209;305 |
| 96 | `eq` |  | Y |  |  | Y |  |  | hole | 329&#8209;330 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
