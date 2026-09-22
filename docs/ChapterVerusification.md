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
| 3 | 05 | 02 | 0 | r213: 760 verified, 0 err, 0 warn; RTT 89 pass; PTT 5/21 | `validate.20260922-050624.log` |
| 4 | 06 | 05 | 0 | r213: 1037 verified, 0 err, 0 warn; RTT 275 pass | `validate.20260922-050731.log` |
| 5 | 11 | 02 | 0 | r213: 651 verified, 0 err, 0 warn; RTT 40 pass | `validate.20260922-050801.log` |
| 6 | 12 | 02 | 0 | r213: 635 verified, 0 err, 0 warn; RTT 40 pass | `validate.20260922-050809.log` |
| 7 | 17 | — | 0 | r213: 645 verified, 0 err, 0 warn; RTT 40 pass; PTT 2/9 | `validate.20260922-050830.log` |
| 8 | 18 | 02 | 0 | r213: 1003 verified, 0 err, 0 warn; RTT 170, PTT 38 pass | `validate.20260922-050912.log` |
| 9 | 19 | 02 | 0 | r213: 824 verified, 0 err, 0 warn; RTT 156, PTT 23 pass | `validate.20260922-051000.log` |
| 10 | 21 | 18, 19 | 0 | r213: 1262 verified, 0 err, 0 warn; RTT 46 pass | `validate.20260922-051054.log` |
| 11 | 23 | — | 0 | r213: 679 verified, 0 err, 0 warn; RTT 92, PTT 17 pass | `validate.20260922-051126.log` |
| 12 | 26 | 02, 18 | 0 | r213: 1098 verified, 0 err, 0 warn; RTT 59 pass | `validate.20260922-051314.log` |
| 13 | 27 | 02, 19 | 0 | r213: 846 verified, 0 err, 0 warn; RTT 48 pass | `validate.20260922-051353.log` |
| 14 | 28 | 19 | 0 | r213: 883 verified, 0 err, 0 warn; RTT 68 pass | `validate.20260922-051419.log` |
| 15 | 30 | — | 0 | r213: 626 verified, 0 err, 0 warn; no RTT | `validate.20260922-051446.log` |
| 16 | 35 | 02, 18, 19 | 0 | r213: 1224 verified, 0 err, 0 warn; RTT 58 pass | `validate.20260922-051905.log` |
| 17 | 36 | 19 | 0 | r213: 867 verified, 0 err, 0 warn; RTT 24 pass | `validate.20260922-051831.log` |
| 18 | 37 | 02, 18, 19, 23 | 0 | r213: 1862 verified, 0 err, 0 warn; RTT 544 pass | `validate.20260922-051953.log` |
| 19 | 38 | 18 | 0 | r213: 1078 verified, 0 err, 0 warn; RTT 53, PTT 2 pass | `validate.20260922-052403.log` |
| 20 | 39 | 18 | 0 | r213: 1218 verified, 0 err, 0 warn; RTT 148, PTT 8 pass | `validate.20260922-053007.log` |
| 21 | 40 | 18 | 0 | r213: 1180 verified, 0 err, 0 warn; RTT 54 pass | `validate.20260922-053048.log` |
| 22 | 41 | 18, 19, 37, 38 | 0 | r213: 2188 verified, 0 err, 0 warn; RTT 250 pass | `validate.20260922-054937.log` |
| 23 | 42 | 02, 19, 41 | 0 | r213: 2312 verified, 0 err, 0 warn; RTT 66 pass | `validate.20260922-055734.log` |
| 24 | 43 | 18, 19, 37, 38, 41, 42 | 0 | r213: 2686 verified, 0 err, 0 warn; RTT 279 pass | `validate.20260922-060943.log` |
| 25 | 44 | 19, 37, 41, 42 | 0 | r213: 2336 verified, 0 err, 0 warn; RTT 45 pass | `validate.20260922-061449.log` |
| 26 | 45 | 19, 37 | 0 | r213: 2034 verified, 0 err, 0 warn; RTT 210 pass | `validate.20260922-061545.log` |
| 27 | 47 | 18 | 0 | r213: 1161 verified, 0 err, 0 warn; RTT 102 pass | `validate.20260922-061642.log` |
| 28 | 49 | 02, 18, 19 | 0 | r213: 1283 verified, 0 err, 0 warn; RTT 136 pass | `validate.20260922-061718.log` |
| 29 | 50 | 02, 30 | 0 | r213: 766 verified, 0 err, 0 warn; RTT 167 pass | `validate.20260922-061737.log` |
| 30 | 51 | 02, 18, 19 | 0 | r213: 1333 verified, 0 err, 0 warn; RTT 109 pass | `validate.20260922-061753.log` |
| 31 | 52 | 18, 19, 37, 38, 41, 43 | 0 | r213: 2943 verified, 0 err, 0 warn; RTT 148 pass | `validate.20260922-061938.log` |
| 32 | 53 | 37, 38, 41 | 0 | r213: 2246 verified, 0 err, 0 warn; RTT 46 pass | `validate.20260922-062132.log` |
| 33 | 54 | 02, 18, 19 | 0 | r213: 1277 verified, 0 err, 0 warn; RTT 53 pass | `validate.20260922-062218.log` |
| 34 | 55 | 19, 37, 41 | 0 | r213: 2290 verified, 0 err, 0 warn; RTT 58 pass | `validate.20260922-062417.log` |
| 35 | 56 | 19 | 0 | r213: 948 verified, 0 err, 0 warn; RTT 54 pass | `validate.20260922-062449.log` |
| 36 | 57 | 05, 06, 45, 56 | 0 | r213: 2583 verified, 0 err, 0 warn; RTT 48 pass | `validate.20260922-062519.log` |
| 37 | 58 | 05, 06, 56 | 0 | r213: 1370 verified, 0 err, 0 warn; RTT 41 pass | `validate.20260922-062822.log` |
| 38 | 59 | 05, 06, 19, 56, 57, 58 | 0 | r213: 2632 verified, 0 err, 0 warn; RTT 41 pass | `validate.20260922-062907.log` |
| 39 | 61 | 05, 06, 19 | 0 | r213: 1243 verified, 0 err, 0 warn; RTT 40 pass | `validate.20260922-062947.log` |
| 40 | 62 | 05, 06, 19 | 0 | r213: 1256 verified, 0 err, 0 warn; RTT 39 pass | `validate.20260922-063225.log` |
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
