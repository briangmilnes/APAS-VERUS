# r218 B: what goes into Z3 for the same function, isolate vs full crate

Start: worktree `~/projects/APAS-VERUS-z3diff`, branch `r218/z3-query-diff`,
at the commit that adds this plan. It deliberately does NOT contain the r218 A
`spinoff_prover` attributes: the point is to measure the queries that fail.

## Question

`prefix_sums_dc_inner` (Chap26 `ScanDCMtPer.rs:119`) verifies under
`scripts/validate.sh isolate Chap26` and exceeds rlimit under the full
`scripts/validate.sh`. The source is identical. What differs in the SMT input
Verus sends to Z3 for that function, and where does each extra piece come
from? The answer goes back to the Verus developers, so it must be evidence,
not a guess.

## Steps

1. Build both configurations exactly as `scripts/validate.sh` does (read the
   script for the verus binary, flags, and the isolate `--cfg` list). Add
   Verus's SMT logging for one function: `--verify-function` for
   `prefix_sums_dc_inner` (module-qualified as Verus requires) plus the
   logging flags from `verus --help` (`--log-smt`, `--log-dir`, or
   `--log-all`; check which exist in this build). Do not use
   `spinoff_prover`.
2. Capture the SMT query for that function in both configurations. Confirm
   the isolate one verifies and the full one exceeds rlimit under the logged
   run too; if either outcome changes under logging, record that.
3. Diff the two queries structurally: counts of declared sorts, functions,
   axioms, and quantified formulas; then the list of declarations/axioms
   present only in the full-crate query, each traced back to its Rust source
   (crate module, item). Group them by origin (trait impl axioms, broadcast
   groups, datatype axioms, spec fn definitions, other).
4. Check whether the extra content is reachable from `prefix_sums_dc_inner`
   (call tree, trait bounds, `broadcast use`) or not. State which items, if
   any, Verus includes without a reachability path.
5. If step 3 shows no difference in the query, the difference is outside the
   query (Z3 process sharing, ordering, seeds): record that and compare
   `--time-expanded` / rlimit counts instead.
6. Record as an experiment per ExperAImentalThinking: committed inputs and
   outputs under `analyses/r218-z3-query-diff/`, a `README.md` with
   `RESULT:`, the evidence, `DATE:`, and toolchain versions. Keep query files
   (compress if large). Logs from every run committed.
7. Write `plans/r218-z3-query-diff-report.md` and a draft issue for the Verus
   developers at `plans/r218-verus-issue-draft.md` (do not file it). Then
   repeat steps 1–4 for the next of the 8 only if time allows.
8. Commit with `git add -A` on branch `r218/z3-query-diff`, push that branch.

## Constraints

Never modify `~/projects/verus/`. No proof changes. Run Verus one process at
a time.
