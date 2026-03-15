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
| 1 | Chap50 | MatrixChainMtEph | 13 | 14 | 0 | 0 | 14 | 0 | 12 | 0 | 2 |
| 2 | Chap50 | MatrixChainMtPer | 10 | 11 | 0 | 0 | 11 | 0 | 9 | 0 | 2 |
| 3 | Chap50 | MatrixChainStEph | 12 | 13 | 0 | 0 | 13 | 0 | 13 | 0 | 0 |
| 4 | Chap50 | MatrixChainStPer | 9 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 5 | Chap50 | OptBinSearchTreeMtEph | 10 | 11 | 0 | 1 | 12 | 0 | 10 | 0 | 2 |
| 6 | Chap50 | OptBinSearchTreeMtPer | 7 | 8 | 0 | 1 | 9 | 0 | 6 | 0 | 3 |
| 7 | Chap50 | OptBinSearchTreeStEph | 10 | 11 | 0 | 1 | 11 | 1 | 10 | 0 | 2 |
| 8 | Chap50 | OptBinSearchTreeStPer | 7 | 8 | 0 | 1 | 8 | 1 | 7 | 0 | 2 |

## Function-by-Function Detail

### Chap50/MatrixChainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;154 |
| 2 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;157 |
| 3 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;160 |
| 4 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;172 |
| 5 | `dimensions` | Y | Y |  |  | Y |  | Y |  | 174 |
| 6 | `set_dimension` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;178 |
| 7 | `update_dimension` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;182 |
| 8 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;186 |
| 9 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;190 |
| 10 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 192 |
| 11 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;203 |
| 12 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;214 |
| 13 | `parallel_min_reduction` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;220 |
| 14 | `eq` |  | Y |  |  | Y |  |  | unknown | 508&#8209;509 |

### Chap50/MatrixChainMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 15 | `new` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;146 |
| 16 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;149 |
| 17 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;152 |
| 18 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;162 |
| 19 | `dimensions` | Y | Y |  |  | Y |  | Y |  | 164 |
| 20 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;167 |
| 21 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 169 |
| 22 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;179 |
| 23 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;190 |
| 24 | `parallel_min_reduction` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;196 |
| 25 | `eq` |  | Y |  |  | Y |  |  | unknown | 420&#8209;421 |

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
| 38 | `eq` |  | Y |  |  | Y |  |  | unknown | 382&#8209;383 |

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
| 48 | `eq` |  | Y |  |  | Y |  |  | unknown | 351&#8209;352 |

### Chap50/OptBinSearchTreeMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 49 | `new` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 50 | `from_keys_probs` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 51 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 52 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 53 | `keys` | Y | Y |  |  | Y |  | Y |  | 117 |
| 54 | `set_key_prob` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 55 | `update_prob` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 56 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;129 |
| 57 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 58 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 135 |
| 59 | `obst_rec` |  |  |  | Y | Y |  |  | unknown | 140&#8209;145 |
| 60 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 370&#8209;371 |

### Chap50/OptBinSearchTreeMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 61 | `new` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 62 | `from_keys_probs` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 63 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;103 |
| 64 | `optimal_cost` | Y | Y |  |  | Y |  | Y |  | 105 |
| 65 | `keys` | Y | Y |  |  | Y |  | Y |  | 107 |
| 66 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 67 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 112 |
| 68 | `obst_rec` |  |  |  | Y | Y |  |  | unknown | 117&#8209;120 |
| 69 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 275&#8209;276 |

### Chap50/OptBinSearchTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 70 | `new` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 71 | `from_keys_probs` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;108 |
| 72 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;113 |
| 73 | `optimal_cost` | Y | Y |  |  | Y |  | Y |  | 115 |
| 74 | `keys` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 75 | `set_key_prob` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;124 |
| 76 | `update_prob` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;130 |
| 77 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;133 |
| 78 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;138 |
| 79 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;141 |
| 80 | `obst_rec_st_eph` |  |  |  | Y | Y |  |  | unknown | 146&#8209;153 |
| 81 | `eq` x2 |  | Y |  |  |  | Y | Y |  | 340&#8209;342 |

### Chap50/OptBinSearchTreeStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 82 | `new` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 83 | `from_keys_probs` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;108 |
| 84 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;113 |
| 85 | `optimal_cost` | Y | Y |  |  | Y |  | Y |  | 115 |
| 86 | `keys` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 87 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 88 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;124 |
| 89 | `obst_rec_st_per` |  |  |  | Y | Y |  |  | unknown | 129&#8209;136 |
| 90 | `eq` x2 |  | Y |  |  |  | Y | Y |  | 302&#8209;304 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
