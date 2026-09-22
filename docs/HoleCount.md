<style>
body { max-width: 95% !important; width: 95% !important; margin: 0 auto !important; padding: 1em !important; }
.markdown-body { max-width: 95% !important; width: 95% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 95% !important; width: 95% !important; }
table { width: 100% !important; }
</style>

# Proof hole count, 2026-09-22 (r215)

First hole measurement since the Verus 0.2026.09.13 migration. Eighteen APAS-VERUS
scripts call veracity binaries, and none had been runnable since the upgrade, so the
hole columns of `docs/ChapterVerusification.md` were blank. This document records the
build, what veracity can and cannot see, and the counts.

A checkout of `briangmilnes/veracity` `main` was already on disk at
`~/projects/veracity` when this round started, with no `target/` directory; its
remote and head were checked against GitHub rather than cloned again. Nothing in it
was edited.

The crate state this measures is the one in `docs/ChaptersInOrder.md`: 44 chapters,
each `scripts/validate.sh isolate ChapNN` reporting 0 errors and 0 warnings on Verus
`0.2026.09.13.671956e`. No Verus run was made for this document.

## 1. The build

| # | Item | Value |
|---|------|-------|
| 1 | Repository | `github.com/briangmilnes/veracity` |
| 2 | Branch | `main` |
| 3 | Commit | `f60414ce072e424e13eb9e6b0117c543cda090b7` |
| 4 | Commit date | 2026-05-24 07:43:15 -0700 |
| 5 | Subject | iterator-upgrade: r205 — D6-companion rewrite |
| 6 | Toolchain | Rust 1.91.0, pinned by its `rust-toolchain.toml` |
| 7 | Command | `cargo build --release -j 6` |
| 8 | Result | 0 errors, 41 warnings, 3m04s |
| 9 | Binaries produced | 49 |

No file in `~/projects/veracity` was modified, and none needed to be.

Rustup installed the pinned 1.91.0 toolchain on demand; APAS-VERUS itself pins
1.98.1, and the two do not interact because veracity builds in its own directory.

The 41 warnings are `unused`, `dead_code` and `unused_mut` in 13 of the 49 binaries.
Of the binaries this round used, `veracity-review-proof-holes` has 3 and
`veracity-review-verus-style` has 7; `veracity-review-module-fn-impls` has none.
All 41 are unused names, not behaviour.

The seven binaries the APAS-VERUS scripts name all exist:
`veracity-review-proof-holes`, `veracity-review-verus-style`,
`veracity-review-module-fn-impls`, `veracity-analyze-alg-analysis`,
`veracity-minimize-lib`, `veracity-paths-read`, `veracity-paths-write`.
`veracity-review-verus-proof-holes`, which appears in six scripts, is a log file
name, not a binary.

## 2. Does veracity parse the 0.2026.09.13 sources?

Yes, every scanned file.

Veracity does not carry its own Verus grammar. It depends on
`verus_syn = { path = "../verus/dependencies/syn" }`, so its parser is whatever
Verus checkout is on the machine — here `671956ec`, the 0.2026.09.13 head. A
commit pushed in May therefore parses September syntax: `assume_specification`,
`broadcast axiom fn`, the prophetic `IteratorSpecImpl` forms and
`#[cfg(verus_keep_ghost)]` on impls all go through the Verus parser unchanged.

This had to be measured rather than inferred, because veracity hides parse
failures: `analyze_verus_block` matches `verus_syn::parse_file(inner)` and, on
`Err(_)`, silently falls back to token analysis with no diagnostic
(`src/bin/review_verus_proof_holes.rs:2964`). A file that fails to parse still
appears in the tables.

A probe was therefore written that extracts `verus!` token trees exactly as
veracity does (`ra_ap_syntax`, `MACRO_CALL` with path `verus` or `verus_`) and runs
`verus_syn::parse_file` on each block, reporting every `Err`. Over
`~/projects/APAS-VERUS/src`: 530 files scanned, 520 `verus!` blocks, 1 parse
failure.

| # | Chap | File | Line | Construct | Scanned? |
|---|------|------|-----:|-----------|----------|
| 1 | - | `experiments/generic_specs_to_prevent_cycles.rs` | 91 | type parameter on a `spec fn` in a trait | no |

