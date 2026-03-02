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
| 1 | Chap50 | MatrixChainMtEph | 13 | 14 | 0 | 2 | 16 | 0 | 0 | 16 | 0 |
| 2 | Chap50 | MatrixChainMtPer | 10 | 11 | 0 | 1 | 12 | 0 | 2 | 9 | 1 |
| 3 | Chap50 | MatrixChainStEph | 12 | 13 | 0 | 0 | 13 | 0 | 12 | 1 | 0 |
| 4 | Chap50 | MatrixChainStPer | 9 | 10 | 0 | 0 | 10 | 0 | 9 | 1 | 0 |
| 5 | Chap50 | OptBinSearchTreeMtEph | 10 | 11 | 0 | 4 | 15 | 0 | 0 | 15 | 0 |
| 6 | Chap50 | OptBinSearchTreeMtPer | 7 | 8 | 0 | 3 | 11 | 0 | 0 | 11 | 0 |
| 7 | Chap50 | OptBinSearchTreeStEph | 10 | 11 | 0 | 1 | 11 | 1 | 0 | 7 | 5 |
| 8 | Chap50 | OptBinSearchTreeStPer | 7 | 8 | 0 | 1 | 8 | 1 | 0 | 5 | 4 |

## Function-by-Function Detail

### Chap50/MatrixChainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new_mceph_dim_lock` |  |  |  | Y | Y |  |  | hole | 67&#8209;69 |
| 2 | `new_mceph_memo_lock` |  |  |  | Y | Y |  |  | hole | 85&#8209;91 |
| 3 | `new` | Y | Y |  |  | Y |  |  | hole | 151 |
| 4 | `from_dimensions` | Y | Y |  |  | Y |  |  | hole | 152 |
| 5 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | hole | 153 |
| 6 | `optimal_cost` | Y | Y |  |  | Y |  |  | hole | 154 |
| 7 | `dimensions` | Y | Y |  |  | Y |  |  | hole | 155 |
| 8 | `set_dimension` | Y | Y |  |  | Y |  |  | hole | 156 |
| 9 | `update_dimension` | Y | Y |  |  | Y |  |  | hole | 157 |
| 10 | `num_matrices` | Y | Y |  |  | Y |  |  | hole | 158 |
| 11 | `clear_memo` | Y | Y |  |  | Y |  |  | hole | 159 |
| 12 | `memo_size` | Y | Y |  |  | Y |  |  | hole | 160 |
| 13 | `multiply_cost` | Y | Y |  |  | Y |  |  | hole | 161 |
| 14 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | hole | 162 |
| 15 | `parallel_min_reduction` | Y | Y |  |  | Y |  |  | hole | 163&#8209;167 |
| 16 | `eq` |  | Y |  |  | Y |  |  | hole | 368 |

### Chap50/MatrixChainMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 17 | `new_mcper_memo_lock` |  |  |  | Y | Y |  |  | hole | 80&#8209;86 |
| 18 | `new` | Y | Y |  |  | Y |  |  | hole | 153&#8209;154 |
| 19 | `from_dimensions` | Y | Y |  |  | Y |  |  | hole | 156&#8209;157 |
| 20 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | hole | 159&#8209;160 |
| 21 | `optimal_cost` | Y | Y |  |  | Y |  |  | hole | 162&#8209;169 |
| 22 | `dimensions` | Y | Y |  |  | Y |  | Y |  | 171 |
| 23 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;174 |
| 24 | `memo_size` | Y | Y |  |  | Y |  |  | hole | 176 |
| 25 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;186 |
| 26 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | hole | 188&#8209;195 |
| 27 | `parallel_min_reduction` | Y | Y |  |  | Y |  |  | hole | 197&#8209;201 |
| 28 | `eq` |  | Y |  |  | Y |  |  | hole | 363&#8209;364 |

### Chap50/MatrixChainStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 29 | `new` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;139 |
| 30 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;144 |
| 31 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;149 |
| 32 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;159 |
| 33 | `dimensions` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;162 |
| 34 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;165 |
| 35 | `set_dimension` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;171 |
| 36 | `update_dimension` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;178 |
| 37 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;183 |
| 38 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;186 |
| 39 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;196 |
| 40 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;211 |
| 41 | `eq` |  | Y |  |  | Y |  |  | hole | 382&#8209;383 |

### Chap50/MatrixChainStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 42 | `new` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;139 |
| 43 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;144 |
| 44 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;149 |
| 45 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;158 |
| 46 | `dimensions` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;161 |
| 47 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 48 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;167 |
| 49 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;177 |
| 50 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;192 |
| 51 | `eq` |  | Y |  |  | Y |  |  | hole | 351&#8209;352 |

