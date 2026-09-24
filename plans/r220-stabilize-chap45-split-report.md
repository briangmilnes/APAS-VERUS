# r220 report: stabilize Chap45 BalancedTreePQ split

Plan: `plans/r220-stabilize-chap45-split.md`. Branch `r220/stabilize-chap45`,
base `57fd49fc0`. Toolchain: Verus `0.2026.09.13.671956e`, Z3 4.16.0.

## Outcome

`split` in `src/Chap45/BalancedTreePQ.rs` verifies alone under the default
seed and under Z3 seeds 1-8, with and without `spinoff_prover`. The full crate
verifies (5620 verified, 0 errors), and the PTT library build (`verus
--compile`), which failed on `split` before, now reports 5620 verified,
0 errors; all 328 PTT tests pass. No `spinoff_prover` attribute is kept: the
crate passes without it. No requires/ensures, exec code, or Alg Analysis
comment changed; no rlimit raised; no assume, admit, accept, or external_body
added. The change is one statement plus a comment.

## Per function

"Alone" means `--verify-only-module Chap45::BalancedTreePQ::BalancedTreePQ
--verify-function BalancedTreePQ::split` under `scripts/validate.sh isolate
Chap45`: the function's query is the only query in its Z3 session. rlimit
figures are Verus `--output-json --time-expanded` per-function values.

| # | Chap | File | Function | Before, alone | After, alone | Seeds 1-8, before | Seeds 1-8, after |
|---|------|------|----------|---------------|--------------|-------------------|------------------|
| 1 | 45 | BalancedTreePQ.rs | split | pass, 1,199,832 | pass, 838,080 | 7/8, 976k-30.0M | 8/8, 437k-737k |

Before, per seed (logs `validate.20260924-121643.log` .. `-121752.log`):

| # | Chap | Seed | Result | rlimit |
|---|------|-----:|--------|-------:|
| 1 | 45 | 1 | pass | 976,560 |
| 2 | 45 | 2 | rlimit exceeded | 30,000,000 |
| 3 | 45 | 3 | pass | 25,305,607 |
| 4 | 45 | 4 | pass | 1,001,479 |
| 5 | 45 | 5 | pass | 1,073,376 |
| 6 | 45 | 6 | pass | 1,094,943 |
| 7 | 45 | 7 | pass | 1,003,494 |
| 8 | 45 | 8 | pass | 992,301 |

After, per seed (logs `validate.20260924-121912.log` .. `-122026.log`;
default seed `-121902.log`):

| # | Chap | Seed | Result | rlimit |
|---|------|-----:|--------|-------:|
| 1 | 45 | 1 | pass | 556,277 |
| 2 | 45 | 2 | pass | 436,796 |
| 3 | 45 | 3 | pass | 614,309 |
| 4 | 45 | 4 | pass | 686,116 |
| 5 | 45 | 5 | pass | 592,646 |
| 6 | 45 | 6 | pass | 576,725 |
| 7 | 45 | 7 | pass | 736,878 |
| 8 | 45 | 8 | pass | 589,632 |

Before-figure alone at the default seed: `validate.20260924-121621.log`.

## Profile findings

| # | Chap | File | Function | Run | Top quantifier | Inst. total | Log |
|---|------|------|----------|-----|----------------|------------:|-----|
| 1 | 45 | BalancedTreePQ.rs | split | before, seed 2 (fails) | seq_lib push_distributes_over_add (:559), 392 inst., cost 82,175 each | 4,856 | validate.20260924-121807.log |
| 2 | 45 | BalancedTreePQ.rs | split | after, default seed (`--profile-all`) | seq_lib lemma_seq_empty_equality (:3505), 80 inst., cost 157 each | 315 | validate.20260924-122055.log |

Before, the next quantifiers were seq_lib `add_empty_right` (:550, 701 inst.),
`lemma_seq_empty_equality` (:3505, 1,085), seq `lemma_seq_add_len` (:1722,
738), seq `lemma_seq_push_len` (:1376, 660), seq_lib `add_empty_left` (:541,
486). 4,856 instantiations is not a matching loop; the cost is a few
quantifiers over `+` and `push` terms, one of which costs 82,175 per
instantiation (sequence extensionality on `(a + b).push(e)`).

Source of those terms: `split` states nothing about sequence contents, yet
every view in it (`self@`, `left@`, `right@`, and the `pq@` in each
`insert` postcondition, including its `to_multiset` clause) is
`spec_inorder(root)` from `src/Chap37/AVLTreeSeqStPer.rs`, an open recursive
spec fn whose body is `spec_inorder(l) + seq![v] + spec_inorder(r)`.
Unfolding each view term creates the `+`/`push` terms that trigger the
`group_seq_properties` broadcasts imported in section 3. Z3's cost therefore
depends on how many such unfoldings it tries before it finds the length
argument, which is the order- and seed-dependence r218 described.

## Change

Chap45 `BalancedTreePQ.rs`, `split`: `hide(spec_inorder);` as the first
statement, with a comment. The proof needs only lengths: `nth` requires
`i < self.elements.spec_seq().len()`, `insert` requires
`left@.len() + 1 < usize::MAX`, and the loop invariant and postcondition are
length equations. Each of these holds with `spec_inorder(root)` as an
uninterpreted term, supplied by `empty`/`insert`/`length` ensures and by
`lemma_size_lt_usize_max` and `lemma_size_eq_inorder_len`. No new lemma was
needed.

## Acceptance (plan steps 4-5)

| # | Chap | Config | Log | Verified | Errors |
|---|------|--------|-----|---------:|-------:|
| 1 | 45 | isolate, spinoff_prover on split | validate.20260924-122116.log | 2034 | 0 |
| 2 | 45 | isolate, no spinoff (final) | validate.20260924-122141.log | 2034 | 0 |
| 3 | all | full crate (final) | validate.20260924-122201.log | 5620 | 0 |
| 4 | all | PTT library build, `verus --compile` | ptt.20260924-122409.log | 5620 | 0 |

No log above contains a warning or a trigger note.

## RTT

`logs/rtt.20260924-122329.log`: 4328 tests run: 4326 passed, 2 failed,
0 skipped. Both failures are known and not targets:
`TestBSTMtEph::mt_bbalpha_comprehensive_operations` (BB[α] not implemented),
`TestSpanTreeMtEph::test_spanning_tree_mt_two_vertices` (nondeterministic
Chap62 bug).

## PTT

`logs/ptt.20260924-122409.log`: library build 5620 verified, 0 errors;
328 tests run: 328 passed, 0 skipped. This is the first PTT run with a
result since the r219 merge (`logs/ptt.20260924-121236.log` on main:
5619 verified, 1 error at `BalancedTreePQ.rs:712`, and the test run then
failed to compile).

## Plan step 6

The PTT `--compile` build exposed no other unstable function.

## Remaining

- `spinoff_prover` acceptance was run on the whole isolate chapter rather than
  combined with each seed; the seed runs were single-function runs, where the
  target query is alone in its Z3 session in any case.
- Other functions in `BalancedTreePQ.rs` (`insert`, `from_seq`, `filter`,
  `map`, `meld`) also carry `x@` view terms that unfold `spec_inorder`. They
  did not fail in any configuration measured in r219 or r220 and were not
  changed; if one becomes unstable, the same `hide(spec_inorder)` applies
  where the proof uses only lengths (not in `insert`, whose proof reasons
  about sequence contents).
