# Review Against Prose: Chapter 58 -- Bellman-Ford's Algorithm

- Date: 2026-03-15
- Reviewer: Claude-Opus-4.6 (Agent 4, Round 21)
- Prose source: `prompts/Chap58.txt` (APAS Chapter 58: Bellman-Ford's Algorithm)

## Phase 1: Inventory

From `veracity-review-module-fn-impls.md`:

| # | Chap | File | Fns | Tr | ML | V! | -V! | Holes | NoSpec |
|---|------|------|-----|----|----|----|----|----|----|
| 1 | 58 | BellmanFordStEphI64.rs | 2 | 1 | 2 | 1 | 1 | 0 | 2 |
| 2 | 58 | BellmanFordStEphF64.rs | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

Total: 2 exec functions (I64 only), 0 holes, 2 clean modules.

Note: `BellmanFordStEphI64.rs` has the trait declaration inside `verus!` but the
implementation is **entirely outside verus!** (`#[cfg(not(verus_keep_ghost))]`). This means
the algorithmic code is NOT verified by Verus -- it only compiles for runtime tests.

## Phase 2: Prose Inventory

| # | Chap | Prose Item | Type |
|---|------|-----------|------|
| 1 | 58 | Definition 58.1 (k-hop Distance) | definition |
| 2 | 58 | Algorithm 58.2 (Bellman-Ford) | algorithm |
| 3 | 58 | Theorem 58.1 (Correctness) | theorem |
| 4 | 58 | Example 58.1 (currency exchange) | example |
| 5 | 58 | Example 58.2 (Dijkstra failure) | example |
| 6 | 58 | Example 58.3 (k-hop) | example |
| 7 | 58 | Example 58.4 (BF steps) | example |
| 8 | 58 | Exercise 58.1 (neg cycle) | exercise |
| 9 | 58 | Exercise 58.2 (simple paths) | exercise |

Cost analysis from prose:
- With tables: Work O(nm log n), Span O(n log n).
- With sequences: Work O(nm), Span O(n log n).
- n rounds, each round: O(n+m) work with sequences, O(log n) span (parallel over vertices).

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

| # | Chap | File | Function | Annotation |
|---|------|------|----------|------------|
| 1 | 58 | BellmanFordStEphI64.rs | bellman_ford | APAS: W O(nm), S O(n log n) (sequences). Claude: W O(nm), S O(nm) -- sequential implementation. |
| 2 | 58 | BellmanFordStEphI64.rs | reconstruct_predecessors | APAS: N/A. Claude: W O(nm), S O(nm). |

### 3b. Implementation Fidelity

**BellmanFordStEphI64.rs (`bellman_ford`)** vs Algorithm 58.2:

| # | Chap | Prose Step | Code | Match? |
|---|------|-----------|------|--------|
| 1 | 58 | D0 = {v -> inf, s -> 0} | `distances.insert(v, if v == source { 0 } else { i64::MAX })` | Yes |
| 2 | 58 | Din(v) = min over in-neighbors | `for Pair(u, weight) in in_neighbors.iter()` with min tracking | Yes |
| 3 | 58 | D' = {v -> min(D[v], Din(v))} | `new_distances.insert(v, min_dist)` | Yes |
| 4 | 58 | if k = \|V\| then None | `if round == n - 1 { return Err(...) }` | Yes |
| 5 | 58 | if all D[v] = D'[v] then Some D | `if !changed { return Ok(sssp) }` | Yes |
| 6 | 58 | else BF D' (k+1) | Loop iteration | Yes |

**Deviations**:
- Uses `HashMap` instead of a table ADT. Functionally equivalent for runtime.
- Uses `saturating_add` for distance arithmetic instead of checked_add. This silently
  clamps overflow to `i64::MAX` rather than propagating overflow correctly. For very
  large negative weights, this could mask arithmetic issues. However, since UNREACHABLE
  vertices are excluded (u_dist != i64::MAX), this is safe in practice.
- `reconstruct_predecessors` is not in the prose. The prose returns only distances, not
  predecessor arrays. This is a standard extension.
- The implementation is **sequential** -- the prose describes parallel computation of D'
  with Span O(log n) per round. The code iterates vertices sequentially in a `for v in
  0..n` loop.

**BellmanFordStEphF64.rs**: Empty stub. Contains only an empty `verus!` block and commented-out
function signature. Blocked by missing `WeightedDirGraphStEphF64`.

### 3c. Spec Fidelity

**BellmanFordStEphI64.rs (`bellman_ford`)**:
- Trait declaration: `fn bellman_ford(...) -> Result<SSSPResultStEphI64, String>` with **NoSpec** (no requires, no ensures).
- The implementation is outside verus! entirely -- no Verus verification occurs.
- **Assessment**: No spec. The function is completely unverified. This is the weakest
  possible state for a core algorithm.

**Structural observation**: The file has an unusual architecture. The trait is declared
inside `verus!` but has no body or impl inside `verus!`. The implementation uses
`HashMap` (from std, not verifiable by Verus) and is gated behind `#[cfg(not(verus_keep_ghost))]`.
This means:
- Verus sees the trait declaration but no implementation.
- The runtime code works correctly but is not verified.
- The trait cannot be implemented inside verus! without replacing HashMap with a
  Verus-compatible table type.

## Phase 4: Parallelism Review

No Mt module exists for Bellman-Ford. The prose describes significant parallelism:
- The tabulate over vertices on lines 5-6 is parallel: Span O(log n) per round.
- Total span: O(n log n).

A parallel implementation would require a BellmanFordMtEph module. This is a gap.

## Phase 5: Runtime Test Review

| # | Chap | Test File | Covers |
|---|------|-----------|--------|
| 1 | 58 | TestBellmanFordStEphI64.rs | BellmanFordStEphI64 |
| 2 | 58 | TestBellmanFordStEphF64.rs | BellmanFordStEphF64 |

Both modules have test files. However, BellmanFordStEphF64 tests may be trivial since the
module is an empty stub.

## Phase 6: PTT Review

No proof-time tests for Chapter 58. Not applicable since the algorithmic code is outside
verus!.

## Phase 7: Gap Analysis

| # | Chap | Gap | Severity | Notes |
|---|------|-----|----------|-------|
| 1 | 58 | bellman_ford is entirely outside verus! | Critical | Core algorithm has no Verus verification. No requires, no ensures, no proof. |
| 2 | 58 | No BellmanFordMtEph module | High | Prose describes parallel BF with Span O(n log n). No parallel implementation exists. |
| 3 | 58 | BellmanFordStEphF64 is empty stub | Medium | Blocked by missing WeightedDirGraphStEphF64. |
| 4 | 58 | Uses HashMap (not Verus-compatible) | High | HashMap prevents verusification. Must switch to a Verus table type (e.g., ArraySeqStEph). |
| 5 | 58 | No correctness spec | High | Theorem 58.1 (d(v) = delta_G(s,v) or neg cycle) not formalized. |
| 6 | 58 | saturating_add instead of checked_add | Low | Silently clamps overflow. Unlikely to cause issues for practical inputs. |
| 7 | 58 | reconstruct_predecessors outside verus! | Medium | Predecessor reconstruction is unverified scaffolding. |

**Verusification path**: To verusify Bellman-Ford:
1. Replace `HashMap<usize, i64>` with `ArraySeqStEphS<i64>` (indexed by vertex).
2. Move implementation inside `verus!`.
3. Add loop invariants: k-hop distance property, convergence detection.
4. Add ensures: `result.Ok ==> forall|v| distances[v] == delta_G(s,v)`.
5. Add negative cycle detection ensures.

**Overall assessment**: Chapter 58 is technically **clean** (0 holes) because the code is
entirely outside verus! -- veracity does not count unverified code as holes. However, the
algorithmic content is **completely unverified**. This is the weakest chapter in the
shortest-paths family. The code runs correctly at runtime (tests pass) but has no formal
proof.

## Phase 8: TOC Review

BellmanFordStEphI64.rs has a minimal structure: trait inside verus!, implementation outside
with cfg gates. No full TOC comment. This is acceptable given the current unverified state
but should be restructured when verusified.

BellmanFordStEphF64.rs is an empty stub with no TOC. Acceptable.