That file's own header records the same result: pattern 4c is marked
`PARSE ERROR — Verus does not support type parameters on spec fns in traits`. The
failure is Verus rejecting the construct, not veracity lagging behind it. Veracity
excludes `experiments/`, so no file it scans falls back to token analysis.

The probe was a throwaway Rust crate in the session scratchpad, not committed. To
rebuild it: one dependency on `ra_ap_syntax = "0.0.233"` and one on
`verus_syn = { path = "/home/milnes/projects/verus/dependencies/syn" }`; walk `src`
for `*.rs`, `SourceFile::parse` each, take every `MACRO_CALL` whose path is `verus`
or `verus_`, slice the token tree's text between its braces, and print every
`verus_syn::parse_file` `Err` with the file and the brace line plus the error's
span.

## 3. What veracity does not look at

`should_exclude` always drops `docs`, `path`, `src/lib.rs`, `src/Types.rs`, and any
directory named `experiments`, `vstdplus` or `standards`, whatever `-e` says. The
report does not mention that it did so. Holes in that scope are counted nowhere.

| # | Scope | `admit()` | `assume(` | `external_body` | `unsafe impl` |
|---|-------|----------:|----------:|----------------:|--------------:|
| 1 | `src/Types.rs` | 9 | 0 | 0 | 0 |
| 2 | `src/vstdplus/` | 14 | 20 | 62 | 1 |
| 3 | `src/standards/` | 0 | 17 | 3 | 4 |

The nine in `src/Types.rs` are `broadcast proof fn ... { admit(); }` axioms at lines
291–353: `axiom_Pair_view_injective`, and `_feq` / `_key_model` pairs for `Pair`,
`Edge`, `LabEdge` and `WeightedEdge`. They assert view injectivity,
`obeys_feq_full` and `obeys_key_model` for the four APAS carrier types and are used
crate-wide through `group_Pair_axioms` and its siblings. They are the largest block
of admit-based proof debt outside `vstdplus/`, and the counts below do not contain
them.

Two further paths are in scope by default but are not APAS source. `scratch/` held a
389-file working copy of `src/` on the day of the run, and `bugs/` holds three Verus
reproducers. `scripts/all-holes-by-chap.sh` excludes neither, so as written it
reported 614 modules, 596 accepted holes and 9 `admit()` holes — the last existing
only in `scratch/srccopy/Types.rs`. Every figure below comes from the same command
with `-e scratch -e bugs` added; the committed
`analyses/veracity-review-verus-proof-holes.log` is that run.

## 4. The hole table

One row per chapter. `Mods` is modules scanned, `Files` the number carrying at least
one non-structural finding. Kinds: `acc` = `accept()`, `extB` =
`external_body_accept_hole`, `trWf` = `trivial_spec_wf`, `reqM` =
`fn_missing_requires`, `ensM` = `fn_missing_ensures`, `rwlk` =
`dummy_rwlock_predicate`, `opq` = `opaque`, `unsf` = `unsafe_block_accept_hole`,
`oth` = `external_accept_hole` plus struct/enum/type-specification outside `verus!`.
`Str` is structural findings veracity marks info-only. `ensM` and `rwlk` are
error-severity; everything else in the table is reviewed-and-accepted or info.

Actionable holes — bare `assume`, `admit`, `external_body` on algorithmic logic —
are 0 in every chapter. There is no `admit()` and no `unsafe impl` outside the six
`Send`/`Sync` marker impls in Chap38, Chap39 and Chap43, which veracity classifies
as structural.

