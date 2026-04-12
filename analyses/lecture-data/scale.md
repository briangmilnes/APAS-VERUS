# Lecture Data — Section 1: Scale

Generated: 2026-04-11 from main at `7ec8eb2bf`.

## Project size

| # | Metric | Value |
|---|--------|-------|
| 1 | First commit date | 2025-11-03 |
| 2 | Latest commit date | 2026-04-11 |
| 3 | Project duration | 160 days (~5.3 months) |
| 4 | Total git commits | 2,596 |
| 5 | Round-tagged commits (R1–R196) | 736 |
| 6 | Latest round | R196 |
| 7 | Active agents to date | 8 (agent1–agent8); 6 currently active |
| 8 | Cumulative agent-rounds (reports) | 281 |
| 9 | Top agent (agent1) round count | 74 reports, latest R196 |

## File and module counts

| # | Metric | Value |
|---|--------|-------|
| 10 | Chapter modules in `lib.rs` | 44 |
| 11 | `src/Chap*/` directories | 44 |
| 12 | Algorithm files (excl. Example/Problem) | 262 |
| 13 | `Example*.rs` files (out of scope) | 7 |
| 14 | `Problem*.rs` files (out of scope) | 3 |
| 15 | `Algorithm*.rs` (named-algorithm files) | 4 |
| 16 | Standards documents | 29 |
| 17 | vstdplus modules | 26 |

## St / Mt / Per breakdown

| # | Variant | File count | Notes |
|---|---------|------------|-------|
| 18 | StEph (sequential, ephemeral) | 80 | Most chapters |
| 19 | MtEph (multi-threaded, ephemeral) | 52 | Parallel coverage |
| 20 | StPer (sequential, persistent) | 33 | Functional structures |
| 21 | MtPer (multi-threaded, persistent) | 20 | Concurrent persistent |

## Lines of code

| # | Scope | LOC |
|---|-------|-----|
| 22 | All `src/Chap*/` algorithm files | 186,223 |
| 23 | `src/vstdplus/` library | 6,719 |
| 24 | `src/standards/` | 6,911 |
| 25 | `src/Types.rs` + `src/Concurrency.rs` + `src/lib.rs` | 1,265 |
| 26 | RTT files in `tests/` | 60,576 |
| 27 | PTT files in `rust_verify_test/` | 13,320 |
| 28 | **Total project Rust LOC** | **275,014** |

## Per-chapter LOC table

| # | Chap | Files | LOC |
|---|------|-------|-----|
| 1 | Chap02 | 2 | 382 |
| 2 | Chap03 | 1 | 140 |
| 3 | Chap05 | 5 | 3,809 |
| 4 | Chap06 | 21 | 13,353 |
| 5 | Chap11 | 5 | 807 |
| 6 | Chap12 | 3 | 450 |
| 7 | Chap17 | 1 | 796 |
| 8 | Chap18 | 9 | 11,750 |
| 9 | Chap19 | 5 | 6,176 |
| 10 | Chap21 | 9 | 1,373 |
| 11 | Chap23 | 2 | 2,090 |
| 12 | Chap26 | 8 | 3,780 |
| 13 | Chap27 | 5 | 1,299 |
| 14 | Chap28 | 11 | 2,832 |
| 15 | Chap30 | 1 | 217 |
| 16 | Chap35 | 4 | 2,067 |
| 17 | Chap36 | 3 | 2,520 |
| 18 | Chap37 | 20 | 20,319 |
| 19 | Chap38 | 3 | 3,640 |
| 20 | Chap39 | 5 | 7,361 |
| 21 | Chap40 | 3 | 4,959 |
| 22 | Chap41 | 7 | 10,075 |
| 23 | Chap42 | 4 | 8,483 |
| 24 | Chap43 | 11 | 12,220 |
| 25 | Chap44 | 1 | 729 |
| 26 | Chap45 | 6 | 6,848 |
| 27 | Chap47 | 9 | 6,317 |
| 28 | Chap49 | 8 | 3,014 |
| 29 | Chap50 | 8 | 4,499 |
| 30 | Chap51 | 9 | 3,993 |
| 31 | Chap52 | 15 | 10,018 |
| 32 | Chap53 | 5 | 2,497 |
| 33 | Chap54 | 5 | 3,262 |
| 34 | Chap55 | 9 | 6,394 |
| 35 | Chap56 | 10 | 3,338 |
| 36 | Chap57 | 3 | 881 |
| 37 | Chap58 | 2 | 655 |
| 38 | Chap59 | 4 | 1,925 |
| 39 | Chap61 | 4 | 974 |
| 40 | Chap62 | 4 | 2,867 |
| 41 | Chap63 | 2 | 578 |
| 42 | Chap64 | 3 | 917 |
| 43 | Chap65 | 5 | 3,803 |
| 44 | Chap66 | 2 | 1,816 |
| | **Total** | **262** | **186,223** |

## Notes

- LOC count is naive `wc -l`; includes blank lines and comments. Spec/proof/exec
  split would require parsing each fn — see `scripts/all-fn-impls-by-chap.sh`
  for a per-fn breakdown if needed.
- Example/Problem files are excluded per CLAUDE.md (textbook demos, not
  algorithmic implementations).
- `Chap37` is the largest at 20,319 LOC (AVL trees, sequences, persistent variants).
- Smallest active chapter: Chap03 at 140 LOC (just InsertionSort).
