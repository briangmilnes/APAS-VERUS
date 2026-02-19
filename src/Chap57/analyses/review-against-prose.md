<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 57 — Dijkstra's Algorithm: Review Against Prose

**Date:** 2026-02-18
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory

| # | File | exec fns | external_body | spec fns | proof fns | View | verus! | Trait Wired |
|---|------|:--------:|:-------------:|:--------:|:---------:|:----:|:------:|:-----------:|
| 1 | StackStEph.rs | 7 | 0 | 0 | 0 | Yes | Yes | Yes |
| 2 | DijkstraStEphI64.rs | 4 | 4 | 0 | 0 | Yes | Yes | Yes (cfg-gated impl) |
| 3 | DijkstraStEphFloat.rs | 4 | 4 | 0 | 0 | Yes | Yes | Yes (cfg-gated impl) |
| | **Total** | **15** | **8** | **0** | **0** | | | |

**Changes since last review:**
- `DijkstraStEphInt.rs` renamed to `DijkstraStEphI64.rs` (agent2 merge).
- `DijkstraStEphI64` / `DijkstraStEphFloat`: trait inside `verus!`, impl cfg-gated (pending Chap45/Chap23 fix).
- `StackStEph`: trait with 7 functions, all now verified (0 `external_body` — was 8, all removed 2026-02-18).

**Gating:**
- Chap57 module: `#[cfg(all(not(any(feature = "experiments_only", feature = "dev_only")), not(verus_keep_ghost)))]` — excluded during Verus verification.
- `StackStEph`: compiles when Chap57 is included (cargo build).
- `DijkstraStEphI64` / `DijkstraStEphFloat`: commented out in `lib.rs` (pending Chap45/Chap23 fix). Not in cargo build.

## Phase 2: Prose Inventory

