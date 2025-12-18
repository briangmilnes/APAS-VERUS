# Chapter 11 — Parallel Fibonacci

Verified implementations of the Fibonacci function demonstrating parallel recursion patterns from APAS.

## Files

### `FibonacciStEph.rs` — Sequential Baseline
- `spec_fib(n)` — Specification function
- `fib(n)` — Iterative O(n) implementation  
- `fib_recursive(n)` — Recursive implementation
- Proves overflow safety for n ≤ 46

### `FibonacciMtPerAllThreads.rs` — Parallel (Macro)
- Uses `ParaPair!` macro for symmetric binary parallelism
- Spawns threads at every recursive call
- Work Θ(φⁿ), Span Θ(n)

### `FibonacciMtPerTSM.rs` — Parallel (TSM)
- Tokenized State Machine at each fork-join
- Tracks left/right completion via ghost tokens
- No intermediate values stored

### `FibonacciMtEph2Threads.rs` — Two Threads (TSM)
- TSM only at top level
- Spawns exactly 2 threads for fib(n-1) and fib(n-2)
- Sequential recursion within each thread

### `FibonacciMtEphRecomputes.rs` — Full Parallel (TSM)
- TSM at every recursive level
- Maximum parallelism with token tracking
- Equivalent to `FibonacciMtPerTSM`

## Verification

All implementations verified against `spec_fib`:
```
spec fn spec_fib(n: nat) -> nat {
    if n <= 1 { n } else { spec_fib(n-1) + spec_fib(n-2) }
}
```

Overflow safety proven for u64 up to n = 46.

