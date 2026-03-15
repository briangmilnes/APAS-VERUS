# Review Against Prose -- Chapter 12: Exercises (Synchronization and Concurrency)

Note: All files in Chap12 are Exercise files. They are explicitly assigned for this
review. They are exercise/demo code implementing concurrency primitives from the
textbook using native Rust synchronization.

## Phase 1: Inventory

Source: `src/Chap12/analyses/veracity-review-module-fn-impls.md`

### Exercise12_1.rs (Spin-lock via fetch-and-add tickets)

| # | Chap | File | Function | Kind | V! | SpecStr | Holes |
|---|------|------|----------|------|----|---------|-------|
| 1 | 12 | Exercise12_1.rs | SpinLock (struct) | type | Y | external_body | accept |
| 2 | 12 | Exercise12_1.rs | spec_locked | spec (trait) | Y | uninterp | 0 |
| 3 | 12 | Exercise12_1.rs | new | exec IT | Y | hole | accept |
| 4 | 12 | Exercise12_1.rs | lock | exec IT | Y | hole | accept |
| 5 | 12 | Exercise12_1.rs | unlock | exec IT | Y | hole | accept |
| 6 | 12 | Exercise12_1.rs | with_lock | exec IT | Y | hole | accept |
| 7 | 12 | Exercise12_1.rs | parallel_increment | exec ML | Y | hole | accept |
| 8 | 12 | Exercise12_1.rs | default | exec IT | Y | NoSpec | 0 |

### Exercise12_2.rs (Fetch-and-add via compare-and-swap)

| # | Chap | File | Function | Kind | V! | SpecStr | Holes |
|---|------|------|----------|------|----|---------|-------|
| 9 | 12 | Exercise12_2.rs | fetch_add_cas | exec IT | Y | NoSpec | 0 |

### Exercise12_5.rs (Lock-free concurrent stack)

| # | Chap | File | Function | Kind | V! | SpecStr | Holes |
|---|------|------|----------|------|----|---------|-------|
| 10 | 12 | Exercise12_5.rs | Node (struct) | type | Y | external | accept |
| 11 | 12 | Exercise12_5.rs | ConcurrentStackMt (struct) | type | Y | external_body | accept |
| 12 | 12 | Exercise12_5.rs | wf | spec (trait) | Y | trivial (true) | accept |
| 13 | 12 | Exercise12_5.rs | new | exec IT | Y | hole | accept |
| 14 | 12 | Exercise12_5.rs | push | exec IT | Y | hole | accept |
| 15 | 12 | Exercise12_5.rs | pop | exec IT | Y | hole | accept |
| 16 | 12 | Exercise12_5.rs | is_empty | exec IT | Y | hole | accept |
| 17 | 12 | Exercise12_5.rs | drain | exec IT | Y | hole | accept |
| 18 | 12 | Exercise12_5.rs | default | exec IT | Y | NoSpec | 0 |
| 19 | 12 | Exercise12_5.rs | drop | exec IT | Y | hole | accept |

Proof holes: 0 formal holes. All annotations are `external_body // accept hole` or
`external // accept hole`, classified as informational by veracity-review. These are
permanent: AtomicUsize, AtomicPtr, and raw pointers have no vstd specs. Proving
functional correctness would require TSM or linearizability arguments that are
disproportionate for exercises.

## Phase 2: Prose Inventory

Source: `prompts/Chap12.txt` (APAS Chapter 12 exercises)

### Definitions

| # | ID | Name | Covered |
|---|-----|------|---------|
| 1 | 12.3 | Synchronization Instructions (spin locks, blocking locks, atomic RMW) | Yes -- Ex12_1 (spin lock), Ex12_2 (CAS) |
| 2 | 12.4 | Nonblocking Synchronization | Yes -- Ex12_5 (lock-free stack) |
| 3 | 12.5 | Compare and Swap | Yes -- Ex12_2, Ex12_5 use CAS |
| 4 | 12.6 | Fetch and Add | Yes -- Ex12_1 uses FAA, Ex12_2 implements FAA via CAS |

### Exercises

| # | ID | Description | File | Covered |
|---|-----|-------------|------|---------|
| 1 | 12.1 | Spin-lock using fetch-and-add | Exercise12_1.rs | Yes |
| 2 | 12.2 | Fetch-and-add using compare-and-swap | Exercise12_2.rs | Yes |
| 3 | 12.5 | Concurrent stack using CAS | Exercise12_5.rs | Yes |

### Algorithms

