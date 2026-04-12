# Lecture Data — Section 3: Trust Base

Generated: 2026-04-11 from main at `7ec8eb2bf`. Counts for `src/Chap*/`
(production code) only unless stated.

## Headline counts

| # | Construct | Count (Chap*/) | Notes |
|---|-----------|----------------|-------|
| 1 | `#[verifier::external_body]` | 184 | algorithmic + thread-spawn boundaries combined |
| 2 | `#[verifier::external_body]` in `vstdplus/` | 82 | library bridges (Arc, RwLock, etc.) |
| 3 | `#[verifier::external_body]` codebase-wide | 365 | including standards/experiments |
| 4 | `assume(...)` calls (line-leading) | 38 | strict count, statement form |
| 5 | `assume(` total occurrences | 419 | includes `if cond { assume(...) }` etc. |
| 6 | `accept(...)` calls (line-leading) | 45 | accept = `assume + admit` per `vstdplus/accept` |
| 7 | `accept(` total occurrences | 269 | |
| 8 | `admit()` calls | **0** | as required by CLAUDE.md |

## Optimization markers

| # | Construct | Count |
|---|-----------|-------|
| 9 | `#[verifier::opaque]` | 6 |
| 10 | `pub broadcast group ...` declarations | 20 |
| 11 | `broadcast use ...` sites | 314 |
| 12 | `#[verifier::rlimit(N)]` overrides (production) | 15 |

## Concurrency primitives

| # | Construct | Count |
|---|-----------|-------|
| 13 | `join(...)` call sites (fork-join points) | 91 |

## Rlimit override hot-spots

| # | Chap | File | Line | rlimit |
|---|------|------|------|--------|
| 1 | 26 | ETSPStEph.rs | 310 | 20 |
| 2 | 26 | ETSPMtEph.rs | 223 | 40 |
| 3 | 28 | MaxContigSubSumOptStEph.rs | 119 | 50 |
| 4 | 37 | AVLTreeSeqStPer.rs | 431 | 15 |
| 5 | 37 | AVLTreeSeqMtPer.rs | 390 | 80 |
| 6 | 41 | OrdKeyMap.rs | 1298 | 20 |
| 7 | 62 | StarPartitionMtEph.rs | 1519 | 20 |
| 8 | 65 | KruskalStEph.rs | 286 | 50 |
| 9 | 65 | UnionFindNoPCStEph.rs | 150 | 20 |
| 10 | 65 | UnionFindNoPCStEph.rs | 530 | 30 |
| 11 | 65 | UnionFindPCStEph.rs | 238 | 20 |
| 12 | 65 | UnionFindPCStEph.rs | 431 | **200** ← largest |
| 13 | 65 | UnionFindPCStEph.rs | 489 | 80 |
| 14 | 65 | UnionFindPCStEph.rs | 625 | 20 |
| 15 | 65 | UnionFindPCStEph.rs | 1084 | 40 |

Chap65 (UnionFind) accounts for **8 of 15** rlimit overrides — concentrated
where path-compression invariants are heaviest.

## Caveats / categorization

The headline `assume`/`accept` total counts include *all* occurrences,
which over-counts (a single `if cond { assume(x) }` registers as one).
The "line-leading" form is the safer count of distinct proof-debt
statements: **38 assume + 45 accept = 83 trust-base statements** in
production code. Line-leading is conservative — it misses some legitimate
usages — so the real number is between 83 and the inflated 688.

`#[verifier::opaque]` is **not** a hole — it's proof bundling. Veracity's
proof-hole reviewer counts it as a hole and inflates the visible
"hole" number; subtract 6 from any veracity-reported hole tally.

## Comparison to current cleanliness

`analyses/chapter-cleanliness-status.log` (R196):
**46 chapters, 45 clean, 1 holed, 4 holes (global), 245 modules.**

The "4 holes" is the veracity-reported residual; subtracting the 6
`#[verifier::opaque]` annotations would actually take it negative —
suggesting the cleanliness counter uses a different definition than
the simple `external_body + assume + accept` aggregate.
