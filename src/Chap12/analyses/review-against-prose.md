<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chap12 Review Against Prose

**Reviewer:** Claude-Opus-4.6
**Date:** 2026-02-19
**Last mechanical audit:** 2026-02-19 — full review regeneration; proof holes log updated.
**Prose file:** `prompts/Chap12.txt`
**Source files:** `Exercise12_1.rs`, `Exercise12_2.rs`, `Exercise12_5.rs`

## Prose Inventory

Chapter 12 covers synchronization primitives. Three exercises implemented;
definitions provide background but require no code.

| # | Item | Type |
|---|------|------|
| 1 | Def 12.3: Synchronization Instructions (spin locks, blocking locks, atomic RMW) | Definition |
| 2 | Def 12.4: Nonblocking Synchronization | Definition |
| 3 | Def 12.5: Compare-and-Swap (CAS) | Definition |
| 4 | Def 12.6: Fetch-and-Add (FAA) | Definition |
| 5 | Ex 12.1: Implement spin-lock using fetch-and-add | Exercise |
| 6 | Ex 12.2: Implement fetch-and-add using compare-and-swap | Exercise |
| 7 | Ex 12.5: Concurrent stack using compare-and-swap | Exercise |

## Code Inventory

| # | File | Lines | Parallel? | Proof holes | Notes |
|---|------|-------|-----------|-------------|-------|
| 1 | Exercise12_1.rs | 127 | Yes — multi-thread test | 6 external_body | Ticket-based spin lock via FAA. wf moved into trait (spec_locked). Default verified (delegates to new). |
| 2 | Exercise12_2.rs | 37 | No (trait on AtomicUsize) | 0 — clean | CAS-loop FAA, well-formed |
| 3 | Exercise12_5.rs | 171 | Yes — lock-free CAS | 7 external_body, 1 external, 4 unsafe | Lock-free Treiber stack. wf in trait. Default verified, Drop inside verus! with external_body. |

## Prose-to-Code Mapping

| # | Prose Item | Code | Spec Fidelity |
|---|-----------|------|---------------|
| 1 | Def 12.5: CAS | `AtomicUsize::compare_exchange_weak` (std) | Direct — Rust std provides CAS |
| 2 | Def 12.6: FAA | `AtomicUsize::fetch_add` (std) | Direct — Rust std provides FAA |
| 3 | Ex 12.1: Spin-lock via FAA | Exercise12_1::SpinLock | Faithful — ticket lock using fetch_add for ticket, load for turn |
| 4 | Ex 12.2: FAA via CAS | Exercise12_2::fetch_add_cas | Faithful — CAS retry loop exactly matches the prose intent |
| 5 | Ex 12.5: Concurrent stack via CAS | Exercise12_5::ConcurrentStackMt | Faithful — classic Treiber stack, push/pop both use CAS loops |

## Cost Analysis

APAS provides no cost specs for these exercises. All operations are
constant-time modulo contention (CAS retries, spin-lock spins).

| # | Function | Claude-Opus-4.6 | Notes |
|---|----------|-----------------|-------|
| 1 | SpinLock::lock | Amortized O(1), worst-case unbounded (spin) | Ticket lock guarantees fairness (FIFO ordering) |
| 2 | SpinLock::unlock | O(1) | Single fetch_add |
| 3 | fetch_add_cas | Amortized O(1), worst-case unbounded (CAS retries) | Matches native FAA semantics but less efficient under contention — CAS may fail spuriously |
| 4 | ConcurrentStackMt::push | Amortized O(1), worst-case unbounded (CAS retries) | Treiber stack, lock-free |
| 5 | ConcurrentStackMt::pop | Amortized O(1), worst-case unbounded (CAS retries) | Lock-free, ABA risk mitigated by Box ownership |
| 6 | ConcurrentStackMt::drain | O(n) | Sequential pop loop |

## Parallelism Review

Not applicable — these are synchronization primitives, not parallel
algorithms. They enable parallelism rather than exhibit it.

## Runtime Test Review

| # | Source module | RTT file | PTT file | Status |
|---|-------------|----------|----------|--------|
| 1 | Exercise12_1 | test_exercise12_1.rs | — | RTT only |
| 2 | Exercise12_2 | test_exercise12_2.rs | — | RTT only |
| 3 | Exercise12_5 | test_exercise12_5.rs | — | RTT only |

