# Review Against Prose -- Chap51

**Date**: 2026-03-15
**Reviewer**: Claude-Opus-4.6 (Agent 4, Round 21)

## Phase 1: Inventory

8 modules: BottomUpDPStEph, BottomUpDPStPer, BottomUpDPMtEph, BottomUpDPMtPer, TopDownDPStEph, TopDownDPStPer, TopDownDPMtEph, TopDownDPMtPer.

Total: 98 functions across 8 files. 0 proof holes. All 8 modules clean. 20 clean proof functions.

Info-level accepts: 8 total (1 per file, all in PartialEq::eq bodies).

## Phase 2: Prose Inventory

| # | Chap | Item | Type | Prose Reference |
|---|------|------|------|-----------------|
| 1 | 51 | Bottom-Up Method | Section 1 | Pebble DAG leaves-to-root, diagonal parallelism |
| 2 | 51 | Bottom-up MED | Algorithm 51.1 | med S T with diagonal pebbling, table M |
| 3 | 51 | medOne | Algorithm 51.1 | Single cell: base cases or min(above, left)+1 or diag |
| 4 | 51 | diagonals | Algorithm 51.1 | Recursive diagonal sweep k=0..\|S\|+\|T\| |
| 5 | 51 | Top-Down Method (Memoization) | Definition 51.2 | Run recursive code with memo table lookup/store |
| 6 | 51 | The Memo Function | Algorithm 51.3 | memo f M a: lookup or compute-and-store |
| 7 | 51 | Memoized MED | Algorithm 51.4 | med S T with medOne threading memo table M |
| 8 | 51 | Top-down parallelism limitation | Note | Top-down as described is inherently sequential |
| 9 | 51 | Diagonal pebbling parallelism | Property | Each diagonal can be pebbled in parallel |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

Cost annotations added to all exec functions in BottomUpDPStEph.rs and TopDownDPStEph.rs:
- `med_bottom_up`: APAS Work O(\|S\|*\|T\|), Span O(\|S\|+\|T\|). Claude notes the StEph implementation uses sequential row fill, so actual Span is O(\|S\|*\|T\|).
- `med_memoized`: APAS Work O(\|S\|*\|T\|), Span O(\|S\|+\|T\|). Claude notes the StEph implementation is sequential with memo threading, so actual Span is O(\|S\|*\|T\|).
- `med_recursive`: Same as med_memoized.
- `initialize_base_cases`: N/A, Verus helper. Work O(\|S\|*\|T\|).
- `compute_cell_value`: N/A, Verus helper. Work O(1).
- All accessors: N/A scaffolding, O(1).

### 3b. Implementation Fidelity

| # | Chap | File | Function | Prose Match | Notes |
|---|------|------|----------|-------------|-------|
| 1 | 51 | BottomUpDPStEph.rs | med_bottom_up | Algorithm 51.1 | Row-by-row fill, not diagonal pebbling |
| 2 | 51 | BottomUpDPStEph.rs | initialize_base_cases | Algorithm 51.1 (setup) | Table allocation with base cases |
| 3 | 51 | BottomUpDPStEph.rs | compute_cell_value | Algorithm 51.1 (medOne) | Single cell computation |
| 4 | 51 | TopDownDPStEph.rs | med_memoized | Algorithm 51.4 (entry point) | Clears memo, calls med_recursive |
| 5 | 51 | TopDownDPStEph.rs | med_recursive | Algorithm 51.4 (medOne) | Recursive with HashMap memo |
| 6 | 51 | BottomUpDPMtEph.rs | med_bottom_up_parallel | Algorithm 51.1 | Row-by-row fill (not truly diagonal parallel) |
| 7 | 51 | TopDownDPMtEph.rs | med_memoized_concurrent | Algorithm 51.4 | Sequential recursion with local memo |
| 8 | 51 | TopDownDPMtEph.rs | med_memoized_parallel | Algorithm 51.4 | Parallel recursion with Arc<RwLock> memo |
| 9 | 51 | TopDownDPMtEph.rs | med_recursive_sequential | Algorithm 51.4 (medOne) | Sequential recursive helper |
| 10 | 51 | TopDownDPMtEph.rs | med_recursive_parallel | Algorithm 51.4 | Parallel recursive helper with fork-join |

