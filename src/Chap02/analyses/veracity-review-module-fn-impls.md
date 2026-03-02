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
| 1 | `lemma_pow2_mono` |  |  |  | Y | Y |  |  | unknown | 23&#8209;25 |
| 2 | `lemma_pow2_46_lt_u64_max` |  |  |  | Y | Y |  |  | unknown | 30&#8209;31 |
| 3 | `lemma_fib_bound` |  |  |  | Y | Y |  |  | unknown | 36&#8209;38 |
| 4 | `lemma_fib_fits_u64` |  |  |  | Y | Y |  |  | unknown | 53&#8209;55 |
| 5 | `lemma_fib_sum_fits_u64` |  |  |  | Y | Y |  |  | unknown | 62&#8209;64 |
| 6 | `fib_seq` |  |  |  | Y | Y |  |  | unknown | 71&#8209;74 |
| 7 | `fib_par` |  |  |  | Y | Y |  |  | unknown | 86&#8209;89 |

### Chap02/HFSchedulerMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 8 | `set_parallelism` |  |  |  | Y | Y |  |  | hole | 87 |
| 9 | `join` |  |  |  | Y | Y |  |  | hole | 97&#8209;108 |
| 10 | `spawn_join` |  |  |  | Y | Y |  |  | hole | 124&#8209;135 |
| 11 | `spawn` |  |  |  | Y | Y |  |  | hole | 155&#8209;162 |
| 12 | `wait` |  |  |  | Y | Y |  |  | hole | 177&#8209;179 |
| 13 | `init_pool` |  |  |  | Y |  | Y | Y |  | 32&#8209;44 |
| 14 | `try_acquire` |  |  |  | Y |  | Y | Y |  | 48&#8209;56 |
| 15 | `acquire` |  |  |  | Y |  | Y | Y |  | 58&#8209;64 |
| 16 | `release` |  |  |  | Y |  | Y | Y |  | 66&#8209;70 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
