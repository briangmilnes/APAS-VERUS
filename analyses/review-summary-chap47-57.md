<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Review Summary: Chapters 47–57

**Date:** 2026-02-17
**Reviewer:** Claude-Opus-4.6 (agent2)
**Scope:** 10 chapters, 80 source files, ~12,200 lines

## Proof Holes Summary

| # | Chapter | Files | external_body | unsafe | assume | admit | Total Holes | Notes |
|---|---------|:-----:|:-------------:|:------:|:------:|:-----:|:-----------:|-------|
| 1 | Chap47 | 9 | 9 | 1 | 0 | 0 | 10 | ~85% code outside verus! (dyn Fn, LinkedList, recursive structs) |
| 2 | Chap49 | 8 | 0 | 0 | 0 | 0 | **0** | All code outside verus! — vacuously clean |
| 3 | Chap50 | 9 | 0 | 0 | 0 | 0 | **0** | All code outside verus! — vacuously clean |
| 4 | Chap51 | 8 | 0 | 0 | 0 | 0 | **0** | All code outside verus! — vacuously clean |
| 5 | Chap52 | 14 | 146 | 0 | 0 | 0 | 146 | All functions external_body |
| 6 | Chap53 | 5 | 23 | 0 | 0 | 0 | 23 | Gated: depends on Chap37/41 |
| 7 | Chap54 | 4 | 5 | 0 | 0 | 0 | 5 | |
| 8 | Chap55 | 8 | 26 | 0 | 0 | 0 | 26 | Gated: depends on Chap37/41 |
| 9 | Chap56 | 12 | 69 | 0 | 0 | 0 | 69 | Float variants use f64 |
| 10 | Chap57 | 3 | 15 | 0 | 0 | 0 | 15 | Dijkstra gated: depends on Chap45 |
| | **Total** | **80** | **293** | **1** | **0** | **0** | **294** | |

### Interpretation

- **Chap49, 50, 51** report 0 holes because Verus limitations forced all code outside `verus!` blocks. The `verus!` blocks are empty. This is **vacuously clean** — there is nothing to verify.
- **Chap52** has the most holes (146) because it has the most files (14) and every exec function is `external_body`.
- **Chap53, 55, 57** are additionally gated with `#[cfg(not(verus_keep_ghost))]` because they depend on unverified chapters (37, 41, 45).

## Verus Verification Status

| # | Chapter | Inside verus! | Outside verus! | Specs | Reason Code Outside |
|---|---------|:------------:|:--------------:|:-----:|---------------------|
| 1 | Chap47 | ~15% | ~85% | 0 | `dyn Fn`, `LinkedList`, recursive structs |
| 2 | Chap49 | ~0% | ~100% | 0 | `&mut` return types, `HashMap` |
| 3 | Chap50 | ~0% | ~100% | 0 | `&mut` return types, `HashMap` |
| 4 | Chap51 | ~0% | ~100% | 0 | `Arc<Mutex<HashMap>>`, `Arc<Mutex<Vec<Vec>>>` |
| 5 | Chap52 | ~100% | ~0% | 0 | All inside, all `external_body` |
| 6 | Chap53 | ~100% | ~0% | 0 | All inside, all `external_body` |
| 7 | Chap54 | ~100% | ~0% | 0 | All inside, all `external_body` |
| 8 | Chap55 | ~100% | ~0% | 0 | All inside, all `external_body` |
| 9 | Chap56 | ~100% | ~0% | 0 | All inside, all `external_body` |
| 10 | Chap57 | ~90% | ~10% | 0 | Mostly inside, `Debug` outside |

**Zero specs across all 10 chapters.** No `requires`, `ensures`, `spec fn`, or `proof fn` exists in any of these files.

## Action Items by Priority

### P0 / HIGH — Correctness bugs and critical structural issues

