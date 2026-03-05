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
| 1 | Chap51 | BottomUpDPMtEph | 8 | 10 | 0 | 1 | 11 | 0 | 8 | 3 | 0 |
| 2 | Chap51 | BottomUpDPMtPer | 6 | 8 | 0 | 1 | 9 | 0 | 6 | 3 | 0 |
| 3 | Chap51 | BottomUpDPStEph | 10 | 12 | 0 | 0 | 12 | 0 | 10 | 2 | 0 |
| 4 | Chap51 | BottomUpDPStPer | 8 | 10 | 0 | 0 | 10 | 0 | 8 | 2 | 0 |
| 5 | Chap51 | TopDownDPMtEph | 9 | 11 | 0 | 3 | 12 | 2 | 8 | 4 | 2 |
| 6 | Chap51 | TopDownDPMtPer | 7 | 9 | 0 | 3 | 10 | 2 | 6 | 4 | 2 |
| 7 | Chap51 | TopDownDPStEph | 14 | 16 | 0 | 1 | 17 | 0 | 16 | 1 | 0 |
| 8 | Chap51 | TopDownDPStPer | 12 | 14 | 0 | 1 | 15 | 0 | 14 | 1 | 0 |

## Function-by-Function Detail

### Chap51/BottomUpDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;61 |
| 2 | `new` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;68 |
| 3 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 4 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 5 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 6 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;82 |
| 7 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;87 |
| 8 | `med_bottom_up_parallel` | Y | Y |  |  | Y |  |  | hole | 89&#8209;97 |
| 9 | `new_bu_eph_lock` |  |  |  | Y | Y |  |  | hole | 109&#8209;110 |
| 10 | `default` |  | Y |  |  | Y |  |  | unknown | 244&#8209;247 |
| 11 | `eq` |  | Y |  |  | Y |  |  | hole | 270&#8209;271 |

### Chap51/BottomUpDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 12 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;61 |
| 13 | `new` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;68 |
| 14 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 15 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 16 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 17 | `med_bottom_up_parallel` | Y | Y |  |  | Y |  |  | hole | 79&#8209;85 |
| 18 | `new_bu_per_lock` |  |  |  | Y | Y |  |  | hole | 97&#8209;98 |
| 19 | `default` |  | Y |  |  | Y |  |  | unknown | 229&#8209;232 |
| 20 | `eq` |  | Y |  |  | Y |  |  | hole | 255&#8209;256 |

### Chap51/BottomUpDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 21 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 50&#8209;51 |
| 22 | `new` | Y | Y |  |  | Y |  |  | unknown | 53&#8209;58 |
| 23 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;61 |
| 24 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 25 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;67 |
| 26 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;72 |
| 27 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;77 |
| 28 | `med_bottom_up` | Y | Y |  |  | Y |  |  | hole | 79&#8209;87 |
| 29 | `initialize_base_cases` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;103 |
| 30 | `compute_cell_value` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;126 |
| 31 | `default` |  | Y |  |  | Y |  |  | unknown | 318&#8209;321 |
| 32 | `eq` |  | Y |  |  | Y |  |  | hole | 344&#8209;345 |

### Chap51/BottomUpDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 33 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;52 |
| 34 | `new` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;59 |
| 35 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 36 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 37 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 38 | `med_bottom_up` | Y | Y |  |  | Y |  |  | hole | 70&#8209;76 |
| 39 | `initialize_base_cases` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;92 |
| 40 | `compute_cell_value` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;115 |
| 41 | `default` |  | Y |  |  | Y |  |  | unknown | 304&#8209;307 |
| 42 | `eq` |  | Y |  |  | Y |  |  | hole | 330&#8209;331 |

### Chap51/TopDownDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 43 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 44 | `new` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;69 |
| 45 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 46 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 47 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 48 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;83 |
| 49 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;88 |
| 50 | `med_memoized_concurrent` | Y | Y |  |  | Y |  |  | hole | 90&#8209;98 |
| 51 | `med_memoized_parallel` | Y | Y |  |  | Y |  |  | hole | 100&#8209;108 |
| 52 | `new_td_eph_lock` |  |  |  | Y | Y |  |  | hole | 118&#8209;119 |
| 53 | `default` |  | Y |  |  | Y |  |  | unknown | 202&#8209;205 |
| 54 | `eq` |  | Y |  |  | Y |  |  | hole | 228&#8209;229 |
| 55 | `med_recursive_concurrent` |  |  |  | Y |  | Y | Y |  | 243&#8209;274 |
| 56 | `med_recursive_parallel` |  |  |  | Y |  | Y | Y |  | 276&#8209;330 |

### Chap51/TopDownDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 57 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 58 | `new` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;69 |
| 59 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 60 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 61 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 62 | `med_memoized_concurrent` | Y | Y |  |  | Y |  |  | hole | 80&#8209;82 |
| 63 | `med_memoized_parallel` | Y | Y |  |  | Y |  |  | hole | 84&#8209;86 |
| 64 | `new_td_per_lock` |  |  |  | Y | Y |  |  | hole | 96&#8209;97 |
| 65 | `default` |  | Y |  |  | Y |  |  | unknown | 177&#8209;180 |
| 66 | `eq` |  | Y |  |  | Y |  |  | hole | 203&#8209;204 |
| 67 | `med_recursive_concurrent` |  |  |  | Y |  | Y | Y |  | 218&#8209;249 |
| 68 | `med_recursive_parallel` |  |  |  | Y |  | Y | Y |  | 251&#8209;305 |

### Chap51/TopDownDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 69 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 70&#8209;72 |
| 70 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 71 | `new` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;101 |
| 72 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 73 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;107 |
| 74 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 75 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 76 | `is_memoized` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;116 |
| 77 | `get_memoized` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;124 |
| 78 | `insert_memo` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;130 |
| 79 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;136 |
| 80 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;141 |
| 81 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;146 |
| 82 | `med_memoized` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;156 |
| 83 | `med_recursive` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;171 |
| 84 | `default` |  | Y |  |  | Y |  |  | unknown | 328&#8209;331 |
| 85 | `eq` |  | Y |  |  | Y |  |  | hole | 355&#8209;356 |

### Chap51/TopDownDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 86 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 70&#8209;72 |
| 87 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 88 | `new` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;101 |
| 89 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 90 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;107 |
| 91 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 92 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 93 | `is_memoized` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;116 |
| 94 | `get_memoized` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;124 |
| 95 | `with_memo_table` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;129 |
| 96 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;134 |
| 97 | `med_memoized` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |
| 98 | `med_recursive` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;154 |
| 99 | `default` |  | Y |  |  | Y |  |  | unknown | 302&#8209;305 |
| 100 | `eq` |  | Y |  |  | Y |  |  | hole | 329&#8209;330 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
