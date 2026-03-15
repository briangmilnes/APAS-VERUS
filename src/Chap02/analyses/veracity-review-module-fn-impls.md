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
| 1 | `lemma_pow2_mono` |  |  |  | Y | Y |  |  | unknown | 24&#8209;26 |
| 2 | `lemma_pow2_46_lt_u64_max` |  |  |  | Y | Y |  |  | unknown | 31&#8209;32 |
| 3 | `lemma_fib_bound` |  |  |  | Y | Y |  |  | unknown | 37&#8209;39 |
| 4 | `lemma_fib_fits_u64` |  |  |  | Y | Y |  |  | unknown | 54&#8209;56 |
| 5 | `lemma_fib_sum_fits_u64` |  |  |  | Y | Y |  |  | unknown | 63&#8209;65 |
| 6 | `fib_seq` |  |  |  | Y | Y |  |  | unknown | 72&#8209;75 |
| 7 | `fib_par` |  |  |  | Y | Y |  |  | unknown | 87&#8209;90 |

### Chap02/HFSchedulerMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 8 | `set_parallelism` |  |  |  | Y | Y |  |  | hole | 88 |
| 9 | `join` |  |  |  | Y | Y |  |  | hole | 98&#8209;109 |
| 10 | `spawn_join` |  |  |  | Y | Y |  |  | hole | 125&#8209;136 |
| 11 | `spawn` |  |  |  | Y | Y |  |  | hole | 156&#8209;163 |
| 12 | `wait` |  |  |  | Y | Y |  |  | hole | 178&#8209;180 |
| 13 | `init_pool` |  |  |  | Y |  | Y | Y |  | 33&#8209;45 |
| 14 | `try_acquire` |  |  |  | Y |  | Y | Y |  | 49&#8209;57 |
| 15 | `acquire` |  |  |  | Y |  | Y | Y |  | 59&#8209;65 |
| 16 | `release` |  |  |  | Y |  | Y | Y |  | 67&#8209;71 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