| # | Chap | Mods | Files | acc | extB | trWf | reqM | ensM | rwlk | opq | unsf | oth | Str |
|---|------|-----:|------:|----:|-----:|-----:|-----:|-----:|-----:|----:|-----:|----:|----:|
| 1 | 2 | 2 | 1 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 4 |
| 2 | 3 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 3 | 5 | 5 | 3 | 6 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 |
| 4 | 6 | 21 | 4 | 25 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 |
| 5 | 11 | 5 | 3 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 6 | 12 | 3 | 2 | 0 | 13 | 1 | 0 | 0 | 0 | 0 | 4 | 1 | 0 |
| 7 | 17 | 1 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 8 | 18 | 9 | 8 | 10 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 2 | 0 |
| 9 | 19 | 5 | 4 | 4 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 10 | 21 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 11 | 23 | 2 | 2 | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 12 | 26 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
| 13 | 27 | 5 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 |
| 14 | 28 | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 15 | 30 | 1 | 1 | 0 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 |
| 16 | 35 | 4 | 2 | 0 | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| 17 | 36 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 18 | 37 | 20 | 13 | 23 | 0 | 0 | 7 | 0 | 0 | 0 | 0 | 0 | 58 |
| 19 | 38 | 3 | 2 | 2 | 0 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 2 |
| 20 | 39 | 5 | 4 | 15 | 0 | 4 | 2 | 0 | 0 | 0 | 0 | 0 | 10 |
| 21 | 40 | 3 | 3 | 20 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 22 | 41 | 7 | 6 | 17 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 2 |
| 23 | 42 | 4 | 3 | 5 | 0 | 0 | 0 | 3 | 0 | 0 | 0 | 0 | 0 |
| 24 | 43 | 11 | 7 | 7 | 0 | 2 | 1 | 4 | 0 | 0 | 0 | 0 | 55 |
| 25 | 44 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
| 26 | 45 | 6 | 6 | 12 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 27 | 47 | 9 | 2 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
| 28 | 49 | 8 | 2 | 0 | 0 | 0 | 2 | 0 | 2 | 0 | 0 | 0 | 0 |
| 29 | 50 | 8 | 8 | 18 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 0 | 0 |
| 30 | 51 | 9 | 8 | 8 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 31 | 52 | 16 | 5 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 32 | 53 | 5 | 2 | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 33 | 54 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 34 | 55 | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 35 | 56 | 10 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 36 | 57 | 3 | 1 | 0 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 |
| 37 | 58 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 38 | 59 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 39 | 61 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 |
| 40 | 62 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
| 41 | 63 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 42 | 64 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 43 | 65 | 5 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 0 | 0 |
| 44 | 66 | 2 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 |
| - | Total | 259 | 108 | 203 | 32 | 28 | 18 | 7 | 4 | 4 | 4 | 6 | 155 |

The 44 chapter rows cover 259 modules. `src/Concurrency.rs` (1 module, 1
`external_body_accept_hole`) and `src/ParaPairs.rs` (1 module, no findings) bring the
crate total to 261 modules, 296 accepted holes and 109 files with a finding.

### Files carrying a finding

Same columns, one row per file.

