# APAS-VERUS

Formally verified implementations of algorithms from "Algorithms Parallel and Sequential" by Acar, Blelloch.
Rust code and proofs in the [Verus](https://github.com/verus-lang/verus) verification framework by Brian Milnes.

Claude used for the code generation and proving in Verus. Although the AIs are improving,
even with 50+ Cursor rules (in .cursor), they made so many bad judgements that I had to
develop two software engineering tools to clean things up:

- [veracity](https://github.com/briangmilnes/veracity) - Verus code analysis tools (proof hole detection, spec strength review, function search).
- [rusticate](https://github.com/briangmilnes/rusticate) - Rust code style and structure review tools.

**Verified: 8 chapters complete (Scheduling, Sorting, Sets/Relations/Mappings, Graphs, Fibonacci, MathSeq, Sequences, Trees)**

**Proof totals: 1298 verified, 0 errors**

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

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
| Pool | ✅ | ✅ | ✅ | ✅ | Bounded thread pool, `pool.join(fa, fb)` |
| FibonacciWSScheduler | ✅ | ✅ | ✅ | ✅ | `fib_pool` using Pool.join |

### Chapter 03: Insertion Sort - ✅ COMPLETE

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
| InsertionSortStEph | ✅ | — | ✅ | ⬜ | Generic, multiset preservation proven |

### Chapter 05: Sets, Relations, Mappings - ✅ COMPLETE

| Data Structure | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|----------------|----------|----------|---------------|-----------------|-------|
| SetStEph | ✅ | — | ✅ | ✅ | Ephemeral set, custom iterator + ghost |
| SetMtEph | ✅ | ✅ | ✅ | ✅ | Multi-threaded, ZERO HOLES (cartesian_product proven) |
| RelationStEph | ✅ | — | ✅ | ✅ | Binary relations, ZERO HOLES |
| MappingStEph | ✅ | — | ✅ | ✅ | Key-value mappings, ZERO HOLES |

### Chapter 06: Graphs - ✅ COMPLETE (ZERO HOLES)

| Data Structure | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|----------------|----------|----------|---------------|-----------------|-------|
| DirGraphStEph | ✅ | — | ✅ | ✅ | Directed graph |
| DirGraphMtEph | ✅ | ✅ | ✅ | ✅ | Parallel directed graph, proven set split |
| UnDirGraphStEph | ✅ | — | ✅ | ✅ | Undirected graph |
| UnDirGraphMtEph | ✅ | ✅ | ✅ | ✅ | Parallel undirected graph, proven set split |
| LabDirGraphStEph | ✅ | — | ✅ | ✅ | Labeled directed graph |
| LabDirGraphMtEph | ✅ | ✅ | ✅ | ✅ | Parallel labeled directed, proven set split |
| LabUnDirGraphStEph | ✅ | — | ✅ | ✅ | Labeled undirected graph |
| LabUnDirGraphMtEph | ✅ | ✅ | ✅ | ✅ | Parallel labeled undirected, proven set split |
| WeightedDirGraphStEph | ✅ | — | ✅ | ✅ | All 12 integer types |

### Chapter 11: Fibonacci - ✅ COMPLETE

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
| FibonacciStEph | ✅ | — | ✅ | ⬜ | Sequential with overflow proofs |
| FibonacciMtEph2Threads | ✅ | ✅ | ✅ | ⬜ | 2-thread parallel |
| FibonacciMtPerAllThreads | ✅ | ✅ | ✅ | ⬜ | ParaPair! macro |
| FibonacciMtEphRecomputes | ✅ | ✅ | ✅ | ⬜ | Recomputation variant |
| FibonacciMtPerTSM | ✅ | ✅ | ✅ | ⬜ | Tokenized state machine |

### Chapter 12: Concurrency Primitives - 🔄 EXTERNAL_BODY

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
| SpinLock | 🔄 | ✅ | ✅ | ⬜ | Ticket lock via fetch-and-add |
| fetch_add_cas | 🔄 | ✅ | ✅ | ⬜ | CAS-based fetch-and-add |
| ConcurrentStackMt | 🔄 | ✅ | ✅ | ⬜ | Lock-free Treiber stack |

18 holes total - atomics and raw pointers not supported by Verus.

### Chapter 17: MathSeq - ✅ COMPLETE (ZERO HOLES)

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
| MathSeqS | ✅ | — | ✅ | ✅ | Vec-backed dense sequence, uses `HashMapWithView` |

### Chapter 18: Sequences - ✅ COMPLETE

| Data Structure | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|----------------|----------|----------|---------------|-----------------|-------|
| ArraySeq | ✅ | — | ✅ | ✅ | Custom iterator + ForLoopGhostIterator |
| ArraySeqStEph | ✅ | — | ✅ | ✅ | Custom iterator + ForLoopGhostIterator |
| ArraySeqStPer | ✅ | — | ✅ | ✅ | Custom iterator + ForLoopGhostIterator |
| ArraySeqMtEph | ✅ | ✅ | ✅ | ✅ | Parallel ops, custom iterator + ghost |
| ArraySeqMtPer | ✅ | ✅ | ✅ | ✅ | Parallel ops, custom iterator + ghost |
| LinkedListStEph | ✅ | — | ✅ | ✅ | Custom iterator + ForLoopGhostIterator |
| LinkedListStPer | ✅ | — | ✅ | ✅ | Custom iterator + ForLoopGhostIterator |

### Chapter 19: Sequences (Advanced) - 🔄 IN PROGRESS

| Data Structure | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|----------------|----------|----------|---------------|-----------------|-------|
| ArraySeqStPer | ✅ | — | ⬜ | ✅ | Compositional algorithms (tabulate, flatten, deflate) |
| ArraySeqStEph | ✅ | — | ⬜ | ✅ | Ephemeral: clone+set update |
| ArraySeqMtEph | ✅ | ✅ | ⬜ | ✅ | Parallel map/filter/reduce via fork-join |

### Chapter 21: Trees and Algorithms - ✅ COMPLETE (ZERO HOLES)

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
| Algorithm21_1 | ✅ | — | ⬜ | ⬜ | |
| Algorithm21_2 | ✅ | — | ⬜ | ⬜ | |
| Algorithm21_5 | ✅ | — | ⬜ | ⬜ | |
| Algorithm21_6 | ✅ | — | ⬜ | ⬜ | |
| Exercise21_5 | ✅ | — | ⬜ | ⬜ | |
| Exercise21_6 | ✅ | — | ⬜ | ⬜ | |
| Exercise21_7 | ✅ | — | ⬜ | ⬜ | |
| Exercise21_8 | ✅ | — | ⬜ | ⬜ | |
| Exercise21_9 | ✅ | — | ⬜ | ⬜ | |
| Problem21_1 | ✅ | — | ⬜ | ⬜ | |
| Problem21_3 | ✅ | — | ⬜ | ⬜ | |
| Problem21_4 | ✅ | — | ⬜ | ⬜ | |

### Chapter 23: Trees - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
| BalBinTreeStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| PrimTreeSeqStPer | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 26: Divide and Conquer - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
| DivConReduceStPer | ⬜ | ⬜ | ⬜ | ⬜ | |
| DivConReduceMtPer | ⬜ | ⬜ | ⬜ | ⬜ | |
| MergeSortStPer | ⬜ | ⬜ | ⬜ | ⬜ | |
| MergeSortMtPer | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 27: Scan and Reduce - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
| ScanContractStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| ScanContractMtEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| ReduceContractStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| ReduceContractMtEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 28: Max Contiguous Subsum - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
| MaxContigSubSumBruteStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| MaxContigSubSumDivConStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| MaxContigSubSumDivConMtEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 35: Order Statistics - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
| OrderStatSelectStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| OrderStatSelectMtEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 36: QuickSort - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
| QuickSortStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| QuickSortMtEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 37: BST Variants - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
| BSTPlainStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| BSTAVLStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| BSTRBStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| BSTSplayStEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 38: Parallel BST - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
| BSTParaStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| BSTParaMtEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 39: Treaps - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
| BSTTreapStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| BSTTreapMtEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 40: BST Key-Value - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
| BSTKeyValueStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| BSTReducedStEph | ⬜ | ⬜ | ⬜ | ⬜ | |
| BSTSizeStEph | ⬜ | ⬜ | ⬜ | ⬜ | |

### Chapter 41: Sets - ⬜ NOT STARTED

| Algorithm | Verified | Parallel | Run Time Test | Proof Time Test | Notes |
|-----------|----------|----------|---------------|-----------------|-------|
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

## Proof State

Full verification: **1298 verified, 0 errors**

| Metric | Count |
|--------|-------|
| Clean modules (no holes) | 100 |
| Holed modules | 61 |
| Clean proof functions | 321 |
| Holed proof functions | 61 |
| Total proof functions | 382 |

### Proof Holes: 197 total

| # | Hole Type | Count | Notes |
|---|-----------|-------|-------|
| 1 | `external_body` | 75 | Unverified function bodies (FFI, atomics, threading) |
| 2 | `admit()` | 46 | Admitted without proof |
| 3 | `assume()` | 52 | Assumed conditions (includes PartialEq bridge) |
| 4 | `assume(false)` | 8 | Unreachable error paths in thread joins |
| 5 | `external` | 3 | Fully external functions |
| 6 | `unsafe {}` | 4 | Unsafe blocks (Chap12 raw pointers) |
| 7 | `unsafe impl` | 2 | Manual Send/Sync (vstdplus/threads_plus) |
| 8 | Other | 7 | external_type/trait_spec, assume_specification, Tracked::assume_new |

Most holes are in infrastructure (vstdplus, Chap02 scheduler, Chap12 concurrency primitives) and PartialEq bridges. Algorithm chapters 03, 06, and 21 have **zero holes**.

## Documentation

API documentation with Verus specifications (requires/ensures):

- [Browse docs/verusdoc/apas_verus/](docs/verusdoc/apas_verus/index.html) - Generated with `scripts/verusdoc.sh`

To regenerate:
```bash
./scripts/verusdoc.sh
```

## Building and Testing

All scripts live in `scripts/`, auto-detect the worktree root, and strip ANSI escape codes for Emacs `M-x compile`.

### Scripts

| # | Script | Usage | Purpose |
|---|--------|-------|---------|
| 1 | `scripts/validate.sh` | `validate.sh [full\|dev\|exp] [--time]` | Verus verification |
| 2 | `scripts/check.sh` | `check.sh` | `cargo check --lib` |
| 3 | `scripts/rtt.sh` | `rtt.sh [filter]` | Runtime tests (`-j 6`, 120s timeout) |
| 4 | `scripts/ptt.sh` | `ptt.sh [filter]` | Compile PTT lib + proof time tests (`-j 6`) |
| 5 | `scripts/holes.sh` | `holes.sh [dir-or-file]` | Proof hole detection |
| 6 | `scripts/validate-check-rtt-ptt.sh` | `validate-check-rtt-ptt.sh` | Full pipeline (stops on first failure) |
| 7 | `scripts/merge-agent.sh` | `merge-agent.sh <branch>` | Merge an agent branch + validate |
| 8 | `scripts/reset-agent-to-main.sh` | `reset-agent-to-main.sh` | Reset agent branch to `origin/main` + force push |

### Verification

```bash
scripts/validate.sh dev            # dev mode (skip cfg-gated modules)
scripts/validate.sh full --time    # full verification with timing breakdown
scripts/validate.sh exp            # experiments only
```

### Compilation Check

```bash
scripts/check.sh                   # cargo check --lib
```

### Runtime Tests (RTTs)

```bash
scripts/rtt.sh                     # all tests
scripts/rtt.sh bst                 # case-insensitive filter on test names
```

### Proof Time Tests (PTTs)

```bash
scripts/ptt.sh                     # compile lib + all PTTs
scripts/ptt.sh Chap05              # compile lib + filtered PTTs
```

### Proof Holes

```bash
scripts/holes.sh                   # all of src/
scripts/holes.sh src/Chap05/       # one chapter
scripts/holes.sh src/Chap05/SetStEph.rs  # one file
```

### Full Pipeline

```bash
scripts/validate-check-rtt-ptt.sh  # validate (dev) → check → RTT → PTT
```

### Benchmarking

```bash
cargo bench                        # all benchmarks
cargo bench --bench BenchInsertionSortStEph  # specific benchmark
```

## Further Documentation

- [docs/Scripts.md](docs/Scripts.md) — detailed reference for every script in `scripts/`
- [docs/WorkingWithMultipleAgentsInWorktrees.md](docs/WorkingWithMultipleAgentsInWorktrees.md) — merge procedure, conflict resolution, and agent reset workflow

## Development Setup

1. Install [Verus](https://github.com/verus-lang/verus) (see `~/projects/verus/BUILD.md`)
2. Install Rust toolchain (pinned in `rust-toolchain.toml`)
3. Clone this repository
4. Run `scripts/validate.sh dev` to verify
5. Run `scripts/rtt.sh` to run tests

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

