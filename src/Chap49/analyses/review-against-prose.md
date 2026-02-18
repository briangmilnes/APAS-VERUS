<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 49: DP — SubsetSum, MinEditDist — Review Against Prose

| Field | Value |
|---|---|
| Date | 2026-02-17 |
| Reviewer | Claude-Opus-4.6 |
| Chapter | 49 — Two Problems (Subset Sums, Minimum Edit Distance) |
| Files | 8 source, 8 test, 0 PTT |
| Prose Source | `prompts/Chap49.txt` |

## Phase 1: File Inventory

| # | File | Variant | Algorithm | Lines | verus! Content |
|---|---|---|---|---|---|
| 1 | `SubsetSumStEph.rs` | St/Eph | Subset Sum (top-down memo) | 202 | empty |
| 2 | `SubsetSumStPer.rs` | St/Per | Subset Sum (top-down memo) | 177 | empty |
| 3 | `SubsetSumMtEph.rs` | Mt/Eph | Subset Sum (parallel memo) | 224 | empty |
| 4 | `SubsetSumMtPer.rs` | Mt/Per | Subset Sum (parallel memo) | 201 | empty |
| 5 | `MinEditDistStEph.rs` | St/Eph | Min Edit Distance (top-down memo) | 248 | empty |
| 6 | `MinEditDistStPer.rs` | St/Per | Min Edit Distance (top-down memo) | 199 | empty |
| 7 | `MinEditDistMtEph.rs` | Mt/Eph | Min Edit Distance (parallel memo) | 251 | empty |
| 8 | `MinEditDistMtPer.rs` | Mt/Per | Min Edit Distance (parallel memo) | 211 | empty |

All 8 `verus! {}` blocks are empty. 100% of executable code is outside verus!.

## Phase 2: Prose Alignment

### SubsetSum — Algorithm 49.3 (Recursive SS, Indexed)

**Textbook algorithm:**
```
SS'(i, j) =
  case (i, j) of
    (_, 0) => true
  | (0, _) => false
  | _ => if (S[i-1] > j) then SS'(i-1, j)
         else (SS'(i-1, j - S[i-1]) or SS'(i-1, j))
in SS'(|S|, k)
```

**Implementation (`subset_sum_rec`):** Matches exactly. The base cases, recursive structure, and element-value check all align with Algorithm 49.3. Memoization via HashMap corresponds to the prose's discussion of sharing in the recursion DAG (§49, "Improving Work by Sharing").

| # | Prose Element | Implementation | Match? |
|---|---|---|---|
| 1 | Base case (_, 0) => true | `(_, 0) => true` | Yes |
| 2 | Base case (0, _) => false | `(0, _) => false` | Yes |
| 3 | Guard S[i-1] > j | `element_value > j` | Yes |
| 4 | Recursive exclude: SS'(i-1, j) | `subset_sum_rec(table, i-1, j)` | Yes |
| 5 | Recursive include: SS'(i-1, j-S[i-1]) | `subset_sum_rec(table, i-1, j-element_value)` | Yes |
| 6 | Combine with OR | `\|\| operator` | Yes |
| 7 | Initial call SS'(\|S\|, k) | `subset_sum_rec(self, n, target)` | Yes |
| 8 | Memoization (DAG sharing) | HashMap<(usize, i32), bool> | Yes |

### MinEditDist — Algorithm 49.6 (Recursive MED, Indexed)

**Textbook algorithm:**
```
MED'(i, j) =
  case (i, j) of
    (i, 0) => i
  | (0, j) => j
  | (i, j) => if (S[i-1] = T[j-1]) then MED'(i-1, j-1)
              else 1 + min(MED'(i, j-1), MED'(i-1, j))
in MED'(|S|, |T|)
```

**Implementation (`min_edit_distance_rec`):** Matches Algorithm 49.6. The base cases return i and j respectively (number of remaining deletions/insertions). The recursive case checks character equality and branches on delete vs. insert.

| # | Prose Element | Implementation | Match? |
|---|---|---|---|
| 1 | Base case (i, 0) => i | `(i, 0) => i` | Yes |
| 2 | Base case (0, j) => j | `(0, j) => j` | Yes |
| 3 | Character match: skip | `if source_char == target_char` → recurse (i-1, j-1) | Yes |
| 4 | Delete: MED'(i-1, j) | `min_edit_distance_rec(table, i-1, j)` | Yes |
| 5 | Insert: MED'(i, j-1) | `min_edit_distance_rec(table, i, j-1)` | Yes |
| 6 | Combine: 1 + min(...) | `1 + std::cmp::min(delete_cost, insert_cost)` | Yes |
| 7 | Initial call MED'(\|S\|, \|T\|) | `min_edit_distance_rec(self, source_len, target_len)` | Yes |
| 8 | Memoization (DAG sharing) | HashMap<(usize, usize), usize> | Yes |