| # | Chap | File | acc | extB | trWf | reqM | ensM | rwlk | opq | unsf | oth |
|---|------|------|----:|-----:|-----:|-----:|-----:|-----:|----:|-----:|----:|
| 1 | 2 | HFSchedulerMtEph.rs | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 3 |
| 2 | 5 | MappingStEph.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 3 | 5 | SetMtEph.rs | 4 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 4 | 5 | SetStEph.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 5 | 6 | DirGraphMtEph.rs | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 6 | 6 | LabDirGraphMtEph.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 7 | 6 | LabUnDirGraphMtEph.rs | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 8 | 6 | UnDirGraphMtEph.rs | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 9 | 11 | FibonacciMtEph2Threads.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 10 | 11 | FibonacciMtEphRecomputes.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 11 | 11 | FibonacciMtPerTSM.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 12 | 12 | Exercise12_1.rs | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 13 | 12 | Exercise12_5.rs | 0 | 7 | 1 | 0 | 0 | 0 | 0 | 4 | 1 |
| 14 | 17 | MathSeq.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 15 | 18 | ArraySeqMtEph.rs | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 16 | 18 | ArraySeqMtEphSlice.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 17 | 18 | ArraySeqMtPer.rs | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 18 | 18 | ArraySeq.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 |
| 19 | 18 | ArraySeqStEph.rs | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 20 | 18 | ArraySeqStPer.rs | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 21 | 18 | LinkedListStEph.rs | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 22 | 18 | LinkedListStPer.rs | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 23 | 19 | ArraySeqMtEph.rs | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 24 | 19 | ArraySeqMtEphSlice.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 25 | 19 | ArraySeqStEph.rs | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 26 | 19 | ArraySeqStPer.rs | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 27 | 23 | BalBinTreeStEph.rs | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 28 | 23 | PrimTreeSeqStPer.rs | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 29 | 27 | ScanContractStEph.rs | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 |
| 30 | 30 | Probability.rs | 0 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 31 | 35 | OrderStatSelectMtEph.rs | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| 32 | 35 | OrderStatSelectMtPer.rs | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| 33 | 37 | AVLTreeSeqMtPer.rs | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 34 | 37 | AVLTreeSeq.rs | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 35 | 37 | AVLTreeSeqStEph.rs | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 36 | 37 | AVLTreeSeqStPer.rs | 4 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| 37 | 37 | BSTAVLMtEph.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 38 | 37 | BSTBBAlphaMtEph.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 39 | 37 | BSTPlainMtEph.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 40 | 37 | BSTRBMtEph.rs | 1 | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 0 |
| 41 | 37 | BSTSetAVLMtEph.rs | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| 42 | 37 | BSTSetBBAlphaMtEph.rs | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| 43 | 37 | BSTSetPlainMtEph.rs | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| 44 | 37 | BSTSetRBMtEph.rs | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| 45 | 37 | BSTSplayMtEph.rs | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 46 | 38 | BSTParaMtEph.rs | 2 | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 0 |
| 47 | 38 | BSTParaStEph.rs | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| 48 | 39 | BSTParaTreapMtEph.rs | 4 | 0 | 1 | 1 | 0 | 0 | 0 | 0 | 0 |
| 49 | 39 | BSTSetTreapMtEph.rs | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 50 | 39 | BSTTreapMtEph.rs | 8 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| 51 | 39 | BSTTreapStEph.rs | 2 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| 52 | 40 | BSTKeyValueStEph.rs | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 53 | 40 | BSTReducedStEph.rs | 6 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 54 | 40 | BSTSizeStEph.rs | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 55 | 41 | ArraySetEnumMtEph.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 56 | 41 | ArraySetStEph.rs | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 57 | 41 | AVLTreeSetMtEph.rs | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 58 | 41 | AVLTreeSetMtPer.rs | 1 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| 59 | 41 | AVLTreeSetStEph.rs | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 60 | 41 | AVLTreeSetStPer.rs | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 61 | 42 | TableMtEph.rs | 2 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| 62 | 42 | TableStEph.rs | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| 63 | 42 | TableStPer.rs | 2 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| 64 | 43 | AugOrderedTableMtEph.rs | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 0 | 0 |
| 65 | 43 | AugOrderedTableStEph.rs | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| 66 | 43 | AugOrderedTableStPer.rs | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| 67 | 43 | OrderedSetMtEph.rs | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 68 | 43 | OrderedTableMtEph.rs | 2 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| 69 | 43 | OrderedTableMtPer.rs | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 70 | 43 | OrderedTableStPer.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 71 | 44 | DocumentIndex.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 72 | 45 | BalancedTreePQ.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 73 | 45 | BinaryHeapPQ.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 74 | 45 | HeapsortExample.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 75 | 45 | LeftistHeapPQ.rs | 4 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| 76 | 45 | SortedListPQ.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 77 | 45 | UnsortedListPQ.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 78 | 47 | FlatHashTable.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 79 | 47 | StructChainedHashTable.rs | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 80 | 49 | SubsetSumMtEph.rs | 0 | 0 | 0 | 1 | 0 | 1 | 0 | 0 | 0 |
| 81 | 49 | SubsetSumMtPer.rs | 0 | 0 | 0 | 1 | 0 | 1 | 0 | 0 | 0 |
| 82 | 50 | MatrixChainMtEph.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 83 | 50 | MatrixChainMtPer.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 84 | 50 | MatrixChainStEph.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 85 | 50 | MatrixChainStPer.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 86 | 50 | OptBinSearchTreeMtEph.rs | 3 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 |
| 87 | 50 | OptBinSearchTreeMtPer.rs | 3 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 |
| 88 | 50 | OptBinSearchTreeStEph.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 89 | 50 | OptBinSearchTreeStPer.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 90 | 51 | BottomUpDPMtEph.rs | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 91 | 51 | BottomUpDPMtPer.rs | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 92 | 51 | BottomUpDPStEph.rs | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 93 | 51 | BottomUpDPStPer.rs | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 94 | 51 | TopDownDPMtEph.rs | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 95 | 51 | TopDownDPMtPer.rs | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 96 | 51 | TopDownDPStEph.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 97 | 51 | TopDownDPStPer.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 98 | 52 | AdjMatrixGraphStPer.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 99 | 52 | AdjSeqGraphStPer.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 100 | 52 | EdgeSetGraphMtEph.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 101 | 52 | EdgeSetGraphStEph.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 102 | 52 | EdgeSetGraphStPer.rs | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 103 | 53 | PQMinStEph.rs | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 104 | 53 | PQMinStPer.rs | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 105 | 57 | DijkstraStEphF64.rs | 0 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 106 | 65 | PrimStEph.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 107 | 65 | UnionFindPCStEph.rs | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 0 |
| 108 | 66 | BoruvkaMtEph.rs | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 109 | - | Concurrency.rs | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## 5. Totals, and the comparison with the last recorded count

