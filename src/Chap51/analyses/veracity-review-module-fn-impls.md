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
| 1 | Chap51 | BottomUpDPMtEph | 10 | 12 | 0 | 1 | 1 | 12 | 0 | 1 | 12 |
| 2 | Chap51 | BottomUpDPMtPer | 8 | 10 | 0 | 1 | 1 | 10 | 0 | 1 | 10 |
| 3 | Chap51 | BottomUpDPStEph | 10 | 12 | 0 | 0 | 0 | 12 | 0 | 0 | 12 |
| 4 | Chap51 | BottomUpDPStPer | 8 | 10 | 0 | 0 | 0 | 10 | 0 | 0 | 10 |
| 5 | Chap51 | TopDownDPMtEph | 15 | 17 | 0 | 1 | 1 | 17 | 0 | 1 | 17 |
| 6 | Chap51 | TopDownDPMtPer | 13 | 15 | 0 | 1 | 1 | 15 | 0 | 1 | 15 |
| 7 | Chap51 | TopDownDPStEph | 13 | 14 | 0 | 0 | 0 | 14 | 0 | 0 | 14 |
| 8 | Chap51 | TopDownDPStPer | 11 | 12 | 0 | 0 | 0 | 12 | 0 | 0 | 12 |

## Function-by-Function Detail

### Chap51/BottomUpDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_bu_eph_lock` |  |  |  | Y | Y |  |  | hole | 43 |
| 2 | `new` | Y | Y |  |  |  | Y | Y |  | 52 |
| 3 | `s_length` | Y | Y |  |  |  | Y | Y |  | 53 |
| 4 | `t_length` | Y | Y |  |  |  | Y | Y |  | 54 |
| 5 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 55 |
| 6 | `set_s` | Y | Y |  |  |  | Y | Y |  | 56 |
| 7 | `set_t` | Y | Y |  |  |  | Y | Y |  | 57 |
| 8 | `med_bottom_up_parallel` | Y | Y |  |  |  | Y | Y |  | 58 |
| 9 | `initialize_base_cases` | Y | Y |  |  |  | Y | Y |  | 59 |
| 10 | `compute_diagonal_parallel` | Y | Y |  |  |  | Y | Y |  | 60 |
| 11 | `compute_cell_value_static` | Y | Y |  |  |  | Y | Y |  | 61&#8209;67 |
| 12 | `eq` |  | Y |  |  |  | Y | Y |  | 182&#8209;184 |
| 13 | `default` |  | Y |  |  |  | Y | Y |  | 190&#8209;194 |

### Chap51/BottomUpDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `new_bu_per_lock` |  |  |  | Y | Y |  |  | hole | 43 |
| 15 | `new` | Y | Y |  |  |  | Y | Y |  | 52 |
| 16 | `s_length` | Y | Y |  |  |  | Y | Y |  | 53 |
| 17 | `t_length` | Y | Y |  |  |  | Y | Y |  | 54 |
| 18 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 55 |
| 19 | `med_bottom_up_parallel` | Y | Y |  |  |  | Y | Y |  | 56 |
| 20 | `initialize_base_cases` | Y | Y |  |  |  | Y | Y |  | 57 |
| 21 | `compute_diagonal_parallel` | Y | Y |  |  |  | Y | Y |  | 58 |
| 22 | `compute_cell_value_static` | Y | Y |  |  |  | Y | Y |  | 59&#8209;65 |
| 23 | `eq` |  | Y |  |  |  | Y | Y |  | 178&#8209;180 |
| 24 | `default` |  | Y |  |  |  | Y | Y |  | 186&#8209;190 |

### Chap51/BottomUpDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 25 | `new` | Y | Y |  |  |  | Y | Y |  | 39 |
| 26 | `s_length` | Y | Y |  |  |  | Y | Y |  | 40 |
| 27 | `t_length` | Y | Y |  |  |  | Y | Y |  | 41 |
| 28 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 42 |
| 29 | `set_s` | Y | Y |  |  |  | Y | Y |  | 43 |
| 30 | `set_t` | Y | Y |  |  |  | Y | Y |  | 44 |
| 31 | `med_bottom_up` | Y | Y |  |  |  | Y | Y |  | 45 |
| 32 | `initialize_base_cases` | Y | Y |  |  |  | Y | Y |  | 46 |
| 33 | `compute_diagonal` | Y | Y |  |  |  | Y | Y |  | 47 |
| 34 | `compute_cell_value` | Y | Y |  |  |  | Y | Y |  | 48 |
| 35 | `eq` |  | Y |  |  |  | Y | Y |  | 132&#8209;134 |
| 36 | `default` |  | Y |  |  |  | Y | Y |  | 140&#8209;144 |

### Chap51/BottomUpDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 37 | `new` | Y | Y |  |  |  | Y | Y |  | 39 |
| 38 | `s_length` | Y | Y |  |  |  | Y | Y |  | 40 |
| 39 | `t_length` | Y | Y |  |  |  | Y | Y |  | 41 |
| 40 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 42 |
| 41 | `med_bottom_up` | Y | Y |  |  |  | Y | Y |  | 43 |
| 42 | `initialize_base_cases` | Y | Y |  |  |  | Y | Y |  | 44 |
| 43 | `compute_diagonal` | Y | Y |  |  |  | Y | Y |  | 45 |
| 44 | `compute_cell_value` | Y | Y |  |  |  | Y | Y |  | 46 |
| 45 | `eq` |  | Y |  |  |  | Y | Y |  | 131&#8209;133 |
| 46 | `default` |  | Y |  |  |  | Y | Y |  | 139&#8209;143 |