**Note:** The textbook's Algorithm 49.5 starts from the front of the sequences (using Cons/Nil lists), while Algorithm 49.6 (indexed) starts from the end. The implementation follows Algorithm 49.6.

## Phase 3: Cost Analysis

### SubsetSum

| # | Variant | APAS Work | APAS Span | Impl Work | Impl Span | Match? |
|---|---|---|---|---|---|---|
| 1 | St (Eph/Per) | Θ(k×\|S\|) | Θ(\|S\|) | Θ(k×\|S\|) | Θ(k×\|S\|) | Work: Yes, Span: No (St is sequential) |
| 2 | Mt (Eph/Per) | Θ(k×\|S\|) | Θ(\|S\|) | Θ(k×\|S\|) | Θ(\|S\|) | Yes |

The APAS span of Θ(|S|) assumes parallel execution. The St variants are sequential, so their span equals their work: Θ(k×|S|). This is expected — the textbook's span analysis applies to the parallel (Mt) variants.

The Mt variants fork both branches with `thread::spawn`, achieving Θ(|S|) span because the recursion depth is |S| (each level removes one element from S) and both branches at each level execute concurrently.

### MinEditDist

| # | Variant | APAS Work | APAS Span | Impl Work | Impl Span | Match? |
|---|---|---|---|---|---|---|
| 1 | St (Eph/Per) | Θ(\|S\|×\|T\|) | Θ(\|S\|+\|T\|) | Θ(\|S\|×\|T\|) | Θ(\|S\|×\|T\|) | Work: Yes, Span: No (St is sequential) |
| 2 | Mt (Eph/Per) | Θ(\|S\|×\|T\|) | Θ(\|S\|+\|T\|) | Θ(\|S\|×\|T\|) | Θ(\|S\|+\|T\|) | Yes |

Same pattern: the APAS span assumes parallelism. St variants are sequential (span = work). Mt variants achieve Θ(|S|+|T|) span because each recursive call either decrements i or j, so the longest path has at most |S|+|T| steps.

## Phase 4: Parallelism Review (Mt Variants)

### SubsetSumMtEph / SubsetSumMtPer

**Parallelism mechanism:** `thread::spawn` on both branches of the recursion (include/exclude element).

**Shared state:** `Arc<Mutex<HashMap<(usize, i32), bool>>>` — shared memoization table.

| # | Aspect | Assessment |
|---|---|---|
| 1 | Fork structure | Correct: forks on both SS'(i-1, j-S[i-1]) and SS'(i-1, j) |
| 2 | Memoization thread-safety | Mutex-protected HashMap — correct but coarse-grained |
| 3 | Thread granularity | Spawns OS threads per fork — excessive for deep recursion |
| 4 | Span achievability | Theoretical Θ(\|S\|), practical span degraded by lock contention |
| 5 | Short-circuit optimization | Missing: `result1 \|\| result2` evaluates both even if result1 is true |

### MinEditDistMtEph / MinEditDistMtPer

**Parallelism mechanism:** `thread::spawn` on both branches (delete/insert) when characters mismatch.

**Shared state:** `Arc<Mutex<HashMap<(usize, usize), usize>>>` — shared memoization table.

| # | Aspect | Assessment |
|---|---|---|
| 1 | Fork structure | Correct: forks on MED'(i-1, j) and MED'(i, j-1) |
| 2 | Memoization thread-safety | Mutex-protected HashMap — correct but coarse-grained |
| 3 | Thread granularity | Spawns OS threads per fork — excessive for deep recursion |
| 4 | Span achievability | Theoretical Θ(\|S\|+\|T\|), practical span degraded by lock contention |
| 5 | Character match path | Serial (no fork needed) — correct optimization |

### Practical Parallelism Concerns

The Mt variants spawn an OS-level `thread::spawn` for every recursive branch. The textbook's analysis assumes O(1) fork/join cost (work-stealing scheduler), but actual `thread::spawn` has ~μs overhead. For inputs of size n, this could create O(n×k) or O(n×m) thread spawns, which will thrash the system.

