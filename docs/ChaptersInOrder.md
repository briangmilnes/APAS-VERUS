# Chapters in order (r213)

Plan: `plans/r213-chapters-in-order.md`. Start: `main` at `9f9485a2f`.
Toolchain: Verus `0.2026.09.13.671956e`, Z3 4.16.0, Rust 1.98.1.

Each chapter is run with `scripts/validate.sh isolate ChapNN` (log under
`logs/validate.*.log`), its run-time tests with `scratch/r212-rtt.sh ChapNN`
(`cargo test --release --no-fail-fast` on the chapter's `[[test]]` targets,
because `cargo-nextest` is not installed; log `logs/rtt.*.log`), and its
registered proof-time tests with `scratch/r212-ptt.sh ChapNN` (log
`logs/ptt-ChapNN.*.log`). Both scripts are copied unchanged from the r212
fixture. "Warnings" counts Verus and rustc warnings in the validate log;
"trigger notes" counts `automatically chose triggers` notes.

The cargo build of the library reports four rustc warnings outside the
chapter under test (`Chap43/OrderedTableStEph.rs` `cfg(never)`,
`Chap18/ArraySeqMtEph.rs` unneeded `mut`, two negative impls in
`vstdplus/threads_plus.rs`); they are handled with the chapter that owns the
file, or listed at the end.

## Summary

| # | Chap | Verified | Err | Warn | RTT | PTT | Commit |
|---|------|---------:|----:|-----:|-----|-----|--------|
| 1 | 02 | 631 | 0 | 0 | 41 pass | none | 1afd25de1 |
| 2 | 03 | 622 | 0 | 0 | 40 pass | none | 9da5c4aba |
| 3 | 05 | 760 | 0 | 0 | 89 pass | 29 pass (r214) | 7680a0ea8 |
| 4 | 06 | 1037 | 0 | 0 | 275 pass | 80 pass (r214) | 8c5300b01 |
| 5 | 11 | 651 | 0 | 0 | 40 pass | none | 58e3ecf69 |
| 6 | 12 | 635 | 0 | 0 | 40 pass | none | 571ed7e81 |
| 7 | 17 | 645 | 0 | 0 | 40 pass | 9 pass (r214) | 8c46d30bd |
| 8 | 18 | 1003 | 0 | 0 | 170 pass | 38 pass | 99d1a9fa7 |
| 9 | 19 | 824 | 0 | 0 | 156 pass | 23 pass | 7ac57e5c8 |
| 10 | 21 | 1262 | 0 | 0 | 46 pass | none | 0a048df27 |
| 11 | 23 | 679 | 0 | 0 | 92 pass | 17 pass | 8d3afb083 |
| 12 | 26 | 1098 | 0 | 0 | 59 pass | none | 3d9aac18c |
| 13 | 27 | 846 | 0 | 0 | 48 pass | none | 5369c431f |
| 14 | 28 | 883 | 0 | 0 | 68 pass | none | ba9524671 |
| 15 | 30 | 626 | 0 | 0 | none | none | a452bde90 |
| 16 | 35 | 1224 | 0 | 0 | 58 pass | none | 9a73d13bb |
| 17 | 36 | 867 | 0 | 0 | 24 pass | none | c54289512 |
| 18 | 37 | 1862 | 0 | 0 | 544 pass | 24 pass | 5c0513151 |
| 19 | 38 | 1078 | 0 | 0 | 53 pass | 2 pass | e1bc1eb32 |
| 20 | 39 | 1218 | 0 | 0 | 148 pass | 8 pass | ccd391de0 |
| 21 | 40 | 1180 | 0 | 0 | 54 pass | 6 pass | 9473aa308 |
| 22 | 41 | 2188 | 0 | 0 | 250 pass | 10 pass | 469798319 |
| 23 | 42 | 2312 | 0 | 0 | 66 pass | 10 pass | 6a0ee50c9 |
| 24 | 43 | 2686 | 0 | 0 | 279 pass | 14 pass | f7931d2de |
| 25 | 44 | 2336 | 0 | 0 | 45 pass | none | ff3727eeb |
| 26 | 45 | 2034 | 0 | 0 | 210 pass | none | 9cc03f02d |
| 27 | 47 | 1161 | 0 | 0 | 102 pass | none | 7521218a2 |
| 28 | 49 | 1283 | 0 | 0 | 136 pass | none | 52b78e66c |
| 29 | 50 | 766 | 0 | 0 | 167 pass | none | f55ba47bb |
| 30 | 51 | 1333 | 0 | 0 | 109 pass | none | 3d5951b8c |
| 31 | 52 | 2943 | 0 | 0 | 148 pass | none | 3c0a192b0 |
| 32 | 53 | 2246 | 0 | 0 | 46 pass | none | 09fcbc85c |
| 33 | 54 | 1277 | 0 | 0 | 53 pass | 8 pass | 4f592130b |
| 34 | 55 | 2290 | 0 | 0 | 58 pass | none | 41e46fbdb |
| 35 | 56 | 948 | 0 | 0 | 54 pass | none | 8b23fbabd |
| 36 | 57 | 2583 | 0 | 0 | 48 pass | none | d169e1d95 |
| 37 | 58 | 1370 | 0 | 0 | 41 pass | none | 936939b04 |
| 38 | 59 | 2632 | 0 | 0 | 41 pass | none | 4fd1c3ac8 |
| 39 | 61 | 1243 | 0 | 0 | 40 pass | none | a77dee8f1 |
| 40 | 62 | 1256 | 0 | 0 | 39 pass | none | b9b94c621 |
| 41 | 63 | 1271 | 0 | 0 | 40 pass | none | 09dea009a |
| 42 | 64 | 1271 | 0 | 0 | 24 pass | none | dee93dcbe |
| 43 | 65 | 2531 | 0 | 0 | 55 pass | none | ae84b55ac |
| 44 | 66 | 805 | 0 | 0 | 40 pass | none | 4aeec3055 |

Notes: the r213 PTT failures (Chap05, Chap17) and the unregistered Chap06
PTTs were on the pre-09.13 iterator model; r214 migrated them (see
"Follow-up (r214)" at the end).

## Chapter sections

### Chap02

- Start and end: 631 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050445.log`). No edit.
- RTT: 3 targets, 41 tests pass (`logs/rtt.20260922-050502.log`).
- PTT: none registered.

### Chap03

- Start and end: 622 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050550.log`). No edit.
- RTT: 1 target, 40 tests pass (`logs/rtt.20260922-050553.log`).
- PTT: none registered.

### Chap05

