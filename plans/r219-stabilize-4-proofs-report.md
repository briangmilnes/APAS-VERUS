# r219 report: stabilize the 4 proofs that failed only in the full crate

Plan: `plans/r219-stabilize-4-proofs.md`. Branch `r219/stabilize-proofs`,
base `c28ce029f`. Toolchain: Verus `0.2026.09.13.671956e`, Z3 4.16.0.

## Outcome

All 4 functions verify in a fresh Z3 process, under 8 Z3 seeds each, and the
full crate verifies: 5620 verified, 0 errors (`logs/validate.20260923-124500.log`,
no warnings, no trigger notes). No `spinoff_prover` attribute is kept: the
full crate passes without it. No requires/ensures changed, no rlimit raised
(the one on `lemma_combined_cycle` was removed, which lowers it from 40 to
the default 10), no assume/admit/accept/external_body added, no exec code
changed.

## Per function

"Alone" means `--verify-only-module <m> --verify-function <f>`: the function's
query is the only query in its Z3 session, the fresh-process case. rlimit
figures are Verus `--output-json` per-function values.

| # | Chap | File | Function | Before, alone | After, alone | Seeds 1-8, after |
|---|------|------|----------|---------------|--------------|------------------|
| 1 | 26 | ETSPMtEph.rs | lemma_combined_cycle | pass, 3,240,197 | pass, 337,158 | 8/8, 293k-379k |
| 2 | 26 | ETSPMtEph.rs | etsp_parallel_inner | rlimit, 33,674,884 | pass, 825,439 | 8/8, 706k-877k |
| 3 | 35 | OrderStatSelectMtEph.rs | select_inner | fail, 959,363 | pass, 722,784 | 8/8, 658k-728k |
| 4 | 35 | OrderStatSelectMtPer.rs | select_inner | fail, 959,393 | pass, 722,783 | 8/8, 658k-728k |

Before-logs: rows 1-2 `validate.20260923-123719.log`, `-123726.log`; rows 3-4
`-123701.log`, `-123707.log` (run on the stashed base source).

## Profile findings

| # | Chap | File | Function | Top quantifier | Inst. | Log |
|---|------|------|----------|----------------|------:|-----|
| 1 | 26 | ETSPMtEph.rs | lemma_combined_cycle | spec_edges_form_cycle (:158), from requires on lt, rt | 1,951 of 2,158 | validate.20260923-124042.log |
| 2 | 26 | ETSPMtEph.rs | etsp_parallel_inner | spec_edges_form_cycle (:158), cost 552,782 each | 422 of 4,687 | validate.20260923-123742.log |
| 3 | 35 | OrderStatSelectMtEph.rs | select_inner | seq_lib to_multiset_contains (:3228) | 97 of 782 | validate.20260923-123312.log |

- Rows 1-2: the cycle predicate is an open spec fn whose quantifier has
  trigger `tour[i]`. Every sequence-index term in the body that is in the
  e-class of a tour (`lt[..]`, `rt[..]`, `combined@[..]`, the push terms)
  fires it, and each firing adds a `spec_next_edge_from` term with modular
  arithmetic. The profile of `etsp_parallel_inner` after the first change
  (hide only; `validate.20260923-123907.log`) still showed 6,532
  instantiations, all on the base-case paths that revealed the definition.
- Row 3 (row 4 is textually identical): 782 instantiations in total, so not a
  matching loop. Z3 returned unknown (incomplete quantifiers): the proof
  depended on Z3 finding the chain `sorted_left[j]` -> multiset count ->
  `left@.contains` -> witness index -> partition postcondition. The Verus
  profiler panics after printing the Chap26 statistics
  (`verifier.rs:716: Failed to find quantifier user_no_function_N`); the
  statistics printed before the panic are the ones above.

## Changes

Chap26 `ETSPMtEph.rs`:
- New `lemma_cycle_at(tour, i)` (elimination at one index) and
  `lemma_cycle_intro(tour)` (introduction), `lemma_two_edge_cycle`,
  `lemma_three_edge_cycle`.
- `lemma_combined_cycle`: `hide(spec_edges_form_cycle)`; each case calls
  `lemma_cycle_at` for the one `lt`/`rt` index it uses; ends with
  `lemma_cycle_intro(combined)`. `#[verifier::rlimit(40)]` removed.
- `etsp_parallel_inner`: `hide(spec_edges_form_cycle)`; the n == 2 and n == 3
  base cases call the two/three-edge lemmas instead of revealing the
  definition. The recursive case uses the predicate only as an atom.

Chap35 `OrderStatSelectMtEph.rs`, `OrderStatSelectMtPer.rs` (`select_inner`):
in both assert-forall blocks the multiset-count assert is replaced by the
explicit chain `sorted.contains(x)` -> `src.contains(x)` (from
`lemma_sort_by_ensures`'s no-new-elements clause) -> `choose` witness ->
`assert(T::le(..))`, which names the partition postcondition's trigger term.

## Fresh-process acceptance (plan step 3)

| # | Chap | Config | Log | Verified | Errors |
|---|------|--------|-----|---------:|-------:|
| 1 | 26 | isolate, spinoff_prover on both fns | validate.20260923-124351.log | 1103 | 0 |
| 2 | 35 | isolate, spinoff_prover on both fns | validate.20260923-124408.log | 1225 | 0 |
| 3 | 26 | isolate, no spinoff (final) | validate.20260923-124429.log | 1103 | 0 |
| 4 | 35 | isolate, no spinoff (final) | validate.20260923-124448.log | 1225 | 0 |
| 5 | all | full crate (final) | validate.20260923-124500.log | 5620 | 0 |

Chap26 has 1103 = 1099 + 4 new lemmas. No log above contains a warning or a
trigger note.

## RTT

`logs/rtt.20260923-124628.log`: 4328 tests run: 4326 passed, 2 failed,
0 skipped. Both failures are known: `TestBSTMtEph::mt_bbalpha_comprehensive_operations`
(BB[α] not implemented), `TestSpanTreeMtEph::test_spanning_tree_mt_two_vertices`
(nondeterministic Chap62 bug).

## PTT

The PTT library build (`verus --compile`) reports 5619 verified, 1 error:
rlimit in Chap45 `BalancedTreePQ.rs:712` `split` (`logs/ptt.20260923-124708.log`,
rerun `logs/ptt.20260923-124852.log`, same result). With the library
unverified, the test run is 328 tests: 3 passed, 325 failed (compile errors
`cannot find apas_verus`), so PTT gave no meaningful result.

This Chap45 failure is not caused by this change: the same PTT build on the
unmodified base (source stashed) reports 5612 verified, 4 errors, the three
targets plus the same `BalancedTreePQ.rs:712`
(`logs/ptt.20260923-125026.log`). It is the Chap45 instability r218 A
mentioned, and it appears in the `--compile` configuration while the
`validate.sh` full run passes. Chap45 was outside this round's files and was
not touched.

## Remaining

- Chap45 `BalancedTreePQ.rs` `split` (line 712): rlimit in the PTT
  `--compile` build; same treatment (profile alone, hide broad quantifiers)
  is the next target. PTT cannot give a result until it verifies.
