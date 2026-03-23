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
| 1 | Chap51 | BottomUpDPMtEph | 8 | 10 | 0 | 0 | 10 | 0 | 9 | 1 | 0 |
| 2 | Chap51 | BottomUpDPMtPer | 6 | 8 | 0 | 0 | 8 | 0 | 7 | 1 | 0 |
| 3 | Chap51 | BottomUpDPStEph | 10 | 12 | 0 | 0 | 12 | 0 | 11 | 1 | 0 |
| 4 | Chap51 | BottomUpDPStPer | 8 | 10 | 0 | 0 | 10 | 0 | 9 | 1 | 0 |
| 5 | Chap51 | TopDownDPMtEph | 9 | 11 | 0 | 3 | 14 | 0 | 13 | 1 | 0 |
| 6 | Chap51 | TopDownDPMtPer | 7 | 9 | 0 | 3 | 12 | 0 | 11 | 1 | 0 |
| 7 | Chap51 | TopDownDPStEph | 14 | 16 | 0 | 1 | 17 | 0 | 16 | 1 | 0 |
| 8 | Chap51 | TopDownDPStPer | 12 | 14 | 0 | 1 | 15 | 0 | 14 | 1 | 0 |

## Function-by-Function Detail

### Chap51/BottomUpDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;52 |
| 2 | `new` | Y | Y |  |  | Y |  |  | unknown | 56&#8209;61 |
| 3 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 4 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 5 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 6 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;83 |
| 7 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;90 |
| 8 | `med_bottom_up_parallel` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;102 |
| 9 | `default` |  | Y |  |  | Y |  |  | unknown | 298&#8209;301 |
| 10 | `eq` |  | Y |  |  | Y |  |  | hole | 324&#8209;325 |

### Chap51/BottomUpDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;52 |
| 12 | `new` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;59 |
| 13 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 14 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 15 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 16 | `med_bottom_up_parallel` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;76 |
| 17 | `default` |  | Y |  |  | Y |  |  | unknown | 265&#8209;268 |
| 18 | `eq` |  | Y |  |  | Y |  |  | hole | 291&#8209;292 |

### Chap51/BottomUpDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 19 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 50&#8209;51 |
| 20 | `new` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;60 |
| 21 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 22 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 23 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 24 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;82 |
| 25 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;89 |
| 26 | `med_bottom_up` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;101 |
| 27 | `initialize_base_cases` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;119 |
| 28 | `compute_cell_value` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;144 |
| 29 | `default` |  | Y |  |  | Y |  |  | unknown | 450&#8209;453 |
| 30 | `eq` |  | Y |  |  | Y |  |  | hole | 476&#8209;477 |

### Chap51/BottomUpDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 31 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;52 |
| 32 | `new` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;59 |
| 33 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 34 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 35 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 36 | `med_bottom_up` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;76 |
| 37 | `initialize_base_cases` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;92 |
| 38 | `compute_cell_value` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;115 |
| 39 | `default` |  | Y |  |  | Y |  |  | unknown | 407&#8209;410 |
| 40 | `eq` |  | Y |  |  | Y |  |  | hole | 433&#8209;434 |

### Chap51/TopDownDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 41 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 97&#8209;99 |
| 42 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;119 |
| 43 | `new` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;128 |
| 44 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;133 |
| 45 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;138 |
| 46 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;143 |
| 47 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;150 |
| 48 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;157 |
| 49 | `med_memoized_concurrent` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;169 |
| 50 | `med_memoized_parallel` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;181 |
| 51 | `med_recursive_sequential` |  |  |  | Y | Y |  |  | unknown | 189&#8209;206 |
| 52 | `med_recursive_parallel` |  |  |  | Y | Y |  |  | unknown | 265&#8209;280 |
| 53 | `default` |  | Y |  |  | Y |  |  | unknown | 454&#8209;457 |
| 54 | `eq` |  | Y |  |  | Y |  |  | hole | 480&#8209;481 |

### Chap51/TopDownDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 55 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 97&#8209;99 |
| 56 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;119 |
| 57 | `new` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;126 |
| 58 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 59 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;132 |
| 60 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;135 |
| 61 | `med_memoized_concurrent` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 62 | `med_memoized_parallel` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;143 |
| 63 | `med_recursive_sequential` |  |  |  | Y | Y |  |  | unknown | 149&#8209;166 |
| 64 | `med_recursive_parallel` |  |  |  | Y | Y |  |  | unknown | 223&#8209;238 |
| 65 | `default` |  | Y |  |  | Y |  |  | unknown | 409&#8209;412 |
| 66 | `eq` |  | Y |  |  | Y |  |  | hole | 435&#8209;436 |

### Chap51/TopDownDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 67 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 70&#8209;72 |
| 68 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 69 | `new` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;103 |
| 70 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;108 |
| 71 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 72 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 73 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 74 | `is_memoized` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;128 |
| 75 | `get_memoized` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;138 |
| 76 | `insert_memo` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;146 |
| 77 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;154 |
| 78 | `set_s` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;161 |
| 79 | `set_t` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;168 |
| 80 | `med_memoized` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;180 |
| 81 | `med_recursive` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;197 |
| 82 | `default` |  | Y |  |  | Y |  |  | unknown | 351&#8209;354 |
| 83 | `eq` |  | Y |  |  | Y |  |  | hole | 378&#8209;379 |

### Chap51/TopDownDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 84 | `lemma_spec_med_fn_bounded` |  |  |  | Y | Y |  |  | unknown | 70&#8209;72 |
| 85 | `lemma_spec_med_bounded` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 86 | `new` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;101 |
| 87 | `s_length` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 88 | `t_length` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;107 |
| 89 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 90 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 91 | `is_memoized` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;116 |
| 92 | `get_memoized` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;124 |
| 93 | `with_memo_table` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;129 |
| 94 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;134 |
| 95 | `med_memoized` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |
| 96 | `med_recursive` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;154 |
| 97 | `default` |  | Y |  |  | Y |  |  | unknown | 300&#8209;303 |
| 98 | `eq` |  | Y |  |  | Y |  |  | hole | 327&#8209;328 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