| # | Algorithm | File | Notes |
|---|-----------|------|-------|
| 1 | Ticket spin-lock (FAA-based) | Exercise12_1.rs | FIFO fairness via ticket/turn counters |
| 2 | CAS-loop FAA | Exercise12_2.rs | compare_exchange_weak retry loop |
| 3 | Treiber stack | Exercise12_5.rs | Lock-free push/pop via CAS on head pointer |

### Cost Specs from Prose

The prose does not give cost specifications for these exercises. It notes that
CAS-based fetch-and-add is "less efficient" than hardware FAA under contention
(Def 12.6 remark), but no asymptotic bounds.

### Theorems

No formal theorems. The exercises ask for implementations, not proofs.

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Annotations

All exec functions already have cost annotations in the required format.

| # | Chap | File | Function | APAS Cost | Agent Cost | Status |
|---|------|------|----------|-----------|------------|--------|
| 1 | 12 | Exercise12_1.rs | new | none | O(1) | present |
| 2 | 12 | Exercise12_1.rs | lock | none | amort O(1), worst unbounded | present |
| 3 | 12 | Exercise12_1.rs | unlock | none | O(1) | present |
| 4 | 12 | Exercise12_1.rs | with_lock | none | O(1) + action cost | present |
| 5 | 12 | Exercise12_1.rs | parallel_increment | none | W Theta(iter), S Theta(iter) | present |
| 6 | 12 | Exercise12_2.rs | fetch_add_cas | none | amort O(1), worst unbounded | present |
| 7 | 12 | Exercise12_5.rs | new | none | O(1) | present |
| 8 | 12 | Exercise12_5.rs | push | none | amort O(1), worst unbounded | present |
| 9 | 12 | Exercise12_5.rs | pop | none | amort O(1), worst unbounded | present |
| 10 | 12 | Exercise12_5.rs | is_empty | none | O(1) | present |
| 11 | 12 | Exercise12_5.rs | drain | none | O(n) | present |

No new annotations needed.

### Phase 3b: Implementation Deviations

| # | Chap | File | Deviation | Severity |
|---|------|------|-----------|----------|
| 1 | 12 | Exercise12_1.rs | Uses Rust AtomicUsize instead of pseudocode FAA; translates to native Rust synchronization as instructed by the prompt. | None -- intentional |
| 2 | 12 | Exercise12_2.rs | Uses compare_exchange_weak (hardware CAS) instead of abstract cas instruction. Uses wrapping_add per Rust semantics. | None -- faithful translation |
| 3 | 12 | Exercise12_5.rs | Uses AtomicPtr (Treiber stack pattern) instead of abstract CAS on references. Uses Box for allocation, raw pointers for node linking. | None -- standard Treiber stack |
| 4 | 12 | Exercise12_5.rs | `drain` not in prose. Project addition for utility. | None -- convenience |

### Phase 3c: Ensures vs Prose

The prose gives operational semantics for CAS and FAA but not formal postconditions.
The implementations match the operational descriptions.

| # | Chap | File | Function | ensures | Prose Match |
|---|------|------|----------|---------|-------------|
| 1 | 12 | Exercise12_1.rs | new | !lock.spec_locked() | matches: lock starts unlocked |
| 2 | 12 | Exercise12_1.rs | lock | self.spec_locked() | matches: lock is held after acquire |
| 3 | 12 | Exercise12_1.rs | unlock | !self.spec_locked() | matches: lock released |
| 4 | 12 | Exercise12_1.rs | with_lock | (none) | acceptable: FnOnce result is opaque |
| 5 | 12 | Exercise12_1.rs | parallel_increment | incremented == 4*iterations | stronger than prose (prose has no spec) |
| 6 | 12 | Exercise12_2.rs | fetch_add_cas | (none) | acceptable: vstd atomic specs lack value postconditions |
| 7 | 12 | Exercise12_5.rs | new | stack.wf() | matches: empty stack is well-formed |
| 8 | 12 | Exercise12_5.rs | push | (none beyond requires) | weak: prose implies push adds to top |
| 9 | 12 | Exercise12_5.rs | pop | (none beyond requires) | weak: prose implies pop returns top |
| 10 | 12 | Exercise12_5.rs | is_empty | (none beyond requires) | weak: prose implies reports emptiness |
| 11 | 12 | Exercise12_5.rs | drain | (none beyond requires) | weak: project addition |

The specs for Exercise12_5 operations are minimal (only `requires self.wf()`). This
is a permanent limitation: the external_body annotations mean Verus cannot verify
functional behavior, and adding stronger ensures would be unsound without proofs.

## Phase 4: Parallelism Review

