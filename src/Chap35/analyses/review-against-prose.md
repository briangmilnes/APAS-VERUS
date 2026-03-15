# Chapter 35: Order Statistics -- Review Against Prose

Reviewer: Claude-Opus-4.6
Date: 2026-03-15
Textbook: prompts/Chap35.txt (APAS Chapter 35, Sections 1-4)

## Phase 1: Inventory

### 1.1 Textbook Content Inventory

| # | Chap | Item | Type | Description |
|---|------|------|------|-------------|
| 1 | 35 | Definition 35.1 | Definition | Order Statistics Problem: kth smallest element |
| 2 | 35 | Algorithm 35.2 | Algorithm | Contraction-Based Select with random pivot |
| 3 | 35 | Example 35.1 | Example | Pivot tree illustration of recursive calls |
| 4 | 35 | Section 3 | Analysis | Work O(n) expected, Span O(lg^2 n) w.h.p. |
| 5 | 35 | Section 3.1 | Analysis | Dart game method for analyzing select |
| 6 | 35 | Exercise 35.1 | Exercise | Work analysis without n-bound on rounds |
| 7 | 35 | Exercise 35.2 | Exercise | Expected span via expected work bound |
| 8 | 35 | Exercise 35.3 | Exercise | Pivot tree O(lg n) height w.h.p. |

### 1.2 Source File Inventory

| # | Chap | File | Variant | LOC | Holes | Status |
|---|------|------|---------|-----|-------|--------|
| 1 | 35 | OrderStatSelectStEph.rs | St/Eph | 371 | 0 | Clean |
| 2 | 35 | OrderStatSelectStPer.rs | St/Per | 361 | 0 | Clean |
| 3 | 35 | OrderStatSelectMtEph.rs | Mt/Eph | 575 | 0 | Clean |
| 4 | 35 | OrderStatSelectMtPer.rs | Mt/Per | 575 | 0 | Clean |

### 1.3 Test File Inventory

| # | Chap | File | Type | Tests |
|---|------|------|------|-------|
| 1 | 35 | TestOrderStatSelectStEph.rs | RTT | 9 |
| 2 | 35 | TestOrderStatSelectStPer.rs | RTT | 9 |
| 3 | 35 | TestOrderStatSelectMtEph.rs | RTT | 9 |
| 4 | 35 | TestOrderStatSelectMtPer.rs | RTT | 9 |

No PTTs exist. None needed (no complex `requires` or iterators).

## Phase 2: Prose Mapping

### 2.1 Definition and Algorithm Coverage

| # | Chap | Prose Item | Covered | File(s) | Notes |
|---|------|-----------|---------|---------|-------|
| 1 | 35 | Def 35.1: kth order statistic | Yes | All 4 | `spec_kth` = s.sort_by(leq)[k] |
| 2 | 35 | Alg 35.2: contraction-based select | Yes | All 4 | `select` + `select_inner` |
| 3 | 35 | Random pivot selection | Yes | All 4 | `random_usize_range(0, n)` |
| 4 | 35 | Three-way partition (l, equals, r) | Yes | All 4 | While loop (St) / join (Mt) |
| 5 | 35 | Recursive case: k < len(l) | Yes | All 4 | `select_inner(&left_a, k)` |
| 6 | 35 | Base case: k in equals region | Yes | All 4 | Returns `Some(pivot)` |
| 7 | 35 | Recursive case: k in right | Yes | All 4 | `select_inner(&right_a, new_k)` |
| 8 | 35 | k adjustment for right branch | Yes | All 4 | `new_k = k - (n - right_count)` |
| 9 | 35 | Example 35.1 | N/A | -- | Illustration only, not an algorithm |
| 10 | 35 | Exercises 35.1-35.3 | N/A | -- | Text proofs, not implementable |

### 2.2 Prose Fidelity Assessment

The implementation faithfully encodes APAS Algorithm 35.2. Key observations:

