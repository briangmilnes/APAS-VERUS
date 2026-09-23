# r218 A: `spinoff_prover` on the 8 proofs that fail only in the full crate

Start: `main` at the commit that adds this plan (parent `50c0aeacf`).
Toolchain: Verus `0.2026.09.13.671956e`, Z3 4.16.0.

## Problem

Every chapter verifies with 0 errors under `scripts/validate.sh isolate ChapNN`
(`logs/validate.20260922-192100.log` … `192853.log`), but the full crate
(`logs/validate.20260922-192857.log`) reports 5608 verified, 8 errors. The same
8 have been present since before r217. One of them sometimes moves to Chap45
`BalancedTreePQ.rs:712`.

| # | Chap | File | Line | Item | Error |
|---|------|------|-----:|------|-------|
| 1 | 26 | ScanDCMtPer.rs | 119 | `fn prefix_sums_dc_inner` | rlimit |
| 2 | 26 | ETSPMtEph.rs | 223 | `proof fn lemma_combined_cycle` (has `rlimit(40)`) | rlimit |
| 3 | 26 | ETSPMtEph.rs | 344 | `fn etsp_parallel_inner` | rlimit |
| 4 | 35 | OrderStatSelectMtEph.rs | 553 | assert-forall inside an enclosing fn | assertion failed |
| 5 | 35 | OrderStatSelectMtPer.rs | 551 | assert-forall inside an enclosing fn | assertion failed |
| 6 | 39 | BSTTreapStEph.rs | 2115 | call site; precondition at line 2064 | precondition not satisfied |
| 7 | 65 | UnionFindArrayStEph.rs | 163 | `proof fn lemma_find_after_link` | rlimit |
| 8 | 65 | UnionFindArrayStEph.rs | 326 | `proof fn lemma_link_preserves_wf` | rlimit |

The Verus guide's checklist (`checklist.md`, "My proof is flaky") recommends
`#[verifier::spinoff_prover]`, which verifies the function in its own Z3
process instead of sharing one with the rest of its module.

## Steps

1. For each row, find the enclosing function (rows 4, 5, 6 give a line inside
   a function) and add `#[verifier::spinoff_prover]` to that function only. No
   other change: no rlimit change, no assert, no assume, no proof edits.
2. `scripts/validate.sh isolate` each touched chapter (26, 35, 39, 65): must
   stay 0 errors, 0 warnings, 0 trigger notes.
3. One full `scripts/validate.sh`. Record verified/error counts and the error
   list.
4. If the full crate still fails: add `spinoff_prover` to any newly surfaced
   function (e.g. Chap45 `BalancedTreePQ.rs:712`) and rerun once. For anything
   still failing, run `scripts/profile.sh` on the full crate (the failure only
   appears there), read `logs/profile/SUMMARY-*.txt`, and report the top
   quantifiers per function. Do not change proofs in this plan.
5. If the full crate verifies with 0 errors: `scripts/rtt.sh`, then
   `scripts/ptt.sh` (PTT could not run before because the library did not
   verify).
6. Report in `plans/r218-spinoff-prover-unstable-8-report.md` (tables with
   Chap column, log names). Commit with `git add -A`, push `main`.

## Success criteria

Full `scripts/validate.sh`: 0 errors. Each touched chapter isolate: 0 errors.
RTT: no failures beyond the two known (Chap62 `star_contract_mt`, Chap37
BB[α] rebalance). PTT: result reported.
