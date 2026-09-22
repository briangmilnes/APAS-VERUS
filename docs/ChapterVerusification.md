<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter Verification State

Updated: 2026-09-22 (r213 rows marked "r213:"; running log `docs/ChaptersInOrder.md`). Toolchain: Verus `0.2026.09.13.671956e`, Rust 1.98.1, Z3 4.16.0.
Last state with 0 errors crate-wide: Verus `0.2026.04.20`, 5,763 verified, 0 holes.

Measured from `logs/validate.20260921-102521.log` (full run, rustc type checking
only: 162 errors in 51 files, 824 deprecated-`finite()` warnings, Z3 not reached)
and the `isolate` runs of rounds r208 and r209. A chapter reaches Z3 only when it
and its transitive dependencies have 0 type errors. "Own errors" counts errors
whose `-->` names a file in that chapter.

## Totals

| # | State | Chapters | Count |
|---|-------|----------|------:|
| 1 | verifies, 0 errors, 0 warnings | 02, 03, 05, 06, 17, 50, 66 | 7 |
| 2 | 0 own errors, dependencies clean, not yet run | 11, 12, 30 | 3 |
| 3 | 0 own errors, waiting on a dependency | 21, 26, 27, 28, 35, 36, 42, 45, 47, 49, 51, 54, 55, 56, 61, 62, 63, 64 | 18 |
| 4 | own type errors | 18, 19, 23, 37, 38, 39, 40, 41, 43, 44, 52, 53, 57, 58, 59, 65 | 16 |

Chap18 and Chap19 hold 34 of the 162 errors in 12 files, all the old iterator
model, and between them block 22 chapters. They are the next migration, per
`docs/IteratorMigrationStudy.md` §7. The 824 warnings are deprecated `finite()`
calls, removed chapter by chapter as each is migrated.

## Foundation

| # | Module | State | Evidence |
|---|--------|-------|----------|
| 1 | `Types.rs`, `Concurrency.rs`, `ParaPairs.rs` | verify; included in every isolate run | `validate.20260921-093012.log` |
| 2 | `vstdplus/` | verifies; `hash_set_with_view_plus`, `hash_map_with_view_plus`, `hash_set_specs` deleted (r209); `arc_rwlock`, `VecQueue`, `partial_order`, `seq_set_pre_0913` commented out; `hash_specs_plus.rs` new | `docs/VstdplusReview.md`, `docs/HashMigration.md` |
| 3 | `standards/` | all 21 registered standards verify, 0 errors | `docs/StandardsUpgrade.md` |
| 4 | `experiments/` | all commented out in `lib.rs` with `RESULT` status; 17 added in r207–r209 | `src/lib.rs` |
| 5 | RTT (`scripts/rtt.sh`) | not run since the upgrade; entries for Chap62, 63, 66 and `test_partial_order` commented out in `Cargo.toml` | — |
| 6 | PTT (`scripts/ptt.sh`) | cannot run: 162 crate errors and `cargo-nextest` missing on the nightly toolchain; 84 of 97 PTT files on the old iterator model | `logs/ptt.20260920-155305.log` |

## Chapters

