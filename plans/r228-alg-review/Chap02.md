# r228 Alg Analysis Review: Chap02

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications (`prompts/Chap02.txt`)

| # | Item | Cost stated |
|---|---|---|
| 1 | Def 15.7 Greedy Scheduling Principle | T_P < W/P + S |
| 2 | Lower bound for any schedule | T_P >= max(W/P, S) |

The chapter states no per-function cost. The Fibonacci functions in
`FibonacciHFScheduler.rs` cite Ch11 Ex 11.1 (Work O(phi^n), Span O(n)).

## 2. Reviewed functions

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 02 | HFSchedulerMtEph.rs | init_pool | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 2 | 02 | HFSchedulerMtEph.rs | try_acquire | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 3 | 02 | HFSchedulerMtEph.rs | acquire | none | W 1 am., S 1 am. | W 1, S 1 + wait | no tb; old differs [1] |
| 4 | 02 | HFSchedulerMtEph.rs | release | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 5 | 02 | HFSchedulerMtEph.rs | set_parallelism | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 6 | 02 | HFSchedulerMtEph.rs | join | none | S max; else W sum | S max; else S sum | no tb; old differs [2] |
| 7 | 02 | HFSchedulerMtEph.rs | spawn_join | none | W sum, S max | W sum, S max | no textbook cost |
| 8 | 02 | HFSchedulerMtEph.rs | spawn | none | W W_f, S S_f | S 1 spawned, else S_f | no textbook cost |
| 9 | 02 | HFSchedulerMtEph.rs | wait | none | W 1, S S_task | W 1, S S_task | no textbook cost |
| 10 | 02 | FibonacciHFScheduler.rs | fib_seq | W phi^n, S phi^n | W phi^n, S phi^n | W phi^n, S phi^n | matches textbook |
| 11 | 02 | FibonacciHFScheduler.rs | fib_par | W phi^n, S n | W phi^n, S n | W phi^n, S n | matches textbook [3] |

Footnotes:

1. `acquire` loops on a condition variable until a slot is free. The wait is
   bounded by other tasks' progress, not amortized O(1). The function has no
   callers in this file.
2. When no pool slot is free, `join` runs `fa` then `fb` in the current
   thread. The span of that path is S_fa + S_fb, not W_fa + W_fb, because
   joins nested inside `fa` and `fb` still fork once slots free up.
3. `fib_par` switches to `fib_seq` for n <= 10. That cutoff adds only a
   constant, so the bounds hold, but it is a parallel/sequential threshold,
   which CLAUDE.md ("No Thread Threshold Optimization") disallows.

## 3. Counts

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 2 |
| 2 | does not match textbook | 0 |
| 3 | does not match old analysis | 2 |
| 4 | no textbook cost | 9 |
| 5 | unannotated functions | 4 |
| 6 | malformed annotations | 0 |

Unannotated: the four `fmt` impls (Debug and Display for `PoolState` and
`ExTaskState`) in `HFSchedulerMtEph.rs`.

## 4. Notable findings

- `fib_par` in `FibonacciHFScheduler.rs` has an n <= 10 sequential cutoff,
  contrary to the project rule against thresholds.
- The old `join` annotation overstated the sequential-fallback span as the
  total work.
- The help-first scheduler gives the stated span bounds only while pool slots
  are free; with P slots the greedy bound T_P < W/P + S is the relevant
  guarantee, not the unbounded-processor span.
