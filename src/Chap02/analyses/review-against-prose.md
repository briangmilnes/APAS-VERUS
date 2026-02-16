<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chap02 Review Against Prose

**Reviewer:** Claude-Opus-4.6
**Date:** 2026-02-15
**Prose file:** `prompts/Chap02.txt`
**Source files:** `FibonacciWSScheduler.rs`, `HFSchedulerMtEph.rs` (renamed from `WSSchedulerMtEph.rs` — "HF" = help-first, reflecting the actual scheduling policy)

## Prose Inventory

The prose covers sections 2.2.1-2.2.2 of APAS (scheduling theory):

| # | Item | Type |
|---|------|------|
| 1 | Def 15.5: Scheduler — maps parallel tasks to processors | Definition |
| 2 | Def 15.6: Greedy Scheduler — assigns ready tasks immediately | Definition |
| 3 | Def 15.7: Greedy Scheduling Principle — TP < W/P + S | Theorem |
| 4 | Optimality of greedy schedulers — TP >= max(W/P, S) | Bound |
| 5 | Parallelism P = W/S and effective processor utilization | Analysis |

No algorithms or pseudocode. No cost tables. The prose is theoretical
background for the work-span model.

## Code Inventory

| # | File | Lines | Functions | Proof holes |
|---|------|-------|-----------|-------------|
| 1 | FibonacciWSScheduler.rs | ~100 | 2 exec (fib_seq, fib_par), 5 proof fns | 0 — clean |
| 2 | HFSchedulerMtEph.rs | ~170 | spawn, wait, TaskState | 8 — external_body/assume(false) |

## Cost Disagreements

| # | Function | APAS | Claude-Opus-4.6 | Issue |
|---|----------|------|-----------------|-------|
| 1 | fib_par | Work Theta(phi^n), Span Theta(n) | Work Theta(phi^n), Span Theta(phi^n) | Closures call fib_seq not fib_par. Only the top-level split is parallel. True Theta(n) span requires recursive self-calls through join. |

fib_seq agrees with APAS: Work Theta(phi^n), Span Theta(phi^n) — sequential, work = span.

## Implementation Fidelity

**fib_seq:** Faithful sequential Fibonacci. Matches the standard recursive
definition. No deviation.

**fib_par:** Deviates from APAS intent. The function spawns two closures via
HFSchedulerMtEph, but each closure calls `fib_seq` (the sequential variant)
rather than recursing through `fib_par`. This means:
- The top-level call forks two threads.
- Each thread computes fib(n-1) and fib(n-2) fully sequentially.
- No recursive parallelism — the call tree is 1 level of parallelism deep.

APAS expects the parallel Fibonacci to recurse in parallel at every level,
giving Span Theta(n) (the longest chain is the n recursive calls on the
larger branch). The current code achieves Span Theta(phi^n) — same as
sequential.

**HFSchedulerMtEph:** APAS defines what a scheduler is (Def 15.5) and states
the greedy scheduling principle (Def 15.7), but gives very little detail on
how to actually implement one. The code implements a bounded help-first
scheduler using `std::thread::spawn`, `Mutex`, `Condvar`, and `JoinHandle`.
"Help-first" means the caller executes a task locally when no pool capacity
is available, rather than blocking — a form of work stealing where the caller
steals its own work. This required:

- Deep knowledge of Verus closure proofs — the `spawn` function takes a
  closure with `requires`/`ensures`, and the returned `TaskState` handle
  must carry the closure's postcondition as a predicate so that `wait` can
  recover it. Getting the type-level predicate threading right is non-trivial.
- External bodies for `std::thread` operations — Verus cannot verify thread
  spawn/join internals, so `spawn`, `wait`, and the `TaskState` wrapper are
  all `external_body` with trusted specs.
- Help-first scheduling policy — the scheduler manages a bounded thread pool
  with capacity tracking via `Mutex`/`Condvar`. When pool capacity is
  exhausted, the caller runs the task inline rather than blocking. This
  prevents deadlocks in nested joins and provides graceful degradation.
- An `assume(false)` in the `Drop` impl — unsound but pragmatically needed
  because Verus requires all code paths to satisfy the proof, including drop,
  and there is no clean way to express "this handle was already consumed by
  `wait`" in Verus's current type system.

This is a significant engineering effort that APAS's prose does not hint at.

## Spec Fidelity

| # | Function | Spec strength | Notes |
|---|----------|--------------|-------|
| 1 | fib_seq | Strong | ensures result == spec_fib(n), plus overflow bounds |
| 2 | fib_par | Strong | ensures result == spec_fib(n), same as fib_seq |
| 3 | spawn | Strong (external_body) | Postcondition on TaskState handle |
| 4 | wait | Strong (external_body) | Returns value satisfying spawned predicate |

The specs correctly state the functional behavior. The Fibonacci spec
functions (spec_fib) match the standard mathematical definition. The
scheduler's spawn/wait specs establish the connection between the closure's
ensures and the returned value.

## Parallelism Audit

| # | Function | APAS Span | Actual Span | Parallel? | Notes |
|---|----------|-----------|-------------|-----------|-------|
| 1 | fib_seq | Theta(phi^n) | Theta(phi^n) | No | Sequential by design |
| 2 | fib_par | Theta(n) | Theta(phi^n) | Top-level only | Closures call fib_seq, not fib_par |

## Gap Analysis

**Prose items with no implementation:**
- The greedy scheduling principle (TP < W/P + S) is not formally proved.
  It is the theoretical justification for the scheduler but there is no
  Verus lemma establishing this bound.
- No formal model of processors, scheduling, or time.

**Code with no prose counterpart:**
- HFSchedulerMtEph — the scheduler implementation itself. The prose defines
  what a scheduler is but gives no pseudocode for one.
- spec_fib, lemma_fib_seq_*, overflow lemmas — Verus-specific proof
  infrastructure.

## Runtime Tests

| # | Test file | Tests |
|---|-----------|-------|
| 1 | test_fibonacci_scheduler.rs | fib_seq and fib_par correctness |
| 2 | test_threads_plus.rs | Thread infrastructure |
| 3 | test_work_stealing_scheduler.rs | Scheduler spawn/wait |

## Proof Holes

| # | File | Hole | Count | Justification |
|---|------|------|-------|---------------|
| 1 | HFSchedulerMtEph.rs | external_body | 6 | std::thread internals unverifiable |
| 2 | HFSchedulerMtEph.rs | external_type_specification | 1 | JoinHandle type spec |
| 3 | HFSchedulerMtEph.rs | assume(false) | 1 | Drop impl — unsound but pragmatic |

FibonacciWSScheduler.rs is clean — no proof holes.

## Summary

Chap02 is a small chapter with two files. The main finding is the fib_par
span discrepancy: the code only parallelizes at the top level instead of
recursing through fib_par, making the span exponential instead of linear.
The scheduler infrastructure is necessarily external_body due to Verus's
concurrency limitations. The prose is theoretical background with no
algorithms to implement directly.