| # | Chapter | Description |
|---|---------|-------------|
| 1 | Chap47 | Fix `hash_index` placeholder — always returns 0 instead of calling hash function |
| 2 | Chap47 | Fix `num_elements` tracking in chained insert/delete (never updated) |
| 3 | Chap51 | Remove substitute branch from TopDown `med_recursive` to match APAS Algorithm 51.4 |
| 4 | Chap51 | Add cross-variant test: verify MED("a","b") gives same result from TopDown and BottomUp |
| 5 | Chap49 | Move algorithmic code into verus! (restructure to avoid HashMap and `&mut` returns) |
| 6 | Chap49 | Add spec functions for DP recurrences (`spec_subset_sum`, `spec_min_edit_dist`) |
| 7 | Chap49 | Add `requires`/`ensures` to main algorithms |

### P1 / HIGH — Verification and spec gaps

| # | Chapter | Description |
|---|---------|-------------|
| 8 | Chap52 | Add `requires`/`ensures` and begin removing `external_body` |
| 9 | Chap52 | Implement parallelism in AdjSeqGraphMtEph, AdjSeqGraphMtPer, AdjMatrixGraphMtEph |
| 10 | Chap52 | Parallelize AdjTableGraphMtPer::delete_vertex and num_edges |
| 11 | Chap53 | Remove `external_body` from exec fns and add `requires`/`ensures` |
| 12 | Chap53 | Add runtime tests for all 5 modules |
| 13 | Chap54 | Remove `external_body` from BFS St variants and add `requires`/`ensures` |
| 14 | Chap55 | Fix `Vec::insert(0,..)` → `push` + `reverse` in topo sort and SCC (O(n²) → O(n)) |
| 15 | Chap55 | Add TOC comment blocks to all 8 files |
| 16 | Chap55 | Remove `not(verus_keep_ghost)` gate when Chap37/41 verified |
| 17 | Chap56 | Remove `external_body` from SSSP/AllPairs result types and add `requires`/`ensures` |
| 18 | Chap56 | Add runtime tests |
| 19 | Chap57 | Add runtime tests for Dijkstra and Stack |
| 20 | Chap57 | Remove `external_body` from StackStEph and add `requires`/`ensures` |

### P2 / MEDIUM — Implementation fidelity and spec quality

| # | Chapter | Description |
|---|---------|-------------|
| 21 | Chap47 | Investigate removing `external_body` from `FlatEntry` methods |
| 22 | Chap47 | Add auto-resize logic |
| 23 | Chap47 | Redesign `HashTable` to avoid `dyn Fn` |
| 24 | Chap49 | Add View impls and PartialEqSpecImpl |
| 25 | Chap49 | Add bottom-up DP variants |
| 26 | Chap50 | Move simple accessors inside verus! with specs |
| 27 | Chap50 | Add spec functions for OBST and MatrixChain cost formulas |
| 28 | Chap51 | Remove or implement unused traits |
| 29 | Chap52 | Add proper View types (EdgeSet/AdjTable currently use `Self`) |
| 30 | Chap52 | Implement weighted graph variants (Section 3.5) |
| 31 | Chap52 | Implement map_vertices/map_edges operations |
| 32 | Chap53 | Fix SelectOne to use recency-based selection (true DFS) |
| 33 | Chap53 | Fix PQMin `find_min_priority` to use O(log n) extraction |
| 34 | Chap53 | Populate parent tree in SearchResult |
| 35 | Chap54 | Make BFSMtEph genuinely parallel (currently sequential despite Mt name) |
| 36 | Chap54 | Add `requires`/`ensures` to BFSMtPer |
| 37 | Chap55 | Replace AVLTreeSet with array-based visited in DFSStPer, CycleDetectStPer |
| 38 | Chap55 | Fix SCCStEph component accumulation (rebuilds Vec per component) |
| 39 | Chap55 | Add `requires`/`ensures` to all 26 functions |
| 40 | Chap56 | Add `spec fn spec_distance` for δ_G(u,v) |
| 41 | Chap56 | Remove `external_body` from PathWeightUtils and verify |
| 42 | Chap57 | Move PQEntry Clone/PartialEq/Eq to PartialEqSpecImpl pattern |
| 43 | Chap57 | Add `ensures` to Ord/PartialOrd impls |
| 44 | Chap57 | Replace HashMap with verified table for visited set |

### P3 / LOW — Polish, exercises, and missing variants

