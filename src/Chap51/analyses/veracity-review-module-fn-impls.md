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
| 1 | Chap51 | BottomUpDPMtEph | 10 | 12 | 0 | 0 | 12 | 0 | 12 | 0 | 0 |
| 2 | Chap51 | BottomUpDPMtPer | 8 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 3 | Chap51 | BottomUpDPStEph | 10 | 12 | 0 | 0 | 12 | 0 | 12 | 0 | 0 |
| 4 | Chap51 | BottomUpDPStPer | 8 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 5 | Chap51 | SeqSpecsAndLemmas | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 6 | Chap51 | TopDownDPMtEph | 9 | 11 | 0 | 2 | 13 | 0 | 13 | 0 | 0 |
| 7 | Chap51 | TopDownDPMtPer | 7 | 9 | 0 | 2 | 11 | 0 | 11 | 0 | 0 |
| 8 | Chap51 | TopDownDPStEph | 14 | 16 | 0 | 0 | 16 | 0 | 16 | 0 | 0 |
| 9 | Chap51 | TopDownDPStPer | 12 | 14 | 0 | 0 | 14 | 0 | 14 | 0 | 0 |

## Function-by-Function Detail

### Chap51/BottomUpDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;59 |
| 2 | `new` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;68 |
| 3 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;73 |
| 4 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;78 |
| 5 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 6 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;91 |
| 7 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;99 |
| 8 | `med_bottom_up_parallel` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;114 |
| 9 | `initialize_base_cases` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;132 |
| 10 | `compute_cell_value` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;157 |
| 11 | `default` |  | Y |  |  | Y |  |  | unknown | 455&#8209;459 |
| 12 | `eq` |  | Y |  |  | Y |  |  | unknown | 481&#8209;482 |

### Chap51/BottomUpDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 13 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;59 |
| 14 | `new` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;68 |
| 15 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;73 |
| 16 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;78 |
| 17 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 18 | `med_bottom_up_parallel` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;94 |
| 19 | `initialize_base_cases` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;112 |
| 20 | `compute_cell_value` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;137 |
| 21 | `default` |  | Y |  |  | Y |  |  | unknown | 426&#8209;430 |
| 22 | `eq` |  | Y |  |  | Y |  |  | unknown | 452&#8209;453 |

### Chap51/BottomUpDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 23 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 24 | `new` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;69 |
| 25 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;74 |
| 26 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 27 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;84 |
| 28 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;92 |
| 29 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;100 |
| 30 | `med_bottom_up` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;115 |
| 31 | `initialize_base_cases` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;133 |
| 32 | `compute_cell_value` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;158 |
| 33 | `default` |  | Y |  |  | Y |  |  | unknown | 474&#8209;478 |
| 34 | `eq` |  | Y |  |  | Y |  |  | unknown | 500&#8209;501 |

### Chap51/BottomUpDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 35 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;59 |
| 36 | `new` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;68 |
| 37 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;73 |
| 38 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;78 |
| 39 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 40 | `med_bottom_up` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;95 |
| 41 | `initialize_base_cases` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;113 |
| 42 | `compute_cell_value` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;138 |
| 43 | `default` |  | Y |  |  | Y |  |  | unknown | 438&#8209;442 |
| 44 | `eq` |  | Y |  |  | Y |  |  | unknown | 464&#8209;465 |

### Chap51/SeqSpecsAndLemmas.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 45 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 65&#8209;67 |

### Chap51/TopDownDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 46 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;159 |
| 47 | `new` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;168 |
| 48 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;173 |
| 49 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;178 |
| 50 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;183 |
| 51 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;191 |
| 52 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;199 |
| 53 | `med_memoized_concurrent` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;214 |
| 54 | `med_memoized_parallel` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;229 |
| 55 | `med_recursive_sequential` |  |  |  | Y | Y |  |  | unknown | 238&#8209;253 |
| 56 | `med_recursive_parallel` |  |  |  | Y | Y |  |  | unknown | 319&#8209;334 |
| 57 | `default` |  | Y |  |  | Y |  |  | unknown | 478&#8209;482 |
| 58 | `eq` |  | Y |  |  | Y |  |  | unknown | 505&#8209;506 |

### Chap51/TopDownDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 59 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;153 |
| 60 | `new` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;162 |
| 61 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;167 |
| 62 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;172 |
| 63 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;177 |
| 64 | `med_memoized_concurrent` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;184 |
| 65 | `med_memoized_parallel` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;191 |
| 66 | `med_recursive_sequential` |  |  |  | Y | Y |  |  | unknown | 199&#8209;214 |
| 67 | `med_recursive_parallel` |  |  |  | Y | Y |  |  | unknown | 279&#8209;294 |
| 68 | `default` |  | Y |  |  | Y |  |  | unknown | 438&#8209;442 |
| 69 | `eq` |  | Y |  |  | Y |  |  | unknown | 465&#8209;466 |

### Chap51/TopDownDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 70 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 71 | `new` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;84 |
| 72 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;89 |
| 73 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 74 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 75 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;104 |
| 76 | `is_memoized` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 77 | `get_memoized` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;119 |
| 78 | `insert_memo` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;127 |
| 79 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;135 |
| 80 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;143 |
| 81 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;151 |
| 82 | `med_memoized` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;166 |
| 83 | `med_recursive` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;183 |
| 84 | `default` |  | Y |  |  | Y |  |  | unknown | 373&#8209;377 |
| 85 | `eq` |  | Y |  |  | Y |  |  | unknown | 400&#8209;401 |

### Chap51/TopDownDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 86 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 87 | `new` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;84 |
| 88 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;89 |
| 89 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 90 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 91 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;104 |
| 92 | `is_memoized` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 93 | `get_memoized` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;119 |
| 94 | `with_memo_table` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;125 |
| 95 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;132 |
| 96 | `med_memoized` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;139 |
| 97 | `med_recursive` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;156 |
| 98 | `default` |  | Y |  |  | Y |  |  | unknown | 336&#8209;340 |
| 99 | `eq` |  | Y |  |  | Y |  |  | unknown | 364&#8209;365 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
