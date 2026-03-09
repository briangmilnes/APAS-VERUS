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
| 1 | Chap50 | MatrixChainMtEph | 13 | 14 | 0 | 0 | 14 | 0 | 0 | 14 | 0 |
| 2 | Chap50 | MatrixChainMtPer | 10 | 11 | 0 | 0 | 11 | 0 | 2 | 8 | 1 |
| 3 | Chap50 | MatrixChainStEph | 12 | 13 | 0 | 0 | 13 | 0 | 12 | 1 | 0 |
| 4 | Chap50 | MatrixChainStPer | 9 | 10 | 0 | 0 | 10 | 0 | 9 | 1 | 0 |
| 5 | Chap50 | OptBinSearchTreeMtEph | 10 | 11 | 0 | 2 | 13 | 0 | 0 | 13 | 0 |
| 6 | Chap50 | OptBinSearchTreeMtPer | 7 | 8 | 0 | 2 | 10 | 0 | 0 | 10 | 0 |
| 7 | Chap50 | OptBinSearchTreeStEph | 10 | 11 | 0 | 1 | 11 | 1 | 7 | 4 | 1 |
| 8 | Chap50 | OptBinSearchTreeStPer | 7 | 8 | 0 | 1 | 8 | 1 | 5 | 3 | 1 |

## Function-by-Function Detail

### Chap50/MatrixChainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` | Y | Y |  |  | Y |  |  | hole | 133 |
| 2 | `from_dimensions` | Y | Y |  |  | Y |  |  | hole | 134 |
| 3 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | hole | 135 |
| 4 | `optimal_cost` | Y | Y |  |  | Y |  |  | hole | 136 |
| 5 | `dimensions` | Y | Y |  |  | Y |  |  | hole | 137 |
| 6 | `set_dimension` | Y | Y |  |  | Y |  |  | hole | 138 |
| 7 | `update_dimension` | Y | Y |  |  | Y |  |  | hole | 139 |
| 8 | `num_matrices` | Y | Y |  |  | Y |  |  | hole | 140 |
| 9 | `clear_memo` | Y | Y |  |  | Y |  |  | hole | 141 |
| 10 | `memo_size` | Y | Y |  |  | Y |  |  | hole | 142 |
| 11 | `multiply_cost` | Y | Y |  |  | Y |  |  | hole | 143 |
| 12 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | hole | 144 |
| 13 | `parallel_min_reduction` | Y | Y |  |  | Y |  |  | hole | 145&#8209;149 |
| 14 | `eq` |  | Y |  |  | Y |  |  | hole | 348 |

### Chap50/MatrixChainMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 15 | `new` | Y | Y |  |  | Y |  |  | hole | 143&#8209;144 |
| 16 | `from_dimensions` | Y | Y |  |  | Y |  |  | hole | 146&#8209;147 |
| 17 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | hole | 149&#8209;150 |
| 18 | `optimal_cost` | Y | Y |  |  | Y |  |  | hole | 152&#8209;159 |
| 19 | `dimensions` | Y | Y |  |  | Y |  | Y |  | 161 |
| 20 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 21 | `memo_size` | Y | Y |  |  | Y |  |  | hole | 166 |
| 22 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;176 |
| 23 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | hole | 178&#8209;185 |
| 24 | `parallel_min_reduction` | Y | Y |  |  | Y |  |  | hole | 187&#8209;191 |
| 25 | `eq` |  | Y |  |  | Y |  |  | hole | 351&#8209;352 |

### Chap50/MatrixChainStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 26 | `new` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;139 |
| 27 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;144 |
| 28 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;149 |
| 29 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;159 |
| 30 | `dimensions` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;162 |
| 31 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;165 |
| 32 | `set_dimension` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;171 |
| 33 | `update_dimension` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;178 |
| 34 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;183 |
| 35 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;186 |
| 36 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;196 |
| 37 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;211 |
| 38 | `eq` |  | Y |  |  | Y |  |  | hole | 382&#8209;383 |

