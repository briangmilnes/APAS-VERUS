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
| 1 | Chap50 | MatrixChainMtEph | 13 | 14 | 0 | 0 | 14 | 0 | 11 | 1 | 2 |
| 2 | Chap50 | MatrixChainMtPer | 10 | 11 | 0 | 0 | 11 | 0 | 8 | 1 | 2 |
| 3 | Chap50 | MatrixChainStEph | 12 | 13 | 0 | 0 | 13 | 0 | 12 | 1 | 0 |
| 4 | Chap50 | MatrixChainStPer | 9 | 10 | 0 | 0 | 10 | 0 | 9 | 1 | 0 |
| 5 | Chap50 | OptBinSearchTreeMtEph | 10 | 11 | 0 | 1 | 12 | 0 | 9 | 1 | 2 |
| 6 | Chap50 | OptBinSearchTreeMtPer | 7 | 8 | 0 | 1 | 9 | 0 | 5 | 1 | 3 |
| 7 | Chap50 | OptBinSearchTreeStEph | 10 | 11 | 0 | 1 | 11 | 1 | 10 | 0 | 2 |
| 8 | Chap50 | OptBinSearchTreeStPer | 7 | 8 | 0 | 1 | 8 | 1 | 7 | 0 | 2 |

## Function-by-Function Detail

### Chap50/MatrixChainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;153 |
| 2 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;156 |
| 3 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;159 |
| 4 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;171 |
| 5 | `dimensions` | Y | Y |  |  | Y |  | Y |  | 173 |
| 6 | `set_dimension` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;177 |
| 7 | `update_dimension` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;181 |
| 8 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;185 |
| 9 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;189 |
| 10 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 191 |
| 11 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;202 |
| 12 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;213 |
| 13 | `parallel_min_reduction` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;219 |
| 14 | `eq` |  | Y |  |  | Y |  |  | hole | 507&#8209;508 |

### Chap50/MatrixChainMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 15 | `new` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;145 |
| 16 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;148 |
| 17 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;151 |
| 18 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;161 |
| 19 | `dimensions` | Y | Y |  |  | Y |  | Y |  | 163 |
| 20 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;166 |
| 21 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 168 |
| 22 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;178 |
| 23 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;189 |
| 24 | `parallel_min_reduction` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;195 |
| 25 | `eq` |  | Y |  |  | Y |  |  | hole | 419&#8209;420 |

### Chap50/MatrixChainStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 26 | `new` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;138 |
| 27 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;143 |
| 28 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;148 |
| 29 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;158 |
| 30 | `dimensions` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;161 |
| 31 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 32 | `set_dimension` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;170 |
| 33 | `update_dimension` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;177 |
| 34 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;182 |
| 35 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;185 |
| 36 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;195 |
| 37 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;210 |
| 38 | `eq` |  | Y |  |  | Y |  |  | hole | 381&#8209;382 |

### Chap50/MatrixChainStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 39 | `new` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;138 |
| 40 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;143 |
| 41 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;148 |
| 42 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;157 |
| 43 | `dimensions` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;160 |
| 44 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;163 |
| 45 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;166 |
| 46 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;176 |
| 47 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;191 |
| 48 | `eq` |  | Y |  |  | Y |  |  | hole | 350&#8209;351 |

### Chap50/OptBinSearchTreeMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 49 | `new` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 50 | `from_keys_probs` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;108 |
| 51 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 52 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;114 |
| 53 | `keys` | Y | Y |  |  | Y |  | Y |  | 116 |
| 54 | `set_key_prob` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;120 |
| 55 | `update_prob` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 56 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 57 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;132 |
| 58 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 134 |
| 59 | `obst_rec` |  |  |  | Y | Y |  |  | unknown | 139&#8209;144 |
| 60 | `eq` x2 |  | Y |  |  | Y |  |  | hole | 369&#8209;370 |

### Chap50/OptBinSearchTreeMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 61 | `new` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;95 |
| 62 | `from_keys_probs` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 63 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 64 | `optimal_cost` | Y | Y |  |  | Y |  | Y |  | 104 |
| 65 | `keys` | Y | Y |  |  | Y |  | Y |  | 106 |
| 66 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 67 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 111 |
| 68 | `obst_rec` |  |  |  | Y | Y |  |  | unknown | 116&#8209;119 |
| 69 | `eq` x2 |  | Y |  |  | Y |  |  | hole | 274&#8209;275 |

### Chap50/OptBinSearchTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 70 | `new` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;101 |
| 71 | `from_keys_probs` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;107 |
| 72 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;112 |
| 73 | `optimal_cost` | Y | Y |  |  | Y |  | Y |  | 114 |
| 74 | `keys` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 75 | `set_key_prob` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;123 |
| 76 | `update_prob` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;129 |
| 77 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;132 |
| 78 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;137 |
| 79 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;140 |
| 80 | `obst_rec_st_eph` |  |  |  | Y | Y |  |  | unknown | 145&#8209;152 |
| 81 | `eq` x2 |  | Y |  |  |  | Y | Y |  | 339&#8209;341 |

### Chap50/OptBinSearchTreeStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 82 | `new` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;101 |
| 83 | `from_keys_probs` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;107 |
| 84 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;112 |
| 85 | `optimal_cost` | Y | Y |  |  | Y |  | Y |  | 114 |
| 86 | `keys` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 87 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 88 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 89 | `obst_rec_st_per` |  |  |  | Y | Y |  |  | unknown | 128&#8209;135 |
| 90 | `eq` x2 |  | Y |  |  |  | Y | Y |  | 301&#8209;303 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
