# Chap11 Spec Audit — Fibonacci

## Summary

All exec functions have **strong** specifications.

## Per-Function Classification

| # | File | Function | requires | ensures | Classification |
|---|------|----------|----------|---------|----------------|
| 1 | FibonacciStEph.rs | fib | n <= 46 | fibonacci == spec_fib(n as nat) | **strong** |
| 2 | FibonacciStEph.rs | fib_recursive | n <= 46 | fibonacci == spec_fib(n as nat) | **strong** |
| 3 | FibonacciStEph.rs | lemma_fib_bound | — | spec_fib(n) < pow2(n) | **strong** |
| 4 | FibonacciStEph.rs | lemma_fib_fits_u64 | n <= 46 | spec_fib(n) <= u64::MAX | **strong** |
| 5 | FibonacciStEph.rs | lemma_fib_sum_fits_u64 | 2 <= n <= 46 | spec_fib(n-1)+spec_fib(n-2) <= u64::MAX | **strong** |

## Notes

- Exact functional correctness: result == spec_fib(n).
- Overflow bound n <= 46 is textbook-faithful for u64.
- Mt variants (FibonacciMtEph2Threads, FibonacciMtEphRecomputes, FibonacciMtPerAllThreads, FibonacciMtPerTSM) all ensure fibonacci == spec_fib(n as nat).
