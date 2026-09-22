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
| 1 | Chap50 | MatrixChainMtEph | 13 | 14 | 0 | 0 | 14 | 0 | 13 | 0 | 1 |
| 2 | Chap50 | MatrixChainMtPer | 10 | 11 | 0 | 0 | 11 | 0 | 10 | 0 | 1 |
| 3 | Chap50 | MatrixChainStEph | 12 | 13 | 0 | 0 | 13 | 0 | 13 | 0 | 0 |
| 4 | Chap50 | MatrixChainStPer | 9 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 5 | Chap50 | OptBinSearchTreeMtEph | 10 | 11 | 0 | 2 | 13 | 0 | 12 | 0 | 1 |
| 6 | Chap50 | OptBinSearchTreeMtPer | 7 | 8 | 0 | 2 | 10 | 0 | 9 | 0 | 1 |
| 7 | Chap50 | OptBinSearchTreeStEph | 10 | 11 | 0 | 1 | 11 | 1 | 10 | 0 | 2 |
| 8 | Chap50 | OptBinSearchTreeStPer | 7 | 8 | 0 | 1 | 8 | 1 | 7 | 0 | 2 |

## Function-by-Function Detail

### Chap50/MatrixChainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 2 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;168 |
| 3 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;172 |
| 4 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;185 |
| 5 | `dimensions` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;190 |
| 6 | `set_dimension` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;197 |
| 7 | `update_dimension` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;205 |
| 8 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;210 |
| 9 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;215 |
| 10 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 218 |
| 11 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;230 |
| 12 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;242 |
| 13 | `parallel_min_reduction` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;249 |
| 14 | `eq` |  | Y |  |  | Y |  |  | unknown | 623&#8209;624 |

### Chap50/MatrixChainMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 15 | `new` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;163 |
| 16 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;167 |
| 17 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;171 |
| 18 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;182 |
| 19 | `dimensions` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;186 |
| 20 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;190 |
| 21 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 193 |
| 22 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;204 |
| 23 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;216 |
| 24 | `parallel_min_reduction` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;223 |
| 25 | `eq` |  | Y |  |  | Y |  |  | unknown | 502&#8209;503 |

### Chap50/MatrixChainStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 26 | `new` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;180 |
| 27 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;187 |
| 28 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;194 |
| 29 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;205 |
| 30 | `dimensions` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;209 |
| 31 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;213 |
| 32 | `set_dimension` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;221 |
| 33 | `update_dimension` | Y | Y |  |  | Y |  |  | unknown | 224&#8209;230 |
| 34 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;237 |
| 35 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;241 |
| 36 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;252 |
| 37 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;268 |
| 38 | `eq` |  | Y |  |  | Y |  |  | unknown | 470&#8209;472 |

### Chap50/MatrixChainStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 39 | `new` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;179 |
| 40 | `from_dimensions` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;186 |
| 41 | `from_dim_pairs` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;193 |
| 42 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;203 |
| 43 | `dimensions` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;207 |
| 44 | `num_matrices` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;211 |
| 45 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;215 |
| 46 | `multiply_cost` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;226 |
| 47 | `matrix_chain_rec` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;242 |
| 48 | `eq` |  | Y |  |  | Y |  |  | unknown | 430&#8209;431 |

### Chap50/OptBinSearchTreeMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 49 | `new` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 50 | `from_keys_probs` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 51 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 52 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 53 | `keys` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;120 |
| 54 | `set_key_prob` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;127 |
| 55 | `update_prob` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;134 |
| 56 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 57 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;144 |
| 58 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 147 |
| 59 | `obst_rec` |  |  |  | Y | Y |  |  | unknown | 155&#8209;167 |
| 60 | `parallel_min_split_cost` |  |  |  | Y | Y |  |  | unknown | 210&#8209;227 |
| 61 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 545&#8209;546 |

### Chap50/OptBinSearchTreeMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 62 | `new` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 63 | `from_keys_probs` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 64 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;101 |
| 65 | `optimal_cost` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 66 | `keys` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 67 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 68 | `memo_size` | Y | Y |  |  | Y |  | Y |  | 116 |
| 69 | `obst_rec` |  |  |  | Y | Y |  |  | unknown | 124&#8209;136 |
| 70 | `parallel_min_split_cost` |  |  |  | Y | Y |  |  | unknown | 179&#8209;196 |
| 71 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 423&#8209;424 |

### Chap50/OptBinSearchTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 72 | `new` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 73 | `from_keys_probs` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;109 |
| 74 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;115 |
| 75 | `optimal_cost` | Y | Y |  |  | Y |  | Y |  | 118 |
| 76 | `keys` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;122 |
| 77 | `set_key_prob` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;129 |
| 78 | `update_prob` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;136 |
| 79 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;140 |
| 80 | `clear_memo` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;146 |
| 81 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;150 |
| 82 | `obst_rec_st_eph` |  |  |  | Y | Y |  |  | unknown | 245&#8209;250 |
| 83 | `eq` x2 |  | Y |  |  |  | Y | Y |  | 385&#8209;387 |

### Chap50/OptBinSearchTreeStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 84 | `new` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 85 | `from_keys_probs` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;109 |
| 86 | `from_key_probs` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;115 |
| 87 | `optimal_cost` | Y | Y |  |  | Y |  | Y |  | 118 |
| 88 | `keys` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;122 |
| 89 | `num_keys` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;126 |
| 90 | `memo_size` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;130 |
| 91 | `obst_rec_st_per` |  |  |  | Y | Y |  |  | unknown | 211&#8209;216 |
| 92 | `eq` x2 |  | Y |  |  |  | Y | Y |  | 342&#8209;344 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