- Start and end: 760 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050624.log`). No edit; no `finite()` left in
  source.
- RTT: 5 targets, 89 tests pass (`logs/rtt.20260922-050632.log`).
- PTT (`logs/ptt-Chap05.20260922-050636.log`): 7 registered files, 5 of 21
  tests pass (`ProveKleeneStPer` 3, one each in `SetStEph.rs`,
  `SetMtEph.rs`); 16 fail to compile because they are still written on the
  pre-09.13 iterator model (`SetStEphIter`, `SetMtEphIter`,
  `RelationStEphIter`, `MappingStEphIter`, `iter_invariant`, `it@.0`). The
  iterator tool reports them as `ptt-no-definer` (the definers were migrated
  by hand in r208), so they need a hand rewrite to the templates of
  `src/standards/iterator_ptt_standard.rs`. Not done in this pass.

### Chap06

- Start and end: 1037 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050731.log`). No edit; no `finite()` left in
  source.
- RTT: 20 targets, 275 tests pass (`logs/rtt.20260922-050744.log`).
- PTT: the 20 files in `rust_verify_test/tests/Chap06/` are not registered
  in `rust_verify_test/Cargo.toml`, so the harness cannot run them; they are
  on the pre-09.13 iterator model. Not run.

### Chap11

- First run on 09.13: 651 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050801.log`). No edit.
- RTT: 2 targets, 40 tests pass (`logs/rtt.20260922-050809.log`).
- PTT: none registered.

### Chap12

- First run on 09.13: 635 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050809.log`). No edit.
- RTT: 3 targets, 40 tests pass (`logs/rtt.20260922-050812.log`).
- PTT: none registered.

### Chap17

- Start and end: 645 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050830.log`). No edit.
- RTT: 1 target, 40 tests pass (`logs/rtt.20260922-050837.log`).
- PTT (`logs/ptt-Chap17.20260922-050838.log`): 2 of 9 tests pass; the 7
  failures are the old iterator model (`MathSeqIter`, `iter_invariant`,
  `it@` on `vec::IntoIter`, `.pos`/`.elements`/`.cur` on the for-loop
  wrapper). Hand rewrite needed; not done in this pass.

### Chap18

- Start: 1003 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050849.log`); the cargo build reported one rustc
  warning in this chapter, `variable does not need to be mutable` for `left`
  at `ArraySeqMtEph.rs:1340`.
- Cause of the warning: in `ninject` of `src/Chap18/ArraySeqMtEph.rs`, the
  split loop read `if k < mid { right.push((pos, val)); }`, because a
  proof-minimisation run (commit `5f90be4bd`, R170, "veracity-minimize-proofs")
  had commented out the exec lines `left.push((pos, val));` and `} else {`
  under `// Veracity: UNNEEDED proof block` markers. The effect was that the
  first half of the updates went to the right-hand worker and the second half
  was dropped, and the left-hand worker received no updates. The weak
  `ninject` postcondition still verified, so nothing flagged it.
- Exec edit (one, restoring the r1xx text of commit `ce91629fc`):
  `if k < mid { left.push((pos, val)); } else { right.push((pos, val)); }`.
  Cost: the loop is Θ(|updates|) work and span before and after (one push per
  update); the parallel apply is unchanged. It restores the algorithm the
  `Alg Analysis` lines describe.
- End: 1003 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050912.log`).
- RTT: 8 targets, 170 tests pass (`logs/rtt.20260922-050921.log`); the
  unused-`mut` warning is gone from the build.
- PTT: 8 files, 38 tests pass (`logs/ptt-Chap18.20260922-050932.log`).

### Chap19

- Start and end: 824 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-051000.log`). No edit.
- RTT: 4 targets, 156 tests pass (`logs/rtt.20260922-051011.log`).
- PTT: 4 files, 23 tests pass (`logs/ptt-Chap19.20260922-051012.log`).

### Chap21

- Start (first run on 09.13): 1262 verified, 3 errors, 0 warnings
  (`logs/validate.20260922-051029.log`). All three were
  `assert_nonlinear_by: Resource limit (rlimit) exceeded` on
  `x % y == 0` facts under Z3 4.16: `Algorithm21_6.rs:52` and `:60`
  (`lemma_product_not_prime`), `Exercise21_9.rs:77`.
- Edit class: nonlinear `by (nonlinear_arith)` asserts replaced by the vstd
  lemma `vstd::arithmetic::div_mod::lemma_mod_multiples_basic(x, m)`
  (`(x * m) % m == 0`), plus `lemma_mul_is_commutative(a, b)` for the
  `(a * b) % a` case. Proof only; no exec edit.
- End: 1262 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-051054.log`).
- RTT: 1 target, 46 tests pass (`logs/rtt.20260922-051106.log`).
- PTT: none registered.

### Chap23

- Start and end: 679 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-051126.log`). No edit.
- RTT: 2 targets, 92 tests pass (`logs/rtt.20260922-051138.log`).
- PTT: 2 files, 17 tests pass (`logs/ptt-Chap23.20260922-051138.log`).

### Regression 1 (after Chap23)

`scratch/r213-regress.sh` over the ten chapters committed before Chap23; every
run 0 errors, 0 warnings, 0 trigger notes, same counts as committed:
Chap02 631 (`051146`), Chap03 622 (`051149`), Chap05 760 (`051151`), Chap06
1037 (`051155`), Chap11 651 (`051205`), Chap12 635 (`051207`), Chap17 645
(`051210`), Chap18 1003 (`051213`), Chap19 824 (`051218`), Chap21 1262
(`051222`); logs `logs/validate.20260922-<time>.log`.

### Chap26

- Start (first run on 09.13): 1097 verified, 1 error
  (`logs/validate.20260922-051243.log`): in `etsp_parallel_inner` of
  `src/Chap26/ETSPMtEph.rs`, `assert(c4)` (the cycle conjunct) failed in the
  `n == 3` base case, and the function body exceeded its rlimit. The base
  case used `reveal(spec_next_edge_from)`, which the spec's own comment names
  as a matching-loop risk.
- Edit class: proof only. The `n == 3` proof block now has the shape the
  StEph twin (`ETSPStEph.rs`) already verifies with: three element asserts,
  `lemma_next_edge_from_eq(tour@, i)` for i = 0, 1, 2, and the three
  `spec_point_eq` asserts, in place of the `reveal`. No rlimit change.
- End: 1098 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-051314.log`).
- RTT: 8 targets, 59 tests pass (`logs/rtt.20260922-051335.log`).
- PTT: none registered.

### Chap27

- First run on 09.13: 846 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-051353.log`). No edit.
- RTT: 4 targets, 48 tests pass (`logs/rtt.20260922-051408.log`).
- PTT: none registered.

### Chap28

