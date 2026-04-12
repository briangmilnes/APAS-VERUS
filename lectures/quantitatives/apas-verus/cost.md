# Lecture Data — Section 4: Cost

Generated: 2026-04-11 from main at `7ec8eb2bf`.

## Latest full validation pass

| # | Metric | Value |
|---|--------|-------|
| 1 | `verified` count | **5,674** |
| 2 | `errors` | **0** |
| 3 | Wall time (validate) | 223 s |
| 4 | Peak rust_verify RSS | 10,135 MB (~10.0 GB) |
| 5 | Peak Z3 RSS | 6,886 MB (~6.7 GB) |
| 6 | Combined peak RSS | ~17 GB |
| 7 | Min free RAM during run | 11,208 MB |
| 8 | rust_verify CPU time | 216 s |
| 9 | Z3 child CPU time | 282 s |

Source: `logs/validate.20260411-204645.log`.

## Latest RTT (run-time tests)

| # | Metric | Value |
|---|--------|-------|
| 10 | Tests run | **3,776** |
| 11 | Tests passed | 3,776 |
| 12 | Tests skipped | 0 |
| 13 | Wall time | 21 s |
| 14 | Test sum time (parallel) | 11.085 s |
| 15 | RTT files in `tests/` | 293 |
| 16 | `#[test]` annotations in `tests/` | 4,308 |
| 17 | `[[test]]` entries in `Cargo.toml` | 279 |

Source: `logs/rtt.20260411-205104.log`.

## Latest PTT (proof-time tests)

| # | Metric | Value |
|---|--------|-------|
| 18 | Tests run | **221** |
| 19 | Tests passed | 221 |
| 20 | Tests skipped | 0 |
| 21 | Wall time | 259 s |
| 22 | Test sum time | 48.656 s |
| 23 | PTT files in `rust_verify_test/tests/` | 93 |

Source: `logs/ptt.20260411-205159.log`.

## Validate timing/RSS over recent runs (sample)

| # | Timestamp | Verified | Elapsed | rust_verify RSS | Z3 RSS |
|---|-----------|----------|---------|-----------------|--------|
| 1 | 2026-04-11 20:46 | 5,674 | 223s | 10,135 MB | 6,886 MB |
| 2 | 2026-04-11 18:42 | 5,723 | 95s | 10,212 MB | 1,184 MB |
| 3 | 2026-04-11 18:35 | 5,724 | 229s | 10,214 MB | 1,569 MB |
| 4 | 2026-04-11 16:46 | 5,702 | 94s | 10,668 MB | 7,114 MB |
| 5 | 2026-04-11 16:43 | 5,702 | 95s | 10,163 MB | 620 MB |
| 6 | 2026-04-11 13:44 | 5,702 | 89s | 10,140 MB | 720 MB |
| 7 | 2026-04-11 09:19 | 5,689 | 85s | 10,121 MB | 635 MB |
| 8 | 2026-04-10 16:55 | 5,667 | 82s | 10,059 MB | 407 MB |
| 9 | 2026-04-10 16:22 | 5,667 | 109s | 14,740 MB | 958 MB |
| 10 | 2026-04-10 15:46 | 5,598 | 80s | 9,910 MB | 520 MB |
| 11 | 2026-04-10 15:43 | 5,598 | 88s | 12,732 MB | 8,180 MB |

Note the wide RSS swing for Z3 (407 MB → 8,180 MB) on similar workloads
— Z3 memory varies heavily based on which file lands first in the work
queue and which quantifiers happen to fire.

`rust_verify` RSS is more stable at ~10–14 GB.

Validate wall time consistently between **80–230 s** for the full crate.

## Hot-spot rlimit overrides

15 functions across 8 files have a non-default `#[verifier::rlimit(N)]`.
Of those, **8 are in `src/Chap65/` (UnionFind/Kruskal)**, with the
single largest at **rlimit 200** on `find()` in `UnionFindPCStEph.rs:431`
— the path-compression invariant. See `trust-base.md` for the full list.

## Notes

- `verified` count fluctuates between 5,598 and 5,724 in the last day's
  runs as files are added/moved/cfg-gated. R196 closed at 5,674.
- All run-time data lives in `logs/`; per-chapter profile data lives
  in `logs/profile/`. The aggregate "5,674 verified" includes spec
  functions, lemmas, and exec functions — not just executable code.