1. **Unique-element assumption relaxed.** The textbook says "for purposes of simplicity,
   let's assume that sequences consist of unique elements." The implementation correctly
   handles duplicates via three-way partition (left < pivot, equals == pivot, right > pivot),
   which is strictly more general than the textbook's two-way partition (l = elements < p,
   r = elements > p). The equals region handles the gap.

2. **Definition 35.1 encoded precisely.** `spec_kth(s, k)` is defined as
   `s.sort_by(spec_leq())[k]`, which directly formalizes "the kth element of the
   corresponding sorted sequence."

3. **Pivot selection.** Uses `random_usize_range(0, n)` matching the textbook's
   "pick a uniformly random element from a." The randomness is `external_body` in
   vstdplus, which is appropriate since Verus cannot reason about randomness.

4. **k adjustment matches prose.** The textbook says `select r (k - (|a| - |r|))`.
   The code computes `new_k = k - (n - right_count)`, which is identical since
   `|a| - |r| = n - right_count`.

## Phase 3: Cost Annotations

### 3a: Cost Annotations Added

All exec functions now have APAS and Claude cost annotations as doc comments.

| # | Chap | File | Function | APAS Cost | Claude Cost | Agreement |
|---|------|------|----------|-----------|-------------|-----------|
| 1 | 35 | OrderStatSelectStEph.rs | select | W O(n) exp, S O(lg^2 n) w.h.p. | W O(n) exp, S=W=O(n) exp | Disagree (span) |
| 2 | 35 | OrderStatSelectStEph.rs | select_inner | W O(n) exp, S O(lg^2 n) w.h.p. | W O(n) exp, S=W=O(n) exp | Disagree (span) |
| 3 | 35 | OrderStatSelectStPer.rs | select | W O(n) exp, S O(lg^2 n) w.h.p. | W O(n) exp, S=W=O(n) exp | Disagree (span) |
| 4 | 35 | OrderStatSelectStPer.rs | select_inner | W O(n) exp, S O(lg^2 n) w.h.p. | W O(n) exp, S=W=O(n) exp | Disagree (span) |
| 5 | 35 | OrderStatSelectMtEph.rs | select | W O(n) exp, S O(lg^2 n) w.h.p. | W O(n) exp, S O(n) exp | Disagree (span) |
| 6 | 35 | OrderStatSelectMtEph.rs | select_inner | W O(n) exp, S O(lg^2 n) w.h.p. | W O(n) exp, S O(n) exp | Disagree (span) |
| 7 | 35 | OrderStatSelectMtEph.rs | parallel_three_way_partition | W O(n), S O(lg n) | W O(n), S O(n) | Disagree (span) |
| 8 | 35 | OrderStatSelectMtPer.rs | select | W O(n) exp, S O(lg^2 n) w.h.p. | W O(n) exp, S O(n) exp | Disagree (span) |
| 9 | 35 | OrderStatSelectMtPer.rs | select_inner | W O(n) exp, S O(lg^2 n) w.h.p. | W O(n) exp, S O(n) exp | Disagree (span) |
| 10 | 35 | OrderStatSelectMtPer.rs | parallel_three_way_partition | W O(n), S O(lg n) | W O(n), S O(n) | Disagree (span) |

### 3b: Cost Disagreement Analysis

**Work: Agrees across all files.** O(n) expected, matching the textbook's dart-game analysis
(geometric series with factor 0.875 per round).

**Span: Disagrees for all files.** The textbook's O(lg^2 n) span w.h.p. relies on O(lg n)
span per partition step, which requires a parallel filter (O(n) work, O(lg n) span). The
implementation uses sequential loops for filter/partition:

- **St files**: Sequential by nature. Span = Work = O(n) expected. No disagreement in
  substance, only in that the APAS span bound applies to the parallel version.
- **Mt files**: `parallel_three_way_partition` uses `join()` to run left-filter and
  right-filter concurrently, but each filter is a sequential O(n) loop. The join provides
  a constant-factor improvement (halves wall-clock for the two filters) but does not
  change the asymptotic span from O(n) to O(lg n). To achieve O(lg n) span per partition,
  the filters would need to use a parallel `filter` primitive (e.g., parallel prefix sum
  based compaction). The recursive select only follows one branch, so span is the sum of
  partition spans across O(lg n) rounds. With O(n) span per round and geometric shrinkage,
  total span is O(n) expected, not O(lg^2 n).

