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
| 1 | Chap11 | FibonacciMtEph2Threads | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 2 | Chap11 | FibonacciMtEphRecomputes | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 3 | Chap11 | FibonacciMtPerAllThreads | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 4 | Chap11 | FibonacciMtPerTSM | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 5 | Chap11 | FibonacciStEph | 0 | 0 | 0 | 5 | 5 | 0 | 5 | 0 | 0 |

## Function-by-Function Detail

### Chap11/FibonacciMtEph2Threads.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `fib_2threads` |  |  |  | Y | Y |  |  | unknown | 101&#8209;103 |

### Chap11/FibonacciMtEphRecomputes.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 2 | `fib_recomputes` |  |  |  | Y | Y |  |  | unknown | 93&#8209;96 |

### Chap11/FibonacciMtPerAllThreads.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3 | `fib` |  |  |  | Y | Y |  |  | unknown | 21&#8209;26 |

### Chap11/FibonacciMtPerTSM.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 4 | `fib` |  |  |  | Y | Y |  |  | unknown | 87&#8209;90 |

### Chap11/FibonacciStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `lemma_fib_bound` |  |  |  | Y | Y |  |  | unknown | 48&#8209;50 |
| 6 | `lemma_fib_fits_u64` |  |  |  | Y | Y |  |  | unknown | 66&#8209;68 |
| 7 | `lemma_fib_sum_fits_u64` |  |  |  | Y | Y |  |  | unknown | 76&#8209;78 |
| 8 | `fib` |  |  |  | Y | Y |  |  | unknown | 89&#8209;93 |
| 9 | `fib_recursive` |  |  |  | Y | Y |  |  | unknown | 127&#8209;132 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