A production-quality implementation would use:
1. A work-stealing thread pool (e.g., Rayon) instead of raw `thread::spawn`
2. A granularity cutoff: recurse sequentially below a threshold depth
3. Fine-grained locking or lock-free memo tables (e.g., DashMap)

These are engineering concerns, not algorithmic ones. The theoretical cost analysis remains correct.

## Phase 5: RTT (Runtime Tests)

| # | Test File | Tests | Status |
|---|---|---|---|
| 1 | `TestSubsetSumStEph.rs` | 24 | Present |
| 2 | `TestSubsetSumStPer.rs` | 16 | Present |
| 3 | `TestSubsetSumMtEph.rs` | 12 | Present |
| 4 | `TestSubsetSumMtPer.rs` | 10 | Present |
| 5 | `TestMinEditDistStEph.rs` | 20 | Present |
| 6 | `TestMinEditDistStPer.rs` | 19 | Present |
| 7 | `TestMinEditDistMtEph.rs` | 22 | Present |
| 8 | `TestMinEditDistMtPer.rs` | 13 | Present |
| | **Total** | **136** | |

Tests cover: basic algorithm correctness (textbook Example 49.1, 49.3), edge cases (empty inputs, single elements, negative targets), mutation (Eph variants), memoization behavior, Display/Debug, Clone, PartialEq, IntoIterator, and macro constructors.

## Phase 6: PTT (Proof Time Tests)

**No PTTs exist for Chapter 49.** This is expected — there is nothing inside `verus!` to test.

| # | PTT File | Tests |
|---|---|---|
| | (none) | 0 |

## Phase 7: Gap Analysis

### Critical Gaps

| # | Gap | Severity | Description |
|---|---|---|---|
| 1 | Empty verus! blocks | Critical | All 8 files have empty `verus! {}` blocks. No specifications, no verification. |
| 2 | No spec functions | Critical | No `spec fn` definitions for the DP recurrences. |
| 3 | No requires/ensures | Critical | No pre/postcondition contracts on any function. |
| 4 | No View impls | Critical | No abstract views for the solver structs. |
| 5 | No proof functions | Critical | No proofs connecting implementation to specification. |

### Structural Gaps

| # | Gap | Severity | Description |
|---|---|---|---|
| 6 | No bottom-up DP variant | Medium | The textbook implies both top-down (memo) and bottom-up (tabulation) approaches. Only top-down is implemented. |
| 7 | No PTTs | Low | Expected since verus! is empty — PTTs would have nothing to test. |
| 8 | Thread granularity | Low | Mt variants spawn OS threads for every fork — impractical for real workloads. |
| 9 | i32 target type for SS | Low | SubsetSum target is `i32`, limiting to 2^31-1. The textbook uses unbounded integers. |

### Style Gaps

| # | Gap | Severity | Description |
|---|---|---|---|
| 10 | Copyright uses `//!` | Minor | Module header rule requires `//` for copyright, `//!` for module doc. All files use `//!` for both. |
| 11 | Missing SPDX line | Minor | Module header rule requires `// SPDX-License-Identifier: Apache-2.0`. |
| 12 | No PartialEqSpecImpl | Minor | St variants use `#[derive(PartialEq)]` outside verus! — no spec connection. Mt variants have manual `PartialEq` without `PartialEqSpecImpl`. |
| 13 | Missing TOC entries | Minor | Files have partial TOC comments (sections 4, 8, 9, 11, 13) but no full TOC block at the top. |

## Phase 8: TOC / In-Out Table

### In/Out Table

All code is outside `verus!` — every item is marked accordingly.

| # | File | Struct | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Trait | Impl |
|---|---|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| 1 | SubsetSumStEph | SubsetSumStEphS | ❌ out | ❌ out | - | - | ❌ out | ✅ out | ✅ out | ❌ out | ❌ out |
| 2 | SubsetSumStPer | SubsetSumStPerS | ❌ out | ❌ out | - | - | ❌ out | ✅ out | ✅ out | ❌ out | ❌ out |
| 3 | SubsetSumMtEph | SubsetSumMtEphS | ❌ out | ❌ out | - | - | - | ✅ out | ✅ out | ❌ out | ❌ out |
| 4 | SubsetSumMtPer | SubsetSumMtPerS | ❌ out | ❌ out | - | - | - | ✅ out | ✅ out | ❌ out | ❌ out |
| 5 | MinEditDistStEph | MinEditDistStEphS | ❌ out | ❌ out | - | - | ❌ out | ✅ out | ✅ out | ❌ out | ❌ out |
| 6 | MinEditDistStPer | MinEditDistStPerS | ❌ out | ❌ out | - | - | ❌ out | ✅ out | ✅ out | ❌ out | ❌ out |
| 7 | MinEditDistMtEph | MinEditDistMtEphS | ❌ out | ❌ out | - | - | - | ✅ out | ✅ out | ❌ out | ❌ out |
| 8 | MinEditDistMtPer | MinEditDistMtPerS | ❌ out | ❌ out | - | - | - | ✅ out | ✅ out | ❌ out | ❌ out |