- First run on 09.13: 883 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-051419.log`). No edit.
- RTT: 10 targets, 68 tests pass (`logs/rtt.20260922-051427.log`).
- PTT: none registered.

### Chap30

- First run on 09.13: 626 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-051446.log`). No edit.
- RTT: none registered. Because `scratch/r212-rtt.sh` with no target runs the
  whole suite, that run (`logs/rtt.20260922-051451.log`) is a full-suite
  measurement at this point: 265 targets, 4199 tests pass; the 12 failures
  are the pre-existing doc-test blocks of
  `src/standards/partial_eq_eq_clone_standard.rs` and `spec_wf_standard.rs`
  (Verus text compiled by rustdoc), as in r211 and r212.
- PTT: none registered.

### Chap35

- Start (first run on 09.13): 1222 verified, 2 errors
  (`logs/validate.20260922-051537.log`): `invariant not satisfied at end of
  loop body` for the `left` partition invariant
  (`forall j. T::le(left@[j], pivot) && left@[j] != pivot`) in the partition
  loop of `select` in `OrderStatSelectStEph.rs:180` and
  `OrderStatSelectStPer.rs:180`.
- First fix (commit `9a73d13bb`, `logs/validate.20260922-051639.log`, 1224
  verified, 0 errors): the two `cmp` facts in the `Less` arm plus a ghost
  snapshot and an `assert forall` after each `push` (26 proof lines per
  file).
- Final fix, replacing the first (committed with Chap36): the two partition
  invariants' trigger moved from the `T::le(..)` application to the vector
  index, `T::le(#[trigger] left@[j], pivot)` and
  `T::le(pivot, #[trigger] right@[j])`, the form `QuickSortStEph.rs` already
  verifies with. The quantified formula is unchanged; only its trigger
  differs. With the `T::le` trigger, a `push` introduces no `T::le` term for
  the new index, so Z3 had nothing to instantiate on. 4 lines changed per
  file, no added proof. 1224 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-051905.log`).
- RTT: 4 targets, 58 tests pass (`logs/rtt.20260922-051650.log`, rerun
  `logs/rtt.20260922-051922.log` after the final fix).
- PTT: none registered.

### Chap36

- Start (first run on 09.13): 864 verified, 3 errors
  (`logs/validate.20260922-051709.log`): `invariant not satisfied at end of
  loop body` for the `left` and `right` partition invariants in the three
  partition loops of `QuickSortMtEph.rs` (`quick_sort_first`,
  `quick_sort_median3`, `quick_sort_random`); the `recommendation not met`
  notes on `sort_by` accompany those errors and disappear with them.
- Edit class: trigger choice, as in Chap35. The six invariants now read
  `T::le(#[trigger] left@[j], pivot)` and `T::le(pivot, #[trigger] right@[j])`,
  the form `QuickSortStEph.rs` verifies with. An intermediate attempt with
  per-push `assert forall` blocks (`logs/validate.20260922-051733.log`) still
  failed and was removed.
- End: 867 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-051831.log`).
- RTT: 4 targets, 24 tests pass (`logs/rtt.20260922-051917.log`).
- PTT: none registered.

### Chap37

- First run on 09.13: 1862 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-051953.log`). No edit.
- RTT: 24 targets, 544 tests pass (`logs/rtt.20260922-052010.log`).
- PTT: 12 files, 24 tests pass (`logs/ptt-Chap37.20260922-052011.log`). `AVLTreeSeq.rs` `insert_at_link` keeps the `#[verifier::rlimit(20)]` r212 added.

### Chap38

- Start: 1079 verified, 0 errors, 111 warnings (all `use of deprecated
  method vstd::set::Set::finite`), 0 trigger notes
  (`logs/validate.20260922-052034.log`).
- Edit class 1, `finite()` removal (Edit tool, one text pattern per call):
  every `.finite()` conjunct deleted from `requires`, `ensures`, closure
  `ensures`, the `Exposed::Node` implications, the type invariant `wf` and
  `spec_bstpara{st,mt}eph_wf`, the `Node` lock-predicate, in
  `BSTParaStEph.rs` (50 sites) and `BSTParaMtEph.rs` (49 sites). Three
  `requires` clauses became empty and were deleted (`collect_in_order` in both
  files, `expose_internal` and `collect_in_order_inner` in the Mt file).
- Edit class 2, a finiteness-only fn: `assert_parambst_view_finite` in
  `BSTParaMtEph.rs`, whose one postcondition was `s@.finite()`, is commented
  out under a `// BYPASSED (r213):` note (not deleted); its two calls in
  Chap41 `AVLTreeSetMtPer.rs` (`assert_avltreesetmtper_always_wf`,
  `assert_avltreesetmtper_bounded_size`) are removed, because `cargo` builds
  the whole library for the run-time tests. Exec cost: each removed call was
  an O(1) ghost-only call; the callers stay O(1).
- Edit class 3, proof hints after the removal
  (`logs/validate.20260922-052342.log`: 1077 verified, 1 error, the
  `join_mid` precondition in `split_inner` of `BSTParaMtEph.rs`): the `Less`
  arm's ordering assert for `lr` and a new one for `rl` in the `Greater` arm
  are now `assert forall ... by { assert(left@.contains(t@)); }` (resp.
  `right@`), naming the subset step that the recursive call's `ensures`
  gives. Proof only.
- End: 1078 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-052403.log`); the count is one lower because the
  bypassed fn is no longer compiled.
- RTT: 2 targets, 53 tests pass (`logs/rtt.20260922-052434.log`).
- PTT: 1 file, 2 tests pass (`logs/ptt-Chap38.20260922-052440.log`).

### Chap39

- Start: 1221 verified, 0 errors, 138 warnings (deprecated `finite()`), 0
  trigger notes (`logs/validate.20260922-052508.log`).
- Edit class 1, `finite()` removal in `BSTParaTreapMtEph.rs` (51 sites),
  `BSTTreapStEph.rs` (45), `BSTSetTreapMtEph.rs` (23), `BSTTreapMtEph.rs`
  (11): conjuncts deleted from `requires`, `ensures`, closure `ensures`,
  `Exposed`/parts implications and the lock predicates; `&&& lv.finite() &&
  rv.finite()` deleted from `spec_param_wf_link`; four wf predicates whose
  only conjunct was `finite()` now read `true`
  (`spec_bstparatreapmteph_wf`, `spec_bstsettreapmteph_wf`,
  `spec_bsttreapmteph_wf` and the `BSTTreapMtEph` type invariant `wf`), as
  `src/standards/finite_sets_standard.rs` rule 1 prescribes.
- Edit class 2, finiteness-only proof steps: three fns whose only
  postcondition was `finite()` are commented out under `// BYPASSED (r213):`
  notes: `param_treap_assert_finite` (`BSTParaTreapMtEph.rs`; its two calls in
  `BSTSetTreapMtEph::join_m` removed; the commented block includes its
  `/// - Alg Analysis` line, kept verbatim inside the comment),
  `lemma_set_of_link_finite` (`BSTTreapMtEph.rs`, no caller) and
  `lemma_wf_implies_finite` (`BSTTreapStEph.rs`, four calls in
  `param_size`, `param_reduce`, `param_in_order` removed). Two
  `assert(old_view.finite())` and two `assert(left@/right@.finite())` deleted.
  One hole removed: `accept(self.ghost_locked_root@.finite())` in
  `BSTTreapMtEph::clone`, whose type invariant is now `true`.
