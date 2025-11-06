# APAS-VERUS

Formally verified implementations of algorithms from "Algorithms Parallel and Sequential" by Acar, Blelloch, and Milnes, using the [Verus](https://github.com/verus-lang/verus) verification framework for Rust.

## Project Structure

- `src/` - Verified algorithm implementations organized by chapter
- `src/vstdadditions/` - Extensions to the Verus standard library
- `tests/` - Rust unit tests for algorithm correctness
- `benches/` - Performance benchmarks using Criterion
- `attic/` - Old/deprecated implementations

## vstdadditions

This directory contains utilities that extend the Verus standard library (`vstd`) with commonly needed functionality:

### TotalOrdered Trait

The `TotalOrdered` trait (from the [Verus guide BST example](https://verus-lang.github.io/verus/guide/container_bst_generic.html)) connects:
- **Spec-level ordering**: `spec fn le(self, other: Self) -> bool` with mathematical properties (reflexive, transitive, antisymmetric, total)
- **Executable comparison**: `fn compare(&self, other: &Self) -> Cmp` with ensures clauses that connect to the spec

This trait enables writing generic verified sorting and ordering algorithms. We provide implementations for all 12 Rust integral types: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.

**Why not in vstd?** This trait is currently only in tutorial examples. We include it here as a reusable component for any algorithm requiring total orderings.

## Lines of Code

```
Verus LOC (Spec/Proof/Exec)

       0/       0/       0 benches/Chap03/BenchInsertionSortStEph.rs
       6/       0/      58 src/Chap03/InsertionSortStEph.rs
       0/       0/      37 src/experiments/ArrayVal.rs
       0/       0/      45 src/experiments/ArrayVecSet.rs
       0/       0/      60 src/experiments/ForFor.rs
       0/       0/      21 src/experiments/ForLoops.rs
       0/       0/      67 src/experiments/WhileWhile.rs
       0/       0/       0 src/lib.rs
      71/     141/     124 src/vstdadditions/TotalOrdered.rs
       0/       0/       0 tests/Chap03/TestInsertionSortStEph.rs

      77/     141/     412 total
     794 total lines
```

**Breakdown:**
- **Spec code**: 77 lines (specifications, invariants, ensures clauses)
- **Proof code**: 141 lines (lemmas, proof blocks, manual proofs)
- **Exec code**: 412 lines (executable implementations)
- **Total**: 630 lines of Verus code (794 including tests/benches/experiments)

## Algorithm Status

### Chapter 03: Sorting - ✅ COMPLETE

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Insertion Sort | ✅ | ✅ | ✅ | Generic over `T: TotalOrdered + Copy`, multiset preservation proven |

### Chapter 05: Sequences and Series - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Merge | ⬜ | ⬜ | ⬜ | |

### Chapter 06: Quicksort - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Quicksort | ⬜ | ⬜ | ⬜ | |

### Chapter 11: Binary Search - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Binary Search | ⬜ | ⬜ | ⬜ | |

### Chapter 12: Hash Tables - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Hash Tables | ⬜ | ⬜ | ⬜ | |

### Chapter 17: Shortest Paths - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Dijkstra's Algorithm | ⬜ | ⬜ | ⬜ | |

### Chapter 18: Minimum Spanning Trees - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Kruskal's Algorithm | ⬜ | ⬜ | ⬜ | |
| Prim's Algorithm | ⬜ | ⬜ | ⬜ | |

### Chapter 19: Sequences - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Array Sequences | ⬜ | ⬜ | ⬜ | |

### Chapter 21: Augmented Trees - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Augmented Trees | ⬜ | ⬜ | ⬜ | |

### Chapter 23: Range Trees - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Range Trees | ⬜ | ⬜ | ⬜ | |

### Chapter 26: Divide and Conquer - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Divide & Conquer Reduce | ⬜ | ⬜ | ⬜ | |

### Chapter 27: Parallel Scan and Reduce - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Scan Contract | ⬜ | ⬜ | ⬜ | |
| Reduce Contract | ⬜ | ⬜ | ⬜ | |

### Chapter 28: Work-Span Analysis - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Work-Span Models | ⬜ | ⬜ | ⬜ | |

### Chapter 35: Order Statistics - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Order Statistics Select | ⬜ | ⬜ | ⬜ | |

### Chapter 36: Integer Sorting - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Radix Sort | ⬜ | ⬜ | ⬜ | |
| Counting Sort | ⬜ | ⬜ | ⬜ | |

### Chapter 37: String Sorting - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| String Sorting | ⬜ | ⬜ | ⬜ | |

### Chapter 38: Parallel BST - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| BST Para | ⬜ | ⬜ | ⬜ | |

### Chapter 39: 2-3 Trees - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| 2-3 Trees | ⬜ | ⬜ | ⬜ | |

### Chapter 40: Red-Black Trees - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Red-Black Trees | ⬜ | ⬜ | ⬜ | |

### Chapter 41: AVL Trees - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| AVL Tree Sets | ⬜ | ⬜ | ⬜ | |

### Chapter 42: Hash Tables (Advanced) - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Advanced Hash Tables | ⬜ | ⬜ | ⬜ | |

### Chapter 43: Priority Queues - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Binary Heaps | ⬜ | ⬜ | ⬜ | |

### Chapter 44: Disjoint Sets - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Union-Find | ⬜ | ⬜ | ⬜ | |

### Chapter 45: Suffix Trees - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Suffix Trees | ⬜ | ⬜ | ⬜ | |

### Chapter 47: Dynamic Programming - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Dynamic Programming | ⬜ | ⬜ | ⬜ | |

### Chapter 49: Maxflow - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Maxflow Algorithms | ⬜ | ⬜ | ⬜ | |

### Chapter 50: Linear Programming - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Simplex Method | ⬜ | ⬜ | ⬜ | |

### Chapter 51: Fast Fourier Transform - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| FFT | ⬜ | ⬜ | ⬜ | |

### Chapter 52: Convex Hull - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Convex Hull | ⬜ | ⬜ | ⬜ | |

### Chapter 53: Computational Geometry - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Geometry Algorithms | ⬜ | ⬜ | ⬜ | |

### Chapter 54: Delaunay Triangulation - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Delaunay Triangulation | ⬜ | ⬜ | ⬜ | |

### Chapter 55: Graph Algorithms - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| DFS | ⬜ | ⬜ | ⬜ | |
| Cycle Detection | ⬜ | ⬜ | ⬜ | |
| Topological Sort | ⬜ | ⬜ | ⬜ | |
| Strongly Connected Components | ⬜ | ⬜ | ⬜ | |

### Chapter 56: Dynamic Graphs - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Dynamic Connectivity | ⬜ | ⬜ | ⬜ | |

### Chapter 57: Approximation Algorithms - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Approximation Algorithms | ⬜ | ⬜ | ⬜ | |

### Chapter 58: Randomized Algorithms - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Randomized Algorithms | ⬜ | ⬜ | ⬜ | |

### Chapter 59: Streaming Algorithms - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Streaming Algorithms | ⬜ | ⬜ | ⬜ | |

### Chapter 61: Machine Learning Basics - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| ML Basics | ⬜ | ⬜ | ⬜ | |

### Chapter 62: Neural Networks - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Neural Networks | ⬜ | ⬜ | ⬜ | |

### Chapter 63: Cryptographic Algorithms - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Crypto Algorithms | ⬜ | ⬜ | ⬜ | |

### Chapter 64: Compression - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Compression | ⬜ | ⬜ | ⬜ | |

### Chapter 65: Error Correction - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Error Correction | ⬜ | ⬜ | ⬜ | |

### Chapter 66: Quantum Algorithms - ⬜ NOT STARTED

| Algorithm | Verified | Tested | Benchmarked | Notes |
|-----------|----------|--------|-------------|-------|
| Quantum Algorithms | ⬜ | ⬜ | ⬜ | |

---

**Algorithms to prove before I sleep: 41 chapters, 60+ algorithms remaining**

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

## License

Copyright (C) 2025 Acar, Blelloch and Milnes

## References

- [Algorithms Parallel and Sequential](http://www.parallel-algorithms-book.com/)
- [Verus Documentation](https://verus-lang.github.io/verus/)
- [APAS-AI Project](https://github.com/your-repo/APAS-AI) - Original unverified Rust implementations

