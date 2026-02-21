<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chap02 Review Against Prose

**Reviewer:** Claude-Opus-4.6
**Date:** 2026-02-19
**Last mechanical audit:** 2026-02-19 — full review regeneration; proof holes log updated.
**Prose file:** `prompts/Chap02.txt`
**Source files:** `FibonacciHFScheduler.rs`, `HFSchedulerMtEph.rs` (renamed from `WSSchedulerMtEph.rs` — "HF" = help-first, reflecting the actual scheduling policy)

## Phase 1: Code Inventory

| # | File | Lines | Functions | Proof holes |
|---|------|-------|-----------|-------------|
| 1 | FibonacciHFScheduler.rs | ~115 | 2 exec (fib_seq, fib_par), 5 proof fns | 0 — clean |
| 2 | HFSchedulerMtEph.rs | ~190 | set_parallelism, join, spawn_join, spawn, wait | 7 — external_body/external_type_specification |

## Phase 2: Prose Inventory

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

## Phase 3: Algorithmic Analysis (Cost Annotations)

All exec functions have APAS and Claude-Opus-4.6 cost annotations:

| # | Function | APAS | Claude-Opus-4.6 |
|---|----------|------|-----------------|
| 1 | fib_seq | Work Θ(φⁿ), Span Θ(φⁿ) | Work Θ(φⁿ), Span Θ(φⁿ) — sequential, work = span |
| 2 | fib_par | Work Θ(φⁿ), Span Θ(n) | Work Θ(φⁿ), Span Θ(n) — recursive fib_par through join() |
| 3 | set_parallelism | N/A (scheduler config) | Work Θ(1), Span Θ(1) |
| 4 | join | N/A (scheduler primitive) | Work Θ(W_fa + W_fb), Span Θ(max(S_fa, S_fb)) when parallel |
| 5 | spawn_join | N/A (scheduler primitive) | Work Θ(W_fa + W_fb), Span Θ(max(S_fa, S_fb)) |
| 6 | spawn | N/A (scheduler primitive) | Work Θ(W_f), Span Θ(S_f) |
| 7 | wait | N/A (scheduler primitive) | Work Θ(1), Span Θ(S_task) |

No cost disagreements. fib_par correctly recurses through fib_par in closures.

## Phase 4: Parallelism Review (Mt Modules)

HFSchedulerMtEph is the only Mt module. It provides fork-join primitives (join, spawn_join, spawn, wait) used by fib_par and other parallel algorithms.

| # | Function | APAS Span | Actual Span | Parallel? | Notes |
|---|----------|-----------|-------------|-----------|-------|
| 1 | fib_seq | Θ(φⁿ) | Θ(φⁿ) | No | Sequential by design |
| 2 | fib_par | Θ(n) | Θ(n) | Yes | Recurses through fib_par; both branches run in parallel via join() |

## Phase 5: RTT Review

| # | Test file | Tests |
|---|-----------|-------|
| 1 | test_fibonacci_scheduler.rs | fib_seq, fib_par correctness (n 0..20, 25, 30) |
| 2 | test_hf_scheduler.rs | spawn_join, join, spawn, wait; nested joins; parallel timing |
| 3 | test_threads_plus.rs | threads_plus (spawn_plus, JoinHandlePlus) — vstdplus infrastructure |

## Phase 6: PTT Review

No PTTs for Chap02. Cargo.toml notes: "HFScheduler uses global static POOL, not testable in PTT framework. RTTs in test_hf_scheduler.rs cover the scheduler." No iterators or verified loops requiring PTTs.

## Phase 7: Gap Analysis

**Prose items with no implementation:**
- The greedy scheduling principle (TP < W/P + S) is not formally proved.
  It is the theoretical justification for the scheduler but there is no
  Verus lemma establishing this bound.
- No formal model of processors, scheduling, or time.

**Code with no prose counterpart:**
- HFSchedulerMtEph — the scheduler implementation itself. The prose defines
  what a scheduler is but gives no pseudocode for one.
- spec_fib, lemma_fib_*, overflow lemmas — Verus-specific proof
  infrastructure.

## Phase 8: TOC Review

FibonacciHFScheduler.rs and HFSchedulerMtEph.rs follow the standard TOC ordering (module, imports, types, spec fns, proof fns, impls).

## Implementation Fidelity

**fib_seq:** Faithful sequential Fibonacci. Matches the standard recursive
definition. No deviation.

**fib_par:** Faithful to APAS. Closures call `fib_par` recursively; both
branches run in parallel via `join()`. Span Θ(n) as expected.

**HFSchedulerMtEph:** APAS defines what a scheduler is (Def 15.5) and states
the greedy scheduling principle (Def 15.7), but gives very little detail on
how to actually implement one. The code implements a bounded help-first
scheduler using `std::thread::spawn`, `Mutex`, `Condvar`, and `JoinHandle`.
"Help-first" means the caller executes a task locally when no pool capacity
is available, rather than blocking. This required:

- Deep knowledge of Verus closure proofs — the `spawn` function takes a
  closure with `requires`/`ensures`, and the returned `TaskState` handle
  must carry the closure's postcondition as a predicate so that `wait` can
  recover it.
- External bodies for `std::thread` operations — Verus cannot verify thread
  spawn/join internals, so `spawn`, `wait`, `join`, `spawn_join`, and the
  `ExTaskState` wrapper are all `external_body` with trusted specs.
- Help-first scheduling policy — when pool capacity is exhausted, the caller
  runs the task inline rather than blocking. This prevents deadlocks in
  nested joins.
- `assume(false); diverge()` in spawn_join Err arm — valid idiom for thread
  panic (per assume-false-diverge rule).

## Spec Fidelity

| # | Function | Spec strength | Notes |
|---|----------|--------------|-------|
| 1 | fib_seq | Strong | ensures result == spec_fib(n), plus overflow bounds |
| 2 | fib_par | Strong | ensures result == spec_fib(n), same as fib_seq |
| 3 | spawn | Strong (external_body) | Postcondition on TaskState handle |
| 4 | wait | Strong (external_body) | Returns value satisfying spawned predicate |
| 5 | join, spawn_join | Strong (external_body) | Both results satisfy closure ensures |

## Proof Holes

| # | File | Hole | Count | Justification |
|---|------|------|-------|---------------|
| 1 | HFSchedulerMtEph.rs | external_body | 6 | std::thread internals unverifiable |
| 2 | HFSchedulerMtEph.rs | external_type_specification | 1 | ExTaskState type spec |

FibonacciHFScheduler.rs is clean — no proof holes.

The `assume(false); diverge()` in spawn_join's Err arm is a valid idiom for
thread panic (per assume-false-diverge rule), not a proof hole.

## Summary

Chap02 is a small chapter with two files. fib_par correctly recurses through
fib_par in closures, achieving Span Θ(n) per APAS. The scheduler
infrastructure (HFSchedulerMtEph) is necessarily external_body due to Verus's
concurrency limitations. The prose is theoretical background with no
algorithms to implement directly.
