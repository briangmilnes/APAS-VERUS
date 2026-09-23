# r218 A report: `spinoff_prover` on the 8 full-crate failures

Start: `main` at `89abdb897`. Toolchain: Verus `0.2026.09.13.671956e`, Z3 4.16.0.

## Outcome

`#[verifier::spinoff_prover]` fixes 4 of the 8 failures (rows 1, 6, 7, 8).
For the other 4 (rows 2, 3, 4, 5), spinoff_prover makes the function fail
in isolate mode as well. Those attributes were removed, so those files are
unchanged. The full crate goes from 5608 verified, 8 errors to 5612 verified,
4 errors (`logs/validate.20260922-202527.log`). The plan's success criterion
of 0 full-crate errors is not met.

The source change is one attribute line on each of 4 functions. There are no
proof edits, rlimit changes, asserts, or holes.

## Per-site results

| # | Chap | File | Function | Before (full) | After (full) | spinoff kept |
|---|------|------|----------|---------------|--------------|--------------|
| 1 | 26 | ScanDCMtPer.rs | `prefix_sums_dc_inner` | rlimit | verifies | yes |
| 2 | 26 | ETSPMtEph.rs | `lemma_combined_cycle` | rlimit | rlimit | no (1) |
| 3 | 26 | ETSPMtEph.rs | `etsp_parallel_inner` | rlimit | rlimit | no (1) |
| 4 | 35 | OrderStatSelectMtEph.rs | `select_inner` | assert fails :553 | assert fails :553 | no (2) |
| 5 | 35 | OrderStatSelectMtPer.rs | `select_inner` | assert fails :551 | assert fails :551 | no (2) |
| 6 | 39 | BSTTreapStEph.rs | `join_with_priority_st` | precondition | verifies | yes |
| 7 | 65 | UnionFindArrayStEph.rs | `lemma_find_after_link` | rlimit | verifies | yes |
| 8 | 65 | UnionFindArrayStEph.rs | `lemma_link_preserves_wf` | rlimit | verifies | yes |

(1) ETSPMtEph.rs: three placements were tried under `isolate Chap26`. Each
one makes `etsp_parallel_inner` exceed its rlimit, and each result below is 1 error, 1098 verified:

| # | Chap | File | spinoff on | Isolate log | Failing fn |
|---|------|------|-----------|-------------|------------|
| 1 | 26 | ETSPMtEph.rs | both fns | validate.20260922-202228.log | etsp_parallel_inner |
| 2 | 26 | ETSPMtEph.rs | both fns (rerun) | validate.20260922-202254.log | etsp_parallel_inner |
| 3 | 26 | ETSPMtEph.rs | lemma_combined_cycle only | validate.20260922-202319.log | etsp_parallel_inner |
| 4 | 26 | ETSPMtEph.rs | etsp_parallel_inner only | validate.20260922-202342.log | etsp_parallel_inner |
| 5 | 26 | ETSPMtEph.rs | neither (baseline) | validate.20260922-202404.log | none (1099, 0) |

So `etsp_parallel_inner` exceeds its rlimit in its own Z3 process. It passes
in isolate only inside the shared module process with `lemma_combined_cycle`
before it. `lemma_combined_cycle` itself verifies when spun off (runs 1, 2).

(2) OrderStatSelect: with spinoff on both `select_inner` functions,
`isolate Chap35` reported 1223 verified, 2 errors. The errors were the same
assert-forall failures seen in the full crate, at OrderStatSelectMtEph.rs:554
and OrderStatSelectMtPer.rs:552 (shifted by the added line)
(`logs/validate.20260922-202429.log`). After the attributes were removed:
1225 verified, 0 errors (`logs/validate.20260922-202516.log`). A fresh Z3
process reproduces the full-crate failure deterministically. The isolate pass
therefore depends on Z3 state that earlier queries in the shared process
leave behind.

## Isolate results (final state)

| # | Chap | Log | Verified | Errors |
|---|------|-----|---------:|-------:|
| 1 | 26 | validate.20260922-202404.log | 1099 | 0 |
| 2 | 35 | validate.20260922-202516.log | 1225 | 0 |
| 3 | 39 | validate.20260922-202436.log | 1219 | 0 |
| 4 | 65 | validate.20260922-202445.log | 2531 | 0 |

None of these logs contains a warning or a trigger note.

## Full crate (final state)

`logs/validate.20260922-202527.log`: 5612 verified, 4 errors.

| # | Chap | File | Line | Error |
|---|------|------|-----:|-------|
| 1 | 35 | OrderStatSelectMtEph.rs | 553 | assertion failed |
| 2 | 35 | OrderStatSelectMtPer.rs | 551 | assertion failed |
| 3 | 26 | ETSPMtEph.rs | 223 | rlimit (`lemma_combined_cycle`) |
| 4 | 26 | ETSPMtEph.rs | 344 | rlimit (`etsp_parallel_inner`) |

The Chap45 `BalancedTreePQ.rs:712` failure did not appear in this run, so the
plan's single step-4 rerun was not needed for it.

## Profiling (step 4)

`scripts/profile.sh` on the full crate crashed twice
(`logs/profile-full-20260922-202658.log`, `logs/profile-full-20260922-202804.log`).
Both runs hit a Verus-internal panic, `rust_verify/src/profiler.rs:29:45:
called Option::unwrap() on a None value`, while analyzing the prover log for
Chap05 `MappingStEph`. The other worker threads then panicked with
`SendError`. The summaries (`logs/profile/SUMMARY-full-20260922-202658.txt`,
`...-202804.txt`) cover only Chap02–Chap06 modules and contain no data for
Chap26 or Chap35. No top-quantifier data is available for the 4 remaining
failures. An isolate profile would not help, because the failures do not
occur in isolate without spinoff. With spinoff they do occur in isolate, so a
profile of `isolate Chap26` or `isolate Chap35` with the attribute temporarily
applied is the next step that can produce data. It was not run here, because
the plan specifies a full-crate profile.

## RTT

`logs/rtt.20260922-202852.log`: 4328 tests run: 4326 passed, 2 failed, 0 skipped.
Both failures are on the known list: `TestBSTMtEph::mt_bbalpha_comprehensive_operations`
(BB[α] rebalancing not implemented) and `TestSpanTreeMtEph::test_spanning_tree_mt_two_vertices`
(nondeterministic Chap62 bug). The plan runs RTT only after a full-crate result
of 0 errors. I ran it anyway because the only change is attribute lines.

## PTT

`logs/ptt.20260922-202914.log`: the PTT library build still reports 5612
verified, 4 errors (same 4 as above). The test run then gives 328 tests run:
89 passed, 239 failed, 0 skipped. The failures are compile errors of the
form `cannot find Chap37 in apas_verus`, caused by the unverified library.
PTT therefore produced no meaningful result, as before.

## Next steps (not done here)

- Profile `isolate Chap26` and `isolate Chap35` with spinoff temporarily on
  the 4 remaining functions, where the failure reproduces deterministically.
- Report the `profiler.rs:29` panic to the Verus maintainers.