**Legend:** ✅ out = correctly outside verus! (Debug/Display must be outside). ❌ out = incorrectly outside verus! (should be inside with specs). `-` = not implemented.

**Key finding:** Clone, PartialEq/Eq, traits, and impl blocks are all outside verus! — they should be inside with specifications. Only Debug and Display are correctly outside.

### Why Everything Is Outside verus!

Two Verus limitations forced all code outside verus!:

1. **`&mut self` return types on trait methods:** Verus does not support `&mut T` return types inside `verus!`. The ephemeral traits return `&mut ArraySeq*` from methods like `multiset_mut()`, `source_mut()`, `target_mut()`.

2. **`HashMap`:** Verus does not verify `std::collections::HashMap`. The memoization table is central to both algorithms. Since the struct contains a HashMap (or `Arc<Mutex<HashMap>>`), everything touching it must be outside verus!.

## Proof Holes

```
$ veracity-review-proof-holes -d src/Chap49

✓ MinEditDistMtEph.rs
✓ MinEditDistMtPer.rs
✓ MinEditDistStEph.rs
✓ MinEditDistStPer.rs
✓ SubsetSumMtEph.rs
✓ SubsetSumMtPer.rs
✓ SubsetSumStEph.rs
✓ SubsetSumStPer.rs

Modules: 8 clean, 0 holed
Holes Found: 0 total
```

0 proof holes — but this is vacuously true since all code is outside `verus!`. There is nothing for Verus to verify.

## Action Items

| # | Priority | Action | Description |
|---|---|---|---|
| 1 | P0 | Move algorithmic code into verus! | Requires restructuring to avoid HashMap and `&mut` return types in traits. Consider separating the spec/proof layer from the exec layer. |
| 2 | P0 | Add spec functions for DP recurrences | Define `spec fn spec_subset_sum(s: Seq<int>, k: int) -> bool` and `spec fn spec_min_edit_dist(s: Seq<T>, t: Seq<T>) -> nat` matching the textbook recurrences. |
| 3 | P0 | Add requires/ensures to main algorithms | At minimum, `ensures result == spec_subset_sum(self@, target)` and similar. |
| 4 | P1 | Add View impls | Define abstract views for solver structs so specs can reason about them. |
| 5 | P1 | Add PartialEqSpecImpl | Move PartialEq inside verus! with the standard pattern. |
| 6 | P1 | Fix module headers | Use `//` for copyright (not `//!`), add SPDX line. |
| 7 | P2 | Add bottom-up DP variants | The textbook discusses tabulation; only top-down memoization is implemented. |
| 8 | P2 | Add granularity cutoff to Mt variants | Avoid spawning OS threads for small subproblems. |
| 9 | P2 | Add PTTs | Once specs exist inside verus!, add proof-time tests. |
| 10 | P3 | Add full TOC blocks | Add standard TOC comment at top of each file. |

## Summary

Chapter 49 implements the SubsetSum and MinEditDist algorithms faithfully according to the APAS textbook. Both algorithms match their respective prose descriptions (Algorithm 49.3 for SS, Algorithm 49.6 for MED). The Mt variants correctly parallelize the recursive branches using `thread::spawn`. All 136 runtime tests pass.

However, the chapter has **zero Verus verification**. All 8 `verus!` blocks are empty. This was forced by two Verus limitations: `HashMap` (used for memoization) and `&mut` return types on trait methods. The 0 proof holes finding is vacuously true — there is nothing to verify.

To bring this chapter to the verification standard of earlier chapters, a significant restructuring is needed: separate the verified spec/proof layer from the unverified exec layer, use alternative data structures for memoization (e.g., Vec-based tables for bottom-up DP), and remove `&mut` returns from trait methods.
