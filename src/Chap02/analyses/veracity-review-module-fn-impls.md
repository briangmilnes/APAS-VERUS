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
| 1 | Chap02 | FibonacciHFScheduler | 0 | 0 | 0 | 7 | 7 | 0 | 7 | 0 | 0 |
| 2 | Chap02 | HFSchedulerMtEph | 0 | 0 | 0 | 9 | 5 | 4 | 0 | 5 | 4 |

## Function-by-Function Detail

### Chap02/FibonacciHFScheduler.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_pow2_mono` |  |  |  | Y | Y |  |  | unknown | 44&#8209;46 |
| 2 | `lemma_pow2_46_lt_u64_max` |  |  |  | Y | Y |  |  | unknown | 51&#8209;52 |
| 3 | `lemma_fib_bound` |  |  |  | Y | Y |  |  | unknown | 57&#8209;59 |
| 4 | `lemma_fib_fits_u64` |  |  |  | Y | Y |  |  | unknown | 75&#8209;77 |
| 5 | `lemma_fib_sum_fits_u64` |  |  |  | Y | Y |  |  | unknown | 84&#8209;86 |
| 6 | `fib_seq` |  |  |  | Y | Y |  |  | unknown | 96&#8209;99 |
| 7 | `fib_par` |  |  |  | Y | Y |  |  | unknown | 112&#8209;115 |

### Chap02/HFSchedulerMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 8 | `set_parallelism` |  |  |  | Y | Y |  |  | hole | 110 |
| 9 | `join` |  |  |  | Y | Y |  |  | hole | 119&#8209;130 |
| 10 | `spawn_join` |  |  |  | Y | Y |  |  | hole | 145&#8209;156 |
| 11 | `spawn` |  |  |  | Y | Y |  |  | hole | 175&#8209;182 |
| 12 | `wait` |  |  |  | Y | Y |  |  | hole | 196&#8209;198 |
| 13 | `init_pool` |  |  |  | Y |  | Y | Y |  | 45&#8209;58 |
| 14 | `try_acquire` |  |  |  | Y |  | Y | Y |  | 62&#8209;71 |
| 15 | `acquire` |  |  |  | Y |  | Y | Y |  | 73&#8209;80 |
| 16 | `release` |  |  |  | Y |  | Y | Y |  | 82&#8209;87 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
