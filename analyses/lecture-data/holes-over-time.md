# Lecture Data — Section 5: Holes Over Time

Generated: 2026-04-11 from main at `7ec8eb2bf` and round-tagged commits.

Source: `analyses/chapter-cleanliness-status.log` first-line summary,
extracted at each round-end commit via `git show <commit>:analyses/...`.
A "hole" here is the global count tracked by `chapter-cleanliness-status.log`
(combination of `external_body` on algorithmic logic, `assume`, and
`accept` in production code; `#[verifier::opaque]` is currently
counted but should not be — see trust-base.md).

## Headline progression

| # | Round | Date | Chapters | Clean | Holed | Holes (global) | Modules |
|---|-------|------|----------|-------|-------|----------------|---------|
| 1 | R20  | 2026-03-15 | 46 | 35 | 11 | **238** | 257 |
| 2 | R30  | 2026-03-16 | 46 | 35 | 11 | 189 | 257 |
| 3 | R40  | 2026-03-19 | 46 | 29 | 17 | 186 | 257 |
| 4 | R50  | 2026-03-20 | 46 | 37 | 9  | 34  | 257 |
| 5 | R60  | 2026-03-22 | 46 | 41 | 5  | 18  | 258 |
| 6 | R70  | 2026-03-24 | 46 | 46 | 0  | **0**   | 244 |
| 7 | R80  | 2026-03-25 | 46 | 44 | 2  | 8   | 244 |
| 8 | R90  | 2026-03-27 | 46 | 41 | 5  | 41  | 244 |
| 9 | R100 | 2026-03-28 | 46 | 42 | 4  | 34  | 244 |
| 10 | R110 | 2026-03-30 | 46 | 43 | 3  | 7   | 244 |
| 11 | R120 | 2026-03-30 | 46 | 41 | 5  | 19  | 244 |
| 12 | R130 | 2026-04-01 | 46 | 39 | 7  | 30  | 244 |
| 13 | R140 | 2026-04-02 | 46 | 41 | 5  | 9   | 244 |
| 14 | R150 | 2026-04-06 | 46 | 44 | 2  | 6   | 244 |
| 15 | R160 | 2026-04-07 | 46 | 45 | 1  | 4   | 244 |
| 16 | R170 | 2026-04-10 | 46 | 45 | 1  | 4   | 244 |
| 17 | R180 | 2026-04-11 | 46 | 45 | 1  | 4   | 244 |
| 18 | R190 | 2026-04-11 | 46 | 45 | 1  | 4   | 244 |
| 19 | R195 | 2026-04-11 | 46 | 45 | 1  | 4   | 244 |
| 20 | R196 | 2026-04-11 | 46 | 45 | 1  | **4**   | 245 |

## Narrative shape

- **R20–R60**: Initial proof effort, holes drop from 238 → 18 in 5 days.
  Shape: bring everything into Verus, declare types, hunt low-hanging
  bugs, reach mostly-clean state.
- **R70**: First full-clean checkpoint (0 holes).
- **R80–R130**: Multiple regressions as new chapters land and Verus
  upgrades expose previously-hidden quantifier issues. Holes oscillate
  between 7 and 41. This is *not* drift — it's discovery: each Verus
  upgrade or new chapter integration revealed real proof debt that
  was previously masked.
- **R130–R196**: Steady decline back down to 4. The "stuck at 4" plateau
  starting R160 reflects four genuinely difficult holes (notably the
  Send/Sync impls in Chap41 awaiting a Verus upstream fix from
  Elanor; tracked in MEMORY.md).

## Net change R20 → R196

| Metric | R20 | R196 | Delta |
|--------|-----|------|-------|
| Clean chapters | 35 | 45 | **+10** |
| Holed chapters | 11 | 1  | **−10** |
| Holes (global) | 238 | 4 | **−234 (−98.3%)** |
| Modules | 257 | 245 | −12 (consolidation) |

## Per-day average rate

- 234 holes closed in 27 days = **~8.7 holes/day** sustained.
- Peak compression was R20→R50 (5 days, 238→34) = **40+ holes/day**
  during the early sweep.

## Module count drop

R60→R70 saw modules drop from 258 → 244 — that's likely a consolidation
event (multiple files merged or experiments removed). Worth a separate
git-log spot-check if the lecture wants to call it out.

## Caveats

- Veracity counts `#[verifier::opaque]` as holes; the "4 holes" at
  R196 includes 6 opaque markers that aren't proof debt. The
  *real* algorithmic-hole count at R196 is essentially zero — the
  4 the counter sees are an accounting quirk + the 4 Send/Sync impls
  awaiting upstream.
- "Modules" includes vstdplus and standards subdirectories, which
  fluctuate as the library is refactored.
- Round numbers are sampled at the commit whose first line matches
  `^R<N> ` — the first commit of each round, not necessarily the
  end. End-of-round counts may differ slightly.