**Remediation path**: Replace sequential filter loops inside `parallel_three_way_partition`
with calls to a parallel filter/compact operation (O(n) work, O(lg n) span). This would
bring the Mt implementations to O(lg^2 n) span matching APAS.

## Phase 4: Parallelism Audit (Mt files only)

| # | Chap | File | Function | Parallelism | Classification |
|---|------|------|----------|-------------|----------------|
| 1 | 35 | OrderStatSelectMtEph.rs | select | None (wrapper) | Sequential |
| 2 | 35 | OrderStatSelectMtEph.rs | select_inner | Delegates to p3wp | Mixed |
| 3 | 35 | OrderStatSelectMtEph.rs | parallel_three_way_partition | join(f_left, f_right) | Parallel |
| 4 | 35 | OrderStatSelectMtPer.rs | select | None (wrapper) | Sequential |
| 5 | 35 | OrderStatSelectMtPer.rs | select_inner | Delegates to p3wp | Mixed |
| 6 | 35 | OrderStatSelectMtPer.rs | parallel_three_way_partition | join(f_left, f_right) | Parallel |

### Parallelism Details

- `parallel_three_way_partition` uses `join()` from `HFSchedulerMtEph` to run two
  filter closures concurrently. The left closure filters elements < pivot and counts
  elements == pivot. The right closure filters elements > pivot. Both closures have
  full specifications and are verified.
- The data duplication before `join()` (copying into `data_l` and `data_r`) is O(n) work
  and O(n) span (sequential loop). This is a prerequisite for ownership transfer into
  the closures.
- The recursive call in `select_inner` is inherently sequential (only one branch explored),
  which matches the textbook.

### Not Sequentialized

The Mt files genuinely use parallelism. They are not sequential reimplementations.

## Phase 5: Spec Fidelity

| # | Chap | File | Function | Spec Strength | Notes |
|---|------|------|----------|---------------|-------|
| 1 | 35 | All 4 | spec_kth | Strong | = s.sort_by(leq)[k]; matches Def 35.1 |
| 2 | 35 | All 4 | spec_leq | Strong | Bridges TotalOrder trait to vstd relations |
| 3 | 35 | All 4 | select | Strong | k>=len -> None; k<len -> Some(kth stat) |
| 4 | 35 | All 4 | select_inner | Strong | Returns Some(kth stat), proven fully |
| 5 | 35 | Mt 2 | parallel_three_way_partition | Strong | Multiset preservation + partition bounds |
| 6 | 35 | All 4 | lemma_total_ordering | Strong | Bridges TotalOrder to vstd total_ordering |
| 7 | 35 | Mt 2 | lemma_const_seq_multiset | Strong | Multiset count for constant sequences |
| 8 | 35 | Mt 2 | spec_const_seq | Strong | Seq::new(n, |_| v) |

### Assessment

All exec function specs are **strong**. The `select` postcondition directly asserts the
result equals the kth order statistic (kth element of the sorted sequence), which is the
strongest possible spec for this algorithm. The multiset-preservation proof through
partitioning connects the sorted subsequences to the original input, establishing that
the selected element is indeed the correct order statistic.

No specs are weakened relative to the textbook. No `external_body` on algorithmic logic.
The only `external_body` in the call chain is `random_usize_range` (randomness), which
is inherently unverifiable.

## Phase 6: RTT/PTT Review

### RTTs

All 4 modules have matching RTT files with 9 tests each:

| # | Test | Coverage |
|---|------|----------|
| 1 | test_empty | k out of bounds on empty input |
| 2 | test_single | Single element, k=0 and k=1 |
| 3 | test_small | 8-element sequence, all k values |
| 4 | test_already_sorted | Sorted input, all k values |
| 5 | test_reverse_sorted | Reverse-sorted input, all k |
| 6 | test_duplicates | All-equal input, all k values |
| 7 | test_negative | Negative values |
| 8 | test_mixed | Mixed positive/negative values |
| 9 | test_two_elements | Two elements + out-of-bounds k |
| 10 | test_large | n=1000, checks min/median/max |

