# r228 Alg Analysis Review: Chap11

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications (`prompts/Chap11.txt`)

| # | Item | Cost stated |
|---|---|---|
| 1 | Ex 11.1 fib via spawn/sync | none in prose |
| 2 | Ex 11.9 parallel fib (spawn/sync) | none in prose |
| 3 | Ex 11.10 parallel fib (SPARC `||`) | none in prose |
| 4 | Ex 11.11 sequential elision | none in prose |

The chapter defines threads, concurrency, parallelism, and races; it states
no Work or Span. The APAS lines in the files carry the standard costs of the
recursive definition (Work O(phi^n); Span O(n) with `||`, O(phi^n)
sequential). This review compares against those lines. The line
`APAS (Ch11 Ex 11.1): Work O(n), Span O(n)` on the iterative
`FibonacciStEph::fib` has no textbook source: the text only gives the
exponential recursive `fib`.

## 2. Reviewed functions

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 11 | FibonacciStEph.rs | fib | W n, S n [1] | W n, S n | W n, S n | matches textbook |
| 2 | 11 | FibonacciStEph.rs | fib_recursive | W phi^n, S phi^n | W phi^n, S phi^n | W phi^n, S phi^n | matches textbook |
| 3 | 11 | FibonacciMtEph2Threads.rs | fib_2threads | W phi^n, S n | W n, S n | W n, S n | not tb: iterative [2] |
| 4 | 11 | FibonacciMtEphRecomputes.rs | fib_recomputes | W phi^n, S n | W phi^n, S n | W phi^n, S n | matches textbook |
| 5 | 11 | FibonacciMtPerAllThreads.rs | fib | W phi^n, S n | W phi^n, S n | W phi^n, S n | matches textbook |
| 6 | 11 | FibonacciMtPerTSM.rs | fib | W phi^n, S n | W phi^n, S n | W phi^n, S n | matches textbook [3] |

Footnotes:

1. The APAS line's O(n) is not in the prose; the iterative loop is the
   project's own variant. Counted as a match against the line.
2. `fib_2threads` spawns exactly two threads, each running the iterative
   `FibonacciStEph::fib` on n-1 and n-2. Work O(n), Span O(n): less work than
   the recursive Ex 11.10 algorithm with the same span. It is not the
   textbook algorithm; the old line already recorded this.
3. `FibonacciMtPerTSM::fib` and `FibonacciMtEphRecomputes::fib_recomputes`
   call `vstd::thread::spawn` twice per internal node, creating O(phi^n) OS
   threads. The asymptotic bounds hold, but thread creation cost dominates in
   practice; `FibonacciMtPerAllThreads::fib` uses `ParaPair!` (HFScheduler
   `join`), which bounds the thread count.

## 3. Counts

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 5 |
| 2 | does not match textbook | 1 |
| 3 | does not match old analysis | 0 |
| 4 | no textbook cost | 0 |
| 5 | unannotated functions | 0 |
| 6 | malformed annotations | 0 |

The tokenized-state-machine transitions (`initialize`, `complete_left`,
`complete_right`, `finalize`) are ghost code, not exec functions, and are
not counted as unannotated.

## 4. Notable findings

- The Chap11 APAS lines cite costs the chapter prose does not state; the
  "Ex 11.1 Work O(n)" line on the iterative `fib` has no textbook source.
- `fib_2threads` is not Ex 11.10: it has two fixed threads over an O(n)
  iterative computation.
- Two variants spawn an OS thread per recursive call (O(phi^n) threads);
  only the `ParaPair!` variant goes through the bounded scheduler.