**Deviations**:
- **Bottom-up**: Prose Algorithm 51.1 uses diagonal pebbling for parallelism. The implementation uses row-by-row sequential fill (for both StEph and MtEph). This is correct but loses the parallel span advantage. The prose notes both row-wise and diagonal orderings are valid.
- **Top-down**: Prose Algorithm 51.4 threads the memo table purely functionally. The implementation uses mutable HashMap references (StEph) or Arc<RwLock> (MtEph). Faithful adaptation to imperative/concurrent setting.
- Both BottomUp and TopDown modules are specialized to MED (char sequences), not generic DP frameworks. The prose presents them as generic techniques applied to MED.

### 3c. Spec Fidelity

| # | Chap | File | Spec Function | Prose Match | Strength |
|---|------|------|---------------|-------------|----------|
| 1 | 51 | BottomUpDPStEph.rs | spec_med (trait) | Algorithm 49.6 | Strong -- recursive MED |
| 2 | 51 | BottomUpDPStEph.rs | spec_min | -- | Helper spec fn |
| 3 | 51 | TopDownDPStEph.rs | spec_med_fn | Algorithm 49.6 | Strong -- standalone MED spec |
| 4 | 51 | TopDownDPStEph.rs | spec_memo_correct | -- | Verus-specific invariant |
| 5 | 51 | TopDownDPMtEph.rs | spec_memo_correct | -- | Verus-specific (with ghost sequences) |

**Spec strength**:
- `med_bottom_up` ensures `distance == spec_med(s_len, t_len)` -- **strong**, fully verified.
- `med_memoized` ensures `distance == spec_med(s_len, t_len)` -- **strong**, fully verified.
- `med_recursive` ensures `distance == spec_med(i, j)` and `spec_memo_correct` -- **strong**, fully verified.
- `initialize_base_cases` ensures table shape and base case correctness -- **strong**.
- `compute_cell_value` ensures `val == spec_med(i, j)` given correct predecessors -- **strong**.
- `lemma_spec_med_bounded` proves `spec_med(i,j) <= i+j` -- **strong** (overflow prevention).

Both BottomUp and TopDown have the same functional spec (spec_med matching Algorithm 49.6), proven from different directions: bottom-up via cell-by-cell loop invariant, top-down via recursive memoization with spec_memo_correct invariant.

## Phase 4: Parallelism Review

| # | Chap | File | Operation | Parallel? | Mechanism |
|---|------|------|-----------|-----------|-----------|
| 1 | 51 | BottomUpDPMtEph.rs | med_bottom_up_parallel | No | Row-by-row sequential fill (same as StEph) |
| 2 | 51 | BottomUpDPMtPer.rs | med_bottom_up_parallel | No | Same row-by-row sequential fill |
| 3 | 51 | TopDownDPMtEph.rs | med_memoized_concurrent | No | Sequential recursion with local mut memo |
| 4 | 51 | TopDownDPMtEph.rs | med_memoized_parallel | Yes | fork-join via HFScheduler on delete/insert branches |
| 5 | 51 | TopDownDPMtPer.rs | med_memoized_concurrent | No | Sequential recursion |
| 6 | 51 | TopDownDPMtPer.rs | med_memoized_parallel | Yes | fork-join via HFScheduler |

**Parallelism assessment**:
- **BottomUpDPMtEph**: Despite the name `med_bottom_up_parallel`, this is a sequential row-by-row fill. The struct uses `ArraySeqMtEphS` (thread-safe data) but the computation itself is not parallelized. Prose Algorithm 51.1 describes diagonal pebbling which would enable per-diagonal parallelism, but this is not implemented.
- **TopDownDPMtEph**: Provides two entry points: `med_memoized_concurrent` (sequential recursion, no parallelism) and `med_memoized_parallel` (fork-join on delete/insert branches with Arc<RwLock> shared memo). The parallel variant genuinely forks subproblems.
- The prose explicitly notes (Section 2, "Limitation of Top-Down Method") that top-down with threaded memo is "inherently sequential." The implementation provides both sequential and parallel variants; the parallel variant uses concurrent memo tables as the prose suggests is an advanced technique.

