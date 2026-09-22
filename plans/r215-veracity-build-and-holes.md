# r215 — build veracity, then count the proof holes in APAS-VERUS

Date: 2026-09-22. One subagent. Two products: `~/projects/veracity` (built,
never modified) and `~/projects/APAS-VERUS` (analysis output only).

## Why

`~/projects/veracity` does not exist on this machine; the repository is
`briangmilnes/veracity` on GitHub, last pushed 2026-05-30. Eighteen APAS-VERUS
scripts call its binaries, and the hole and cleanliness columns of
`docs/ChapterVerusification.md` have been blank since the upgrade because of
it. The crate now verifies on Verus 0.2026.09.13, so a hole count is the first
honest measurement of what the migration left behind.

## Rules

1. **Never modify veracity.** CLAUDE.md: its source is maintained by a
   separate agent. Clone it and build it. Do not edit its source, its
   `Cargo.toml`, or its lockfile to make a build succeed. If it does not
   build, record the exact error and stop that step; feedback for veracity
   goes in a file under `plans/`, not into its tree.
2. Read a script before running it. The hole scripts write into
   `src/ChapNN/analyses/` and `analyses/`; that output is the deliverable.
3. Do not run `scripts/validate.sh`, `rtt.sh`, or `ptt.sh`: the crate's state
   is already measured in `docs/ChaptersInOrder.md`. Another agent may be
   running Verus.
4. No subagents. Nothing committed until step 5, and then only the analysis
   output and the doc.

## Steps

| # | Step | Detail | Check |
|---|------|--------|-------|
| 1 | Clone | `git clone https://github.com/briangmilnes/veracity ~/projects/veracity`; record the head commit and its date | directory exists |
| 2 | Build | `cargo build --release` in that directory, with the project's pinned toolchain if it has one, otherwise the default 1.98.1. Read the whole output | the binaries the scripts name exist under `target/release/`: `veracity-review-proof-holes`, `veracity-review-verus-style`, `veracity-review-module-fn-impls`, and whatever else `grep -l veracity ~/projects/APAS-VERUS/scripts/*` reveals |
| 3 | Version check | veracity parses Verus source. Its last push predates the 09.13 upgrade, so it may not parse `assume_specification`, `broadcast axiom fn`, the prophetic iterator forms, or `#[cfg(verus_keep_ghost)]` on impls. Run `scripts/holes.sh src/Chap02/` first and read the output for parse failures before running anything crate-wide | parse failures counted, not hidden |
| 4 | Count | `scripts/all-holes-by-chap.sh`, then `scripts/chapter-cleanliness-status.sh`. Also `scripts/all-style-by-chap.sh` and `scripts/all-fn-impls-by-chap.sh` if they run clean | per-chapter logs written under `src/ChapNN/analyses/` |
| 5 | Report and commit | write `docs/HoleCount.md`; `git add -A`; commit with the Co-Authored-By line; push | pushed |

## Deliverable: `docs/HoleCount.md`

1. The veracity commit built, its date, the toolchain, and any build warnings.
2. Whether veracity parses the 09.13 sources, and every construct it could not
   parse, with a file and line for each.
3. The hole table: one row per chapter with `#`, `Chap`, holes by kind
   (`assume`, `accept`, `admit`, `external_body`, `unsafe impl`, weak or
   missing specs as veracity classifies them), and files affected.
4. The totals, and a comparison with the last recorded count (0 holes in
   45 of 46 chapters, 4 in Chap41, as of 2026-04-07 in
   `docs/ChapterVerusification.md`'s history).
5. The holes this migration is known to have added or removed, cross-checked
   against `docs/HashMigration.md`, `docs/ChaptersInOrder.md`, and the two
   `external_body` `Hash` bodies in Chap05.
6. Anything veracity reports that looks wrong, as feedback for its maintainer,
   in `plans/r215-veracity-feedback.md` rather than in its tree.