- Exec cost: the two removed calls of `param_treap_assert_finite` were O(1)
  ghost-only calls; `join_m` stays O(lg n). No other exec change.
- End: 1218 verified (three fewer fns), 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-053007.log`). A first run after the removal
  stopped at 12 `E0425` errors from four `ensures` keywords the Edit tool had
  joined to the next token (`ensuresjoined@`), fixed before this run.
- RTT: 4 targets, 148 tests pass (`logs/rtt.20260922-053019.log`).
- PTT: 4 files, 8 tests pass (`logs/ptt-Chap39.20260922-053025.log`).

### Chap40

- Start and end: 1180 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-053048.log`). No edit.
- RTT: 3 targets, 54 tests pass (`logs/rtt.20260922-053058.log`).
- PTT: 3 files, 6 tests pass (`logs/ptt-Chap40.20260922-053058.log`).

### Chap41

- Start: did not compile, 15 errors and 77 warnings (deprecated `finite()`)
  (`logs/validate.20260922-053119.log`): `Set::new` now returns
  `Option<Set>` (`ArraySetEnumMtEph` view, `OrdKeyMap::spec_pair_set_to_map`),
  `Set::lemma_map_finite` is gone, and the old iterator model's `IntoIter@`
  no longer exists.
- Edit class 3, iterator migration: `iterator-upgrade --apply --into-iter c`
  over all six files (100 sites). `IntoIterator for &Self` impls whose
  `requires` cannot be checked are commented out (form C); loops use `.iter()`
  with a ghost `pos`. PTT loop/for-borrow-into tests on those impls are
  commented out with form-C notes (`ProveArraySetStEph.rs` by hand); the two
  generated `ProveOrdKeyMap.rs` iter tests had element type `u64` and now use
  `Pair<u64, u64>`. RTT callers `(&set).into_iter()` / `for v in &set` became
  `set.iter()` in `TestArraySetStEph.rs`, `TestAVLTreeSetStEph.rs`,
  `TestAVLTreeSetStPer.rs`.
- Edit class 4, finite-by-type API: `ArraySetEnumMtEph` view is now
  `Set::<usize>::range(0, universe_size).filter(...)`, and its loop
  invariants filter `self@`. `OrdKeyMap::spec_pair_set_to_map` is
  `Map::new(s.map(|p| p.0), |k| choose|v| s.contains((k, v)))`; a new
  broadcast lemma `lemma_pair_set_to_map_dom_contains` states its domain,
  used by fn-level `broadcast use` in three set-to-map lemmas and called in
  `lemma_map_contains_pair_in_set`. Small membership asserts added in
  `lemma_set_to_map_union_root`, `union_with`, `map_values` and the
  `ordkeymap_prev` root branch; explicit `#[trigger]` on two `requires`
  quantifiers.
- Edit class 1 and 2, `finite()` removal (about 80 sites) across
  `OrdKeyMap.rs`, `ArraySetStEph.rs`, `ArraySetEnumMtEph.rs` and the four
  `AVLTreeSet*` files. `spec_avltreesetmteph_wf` and both
  `spec_avltreesetmtper_wf` now read `true`; an empty `requires` on `find`
  deleted. BYPASSED (commented, not deleted): `lemma_bounded_usize_set_finite`,
  `lemma_view_finite` (`ArraySetEnumMtEph.rs`) and
  `lemma_pair_set_to_map_dom_finite` (`OrdKeyMap.rs`), with their calls
  removed; `lemma_pair_set_to_map_len` no longer requires `s.finite()`.
- Dependency fix: `LinkedListStPer::scan` (Chap18) hit its rlimit under
  `isolate Chap41`; its inline reveal is replaced by a call to a new free
  proof fn `lemma_take_fold_left_step` (r212 pattern), old lines kept as
  `// BYPASSED (r213):` comments. No rlimit raised.
- Exec cost: only O(1) ghost-only lemma calls removed; no Alg Analysis line
  changed, no exec statement changed.
- End: 2188 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-054937.log`), after seven intermediate runs.
- RTT: 8 targets, 250 tests pass (`logs/rtt.20260922-055208.log`).
- PTT: 6 files, 10 tests pass (`logs/ptt-Chap41.20260922-055210.log`).

### Chap42

- Start: did not compile (`logs/validate.20260922-055354.log`): the
  `TableMtEphTrait::iter` spec used the old-model `iter_invariant_tablemteph`,
  which no longer exists; 21 deprecated `finite()` sites.
- Edit class 3, iterator migration: `iterator-upgrade --apply --into-iter c`
  commented out `IntoIterator for &TableMtEph` (form C, it required
  `spec_tablemteph_wf`). The `iter` trait spec is now on the prophetic model:
  `remaining(&it) == into_iter_elts(it).as_ref()`,
  `spec_entries_to_map(into_iter_elts(it).map(view)) == self@` and
  `decrease(&it) is Some`; the impl proves the middle clause with one `=~=`
  assert. `ProveTableMtEph.rs` was rewritten on the new model (two tests; the
  two into-iter tests dropped as form C); the generated `ProveTableStEph.rs`
  and `ProveTableStPer.rs` had element type `u64` and now use
  `Pair<u64, u64>`.
- Edit class 1 and 2, `finite()` removal: `keys@.finite()` conjuncts deleted
  from `restrict`/`subtract` requires and loop invariants in all three table
  files; `result@.dom().finite()` deleted from the `collect_by_key` invariant
  (`TableStPer.rs`); the three `from_sorted_entries` fns lose their
  finite-only `ensures` and proof block. `lemma_entries_to_map_finite`
  (`TableSpecsAndLemmas.rs`) BYPASSED, its calls and one finite-only `assert
  ... by` removed. Chap52 still calls the lemma inside ghost code; it is
  fixed when Chap52 is reached (cargo builds do not see it).
- Rlimit: `TableMtEph::union` exceeded the default rlimit. The profile
  (`logs/validate.20260922-055536.log`) shows 61K instantiations and no
  matching loop (top: the quadratic `phase2_sources[j1] < phase2_sources[j2]`
  invariant, 27K). It fails at 20 and passes at 40; `#[verifier::rlimit(40)]`
  with a comment citing the profile.