## Phase 5: Runtime Test Review

All 8 files have corresponding RTTs:
- TestBottomUpDPStEph.rs, TestBottomUpDPStPer.rs, TestBottomUpDPMtEph.rs, TestBottomUpDPMtPer.rs
- TestTopDownDPStEph.rs, TestTopDownDPStPer.rs, TestTopDownDPMtEph.rs, TestTopDownDPMtPer.rs

Coverage: tests exercise construction, MED computation on textbook examples (e.g., "tcat" vs "atc"), empty strings, single characters, and accessor methods.

## Phase 6: PTT Review

No PTTs exist for Chap51. No iterators or complex loop forms. **No PTTs needed.**

## Phase 7: Gap Analysis

### Prose items without implementation

| # | Chap | Item | Notes |
|---|------|------|-------|
| 1 | 51 | Diagonal pebbling (Algorithm 51.1) | Implementation uses row-by-row fill, not diagonal |
| 2 | 51 | Generic DP framework | BottomUp/TopDown are MED-specific, not generic over DP problems |

### Code without prose counterpart

| # | Chap | File | Item | Notes |
|---|------|------|------|-------|
| 1 | 51 | BottomUpDP* | initialize_base_cases | Decomposed helper for table setup |
| 2 | 51 | BottomUpDP* | compute_cell_value | Decomposed helper for single cell |
| 3 | 51 | TopDownDP* | is_memoized, get_memoized, insert_memo | Explicit memo table API |
| 4 | 51 | TopDownDP* | med_memoized_concurrent | Sequential variant in Mt module |
| 5 | 51 | TopDownDPMtEph | med_recursive_sequential, med_recursive_parallel | Two separate recursive helpers |
| 6 | 51 | all | Default impl | Convenient construction with empty sequences |
| 7 | 51 | all | new, set_s, set_t, s_length, t_length, is_empty | Container scaffolding |
| 8 | 51 | all | Clone, PartialEq, Eq, Debug, Display | Derive/trait impls |

### Spec gaps

None significant. Both med_bottom_up and med_memoized have strong functional specs connecting results to spec_med. The only gap is that `med_recursive_parallel` in TopDownDPMtEph has `ensures dist == spec_med_fn(...)` which is strong and fully verified.

## Phase 8: TOC Review

All 8 files follow the standard TOC ordering:
1. module, 2. imports, 3. broadcast use (where used), 4. type definitions, 6. spec fns, 7. proof fns (TopDown only), 8. traits, 9. impls, 11. derive impls in verus!, 13. derive impls outside verus!

BottomUpDPStEph.rs has a duplicate "Table of Contents" comment at line 8 (outside module) and line 12 (inside module). Minor cosmetic issue.

No section ordering violations.

## Proof Holes Summary

**0 holes** across all 8 modules. All 8 modules clean. 20 clean proof functions.

**Info-level accepts**: 8 total (1 per file), all in PartialEq::eq bodies -- standard view-based equality accept pattern.

**Key strengths**:
- Both bottom-up and top-down implementations prove `result == spec_med(s_len, t_len)`.
- Bottom-up proof uses cell-by-cell loop invariant with predecessor lookups.
- Top-down proof uses `spec_memo_correct` invariant maintained through recursive calls.
- `lemma_spec_med_bounded` proven in all files (prevents arithmetic overflow).
- TopDownDPMtEph parallel variant maintains `spec_memo_correct` through concurrent Arc<RwLock> access with verified memo-correct RwLockPredicate.

**Comparison**: Chap51 has the strongest specs of the three DP chapters. Both BottomUp and TopDown achieve full functional verification of MED. By contrast, Chap49's MED/SS modules lack functional specs (T-to-int bridge gap), and Chap50's OBST lacks spec functions entirely (f64 gap).
