# r215 — feedback for the veracity maintainer

Date: 2026-09-22. Written from APAS-VERUS; nothing in `~/projects/veracity` was
edited. Tool built from `briangmilnes/veracity` `main` at
`f60414ce072e424e13eb9e6b0117c543cda090b7` (2026-05-24), Rust 1.91.0, `cargo
build --release`: 0 errors, 41 warnings.

The measurement this feedback comes from is `docs/HoleCount.md` in APAS-VERUS.
Items 1–6 are veracity observations; items 7–9 are APAS-VERUS script problems
recorded here only because they were found in the same run and they change what
veracity is asked to scan.

## 1. The output log carries no tool version

`analyses/veracity-review-verus-proof-holes.log` records the command line and
the run duration, but not which build of veracity produced it. The log is a
committed file that is diffed across months; without a version stamp a change in
the numbers cannot be attributed to the codebase or to the tool. `build.rs`
already emits `GIT_HASH`; printing it (and a `--version` flag, which the binary
does not accept today) in the log header would close this.

Concretely: the committed baseline log in APAS-VERUS was produced in April 2026
by an unknown veracity build, and this round's log by `f60414ce`. The comparison
in `docs/HoleCount.md` §5 therefore cannot separate codebase change from tool
change for any category whose definition may have moved.

## 2. A `verus_syn` parse failure is silent

`analyze_verus_block` (`src/bin/review_verus_proof_holes.rs:2964`) matches on
`verus_syn::parse_file(inner)` and, on `Err(_)`, calls
`analyze_verus_macro_tokens` with no diagnostic. A file whose `verus!` block the
Verus grammar cannot parse is then analysed by the token fallback and reported
in the ordinary tables, indistinguishable from a file that parsed. There is no
flag or log line that counts fallbacks.

This matters because `verus_syn` is a path dependency on
`../verus/dependencies/syn`, so the grammar tracks whatever Verus checkout is on
the machine. That is the right design — it is why `f60414ce`, pushed in May,
parses the 0.2026.09.13 sources — but it also means the parse surface changes
under the tool without notice. Suggested: count fallbacks and print
`file:line: warning: verus_syn parse failure, token fallback used`.

Measured effect today: a probe that runs `verus_syn::parse_file` over the same
`verus!` blocks veracity extracts reports 1 failure in 520 blocks across 530
files, in `src/experiments/generic_specs_to_prevent_cycles.rs:91`, which is an
experiment whose `RESULT` marker records that Verus itself rejects the
construct. Veracity excludes `experiments/`, so no scanned file falls back
today. The silence is still a latent measurement error.

## 3. Error-severity findings do not reach the headline count

Section 1 of the report prints lines such as

    src/Chap49/SubsetSumMtEph.rs:336: error: dummy_rwlock_predicate - ...
    src/Chap42/TableMtEph.rs:NNN: error: fn_missing_ensures

while section 3 prints `Holes Found: 0 (actionable)` and
`No proof holes found! All proofs are complete.` The only place the 11
error-severity findings appear in section 3 is the `Modules: 10 holed` line. A
reader who reads the summary and not section 1 will record zero.

Suggested: an `Errors: N total` block in section 3, broken down by kind, beside
`Warnings:` and `Accepted (reviewed):`, and a closing line that does not say
"All proofs are complete" while errors are present.

## 4. `Holes Found:` changed shape and broke a downstream consumer

`scripts/chapter-cleanliness-status.sh` in APAS-VERUS parses
`^Holes Found: ([0-9]+) total`. The tool now prints `Holes Found: 0
(actionable)`. The script's global-holes variable therefore stays 0 and it falls
back to summing the per-chapter figures. The sum happens to agree today, so no
wrong number was published, but the coupling is undeclared.

The same report now prints the whole cleanliness table itself in section 4.6,
which makes the shell script largely redundant. If section 4.6 is intended to
replace it, saying so in the README would let the project retire the script.

