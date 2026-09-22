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
| 3 | 05 | 760 | 0 | 0 | 89 pass | 5 of 21 (1) | 7680a0ea8 |
| 4 | 06 | 1037 | 0 | 0 | 275 pass | not registered | 8c5300b01 |
| 5 | 11 | 651 | 0 | 0 | 40 pass | none | 58e3ecf69 |
| 6 | 12 | 635 | 0 | 0 | 40 pass | none | 571ed7e81 |
| 7 | 17 | 645 | 0 | 0 | 40 pass | 2 of 9 (1) | 8c46d30bd |
| 8 | 18 | 1003 | 0 | 0 | 170 pass | 38 pass | 99d1a9fa7 |
| 9 | 19 | 824 | 0 | 0 | 156 pass | 23 pass | 7ac57e5c8 |
| 10 | 21 | 1262 | 0 | 0 | 46 pass | none | 0a048df27 |
| 11 | 23 | 679 | 0 | 0 | 92 pass | 17 pass | 8d3afb083 |
| 12 | 26 | 1098 | 0 | 0 | 59 pass | none | r213 Chap26 |
| 13 | 27 | 846 | 0 | 0 | 48 pass | none | r213 Chap27 |
| 14 | 28 | 883 | 0 | 0 | 68 pass | none | r213 Chap28 |
| 15 | 30 | 626 | 0 | 0 | none | none | r213 Chap30 |
| 16 | 35 | 1224 | 0 | 0 | 58 pass | none | r213 Chap35 |
| 17 | 36 | 867 | 0 | 0 | 24 pass | none | r213 Chap36 |
| 18 | 37 | 1862 | 0 | 0 | 544 pass | 24 pass | r213 Chap37 |
| 19 | 38 | 1078 | 0 | 0 | 53 pass | 2 pass | r213 Chap38 |
| 20 | 39 | 1218 | 0 | 0 | 148 pass | 8 pass | r213 Chap39 |
| 21 | 40 | 1180 | 0 | 0 | 54 pass | 6 pass | r213 Chap40 |
| 22 | 41 | 2188 | 0 | 0 | 250 pass | 10 pass | r213 Chap41 |

Notes: (1) the failing proof-time tests are on the pre-09.13 iterator model
and do not compile; see the chapter section.

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
