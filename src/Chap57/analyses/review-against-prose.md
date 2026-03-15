# Review Against Prose: Chapter 57 -- Dijkstra's Algorithm

- Date: 2026-03-15
- Reviewer: Claude-Opus-4.6 (Agent 4, Round 21)
- Prose source: `prompts/Chap57.txt` (APAS Chapter 57: Dijkstra's Algorithm)

## Phase 1: Inventory

From `veracity-review-module-fn-impls.md`:

| # | Chap | File | Fns | Tr | IT | V! | -V! | Holes | NoSpec |
|---|------|------|-----|----|----|----|----|----|----|
| 1 | 57 | DijkstraStEphI64.rs | 4 | 1 | 2 | 4 | 0 | 0 | 2 |
| 2 | 57 | DijkstraStEphF64.rs | 2 | 0 | 2 | 0 | 2 | 0 | 2 |
| 3 | 57 | StackStEph.rs | 7 | 6 | 7 | 7 | 0 | 0 | 0 |

Total: 13 exec functions, 0 holes, 3 clean modules.

## Phase 2: Prose Inventory

| # | Chap | Prose Item | Type |
|---|------|-----------|------|
| 1 | 57 | Lemma 57.1 (Dijkstra's Property) | lemma |
| 2 | 57 | Algorithm 57.1 (Dijkstra's Algorithm) | algorithm |
| 3 | 57 | Theorem 57.2 (Correctness) | theorem |
| 4 | 57 | Algorithm 57.2 (Dijkstra with PQ) | algorithm |
| 5 | 57 | Example 57.1 (BFS failure) | example |
| 6 | 57 | Example 57.2 (frontier) | example |
| 7 | 57 | Example 57.3 (Dijkstra run) | example |
| 8 | 57 | Exercise 57.1 (decreaseKey) | exercise |

Cost analysis from prose:
- Dijkstra with PQ: Work O(m log n), Span O(m log n) (sequential).
- PQ operations (insert, deleteMin): O(log m) each, m calls each.
- Table operations (find, insert): O(log n) each.
- Total: O(m log n) work and span.
- With decreaseKey: O(m + n log n) work.

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

| # | Chap | File | Function | Annotation |
|---|------|------|----------|------------|
| 1 | 57 | DijkstraStEphI64.rs | dijkstra | APAS: W O(m log n), S O(m log n). Claude agrees. |
| 2 | 57 | DijkstraStEphI64.rs | pq_entry_new | APAS: N/A. Claude: O(1). |
| 3 | 57 | StackStEph.rs | new | APAS: (no cost stated). Claude: O(1). |
| 4 | 57 | StackStEph.rs | push | APAS: (no cost stated). Claude: O(1) amortized. |
| 5 | 57 | StackStEph.rs | pop | APAS: (no cost stated). Claude: O(1) amortized. |
| 6 | 57 | StackStEph.rs | peek | APAS: (no cost stated). Claude: O(1). |
| 7 | 57 | StackStEph.rs | is_empty | APAS: (no cost stated). Claude: O(1). |
| 8 | 57 | StackStEph.rs | size | APAS: (no cost stated). Claude: O(1). |

### 3b. Implementation Fidelity

**DijkstraStEphI64.rs (`dijkstra`)** vs Algorithm 57.2:

| # | Chap | Prose Step | Code | Match? |
|---|------|-----------|------|--------|
| 1 | 57 | Q0 = PQ.insert(0,s) empty | `BinaryHeapPQ::singleton(pq_entry_new(0, source))` | Yes |
| 2 | 57 | deleteMin Q | `pq.delete_min()` | Yes |
| 3 | 57 | if (v,_) in X skip | `if visited.mem(&v) { continue }` | Yes |
| 4 | 57 | X' = X union {(v,d)} | `visited.insert(v); sssp.set_distance(v, dist)` | Yes |
| 5 | 57 | relax: PQ.insert(d+w,u) | `pq = pq.insert(pq_entry_new(new_dist, u_idx))` | Yes |
| 6 | 57 | iterate relax over N+(v) | `for neighbor in iter: it` | Yes |

**Deviation**: The prose stores `(v, d)` in a visited table `X` that maps vertices to
distances. The code uses a separate `SetStEph<usize>` for visited membership and stores
distances in `SSSPResultStEphI64`. This is equivalent but splits the visited table into
two structures. The code also updates predecessors during relaxation (line 163: `if
sssp.get_distance(u_idx) > new_dist { sssp.set_predecessor(u_idx, v) }`), which the
prose does not mention explicitly -- it returns only `d(v)` values, not paths. This is
a standard extension for path reconstruction.

**DijkstraStEphF64.rs**: Blocked. Contains only PQEntry type definition and Ord/PartialOrd
impls. No `dijkstra` function. Blocked because `WeightedDirGraphStEphF64` does not exist.

**StackStEph.rs**: Standard LIFO stack backed by `Vec<T>`. Not explicitly described in the
prose but referenced as infrastructure. Prose does not define a stack -- this is Verus
scaffolding for general use (used by Dijkstra in some implementations).

### 3c. Spec Fidelity

**DijkstraStEphI64.rs (`dijkstra`)**:
- `requires`: `source < graph.vertices().size()`, `spec_labgraphview_wf(graph@)`, `valid_key_type_WeightedEdge`. Reasonable preconditions for a graph algorithm.
- `ensures`: `sssp.distances.spec_len() == graph.vertices().size()` and `sssp.source == source`.
- **Weakness**: No ensures relating the computed distances to shortest path distances (delta_G(s,v)). The ensures only state structural properties (length, source). The correctness theorem (Theorem 57.2: d(v) = delta_G(s,v)) is not expressed in the spec.
- **Assessment**: Weak. The ensures are structural, not algorithmic. A correct implementation of Dijkstra should ensure that the returned distances are shortest path distances, but this would require a spec function for delta_G which does not exist.

**StackStEph.rs**:
- All 6 trait functions have specs.
- `new` ensures `empty@ == Seq::empty()`.
- `push` ensures `self@ == old(self)@.push(item)`.
- `pop` ensures correct last/drop_last semantics.
- `peek`, `is_empty`, `size` all have appropriate ensures.
- **Assessment**: Strong. Full Seq-level specifications.

**PQEntry**: View returns `Self`. No meaningful spec -- it is a simple data carrier.

### F64 Differences

DijkstraStEphF64.rs is a **stub**: it defines PQEntry with WrappedF64 distance and
Ord/PartialOrd impls outside verus!, but the `dijkstra` function itself is commented out.
Blocked by the absence of `WeightedDirGraphStEphF64`.

## Phase 4: Parallelism Review

No Mt modules. Dijkstra is explicitly sequential in the prose ("it is a sequential
algorithm" -- Chapter 57 introduction). No parallelism expected.

## Phase 5: Runtime Test Review

| # | Chap | Test File | Covers |
|---|------|-----------|--------|
| 1 | 57 | TestDijkstraStEphI64.rs | DijkstraStEphI64 |
| 2 | 57 | TestDijkstraStEphF64.rs | DijkstraStEphF64 |
| 3 | 57 | TestStackStEph.rs | StackStEph |

Full RTT coverage for all 3 modules.

## Phase 6: PTT Review

No proof-time tests for Chapter 57. None needed -- no iterators (the stack does not
implement the iterator standard) and no complex callability.

## Phase 7: Gap Analysis

| # | Chap | Gap | Severity | Notes |
|---|------|-----|----------|-------|
| 1 | 57 | dijkstra ensures are structural only | High | No ensures that d(v) = delta_G(s,v). Correctness theorem (57.2) is not expressed. Would need a spec for delta_G. |
| 2 | 57 | DijkstraStEphF64 is a stub | Medium | No dijkstra function body. Blocked by missing WeightedDirGraphStEphF64 graph type. |
| 3 | 57 | pq_entry_new has `requires true` | Low | Vacuous precondition. Warning from veracity. Should be removed. |
| 4 | 57 | No spec_dijkstra function | Medium | No abstract spec capturing Dijkstra's correctness. Distance result lacks semantic connection to graph shortest paths. |
| 5 | 57 | Edge weight overflow | Low | Code does `dist + (*weight as i64)` without overflow checking. Graph uses i128 weights cast to i64. |
| 6 | 57 | No decreaseKey variant | Low | Exercise 57.1 not implemented. Expected -- exercises are optional. |

**Overall assessment**: Chapter 57 is **clean** (0 holes) but specs are **weak** for the
core algorithm. The `dijkstra` function has proven loop invariants for structural
properties but lacks algorithmic correctness ensures. The Stack is fully specified. F64
variant is blocked.

## Phase 8: TOC Review

- DijkstraStEphI64.rs: Follows standard (sections 1,2,4,5,8,9,13). Section numbering present.
- DijkstraStEphF64.rs: Minimal file, no TOC (acceptable for a stub).
- StackStEph.rs: Follows standard (sections 1,2,4,5,8,9,11,13). Section numbering present.

No ordering violations.
