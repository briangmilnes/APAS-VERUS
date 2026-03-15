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
| 1 | Chap51 | BottomUpDPMtEph | 8 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 2 | Chap51 | BottomUpDPMtPer | 6 | 8 | 0 | 0 | 8 | 0 | 8 | 0 | 0 |
| 3 | Chap51 | BottomUpDPStEph | 10 | 12 | 0 | 0 | 12 | 0 | 12 | 0 | 0 |
| 4 | Chap51 | BottomUpDPStPer | 8 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 5 | Chap51 | TopDownDPMtEph | 9 | 11 | 0 | 3 | 14 | 0 | 14 | 0 | 0 |
| 6 | Chap51 | TopDownDPMtPer | 7 | 9 | 0 | 3 | 12 | 0 | 12 | 0 | 0 |
| 7 | Chap51 | TopDownDPStEph | 14 | 16 | 0 | 1 | 17 | 0 | 17 | 0 | 0 |
| 8 | Chap51 | TopDownDPStPer | 12 | 14 | 0 | 1 | 15 | 0 | 15 | 0 | 0 |

## Function-by-Function Detail

### Chap51/BottomUpDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;53 |
| 2 | `new` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;60 |
| 3 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 4 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 5 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 6 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;74 |
| 7 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;79 |
| 8 | `med_bottom_up_parallel` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;89 |
| 9 | `default` |  | Y |  |  | Y |  |  | unknown | 285&#8209;288 |
| 10 | `eq` |  | Y |  |  | Y |  |  | unknown | 311&#8209;312 |

### Chap51/BottomUpDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;53 |
| 12 | `new` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;60 |
| 13 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 14 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 15 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 16 | `med_bottom_up_parallel` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;77 |
| 17 | `default` |  | Y |  |  | Y |  |  | unknown | 266&#8209;269 |
| 18 | `eq` |  | Y |  |  | Y |  |  | unknown | 292&#8209;293 |

### Chap51/BottomUpDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 19 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;52 |
| 20 | `new` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;59 |
| 21 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 22 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 23 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 24 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 25 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;78 |
| 26 | `med_bottom_up` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;88 |
| 27 | `initialize_base_cases` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;104 |
| 28 | `compute_cell_value` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;127 |
| 29 | `default` |  | Y |  |  | Y |  |  | unknown | 433&#8209;436 |
| 30 | `eq` |  | Y |  |  | Y |  |  | unknown | 459&#8209;460 |

### Chap51/BottomUpDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 31 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;53 |
| 32 | `new` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;60 |
| 33 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 34 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 35 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 36 | `med_bottom_up` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;77 |
| 37 | `initialize_base_cases` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;93 |
| 38 | `compute_cell_value` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;116 |
| 39 | `default` |  | Y |  |  | Y |  |  | unknown | 408&#8209;411 |
| 40 | `eq` |  | Y |  |  | Y |  |  | unknown | 434&#8209;435 |

### Chap51/TopDownDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 41 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 99&#8209;101 |
| 42 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 43 | `new` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;128 |
| 44 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 45 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 46 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 47 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;142 |
| 48 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;147 |
| 49 | `med_memoized_concurrent` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;157 |
| 50 | `med_memoized_parallel` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;167 |
| 51 | `med_recursive_sequential` |  |  |  | Y | Y |  |  | unknown | 173&#8209;190 |
| 52 | `med_recursive_parallel` |  |  |  | Y | Y |  |  | unknown | 249&#8209;264 |
| 53 | `default` |  | Y |  |  | Y |  |  | unknown | 439&#8209;442 |
| 54 | `eq` |  | Y |  |  | Y |  |  | unknown | 465&#8209;466 |

### Chap51/TopDownDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 55 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 99&#8209;101 |
| 56 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 57 | `new` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;128 |
| 58 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 59 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 60 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 61 | `med_memoized_concurrent` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;141 |
| 62 | `med_memoized_parallel` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |
| 63 | `med_recursive_sequential` |  |  |  | Y | Y |  |  | unknown | 151&#8209;168 |
| 64 | `med_recursive_parallel` |  |  |  | Y | Y |  |  | unknown | 227&#8209;242 |
| 65 | `default` |  | Y |  |  | Y |  |  | unknown | 414&#8209;417 |
| 66 | `eq` |  | Y |  |  | Y |  |  | unknown | 440&#8209;441 |

### Chap51/TopDownDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 67 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 71&#8209;73 |
| 68 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;95 |
| 69 | `new` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;102 |
| 70 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 71 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;108 |
| 72 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 73 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;114 |
| 74 | `is_memoized` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 75 | `get_memoized` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;125 |
| 76 | `insert_memo` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;131 |
| 77 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;137 |
| 78 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;142 |
| 79 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;147 |
| 80 | `med_memoized` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;157 |
| 81 | `med_recursive` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;172 |
| 82 | `default` |  | Y |  |  | Y |  |  | unknown | 329&#8209;332 |
| 83 | `eq` |  | Y |  |  | Y |  |  | unknown | 356&#8209;357 |

### Chap51/TopDownDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 84 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 71&#8209;73 |
| 85 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;95 |
| 86 | `new` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;102 |
| 87 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 88 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;108 |
| 89 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 90 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;114 |
| 91 | `is_memoized` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 92 | `get_memoized` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;125 |
| 93 | `with_memo_table` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;130 |
| 94 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;135 |
| 95 | `med_memoized` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 96 | `med_recursive` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;155 |
| 97 | `default` |  | Y |  |  | Y |  |  | unknown | 303&#8209;306 |
| 98 | `eq` |  | Y |  |  | Y |  |  | unknown | 330&#8209;331 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