| # | Quantity | 2026-04-20 baseline | 2026-09-22 | Change |
|---|----------|--------------------:|-----------:|-------:|
| 1 | Modules scanned | 261 | 261 | 0 |
| 2 | Modules clean, no error | 261 | 251 | −10 |
| 3 | Modules with an error | 0 | 10 | +10 |
| 4 | Actionable holes | 0 | 0 | 0 |
| 5 | Error-severity findings | 0 | 11 | +11 |
| 6 | Accepted (reviewed) holes | 294 | 296 | +2 |
| 7 | Structural, info only | 157 | 155 | −2 |
| 8 | Proof fns, all clean | 544 | 565 | +21 |
| 9 | Exec fns, requires+ensures | 5,910 | 5,795 | −115 |
| 10 | Exec fns, spec missing | 56 | 63 | +7 |
| 11 | Proof/spec fns counted | 3,100 | 2,430 | −670 |

The baseline is the committed `analyses/veracity-review-verus-proof-holes.log`, last
written 2026-04-20. Its scan included the three `bugs/` reproducers, which the
current run excludes, so the baseline column has their contribution subtracted: 3
modules, 1 of them with errors, 5 proof functions, 2 exec fns with no spec, 2
accepted holes, 2 `fn_missing_ensures` and 2 `fn_missing_requires`. Their
contribution was measured by running the current scan with and without `-e bugs`.

Rows 9 and 11 are large and this document cannot account for them. 670 fewer
proof/spec functions and 115 fewer fully specified exec functions is far more than
the migration's named edits, and the two logs come from different veracity builds
(see the caveat below). Either functions were deleted between April and September or
the classifier's function-counting changed; `analyses/veracity-count-loc.log`, not
refreshed here, is where that question is settled.

By kind:

| # | Kind | Severity | 2026-04-20 | 2026-09-22 | Change |
|---|------|----------|-----------:|-----------:|-------:|
| 1 | `accept()` | accepted | 216 | 203 | −13 |
| 2 | `external_body_accept_hole` | accepted | 33 | 33 | 0 |
| 3 | `trivial_spec_wf` | accepted | 16 | 28 | +12 |
| 4 | `fn_missing_requires` | accepted | 15 | 18 | +3 |
| 5 | `opaque` | accepted | 4 | 4 | 0 |
| 6 | `unsafe_block_accept_hole` | accepted | 4 | 4 | 0 |
| 7 | `external_accept_hole` | accepted | 3 | 3 | 0 |
| 8 | outside-`verus!` struct/enum/type | accepted | 3 | 3 | 0 |
| 9 | `fn_missing_ensures` | error | 0 | 7 | +7 |
| 10 | `dummy_rwlock_predicate` | error | 0 | 4 | +4 |
| 11 | `assume` / `admit` / `external_body` on logic | actionable | 0 | 0 | 0 |

The earlier figure the round plan cites — "0 errors, 4 holes (Chap41 only), 45/46
chapters clean", `docs/ChapterVerusification.md` at commit `901ed4b4`, dated
2026-04-07 — is two weeks older than the committed log. Those four Chap41 holes were
already closed by 2026-04-20, so nothing in this round bears on them.

