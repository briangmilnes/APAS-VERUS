# r218 B: Z3 input for the same function, isolate vs full crate

RESULT: SUCCEEDS (the question is answered). For both targets the SMT content
Verus sends to Z3 is the same set of commands in both configurations; only the
order of the module prelude differs. Reordering alone reproduces the pass and
the fail in a standalone Z3, and both targets are unstable under Z3 seeds and
under changes to earlier queries in the same Z3 session.

DATE: 2026-09-22

Toolchain: Verus 0.2026.09.13.671956e (commit
671956ec527d3b7164779f767bdbfe769bedce6c, release, rust 1.98.1), Z3 4.16.0,
APAS-VERUS commit 89abdb897 (branch r218/z3-query-diff, no `spinoff_prover`
attributes).

## Question

`prefix_sums_dc_inner` (Chap26 `ScanDCMtPer.rs:119`) and `select_inner`
(Chap35 `OrderStatSelectMtEph.rs:493`) verify under
`scripts/validate.sh isolate ChapNN` and fail under the full crate. What
differs in the Z3 input, and is the difference in the query or in the Z3
session?

## Capture

Each capture ran `scripts/validate.sh` with
`VERUS_EXTRA_ARGS="--verify-module <module> --log smt-transcript --log smt --log-dir <dir> --time-expanded --output-json"`.
`--verify-module` keeps the module's whole Z3 session (prelude plus every
query of the module, in order), so the transcript contains the prior queries.
Each outcome under logging matches the unlogged validate runs (isolate passes, full fails).

| # | Chap | Dir | Config | Verus result | Verus JSON rlimit |
|---|------|-----|--------|--------------|-------------------|
| 1 | 26 | `chap26/iso-mod` | isolate Chap26 | verified | 5,035,146 |
| 2 | 26 | `chap26/full-mod` | full crate | rlimit exceeded | 30,146,410 |
| 3 | 35 | `chap35/iso-mod` | isolate Chap35 | verified | 612,369 |
| 4 | 35 | `chap35/full-mod` | full crate | assertion failed | 994,569 |
| 5 | 35 | `chap35/iso-spinoff-mod` | isolate, `-V spinoff-all` | assertion failed | 970,670 |

Console logs: `chap26/*.console.log`, `chap35/*.console.log` (copies of the
`logs/validate.*.log` files of the same runs, also committed in `logs/`).

## Structural comparison

`scripts/compare-blocks.sh` and `scripts/compare-commands.sh` normalize names
that depend only on emission order or on the crate-wide closure counter
(`anonymous_closure%N`, `impl_closure&__FnOnceN` in qids, `%%lambda%%N`,
`%%global_location_label%%N`, span ids `(#N)`, line wrapping) and compare the
files as multisets of blocks and of top-level commands.

| # | Chap | Measure | Isolate | Full |
|---|------|---------|---------|------|
| 1 | 26 | blocks | 215 | 215 |
| 2 | 26 | declare-sort / declare-fun | 14 / 196 | 14 / 196 |
| 3 | 26 | declare-const / assert / forall | 1193 / 542 / 381 | 1193 / 542 / 381 |
| 4 | 26 | blocks only in one file | 0 | 0 |
| 5 | 26 | header-order diff lines | 32 | 32 |
| 6 | 26 | queries before target | 6 | 6 |
| 7 | 26 | Z3 rlimit counter before target | 259,575 | 259,539 |
| 8 | 26 | target rlimit used | 4,888,736 unsat | 30,000,000 unknown (canceled) |
| 9 | 35 | blocks (full: error reruns removed) | 601 | 601 |
| 10 | 35 | declare-sort / declare-fun | 9 / 320 | 9 / 320 |
| 11 | 35 | declare-const / assert / forall | 795 / 1070 / 871 | 795 / 1070 / 871 |
| 12 | 35 | blocks only in one file | 0 | 0 |
| 13 | 35 | header-order diff lines | 124 | 124 |
| 14 | 35 | queries before target | 8 | 8 |
| 15 | 35 | Z3 rlimit counter before target | 1,821,882 | 1,716,089 |
| 16 | 35 | target rlimit used | 612,369 unsat | 916,388 unknown (incomplete quantifiers) |

Files: `chap26/compare-chap26.txt`, `chap35/compare-chap35.txt`,
`chap35/compare-iso-vs-fullclean.txt`, `chap35/prelude-diff/`.
The raw full Chap35 log contains Verus's error-localization reruns after the
failure; `chap35/variants/full-clean.smt2` removes them (lines 20433-20437,
20439-20442, 20445-22387) so that the two files can be compared and replayed.
The target query text of Chap26 is identical (`chap26/normalized/*-target-block.smt2`,
2335 lines; the full one adds only `(get-info :reason-unknown)`).

## Replay experiments (standalone Z3 4.16.0)