- Exec cost: no exec statement changed; only ghost calls removed.
- End: 2312 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-055734.log`).
- RTT: 4 targets, 66 tests pass (`logs/rtt.20260922-055909.log`).
- PTT: 3 files, 10 tests pass (`logs/ptt-Chap42.20260922-055910.log`).

### Chap43

- Start: did not compile (`logs/validate.20260922-060637.log`, taken after
  the iterator and `finite()` edits; before them `Map::new(pred, f)` in
  `spec_pair_set_to_map`, 367 deprecated `finite()` sites and the removed
  `Set::lemma_map_finite` stopped the build).
- Edit class 3, iterator migration: `iterator-upgrade --apply --into-iter c`
  over the 12 files (110 sites): `OrderedSet*` and `OrderedTable*` `iter()`
  now return `std::vec::IntoIter` with the prophetic triple (plus the kept
  length conjunct), `IntoIterator for &Self` impls are form C. By hand:
  `OrderedTableMtEph::iter` returns `snapshot.into_iter()` (it built the
  old iterator struct over the same snapshot); the three `AugOrderedTable*`
  `iter` wrappers get the new signature and ensures, their `IntoIterator`
  impls form C; `OrderedTableMtPer`'s custom `OrderedTableMtPerIter` wrapper
  over the removed `OrderedTableStPerIter` is commented out (section 10b
  note) and `iter()` returns the St iterator. PTTs: the tool left
  `ProveOrderedSetStEph/StPer`, `ProveOrderedTableStEph/StPer/MtEph/MtPer`
  on the old model; their loop/for-borrow-iter tests were rewritten on the
  new model by hand and `ProveOrderedTableMtPer`'s two into-iter tests
  commented out as form C.
- Edit class 4, finite-by-type API: `spec_pair_set_to_map`
  (`OrderedSpecsAndLemmas.rs`) is now `Map::new(s.map(|p| p.0), ...)`, with
  the broadcast lemma `lemma_pair_set_to_map_dom_contains` (same as Chap41's)
  in the module-level `broadcast use` of `OrderedTableStEph.rs` and
  `OrderedTableStPer.rs`, fn-level in `lemma_set_to_map_insert` and
  `lemma_pair_set_to_map_len`, called in `lemma_map_contains_pair_in_set`.
  Four asserts earlier marked "Veracity: UNNEEDED" are restored
  (`OrderedTableStEph` restrict/subtract proofs, `OrderedTableStPer`
  subtract loop) and one added in `OrderedTableStPer::map`: without the
  definitional domain they now carry the `sorted@.contains` trigger.
  Explicit `#[trigger]` added on three quantifiers Verus now flags
  (`OrderedSetStEph`/`StPer` `to_seq` proofs,
  `lemma_sorted_keys_pairwise_distinct`).
- Edit class 1 and 2, `finite()` removal (all 367 sites): conjuncts deleted
  from `requires`/`ensures`/invariants; `ensures` clauses whose only
  conjunct was `finite()` deleted (about 60 trait and impl fns, mostly in
  the `AugOrderedTable*` files, whose trait specs were finite-only); empty
  `requires` of `AugOrderedTableMtEph::reduce_val` and its two closures
  deleted. Wf and type-invariant predicates: `spec_orderedtablemtper_wf` now
  `true`; `OrderedSetMtEph` type invariant `true`; finite conjunct dropped
  from `spec_orderedtablemteph_wf`, `spec_orderedsetmteph_wf`,
  `spec_augorderedtablemteph_wf` and the `OrderedTableMtPer` type invariant.
  BYPASSED: `lemma_pair_set_to_map_dom_finite`, 30 calls removed. Holes
  removed: three `assume(self@.dom().finite())` (`OrderedTableMtEph`
  `domain`, `reduce`, `collect`) and one `accept(view.dom().finite())`
  (`OrderedTableMtEph::clone`).
- Exec cost: `OrderedTableMtEph::iter` still copies the O(n) snapshot and
  now wraps it with `Vec::into_iter` (O(1)) instead of a struct literal. No
  other exec change; no Alg Analysis line changed.
- End: 2686 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-060943.log`).
- RTT: 11 targets, 279 tests pass (`logs/rtt.20260922-061245.log`).
- PTT: 7 files, 14 tests pass (`logs/ptt-Chap43.20260922-061251.log`).

### Chap44

- Start: did not compile (`logs/validate.20260922-061358.log`): `tokens`
  used the old-model `chars@` in its loop `decreases`; 5 deprecated
  `finite()` sites.
- Edit class 3: `iterator-upgrade` found no delegated site (one residual
  loop-with-break over `std::str::Chars`). By hand, the `tokens` loop now
  uses the prophetic model: invariant `obeys_prophetic_iter_laws(&chars)`
  and `decrease(&chars) is Some`, `decreases decrease(&chars)->0` (the old
  invariant was `true`).
- Edit class 1 and 2: two `gds.finite()` invariant conjuncts and two
  finite-only `assert(ds@.finite())` removed in `DocumentIndex.rs`.
- Dependency fix: `TableMtEph::union` (Chap42) exceeded its r213
  `rlimit(40)` under `isolate Chap44` (`logs/validate.20260922-061421.log`);
  raised to 60, which passed both isolates (see Chap42 for the profile).
- Exec cost: none changed.
- End: 2336 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-061449.log`).
- RTT: 2 targets, 45 tests pass (`logs/rtt.20260922-061517.log`).
- PTT: none registered.

### Chap45

- Start and end: 2034 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-061545.log`). No edit.
- RTT: 7 targets, 210 tests pass (`logs/rtt.20260922-061606.log`).
- PTT: none registered.

### Chap47

- Start: 1160 verified, 1 error, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-061622.log`): postcondition of
  `lemma_consecutive_even` (`QuadProbFlatHashTableStEph.rs`), a nonlinear
  `a * (a + 1) % 2 == 0` step Z3 4.16 no longer closes.
- Edit class 5, proof repair: three intermediate asserts after the existing
  vstd `div_mod` lemma calls (`(a % 2) * (a + 1) == 0`, `(a + 1) % 2 == 0`,
  `a * ((a + 1) % 2) == 0`). `iterator-upgrade` found no site; no
  `finite()` sites.
- Exec cost: none changed.
- End: 1161 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-061642.log`).
- RTT: 7 targets, 102 tests pass (`logs/rtt.20260922-061654.log`).
- PTT: none registered.

### Chap49

- Start and end: 1283 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-061718.log`). No edit.
- RTT: 8 targets, 136 tests pass (`logs/rtt.20260922-061730.log`).
- PTT: none registered.

### Chap50

- Start and end: 766 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-061737.log`). No edit.
- RTT: 9 targets, 167 tests pass (`logs/rtt.20260922-061745.log`).
- PTT: none registered.

### Chap51

- Start and end: 1333 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-061753.log`). No edit.
- RTT: 8 targets, 109 tests pass (`logs/rtt.20260922-061805.log`).
- PTT: none registered.

### Chap52

