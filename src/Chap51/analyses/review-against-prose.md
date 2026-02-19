<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 51: Implementing Dynamic Programming — Review Against Prose

**Date:** 2026-02-17
**Reviewer:** Claude-Opus-4.6 (automated)
**Project:** APAS-VERUS-agent2
**Prose source:** `prompts/Chap51.txt` (Chapter 51 of APAS textbook)

## Phase 1: Prose Summary

Chapter 51 presents two techniques for implementing sharing in recursive solutions to achieve polynomial work:

1. **Bottom-Up (Section 1):** Constructing the DAG bottom-up by "pebbling" vertices whose in-neighbors are already computed. The chapter uses Minimum Edit Distance (MED) as the running example. Algorithm 51.1 pebbles the DAG diagonally, enabling within-diagonal parallelism. The `medOne` function computes a cell as: base cases `(i,0)→i`, `(0,j)→j`; match → `M[i-1,j-1]`; mismatch → `1 + min(M[i,j-1], M[i-1,j])`. No substitute operation.

2. **Top-Down / Memoization (Section 2):** Running the recursive code as-is but storing results in a memo table (Algorithm 51.3: the `memo` function, Algorithm 51.4: memoized MED). The memo table is threaded through the computation, making it inherently sequential. The prose explicitly states this limitation and mentions "advanced techniques" (hidden state, concurrent hash tables, synchronization variables) that are "beyond the scope of this book."

**Key observation from prose:** The APAS MED uses only insert and delete operations (no substitution). When characters match, the cost is `M[i-1,j-1]` (free diagonal). When they don't match, the cost is `1 + min(insert, delete)`. Substitution (replacing one character with another) is NOT an operation in this formulation.

## Phase 2: File Inventory

| # | File | Algorithm | Variant | Lines | verus! content |
|---|---|---|---|---|---|
| 1 | `TopDownDPStEph.rs` | Algo 51.4 (Memoized MED) | St/Eph | 193 | Empty |
| 2 | `TopDownDPStPer.rs` | Algo 51.4 (Memoized MED) | St/Per | 194 | Empty |
| 3 | `TopDownDPMtEph.rs` | Algo 51.4 (Memoized MED) + parallel | Mt/Eph | 291 | Empty |
| 4 | `TopDownDPMtPer.rs` | Algo 51.4 (Memoized MED) + parallel | Mt/Per | 283 | Empty |
| 5 | `BottomUpDPStEph.rs` | Algo 51.1 (Bottom-Up MED) | St/Eph | 190 | Empty |
| 6 | `BottomUpDPStPer.rs` | Algo 51.1 (Bottom-Up MED) | St/Per | 181 | Empty |
| 7 | `BottomUpDPMtEph.rs` | Algo 51.1 (Bottom-Up MED) + parallel diag | Mt/Eph | 221 | Empty |
| 8 | `BottomUpDPMtPer.rs` | Algo 51.1 (Bottom-Up MED) + parallel diag | Mt/Per | 212 | Empty |

All 8 files have corresponding test files in `tests/Chap51/`.

## Phase 3: Algorithmic Fidelity

### BottomUp (Algorithm 51.1) — Faithful

All 4 BottomUp files correctly implement Algorithm 51.1:
- Base cases: `(i, 0) → i`, `(0, j) → j`
- Match: `table[i-1][j-1]` (free diagonal)
- Mismatch: `1 + min(table[i-1][j], table[i][j-1])` (delete, insert)
- No substitute operation
- Diagonal pebbling order with `k` from `1` to `|S|+|T|`
- Index calculations using `max(1, k - |T|)` and `min(k, |S|)` match prose

### TopDown (Algorithm 51.4) — DEVIATION: Extra Substitute Branch

All 4 TopDown files add a `substitute_cost = 1 + med_recursive(i-1, j-1)` branch when characters don't match. The APAS pseudocode has only insert and delete in the mismatch case:

**APAS Algorithm 51.4 (mismatch):**
```
let (M2, v1) = memo medOne M (i, j − 1)      -- insert
    (M3, v2) = memo medOne M2 (i − 1, j)      -- delete
in (M3, 1 + min(v1, v2))
```

**Implementation (mismatch):**
```rust
let insert_cost = 1 + self.med_recursive(i, j - 1);
let delete_cost = 1 + self.med_recursive(i - 1, j);
let substitute_cost = 1 + self.med_recursive(i - 1, j - 1);  // NOT IN APAS
insert_cost.min(delete_cost).min(substitute_cost)
```

**Impact:** The TopDown and BottomUp implementations compute DIFFERENT edit distances. For the textbook example ("tcat" → "atc"), both give 3, so the tests pass. But for inputs like "a" → "b":
- BottomUp (APAS): `2` (delete 'a' + insert 'b')
- TopDown (with substitute): `1` (substitute 'a' → 'b')

This is a **semantic inconsistency** across the two algorithm families.

### Mt Parallel TopDown — Beyond Prose