| # | Chapter | Description |
|---|---------|-------------|
| 45 | Chap47 | Remove unused ChainEntry struct |
| 46 | Chap47 | Replace unsafe pointer arithmetic in second_hash |
| 47 | Chap49 | Add granularity cutoff to Mt variants |
| 48 | Chap49 | Add PTTs and full TOC blocks |
| 49 | Chap50 | Implement Exercise 50.5 (return optimal tree structure) |
| 50 | Chap51 | Consider moving base-case code inside verus! |
| 51 | Chap51 | Add threshold to Mt diagonal spawning |
| 52 | Chap52 | Create missing source files: EdgeSetGraphMtEph, AdjTableGraphMtEph |
| 53 | Chap52 | Create missing test files: TestAdjSeqGraphMtEph, TestAdjMatrixGraphMtEph |
| 54 | Chap52 | Upgrade PartialEq impls to PartialEqSpecImpl pattern |
| 55 | Chap52 | Implement Exercise 52.5 (constant-span edge deletion via inject) |
| 56 | Chap53 | Add formal TOC comment blocks |
| 57 | Chap53 | Add spec fns for reachability and frontier correctness |
| 58 | Chap54 | Implement Algorithm 54.5 (BFSDistance) and 54.6 (BFS Tree) |
| 59 | Chap55 | Implement generic DFS framework (Alg 55.4/55.5) |
| 60 | Chap55 | Implement stack-based DFS (Alg 55.1) |
| 61 | Chap55 | Implement DFS Numbers module (Def 55.6) |
| 62 | Chap56 | Fix module header cost for validate_subpath_property (O(k²) → O(k)) |
| 63 | Chap57 | Implement Exercise 57.1 (decreaseKey variant) |
| 64 | Chap57 | Consider removing unused StackStEph |

## Runtime Test Coverage

| # | Chapter | Source Files | Test Files | Tests | Coverage |
|---|---------|:-----------:|:----------:|:-----:|----------|
| 1 | Chap47 | 9 | 7 | ~81 | Missing: ChainedHashTable, FlatHashTable (base trait files) |
| 2 | Chap49 | 8 | 8 | ~136 | Full |
| 3 | Chap50 | 9 | 9 | ~100+ | Full |
| 4 | Chap51 | 8 | 8 | ~112 | Full |
| 5 | Chap52 | 14 | 13 | ~150+ | Missing: TestAdjSeqGraphMtEph, TestAdjMatrixGraphMtEph |
| 6 | Chap53 | 5 | 5 | ~50+ | Full |
| 7 | Chap54 | 4 | 4 | ~28 | Full |
| 8 | Chap55 | 8 | 8 | ~63 | Full |
| 9 | Chap56 | 12 | 12 | ~100+ | Full |
| 10 | Chap57 | 3 | 3 | ~30+ | Full |

## Dependency Gates

These chapters cannot be verified by Verus until their dependencies are also verusified:

| # | Chapter | Gated By | Depends On |
|---|---------|----------|------------|
| 1 | Chap52 (AdjTable*, EdgeSet*) | `not(verus_keep_ghost)` | Chap37, 41, 43 |
| 2 | Chap53 (all files) | `not(verus_keep_ghost)` | Chap37, 41 |
| 3 | Chap55 (all files) | `not(verus_keep_ghost)` | Chap37, 41 |
| 4 | Chap57 (Dijkstra files) | `not(verus_keep_ghost)` | Chap45 |

## Strategic Observations

1. **The DP chapters (49, 50, 51) are the hardest to verusify** because their core data structures (HashMap for memoization, Arc/Mutex for parallelism) are fundamentally incompatible with Verus. A bottom-up tabulation approach using Vec/ArraySeq would be the most viable path forward.

2. **The graph chapters (52, 54, 56) are the most promising** for immediate verification work because their code is already inside `verus!` blocks and the algorithms are structurally simple (array indexing, loops).

3. **Chap47 has actual bugs** (hash_index returns 0, num_elements not maintained). These should be fixed regardless of verification status.

4. **Chap51 has an algorithm deviation** — the TopDown MED implementation includes a substitute operation not in APAS Algorithm 51.4. This produces different results from BottomUp on certain inputs.

5. **The common critical path is Chap37 (AVLTreeSet) → Chap41 (AVLTreeMap)** — verifying these unlocks Chap52 (AdjTable/EdgeSet), Chap53, and Chap55.