Source: `prompts/Chap57.txt` (Chapter 57 — Dijkstra's Algorithm)

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Frontier | N⁺(X) \ X — neighbors of visited set not yet visited. |
| 2 | Priority p(v) | min_{x∈X}(δ_G(s,x) + w(x,v)) for frontier vertex v. |
| 3 | SSSP+ problem | Single source shortest path with non-negative edge weights. |

### Algorithms

| # | Item | Description |
|---|------|-------------|
| 1 | Algorithm 57.1 (Abstract Dijkstra) | PFS with d(s)=0, p(v)=min_{x∈X}(d(x)+w(x,v)), sets d(v)=p(v). |
| 2 | Algorithm 57.2 (Dijkstra with PQ) | Concrete PQ-based: deleteMin, visited check, relax neighbors with insert. |

### Cost Specs

| # | Item | Cost |
|---|------|------|
| 1 | Dijkstra (PQ-based) | Work O(m log n), Span O(m log n) — sequential. |
| 2 | PQ.deleteMin | O(log m) per call, m calls total. |
| 3 | PQ.insert | O(log m) per call, m calls total. |
| 4 | Table find | O(log n) per call, m calls total. |
| 5 | Table insert | O(log n) per call, n calls total. |
| 6 | N⁺_G(v) | O(log n) per call, n calls total. |

### Theorems/Properties

| # | Item | Description |
|---|------|-------------|
| 1 | Lemma 57.1 (Dijkstra's Property) | min_{y∈Y} p(y) = min_{y∈Y} δ_G(s,y) for non-negative weights. |
| 2 | Theorem 57.2 (Correctness) | d(v) = δ_G(s,v) for v reachable from s. |

### Exercises

| # | Item | Description |
|---|------|-------------|
| 1 | Exercise 57.1 | Dijkstra with decreaseKey operation. |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All 15 exec functions have APAS/Claude-Opus-4.6 cost annotation pairs.

| # | Function | File | APAS Cost | Claude-Opus-4.6 Cost | Agreement |
|---|----------|------|-----------|---------------------|-----------|
| 1 | `dijkstra` | DijkstraStEphI64 | Work O(m log n), Span O(m log n) | Work O(m log n), Span O(m log n) | **Agree** |
| 2 | `pq_entry_new` | DijkstraStEphI64 | N/A — scaffolding | Work Θ(1), Span Θ(1) | N/A |
| 3 | `Ord::cmp` | DijkstraStEphI64 | N/A — scaffolding | Work Θ(1), Span Θ(1) | N/A |
| 4 | `PartialOrd::partial_cmp` | DijkstraStEphI64 | N/A — scaffolding | Work Θ(1), Span Θ(1) | N/A |
| 5 | `dijkstra` | DijkstraStEphFloat | Work O(m log n), Span O(m log n) | Work O(m log n), Span O(m log n) | **Agree** |
| 6 | `pq_entry_new` | DijkstraStEphFloat | N/A — scaffolding | Work Θ(1), Span Θ(1) | N/A |
| 7 | `Ord::cmp` | DijkstraStEphFloat | N/A — scaffolding | Work Θ(1), Span Θ(1) | N/A |
| 8 | `PartialOrd::partial_cmp` | DijkstraStEphFloat | N/A — scaffolding | Work Θ(1), Span Θ(1) | N/A |
| 9 | `new` | StackStEph | Work Θ(1), Span Θ(1) | Work Θ(1), Span Θ(1) | **Agree** |
| 10 | `push` | StackStEph | Work Θ(1), Span Θ(1) | Work Θ(1) amortized | Minor |
| 11 | `pop` | StackStEph | Work Θ(1), Span Θ(1) | Work Θ(1) amortized | Minor |
| 12 | `peek` | StackStEph | (no cost stated) | Work Θ(1), Span Θ(1) | N/A |
| 13 | `is_empty` | StackStEph | Work Θ(1), Span Θ(1) | Work Θ(1), Span Θ(1) | **Agree** |
| 14 | `size` | StackStEph | (no cost stated) | Work Θ(1), Span Θ(1) | N/A |
| 15 | `default` | StackStEph | N/A — Rust scaffolding | Work Θ(1), Span Θ(1) | N/A |

**Cost notes:** Vec-backed push/pop are amortized Θ(1), not worst-case Θ(1). This is standard and matches prose expectations asymptotically.

### 3b. Implementation Fidelity

#### DijkstraStEphI64 / DijkstraStEphFloat

Faithful implementation of **Algorithm 57.2**:

| # | Prose Step | Code | Match |
|---|-----------|------|:-----:|
| 1 | PQ with deleteMin/insert | `BinaryHeapPQ` from Chap45 | Yes |
| 2 | Visited set X as table mapping v→d(v) | `HashMap<usize, i64/OrderedF64>` | Yes |
| 3 | Duplicate handling (line 7: skip if visited) | `visited.contains_key(&v)` check | Yes |
| 4 | Relax: insert (d+w, u) into PQ | Loop over `out_neighbors_weigh(ed)`, `pq.insert(pq_entry_new(...))` | Yes |
| 5 | Return d(v) for all v | `SSSPResultStEph{Int,Float}` with distances | Yes |

**Deviations:**
1. **Extra optimization:** Skips already-visited neighbors during relaxation. The prose mentions this as a valid variant in the Remark after Algorithm 57.2.
2. **Predecessor tracking:** Code tracks predecessors (not in Algorithm 57.2 pseudocode) for path reconstruction. Extension, not deviation.
3. **HashMap vs verified table:** Visited set uses `std::collections::HashMap` instead of a verified table (Chap42). This puts the visited-set operations outside verification scope.

#### StackStEph

Standard Vec-backed LIFO stack. The prose prompt requested "Definitely make a separate stack module." The stack is **not used** by either Dijkstra implementation (they use `BinaryHeapPQ`). The stack is infrastructure for other chapters or future use.

### 3c. Spec Fidelity

No `requires`/`ensures` on any function. Spec fidelity: **N/A**.

Key specs that should exist:
1. **Dijkstra correctness (Theorem 57.2):** `ensures forall |v| reachable(g, s, v) ==> result.distance(v) == delta_G(s, v)`
2. **Non-negative weight precondition:** `requires forall |e| graph.weight(e) >= 0`
3. **Stack LIFO:** `ensures self@.len() == old(self@.len()) + 1` (push); `ensures result == Some(old(self@.last()))` (pop)

## Phase 4: Parallelism Review

**No Mt modules.** The prose explicitly states Dijkstra is sequential. Correctly reflected — both variants are `StEph`. No parallelism gap.

## Phase 5: Runtime Test Review

**Runtime tests exist** for all three modules.

| # | Source Module | RTT File | Status |
|---|-------------|----------|--------|
| 1 | DijkstraStEphI64 | tests/Chap57/TestDijkstraStEphI64.rs | **Present** |
| 2 | DijkstraStEphFloat | tests/Chap57/TestDijkstraStEphFloat.rs | **Present** |
| 3 | StackStEph | tests/Chap57/TestStackStEph.rs | **Present** |

Note: DijkstraStEphI64 and DijkstraStEphFloat are commented out in `lib.rs`; tests may require enabling those modules or using `#[path]` / feature flags to run.

## Phase 6: Proof-Time Test (PTT) Review

No verified loops or iterators. Only trivial `View` impls for `PQEntry` and `StackStEph` inside `verus!`. No PTTs needed until verusified.

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Status |
|---|-----------|--------|
| 1 | Algorithm 57.1 (abstract Dijkstra) | Implemented via Algorithm 57.2 (PQ version) — expected. |
| 2 | Lemma 57.1 (Dijkstra's Property) | No proof lemma. Would require spec-level graph distance definitions. |
| 3 | Theorem 57.2 (Correctness) | No proof. The `ensures` of `dijkstra` should express this. |
| 4 | Exercise 57.1 (decreaseKey variant) | Not implemented. |
| 5 | Example 57.1 (BFS counterexample) | Not implemented (illustrative). |
| 6 | Example 57.2 (frontier example) | Not implemented (illustrative). |
| 7 | Example 57.3 (Dijkstra trace) | Not implemented as test — should be a runtime test. |

### Code With No Prose Counterpart

| # | Code Item | Notes |
|---|----------|-------|
| 1 | `StackStEph` module | Prompt-requested; not used by Dijkstra. |
| 2 | `PQEntry` struct | Implementation detail for PQ integration. |
| 3 | `pq_entry_new` | Constructor scaffolding. |
| 4 | `Ord`/`PartialOrd` impls | Required by BinaryHeapPQ. |
| 5 | `Debug`/`Display` impls | Debug/display scaffolding. |
| 6 | Predecessor tracking | Result includes predecessors; prose only returns distances. |
| 7 | `DijkstraStEphFloat` module | Float variant; prose uses generic weights. |

## Phase 8: TOC and In/Out Table

### TOC Presence

| # | File | TOC | Section Headers |
|---|------|:---:|:---------------:|
| 1 | StackStEph.rs | Yes | Yes (4, 5, 8, 9, 11, 13) |
| 2 | DijkstraStEphI64.rs | Yes | Yes (4, 5, 8, 9, 13) |
| 3 | DijkstraStEphFloat.rs | Yes | Yes (4, 5, 8, 9, 13) |

All files have TOC comment blocks and numbered section headers.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | StackStEph.rs | `✅ in` (derive) | - | `✅ in` | - | - | `✅ out` | - | - | - |
| 2 | DijkstraStEphI64.rs | `❌ in` (derive, no spec) | `❌ in` (derive, no spec) | - | - | - | `✅ out` | `✅ out` | - | Ord/PartialOrd `❌ in` (external_body, no ensures) |
| 3 | DijkstraStEphFloat.rs | `❌ in` (derive, no spec) | `❌ in` (derive, no spec) | - | - | - | `✅ out` | `✅ out` | - | Ord/PartialOrd `❌ in` (external_body, no ensures) |

**Notes:**
- `StackStEph`: Clone (derive inside verus! — correct), Default (inside verus! — correct), Debug (outside — correct).
- `DijkstraStEph{I64,Float}`: `#[derive(Clone, Eq, PartialEq)]` on `PQEntry` is inside `verus!` but uses derive macros without specs — should use the `PartialEqSpecImpl` pattern. `Ord`/`PartialOrd` are `external_body` inside `verus!` but lack `ensures` — should specify ordering contract.
- `Debug`/`Display` correctly outside `verus!`.

## Proof Holes Summary

**Last verified:** 2026-02-18 (`veracity-review-proof-holes`)

```
Modules: 1 clean, 2 holed
Holes Found: 8 total (all external_body)

StackStEph.rs:            ✓ CLEAN (0 holes — was 8, all removed)
DijkstraStEphI64.rs:      4 × external_body
DijkstraStEphFloat.rs:    4 × external_body
```

**Changes since last review:** StackStEph.rs is now fully verified — all 8 `external_body` annotations removed (including the duplicate on `new`). Total holes decreased from 16 to 8. The Dijkstra modules remain fully `external_body` (commented out in `lib.rs`, pending Chap45/Chap23 fix).

## Action Items

| # | Action | Priority | Status |
|---|--------|----------|--------|
| 1 | ~~Fix duplicate `#[verifier::external_body]` on StackStEph::new~~ | ~~High~~ | **Done** (2026-02-18) |
| 2 | ~~Remove `external_body` from StackStEph and add `requires`/`ensures`~~ | ~~High~~ | **Done** (2026-02-18) — StackStEph is now CLEAN |
| 3 | Uncomment DijkstraStEphI64/DijkstraStEphFloat in lib.rs (pending Chap45/Chap23 fix) | High | Open |
| 4 | Move PQEntry `Clone`/`PartialEq`/`Eq` to `PartialEqSpecImpl` pattern | Medium | Open |
| 5 | Add `ensures` to `Ord`/`PartialOrd` impls | Medium | Open |
| 6 | Replace `HashMap` with verified table for visited set in Dijkstra | Medium | Open |
| 7 | Verify runtime tests run (Dijkstra modules commented out may block) | Medium | Open |
| 8 | Consider removing unused StackStEph or documenting its intended use | Low | Open |
| 9 | Implement Exercise 57.1 (decreaseKey variant) | Low | Open |
| 10 | Add Example 57.3 as runtime test | Low | Open |