One caveat limits every row above. Veracity stamps no version into its log
(`plans/r215-veracity-feedback.md` §1), so the April log's producing build is
unknown while this one is `f60414ce`. Where a count moved, a change in the
classifier cannot be excluded by the logs alone. The attributions in §6 are the ones
that a named source change in `docs/HashMigration.md` accounts for, checked against
the file and line.

## 6. What the migration added and removed

No new `assume`, `admit` or `external_body` appears anywhere. Six of the eleven new
error-severity findings are `.finite()` deletions that `docs/HashMigration.md` names
by file and line; the other five are treated below.

| # | Chap | File | Finding | `HashMigration.md` |
|---|------|------|---------|--------------------|
| 1 | 49 | `SubsetSumMtEph.rs` | `dummy_rwlock_predicate` | §2.5 item 6, §5 item 13 |
| 2 | 49 | `SubsetSumMtPer.rs` | `dummy_rwlock_predicate` | §2.5 item 6, §5 item 13 |
| 3 | 50 | `OptBinSearchTreeMtEph.rs` | `dummy_rwlock_predicate` | §2.6 item 8, §5 item 13 |
| 4 | 50 | `OptBinSearchTreeMtPer.rs` | `dummy_rwlock_predicate` | §2.6 item 8, §5 item 13 |
| 5 | 49 | `SubsetSumMtEph.rs` | `fn_missing_requires` | §2.5 item 6 (`new_arc_memo`) |
| 6 | 49 | `SubsetSumMtPer.rs` | `fn_missing_requires` | §2.5 item 6 (`new_arc_memo`) |

The four `RwLockPredicate::inv` bodies are now literally `true`. `HashMigration.md`
§5 item 2 states the case: `v@.dom().finite()` is identically true at vstd
0.2026.09.13, because `Set` is finite by type, so no invariant was lost — but the
predicate now says nothing, which is what veracity reports. Item 13 of the same
section already lists it as open.

The twelve new `trivial_spec_wf` findings are the same deletion applied to
well-formedness predicates whose only conjunct was a `.finite()`. Two read directly:

- `src/Chap41/AVLTreeSetMtEph.rs:359`, `open spec fn spec_avltreesetmteph_wf(&self) -> bool { true }`.
- `src/Chap43/OrderedSetMtEph.rs:294`, `#[verifier::type_invariant] spec fn wf(self) -> bool { true }`.

New sites: Chap05 `SetMtEph.rs`; Chap39 `BSTParaTreapMtEph.rs`,
`BSTSetTreapMtEph.rs`, `BSTTreapMtEph.rs` ×2; Chap41 `AVLTreeSetMtEph.rs`,
`AVLTreeSetMtPer.rs` ×2; Chap43 `OrderedSetMtEph.rs`, `OrderedTableMtPer.rs`;
Chap53 `PQMinStEph.rs`, `PQMinStPer.rs`.

The thirteen removed `accept()` calls are the other half of the same edit: the
`.finite()` conjunct went, and the `accept` that discharged it went with it. They
are in Chap37 `BSTAVLMtEph`, `BSTBBAlphaMtEph`, `BSTPlainMtEph`, `BSTRBMtEph`,
`BSTSetAVLMtEph`, `BSTSetBBAlphaMtEph`, `BSTSetPlainMtEph`, `BSTSetRBMtEph`,
`BSTSetSplayMtEph`; Chap39 `BSTTreapMtEph`; Chap41 `AVLTreeSetMtEph`; Chap43
`OrderedSetMtEph`, `OrderedTableMtEph`.

The seven `fn_missing_ensures` findings — Chap42 `TableMtEph`, `TableStEph`,
`TableStPer`; Chap43 `AugOrderedTableMtEph` ×2, `AugOrderedTableStEph`,
`AugOrderedTableStPer` — have no counterpart in `HashMigration.md`, and the April
log's only two were in `bugs/`. Whether the sources lost an `ensures` or the
classifier changed cannot be decided from the logs; the seven sites are named in the
per-file table above and are the first thing to read.

### The two Chap05 `external_body` `Hash` bodies

