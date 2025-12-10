# APAS-VERUS

Formally verified implementations of algorithms from "Algorithms Parallel and Sequential" by Acar, Blelloch, and Milnes, using the [Verus](https://github.com/verus-lang/verus) verification framework for Rust.

**Verified: 5 chapters complete (Scheduling, Sorting, Sets/Relations/Mappings, Graphs, Fibonacci)**

## Project Structure

- `src/` - Verified algorithm implementations organized by chapter
- `src/vstdadditions/` - Extensions to the Verus standard library
- `tests/` - Rust unit tests for algorithm correctness
- `benches/` - Performance benchmarks using Criterion
- `attic/` - Old/deprecated implementations

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
| Pool | ✅ | ✅ | ✅ | ⬜ | Bounded thread pool, `pool.join(fa, fb)` |
| FibonacciWSScheduler | ✅ | ✅ | ✅ | ⬜ | `fib_pool` using Pool.join |

### Chapter 03: Insertion Sort - ✅ COMPLETE

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| InsertionSortStEph | ✅ | — | ✅ | ✅ | Generic, multiset preservation proven |

### Chapter 05: Sets, Relations, Mappings - ✅ COMPLETE

| Data Structure | Verified | Parallel | Tested | Benchmarked | Notes |
|----------------|----------|----------|--------|-------------|-------|
| SetStEph | ✅ | — | ✅ | ⬜ | Ephemeral set with iterator proofs |
| SetMtEph | ✅ | ✅ | ✅ | ⬜ | Multi-threaded parallel set |
| RelationStEph | ✅ | — | ✅ | ⬜ | Binary relations |
| MappingStEph | ✅ | — | ✅ | ⬜ | Key-value mappings |

### Chapter 06: Graphs - ✅ COMPLETE

| Data Structure | Verified | Parallel | Tested | Benchmarked | Notes |
|----------------|----------|----------|--------|-------------|-------|
| DirGraphStEph | ✅ | — | ✅ | ⬜ | Directed graph |
| DirGraphMtEph | ✅ | ✅ | ✅ | ⬜ | Parallel directed graph |
| UnDirGraphStEph | ✅ | — | ✅ | ⬜ | Undirected graph |
| UnDirGraphMtEph | ✅ | ✅ | ✅ | ⬜ | Parallel undirected graph |
| LabDirGraphStEph | ✅ | — | ✅ | ⬜ | Labeled directed graph |
| LabDirGraphMtEph | ✅ | ✅ | ✅ | ⬜ | Parallel labeled directed |
| LabUnDirGraphStEph | ✅ | — | ✅ | ⬜ | Labeled undirected graph |
| LabUnDirGraphMtEph | ✅ | ✅ | ✅ | ⬜ | Parallel labeled undirected |
| WeightedDirGraphStEph | ✅ | — | ✅ | ⬜ | All 12 integer types |
| WeightedDirGraphMtEph | ✅ | ✅ | ✅ | ⬜ | Parallel weighted directed |

### Chapter 11: Fibonacci - ✅ COMPLETE

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| FibonacciStEph | ✅ | — | ✅ | ⬜ | Sequential with overflow proofs |
| FibonacciMtEph2Threads | ✅ | ✅ | ✅ | ⬜ | 2-thread parallel |
| FibonacciMtPerAllThreads | ✅ | ✅ | ✅ | ⬜ | ParaPairDisjoint! macro |
| FibonacciMtEphRecomputes | ✅ | ✅ | ✅ | ⬜ | Recomputation variant |
| FibonacciMtPerTSM | ✅ | ✅ | ✅ | ⬜ | Tokenized state machine |

### Chapter 12: Concurrency Primitives - 🔄 EXTERNAL_BODY

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| SpinLock | 🔄 | ✅ | ✅ | ⬜ | Ticket lock via fetch-and-add |
| fetch_add_cas | 🔄 | ✅ | ✅ | ⬜ | CAS-based fetch-and-add |
| ConcurrentStackMt | 🔄 | ✅ | ✅ | ⬜ | Lock-free Treiber stack |

18 holes total - atomics and raw pointers not supported by Verus.

### Chapter 17: MathSeq - ✅ COMPLETE

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| MathSeqS | ✅ | — | ✅ | ⬜ | Vec-backed dense sequence, 6 external_body |

### Chapter 18: Sequences - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| ArraySeq | ⬜ | ⬜ | ⬜ | ⬜ | |
| ArraySeqStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| ArraySeqMtEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| LinkedListStEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 19: Sequences (Advanced) - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| ArraySeqStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| ArraySeqMtEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 21: Trees and Algorithms - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| Algorithm21_1 | ⬜ | ⬜ | ⬜ | ⬜ | |
| Algorithm21_2 | ⬜ | ⬜ | ⬜ | ⬜ | |
| Algorithm21_5 | ⬜ | ⬜ | ⬜ | ⬜ | |
| Algorithm21_6 | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 23: Trees - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| BalBinTreeStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| PrimTreeSeqStPer | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 26: Divide and Conquer - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| DivConReduceStPer | ⬜ | ⬜ | ⬜ | ⬜ | |
| DivConReduceMtPer | ⬜ | ⬜ | ⬜ | ⬜ | |
| MergeSortStPer | ⬜ | ⬜ | ⬜ | ⬜ | |
| MergeSortMtPer | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 27: Scan and Reduce - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| ScanContractStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| ScanContractMtEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| ReduceContractStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| ReduceContractMtEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 28: Max Contiguous Subsum - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| MaxContigSubSumBruteStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| MaxContigSubSumDivConStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| MaxContigSubSumDivConMtEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 35: Order Statistics - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| OrderStatSelectStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| OrderStatSelectMtEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 36: QuickSort - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| QuickSortStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| QuickSortMtEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 37: BST Variants - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| BSTPlainStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| BSTAVLStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| BSTRBStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| BSTSplayStEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 38: Parallel BST - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| BSTParaStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| BSTParaMtEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 39: Treaps - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| BSTTreapStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| BSTTreapMtEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 40: BST Key-Value - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| BSTKeyValueStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| BSTReducedStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| BSTSizeStEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 41: Sets - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Tested | Benchmarked | Notes |
|-----------|----------|----------|--------|-------------|-------|
| ArraySetStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| AVLTreeSetStEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapters 42-66: Future Work - ⬜ NOT STARTED

See APAS-AI for unverified implementations of:
- Chapter 42-45: Hash Tables, Priority Queues, Union-Find, Suffix Trees
- Chapter 47-59: Dynamic Programming, Maxflow, Linear Programming, FFT, Geometry
- Chapter 61-66: ML, Neural Networks, Crypto, Compression, Error Correction, Quantum

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

