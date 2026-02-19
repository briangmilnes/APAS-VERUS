<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 49: DP — SubsetSum, MinEditDist — Review Against Prose

**Date:** 2026-02-19
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap49.txt`

## Phase 1: Inventory

72 function entries extracted by `veracity-review-module-fn-impls` across 8 source files. All 8 files have empty `verus!` blocks — 100% of executable code is outside `verus!`.

| # | File | Variant | Algorithm | Lines | V! Content |
|---|------|---------|-----------|------:|------------|
| 1 | `SubsetSumStEph.rs` | St/Eph | Subset Sum (top-down memo) | 204 | empty |
| 2 | `SubsetSumStPer.rs` | St/Per | Subset Sum (top-down memo) | 174 | empty |
| 3 | `SubsetSumMtEph.rs` | Mt/Eph | Subset Sum (parallel memo) | 227 | empty |
| 4 | `SubsetSumMtPer.rs` | Mt/Per | Subset Sum (parallel memo) | 198 | empty |
| 5 | `MinEditDistStEph.rs` | St/Eph | Min Edit Distance (top-down memo) | 254 | empty |
| 6 | `MinEditDistStPer.rs` | St/Per | Min Edit Distance (top-down memo) | 199 | empty |
| 7 | `MinEditDistMtEph.rs` | Mt/Eph | Min Edit Distance (parallel memo) | 261 | empty |
| 8 | `MinEditDistMtPer.rs` | Mt/Per | Min Edit Distance (parallel memo) | 211 | empty |

**Root cause for code outside verus!:** All modules use `std::collections::HashMap` for memoization, which Verus does not support. Mt variants additionally use `Arc<Mutex<HashMap>>`.

## Phase 2: Prose Inventory

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Definition 49.1 | Subset Sum (SS) Problem: given multiset S of positive integers and value k, determine if any X ⊆ S sums to k |
| 2 | Definition 49.4 | Minimum Edit Distance (MED) Problem: given Σ, S, T, find min insertions + deletions to transform S to T |

### Algorithms

| # | Item | Description |
|---|------|-------------|
| 1 | Algorithm 49.3 | Recursive Subset Sum (Indexed): `SS'(i, j)` with base cases `(_, 0) => true`, `(0, _) => false`, recursive case checks `S[i-1] > j` |
| 2 | Algorithm 49.5 | Recursive MED: `MED(S, T)` with base cases `(_, Nil) => |S|`, `(Nil, _) => |T|`, match case `s=t => MED(S', T')`, else `1 + min(MED(S, T'), MED(S', T))` |
| 3 | Algorithm 49.6 | Recursive MED (Indexed): same but with integer indices `MED'(i, j)` |

### Cost Specs

| # | Problem | Work | Span | Parallelism |
|---|---------|------|------|-------------|
| 1 | Subset Sum | O(k·|S|) | O(|S|) | O(k) |
| 2 | Min Edit Distance | O(|S|·|T|) | O(|S|+|T|) | O(|S|·|T|/(|S|+|T|)) |

### Examples

| # | Item | Description |
|---|------|-------------|
| 1 | Example 49.1 | SS({1,4,2,9}, 8) = false; SS({1,4,2,9}, 12) = true |
| 2 | Example 49.2 | SS({1,1,1}, 3) recursion tree with sharing |
| 3 | Example 49.3 | MED(⟨A,B,C,A,D,A⟩, ⟨A,B,A,D,C⟩) = 3 |
| 4 | Example 49.4 | Greedy failure on MED: C-A mismatch, wrong choice leads to suboptimal |
| 5 | Example 49.5 | MED DAG showing shared subproblems |

### Properties

| # | Property | Description |
|---|----------|-------------|
| 1 | SS sharing | Total distinct subproblems bounded by |S|·(k+1) |
| 2 | MED sharing | Total distinct arguments bounded by (|S|+1)·(|T|+1) |
| 3 | MED DAG depth | At most |S|+|T| since each call removes from S or T |
| 4 | Pseudo-polynomial | SS is NP-hard in general but O(k·|S|) when k polynomial in |S| |

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Annotations

