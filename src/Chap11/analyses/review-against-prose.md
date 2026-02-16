<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chap11 Review Against Prose

**Reviewer:** Claude-Opus-4.6
**Date:** 2026-02-16
**Prose file:** `prompts/Chap11.txt`
**Source files:** `FibonacciStEph.rs`, `FibonacciMtPerAllThreads.rs`, `FibonacciMtPerTSM.rs`, `FibonacciMtEph2Threads.rs`, `FibonacciMtEphRecomputes.rs`

## Prose Inventory

Chapter 11 covers threads, concurrency, and parallelism. Mostly conceptual —
definitions, examples, and discussion. No cost tables.

| # | Item | Type |
|---|------|------|
| 1 | Def 11.1: Thread — spawn and sync operations | Definition |
| 2 | Ex 11.1: Spawn two threads to compute fib(n) and fib(2n) | Example |
| 3 | Ex 11.2: Threads communicating via references | Example |
| 4 | Def 11.2: Thread Scheduler | Definition |
| 5 | Ex 11.3: One-processor schedule | Example |
| 6 | Ex 11.4: Two-processor schedule | Example |
| 7 | Def 11.3: Concurrency — problem property | Definition |
| 8 | Def 11.4: Parallelism — solution property | Definition |
| 9 | Ex 11.9: Parallel Fibonacci (spawn/sync style) | Algorithm |
| 10 | Ex 11.10: Parallel Fibonacci in SPARC (par operator) | Algorithm |
| 11 | Def 11.5: Sequential Elision — replace par with seq pairs | Definition |
| 12 | Ex 11.11: Sequential elision of parallel fib | Example |
| 13 | Def 11.6: Data Race | Definition |
| 14 | Ex 11.12-11.14: Concurrent writes, concurrent additions, races | Examples |

The key algorithms are Examples 11.9 and 11.10: parallel Fibonacci via
spawn/sync and via SPARC's parallel pairs. Both have Work Theta(phi^n),
Span Theta(n).

## Code Inventory

| # | File | Lines | Parallel? | Proof holes | Notes |
|---|------|-------|-----------|-------------|-------|
| 1 | FibonacciStEph.rs | 145 | No | 0 — clean | Sequential: iterative fib + recursive fib_recursive, 3 proof fns |
| 2 | FibonacciMtPerAllThreads.rs | 57 | Yes — ParaPair! | 0 — clean | Fully verified parallel fib via ParaPair!, recursive |
| 3 | FibonacciMtPerTSM.rs | 169 | Yes — TSM + vstd::thread | 2 (assume(false) in join error arms) | Parallel fib using Tokenized State Machine |
| 4 | FibonacciMtEph2Threads.rs | ~170 | Yes — TSM + vstd::thread | 2 (assume(false) in join error arms) | Two-thread fib using TSM, top-level split only |
| 5 | FibonacciMtEphRecomputes.rs | ~170 | Yes — TSM + vstd::thread | 2 (assume(false) in join error arms) | Recursive parallel fib using TSM at every level |

## Prose-to-Code Mapping

| # | Prose Item | Code | Spec Fidelity |
|---|-----------|------|---------------|
| 1 | fib definition (Ex 11.1) | spec_fib in FibonacciStEph | Strong — standard recursive definition |
| 2 | Sequential fib | fib() in FibonacciStEph | Strong — iterative, ensures result == spec_fib(n), n <= 46 for u64 |
| 3 | Recursive fib | fib_recursive() in FibonacciStEph | Strong — matches prose structure exactly |
| 4 | Ex 11.9: Parallel fib (spawn/sync) | FibonacciMtEph2Threads, FibonacciMtEphRecomputes | Strong — TSM tracks fork-join, ensures result == spec_fib(n) |
| 5 | Ex 11.10: Parallel fib (SPARC par) | FibonacciMtPerAllThreads | Strong — ParaPair! is the SPARC `\|\|` operator, fully verified |
| 6 | Def 11.1: spawn/sync | vstd::thread::spawn + JoinHandle::join | Used in TSM variants |
| 7 | Def 11.2: Thread Scheduler | HFSchedulerMtEph (Chap02) | Not directly referenced by Chap11 code |
| 8 | Def 11.5: Sequential Elision | FibonacciStEph::fib_recursive is the sequential elision of FibonacciMtPerAllThreads::fib | Structural match |

## Cost Analysis

| # | Function | APAS | Claude-Opus-4.6 | Notes |
|---|----------|------|-----------------|-------|
| 1 | fib (iterative) | — | Work Theta(n), Span Theta(n) | Not in prose; standard iterative fib |
| 2 | fib_recursive | Work Theta(phi^n), Span Theta(phi^n) | Agrees | Sequential, so span == work |
| 3 | FibonacciMtPerAllThreads::fib | Work Theta(phi^n), Span Theta(n) | Agrees | ParaPair! provides true parallel recursion at every level |
| 4 | FibonacciMtPerTSM::fib | Work Theta(phi^n), Span Theta(n) | Agrees | TSM + vstd::thread, recursive parallelism |
| 5 | FibonacciMtEph2Threads::fib_2threads | Work Theta(phi^n), Span Theta(phi^n) | **Disagrees with Theta(n)** | Top-level split only; each thread calls fib_recursive (sequential). Span == Work. |
| 6 | FibonacciMtEphRecomputes::fib_recomputes | Work Theta(phi^n), Span Theta(n) | Agrees | TSM at every recursive level, true recursive parallelism |

Row 5 is the same pattern seen in Chap02's `fib_par` — only the top-level
call is parallel, so the span is not improved.

## Parallelism Review

