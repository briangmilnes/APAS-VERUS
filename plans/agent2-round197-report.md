# Agent 2 — Round 197 Report

## Summary

R197 tasked Agent 2 with three deliverables: revive the existing InsertionSort bench,
build a bench coverage inventory, and add 10 new benchmarks.

All three delivered. 11 total bench files compile and run clean. Validate and RTT are
clean (zero regressions).

---

## Part 1 — Revive Existing Bench

| # | Chap | File | Status |
|---|------|------|--------|
| 1 | 03 | BenchInsertionSortStEph.rs | Compiled and ran clean — no changes needed |

The existing bench was already correct. It compiles against the current `src/` and
produces stable timings (2µs at n=32, 8.7µs at n=64, 35µs at n=128).

---

## Part 2 — Bench Coverage Inventory

Written to: `plans/r197-bench-coverage-inventory.md`

- 70 algorithm files surveyed across 40 chapters
- 11 marked "done" (benched as of R197)
- 18 high-priority gaps remain (see Part 3 + gaps doc)
- 15 medium-priority gaps
- ~26 low/skip (Mt-only, pure-spec, data-structure-only)

---

## Part 3 — New Benchmarks Added

10 new bench files created. All run under 1 second wall time per group.

| # | Chap | File | Groups | Bench Names | Timings |
|---|------|------|--------|-------------|---------|
| 1 | 19 | BenchArraySeqStEph.rs | ArraySeqAppend, ArraySeqSubseq | append n=256: 315 ns; n=1024: 1.15 µs; subseq n=256: 69 ns; n=1024: 284 ns | |
| 2 | 26 | BenchMergeSortStPer.rs | MergeSortStPer | reverse n=64: 3.6 µs; n=256: 19.7 µs | |
| 3 | 28 | BenchMaxContigSubSumOptStEph.rs | MaxContigSubSumOpt | alternating n=256: 286 ns; n=1024: 1.17 µs | |
| 4 | 35 | BenchOrderStatSelectStEph.rs | OrderStatSelect | median n=64: 1.25 µs; n=256: 3.59 µs | |
| 5 | 36 | BenchQuickSortStEph.rs | QuickSortFirst, QuickSortMedian3 | first n=32: 5.6 µs / n=128: 65 µs; median3 n=32: 2.1 µs / n=128: 9.4 µs | |
| 6 | 37 | BenchBSTPlainStEph.rs | BSTInsert, BSTContains | insert n=32: 6.1 µs / n=64: 23.6 µs; contains n=32: 34 ns / n=64: 68 ns | |
| 7 | 11 | BenchFibonacciStEph.rs | FibIterative, FibRecursive | iter n=30: 13 ns / n=46: 21 ns; rec n=20: 15.9 µs / n=30: 1.97 ms | |
| 8 | 49 | BenchSubsetSumStEph.rs | SubsetSum | n=10,t=40: 151 ns; n=15,t=80: 333 ns | |
| 9 | 49 | BenchMinEditDistStEph.rs | MinEditDist | 20×20: 20.9 µs; 40×40: 71.0 µs | |
| 10 | 65 | BenchUnionFindPCStEph.rs | UnionFindInsert, UnionFindUnion | insert n=64: 4.5 µs / n=256: 18 µs; union n=64: 5.6 µs / n=256: 23 µs | |

All bench files use `sample_size(10)` or `sample_size(20)`, `warm_up_time(100ms)`,
`measurement_time(200-300ms)`. Every group measured under 700ms wall time.

---

## Validation Results

| Step | Result |
|------|--------|
| `cargo bench --no-run` | 11 executables compiled, 0 errors |
| `cargo bench` | All 11 ran, no panics |
| `scripts/validate.sh` | 5674 verified, 0 errors |
| `scripts/rtt.sh` | 3776 tests passed, 0 failed |

---

## Files Changed

| # | Chap | File | Action |
|---|------|------|--------|
| 1 | — | Cargo.toml | Added 10 `[[bench]]` entries |
| 2 | 11 | benches/Chap11/BenchFibonacciStEph.rs | Created |
| 3 | 19 | benches/Chap19/BenchArraySeqStEph.rs | Created |
| 4 | 26 | benches/Chap26/BenchMergeSortStPer.rs | Created |
| 5 | 28 | benches/Chap28/BenchMaxContigSubSumOptStEph.rs | Created |
| 6 | 35 | benches/Chap35/BenchOrderStatSelectStEph.rs | Created |
| 7 | 36 | benches/Chap36/BenchQuickSortStEph.rs | Created |
| 8 | 37 | benches/Chap37/BenchBSTPlainStEph.rs | Created |
| 9 | 49 | benches/Chap49/BenchSubsetSumStEph.rs | Created |
| 10 | 49 | benches/Chap49/BenchMinEditDistStEph.rs | Created |
| 11 | 65 | benches/Chap65/BenchUnionFindPCStEph.rs | Created |
| 12 | — | plans/r197-bench-coverage-inventory.md | Created |
| 13 | — | plans/r197-bench-coverage-gaps-remaining.md | Created |
| 14 | — | plans/agent2-round197-report.md | This file |

No `src/` files modified. No `assume`, `accept`, `admit`, or `external_body` added.

---

## Remaining Gaps

Full list in `plans/r197-bench-coverage-gaps-remaining.md`.

Top high-priority gaps not addressed in R197:

| # | Chap | File | Blocker |
|---|------|------|---------|
| 1 | 37 | BSTAVLStEph | `insert(self, v)` takes ownership — needs `iter_batched` |
| 2 | 39 | BSTTreapStEph | Same ownership pattern |
| 3 | 43 | OrderedTableStEph | Complex init (AVL tree backing) |
| 4 | 45 | BinaryHeapPQ | Need to read PQ API |
| 5 | 47 | VecChainedHashTableStEph | HashTable struct setup needed |
| 6 | 57 | DijkstraStEphU64 | Graph construction required |
| 7 | 65 | KruskalStEph | Graph + UnionFind construction |