- Start: did not compile (`logs/validate.20260922-061845.log`, after the
  `finite()` edits): `Set::new` now returns `Option<Set>` in the four
  `EdgeSetGraph*` `spec_out_neighbors` bodies and in the
  `EdgeSetGraphMtPer::out_neighbors` spec and proof.
- Edit class 4, finite-by-type API: out-neighbour sets are now
  `edges@.filter(|p| p.0 == u).map_by(|p| p.1, |v| (u, v))`, the same set,
  whose `lemma_map_by_contains` (already broadcast through
  `group_set_lib_default`) gives membership without an existential. In
  `EdgeSetGraphMtPer`, the `out_neighbors` ensures states the same
  expression over `spec_edges()` and its proof compares against
  `spec_out_neighbors(u@)`.
- Edit class 1 and 2, `finite()` removal: the `when m.dom().finite()` clause
  of `spec_sum_adj_sizes` and the finite conjuncts of
  `lemma_sum_adj_remove`, `lemma_sum_adj_sizes_monotone`, an
  `AdjTableGraphMtPer` loop invariant and `spec_adjtablegraphmtper_wf`
  deleted; seven finite-only asserts deleted in `AdjTableGraphMtPer.rs`
  (one `assert ... by` kept as its inner `dom() =~=` assert); the six calls
  of Chap42's bypassed `lemma_entries_to_map_finite` and its import removed.
  `iterator-upgrade` found no site.
- Exec cost: none changed.
- End: 2943 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-061938.log`).
- RTT: 15 targets, 148 tests pass (`logs/rtt.20260922-062011.log`).
- PTT: none registered.

### Chap53

- Start: did not compile (`logs/validate.20260922-062118.log`, after the
  `finite()` edits): `PQMinStPer` called `Set::lemma_map_finite`, which
  vstd no longer has; before the edits, 45 deprecated `finite()` sites.
- Edit class 1 and 2, `finite()` removal: 43 `vertex_universe.finite()`
  conjuncts deleted from `requires` and loop invariants in the five files;
  `spec_pqminsteph_wf_generic` and its StPer twin, whose body was
  `s.visited@.finite() && s.priorities@.finite()`, now read `true`; the
  finite-only `frontier_updated@.lemma_map_finite(f)` call removed (the
  `lemma_map_size` and `lemma_len_subset` capacity proof after it is
  unchanged). `iterator-upgrade` found no site.
- Exec cost: none changed.
- End: 2246 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-062132.log`).
- RTT: 5 targets, 46 tests pass (`logs/rtt.20260922-062156.log`).
- PTT: none registered.

### Chap54

- Start and end: 1277 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-062218.log`). No source edit; no `finite()`
  sites; `iterator-upgrade` found no site.
- RTT: 4 targets, 53 tests pass (`logs/rtt.20260922-062241.log`).
- PTT: `ProveBFSStEph.rs` and `ProveBFSMtEph.rs` iterated the BFS orders
  with the removed `ArraySeqStEphIter`/`ArraySeqMtEphIter` and
  `iter_invariant` (8 failures, `logs/ptt-Chap54.20260922-062241.log`). The
  eight tests were rewritten by hand on the prophetic `std::slice::Iter`
  model (loop: `obeys_prophetic_iter_laws`, `remaining(&it).len()` count
  invariant, `decreases decrease(&it)->0`; for: `it.seq() ==
  orig.as_ref()`, `count == it.index()`), keeping each test's claim that
  the count equals `tree.order.spec_len()`. 2 files, 8 tests pass
  (`logs/ptt-Chap54.20260922-062353.log`).

### Chap55

- Start and end: 2290 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-062417.log`). No edit.
- RTT: 8 targets, 58 tests pass (`logs/rtt.20260922-062441.log`).
- PTT: none registered.

### Chap56

- Start and end: 948 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-062449.log`). No edit.
- RTT: 12 targets, 54 tests pass (`logs/rtt.20260922-062459.log`).
- PTT: none registered.

### Chap57

- Start: 6 deprecated `finite()` warnings (`visited@.finite()`,
  `used_edges.finite()` loop-invariant conjuncts in `DijkstraStEphF64.rs`
  and `DijkstraStEphU64.rs`); not run separately before the edit.
- Edit class 1: the six conjuncts deleted. `iterator-upgrade` found no site.
- Exec cost: none changed.
- End: 2583 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-062519.log`).
- RTT: 3 targets, 48 tests pass (`logs/rtt.20260922-062549.log`).
- PTT: none registered.

### Chap58

- Start: 1368 verified, 2 errors, 0 warnings (`logs/validate.20260922-062607.log`,
  reproduced in `logs/validate.20260922-062655.log`). Both errors are in
  the dependency Chap19, `ArraySeqStEph.rs`: the postconditions of `length`
  (`len == self.spec_len()`) and `nth` (`*nth_elem == self.spec_index(i)`)
  and the `Vec` index precondition in `nth`. They appear only in this
  dependency set (Chap02, 05, 06, 19, 56, 58, without Chap18); `isolate
  Chap19`, `Chap56` and `Chap57` verify the same bodies. The `group_vec_axioms`
  facts that connect `Vec::len` and `Vec` indexing to `self.seq@` do not fire
  here.
- Edit class 5, proof repair (Chap19 `ArraySeqStEph.rs`): `length` binds the
  result and asserts `len == self.seq@.len()` and
  `self.spec_len() == self.seq@.len()`; `nth` binds the reference and
  asserts `*nth_elem == self.seq@[index]` and
  `self.spec_index(index) == self.seq@[index]`. Exec behaviour and cost
  unchanged (a `let` binding instead of a tail expression).
- `iterator-upgrade` found no site; no `finite()` sites.
- End: 1370 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-062822.log`).
- RTT: 2 targets, 41 tests pass (`logs/rtt.20260922-062843.log`).
- PTT: none registered.

### Chap59

- Start and end: 2632 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-062907.log`). No edit.
- RTT: 4 targets, 41 tests pass (`logs/rtt.20260922-062939.log`).
- PTT: none registered.

### Chap61