| # | Chap | File | Function | Classification | Notes |
|---|------|------|----------|----------------|-------|
| 1 | 12 | Exercise12_1.rs | parallel_increment | parallel | 4 threads via HFScheduler spawn/wait |
| 2 | 12 | Exercise12_1.rs | new, lock, unlock, with_lock | sequential | Single-threaded primitives (used by parallel callers) |
| 3 | 12 | Exercise12_2.rs | fetch_add_cas | sequential | Single-threaded CAS loop (thread-safe, not parallel) |
| 4 | 12 | Exercise12_5.rs | all functions | sequential | Thread-safe but single-caller operations |

Exercise12_1.parallel_increment is the only explicitly parallel function.
Exercise12_2 and Exercise12_5 implement thread-safe (concurrent) data structures
that can be used from multiple threads, but the individual operations are sequential.

## Phase 5: Runtime Test Review

### test_exercise12_1.rs

| # | Test | What it covers |
|---|------|----------------|
| 1 | spin_lock_excludes_parallel_threads | 4 threads, 128 increments each, checks total = 512 |
| 2 | spin_lock_with_lock_executes_body | with_lock runs the closure |
| 3 | parallel_increment_counts_all_iterations | parallel_increment(1000) == 4000 |
| 4 | spin_lock_is_non_reentrant | Verifies lock blocks a second acquirer |
| 5 | test_default_trait | Default::default() produces usable lock |
| 6 | test_with_lock_returns_value | with_lock returns closure's value |

Coverage: strong. Tests mutual exclusion, fairness (implicit via deterministic count),
reentrancy blocking, and trait conformance. Uses thread timeouts where needed.

### test_exercise12_2.rs

| # | Test | What it covers |
|---|------|----------------|
| 1 | fetch_add_cas_returns_previous_value | Returns old value, atomically adds |
| 2 | trait_impl_works | Sequential multi-step correctness |
| 3 | fetch_add_cas_is_thread_safe | 8 threads, 1000 ops each, total = 8000 |
| 4 | fetch_add_cas_zero_delta | Identity case |
| 5 | fetch_add_cas_wrapping | Wrapping at usize::MAX |

Coverage: strong. Tests sequential correctness, thread safety under contention,
edge cases (zero delta, wrapping overflow).

### test_exercise12_5.rs

| # | Test | What it covers |
|---|------|----------------|
| 1 | push_pop_lifo_single_thread | LIFO ordering, pop on empty returns None |
| 2 | pop_on_empty_returns_none | Empty stack behavior |
| 3 | multi_thread_push_collects_all_items | 4 threads push 1000 each, drain gets all |
| 4 | multi_thread_pop_consumes_all_elements | 4 threads pop concurrently, all consumed |
| 5 | test_default_trait | Default::default() works |
| 6 | test_drain | Drain returns all elements |
| 7 | test_multiple_push_pop_cycles | Interleaved push/pop cycles |

Coverage: strong. Tests LIFO ordering, concurrent push, concurrent pop, drain, and
trait conformance. The multi-thread tests verify no lost elements.

### Coverage Assessment

All three exercise files have thorough runtime tests covering correctness, thread
safety, and edge cases.

## Phase 6: PTT Review

No PTTs exist for Chap12. None are needed:
- These are exercise files with external_body implementations (nothing for Verus to
  prove beyond the specs).
- No iterators.
- The requires clauses are simple (wf() or none).

## Phase 7: Gap Analysis

### Proof Gaps

All holes in Chap12 are permanent and documented:
- Exercise12_1: 6 external_body accept holes (struct + 4 trait methods + parallel_increment).
  AtomicUsize operations have no vstd specs.
- Exercise12_2: 0 holes. The CAS loop is accepted as well-formed without functional spec.
- Exercise12_5: 13 informational annotations (external_body, external, unsafe_block,
  trivial_spec_wf). AtomicPtr and raw pointers have no vstd specs.

These are structural: the concurrency primitives used (AtomicUsize, AtomicPtr, raw
pointers) are outside Verus's verification model. Proving correctness would require
TSM or linearizability arguments that are disproportionate for textbook exercises.

### Spec Gaps

| # | Chap | File | Function | Gap | Severity |
|---|------|------|----------|-----|----------|
| 1 | 12 | Exercise12_1.rs | with_lock | No ensures for result | Low -- FnOnce opaque to Verus |
| 2 | 12 | Exercise12_2.rs | fetch_add_cas | No ensures | Medium -- could state "returns old value" if vstd supported it |
| 3 | 12 | Exercise12_5.rs | push, pop, is_empty, drain | No functional ensures | Medium -- permanent, external_body |