### Chap50/MatrixChainStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 39 | `new` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;139 |
| 40 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;144 |
| 41 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;149 |
| 42 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;158 |
| 43 | `dimensions` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;161 |
| 44 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 45 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;167 |
| 46 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;177 |
| 47 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;192 |
| 48 | `eq` |  | Y |  |  | Y |  |  | hole | 351&#8209;352 |

### Chap50/OptBinSearchTreeMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 49 | `new` | Y | Y |  |  | Y |  |  | hole | 71 |
| 50 | `from_keys_probs` | Y | Y |  |  | Y |  |  | hole | 72 |
| 51 | `from_key_probs` | Y | Y |  |  | Y |  |  | hole | 73 |
| 52 | `optimal_cost` | Y | Y |  |  | Y |  |  | hole | 74 |
| 53 | `keys` | Y | Y |  |  | Y |  |  | hole | 75 |
| 54 | `set_key_prob` | Y | Y |  |  | Y |  |  | hole | 76 |
| 55 | `update_prob` | Y | Y |  |  | Y |  |  | hole | 77 |
| 56 | `num_keys` | Y | Y |  |  | Y |  |  | hole | 78 |
| 57 | `clear_memo` | Y | Y |  |  | Y |  |  | hole | 79 |
| 58 | `memo_size` | Y | Y |  |  | Y |  |  | hole | 80 |
| 59 | `parallel_min_reduction` |  |  |  | Y | Y |  |  | hole | 86 |
| 60 | `obst_rec` |  |  |  | Y | Y |  |  | hole | 109 |
| 61 | `eq` x2 |  | Y |  |  | Y |  |  | hole | 258 |

### Chap50/OptBinSearchTreeMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 62 | `new` | Y | Y |  |  | Y |  |  | hole | 66 |
| 63 | `from_keys_probs` | Y | Y |  |  | Y |  |  | hole | 67 |
| 64 | `from_key_probs` | Y | Y |  |  | Y |  |  | hole | 68 |
| 65 | `optimal_cost` | Y | Y |  |  | Y |  |  | hole | 69 |
| 66 | `keys` | Y | Y |  |  | Y |  |  | hole | 70 |
| 67 | `num_keys` | Y | Y |  |  | Y |  |  | hole | 71 |
| 68 | `memo_size` | Y | Y |  |  | Y |  |  | hole | 72 |
| 69 | `parallel_min_reduction` |  |  |  | Y | Y |  |  | hole | 78 |
| 70 | `obst_rec` |  |  |  | Y | Y |  |  | hole | 101 |
| 71 | `eq` x2 |  | Y |  |  | Y |  |  | hole | 198 |

### Chap50/OptBinSearchTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 72 | `new` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;96 |
| 73 | `from_keys_probs` | Y | Y |  |  | Y |  |  | hole | 98&#8209;102 |
| 74 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;107 |
| 75 | `optimal_cost` | Y | Y |  |  | Y |  |  | hole | 109 |
| 76 | `keys` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 77 | `set_key_prob` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 78 | `update_prob` | Y | Y |  |  | Y |  |  | hole | 120&#8209;124 |
| 79 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;127 |
| 80 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;132 |
| 81 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;135 |
| 82 | `obst_rec_st_eph` |  |  |  | Y | Y |  |  | hole | 141 |
| 83 | `eq` x2 |  | Y |  |  |  | Y | Y |  | 288&#8209;290 |

### Chap50/OptBinSearchTreeStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 84 | `new` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;96 |
| 85 | `from_keys_probs` | Y | Y |  |  | Y |  |  | hole | 98&#8209;102 |
| 86 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;107 |
| 87 | `optimal_cost` | Y | Y |  |  | Y |  |  | hole | 109 |
| 88 | `keys` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 89 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 90 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 91 | `obst_rec_st_per` |  |  |  | Y | Y |  |  | hole | 124 |
| 92 | `eq` x2 |  | Y |  |  |  | Y | Y |  | 250&#8209;252 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
