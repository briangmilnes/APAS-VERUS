# APAS-VERUS

Formally verified implementations of algorithms from "Algorithms Parallel and Sequential" by Acar, Blelloch, and Milnes, using the [Verus](https://github.com/verus-lang/verus) verification framework for Rust.

**Verified: 4 algorithms, 9 data structures**

## Project Structure

- `src/` - Verified algorithm implementations organized by chapter
- `src/vstdadditions/` - Extensions to the Verus standard library
- `tests/` - Rust unit tests for algorithm correctness
- `benches/` - Performance benchmarks using Criterion
- `attic/` - Old/deprecated implementations

## Verified Data Structures

### Chapter 05: Sets, Relations, and Mappings - ✅ COMPLETE

| Data Structure | Verified | Notes |
|----------------|----------|-------|
| `SetStEph<T>` | ✅ | Ephemeral set with `to_seq()`, iterator proofs |
| `RelationStEph<T, U>` | ✅ | Binary relations |
| `MappingStEph<K, V>` | ✅ | Key-value mappings |

### Chapter 06: Graphs - ✅ COMPLETE

| Data Structure | Verified | Notes |
|----------------|----------|-------|
| `DirGraphStEph<V>` | ✅ | Directed graph |
| `UnDirGraphStEph<V>` | ✅ | Undirected graph |
| `LabDirGraphStEph<V, L>` | ✅ | Labeled directed graph |
| `LabUnDirGraphStEph<V, L>` | ✅ | Labeled undirected graph |
| `WeightedDirGraphStEphU32<V>` | ✅ | Weighted directed graph with `total_weight()` proven |

**Weighted Graph Variants (proven, not compiled by default):**
- All unsigned: `U8`, `U16`, `U32`, `U64`, `U128`, `Usize`
- All signed: `I8`, `I16`, `I32`, `I64`, `I128`, `Isize`

### vstdplus Library Extensions

| Module | Description |
|--------|-------------|
| `arithmetic/power2_plus` | Power of 2 lemmas (`lemma_pow2_mono`, bounds) |
| `checked_nat` | Overflow-checked unsigned integers (`CheckedU8`..`CheckedU128`) |
| `checked_int` | Overflow-checked signed integers (`CheckedI8`..`CheckedI128`) |
| `clone_plus` | `ClonePlus` trait for Verus-compatible cloning |
| `feq` | Functional equality |
| `hash_set_specs` | HashSet specification helpers |
| `hash_set_with_view_plus` | Enhanced `HashSet` with iterator specs |
| `partial_order` | `PartialOrdered` trait |
| `pervasives_plus` | Common utility functions |
| `seq` | Sequence lemmas |
| `seq_set` | Lemmas connecting `Seq` and `Set` operations, weighted sums |
| `threads_plus` | Verified thread primitives (`spawn_plus`, `JoinHandlePlus`) |
| `total_order` | `TotalOrdered` trait for all 12 integer types |
| `VecQueue` | Verified queue using `Vec` |

## Algorithm Status

### Chapter 02: Scheduling - ✅ COMPLETE

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Work-Stealing Scheduler | ✅ | ✅ | ✅ | ⬜ | `Pool` with bounded parallelism, verified `join` propagates `ensures` |

**Scheduling Infrastructure:**
- `Pool` - Bounded thread pool with `Pool::new(n)` and `pool.join(fa, fb)`
- `join` - Verified parallel join, propagates closure `requires`/`ensures` through `spawn_plus`
- `threads_plus` - Verus-compatible wrappers for `std::thread` (`spawn_plus`, `JoinHandlePlus`)

### Chapter 03: Sorting - ✅ COMPLETE

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Insertion Sort | ✅ | ⬜ | ✅ | ✅ | Generic over `T: TotalOrdered + Copy`, multiset preservation proven |

### Chapter 11: Fibonacci - ✅ COMPLETE

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Fibonacci (Sequential) | ✅ | ⬜ | ✅ | ⬜ | `fib_seq` with overflow proofs via `lemma_fib_sum_fits_u64` |
| Fibonacci (Pool) | ✅ | ✅ | ✅ | ⬜ | `fib_pool` using `Pool.join`, verified `ensures` propagation |
| Fibonacci (ParaPair) | ✅ | ✅ | ✅ | ⬜ | `ParaPairDisjoint!` macro for fork-join |

**Parallelism Infrastructure:**
- `Pool` - Work-stealing scheduler with `pool.join(fa, fb)` (see Chapter 02)
- `ParaPairs.rs` - Disjoint parallel pair abstraction
- `para_pair_disjoint` - Verified function with `f.requires()`/`f.ensures()` propagation
- `ParaPairDisjoint!` - Macro for verified parallel fork-join

### Chapter 05: Sequences and Series - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Merge | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 06: Quicksort - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Quicksort | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 11: Binary Search - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Binary Search | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 12: Hash Tables - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Hash Tables | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 17: Shortest Paths - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Dijkstra's Algorithm | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 18: Minimum Spanning Trees - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Kruskal's Algorithm | ⬜ | ⬜ | ⬜ | ⬜ | |
| Prim's Algorithm | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 19: Sequences - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Array Sequences | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 21: Augmented Trees - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Augmented Trees | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 23: Range Trees - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Range Trees | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 26: Divide and Conquer - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Divide & Conquer Reduce | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 27: Parallel Scan and Reduce - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Scan Contract | ⬜ | ⬜ | ⬜ | ⬜ | |
| Reduce Contract | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 28: Work-Span Analysis - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Work-Span Models | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 35: Order Statistics - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Order Statistics Select | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 36: Integer Sorting - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Radix Sort | ⬜ | ⬜ | ⬜ | ⬜ | |
| Counting Sort | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 37: String Sorting - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| String Sorting | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 38: Parallel BST - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| BST Para | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 39: 2-3 Trees - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| 2-3 Trees | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 40: Red-Black Trees - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Red-Black Trees | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 41: AVL Trees - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| AVL Tree Sets | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 42: Hash Tables (Advanced) - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Advanced Hash Tables | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 43: Priority Queues - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Binary Heaps | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 44: Disjoint Sets - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Union-Find | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 45: Suffix Trees - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Suffix Trees | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 47: Dynamic Programming - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Dynamic Programming | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 49: Maxflow - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Maxflow Algorithms | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 50: Linear Programming - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Simplex Method | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 51: Fast Fourier Transform - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| FFT | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 52: Convex Hull - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Convex Hull | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 53: Computational Geometry - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Geometry Algorithms | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 54: Delaunay Triangulation - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Delaunay Triangulation | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 55: Graph Algorithms - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| DFS | ⬜ | ⬜ | ⬜ | ⬜ | |
| Cycle Detection | ⬜ | ⬜ | ⬜ | ⬜ | |
| Topological Sort | ⬜ | ⬜ | ⬜ | ⬜ | |
| Strongly Connected Components | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 56: Dynamic Graphs - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Dynamic Connectivity | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 57: Approximation Algorithms - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Approximation Algorithms | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 58: Randomized Algorithms - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Randomized Algorithms | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 59: Streaming Algorithms - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Streaming Algorithms | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 61: Machine Learning Basics - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| ML Basics | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 62: Neural Networks - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Neural Networks | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 63: Cryptographic Algorithms - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Crypto Algorithms | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 64: Compression - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Compression | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 65: Error Correction - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Error Correction | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 66: Quantum Algorithms - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Quantum Algorithms | ⬜ | ⬜ | ⬜ | ⬜ | |

---

**Algorithms to prove before I sleep: 41 chapters, 227 algorithm variants remaining**

---

**Legend:**
- ✅ Complete - All algorithms in chapter verified, tested, and benchmarked
- 🔄 In Progress - Some work done but not complete
- ⬜ Not Started - No work begun

## Building and Testing

### Verification

```bash
# Verify all code with cargo-verus (uses incremental caching)
cargo-verus verify

# Verify with timing breakdown
cargo-verus verify -- --time-expanded

# Verify only a specific module
verus src/lib.rs --crate-type=lib --verify-only-module Chap03::InsertionSortStEph::InsertionSortStEph
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test --test TestInsertionSortStEph
```

### Benchmarking

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench BenchInsertionSortStEph
```

## Development Setup

1. Install [Verus](https://github.com/verus-lang/verus) and ensure it's in your PATH
2. Install Rust toolchain 1.88.0 (pinned in `rust-toolchain.toml`)
3. Clone this repository
4. Run `cargo-verus verify` to verify all code
5. Run `cargo test` to run tests

## Verification Approach

We use Verus to prove:
- **Functional correctness**: Algorithms satisfy their specifications (e.g., sorted output, correct traversal order)
- **Memory safety**: No undefined behavior, proper bounds checking
- **Resource properties**: Multiset preservation (e.g., sorting doesn't lose/add elements)

For generic algorithms, we use traits like `TotalOrdered` to abstract over ordering relationships while maintaining provability.

## experiments

The `src/experiments/` directory contains explorations of Verus verification techniques and proofs of fundamental loop patterns:

### Loop Verification Study

A comprehensive study of how Verus verifies different loop constructs (`while`, `loop`, `for`) with various iterator patterns:

- **`seq_while_basic_proofs.rs`** - Verified `while` loops for basic sequence operations
- **`seq_loop_basic_proofs.rs`** - Same operations using `loop { ... return }` patterns 
- **`seq_for_basic_proofs.rs`** - Same operations using `for` loops with range iterators
- **`verus_wrapped_iter_loops.rs`** - Manual desugaring of `for` loop auto-invariants

See `docs/` for detailed write-ups on Verus loop proofs and iterator patterns.

## License

Copyright (C) 2025 Acar, Blelloch and Milnes

## References

- [Algorithms Parallel and Sequential](http://www.parallel-algorithms-book.com/)
- [Verus Documentation](https://verus-lang.github.io/verus/)
- [APAS-AI Project](https://github.com/your-repo/APAS-AI) - Original unverified Rust implementations