The prose explicitly states that top-down memoization is "inherently sequential" and that parallel techniques are "beyond the scope of this book." The Mt files provide both:
- `med_memoized_concurrent()` — sequential recursion with `Arc<Mutex<HashMap>>` (faithful to prose spirit)
- `med_memoized_parallel()` — `thread::spawn` per recursive branch (goes beyond prose)

This is a reasonable engineering extension, properly documented as beyond APAS.

### Mt Parallel BottomUp — Faithful

The Mt BottomUp files use `thread::spawn` per diagonal element, which directly implements the "within-diagonal parallelism" the prose describes. This is faithful to the textbook.

## Phase 4: Cost Analysis

### BottomUp Costs

| # | Function | APAS | Implementation | Match? |
|---|---|---|---|---|
| 1 | `med_bottom_up` (St) | W: Θ(mn), S: Θ(m+n) | W: Θ(mn), S: Θ(mn) | **No** — St has no parallelism, so span = work |
| 2 | `med_bottom_up_parallel` (Mt) | W: Θ(mn), S: Θ(m+n) | W: Θ(mn), S: Θ(m+n) | Yes |
| 3 | `compute_diagonal` (St) | W: Θ(diag len), S: Θ(1) | W: Θ(min(m,n)), S: Θ(min(m,n)) | **No** — St loop is sequential |
| 4 | `compute_diagonal_parallel` (Mt) | W: Θ(diag len), S: Θ(1) | W: Θ(min(m,n)), S: Θ(1) | Yes |
| 5 | `compute_cell_value` / `_static` | W: Θ(1), S: Θ(1) | W: Θ(1), S: Θ(1) | Yes |
| 6 | `initialize_base_cases` | N/A (scaffolding) | W: Θ(m+n), S: Θ(m+n) | N/A |

Where m = |S|, n = |T|.

### TopDown Costs

| # | Function | APAS | Implementation | Match? |
|---|---|---|---|---|
| 1 | `med_memoized` (St) | W: Θ(mn), S: Θ(mn) | W: Θ(mn), S: Θ(mn) | Yes |
| 2 | `med_memoized_concurrent` (Mt) | W: Θ(mn), S: Θ(mn) | W: Θ(mn), S: Θ(mn) | Yes |
| 3 | `med_memoized_parallel` (Mt) | N/A (beyond prose) | W: Θ(mn), S: Θ(m+n) | N/A |
| 4 | `med_recursive` (per call) | W: Θ(1) amort | W: Θ(1) amort | Yes |

## Phase 5: Specification Strength

| # | File | Spec Functions | Proof Functions | requires/ensures | Strength |
|---|---|---|---|---|---|
| 1 | TopDownDPStEph.rs | 0 | 0 | 0 | **none** |
| 2 | TopDownDPStPer.rs | 0 | 0 | 0 | **none** |
| 3 | TopDownDPMtEph.rs | 0 | 0 | 0 | **none** |
| 4 | TopDownDPMtPer.rs | 0 | 0 | 0 | **none** |
| 5 | BottomUpDPStEph.rs | 0 | 0 | 0 | **none** |
| 6 | BottomUpDPStPer.rs | 0 | 0 | 0 | **none** |
| 7 | BottomUpDPMtEph.rs | 0 | 0 | 0 | **none** |
| 8 | BottomUpDPMtPer.rs | 0 | 0 | 0 | **none** |

**Reason:** All code resides outside `verus!` blocks due to Verus limitations with `Arc<Mutex<HashMap>>`, `Arc<Mutex<Vec<Vec<usize>>>>`, and `std::thread`. The `verus!` blocks in all 8 files are empty.

## Phase 6: In/Out Table

Since all code is outside `verus!`, every trait impl is "out":

| # | File | Clone | PartialEq/Eq | Default | Debug | Display | Notes |
|---|---|:---:|:---:|:---:|:---:|:---:|---|
| 1 | TopDownDPStEph | ✅ out (derive) | ✅ out (derive) | ✅ out | ✅ out | ✅ out | All outside verus! |
| 2 | TopDownDPStPer | ✅ out (derive) | ✅ out (derive) | ✅ out | ✅ out | ✅ out | All outside verus! |
| 3 | TopDownDPMtEph | ✅ out (derive) | ✅ out (manual) | ✅ out | ✅ out | ✅ out | PartialEq manual (Mutex) |
| 4 | TopDownDPMtPer | ✅ out (derive) | ✅ out (manual) | ✅ out | ✅ out | ✅ out | PartialEq manual (Mutex) |
| 5 | BottomUpDPStEph | ✅ out (derive) | ✅ out (derive) | ✅ out | ✅ out | ✅ out | All outside verus! |
| 6 | BottomUpDPStPer | ✅ out (derive) | ✅ out (derive) | ✅ out | ✅ out | ✅ out | All outside verus! |
| 7 | BottomUpDPMtEph | ✅ out (derive) | ✅ out (derive) | ✅ out | ✅ out | ✅ out | All outside verus! |
| 8 | BottomUpDPMtPer | ✅ out (derive) | ✅ out (derive) | ✅ out | ✅ out | ✅ out | All outside verus! |