| # | Module | Function | APAS Work | APAS Span | Actual Work | Actual Span | Match? | Notes |
|---|--------|----------|-----------|-----------|-------------|-------------|:------:|-------|
| 1 | SubsetSumSt* | `subset_sum_rec` | O(k·\|S\|) | O(\|S\|) | O(k·\|S\|) | O(k·\|S\|) | Work: Yes, Span: **No** | Sequential — span equals work |
| 2 | SubsetSumMt* | `subset_sum_rec` | O(k·\|S\|) | O(\|S\|) | O(k·\|S\|) | O(\|S\|) | Yes | `thread::spawn` on both branches gives O(\|S\|) span |
| 3 | MinEditDistSt* | `min_edit_distance_rec` | O(\|S\|·\|T\|) | O(\|S\|+\|T\|) | O(\|S\|·\|T\|) | O(\|S\|·\|T\|) | Work: Yes, Span: **No** | Sequential — span equals work |
| 4 | MinEditDistMt* | `min_edit_distance_rec` | O(\|S\|·\|T\|) | O(\|S\|+\|T\|) | O(\|S\|·\|T\|) | O(\|S\|+\|T\|) | Yes | `thread::spawn` on delete/insert branches |

### Phase 3b: Implementation Fidelity

| # | Prose Item | Implementation | Fidelity | Notes |
|---|-----------|---------------|----------|-------|
| 1 | Algorithm 49.3 (SS Indexed) | `subset_sum_rec` (all 4 variants) | ✅ Faithful | Base cases `(_, 0) => true`, `(0, _) => false`, recursive structure matches exactly |
| 2 | Algorithm 49.6 (MED Indexed) | `min_edit_distance_rec` (all 4 variants) | ✅ Faithful | Base cases `(i, 0) => i`, `(0, j) => j`, match/insert/delete logic matches exactly |
| 3 | Memoization (sharing) | `HashMap<(usize, _), _>` | ✅ Faithful | Top-down memoization implements the prose's "sharing of subproblems" discussion |
| 4 | Parallel SS via fork | Mt variants spawn threads | ✅ Faithful | Both branches `SS'(i-1, j-S[i-1])` and `SS'(i-1, j)` forked in parallel |
| 5 | Parallel MED via fork | Mt variants spawn threads | ✅ Faithful | Delete `MED'(i-1, j)` and insert `MED'(i, j-1)` forked in parallel |
| 6 | MED comparison `S[i-1] = T[j-1]` | Code uses `source_char == target_char` | ✅ Faithful | Prose uses `S[i-1] = T[i-1]` (typo in prose — should be `T[j-1]`) |
| 7 | SS only considers positive integers | Code accepts `i32` (can be negative) | ⚠️ Minor | `target < 0 => false` guard handles gracefully |

### Phase 3c: Spec Fidelity

**N/A** — No Verus specifications exist. All functions are unspecified plain Rust.

## Phase 4: Parallelism Review

Mt modules exist for both algorithms.

| # | Module | Parallel? | Mechanism | Notes |
|---|--------|:---------:|-----------|-------|
| 1 | `SubsetSumMtEph.rs` | ✅ Genuine | `thread::spawn` on both SS branches | Shares memo via `Arc<Mutex<HashMap>>` |
| 2 | `SubsetSumMtPer.rs` | ✅ Genuine | `thread::spawn` on both SS branches | Shares memo via `Arc<Mutex<HashMap>>` |
| 3 | `MinEditDistMtEph.rs` | ✅ Genuine | `thread::spawn` on delete/insert branches | Shares memo via `Arc<Mutex<HashMap>>` |
| 4 | `MinEditDistMtPer.rs` | ✅ Genuine | `thread::spawn` on delete/insert branches | Shares memo via `Arc<Mutex<HashMap>>` |

**Assessment:** All 4 Mt modules genuinely spawn threads for the recursive fork. However, the `Arc<Mutex<HashMap>>` creates contention — every memo lookup/insert acquires a global lock, potentially serializing the computation under high contention. The prose's O(|S|) span for SS and O(|S|+|T|) span for MED assume lock-free access to the shared memo table.

**Note:** Mt modules spawn a thread for **every** recursive call (not just at the top level). This creates an exponential number of threads before memoization kicks in. In practice, the shared memo table short-circuits most threads early, but the thread creation overhead is significant.

## Phase 5: Runtime Test Review