### Chap50/OptBinSearchTreeMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 52 | `new_obst_eph_keys_lock` |  |  |  | Y | Y |  |  | hole | 44&#8209;45 |
| 53 | `new_obst_eph_memo_lock` |  |  |  | Y | Y |  |  | hole | 57&#8209;58 |
| 54 | `new` | Y | Y |  |  | Y |  |  | hole | 82 |
| 55 | `from_keys_probs` | Y | Y |  |  | Y |  |  | hole | 83 |
| 56 | `from_key_probs` | Y | Y |  |  | Y |  |  | hole | 84 |
| 57 | `optimal_cost` | Y | Y |  |  | Y |  |  | hole | 85 |
| 58 | `keys` | Y | Y |  |  | Y |  |  | hole | 86 |
| 59 | `set_key_prob` | Y | Y |  |  | Y |  |  | hole | 87 |
| 60 | `update_prob` | Y | Y |  |  | Y |  |  | hole | 88 |
| 61 | `num_keys` | Y | Y |  |  | Y |  |  | hole | 89 |
| 62 | `clear_memo` | Y | Y |  |  | Y |  |  | hole | 90 |
| 63 | `memo_size` | Y | Y |  |  | Y |  |  | hole | 91 |
| 64 | `parallel_min_reduction` |  |  |  | Y | Y |  |  | hole | 97 |
| 65 | `obst_rec` |  |  |  | Y | Y |  |  | hole | 122 |
| 66 | `eq` x2 |  | Y |  |  | Y |  |  | hole | 271 |

### Chap50/OptBinSearchTreeMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 67 | `new_obst_per_memo_lock` |  |  |  | Y | Y |  |  | hole | 46&#8209;47 |
| 68 | `new` | Y | Y |  |  | Y |  |  | hole | 71 |
| 69 | `from_keys_probs` | Y | Y |  |  | Y |  |  | hole | 72 |
| 70 | `from_key_probs` | Y | Y |  |  | Y |  |  | hole | 73 |
| 71 | `optimal_cost` | Y | Y |  |  | Y |  |  | hole | 74 |
| 72 | `keys` | Y | Y |  |  | Y |  |  | hole | 75 |
| 73 | `num_keys` | Y | Y |  |  | Y |  |  | hole | 76 |
| 74 | `memo_size` | Y | Y |  |  | Y |  |  | hole | 77 |
| 75 | `parallel_min_reduction` |  |  |  | Y | Y |  |  | hole | 83 |
| 76 | `obst_rec` |  |  |  | Y | Y |  |  | hole | 108 |
| 77 | `eq` x2 |  | Y |  |  | Y |  |  | hole | 205 |

### Chap50/OptBinSearchTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 78 | `new` | Y | Y |  |  | Y |  |  | hole | 57 |
| 79 | `from_keys_probs` | Y | Y |  |  | Y |  |  | hole | 58 |
| 80 | `from_key_probs` | Y | Y |  |  | Y |  |  | hole | 59 |
| 81 | `optimal_cost` | Y | Y |  |  | Y |  |  | hole | 60 |
| 82 | `keys` | Y | Y |  |  | Y |  | Y |  | 61 |
| 83 | `set_key_prob` | Y | Y |  |  | Y |  |  | hole | 62 |
| 84 | `update_prob` | Y | Y |  |  | Y |  |  | hole | 63 |
| 85 | `num_keys` | Y | Y |  |  | Y |  | Y |  | 64 |
| 86 | `clear_memo` | Y | Y |  |  | Y |  | Y |  | 65 |
| 87 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 66 |
| 88 | `obst_rec_st_eph` |  |  |  | Y | Y |  |  | hole | 72 |
| 89 | `eq` x2 |  | Y |  |  |  | Y | Y |  | 224&#8209;226 |

### Chap50/OptBinSearchTreeStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 90 | `new` | Y | Y |  |  | Y |  |  | hole | 57 |
| 91 | `from_keys_probs` | Y | Y |  |  | Y |  |  | hole | 58 |
| 92 | `from_key_probs` | Y | Y |  |  | Y |  |  | hole | 59 |
| 93 | `optimal_cost` | Y | Y |  |  | Y |  |  | hole | 60 |
| 94 | `keys` | Y | Y |  |  | Y |  | Y |  | 61 |
| 95 | `num_keys` | Y | Y |  |  | Y |  | Y |  | 62 |
| 96 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 63 |
| 97 | `obst_rec_st_per` |  |  |  | Y | Y |  |  | hole | 69 |
| 98 | `eq` x2 |  | Y |  |  |  | Y | Y |  | 195&#8209;197 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
