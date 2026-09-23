# r218 B report: Z3 input, isolate vs full crate

Plan: `plans/r218-z3-query-diff-isolate-vs-full.md`. Evidence:
`analyses/r218-z3-query-diff/` (README there lists every file). Branch
`r218/z3-query-diff`, base commit 89abdb897, Verus 0.2026.09.13.671956e,
Z3 4.16.0.

## Result

For both targets, Verus sends Z3 the same SMT commands in the isolate and the
full-crate configuration. Nothing is added or removed. What differs is the
order in which the module prelude (declarations, function axioms, broadcast
axioms) is emitted. Replaying the logged files in a standalone Z3 reproduces
both outcomes, and permuting only the block order moves the failure from one
file to the other. The difference is therefore in Z3 session state, not in the
content of the target query: the prelude order, and the earlier queries run in
the same Z3 process, decide whether the target query succeeds.

Both targets are also unstable on their own terms: under Z3 random seeds the
isolate configuration fails for 2 of 8 seeds (Chap26) and 10 of 16 seeds
(Chap35). Isolate mode is not a more reliable configuration; it is a different
point in a noisy space.

## Targets

| # | Chap | File | Function | Isolate | Full |
|---|------|------|----------|---------|------|
| 1 | 26 | ScanDCMtPer.rs | `prefix_sums_dc_inner` | unsat, 4,888,736 rlimit | unknown, rlimit 30,000,000 hit |
| 2 | 35 | OrderStatSelectMtEph.rs | `select_inner` | unsat, 612,369 rlimit | unknown (incomplete quantifiers), 916,388 |

The rlimit figures are Z3's `:rlimit-count` delta across the target
check-sat, read from the transcripts. Verus's `--output-json` figures
differ (5,035,146 / 30,146,410 and 612,369 / 994,569); I did not trace how
Verus computes its per-function figure.

## Step 1-2: capture

`scripts/validate.sh` with `VERUS_EXTRA_ARGS="--verify-module <module> --log
smt-transcript --log smt --log-dir ... --time-expanded --output-json"`. Using
`--verify-module` rather than `--verify-function` keeps the module's whole Z3
session, so the transcript shows every query that precedes the target.
Outcomes under logging match the unlogged runs.

## Step 3: structural diff

| # | Chap | Measure | Isolate | Full |
|---|------|---------|---------|------|
| 1 | 26 | blocks | 215 | 215 |
| 2 | 26 | sorts / funs / consts | 14 / 196 / 1193 | 14 / 196 / 1193 |
| 3 | 26 | asserts / foralls | 542 / 381 | 542 / 381 |
| 4 | 26 | blocks only in one file | 0 | 0 |
| 5 | 26 | queries before target | 6 | 6 |
| 6 | 26 | rlimit counter before target | 259,575 | 259,539 |
| 7 | 35 | blocks | 601 | 601 |
| 8 | 35 | sorts / funs / consts | 9 / 320 / 795 | 9 / 320 / 795 |
| 9 | 35 | asserts / foralls | 1070 / 871 | 1070 / 871 |
| 10 | 35 | blocks only in one file | 0 | 0 |
| 11 | 35 | queries before target | 8 | 8 |
| 12 | 35 | rlimit counter before target | 1,821,882 | 1,716,089 |

The comparison normalizes names that depend only on emission order or on the
crate-wide closure counter: `anonymous_closure%556` (isolate) vs
`anonymous_closure%29498` (full), `impl_closure&__FnOnce697` vs
`FnOnce28769` in qids, `%%lambda%%N`, `%%global_location_label%%N`, span ids,
and line wrapping (the longer ids change where the printer breaks lines).
Chap35's full log was cleaned of Verus's post-failure error-localization
queries before comparison (`chap35/variants/full-clean.smt2`).

The only remaining difference is order: the sequence of block headers differs
by 32 diff lines in Chap26 and 124 in Chap35. The queries run
in the same order in both configurations (the `--output-json`
function-breakdown lists them in a different order, which is a reporting
order only).

Grouping the extra content by origin (trait impl axioms, broadcast groups,
datatype axioms, spec fn definitions) is empty for both targets: there is no
extra content.

## Step 4: reachability

Not applicable: no item is present in one configuration and absent in the
other. The moved blocks in Chap26 are vstd items the module already uses
(`seq::impl&%2::spec_add`, `skip`, `take`, `seq_lib::impl&%0::map`,
`drop_last`, `Seq::last`, the `add_empty_left/right` and
`push_distributes_over_add` broadcasts, `vec::impl&%43::push`,
`usize_specs::wrapping_add`, `HFSchedulerMtEph::join`,
`ArraySeqMtPerS::view` and a trait-impl axiom).

## Step 5: what decides the outcome