| # | Test File | Test Count | Coverage |
|---|-----------|:----------:|----------|
| 1 | `TestSubsetSumStEph.rs` | 24 | Basic, examples 49.1/49.2, edge cases, negative target, large k, memo behavior, set/clear, Display |
| 2 | `TestSubsetSumStPer.rs` | 16 | Same core tests; persistent semantics (original unchanged) |
| 3 | `TestSubsetSumMtEph.rs` | 12 | Parallel correctness, shared memo, set mutation |
| 4 | `TestSubsetSumMtPer.rs` | 10 | Parallel correctness, persistent semantics |
| 5 | `TestMinEditDistStEph.rs` | 20 | Example 49.3, identity, empty, single-char, all-different, same-length, Display |
| 6 | `TestMinEditDistStPer.rs` | 19 | Same core tests; persistent semantics |
| 7 | `TestMinEditDistMtEph.rs` | 22 | Parallel correctness, shared memo, set mutation |
| 8 | `TestMinEditDistMtPer.rs` | 13 | Parallel correctness, persistent semantics |

**Total: 136 runtime tests across 8 files.**

Test quality is solid:
- ✅ Textbook examples tested (SS {1,4,2,9} k=8 → false, k=12 → true; MED(ABCADA, ABADC) = 3)
- ✅ Edge cases: empty sequences, single element, target=0, target<0
- ✅ Memoization behavior tested (memo table populated, cleared on set)
- ✅ Persistent semantics: original object unchanged after computation
- ✅ Mt variants tested for correctness (same results as St)
- ✅ Display/Debug formatting
- ✅ IntoIterator (StEph, StPer variants)

| # | Gap | Severity | Notes |
|---|-----|----------|-------|
| 1 | No performance comparison St vs Mt | Low | Tests verify correctness but not that Mt is actually faster |
| 2 | No large-scale stress tests | Low | Largest test is ~10 elements; thread explosion risk not exercised |

## Phase 6: PTT Review

**No PTTs needed or present.** All 8 files have empty `verus!` blocks — no iterators, no verified loops, no ghost state.

## Phase 7: Gap Analysis

### Prose Items with No Implementation

| # | Prose Item | Status | Notes |
|---|-----------|--------|-------|
| 1 | Bottom-up DP table filling | Not implemented | All implementations use top-down memoization; prose discusses both approaches |
| 2 | Actual edit sequence (not just distance) | Not implemented | Prose says "easy to extend" as an exercise |
| 3 | Recursion DAG visualization | Not implemented | Examples 49.2, 49.5 show DAGs; code does not visualize |
| 4 | Greedy failure analysis | Not implemented | Example 49.4 discusses why greedy fails; not demonstrated in code |
| 5 | Pseudo-polynomial complexity analysis | Not implemented | No formal cost specifications |

### Code with No Prose Counterpart

| # | Item | Module | Purpose |
|---|------|--------|---------|
| 1 | `new()` constructors | All 8 modules | Create empty solver — utility |
| 2 | `multiset()`/`source()`/`target()` accessors | All 8 modules | Field access |
| 3 | `multiset_mut()`, `source_mut()`, `target_mut()` | StEph and MtEph variants | Ephemeral mutation accessors |
| 4 | `set()`, `set_source()`, `set_target()` | StEph and MtEph variants | Element mutation with memo clear |
| 5 | `clear_memo()`, `memo_size()` | All 8 modules | Memo table management |
| 6 | `IntoIterator` impls | StEph, StPer variants | Iteration over elements |
| 7 | `Display`, `Debug` impls | All 8 modules | Formatting |
| 8 | `PartialEq`, `Eq` impls | Mt variants | Equality by multiset/sequence only (ignoring memo) |
| 9 | `*Lit!` macros | All 8 modules | Convenience constructors |

## Phase 8: TOC and In/Out Review

### TOC Headers

| # | File | Has TOC | Notes |
|---|------|:-------:|-------|
| 1 | `SubsetSumStEph.rs` | Partial | Sections 4, 8, 9, 13 present but not numbered as standard TOC |
| 2 | `SubsetSumStPer.rs` | Partial | Same pattern |
| 3 | `SubsetSumMtEph.rs` | Partial | Sections 4, 8, 9, 11, 13 |
| 4 | `SubsetSumMtPer.rs` | Partial | Same pattern |
| 5 | `MinEditDistStEph.rs` | Partial | Sections 4, 8, 9, 13 |
| 6 | `MinEditDistStPer.rs` | Partial | Same pattern |
| 7 | `MinEditDistMtEph.rs` | Partial | Sections 4, 8, 9, 11, 13 |
| 8 | `MinEditDistMtPer.rs` | Partial | Same pattern |