| # | Module | Mechanism | Parallel? | Span | Notes |
|---|--------|-----------|-----------|------|-------|
| 1 | FibonacciStEph | None | No | Theta(n) iterative / Theta(phi^n) recursive | Sequential baseline |
| 2 | FibonacciMtPerAllThreads | ParaPair! at every level | Yes — full | Theta(n) | Gold standard: fully verified, no proof holes |
| 3 | FibonacciMtPerTSM | TSM + vstd::thread at every level | Yes — full | Theta(n) | Fully parallel, 2 assume(false) in join error arms |
| 4 | FibonacciMtEph2Threads | TSM + vstd::thread, top level only | Partial | Theta(phi^n) | Two threads, each runs sequential fib |
| 5 | FibonacciMtEphRecomputes | TSM + vstd::thread at every level | Yes — full | Theta(n) | Fully parallel, 2 assume(false) in join error arms |

FibonacciMtPerAllThreads is the cleanest implementation — zero proof holes,
full recursive parallelism via ParaPair!, and the simplest code. It directly
implements Example 11.10 (SPARC parallel pairs).

## Runtime Test Review

| # | Source module | RTT file | PTT file | Status |
|---|-------------|----------|----------|--------|
| 1 | FibonacciStEph | TestFibonacciMt.rs | — | Missing PTT |
| 2 | FibonacciMtPerAllThreads | TestFibonacciMt.rs | — | Missing PTT |
| 3 | FibonacciMtPerTSM | TestFibonacciMt.rs | — | Missing PTT |
| 4 | FibonacciMtEph2Threads | TestFibonacciMt.rs | — | Missing PTT |
| 5 | FibonacciMtEphRecomputes | TestFibonacciMt.rs | — | Missing PTT |
| 6 | ParaPairs (module-level) | TestParaPairs.rs | — | Missing PTT |

All 5 Fibonacci modules share a single RTT file (`TestFibonacciMt.rs`).
No PTT files exist for Chap11 — the `rust_verify_test/tests/Chap11/`
directory does not exist.

### Iterator/Loop Coverage

FibonacciStEph uses a `while` loop (iterative fib). No iterators. No PTT
tests this loop form. The parallel modules use recursion, not loops.

## Gap Analysis

**Prose items with no implementation:**

| # | Prose item | Notes |
|---|-----------|-------|
| 1 | Def 11.2: Thread Scheduler | Implemented in Chap02 (HFSchedulerMtEph), not Chap11 |
| 2 | Def 11.3: Concurrency | Conceptual definition, no code needed |
| 3 | Def 11.4: Parallelism | Conceptual definition, no code needed |
| 4 | Def 11.5: Sequential Elision | Structural relationship, not an algorithm |
| 5 | Def 11.6: Data Race | Safety property, not an algorithm |
| 6 | Ex 11.1-11.2: Spawn fib(n) and fib(2n) | Partially covered by fib_2threads (spawns fib(n-1) and fib(n-2) instead) |

**Code with no prose counterpart:**

- `spec_fib` spec function and lemmas (`lemma_fib_bound`, `lemma_fib_fits_u64`, `lemma_fib_sum_fits_u64`) — Verus proof infrastructure for u64 overflow bounds
- `fib` iterative implementation — more efficient than the prose's recursive form
- ParaPair! macro infrastructure — abstraction over SPARC's parallel pairs
- TSM (Tokenized State Machine) machinery — Verus-specific concurrency proof technique
- `ParaPairs` module — general parallel pair abstraction used by Chap06 Mt modules too

## Proof Holes

| # | File | Hole | Count | Justification |
|---|------|------|-------|---------------|
| 1 | FibonacciMtPerTSM.rs | assume(false) in join Err arm | 2 | Thread join cannot fail in practice; Verus requires handling Result::Err |
| 2 | FibonacciMtEph2Threads.rs | assume(false) in join Err arm | 2 | Same pattern |
| 3 | FibonacciMtEphRecomputes.rs | assume(false) in join Err arm | 2 | Same pattern |

All 6 `assume(false)` instances are in `JoinHandle::join()` error match arms.
In Rust, `thread::join()` returns `Result<T, Box<dyn Any>>` where `Err` means
the thread panicked. Verus cannot prove threads don't panic (that would require
verifying the absence of all panics in the spawned closure's transitive call
graph). The `assume(false)` says "this branch is unreachable" — a pragmatic
assumption since verified code should not panic.

FibonacciMtPerAllThreads avoids this entirely by using `ParaPair!` which
encapsulates the spawn/join pattern and handles the error arms internally.

## Summary

Chap11 is the threads and parallelism chapter. The code provides five Fibonacci
implementations spanning the spectrum from sequential to fully parallel:

1. **FibonacciStEph** — sequential baseline (iterative + recursive), fully clean.
2. **FibonacciMtPerAllThreads** — the gold standard: fully verified parallel
   recursion via ParaPair!, zero proof holes, directly implements Ex 11.10.
3. **FibonacciMtPerTSM** — parallel recursion using TSM, demonstrates the
   Verus concurrency proof technique at every recursive level.
4. **FibonacciMtEph2Threads** — top-level parallel split only (span == work).
5. **FibonacciMtEphRecomputes** — full recursive parallelism using TSM.

Key findings:
- The ParaPair! abstraction (module 2) eliminates all join-error proof holes
  and produces the cleanest code. It is the recommended pattern.
- Three TSM modules have 2 assume(false) each (6 total), all in join error
  arms — a known Verus limitation with thread join.
- No PTT files exist for Chap11. This is the biggest gap.
- FibonacciMtEph2Threads has the same span issue as Chap02's fib_par: only
  top-level parallelism, so Span == Work despite APAS expecting Theta(n).