| # | Chap | Deps | Own errors | State | Evidence |
|---|------|------|-----------:|-------|----------|
| 1 | 02 | — | 0 | r213: 631 verified, 0 err, 0 warn; RTT 41 pass | `validate.20260922-050445.log` |
| 2 | 03 | — | 0 | r213: 622 verified, 0 err, 0 warn; RTT 40 pass | `validate.20260922-050550.log` |
| 3 | 05 | 02 | 0 | verifies: 760, 0 errors | `validate.20260921-093012.log` |
| 4 | 06 | 05 | 0 | verifies: 1024, 0 own errors; rerun pending since Chap05's 2 errors were fixed after | `validate.20260920-194725.log` |
| 5 | 11 | 02 | 0 | not run; deps have 0 errors | — |
| 6 | 12 | 02 | 0 | not run; deps have 0 errors | — |
| 7 | 17 | — | 0 | verifies: 635, 0 errors | `validate.20260921-083557.log` |
| 8 | 18 | 02 | 22 | old iterator model in 8 files | full log |
| 9 | 19 | 02 | 12 | old iterator model in 4 files | full log |
| 10 | 21 | 18, 19 | 0 | blocked on 18, 19 | full log |
| 11 | 23 | — | 6 | old iterator model in 2 files | full log |
| 12 | 26 | 02, 18 | 0 | blocked on 18 | full log |
| 13 | 27 | 02, 19 | 0 | blocked on 19 | full log |
| 14 | 28 | 19 | 0 | blocked on 19 | full log |
| 15 | 30 | — | 0 | not run; no deps | — |
| 16 | 35 | 02, 18, 19 | 0 | blocked on 18, 19 | full log |
| 17 | 36 | 19 | 0 | blocked on 19 | full log |
| 18 | 37 | 02, 18, 19, 23 | 5 | old iterator model in 5 files; also blocked on 18, 19, 23 | full log |
| 19 | 38 | 18 | 1 | old iterator model; blocked on 18 | full log |
| 20 | 39 | 18 | 4 | old iterator model in 4 files; blocked on 18 | full log |
| 21 | 40 | 18 | 3 | old iterator model in 3 files; blocked on 18 | full log |
| 22 | 41 | 18, 19, 37, 38 | 15 | `Set::new` (9, `ArraySetEnumMtEph.rs`), `Map::new`, old iterators | full log |
| 23 | 42 | 02, 19, 41 | 0 | blocked on 19, 41 | full log |
| 24 | 43 | 18, 19, 37, 38, 41, 42 | 6 | `Map::new`, `lemma_map_finite`, old iterators | full log |
| 25 | 44 | 19, 37, 41, 42 | 2 | `lemma_set_insert_len` shape | full log |
| 26 | 45 | 19, 37 | 0 | blocked on 19, 37 | full log |
| 27 | 47 | 18 | 0 | blocked on 18 | full log |
| 28 | 49 | 02, 18, 19 | 0 | type-checked in r209; blocked on 18, 19 | `validate.20260921-084828.log` |
| 29 | 50 | 02, 30 | 0 | verifies: 756, 0 errors | `validate.20260921-085534.log` |
| 30 | 51 | 02, 18, 19 | 0 | type-checked in r209; blocked on 18, 19 | `validate.20260921-085917.log` |
| 31 | 52 | 18, 19, 37, 38, 41, 43 | 7 | `Set::new` in 4 `EdgeSetGraph*` files | full log |
| 32 | 53 | 37, 38, 41 | 1 | `lemma_map_finite` in `PQMinStPer.rs` | full log |
| 33 | 54 | 02, 18, 19 | 0 | blocked on 18, 19 | full log |
| 34 | 55 | 19, 37, 41 | 0 | blocked on 19, 37, 41 | full log |
| 35 | 56 | 19 | 0 | blocked on 19 | full log |
| 36 | 57 | 05, 06, 45, 56 | 24 | old-model loops in 2 Dijkstra files | full log |
| 37 | 58 | 05, 06, 56 | 16 | old-model loops in 2 BellmanFord files | full log |
| 38 | 59 | 05, 06, 19, 56, 57, 58 | 32 | old-model loops in 4 Johnson files | full log |
| 39 | 61 | 05, 06, 19 | 0 | type-checked in r209; blocked on 19 | `validate.20260921-091115.log` |
| 40 | 62 | 05, 06, 19 | 0 | type-checked in r209; blocked on 19 | `validate.20260921-091932.log` |
| 41 | 63 | 05, 06, 62 | 0 | type-checked in r209; blocked on 19 | `validate.20260921-092508.log` |
| 42 | 64 | 05, 06, 62 | 0 | type-checked in r209; blocked on 19 | `validate.20260921-092658.log` |
| 43 | 65 | 05, 06, 45 | 6 | `KruskalStEph.rs` 4, `PrimStEph.rs` 2; blocked on 45 | full log |
| 44 | 66 | 05 | 0 | verifies: 795, 0 errors | `validate.20260921-084327.log` |

Chapters 07–10, 13–16, 20, 22, 24, 25, 29, 31–34, 46, 48, 60 have no directory.

## How to refresh

1. `scripts/validate.sh`; read the newest `logs/validate.*.log`; attribute each
   error's `-->` line to its chapter for the "Own errors" column.
2. `scripts/validate.sh isolate ChapNN` for each chapter whose transitive
   dependencies have 0 own errors; record `N verified, M errors` and the log.
3. Once the crate has 0 type errors, `scripts/all-holes-by-chap.sh` and
   `scripts/chapter-cleanliness-status.sh` give the hole columns (they need
   veracity, which is not on this machine).