- Start and end: 1243 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-062947.log`). No edit.
- RTT: 4 targets, 40 tests pass (`logs/rtt.20260922-063003.log`).
- PTT: none registered.

### Chap62

- Start: 1254 verified, 2 errors, 0 warnings
  (`logs/validate.20260922-063012.log`), both in `StarPartitionMtEph.rs`:
  - `hash_coin_flips_mt`: the merge loop's invariant "every key of `right`
    occurs in `it_seq`" was not established before the loop;
  - `build_p_vec_with_inject_mt`, `size == 1`: three postconditions at
    `return result` (the `Some(center)` branch).
  After those were fixed, a third failure surfaced
  (`logs/validate.20260922-063125.log`): `f2.requires(())` at the
  `ParaPair!` in `build_th_edges_mt`, with every conjunct of the requires
  proved separately ("the proof is flaky" note).
- Edit class 5, proof repair (all ghost code):
  - before the merge loop, an `assert forall` from the `HashMap::iter`
    postcondition (`remaining(&rit).contains((&k, &right@[k]))`, and
    `it_seq` is its `unref`), choosing the index and asserting
    `it_seq[i] == (k, right@[k])`;
  - in the `Some(center)` branch, asserts that `satellite_map@` contains the
    raw vertex, that `key_view(satellite_map@)` contains `sv` with value
    `*center`, and that `result@[0]@` is that value's view, which lets the
    requires quantifiers on satellite keys and values fire;
  - before the `ParaPair!` in `build_th_edges_mt`, an `assert forall ...
    by` for the edge-range requires of `f2` (one assert per conjunct), the
    two vertex-index requires, and `assert(f2.requires(()))`.
- `iterator-upgrade` found no site; no `finite()` sites.
- Exec cost: none changed.
- End: 1256 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-063225.log`).
- RTT: 4 targets, 39 tests pass (`logs/rtt.20260922-063242.log`).
- PTT: none registered.

### Chap63

- Start and end: 1271 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-063307.log`). No edit.
- RTT: 2 targets, 40 tests pass (`logs/rtt.20260922-063325.log`).
- PTT: none registered.

### Chap64

- Start and end: 1271 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-063333.log`). No edit.
- RTT: 3 targets, 24 tests pass (`logs/rtt.20260922-063354.log`).
- PTT: none registered.

### Chap65 (not clean: 4 functions left failing)

- Start: did not compile (`logs/validate.20260922-063518.log`, after the
  `finite()` edits): `KruskalStEph::mst_weight` used the old-model `it@` on
  `hash_set::Iter`; before the edits, `PrimStEph` also called the removed
  `Set::lemma_map_finite` and there were 43 deprecated `finite()` sites.
  Chap65 had not verified on 09.13 before this round.
- Edit class 1 and 2, `finite()` removal: the `.finite()` conjunct of
  `spec_size_rank_inv_map` (both union-find files) and of its seven
  `assert forall` restatements; `parent.dom().finite()` / `po.dom().finite()`
  / `pn.dom().finite()` requires and ensures conjuncts; `&&&
  parent.dom().finite()` in `spec_light_wf` (`UnionFindPCStEph.rs`); the
  finite-only asserts (`st_old*.finite()`, `po.dom().finite()`,
  `(su + sv).finite()` with its `by`, `su.finite() && ..` reduced to the
  length part); `DA.finite()`, `used_pairs.finite()` invariants and the two
  `lemma_map_finite` calls in `PrimStEph.rs`; one assert in
  `KruskalStEph.rs`.
- Edit class 3: the `mst_weight` loop now uses the prophetic model
  (`obeys_prophetic_iter_laws`, `decrease(&it) is Some`, `decreases
  decrease(&it)->0`).
- Edit class 5, proof repair in `KruskalStEph.rs`:
  `lemma_sorted_edge_in_graph_v` names the `LabGraphView` literal and asserts
  its `A` contains the edge triple, so the graph wf quantifier fires; the
  empty-union-find base case calls `lemma_key_view_len(uf.parent@)`; four
  `forall|x| labeled_view.contains(x) <==> mapped_es.contains(x)` quantifiers
  get an explicit `#[trigger]` on `mapped_es.contains(x)` (the trigger Verus
  had picked). Kruskal's three assertion failures and four trigger notes are
  gone.
- Left failing (all `function body check: Resource limit (rlimit)
  exceeded`, `logs/validate.20260922-064919.log`, 2522 verified, 4 errors):
  1. `UnionFindArrayStEph.rs:403` `UnionFindArray::union` (default rlimit).
     Profile (`logs/validate.20260922-063901.log`): 38,231 instantiations, no
     matching loop (top: a vstd `seq.rs` axiom, 17,815). Raised to 30 and to
     60, it still failed (`logs/validate.20260922-064407.log`,
     `logs/validate.20260922-064632.log`), so the attribute was taken out
     again: not an rlimit problem to be solved by raising it.
  2. `UnionFindNoPCStEph.rs:549` `UnionFind::union_sets` (existing
     `rlimit(30)`). Profile: 706,251 instantiations; the costliest quantifier
     is the domain-closure conjunct of `spec_uf_wf`,
     `forall|k| #[trigger] key_view(parent).dom().contains(k) ==>
     key_view(parent).dom().contains(pv(parent, k))`, whose conclusion
     re-matches its own trigger (a matching loop), with
     `lemma_key_view_contains` second (179,194). By rule, no rlimit raise.
     The same conjunct occurs in seven places across both union-find files;
     re-triggering it on `pv::<V>(parent, k)` is the likely fix, a
     cross-file proof change not attempted here.
  3. `UnionFindPCStEph.rs:509` `lemma_build_final_wf` (existing
     `rlimit(80)`). Its profile rerun ran the machine to 1.1 GB free (Z3
     19 GB) and the analysis did not finish, so no profile; not raised.
  4. `UnionFindPCStEph.rs:1096` `UnionFindPC::union` (existing `rlimit(40)`):
     passed in the profile run's first pass and failed in the other three
     runs; not profiled, not raised.
- Memory: every Chap65 run peaks near 14 GB of Z3 RSS.
- Exec cost: none changed.
- RTT: 5 targets build; they run 0 tests, because every file in
  `tests/Chap65/` starts with `#![cfg(feature = "all_chapters")]`
  (`logs/rtt.20260922-064856.log`).
- PTT: none registered.

### Chap66