### Chap51/TopDownDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 47 | `new_td_eph_lock` |  |  |  | Y | Y |  |  | hole | 37 |
| 48 | `new` | Y | Y |  |  |  | Y | Y |  | 57 |
| 49 | `med_memoized_concurrent` | Y | Y |  |  |  | Y | Y |  | 58 |
| 50 | `med_memoized_parallel` | Y | Y |  |  |  | Y | Y |  | 59 |
| 51 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 60 |
| 52 | `is_memoized` | Y | Y |  |  |  | Y | Y |  | 61 |
| 53 | `get_memoized` | Y | Y |  |  |  | Y | Y |  | 62 |
| 54 | `insert_memo` | Y | Y |  |  |  | Y | Y |  | 63 |
| 55 | `s_length` | Y | Y |  |  |  | Y | Y |  | 64 |
| 56 | `t_length` | Y | Y |  |  |  | Y | Y |  | 65 |
| 57 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 66 |
| 58 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 67 |
| 59 | `set_s` | Y | Y |  |  |  | Y | Y |  | 68 |
| 60 | `set_t` | Y | Y |  |  |  | Y | Y |  | 69 |
| 61 | `med_recursive_concurrent` | Y | Y |  |  |  | Y | Y |  | 70 |
| 62 | `med_recursive_parallel` | Y | Y |  |  |  | Y | Y |  | 71 |
| 63 | `eq` |  | Y |  |  |  | Y | Y |  | 225&#8209;235 |
| 64 | `default` |  | Y |  |  |  | Y | Y |  | 239&#8209;245 |

### Chap51/TopDownDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 65 | `new_td_per_lock` |  |  |  | Y | Y |  |  | hole | 37 |
| 66 | `new` | Y | Y |  |  |  | Y | Y |  | 57 |
| 67 | `med_memoized_concurrent` | Y | Y |  |  |  | Y | Y |  | 58 |
| 68 | `med_memoized_parallel` | Y | Y |  |  |  | Y | Y |  | 59 |
| 69 | `with_memo_table` | Y | Y |  |  |  | Y | Y |  | 60 |
| 70 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 61 |
| 71 | `is_memoized` | Y | Y |  |  |  | Y | Y |  | 62 |
| 72 | `get_memoized` | Y | Y |  |  |  | Y | Y |  | 63 |
| 73 | `s_length` | Y | Y |  |  |  | Y | Y |  | 64 |
| 74 | `t_length` | Y | Y |  |  |  | Y | Y |  | 65 |
| 75 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 66 |
| 76 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 67 |
| 77 | `med_recursive_concurrent` | Y | Y |  |  |  | Y | Y |  | 68 |
| 78 | `med_recursive_parallel` | Y | Y |  |  |  | Y | Y |  | 69 |
| 79 | `eq` |  | Y |  |  |  | Y | Y |  | 217&#8209;227 |
| 80 | `default` |  | Y |  |  |  | Y | Y |  | 231&#8209;237 |

### Chap51/TopDownDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 81 | `new` | Y | Y |  |  |  | Y | Y |  | 43 |
| 82 | `med_memoized` | Y | Y |  |  |  | Y | Y |  | 44 |
| 83 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 45 |
| 84 | `is_memoized` | Y | Y |  |  |  | Y | Y |  | 46 |
| 85 | `get_memoized` | Y | Y |  |  |  | Y | Y |  | 47 |
| 86 | `insert_memo` | Y | Y |  |  |  | Y | Y |  | 48 |
| 87 | `s_length` | Y | Y |  |  |  | Y | Y |  | 49 |
| 88 | `t_length` | Y | Y |  |  |  | Y | Y |  | 50 |
| 89 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 51 |
| 90 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 52 |
| 91 | `set_s` | Y | Y |  |  |  | Y | Y |  | 53 |
| 92 | `set_t` | Y | Y |  |  |  | Y | Y |  | 54 |
| 93 | `med_recursive` | Y | Y |  |  |  | Y | Y |  | 55 |
| 94 | `default` |  | Y |  |  |  | Y | Y |  | 128&#8209;134 |

### Chap51/TopDownDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 95 | `new` | Y | Y |  |  |  | Y | Y |  | 43 |
| 96 | `med_memoized` | Y | Y |  |  |  | Y | Y |  | 44 |
| 97 | `with_memo_table` | Y | Y |  |  |  | Y | Y |  | 45 |
| 98 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 46 |
| 99 | `is_memoized` | Y | Y |  |  |  | Y | Y |  | 47 |
| 100 | `get_memoized` | Y | Y |  |  |  | Y | Y |  | 48 |
| 101 | `s_length` | Y | Y |  |  |  | Y | Y |  | 49 |
| 102 | `t_length` | Y | Y |  |  |  | Y | Y |  | 50 |
| 103 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 51 |
| 104 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 52 |
| 105 | `med_recursive` | Y | Y |  |  |  | Y | Y |  | 53 |
| 106 | `default` |  | Y |  |  |  | Y | Y |  | 122&#8209;128 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
