# Review of `src/vstdplus/` against vstd 0.2026.09.13

Date: 2026-09-20. Round r207, agent 2. Plan: `plans/r207-agent2-vstdplus-review.md`.
This is a review. No source file was edited. Every recommendation names the
concrete edit; none was applied.

Table conventions: every cell is at most 40 characters. An identifier longer
than that is cut with `…` in the cell and spelled out in the "Full names" list
under the table. The "vstd" column gives `file:line` in
`~/projects/verus/source/vstd/`; "—" means no equivalent. Numbers in
parentheses after an item are its users under `src/` outside `vstdplus/`.

## 0. Versions reviewed

| # | Chap | Input | Version |
|---|------|-------|---------|
| 1 | vstdplus | 27 files, 6,726 lines | HEAD `4bd6b4f92`, no local changes |
| 2 | vstdplus | `seq_set.rs` (agent 1 editing) | HEAD blob `b5d0e8e82`; see note |
| 3 | vstd | `~/projects/verus/source/vstd/` | tag `0.2026.09.13.671956e` |
| 4 | toolchain | `rust-toolchain.toml` | rustc 1.98.1 (2026-09-01) |
| 5 | log | `logs/validate.20260920-145749.log` | 168 errors, all name resolution |

Concurrent-edit note. When every vstdplus file was read, `git status
src/vstdplus/` printed nothing. At 15:10 local time agent 1's working copy
changed three files (uncommitted; `git diff --stat`: `seq_set.rs` 89 lines,
`hash_map_with_view_plus.rs` 2, `hash_set_with_view_plus.rs` 2). The diff
renames `group_set_axioms` to `group_set_lemmas` (11 sites) and
`group_map_axioms` to `group_map_lemmas` (1), and in the 13 fold lemmas of
`seq_set.rs` replaces `vstd::set::fold::lemma_fold_{empty,insert}` with
`vstd::iset::fold::lemma_fold_{empty,insert}(prefix.to_set().to_iset(), ...)`
and deletes the 13 `seq_to_set_is_finite` calls. The classification below is of
the HEAD version; the diff changes no item's class. Section 3.20 note 7 states
the two proof obligations the diff leaves open.

User counts were re-derived with `grep -rl 'vstdplus::<module>'` over `src/`
excluding `src/vstdplus/`, and separately over `tests/` and
`rust_verify_test/`. They match the plan's numbers; the plan omitted
`arithmetic/power2_plus` (1 user). Per-item counts are `grep -rlw <name>` over
the same trees; for names that are common words (`new`, `len`, `insert`,
`join`, `view`, `eq`) the count is noise and the item takes the module's class.

## 1. Summary

Classes: provided = vstd 09.13 has an item with the same statement; needed = no
vstd equivalent and a user under `src/` outside `experiments/`; unused = no user
under `src/` (users in experiments count as none when `lib.rs` comments the
experiment out, which is the case for every such experiment); broken =
references a vstd item that no longer exists, or a statement now vacuous.

| # | Chap | Module | Lines | Users src | Users test | Prov | Need | Unus | Brok | Action |
|---|------|--------|------:|----------:|-----------:|-----:|-----:|-----:|-----:|--------|
| 1 | vstdplus | accept.rs | 48 | 88 | 0 | 0 | 1 | 0 | 0 | keep; stub builds on 1.98.1 |
| 2 | vstdplus | arc_rwlock.rs | 71 | 18 | 0 | 3 | 0 | 0 | 1 | delete; use Arc::new, clone |
| 3 | vstdplus | arithmetic/power2_plus.rs | 83 | 1 | 0 | 0 | 2 | 4 | 0 | delete 4 lemmas |
| 4 | vstdplus | checked_int.rs | 593 | 7 | 12 | 4 | 23 | 15 | 0 | delete trait, 15 items |
| 5 | vstdplus | checked_nat.rs | 485 | 9 | 1 | 14 | 0 | 16 | 0 | delete; use vstd overflow.rs |
| 6 | vstdplus | clone_plus.rs | 114 | 67 | 0 | 0 | 5 | 1 | 0 | delete clone_pred2 |
| 7 | vstdplus | clone_view.rs | 139 | 29 | 0 | 0 | 10 | 0 | 0 | keep |
| 8 | vstdplus | feq.rs | 314 | 206 | 15 | 1 | 22 | 15 | 0 | delete ExFeq, 13 impls, 1 lemma |
| 9 | vstdplus | float.rs | 520 | 16 | 13 | 0 | 41 | 4 | 0 | delete 4 fns |
| 10 | vstdplus | hash_map_with_view_plus.rs | 303 | 38 | 9 | 8 | 5 | 0 | 9 | repair iterator (Phase 3) |
| 11 | vstdplus | hash_set_specs.rs | 37 | 0 | 0 | 0 | 0 | 1 | 0 | delete |
| 12 | vstdplus | hash_set_with_view_plus.rs | 300 | 26 | 1 | 5 | 6 | 0 | 11 | repair iterator; delete axiom |
| 13 | vstdplus | hashed_checked_u32.rs | 122 | 0 | 0 | 0 | 0 | 9 | 0 | delete |
| 14 | vstdplus | monoid.rs | 45 | 22 | 1 | 0 | 3 | 0 | 0 | keep |
| 15 | vstdplus | multiset.rs | 237 | 12 | 0 | 3 | 4 | 2 | 0 | delete 2 lemmas |
| 16 | vstdplus | partial_order.rs | 433 | 0 | 1 | 0 | 0 | 16 | 0 | delete module and RTT |
| 17 | vstdplus | pervasives_plus.rs | 62 | 2 | 0 | 0 | 2 | 0 | 0 | keep |
| 18 | vstdplus | rand.rs | 100 | 10 | 0 | 0 | 5 | 0 | 0 | keep |
| 19 | vstdplus | seq.rs | 286 | 4 | 0 | 0 | 4 | 11 | 0 | delete 11 sum items |
| 20 | vstdplus | seq_set.rs | 1260 | 30 | 0 | 12 | 45 | 15 | 12 | repair 12; delete 27; shrink 3 |
| 21 | vstdplus | smart_ptrs.rs | 67 | 19 | 0 | 0 | 4 | 0 | 0 | keep |
| 22 | vstdplus | sqrt.rs | 60 | 0 (2 implicit) | 0 | 0 | 1 | 5 | 0 | keep usize; delete 5 |
| 23 | vstdplus | strings.rs | 89 | 1 | 0 | 3 | 4 | 0 | 0 | delete 3 String wrappers |
| 24 | vstdplus | threads_plus.rs | 141 | 1 | 1 | 9 | 0 | 1 | 0 | delete; use vstd::thread |
| 25 | vstdplus | total_order.rs | 723 | 66 | 1 | 0 | 31 | 0 | 0 | keep |
| 26 | vstdplus | VecQueue.rs | 94 | 0 | 0 | 2 | 0 | 0 | 0 | delete; vstd VecDeque |
| 27 | — | totals | 6726 | — | — | 64 | 218 | 115 | 33 | see section 4 |

Reading of the totals: of 430 classified items, 64 have a vstd equivalent, 115
have no user, 33 no longer compile or are vacuous, and 218 remain needed. The
name-resolution errors in the log that fall inside `src/vstdplus/` are 53 of
168: 51 in `seq_set.rs` (11 `group_set_axioms`, 13 `lemma_fold_empty`, 13
`lemma_fold_insert`, 13 `seq_to_set_is_finite`, one of them on line 21), 1 in
`hash_set_with_view_plus.rs` (`group_set_axioms`), 1 in
`hash_map_with_view_plus.rs` (`group_map_axioms`).