Replays in standalone Z3 (`*/replay/summary.tsv`):

| # | Chap | Variant | Target |
|---|------|---------|--------|
| 1 | 26 | isolate log as is | unsat 4,888,736 |
| 2 | 26 | full log as is | unknown 30,000,000 |
| 3 | 26 | full order, isolate closure names | unknown |
| 4 | 26 | isolate order, full closure names | unsat 4,888,736 |
| 5 | 26 | isolate content, full block order | unknown |
| 6 | 26 | full content, isolate block order | unsat 4,888,736 |
| 7 | 26 | isolate log, target query only | unknown |
| 8 | 26 | full log, target query only | unknown |
| 9 | 35 | isolate content, full block order | unknown |
| 10 | 35 | full content, isolate block order | unsat 612,369 |
| 11 | 35 | either log, target query only | unknown |

- Names are irrelevant (rows 3, 4). Block order alone decides (rows 5, 6, 9,
  10).
- Chap26 has one decisive move. Taking the first k blocks in full order and
  the rest in isolate order, k ≤ 71 passes and k ≥ 72 fails. The 72nd block is
  `Function-Axioms vstd::seq_lib::impl&%0::drop_last`; emitting `drop_last`'s
  spec and axioms before `Function-Axioms vstd::seq::impl&%2::spec_add` and
  the three `add`/`push` broadcasts turns a 4.9M-rlimit proof into a timeout.
- Chap35 has no single decisive move: the same scan flips five times.
- Earlier queries in the same Z3 process matter. With no earlier queries,
  both targets fail in both orders. Dropping subsets of earlier queries moves
  the Chap26 target between 4.9M and over 30M, and the full order with
  queries 2..7 passes at 17.0M (`*/replay/prior-query-subsets.tsv`). This
  matches r218 A: with `spinoff_prover`, `select_inner` fails even in isolate
  (reproduced here with `-V spinoff-all`, `chap35/iso-spinoff-mod/`).
- Seeds: isolate passes 6/8 and full 2/8 (Chap26); isolate 6/16 and full 5/16
  (Chap35) (`*/replay/random-seed.tsv`).

## Why the order differs (Verus source, read-only)

`verify_crate_inner` (`rust_verify/src/verifier.rs:2015`) builds one
`GlobalCtx` from the whole crate (`GlobalCtx::new(&krate, ...)` at line 2042).
`GlobalCtx::new` computes `func_call_sccs` with `sort_sccs()`
(`vir/src/context.rs:609`), a depth-first postorder over the whole-crate call
graph's SCCs taken in SCC-index order (`vir/src/scc.rs:176`). Each module
bucket is pruned (`verifier.rs:1930`) but SST construction
(`vir/src/ast_to_sst_crate.rs:33`) and the op generator
(`rust_verify/src/commands.rs:104`) walk `ctx.global.func_call_sccs`. So a
module's prelude order is the restriction of a topological order of the
entire crate: items that do not depend on each other (for example `drop_last`
and `spec_add`) are ordered by where the DFS over unrelated parts of the
crate first reaches them. Adding or removing unrelated chapters changes that
order. I did not instrument Verus to confirm that this is the only source of
the reordering; the observed permutation (the moved blocks keep their
content and only their relative position changes) is consistent with it.

## What I could not determine

- Which crate item makes the full-crate DFS reach `drop_last` before
  `spec_add`. `--log call-graph` on the full crate would show it; not run.
- Why that particular swap costs Z3 so much (no quantifier profile was taken
  for the replay files).
- The qid that is missing from `qid_map` in the profiler panic.

## Separate finding: profiler panic

`-V capture-profiles` panics in `rust_verify/src/profiler.rs:29:45`
(`called Option::unwrap() on a None value`, in
`write_instantiation_graph`). Main's full-crate log
`logs/profile-full-20260922-202658.log` names `MappingStEph` near the panic,
but with `--num-threads 1` the panic follows the `RelationStEph` analysis,
and `--verify-module Chap05::RelationStEph::RelationStEph` reproduces it alone
(`analyses/r218-z3-query-diff/profile-panic/`). `MappingStEph` alone does not
panic.

## Consequences for APAS-VERUS

- Isolate-vs-full disagreements are not evidence of missing or extra axioms.
  They are proof instability that the module's prelude order exposes.
- `spinoff_prover` changes the session state too; r218 A's four fixes and
  three regressions are consistent with the same phenomenon (not tested here
  beyond `select_inner`).
- The robust fix is in the proofs (fewer live quantifiers or explicit lemma
  calls in these functions), measured by passing under several
  `smt.random_seed` values, not by choosing a configuration.

## Files

- Evidence and README: `analyses/r218-z3-query-diff/`
- Draft issue for Verus: `plans/r218-verus-issue-draft.md`