## 5. `assume(false); diverge()` thread-join arms are labelled `accept()`

Three Chap11 files contain no `accept` call at all, yet the report attributes
two `accept()` findings to each:

    src/Chap11/FibonacciMtEph2Threads.rs:173: info: accept()

Line 173 is
`Result::Err(_) => { proof { assume(false); }; diverge() } // accept hole: thread join error arm unreachable`.
The classification is driven by the trailing `// accept hole` comment, not by
the token. The finding is correct in substance — it is a reviewed hole — but the
kind name is wrong, and it inflates the `accept()` total by 6 over the number of
`accept` calls in the tree. A distinct kind (`thread_join_diverge`, or
`assume_accept_hole`) would keep `accept()` meaning `accept()`.

## 6. The hardcoded exclusion list hides real holes

`should_exclude` (`src/bin/review_verus_proof_holes.rs:787`) always excludes
`docs`, `path`, `src/lib.rs`, `src/Types.rs`, and any directory named
`experiments`, `vstdplus` or `standards`, independently of `-e`. There is no
flag to override it and the report does not say it happened.

In APAS-VERUS today the excluded scope holds:

| # | Scope | `admit()` | `assume(` | `external_body` | `unsafe impl` |
|---|-------|----------:|----------:|----------------:|--------------:|
| 1 | `src/Types.rs` | 9 | 0 | 0 | 0 |
| 2 | `src/vstdplus/` | 14 | 20 | 62 | 1 |
| 3 | `src/standards/` | 0 | 17 | 3 | 4 |

The nine in `src/Types.rs` are `broadcast proof fn ... { admit(); }` axioms for
`Pair`, `Edge`, `LabEdge` and `WeightedEdge` view injectivity, `obeys_feq_full`
and `obeys_key_model`. They are proof debt the crate-wide count reports as zero.
Suggested: keep the defaults, but list the excluded paths in the report header
and add a flag that includes them.

## 7. APAS-VERUS: `scripts/all-holes-by-chap.sh` scans working copies

The top-level invocation passes `$PROJECT_ROOT` with excludes for `benches`,
`tests`, `rust_verify_test`, `src/vstdplus`, `src/standards` and
`src/experiments`. It does not exclude `scratch/` or `bugs/`. On 2026-09-22
`scratch/srccopy/` held a 389-file copy of `src/`, so the unmodified script
reported 614 modules instead of 261, 596 accepted holes instead of 296, and
9 `admit()` holes that exist only in `scratch/srccopy/Types.rs`. Adding
`-e scratch -e bugs` restores the correct figures. This is an APAS-VERUS script
change, not a veracity change; it is recorded here because a maintainer reading
the inflated log would see it as a veracity defect.

## 8. APAS-VERUS: `scripts/chapter-cleanliness-status.sh` requires `gawk`

The script calls `gawk` and uses 3-argument `match()`. This machine has `mawk`
and `nawk` only, so the script exits 127. Worse, its pipeline ends in
`| tee "$OUT"`, and `tee` truncates `analyses/chapter-cleanliness-status.log` to
0 bytes before `gawk` fails, destroying the committed log even though the
analysis never ran. Redirecting to a temporary file and moving it on success
would prevent that.

## 9. APAS-VERUS: `scripts/all-style-by-chap.sh` writes one log, not 44

The loop runs `veracity-review-verus-style -c "$PROJECT_ROOT" "$dir"` once per
chapter. The tool writes to `<codebase>/analyses/veracity-review-verus-style.log`
and takes no output-path option, so each iteration overwrites the previous one
and the file left behind covers only the last chapter (Chap66, 2 files, 12
warnings). The script's comment and the `CLAUDE.md` analysis-script table both
say it produces a per-chapter log; it does not. Either the tool needs an output
path option or the script needs to copy the log per chapter, as
`all-fn-impls-by-chap.sh` already does.