Upstream changes from the plan's list with no vstdplus item affected: `BTreeMap`
specs (#2831), `NonZero` (#2471), `checked_next_multiple_of` (#2696),
`saturating_mul` (#2809), `Structural` for tuples and arrays (#2830). The
`no_unwind` clauses on integer arithmetic (#2893) change nothing in
`checked_int.rs` or `checked_nat.rs`, whose arithmetic bodies are `external_body`.

## 2. Method

For each module: read the file; list every `pub` item; for each item search
vstd for an item with the same statement (not the same name), reading the
candidate's `requires` and `ensures`; count users under `src/`, `tests/`,
`rust_verify_test/`; classify.

## 3. Per-module tables

### 3.1 `accept.rs` (48 lines; 88 users)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `accept` proof fn (`assume(b)`) | needed | — | keep (policy) |
| 2 | vstdplus | `cargo_accept::accept` stub | needed | — | keep |

Cargo stub under Rust 1.98.1. The stub (lines 40-48) is a plain `pub fn
accept(_b: bool) {}` behind `#[cfg(not(verus_keep_ghost))]`. A standalone copy
of lines 40-48 compiled with `rustc --edition 2021 --crate-type lib` under the
pinned toolchain: rustc 1.98.1, exit 0, zero warnings (scratchpad file
`accept_cargo_stub.rs`). `Cargo.toml` line 1442 declares
`check-cfg = ['cfg(verus_keep_ghost)', 'cfg(verus_keep_ghost_body)']`, so the
cfg raises no `unexpected_cfgs` warning. The other half of the question, whether
`verus!` erases the `proof fn` so that it does not collide with the `pub use`,
is decided by the crates.io dependency `vstd = "0.0.0-2025-08-12-1837"` in the
`[dependencies]` table of `Cargo.toml`, which the upgrade did not change. This
agent may not run `scripts/rtt.sh` and no `logs/rtt.*.log` exists after the
upgrade, so that half is unmeasured (section 5 item 3).

### 3.2 `arc_rwlock.rs` (71 lines; 18 users)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `new_arc_rwlock` (17) | provided | smart_ptrs.rs:41; rwlock.rs:502 | replace users |
| 2 | vstdplus | `clone_arc_rwlock` (12) | provided | vstdplus smart_ptrs.rs:29 | replace users |
| 3 | vstdplus | `clone_arc` (4) | broken | vstdplus smart_ptrs.rs:29 | replace users |

1. `Arc::<T>::new(t)` ensures `*v == t` (`std_specs/smart_ptrs.rs:41`) and
   `RwLock::new(val, Ghost(pred))` ensures `s.pred() == pred` (`rwlock.rs:502`);
   together they give `arc.pred() == pred`, which is `new_arc_rwlock`'s only
   postcondition. Replace each call `new_arc_rwlock(v, Ghost(p))` by
   `Arc::new(RwLock::new(v, Ghost(p)))`. Users: Chap50 `OptBinSearchTreeMtEph.rs`,
   `OptBinSearchTreeMtPer.rs`, `MatrixChainMtEph.rs`, `MatrixChainMtPer.rs`;
   Chap51 `TopDownDPMtEph.rs`, `TopDownDPMtPer.rs`; Chap62
   `StarPartitionMtEph.rs`; `standards/hfscheduler_standard.rs` (which has its
   own local copies); the rest are commented-out experiments.
2. `smart_ptrs.rs:29` (vstdplus) already specifies `<Arc<T,A> as Clone>::clone`
   with `ensures res == *a`, from which `cloned.pred() == arc.pred()` follows.
   Replace each `clone_arc_rwlock(&a)` by `a.clone()`.
3. `clone_arc<T>` ensures `*cloned == *arc` where `*cloned: T` and `*arc:
   Arc<T>`; under #2377 a spec `==` needs `SpecEq<Arc<T>> for T`, which
   `builtin/src/lib.rs:1172-1201` does not provide (plan r206 §2 row 11). The
   intended statement `cloned == *arc` is exactly the assume_specification in
   note 2. Replace each `clone_arc(&a)` by `a.clone()`.
4. After 1-3 the module has no items: delete `src/vstdplus/arc_rwlock.rs`, the
   line `pub mod arc_rwlock;` in `src/lib.rs`, and the `use
   crate::vstdplus::arc_rwlock::...` lines in the seven chapter files. The
   sentences naming `arc_rwlock.rs` in `standards/arc_usage_standard.rs` lines
   26 and 63 and in `standards/hfscheduler_standard.rs` then describe a file
   that no longer exists (agent 3's files).

### 3.3 `arithmetic/power2_plus.rs` (83 lines; 1 user: Chap11 `FibonacciStEph.rs`)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `lemma_pow2_mono` (2 sites) | needed | power2.rs:178 (strict only) | keep |
| 2 | vstdplus | `lemma_pow2_46_lt_u64_max` (2) | needed | power2.rs:270 (constant) | keep |
| 3 | vstdplus | `lemma_pow2_63_lt_u64_max` (0) | unused | — | delete |
| 4 | vstdplus | `lemma_pow2_lt_u64_max` (0) | unused | — | delete |
| 5 | vstdplus | `lemma_pow2_31_lt_u32_max` (0) | unused | — | delete |
| 6 | vstdplus | `lemma_pow2_lt_u32_max` (0) | unused | — | delete |

1. vstd has `lemma_pow2_strictly_increases` (power2.rs:178) and
   `lemma2_to64_rest` (power2.rs:270), not the non-strict or bounded forms.
   Delete lines 46-80 (items 3-6): 35 lines. Items 1-2 are also named in Chap02
   `FibonacciHFScheduler.rs`, which does not import this module (it declares
   local copies under the chapter-standalone rule).

### 3.4 `checked_int.rs` (593 lines; 7 users)

Users: 6 Chap06 `WeightedDirGraphStEphI*.rs`, 1 experiment; 12 RTT files
`tests/experiments/TestChecked*.rs`. vstd has no signed checked type
(`arithmetic/overflow.rs` is unsigned only), so the type family stays.
`std_specs/num.rs:404-460` specifies `iN::checked_add/sub/mul` with
`no_unwind`; the module's `external_body` arithmetic bodies call those, so the
24 trusted bodies (4 per type) could be verified instead of trusted, which is
proof work outside this review.

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `CheckedI8..CheckedIsize` (6 structs) | needed | — | keep |
| 2 | vstdplus | `new`, `new_out_of_range`, `spec_new` | needed | — | keep |
| 3 | vstdplus | `is_normal`, `is_out_of_range` + specs | needed | — | keep |
| 4 | vstdplus | `unwrap`, `to_option` | needed | — | keep |
| 5 | vstdplus | 6 `*_value` and `*_checked` methods | needed | num.rs:404-460 (bodies) | keep |
| 6 | vstdplus | `spec_min`, `spec_max` | needed | — | keep |
| 7 | vstdplus | `spec_is_overflowed/underflowed` | needed | — | keep |
| 8 | vstdplus | `CheckedIntTrait` (0) | unused | — | delete |
| 9 | vstdplus | module `spec_add`, `spec_mul` (0) | unused | — | delete |
| 10 | vstdplus | 5 module `spec_is_*` predicates (0) | unused | — | delete |
| 11 | vstdplus | `lemma_mul_commutative_ghost` (3) | provided | mul.rs (see note 1) | keep or inline |
| 12 | vstdplus | `lemma_mul_associative_ghost` (2) | provided | mul.rs | keep or inline |
| 13 | vstdplus | `lemma_mul_distributes_over_add_…` (2) | provided | mul.rs | keep or inline |
| 14 | vstdplus | `lemma_mul_distributes_over_sub_…` (0) | provided | mul.rs | delete |
| 15 | vstdplus | 2 `lemma_add_*_ghost` (2) | needed | — (linear arithmetic) | keep |
| 16 | vstdplus | `lemma_mul_commutative_normal` (1) | needed | — | keep |
| 17 | vstdplus | 2 `lemma_sub_anticommutative_*` (0) | unused | — | delete |
| 18 | vstdplus | 2 `lemma_add_*_normal` (0) | unused | — | delete |
| 19 | vstdplus | `lemma_mul_associative_normal` (0) | unused | — | delete |
| 20 | vstdplus | 2 `lemma_mul_distributes_*_normal` (0) | unused | — | delete |

Full names: row 13 `lemma_mul_distributes_over_add_ghost`; row 14
`lemma_mul_distributes_over_sub_ghost`.

1. Rows 11-14 restate `arithmetic/mul.rs` `lemma_mul_is_commutative`,
   `lemma_mul_is_associative`, `lemma_mul_is_distributive_add`,
   `lemma_mul_is_distributive_sub` on `a@`; the bodies already call them.
2. Delete lines 37-60 (rows 9-10), 65-115 (row 8), and inside
   `checked_int_gen!` the bodies of rows 14, 17-20 (lines 315-333, 346-359,
   368-377, 379-394) plus the `impl CheckedIntTrait` block (lines 397-483,
   which goes with row 8). About 135 source lines.
3. The `#[cfg(not(verus_keep_ghost))] pub mod checked_int` stub (lines 500-593)
   stays; the RTTs use it.

### 3.5 `checked_nat.rs` (485 lines; 9 users)

Users: 6 Chap06 `WeightedDirGraphStEphU*.rs`, 3 experiments; 1 RTT
(`tests/vstdplus/test_checked_nat_with_checked_view.rs`, see note 4). vstd
`arithmetic/overflow.rs` (since 2025-02-26, #1459; exported by
`arithmetic/mod.rs` under `cfg(not(verus_verify_core))`) defines `CheckedU8`,
`CheckedU16`, `CheckedU32`, `CheckedU64`, `CheckedU128`, `CheckedUsize` with
`View::V = nat`, and its header states it contains no trusted code. The
vstdplus family has `View::V = int` and 24 `external_body` arithmetic bodies.

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `CheckedU8..CheckedUsize` (6 structs) | provided | overflow.rs:302-307 | replace users |
| 2 | vstdplus | `new`, `spec_new` | provided | overflow.rs:111,119 | replace users |
| 3 | vstdplus | `new_overflow` | provided | overflow.rs:129 `new_overflowed` | rename at users |
| 4 | vstdplus | `is_overflow`, `spec_is_overflow` | provided | overflow.rs:140,148 `is_overflowed` | rename at users |
| 5 | vstdplus | `is_normal`, `spec_is_normal` | provided | `!is_overflowed()` | rewrite at users |
| 6 | vstdplus | `unwrap`, `to_option` | provided | overflow.rs:159,171 | replace users |
| 7 | vstdplus | `add_value`, `add_checked` | provided | overflow.rs:187,206 | replace users |
| 8 | vstdplus | `mul_value`, `mul_checked` | provided | overflow.rs:228,260 | replace users |
| 9 | vstdplus | `spec_max` (9 by name) | provided | `uN::MAX as nat` | rewrite at users |
| 10 | vstdplus | `CheckedNatTrait` (0) | unused | — | delete |
| 11 | vstdplus | 7 module spec fns (`spec_add`, …) | unused | — | delete |
| 12 | vstdplus | 3 `lemma_mul_*_ghost` | provided | mul.rs lemmas | delete |
| 13 | vstdplus | 2 `lemma_add_*_ghost`, 5 `*_normal` (0) | unused | — | delete |
| 14 | vstdplus | `lemma_sum_monotonic` (0) | unused | — | delete |
| 15 | vstdplus | `lemma_add_normal_if_sum_fits` (0) | unused | — | delete |
| 16 | vstdplus | `lemma_partial_sum_bounded` (0) | unused | — | delete |

1. In the six Chap06 `WeightedDirGraphStEphU*.rs` files, `src/vstdplus/seq.rs`
   line 23, and `experiments/{HashCheckedU32,seq_set_exec,checked_comm}.rs`:
   replace `use crate::vstdplus::checked_nat::checked_nat::*` by `use
   vstd::arithmetic::overflow::*`; rename `is_overflow()` to `is_overflowed()`,
   `new_overflow` to `new_overflowed`, `is_normal()` to `!is_overflowed()`.
2. The view type changes from `int` to `nat`. In `WeightedDirGraphStEphU32.rs`
   line 359 the loop invariant is `sum@ == ...fold_left(0int, |acc: int, e|
   acc + e@.2 as nat)`; with `sum@: nat` it becomes `fold_left(0nat, |acc: nat,
   e| acc + e@.2 as nat)`, which is `spec_weighted_seq_sum` of the mapped
   sequence directly. That removes the need for the six
   `lemma_fold_left_int_equals_nat_as_int*` bridges in `seq_set.rs` (3.20 row
   30).
3. Delete `src/vstdplus/checked_nat.rs`, the `pub mod checked_nat;` line in
   `src/lib.rs`, and `src/vstdplus/hashed_checked_u32.rs` (3.13: it implements
   `Hash`, `PartialEq`, `Eq`, `Display`, `Debug` for the local `CheckedU32`;
   for vstd's type those impls violate the orphan rule, and they have no user).
   485 + 122 lines.
4. The RTT imports `apas_verus::vstdplus::checked_nat_with_checked_view`, a
   module that exists neither in `src/vstdplus/` nor in `src/lib.rs`; the test
   cannot have compiled since that module was renamed. Rewrite it against
   `vstd::arithmetic::overflow::CheckedU32` or delete it.
5. Trust change: 24 `external_body` bodies (4 per type) are replaced by vstd
   bodies that verus verifies; `scripts/holes.sh src/vstdplus/` should report
   24 fewer `external_body` entries.

### 3.6 `clone_plus.rs` (114 lines; 67 users)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `ClonePlus::clone_plus` (21) | needed | — (section 5 item 1) | keep pending experiment |
| 2 | vstdplus | `clone_fn` (10) | needed | — | keep |
| 3 | vstdplus | `clone_fn2` (11) | needed | — | keep |
| 4 | vstdplus | `clone_pred` (7) | needed | — | keep |
| 5 | vstdplus | `clone_fn_usize` (3) | needed | — | keep |
| 6 | vstdplus | `clone_pred2` (0) | unused | — | delete |

1. vstd's `ExClone::clone` (`std_specs/clone.rs:6-11`) declares no `ensures`,
   so nothing in vstd states `cloned(*self, res)` for a generic `T: Clone`.
   Whether a plain `x.clone()` already yields it is section 5 item 1.
2. Rows 2-5 stay because verus does not recognise `Clone` on closure types
   (`experiments/closure_clone_first_class.rs`, RESULT: FAILS at 03.28; not
   rerun at 09.13, section 5 item 2).
3. Delete `clone_pred2` at lines 86-94 and its stub at line 113: 10 lines.

### 3.7 `clone_view.rs` (139 lines; 29 users)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `ClonePreservesView` (18) | needed | — | keep |
| 2 | vstdplus | `ClonePreservesWf` (7) | needed | — | keep |
| 3 | vstdplus | 8 primitive `clone_view` impls | needed | clone.rs:26-50 (bool, char only) | keep |

No vstd trait carries `clone()@ == self@` as a bound. `laws_eq::obeys_view_eq`
(`laws_eq.rs:35`) relates `eq_spec` to views, not `clone`. vstd specifies
`clone` for `bool`, `char`, `&T`, and `[T; N]` (`clone.rs:26-50`), not for the
integer types.

### 3.8 `feq.rs` (314 lines; 206 users; 15 PTT users)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `feq_reflexive/symmetric/transitive` | needed | laws_eq.rs:11 (no reflexive) | keep |
| 2 | vstdplus | `obeys_feq_properties` | needed | laws_eq.rs:11 (partial) | keep |
| 3 | vstdplus | `obeys_feq_view` | needed | laws_eq.rs:35 (one direction) | keep |
| 4 | vstdplus | `obeys_feq_clone` (92) | needed | — | keep |
| 5 | vstdplus | `obeys_feq_eq` (1) | provided | laws_eq.rs:29 `obeys_concrete_eq` | keep (internal) |
| 6 | vstdplus | `obeys_feq_view_injective` (11) | needed | — | keep |
| 7 | vstdplus | `obeys_feq_full` (47) | needed | — | keep |
| 8 | vstdplus | `obeys_feq_fulls` (13) | needed | — | keep |
| 9 | vstdplus | `obeys_feq_full_trigger` (40) | needed | — | keep |
| 10 | vstdplus | `obeys_view_eq_trigger` (7) | needed | — | keep |
| 11 | vstdplus | `lemma_cloned_view_eq` (22) | needed | — | keep |
| 12 | vstdplus | `lemma_reveal_view_injective` (26) | needed | — | keep |
| 13 | vstdplus | `lemma_view_injective` (0) | unused | relations.rs:11 `injective` | delete |
| 14 | vstdplus | `axiom_cloned_implies_eq` (8) | needed | — | keep (admit, policy) |
| 15 | vstdplus | `axiom_cloned_implies_eq_owned` (25) | needed | — | keep (admit, policy) |
| 16 | vstdplus | 2 `axiom_strictly_cloned_implies_eq*` | needed | trigger variants of 14-15 | keep |
| 17 | vstdplus | `lemma_seq_map_cloned_view_eq` (7) | needed | — | keep |
| 18 | vstdplus | `axiom_obeys_feq_full` (admit) | needed | laws_eq.rs:376 (eq conjuncts) | keep |
| 19 | vstdplus | `axiom_obeys_view_eq` (admit) | needed | laws_eq.rs:76 (primitives) | keep |
| 20 | vstdplus | `group_feq_axioms` | needed | — | keep |
| 21 | vstdplus | `ExFeq` + `FeqSpec` extension | unused | cmp.rs:31 `ExEq` | delete |
| 22 | vstdplus | 13 `FeqSpecImpl` impls (`obeys_feq`) | unused | — | delete |
| 23 | vstdplus | `feq` exec fn (208) | needed | — | keep (assume, policy) |
| 24 | vstdplus | cargo stub `feq`, `obeys_feq_clone` | needed | — | keep |

1. `grep -rw 'obeys_feq()'` over `src/`, `tests/`, `rust_verify_test/` finds
   no call outside `feq.rs`; `obeys_feq_full` is built from
   `obeys_feq_properties`, not from the trait method. Delete lines 210-285
   (rows 21-22, 76 lines) and the import `core::marker::PointeeSized` (line
   28). After that, `src/lib.rs` line 8
   `#![cfg_attr(verus_keep_ghost, feature(sized_hierarchy))]` and its comment
   (lines 5-6) have no remaining reason; `smart_ptrs.rs:29` still needs
   `allocator_api` (line 7).
2. Delete `lemma_view_injective` (lines 120-131, 12 lines).
3. vstd's `ExEq` (cmp.rs:31) and vstdplus's `ExFeq` are two external trait
   specifications for `core::cmp::Eq`. `ExEq` was merged 2025-07-09 (#1569), before
   04.20, so the pair coexisted; deleting `ExFeq` removes the question.
4. For `bool` and the 12 integer types, vstd `group_laws_eq` (`laws_eq.rs:376`)
   proves `obeys_eq`, `obeys_concrete_eq`, `obeys_view_eq` without `admit`. The
   `cloned` conjunct of `obeys_feq_full` has no vstd source for integers
   (`clone.rs` specifies `bool` and `char` clones only), so
   `axiom_obeys_feq_full` cannot be discharged for integers from vstd alone
   (section 5 item 4). No edit now.

### 3.9 `float.rs` (520 lines; 16 users; 13 RTT files)

vstd `float.rs` at 09.13 (3,682 bytes) contains `FloatBitsProperties` (which
this module imports), `f32`/`f64` `Clone` specs, and `IeeeFloatCast`. It has no
ordering or arithmetic axioms; `std_specs/cmp.rs:272` keeps `le_ensures`
uninterpreted by design (comment at cmp.rs:256-262).

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | 10 `axiom_f64/f32_*` order axioms | needed | — | keep |
| 2 | vstdplus | `group_float_finite_total_order` | needed | — | keep |
| 3 | vstdplus | `FloatTotalOrder` + f64, f32 impls | needed | — | keep |
| 4 | vstdplus | `all_float_wf` (experiment only) | unused | — | delete |
| 5 | vstdplus | `spec_float_sorted` (0) | unused | relations.rs:88 `sorted_by` | delete |
| 6 | vstdplus | `WrappedF64` + View + clone_view impl | needed | — | keep |
| 7 | vstdplus | `spec_is_finite`, `is_finite`, `eq` | needed | — | keep |
| 8 | vstdplus | `dist_lt` (4), `dist_add` (9) | needed | — | keep |
| 9 | vstdplus | `dist_sub` (2), `approx_eq` (2) | needed | — | keep |
| 10 | vstdplus | `dist_le` (0) | unused | — | delete |
| 11 | vstdplus | `f64_is_finite` (0) | unused | — | delete |
| 12 | vstdplus | `finite_dist` (0 src, 1 RTT) | unused | — | delete with RTT use |
| 13 | vstdplus | `UNREACHABLE_SPEC`, `unreachable_dist` | needed | — | keep |
| 14 | vstdplus | `zero_dist` (13) | needed | — | keep |
| 15 | vstdplus | 5 `f64_*_spec` uninterp fns | needed | — | keep |
| 16 | vstdplus | `f64_add/sub/mul/sqrt` exec (Chap26) | needed | — | keep |
| 17 | vstdplus | 6 `axiom_f64_*` arithmetic + group | needed | — | keep |
| 18 | vstdplus | 9 derive impls outside `verus!` | needed | — | keep |

1. Delete lines 59-67 (rows 4-5), 372-378 (row 10), 414-420 (row 11), and
   438-444 (row 12, after removing its one use in a Chap56 RTT): 33 lines.

### 3.10 `hash_map_with_view_plus.rs` (303 lines; 38 users; 8 RTT, 1 PTT)

vstd offers two models: the wrapper `HashMapWithView<Key, Value>`
(`hash_map.rs:26`, private field `m`, `View::V = Map<Key::V, Value>`, no
`iter`, no `Clone` or `PartialEq`), and direct specs on
`std::collections::HashMap` (`std_specs/hash.rs:372` `iter`, `:564` `len`,
`:578` `clone`, `:591` `new`; `View::V = Map<Key, Value>`). At 09.13
`hash_map::Iter` implements `IteratorSpecImpl` (`hash.rs:351`), not the `(int,
Seq)` `View` this module's iterator reads through `self.inner@` (line 180).

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `HashMapWithViewPlus` + uninterp View | needed | hash_map.rs:26 (no iter/Clone/Eq) | keep |
| 2 | vstdplus | trait `new` (requires feq injective) | provided | hash_map.rs:43 (same statement) | keep |
| 3 | vstdplus | `len`, `is_empty`, `get`, `insert` | provided | hash_map.rs:95,82,144,106 | keep |
| 4 | vstdplus | `clear`, `contains_key`, `remove` | provided | hash_map.rs:158,133,118 | keep |
| 5 | vstdplus | `iter` ensures (`r@.0`, `r@.1`) | broken | hash.rs:372 `HashMap::iter` | repair |
| 6 | vstdplus | `HashMapWithViewPlusIter` + View | broken | hash.rs:351 `IteratorSpecImpl` | delete |
| 7 | vstdplus | `iter_invariant` | broken | — | delete |
| 8 | vstdplus | `Iterator::next` two-arm ensures | broken | — | delete |
| 9 | vstdplus | `…GhostIterator` + View | broken | — | delete |
| 10 | vstdplus | `ForLoopGhostIteratorNew` impl | broken | — | delete |
| 11 | vstdplus | `ForLoopGhostIterator` impl | broken | — | delete |
| 12 | vstdplus | `Clone` (external_body, view eq) | needed | hash.rs:578 (per-value `cloned`) | keep |
| 13 | vstdplus | `PartialEq`, `Eq` (no ensures) | needed | — | keep |

Full names: row 9 `HashMapWithViewPlusGhostIterator`.

1. Phase 3 rewrite (delegated iteration, the
   `standards/prophetic_iterators_standard.rs` pattern): change `fn iter(&self)
   -> std::collections::hash_map::Iter<'_, Key, Value>` with `ensures` copied
   from `hash.rs:372-397` but stated over `self@` (`obeys_key_model::<Key>()
   ==> IteratorSpec::remaining(&it).unref().to_set() == ...`), and delete rows
   6-11 (lines 170-278, 109 lines). The ensures at lines 106-114 already state
   the two containment directions the prophetic form needs.
2. Rows 2-4 are identical in statement to `HashMapWithView`; the struct stays
   only because vstd's wrapper has no `iter`, `Clone`, or `PartialEq`.
3. Rows 12-13 remain `external_body` with an uninterpreted view; hash.rs:578
   states `cloned(this@[k], other@[k])` per key, which cannot reach
   `cloned@ == self@` without a view axiom (section 5 item 5).
4. PTT `rust_verify_test/tests/vstdplus/HashMapWithViewPlus.rs` exercises the
   old six loop patterns and must be rewritten with the module.

### 3.11 `hash_set_specs.rs` (37 lines; 0 users)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `HashSet::clone` spec (no ensures) | unused | hash.rs:578 (HashMap only) | delete |

1. The only `HashSet::clone` call under `src/` is inside the `external_body`
   `Clone for HashSetWithViewPlus` (`hash_set_with_view_plus.rs:274-279`),
   whose body verus does not check, so this specification is never consulted.
   Delete the file and `pub mod hash_set_specs;` in `src/lib.rs`: 37 lines.

### 3.12 `hash_set_with_view_plus.rs` (300 lines; 26 users; 1 PTT)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | struct + View `inner@.map(\|k\| k@)` | needed | hash_set.rs:27 (no iter/Clone/Hash) | keep |
| 2 | vstdplus | `axiom_hash_set_with_view_plus_finite` | broken | set.rs:227 (`finite()` is `true`) | delete |
| 3 | vstdplus | `group_hash_set_with_view_plus_axioms` | broken | — | delete |
| 4 | vstdplus | `new`, `with_capacity` | provided | hash_set.rs:44,60; hash.rs:927,940 | keep |
| 5 | vstdplus | `len`, `contains`, `insert` | provided | hash.rs:913,1004,961 | drop external_body |
| 6 | vstdplus | trait `iter` ensures | broken | hash.rs:1115 `HashSet::iter` | repair |
| 7 | vstdplus | `HashSetWithViewPlusIter` + View | broken | hash.rs:864 `IteratorSpecImpl` | delete |
| 8 | vstdplus | `iter_invariant` | broken | — | delete |
| 9 | vstdplus | `Iterator::next` ensures | broken | — | delete |
| 10 | vstdplus | `…GhostIterator` + View | broken | — | delete |
| 11 | vstdplus | `ForLoopGhostIteratorNew` impl | broken | — | delete |
| 12 | vstdplus | `ForLoopGhostIterator` impl | broken | — | delete |
| 13 | vstdplus | `Clone` (external_body, view eq) | needed | — | keep |
| 14 | vstdplus | `Hash` (external_body) | needed | — | keep |
| 15 | vstdplus | `PartialEq`, `Eq` (no ensures) | needed | — | keep |

Full names: row 10 `HashSetWithViewPlusGhostIterator`.

1. Delete lines 73-83 (rows 2-3): the axiom's ensures `s@.finite()` unfolds to
   `true` (set.rs:227-231, `#[deprecated]`). 12 lines.
2. Row 5: `HashSet::insert` ensures `final(m)@ == old(m)@.insert(k)`
   (hash.rs:961) and `Set::lemma_set_map_insert_commute` (set_lib.rs:455)
   together prove `self@ == old(self)@.insert(k@)` through the open view, so
   `insert`, `contains` (hash.rs:1004), and `len` (hash.rs:913 with
   `lemma_map_size` set_lib.rs:1111 under the `feq` injectivity already in
   `new`'s requires) can lose `external_body`. Three trusted bodies fewer; the
   vstd wrapper `HashSetWithView` (hash_set.rs:44-118) states the same ensures.
3. Rows 6-12: same Phase 3 rewrite as 3.10, returning
   `std::collections::hash_set::Iter<'_, Key>` with ensures from hash.rs:1115.
   Delete lines 166-268 (103 lines).
4. PTT `rust_verify_test/tests/vstdplus/HashSetWithViewPlus.rs` must be
   rewritten with the module.

### 3.13 `hashed_checked_u32.rs` (122 lines; 0 users)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `CheckedU32_feq_trigger` | unused | — | delete |
| 2 | vstdplus | `valid_key_type_CheckedU32` | unused | — | delete |
| 3 | vstdplus | `axiom_CheckedU32_feq` (admit) | unused | — | delete |
| 4 | vstdplus | `axiom_CheckedU32_key_model` (admit) | unused | — | delete |
| 5 | vstdplus | `group_CheckedU32_axioms` | unused | — | delete |
| 6 | vstdplus | `Hash`, `PartialEq`, `Eq` for CheckedU32 | unused | — | delete |
| 7 | vstdplus | `Display`, `Debug` for CheckedU32 | unused | — | delete |

1. The only file naming any item is the commented-out experiment
   `experiments/HashCheckedU32.rs` (`lib.rs:158`, FAILS). No chapter hashes a
   `CheckedU32`. Delete the file and `pub mod hashed_checked_u32;` in
   `src/lib.rs`: 122 lines, 2 `admit`, 2 `external_body`. Required anyway by
   3.5 note 3.

### 3.14 `monoid.rs` (45 lines; 22 users)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `spec_monoid` (23) | needed | relations.rs:19 (`associative` only) | keep |
| 2 | vstdplus | `spec_left_identity` | needed | — | keep |
| 3 | vstdplus | `spec_right_identity` | needed | — | keep |

### 3.15 `multiset.rs` (237 lines; 12 users)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `spec_filter_len` (12) | provided | seq_lib.rs:200 `filter(pred).len()` | keep now; bridge later |
| 2 | vstdplus | `lemma_spec_filter_len_nonneg` (0) | unused | trivial (`nat` length) | delete |
| 3 | vstdplus | `lemma_spec_filter_len_le_len` (0) | unused | seq_lib.rs:215 `lemma_filter_len` | delete |
| 4 | vstdplus | `lemma_spec_filter_len_concat` (3) | provided | seq_lib.rs:295 (see note 1) | keep with row 1 |
| 5 | vstdplus | `lemma_flatten_01_eq_spec_…` (2) | needed | — | keep |
| 6 | vstdplus | `lemma_flatten_01_multiset_eq_…` (2) | needed | — | keep |
| 7 | vstdplus | `lemma_seq_concat_to_multiset_…` (3) | needed | multiset.rs:371 (count only) | keep |
| 8 | vstdplus | `lemma_multiset_filter_distributes_…` | needed | — | keep (used by row 7) |

Full names: row 5 `lemma_flatten_01_eq_spec_filter_len`; row 6
`lemma_flatten_01_multiset_eq_filter`; row 7
`lemma_seq_concat_to_multiset_filter`; row 8
`lemma_multiset_filter_distributes_over_add`.

1. `spec_filter_len` recurses on `drop_last` exactly as vstd's `Seq::filter`
   (seq_lib.rs:200-212), so `spec_filter_len(s, p) == s.filter(p).len()` is
   provable by induction on `s.len()`; row 4 is then
   `Seq::filter_distributes_over_add` (seq_lib.rs:295) plus `lemma_seq_add_len`.
   Replacing the spec in 12 files (Chap18 ArraySeq family and LinkedList,
   Chap19, Chap21 `Exercise21_8.rs`, Chap23 `PrimTreeSeqStPer.rs`) is proof
   work for Phase 4, not a rename. Until then add one bridging lemma and keep
   rows 1 and 4.
2. Delete lines 50-68 (rows 2-3): 19 lines.

### 3.16 `partial_order.rs` (433 lines; 0 users; 1 RTT)

RTT: `tests/vstdplus/test_partial_order.rs`.

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `PartialOrder` trait | unused | relations.rs:74; cmp.rs:42 | delete |
| 2 | vstdplus | 12 integer impls | unused | laws_cmp.rs:257 `group_laws_cmp` | delete |
| 3 | vstdplus | `f32`, `f64` impls (6 admit) | unused | — | delete |
| 4 | vstdplus | `partial_order_ensures` uninterp | unused | — | delete |

1. vstd states the laws as `relations::partial_ordering(r)` (relations.rs:74)
   and the exec/spec bridge as `PartialOrdSpec::partial_cmp_spec` (cmp.rs:42-47).
   The float impls define `le` as `arbitrary()` with three `admit` proof bodies
   each and an `external_body` `compare`. Delete `src/vstdplus/partial_order.rs`,
   `pub mod partial_order;` in `src/lib.rs`, and the RTT: 433 lines, 6 `admit`,
   2 `external_body`.

### 3.17 `pervasives_plus.rs` (62 lines; 2 users)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `comment` spec fn (4) | needed | — | keep |
| 2 | vstdplus | `vec_swap` (Chap45, Chap65) | needed | vec.rs:192 (`swap_remove` only) | keep |

Users of `vec_swap`: Chap45 `BinaryHeapPQ.rs`, Chap65 `KruskalStEph.rs`. vstd
has no `Vec::swap` or slice `swap` spec (`std_specs/vec.rs`, `std_specs/slice.rs`).

### 3.18 `rand.rs` (100 lines; 10 users)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `random_usize_range` (8) | needed | — | keep |
| 2 | vstdplus | `seeded_rng` (2) | needed | — | keep |
| 3 | vstdplus | `random_bool_seeded` (2) | needed | — | keep |
| 4 | vstdplus | `SeededRng`, `ExSeededRng` | needed | — | keep |

### 3.19 `seq.rs` (286 lines; 4 users)

Users: Chap21 `Problem21_4.rs`, `Exercise21_7.rs`, `Algorithm21_2.rs`,
`Exercise21_5.rs`, all through the flatten items.

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `spec_sum_u32_seq`, `spec_sum_u32_fold` | unused | — | delete |
| 2 | vstdplus | `spec_sum_int_seq`, `spec_sum_int_fold` | unused | seq_lib.rs:918 `fold_left` | delete |
| 3 | vstdplus | `spec_sum_checked_u32_seq` (0) | unused | — | delete |
| 4 | vstdplus | `lemma_sum_int_equiv` (0) | unused | — | delete |
| 5 | vstdplus | `lemma_sum_int_push` (0) | unused | — | delete |
| 6 | vstdplus | `lemma_sum_int_unfold_take` (0) | unused | — | delete |
| 7 | vstdplus | `lemma_sum_checked_u32_unfold_take` (0) | unused | — | delete |
| 8 | vstdplus | `seq_u32_to_CheckedU32` (0; assume) | unused | — | delete |
| 9 | vstdplus | `spec_inner_lens_sum` (1) | needed | — | keep |
| 10 | vstdplus | `lemma_flatten_uniform_len` (3) | needed | seq_lib.rs:2781 (`<=` only) | keep |
| 11 | vstdplus | `lemma_flatten_len_is_inner_lens_sum` | needed | — | keep |
| 12 | vstdplus | `lemma_flatten_all` (1) | needed | — | keep |
| 13 | vstdplus | `lemma_flatten_contains` (0) | unused | seq_lib.rs:2823 composes | delete |

1. Row 8 contains an `assume` (line 276, marked FIXME). Delete lines 41-79,
   92-145, 216-232, 236-283 and the import on line 23: about 166 lines and one
   `assume`.

### 3.20 `seq_set.rs` (1260 lines; 30 users)

Users: 4 Chap05, 20 Chap06, 1 Chap17, 1 Chap41, 4 experiments. The 84 `pub`
items fall into groups; each row is a group. "exp only" means the only callers
are experiments that `lib.rs` lists as FAILS and commented out (`seq_set_exec`,
`simple_set_iter`, `simple_hash_set_iter`).

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `spec_nat_seq_sum`, `spec_nat_set_sum` | unused | — (exp only) | delete |
| 2 | vstdplus | 24 `spec_*weighted_{seq,set}_sum*` | needed | — | keep |
| 3 | vstdplus | 2 `lemma_push_not_contains_to_set_*` | provided | seq_lib.rs:2242 | delete |
| 4 | vstdplus | `lemma_push_not_contains_to_set` | provided | seq_lib.rs:2242 (no precondition) | replace |
| 5 | vstdplus | 2 `lemma_take_extends_set_*` | provided | seq_lib.rs:2129 + :2242 | delete |
| 6 | vstdplus | `lemma_take_one_more_extends_…` | provided | seq_lib.rs:2129 | delete |
| 7 | vstdplus | `lemma_set_contains_insert_idempotent` | unused | `=~=` with set.rs:358-380 | delete |
| 8 | vstdplus | `…the_seq_set_with_view` (Chap05) | provided | seq_lib.rs:2418 + :2242 | shrink body |
| 9 | vstdplus | `lemma_take_full_to_set_with_view` | provided | seq_lib.rs:2140 `lemma_take_len` | delete |
| 10 | vstdplus | `lemma_seq_index_in_map_to_set` (24) | provided | seq_lib.rs:757 conjunct 1 | shrink body |
| 11 | vstdplus | `lemma_map_to_set_contains_index` (21) | provided | seq_lib.rs:757 conj. 2; :1531 | shrink body |
| 12 | vstdplus | `lemma_map_not_contains_implies_all_ne` | needed | — | keep |
| 13 | vstdplus | `lemma_seq_map_to_set_equality` (Chap05) | needed | — | keep |
| 14 | vstdplus | `lemma_take_one_more_intersect` (Chap05) | needed | — | keep |
| 15 | vstdplus | `lemma_spec_nat_seq_fold_equals_…` | unused | — (also broken) | delete |
| 16 | vstdplus | private `lemma_to_seq_no_duplicates` | provided | set_lib.rs:660 (#1797) | delete |
| 17 | vstdplus | `lemma_spec_nat_seq_sum_is_nat_set_sum` | unused | — (finite requires vacuous) | delete |
| 18 | vstdplus | `lemma_nat_partial_sum_monotonic` | unused | — | delete |
| 19 | vstdplus | `lemma_nat_fold_left_step` | unused | — (exp only) | delete |
| 20 | vstdplus | `…sum_no_intermediate_overflow` | unused | — | delete |
| 21 | vstdplus | `lemma_nat_any_order_no_overflow` | unused | — | delete |
| 22 | vstdplus | `lemma_no_dup_same_set_implies_…` | unused | — | delete |
| 23 | vstdplus | `…seq_sum_permutation_invariant` | unused | seq_lib.rs:3622 (used inside) | delete |
| 24 | vstdplus | `lemma_u32_view_identity` | unused | — (exp only) | delete |
| 25 | vstdplus | `lemma_to_seq_gives_same_set` | unused | — (exp only) | delete |
| 26 | vstdplus | `lemma_seq_map_to_set_eq_set_map` | provided | seq_lib.rs:1403 | delete |
| 27 | vstdplus | `lemma_set_contains_iff_to_seq_map_…` | provided | set_lib.rs:640; seq_lib.rs:757 | delete |
| 28 | vstdplus | 12 `lemma_*weighted_seq_fold_equals_…` | broken | iset.rs:613, 627 | repair |
| 29 | vstdplus | private `lemma_weighted_fold_left_step` | needed | — | keep |
| 30 | vstdplus | `lemma_weighted_seq_sum_is_set_sum` (0) | unused | — (also broken) | delete |
| 31 | vstdplus | `lemma_int_fold_equals_nat_fold_…` | unused | duplicate of row 33 | delete |
| 32 | vstdplus | 12 `lemma_*seq_fold_left_plus_is_…` | needed | — | keep |
| 33 | vstdplus | 6 `lemma_fold_left_int_equals_nat_…` | needed | — | delete after 3.5 |

Full names: row 6 `lemma_take_one_more_extends_the_seq_set`; row 8
`lemma_take_one_more_extends_the_seq_set_with_view`; row 15
`lemma_spec_nat_seq_fold_equals_spec_set_fold`; row 20
`lemma_spec_nat_seq_sum_no_intermediate_overflow`; row 22
`lemma_no_dup_same_set_implies_same_multiset`; row 23
`lemma_spec_nat_seq_sum_permutation_invariant`; row 27
`lemma_set_contains_iff_to_seq_map_contains`; row 28
`lemma_[signed_]weighted_seq_fold_equals_set_fold{,_u8,…,_isize}`; row 31
`lemma_int_fold_equals_nat_fold_weighted`; row 32
`lemma_[signed_]seq_fold_left_plus_is_weighted_seq_sum{,_u8,…,_isize}`; row 33
`lemma_fold_left_int_equals_nat_as_int{,_u8,_u16,_u64,_u128,_usize}`. vstd
names: seq_lib.rs:2242 `Seq::lemma_push_to_set_commute`; :2129
`lemma_take_succ_push`; :2418 `lemma_map_take_succ`; :757 `to_set_ensures`;
:1531 `lemma_contains_to_index`; :1403 `lemma_to_set_map_commutes`;
set_lib.rs:660 `Set::lemma_to_seq_no_duplicates`; :640
`lemma_to_seq_to_set_id`; iset.rs:613 `lemma_fold_insert`; :627
`lemma_fold_empty`.

Counts: provided 12 (rows 3-6, 8-11, 16, 26-27; rows 3 and 5 hold 2 each),
broken 12 (row 28), unused 15 (rows 1, 7, 15, 17-25, 30-31), needed 45 (rows 2,
12-14, 29, 32-33).

1. Rows 3-4 (lines 145-196, 52 lines): vstd `Seq::lemma_push_to_set_commute`
   states `self.push(elem).to_set() =~= self.to_set().insert(elem)` with no
   precondition (seq_lib.rs:2242, since 2025-06-19), so the `!seq.contains(v)`
   requirement and the subset/superset halves are unnecessary. In Chap41
   `ArraySetStEph.rs` and in the 13 fold lemmas replace
   `lemma_push_not_contains_to_set(prefix, last)` by
   `prefix.lemma_push_to_set_commute(last)`.
2. Rows 5-6 (lines 198-256, 59 lines): `lemma_take_succ_push` gives
   `s.take(i+1) =~= s.take(i).push(s[i])` (seq_lib.rs:2129); with note 1 that
   is the whole statement.
3. Row 8 (lines 269-306): keep the signature (Chap05 `SetStEph.rs` and
   `SetMtEph.rs` call it with the `map(|i, k| k@)` shape) and replace the
   38-line body by `seq.lemma_map_take_succ(|k: T| k@, n);
   seq.take(n).map_values(|k: T| k@).lemma_push_to_set_commute(seq[n]@);` plus
   `assert(seq.take(n).map(|i: int, k: T| k@) =~= seq.take(n).map_values(|k: T|
   k@));` and the same `=~=` for `take(n+1)`. `map` and `map_values` are both
   `Seq::new(len, ...)` (seq_lib.rs:24, 30), so the `=~=` holds by index.
4. Rows 10-11 (lines 319-344): replace each body by `broadcast use
   Seq::to_set_ensures;` (row 10) and by `broadcast use Seq::to_set_ensures;
   let idx = seq.map(|i: int, k: T| k@).lemma_contains_to_index(s);` (row 11).
   The 45 call sites keep their names.
5. Row 16 (lines 490-521): delete; `Set::lemma_to_seq_no_duplicates` is a
   broadcast lemma reachable as `s.lemma_to_seq_no_duplicates()`
   (set_lib.rs:660, added 2026-09-08, #1797).
6. Rows 1, 7, 15, 17-27, 30-31 (lines 31-34, 258-265, 450-488, 523-533,
   536-652, 654-726, 784-810): delete, about 300 lines. They were written for
   `experiments/seq_set_exec.rs`, which `lib.rs:165` marks FAILS.
7. Row 28, repair recipe for each of the 12 lemmas (and row 15 if kept).
   `Set::fold` is `#[verifier::inline] self.to_iset().fold(z, f)` (set.rs:298),
   and the fold lemmas now live on `ISet` (iset.rs:613 `lemma_fold_insert`
   requiring `s.finite()`, `!s.contains(a)`, `is_fun_commutative(f)`;
   iset.rs:627 `lemma_fold_empty`). Empty case: `assert(seq.to_set().to_iset()
   =~= ISet::<T>::empty()); vstd::iset::fold::lemma_fold_empty::<T, B>(z, f);`.
   Step case: `assert(prefix.to_set().insert(last).to_iset() =~=
   prefix.to_set().to_iset().insert(last));` (ISet extensionality;
   `Set::contains` is `to_iset().contains` at set.rs:147, and
   `lemma_set_insert_same/different` are in `group_set_lemmas`);
   `prefix.to_set().to_iset().finite()` from `Set::axiom_is_finite` (set.rs:76);
   `!prefix.to_set().to_iset().contains(last)` from `!prefix.contains(last)` and
   `Seq::to_set_ensures` (seq_lib.rs:757); then
   `vstd::iset::fold::lemma_fold_insert(prefix.to_set().to_iset(), z, f, last)`.
   Agent 1's working copy (section 0) makes the two vstd calls but states
   neither `=~=` bridge; verus will report the `ensures` of each of the 13
   lemmas unproved until the bridges are added. `seq_to_set_is_finite`
   (removed, #2486) needs no replacement: finiteness is by type.
8. Row 33: after 3.5 the Chap06 invariants are stated over `nat` and these six
   bridges (about 40 lines) have no caller.
9. Broadcast header (lines 19-26): agent 1's rename to `group_set_lemmas` is
   the correct one; `vstd::seq::group_seq_axioms` (line 23) is deprecated in
   favour of `group_seq_lemmas` (seq.rs:1812) and will emit a deprecation
   warning; `group_seq_properties`, `group_to_multiset_ensures`
   (seq_lib.rs:3966, 3956) and `group_set_lib_default` (set_lib.rs:1457) exist.

Net: 12 lemmas repaired, 27 deleted, 3 shrunk; about 430 lines removed now and
40 more after 3.5.

### 3.21 `smart_ptrs.rs` (67 lines; 19 users)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `Arc::clone` spec (`res == *a`) | needed | smart_ptrs.rs:53 (Box only) | keep |
| 2 | vstdplus | `call_f` (3) | needed | — | keep |
| 3 | vstdplus | `arc_deref` (18; external_body) | needed | — (no `Deref for Arc` spec) | keep |
| 4 | vstdplus | `arc_vec_as_slice` (2; external_body) | needed | — | keep |

vstd specifies `Deref` only for `String`, `Vec`, and `ManuallyDrop`
(string.rs:362, vec.rs:248, manually_drop.rs:47). Row 1 is what makes 3.2
deletable; it needs `feature(allocator_api)` in `lib.rs` line 7.

### 3.22 `sqrt.rs` (60 lines; 0 importers; 2 implicit users)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `usize::isqrt` assume_specification | needed | — (no `isqrt` in vstd) | keep |
| 2 | vstdplus | `u8/u16/u32/u64/u128::isqrt` (0 calls) | unused | — | delete |

1. `assume_specification` needs no `use`, which is why the importer count is 0;
   `Chap21/Algorithm21_6.rs:85` and `Chap21/Exercise21_8.rs:187` call
   `n.isqrt()` on `usize`. Delete lines 33-56: 25 lines.

### 3.23 `strings.rs` (89 lines; 1 user: Chap44 `DocumentIndex.rs`)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `char_is_ascii_alphabetic` + spec | needed | char.rs (len_utf8, is_whitespace) | keep |
| 2 | vstdplus | `char_to_ascii_lowercase` + spec | needed | — | keep |
| 3 | vstdplus | `string_push` (1) | provided | string.rs:386 `String::push` | replace |
| 4 | vstdplus | `string_clear` (0) | provided | string.rs:411 `String::clear` | delete |
| 5 | vstdplus | `string_is_empty` (1) | provided | string.rs:405 `String::is_empty` | replace |

1. The three `String` specs were merged 2026-08-04 (#2738). In
   `Chap44/DocumentIndex.rs` replace `string_push(&mut s, c)` by `s.push(c)`
   and `string_is_empty(&s)` by `s.is_empty()`; the vstd ensures are
   `final(s)@ == old(s)@.push(c)` and `res == (s@.len() == 0)`, the same
   statements. Delete lines 68-86: 19 lines and 3 `external_body`.

### 3.24 `threads_plus.rs` (141 lines; 1 user; 1 RTT)

User: Chap02 `HFSchedulerMtEph.rs`. RTT: `tests/Chap02/test_threads_plus.rs`.

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `JoinHandlePlus`, `predicate`, `join` | provided | thread.rs:16-50 (same ensures) | replace |
| 2 | vstdplus | `spawn_plus` | provided | thread.rs:107 `spawn` (same spec) | replace |
| 3 | vstdplus | `is_finished` (RTT only) | unused | — | delete |
| 4 | vstdplus | `ThreadIdPlus` | provided | thread.rs:143 `ThreadId` | delete |
| 5 | vstdplus | `IsThreadPlus`, `view`, `agrees` | provided | thread.rs:164-197 `IsThread` | delete |
| 6 | vstdplus | `ghost_thread_id_plus` | provided | thread.rs `ghost_thread_id` | delete |
| 7 | vstdplus | `thread_id_plus` | provided | thread.rs:200 `thread_id` | delete |

1. In `Chap02/HFSchedulerMtEph.rs` line 25 replace the import by `use
   vstd::thread::{spawn, JoinHandle};`, and at lines 38, 158, 185 replace
   `JoinHandlePlus<T>` by `JoinHandle<T>` and `spawn_plus(f)` by `spawn(f)`.
   Delete `src/vstdplus/threads_plus.rs`, `pub mod threads_plus;` in
   `src/lib.rs`, and the RTT (it tests `is_finished`, which the scheduler does
   not call): 141 lines, 4 `external_body`. `standards/no_unsafe_standard.rs`
   line 48 names this file's already-deleted `ThreadShareablePlus`.

### 3.25 `total_order.rs` (723 lines; 66 users; 1 RTT)

RTT: `tests/vstdplus/test_total_order.rs`.

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `TotalOrder` (`le`, 4 laws, `cmp`) | needed | relations.rs:55 (laws only) | keep |
| 2 | vstdplus | 2 `cmp_spec_*_implies_le` defaults | needed | — | keep (assume, policy) |
| 3 | vstdplus | 12 integer impls | needed | laws_cmp.rs:257 (see note 1) | keep |
| 4 | vstdplus | `IsLtTransitive` + 12 impls (Chap39) | needed | laws_cmp.rs:19-22 | keep |
| 5 | vstdplus | `String` impls (accept-annotated) | needed | — (no String Ord spec) | keep |
| 6 | vstdplus | `Pair<K, V>` impl | needed | laws_cmp.rs:134 (tuples only) | keep |
| 7 | vstdplus | `char` impl (2 assumes) | needed | — (no char in laws_cmp) | keep |

1. No item is deletable. `relations::total_ordering(r)` (relations.rs:55)
   states the four laws for a spec relation but bundles no exec `cmp`. vstd's
   `laws_cmp` and `OrdSpec` (cmp.rs:116) give, for the 12 integer types,
   `obeys_cmp::<T>()` from which reflexivity, antisymmetry, transitivity and
   totality of `is_le` follow; a later redesign could define `TotalOrder::le`
   as `self.is_le(other)` for `Self: Ord` and derive the laws from
   `obeys_cmp`, deleting the 12 integer impls (about 370 lines) at the cost of
   proof changes in 66 files. Not recommended for this migration.

### 3.26 `VecQueue.rs` (94 lines; 0 users)

| # | Chap | Item | Class | vstd | Edit |
|---|------|------|-------|------|------|
| 1 | vstdplus | `VecQueue` struct + View | provided | vecdeque.rs:20 `VecDeque` | delete |
| 2 | vstdplus | `VecQueueTrait` (5 methods) | provided | vecdeque.rs:93,87,118,151 | delete |

1. `VecDeque::push_back` and `pop_front` (vecdeque.rs:118, 151) carry the same
   `Seq` postconditions as `enqueue` and `dequeue`, and are O(1) where
   `dequeue` is O(n) (`Vec::remove(0)`). Delete the file and `pub mod
   VecQueue;` in `src/lib.rs`: 94 lines.

## 4. Recommendations ranked by source lines removed

Lines are counts of source lines deleted from `src/vstdplus/` (call-site edits
not counted). Rows marked Phase 3 or 4 depend on the iterator rewrite or on
proof repair and are not first-round work.

| # | Chap | Recommendation | Lines | Holes removed | Prerequisite |
|---|------|----------------|------:|---------------|--------------|
| 1 | vstdplus | 3.5 + 3.13: vstd overflow.rs | 607 | 24 external_body, 2 admit | 6 Chap06 renames |
| 2 | vstdplus | 3.16 delete partial_order.rs + RTT | 433 | 6 admit, 2 external_body | none |
| 3 | vstdplus | 3.20 seq_set.rs delete 27, shrink 3 | 430 | 0 | 14 call-site renames |
| 4 | vstdplus | 3.10 + 3.12 iterator rewrite | 212 | 0 | Phase 3 |
| 5 | vstdplus | 3.19 seq.rs unused sum items | 166 | 1 assume | none |
| 6 | vstdplus | 3.24 delete threads_plus.rs + RTT | 141 | 4 external_body | 4 edits in Chap02 |
| 7 | vstdplus | 3.4 checked_int.rs trait + 15 items | 135 | 0 | none |
| 8 | vstdplus | 3.26 delete VecQueue.rs | 94 | 0 | none |
| 9 | vstdplus | 3.8 feq.rs ExFeq, 13 impls, 1 lemma | 88 | 0 | none |
| 10 | vstdplus | 3.2 delete arc_rwlock.rs | 71 | 3 external_body | 33 call-site edits |
| 11 | vstdplus | 3.20 row 33 (int/nat bridges) | 40 | 0 | row 1 |
| 12 | vstdplus | 3.11 delete hash_set_specs.rs | 37 | 0 | none |
| 13 | vstdplus | 3.3 power2_plus.rs 4 lemmas | 35 | 0 | none |
| 14 | vstdplus | 3.9 float.rs 4 fns | 33 | 0 | 1 RTT edit |
| 15 | vstdplus | 3.22 sqrt.rs 5 unused types | 25 | 0 | none |
| 16 | vstdplus | 3.23 strings.rs 3 wrappers | 19 | 3 external_body | 2 edits in Chap44 |
| 17 | vstdplus | 3.15 multiset.rs 2 lemmas | 19 | 0 | none |
| 18 | vstdplus | 3.12 finite axiom + group | 12 | 1 admit | none |
| 19 | vstdplus | 3.6 clone_pred2 | 10 | 1 external_body | none |
| 20 | vstdplus | 3.12 row 5 verify 3 bodies | 0 | 3 external_body | Phase 4 |
| 21 | — | total | 2607 | 46 | of 6,726 lines |

Rows 1-3, 5-9, 12-19 (2,392 lines) are deletions with at most a rename at the
call sites and can precede the iterator rewrite. The repairs that must be
applied before anything in `src/vstdplus/` compiles are the 13 fold lemmas of
3.20 row 28 (recipe in 3.20 note 7) and the two broadcast renames agent 1 has
already made.

## 5. Items not classified, and why

| # | Chap | Item | Question | How to settle it |
|---|------|------|----------|------------------|
| 1 | vstdplus | `ClonePlus::clone_plus` (21) | is `cloned` free from `.clone()`? | experiment (note 1) |
| 2 | vstdplus | `clone_fn`, `clone_fn2`, `clone_pred*` | Clone on closures at 09.13? | rerun experiment (note 2) |
| 3 | vstdplus | `accept.rs` cargo path | does `verus!` still erase the fn? | one `scripts/rtt.sh` run |
| 4 | vstdplus | `axiom_obeys_feq_full` for integers | does integer `clone()` give `==`? | experiment (note 4) |
| 5 | vstdplus | hash_map Plus `Clone`, `PartialEq` | view too opaque for ensures | design decision (note 5) |
| 6 | vstdplus | checked_nat RTT file | imports a missing module | read the next `rtt` log |

1. `strictly_cloned(a, b)` is defined as `call_ensures(T::clone, (&a,), b)`
   (pervasive.rs:416), which is the predicate verus asserts after a call, so a
   plain `x.clone()` on generic `T: Clone` should already yield `cloned(*x, r)`
   without `ClonePlus`. No experiment in `src/experiments/` measures this
   (`experiments/clone_plus.rs` tests only `clone_plus()`). Experiment: `fn
   t<T: Clone>(x: &T) { let r = x.clone(); assert(cloned(*x, r)); }`. If it
   SUCCEEDS, `ClonePlus` (21 users, 1 `external_body`) is provided by the
   language and the 114-line module shrinks to the four closure helpers.
2. `experiments/closure_clone_first_class.rs` recorded FAILS at 03.28 ("Verus
   does not recognize this trait bound: <{closure} as Clone>"). Rerun at 09.13;
   SUCCEEDS would make `clone_fn`, `clone_fn2`, `clone_pred`, `clone_fn_usize`
   (31 users, 4 `external_body`) provided by the language.
3. Section 3.1: the stub compiles under rustc 1.98.1; whether the `verus!`
   macro from crates.io `vstd 0.0.0-2025-08-12-1837` erases `proof fn accept`
   in a cargo build is unchanged by the upgrade but unmeasured since it, and
   this agent may not run `scripts/rtt.sh`.
4. vstd `group_laws_eq` proves the `eq` and `view` conjuncts of
   `obeys_feq_full` for `bool` and the 12 integers without `admit`; the
   `cloned` conjunct has no vstd source for integers (clone.rs specifies `bool`
   and `char` only). Experiment: `let x: u64 = 3; let y = x.clone(); assert(y
   == x);`. If it SUCCEEDS, `axiom_obeys_feq_full` can be proved for the 13
   primitive types and the `admit` scoped to user types.
5. With `HashMapWithViewPlus`'s `View` uninterpreted (line 59), `cloned@ ==
   self@` cannot be derived from hash.rs:578, which states `cloned` per value.
   `hash_set_with_view_plus.rs` opens its view as `inner@.map(...)`; the same
   choice here (`inner@.map_keys(...)` over `Key::V`) would let `Clone`,
   `PartialEq` (which today has no `ensures` at all), `len`, `get`, `insert`,
   `remove`, `contains_key`, `clear` be proved from `std_specs/hash.rs`
   instead of trusted. A design decision, not a classification.
6. `tests/vstdplus/test_checked_nat_with_checked_view.rs` imports
   `apas_verus::vstdplus::checked_nat_with_checked_view`, which does not exist;
   whether `scripts/rtt.sh` compiles this file today is unknown from this
   review.

## 6. Shape of the edits, for the orchestrator's CST decision

Call-site edits are renames or one-token substitutions: 14 files for 3.20 rows
3-4, 7 files for 3.2, 6 files for 3.5, 2 for 3.23, 1 for 3.24. The 13
fold-lemma repairs in 3.20 note 7 are three-line insertions of identical shape,
which a CST transformer could apply from one template. The iterator rewrite
(3.10, 3.12) is the same rewrite the 119 chapter files need and belongs to
that tool.