**Note:** "✅ out" here means correctly placed outside verus! — but the IDEAL would be inside verus! with specs if Verus supported the required types. The placement is forced by Verus limitations, not by design choice.

## Phase 7: Proof Holes

```
veracity-review-proof-holes output (2026-02-18):

✓ BottomUpDPMtEph.rs  (but 1 bare_impl error)
✓ BottomUpDPMtPer.rs  (but 1 bare_impl error)
✓ BottomUpDPStEph.rs  (but 1 bare_impl error)
✓ BottomUpDPStPer.rs  (but 1 bare_impl error)
✓ TopDownDPMtEph.rs   (but 1 bare_impl error)
✓ TopDownDPMtPer.rs   (but 1 bare_impl error)
✓ TopDownDPStEph.rs   (but 1 bare_impl error)
✓ TopDownDPStPer.rs   (but 1 bare_impl error)

Proof Functions: 0 total
Holes Found: 0 total
Errors: 8 bare impl(s) in files with trait definitions
```

Zero proof holes (vacuously — no proof functions, spec functions, or requires/ensures). However, the bare_impl detector reports **8 bare impl errors**: each file defines a trait (e.g., `BottomUpDPStEphTrait`) but the `impl` block is `impl BottomUpDPStEphS` rather than `impl BottomUpDPStEphTrait for BottomUpDPStEphS`. These need trait wiring.

## Phase 8: Test Coverage

| # | Test File | Tests | Key Tests |
|---|---|---|---|
| 1 | TestTopDownDPStEph.rs | 17 | textbook MED, identical strings, memo ops, insert/overwrite |
| 2 | TestTopDownDPStPer.rs | 14 | textbook MED, identical strings, with_memo_table, clear_memo |
| 3 | TestTopDownDPMtEph.rs | 17 | concurrent MED, parallel MED, memo ops, insert/overwrite |
| 4 | TestTopDownDPMtPer.rs | 14 | concurrent MED, parallel MED, clear_memo |
| 5 | TestBottomUpDPStEph.rs | 13 | textbook MED, identical strings, set_s/set_t |
| 6 | TestBottomUpDPStPer.rs | 15 | textbook MED, one-empty, single-char same/different, PartialEq true/false |
| 7 | TestBottomUpDPMtEph.rs | 11 | parallel textbook MED, parallel identical |
| 8 | TestBottomUpDPMtPer.rs | 11 | parallel textbook MED, parallel one-empty |

**Total: 112 tests across 8 files.**

All files test the textbook example: MED("tcat", "atc") = 3.

### Test Gap

No test exercises an input where TopDown (with substitute) and BottomUp (without substitute) give different results. For example, MED("a", "b") = 1 (TopDown) vs 2 (BottomUp). This inconsistency is masked by the textbook example.

## Action Items

| # | Priority | Item | Files Affected |
|---|---|---|---|
| 1 | **HIGH** | Remove substitute branch from TopDown `med_recursive` / `med_recursive_concurrent` / `med_recursive_parallel` to match APAS Algorithm 51.4 | TopDownDPStEph, TopDownDPStPer, TopDownDPMtEph, TopDownDPMtPer |
| 2 | **HIGH** | Add cross-variant test: verify MED("a", "b") gives same result from TopDown and BottomUp | All test files |
| 3 | **HIGH** | Wire traits: `impl Type` → `impl Trait for Type` (8 bare_impl errors). Traits exist but are not used as impls. | All 8 files |
| 4 | **LOW** | Consider moving base-case-only code inside `verus!` (struct definitions, spec functions for MED correctness) even if the main algorithm stays outside | All 8 files |
| 5 | **LOW** | TopDownDPStPer.rs `med_memoized()` clones the memo table on every call — this defeats persistence semantics. Consider returning updated self. | TopDownDPStPer.rs |
| 6 | **INFO** | BottomUpDPMtEph/MtPer spawn one thread per diagonal element — for small diagonals this is pure overhead. Consider a threshold. | BottomUpDPMtEph, BottomUpDPMtPer |

## Summary

Chapter 51 is a batch-verusified implementation of two DP techniques (bottom-up tabulation, top-down memoization) applied to Minimum Edit Distance. All 8 files compile and run correctly with 112 passing tests and 0 proof holes.

The primary finding is a **semantic deviation** in all 4 TopDown files: they include a substitute operation not present in the APAS pseudocode, causing them to compute standard Levenshtein distance rather than the APAS edit distance. This goes undetected because the textbook test case happens to produce the same result under both formulations.

All code resides outside `verus!` due to Verus limitations with concurrent data structures (`Arc<Mutex<HashMap>>`, `Arc<Mutex<Vec<Vec<usize>>>>`). This means there are zero specifications, zero proof functions, and zero verified properties. The "0 proof holes" result is vacuously true.