`src/Chap05/SetStEph.rs:967` and `src/Chap05/SetMtEph.rs:1235`. Veracity classifies
both as `structural_false_positive STD_TRAIT_IMPL hash` at high confidence: info
only, counted in neither the hole total nor the accepted total, in this run and in
the April baseline. `HashMigration.md` §5 item 2 ("Exec changes beyond rule 1"),
point 1, records that the body changed —
`self.elements.hash(state)` became a per-element `key.hash(state)` loop, because
`std::collections::HashSet` implements no `Hash` — and that the change moved the
wrapper's body rather than adding logic. The migration added no hole there.

`HashMigration.md` §5 item 3 states that the Chap05 `PartialEq` accepts could not be
closed through the new `HashSet::eq` specification and that no hole was added. The
count agrees: Chap05 `accept()` is 6 in both columns; its one new finding is the
`trivial_spec_wf` in `SetMtEph.rs`.

`docs/ChaptersInOrder.md` records 0 errors and 0 warnings for all 44 chapters,
including 49, 50, 42 and 43. Nothing here contradicts that: `dummy_rwlock_predicate`
and `fn_missing_ensures` are veracity's judgements about specification strength, not
Verus errors. A predicate that is `true` verifies; it just proves nothing.

## 7. What was run, and what did not run

| # | Script | Result |
|---|--------|--------|
| 1 | `scripts/holes.sh src/Chap02/` | ran; 2 modules, 0 holes, 5 accepted, 4 structural |
| 2 | `scripts/all-holes-by-chap.sh` | ran; 44 per-chapter logs written |
| 3 | top-level hole run | rerun by hand with `-e scratch -e bugs` (§3) |
| 4 | `scripts/chapter-cleanliness-status.sh` | failed, exit 127: `gawk` absent |
| 5 | `scripts/all-style-by-chap.sh` | ran; writes one log, not 44 (below) |
| 6 | whole-tree style run | rerun by hand over `src` to restore the full log |
| 7 | `scripts/all-fn-impls-by-chap.sh` | ran; 44 `.md` and `.json` pairs written |
| 8 | `scripts/validate.sh`, `rtt.sh`, `ptt.sh` | not run, by instruction |

`src/vstdplus/analyses/veracity-review-verus-proof-holes.log` is committed in its new
form: 0 modules. The hole tool was pointed at `src/vstdplus/` to test whether its
0-module result was a parse failure or an exclusion, and it is an exclusion (§3). The
log now records that veracity scans nothing there, which is the current fact about
the tool.

`chapter-cleanliness-status.sh` calls `gawk` and uses 3-argument `match()`; this
machine has `mawk` and `nawk` only, and there is no `sudo` to install `gawk`. Its
pipeline ends in `| tee "$OUT"`, so `tee` truncated
`analyses/chapter-cleanliness-status.log` to 0 bytes before `gawk` failed.
`analyses/chapter-cleanliness-status.log` was regenerated from the fresh hole log by
a `mawk` transcription of the same extraction, producing byte-identical structure to
the committed file: 46 chapters, 46 clean, 0 holed, 0 holes, 261 modules. The
veracity report now prints the same table itself, in section 4.6.

`all-style-by-chap.sh` runs `veracity-review-verus-style -c "$PROJECT_ROOT" "$dir"`
44 times; the tool writes to `<codebase>/analyses/veracity-review-verus-style.log`
and takes no output-path option, so each iteration overwrote the last and the file
left behind covered Chap66 alone. It was rerun once over `src` with
`-e src/experiments -e src/standards -e src/vstdplus`, which restores the
963 KB whole-tree log and its summary: 1,119 warnings over 277 files. The four
largest rules are `[24]` copyright-line placement (276), `[18]` definition order
inside `verus!` (163), `[22:spec]` free spec fn that should be a trait signature
(155) and `[22:exec]` free exec fn that should be a trait method (113). Style is
outside this round's scope; the log is committed for whoever takes it up.

## 8. Feedback for veracity's maintainer

Nine items, in `plans/r215-veracity-feedback.md`: no version stamp in the log;
silent `verus_syn` parse fallback; error-severity findings absent from the summary
count; the `Holes Found:` format change that broke a downstream parser; thread-join
`assume(false); diverge()` arms reported as `accept()`; and the undeclared hardcoded
exclusion of `Types.rs`, `lib.rs`, `vstdplus`, `standards` and `experiments`. Three
further items are APAS-VERUS script problems found in the same run. Nothing in
`~/projects/veracity` was changed.