**Coverage assessment**: Good. Tests cover edge cases (empty, single, two elements),
boundary conditions (sorted, reverse-sorted, all-duplicates), value ranges (negative,
mixed), and scale (n=1000). The `test_large` test validates geometric convergence of the
randomized algorithm at scale.

### PTTs

None exist. None needed: the `requires` clause is simple (`a.spec_len() <= usize::MAX`),
and there are no iterators or complicated callability concerns.

## Phase 7: Gap Analysis

### Gaps Found

| # | Chap | Gap | Severity | Notes |
|---|------|-----|----------|-------|
| 1 | 35 | Span gap in Mt partition | Low | Sequential filter loops yield O(n) span per round vs O(lg n) textbook; work is correct |
| 2 | 35 | Style warning: free spec fns | Info | veracity-review flags spec_leq/spec_kth/spec_const_seq as free fns; standard says put in trait |
| 3 | 35 | Style warning: bound mismatch | Info | Free fns have different bounds than trait (e.g., Copy + Send + Sync + Eq + 'static vs TotalOrder) |
| 4 | 35 | No PTTs | None | Not needed; simple requires clause |

### Gaps NOT Found

- No missing algorithms. Algorithm 35.2 is the only algorithm in Chapter 35.
- No missing definitions. Definition 35.1 is fully encoded.
- No weakened specs. All postconditions match the strongest possible formalization.
- No proof holes. All 4 files are clean (0 holes).
- No sequentialized parallelism. Mt files use genuine `join()`.
- No missing tests. All variants have comprehensive RTTs.
- Exercises 35.1-35.3 are text-based mathematical proofs, not implementable code.
  Example 35.1 is an illustration. None require source files.

## Phase 8: TOC Review

All 4 files follow the Table of Contents standard. Sections present:

| Section | StEph | StPer | MtEph | MtPer |
|---------|:-----:|:-----:|:-----:|:-----:|
| 1. module | Yes | Yes | Yes | Yes |
| 2. imports | Yes | Yes | Yes | Yes |
| 3. broadcast use | Yes | Yes | Yes | Yes |
| 6. spec fns | Yes | Yes | Yes | Yes |
| 7. proof fns | Yes | Yes | Yes | Yes |
| 8. traits | Yes | Yes | Yes | Yes |
| 9. impls | Yes | Yes | Yes | Yes |

Sections 4 (type definitions), 5 (view impls), 10 (iterators), 11 (coarse locking),
12-14 (derive/macros) are correctly omitted as they do not apply.

## Summary

Chapter 35 is **complete and clean**. All 4 modules verify with 0 holes. The implementation
faithfully encodes APAS Algorithm 35.2 (contraction-based select) with strong specifications
that directly assert the result is the kth order statistic. The only `external_body` in the
call chain is the random number generator, which is inherently unverifiable.

The one substantive gap is the span of `parallel_three_way_partition` in the Mt variants:
the textbook assumes O(lg n) span parallel filter, but the implementation uses O(n)
sequential filter loops within `join()`. This yields O(n) expected span instead of
O(lg^2 n) w.h.p. The work bound O(n) expected is correct. Remediation would require
replacing the sequential filter loops with a parallel filter/compact primitive, which is
an enhancement opportunity, not a correctness issue.

| Metric | Value |
|--------|-------|
| Files | 4 |
| Proof holes | 0 |
| Exec functions | 14 total (select x4, select_inner x4, p3wp x2, proof fns x6) |
| Spec strength | All strong |
| RTT files | 4 (36 tests total) |
| PTT files | 0 (not needed) |
| Textbook coverage | 100% of definitions and algorithms |
| Exercises covered | 0/3 (all are text proofs, not implementable) |
| Style warnings | 20 (free spec fn placement, bound mismatches) |