All replays run `z3 -smt2 <file>`; outputs are in `*/replay/`.
`scripts/target-rlimit.sh` reads the rlimit used by the target check-sat
(7th in Chap26, 9th in Chap35).

| # | Chap | Variant | Content from | Order from | Target |
|---|------|---------|--------------|------------|--------|
| 1 | 26 | iso-mod | isolate | isolate | unsat, 4,888,736 |
| 2 | 26 | full-mod | full | full | unknown, 30,000,000 |
| 3 | 26 | A | full, closure ids renamed to isolate's | full | unknown |
| 4 | 26 | B | isolate, closure ids renamed to full's | isolate | unsat, 4,888,736 |
| 5 | 26 | C | isolate | full | unknown |
| 6 | 26 | D | full | isolate | unsat, 4,888,736 |
| 7 | 26 | E hybrid k=71 | isolate | first 71 full, rest isolate | unsat |
| 8 | 26 | F hybrid k=72 | isolate | first 72 full, rest isolate | unknown |
| 9 | 26 | G target only | isolate | isolate, no prior queries | unknown |
| 10 | 26 | H target only | full | full, no prior queries | unknown |
| 11 | 35 | iso-mod | isolate | isolate | unsat, 612,369 |
| 12 | 35 | full-clean | full | full | unknown, 916,388 |
| 13 | 35 | C | isolate | full | unknown |
| 14 | 35 | D | full | isolate | unsat, 612,369 |
| 15 | 35 | G target only | isolate | isolate, no prior queries | unknown |
| 16 | 35 | H target only | full | full, no prior queries | unknown |

Hybrid scans (`*/replay/hybrid-scan.tsv`, `scripts/hybrid.sh`): the first k
blocks in full order, the rest in isolate order.
- Chap26: k = 0..71 pass, k = 72..180 fail. Block 72 of the full order is
  `;; Function-Axioms vstd::seq_lib::impl&%0::drop_last`. Moving the
  `drop_last` spec and axioms ahead of `Function-Axioms vstd::seq::impl&%2::spec_add`
  and the broadcasts `add_empty_left`, `add_empty_right`,
  `push_distributes_over_add` is the single step that flips the result.
- Chap35: the result flips five times as k goes from 0 to 601 (fail at k=22,
  180, 182..250, 252..601; pass otherwise).

Prior-query subsets (`*/replay/prior-query-subsets.tsv`,
`scripts/keep-queries.sh`): keeping the prelude and dropping some earlier
queries changes the target's rlimit from 4.9M to over 30M in Chap26 and
flips pass/fail in both chapters. Full order with Chap26 queries 2..7 passes
(17,015,107).

Seeds (`*/replay/random-seed.tsv`, `z3 smt.random_seed=N`):
- Chap26: isolate passes 6 of 8 seeds, full passes 2 of 8.
- Chap35: isolate passes 6 of 16 seeds, full passes 5 of 16.

## Profiler panic (separate finding)

`profile-panic/`: `-V capture-profiles` panics at
`rust_verify/src/profiler.rs:29:45` (`Option::unwrap()` on `None`) while
writing the instantiation graph for `Chap05::RelationStEph::RelationStEph`.
Reproduced alone with `--verify-module Chap05::RelationStEph::RelationStEph`
(log `profile-repro-Chap05-RelationStEph.*.log`, profile
`Chap05__RelationStEph__RelationStEph.profile.gz`). The `MappingStEph`
module named in the original full-crate log does not panic alone
(`profile-repro-Chap05-MappingStEph.*.log`); the single-thread full run
(`profile-repro-full-threads1.*.log`) shows the panic follows the
`RelationStEph` analysis.

## Layout

- `scripts/`: shell and awk tools used above (no Python, no Perl).
- `chap26/`, `chap35/`: raw Verus logs (`*-mod/`), normalized copies,
  variants, replay outputs. Large SMT files are gzipped.
- `profile-panic/`: the profiler panic evidence.

## Reproducing

SMT logs, transcripts and replay outputs over 50 KB are gzipped; `gunzip -k`
them (or pipe through `zcat`) before running the scripts, which take plain
`.smt2` paths. Example, from `chap26/`:

```bash
gunzip -k iso-mod/*.smt2.gz full-mod/*.smt2.gz
Z3=~/projects/verus/source/target-verus/release/z3
../scripts/reorder.sh iso-mod/*.smt2 full-mod/*.smt2 > /tmp/C.smt2   # isolate content, full order
$Z3 -smt2 /tmp/C.smt2 > /tmp/C.out && ../scripts/target-rlimit.sh /tmp/C.out 7   # unknown 30000000
../scripts/hybrid.sh iso-mod/*.smt2 iso-mod/*.smt2 full-mod/*.smt2 72 > /tmp/F.smt2   # fails; 71 passes
```