- Start and end: 805 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-065209.log`). No edit.
- RTT: 2 targets, 40 tests pass (`logs/rtt.20260922-065219.log`).
- PTT: none registered.

## End of round

- Rlimits raised in r213: one, `TableMtEph::union` (`src/Chap42/TableMtEph.rs`),
  default to `rlimit(60)` after a profile with no matching loop (61K
  instantiations; 40 passed `isolate Chap42` but not `isolate Chap44`). The
  r212 `rlimit(20)` on Chap37 `insert_at_link` is unchanged. A trial
  `rlimit(30)`/`rlimit(60)` on Chap65 `UnionFindArray::union` did not help and
  was taken out.
- Functions left failing: four, all in Chap65, all rlimit (see the Chap65
  section): `UnionFindArray::union`, `UnionFind::union_sets` (NoPC; matching
  loop in the `spec_uf_wf` domain-closure conjunct), `lemma_build_final_wf`
  and `UnionFindPC::union` (PC).
- Later-chapter fixes in earlier chapters (each verified again by the next
  dependent isolate): Chap18 `LinkedListStPer::scan` (with Chap41), Chap19
  `ArraySeqStEph::length`/`nth` (with Chap58), Chap42 `TableMtEph::union`
  rlimit (with Chap44), Chap38 bypassed-lemma callers in Chap41.
- PTTs still on the old iterator model: Chap05 (16 tests) and Chap17 (7);
  Chap06's are not registered.

## Follow-up (r214)

### Chap65 (now clean)

- Trigger change (both union-find files, 34 quantifiers): every
  domain-closure quantifier `forall k. #[trigger] dom.contains(k) ==>
  dom.contains(pv(parent, k))` now also marks `pv(parent, k)` as a trigger,
  so the multi-pattern needs both terms and the conclusion no longer
  re-matches its own trigger. The specs are unchanged. With only this change
  (`logs/validate.20260922-065457.log`): `lemma_build_final_wf` verifies at
  its existing `rlimit(80)`, and the chapter's Z3 peak falls from about
  14 GB to 4.2 GB; three functions still fail.
- `UnionFindNoPCStEph::union_sets`:
  - Profile after the trigger change
    (`logs/validate.20260922-065634.log`): 520K instantiations, no loop.
  - New pure-map lemmas: `lemma_link_size_rank_inv` (size-rank invariant
    after linking root `ra` under root `rb`) and `lemma_link_preserves_inv`
    (every map-level invariant plus the find result for every element, with
    `pn = po.insert(ra, x)`; the ranks are either unchanged with
    `ro[ra] < ro[rb]`, or equal with `rb`'s rank growing by one). Each of
    the three branches now calls the lemma; the old inline proofs are kept
    as `/* r214: ... */` comments.
  - It still failed (profile `logs/validate.20260922-070523.log`: 955K
    instantiations, led by the `hash_specs_plus` key-view bridge lemmas,
    whose multi-triggers fire over every raw key). `group_key_view_lemmas`
    moved from the module's `broadcast use` to function-level `broadcast
    use` in `new`, `insert`, `find`, `equals` and `size`. `union_sets` calls
    `lemma_key_view_contains` (two rank reads), `lemma_key_view_insert`
    (three inserts) and `lemma_key_view_len` directly; `find`'s loop calls
    `lemma_key_view_contains(self.parent@, curr)` (the function-level
    `broadcast use` did not reach the loop body).
  - It now verifies at its existing `rlimit(30)`. A trial `rlimit(60)`
    was not needed and was taken out.
- `UnionFindPCStEph::union`: the same two lemmas, adapted to
  `spec_light_wf`, and the same branch rewrite. It verifies at its existing
  `rlimit(40)` with the module-level key-view group left in place.
- `UnionFindArrayStEph::union`: a `Seq<int>` version,
  `lemma_link_preserves_wf`, and the same branch rewrite. Default rlimit.
- Exec changes (none alter the cost): the inserted clones are bound to
  names (`ku`, `kv`, `root_u3`) before `insert`, so the proofs can name the
  value.
- Rlimits raised: none.
- End: 2531 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-071941.log`, 25 s, Z3 peak 358 MB).
- RTT: `#![cfg(feature = "all_chapters")]` was added to the Chap65 test
  files by `c1d126e82` ("Comment out Chap65 ... Z3 matching loop on union
  wf"), when Chap65 was dropped from the build. That reason no longer holds,
  so the gate is removed from all five files. 5 targets, 55 tests pass
  (`logs/rtt.20260922-072032.log`).

### Chap05 PTTs (now passing)

- The 16 failing tests used the pre-09.13 iterator model (`SetStEphIter`,
  `iter_invariant`, `it@`). Every Chap05 collection iterates with
  `std::collections::hash_set::Iter`, so each test was rewritten to the
  prophetic model: `orig = into_iter_hash_keys(it0)`; the `loop` form keeps
  `obeys_prophetic_iter_laws`, `remaining` and `decrease` in the invariant;
  the `for` form uses `it.seq().unref() == orig` and `it.index()`. Both
  collect the elements and prove `collected@ =~= orig`. The tests were
  generated by `scratch/r214/gen_hash_ptt.sh`.
- Rewritten: `ProveSetStEph.rs`, `ProveSetMtEph.rs` (4 each),
  `SetStEph.rs`, `SetMtEph.rs` (loop and for; `from_vec` kept),
  `RelationStEph.rs`, `MappingStEph.rs` (2 each).
- `ProveRelationStEph.rs` and `ProveMappingStEph.rs` were not registered in
  `rust_verify_test/Cargo.toml`; both are migrated and registered (4 each).
- Result: 9 files, 29 of 29 pass (`logs/ptt-Chap05.20260922-072425.log`).
  `scripts/validate-standard.sh ... ptt` covers only `src/standards/`, so the
  chapter PTTs ran through the real `rust_verify_test` harness with
  `scratch/r212-ptt.sh Chap05`.

### Chap17 PTTs (now passing)

- `ProveMathSeq.rs` (6 tests) used `MathSeqIter`, `iter_invariant` and `it@`.
  MathSeq delegates to `std::slice::Iter` and `std::vec::IntoIter` over
  `data`, so the file was rebuilt from the passing Chap18
  `ProveArraySeqStEph.rs` (the `iterator_ptt_standard.rs` templates, all six
  patterns) with `orig = a.data@`.
- `prove_MathSeq_iters.rs`: `mathseq_iter_range` used the old range wrapper
  field `iter.cur`; it now reads `it.index() <= len`.
- Result: 2 files, 9 of 9 pass (`logs/ptt-Chap17.20260922-072539.log`).

### Chap06 PTTs (registered, passing)

- The 20 files in `rust_verify_test/tests/Chap06/` are now registered in
  `rust_verify_test/Cargo.toml`.
- They used `SetStEphIter`, `iter_invariant`, `it@` and the old `for`-loop
  wrapper fields (`iter.elements`, `iter.pos`). They also called
  `g.iter_vertices()`, `g.iter_arcs()` and `g.iter_edges()`, which only
  `DirGraphStEph` still has.
- Migration (`scratch/r214/migrate_chap06_ptt.sh`):
  - Setup code, `requires` clauses and test names are copied unchanged.
  - Each loop is replaced with the `iterator_ptt_standard.rs` template over
    `hash_set::Iter`, with `orig = into_iter_hash_keys(it0)`.
  - `DirGraphStEph` keeps `iter_vertices()` and `iter_arcs()`.
  - The other graphs iterate through the accessor sets:
    `g.vertices().iter()`, `g.arcs().iter()` or `g.edges().iter()` for
    `Edge`, and `g.labeled_arcs().iter()` or `g.labeled_edges().iter()` for
    `LabEdge`.
  - The old trailing `assert(iter_seq.no_duplicates())` is not in the
    template and was dropped.
- Result: 20 files, 80 of 80 pass (`logs/ptt-Chap06.20260922-072731.log`).