### Prose Coverage Gaps

| # | Gap | Severity | Notes |
|---|-----|----------|-------|
| 1 | Exercise 12.3 (not assigned) | N/A | Not in Chap12 source files |
| 2 | Exercise 12.4 (not assigned) | N/A | Not in Chap12 source files |
| 3 | Dekker's algorithm (Remark after Def 12.3) | N/A | Not an exercise |
| 4 | Blocking locks (Def 12.3 category 2) | N/A | Only spin locks and CAS implemented |

All three assigned exercises (12.1, 12.2, 12.5) are fully implemented.

### Style Warnings (from veracity-review-verus-style.log)

| # | Chap | File | Warning | Severity |
|---|------|------|---------|----------|
| 1 | 12 | Exercise12_1.rs | [12] with_lock should have requires/ensures | Low |
| 2 | 12 | Exercise12_1.rs | [14] SpinLock missing Display impl | Cosmetic |
| 3 | 12 | Exercise12_2.rs | [12] fetch_add_cas should have requires/ensures | Low |
| 4 | 12 | Exercise12_5.rs | [14] Node missing Debug/Display | Cosmetic |
| 5 | 12 | Exercise12_5.rs | [14] ConcurrentStackMt missing Debug/Display | Cosmetic |
| 6 | 12 | Exercise12_5.rs | [22] spec fn wf has body in trait (should be abstract) | Low |
| 7 | 12 | All 3 files | [24] copyright format | Cosmetic |

Warning 6: `wf` in `ConcurrentStackMtTrait` has `open spec fn wf(&self) -> bool { true }`
directly in the trait. Per project standard, this should be an abstract spec in the
trait with `true` body in the impl. Low priority since it is exercise code.

### Structural Notes

- Exercise12_1: `Default for SpinLock` is inside `verus!` but is not `#[cfg(verus_keep_ghost)]`.
  This is fine since SpinLock is always available.
- Exercise12_1: `parallel_increment` is a free function, not a trait method. Acceptable
  for a demonstration function.
- Exercise12_2: Implements `FetchAddCasTrait` for `AtomicUsize` (extending a std type).
  Clean pattern.
- Exercise12_5: Uses `unsafe` blocks (3 in push/pop, 1 in drop). These are necessary for
  the Treiber stack pattern with raw pointers. Marked with `// accept hole`.
- Exercise12_5: Missing TOC for Exercise12_1.rs and Exercise12_2.rs.

## Phase 8: TOC Review

### Exercise12_1.rs

No TOC present. Should have one (sections 1, 2, 4, 8, 9, 14).

### Exercise12_2.rs

No TOC present. Should have one (sections 1, 2, 8, 9).

### Exercise12_5.rs

Present TOC:
```
//  Table of Contents
//  1. module
//  4. type definitions
//  8. traits
//  9. impls
```

Actual structure: module (line 20), imports (lines 21-23), type definitions (lines 31-42),
traits (lines 48-89), impls (lines 94-167). Section 2 (imports) is missing from the
TOC. Sections 12/14 (Default/Drop impls inside verus!) are present but not listed.
Otherwise accurate.

### TOC Summary

| # | Chap | File | TOC Present | Correct |
|---|------|------|-------------|---------|
| 1 | 12 | Exercise12_1.rs | No | -- |
| 2 | 12 | Exercise12_2.rs | No | -- |
| 3 | 12 | Exercise12_5.rs | Yes | Minor: missing section 2, 12/14 |

## Summary

Chapter 12 implements three textbook exercises on synchronization primitives using
native Rust concurrency (AtomicUsize, AtomicPtr, CAS). All three exercises are
complete and have thorough runtime tests. Proof holes are permanent and structural:
the concurrency primitives used are outside Verus's verification model.

The implementations are faithful translations of the textbook's pseudocode into
idiomatic Rust concurrency patterns (ticket lock, CAS retry loop, Treiber stack).

### Actionable Items

| # | Priority | Item |
|---|----------|------|
| 1 | Low | Add TOC headers to Exercise12_1.rs and Exercise12_2.rs |
| 2 | Low | Add section 2 (imports) to Exercise12_5.rs TOC |
| 3 | Cosmetic | Add Display impl for SpinLock (Exercise12_1.rs) |
| 4 | Cosmetic | Add Debug/Display impls for ConcurrentStackMt (Exercise12_5.rs) |
| 5 | Low | Move wf body from trait to impl in Exercise12_5.rs |

---

Date: 2026-03-15
Reviewer: Claude-Opus-4.6, Agent 1