RTT coverage is solid:
- Exercise12_1: 6 tests — mutual exclusion, with_lock, non-reentrancy, Default
- Exercise12_2: 5 tests — basic FAA, thread safety, zero delta, wrapping
- Exercise12_5: 6 tests — LIFO, empty pop, multi-thread push, multi-thread pop, drain, cycles

No PTTs needed — all function bodies are `external_body` so there are no
Verus proofs to test. The trait specs are minimal (e.g., `wf()` returns true).

## Gap Analysis

**Prose items with no implementation:**

| # | Prose item | Notes |
|---|-----------|-------|
| 1 | Def 12.3: Spin locks, blocking locks | SpinLock covers spin locks; no blocking lock (mutex) exercise |
| 2 | Def 12.4: Nonblocking synchronization | ConcurrentStackMt is a nonblocking data structure |
| 3 | Dekker's algorithm | Mentioned as a remark, not an exercise |

No gaps — all three assigned exercises are implemented.

**Code with no prose counterpart:**

- `SpinLock::with_lock` — convenience method (RAII-like lock guard pattern)
- `ConcurrentStackMt::drain` — utility for testing
- `ConcurrentStackMt::is_empty` — utility
- `Default` impls — Rust idiom, not in prose

## Proof Holes

| # | File | Hole | Count | Justification |
|---|------|------|-------|---------------|
| 1 | Exercise12_1.rs | external_body (struct + 5 trait fns + parallel_increment) | 6 | Atomic operations (AtomicUsize) and OS threads are beyond Verus. wf moved into trait (spec_locked). Default verified (delegates to new). |
| 2 | Exercise12_5.rs | external_body (struct + 6 fns + Drop) | 7 | AtomicPtr, raw pointers, and unsafe Box::from_raw are beyond Verus. wf in trait. Default verified (delegates to new). Drop inside verus! with external_body + opens_invariants none + no_unwind. |
| 3 | Exercise12_5.rs | external (Node struct) | 1 | Raw pointer field (*mut Node) |
| 4 | Exercise12_5.rs | unsafe blocks | 4 | Raw pointer dereference and Box::from_raw in lock-free algorithms |

All proof holes (18 total: 4 unsafe, 13 external_body, 1 external) are inherent
to the domain: Verus cannot verify atomic operations, raw pointers, or OS-level
synchronization. The trait specs provide trusted documentation of linearizable
behavior.

Both `Default` impls are verified (delegate to `new()`). `Drop` for
`ConcurrentStackMt` is inside `verus!` with `external_body`, `opens_invariants
none`, and `no_unwind`. Only `Debug` for `SpinLock` remains outside `verus!` —
correct per the style rule.

Exercise12_2 is the only clean module — its CAS loop uses only Verus-visible
operations on AtomicUsize (via vstd's `exec_allows_no_decreases_clause`).

## Style Notes

1. Both `Default` impls are inside `verus!` and fully verified (no `external_body` needed — they delegate to `new()`).
2. `Drop` for `ConcurrentStackMt` is inside `verus!` with `external_body`, `opens_invariants none`, `no_unwind`.
3. `Debug` for `SpinLock` remains outside `verus!` — correct per style rule.

## Summary

Chap12 is pure exercises in synchronization primitives. All three are
implemented faithfully:

1. **Exercise12_1** — Ticket-based spin lock using fetch-and-add. Fair (FIFO).
2. **Exercise12_2** — Fetch-and-add via CAS retry loop. Clean (no proof holes).
3. **Exercise12_5** — Lock-free Treiber stack using CAS. Classic concurrent
   data structure with proper memory management via Box ownership.

The proof holes (18 total: 4 unsafe, 13 external_body, 1 external) are all
inherent to concurrent programming with atomics and raw pointers — Verus
cannot verify these. The specs serve as trusted documentation. Runtime tests
provide good coverage of correctness under contention. Both `Default` impls
are verified, `Drop` is inside `verus!`. Bare-impl fixes: wf moved into trait
(SpinLockTrait::spec_locked, ConcurrentStackMtTrait::wf). All exec functions
have cost annotation doc comments. No remaining action items.