All files have section comments but not the full formal TOC block at the top. Since code is outside `verus!`, this is acceptable.

### In/Out Table

All code is outside `verus!` due to `HashMap` usage. The `verus!` blocks are empty.

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|
| 1 | SubsetSumStEph | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out |
| 2 | SubsetSumStPer | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out |
| 3 | SubsetSumMtEph | ✅ out | ✅ out | - | - | - | ✅ out | ✅ out | ✅ out |
| 4 | SubsetSumMtPer | ✅ out | ✅ out | - | - | - | ✅ out | ✅ out | ✅ out |
| 5 | MinEditDistStEph | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out |
| 6 | MinEditDistStPer | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | ✅ out |
| 7 | MinEditDistMtEph | ✅ out | ✅ out | - | - | - | ✅ out | ✅ out | ✅ out |
| 8 | MinEditDistMtPer | ✅ out | ✅ out | - | - | - | ✅ out | ✅ out | ✅ out |

All placements correct — everything must be outside `verus!` because of `HashMap` dependency.

## Proof Holes Summary

```
✓ MinEditDistMtEph.rs
✓ MinEditDistMtPer.rs
✓ MinEditDistStEph.rs
✓ MinEditDistStPer.rs
✓ SubsetSumMtEph.rs
✓ SubsetSumMtPer.rs
✓ SubsetSumStEph.rs
✓ SubsetSumStPer.rs

Modules: 8 clean, 0 holed
Proof Functions: 0 total
Holes Found: 0 total
Errors: 0
```

**0 proof holes** — trivially clean because no Verus verification exists.

**0 structural errors.** The bare `impl` that previously existed in `MinEditDistMtEph.rs` has been fixed: `min_edit_distance_rec` is now a free function and `min_edit_distance` is in the trait impl.

## Spec Strength Summary

| Classification | Count |
|---|:---:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 72 |

All 72 functions have **no spec** — the entire chapter is unverified plain Rust.

## Overall Assessment

### Strengths

1. **Faithful algorithm implementation**: Both `subset_sum_rec` and `min_edit_distance_rec` match the prose algorithms (49.3 and 49.6) exactly in structure and base cases.
2. **Complete 4-variant coverage**: Each algorithm has St/Eph, St/Per, Mt/Eph, Mt/Per — the full APAS variant matrix.
3. **Genuine parallelism**: Mt variants use `thread::spawn` to fork both branches, achieving the prose's O(|S|) span for SS and O(|S|+|T|) span for MED (modulo lock contention).
4. **Good test coverage**: 136 tests across 8 files, including textbook examples, edge cases, memoization behavior, and persistence semantics.
5. **Correct memoization**: Top-down with HashMap faithfully implements the prose's "sharing of subproblems" via the recursion DAG.
6. **Cost annotations**: Every function has APAS and Claude cost annotations.
7. **Clean structure**: All modules follow the trait-impl pattern correctly with no bare impl violations.

### Weaknesses

1. **Zero formal verification**: `HashMap` dependency forces all code outside `verus!`. No specs, proofs, or invariants.
2. **Thread explosion in Mt variants**: Every recursive call spawns a thread before checking memo, creating exponential thread creation before memoization short-circuits. A threshold-based approach (parallelize only at top levels) would be more practical.
3. **Lock contention**: `Arc<Mutex<HashMap>>` serializes memo access. A concurrent hash map or lock-free structure would better exploit parallelism.
4. **No bottom-up DP**: Prose discusses both top-down and bottom-up approaches; only top-down implemented.
5. **No edit sequence recovery**: Only computes distance, not the actual edits.

### Review TODOs

| # | Priority | TODO | Notes |
|---|:--------:|------|-------|
| 1 | High | Add thread creation threshold | Parallelize only at top k levels to avoid exponential thread spawn |
| 2 | Medium | Consider verified DP approach | Replace HashMap with a 2D array (verifiable); enables Verus specifications |
| 3 | Medium | Add bottom-up DP variant | Prose discusses it; table-filling approach may be easier to verify |
| 4 | Low | Add edit sequence recovery | Prose mentions it as exercise |
| 5 | Low | Replace `Arc<Mutex<HashMap>>` with concurrent map | Reduce lock contention in Mt variants |
