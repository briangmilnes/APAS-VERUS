# Iterator transformer on Chap18 (r210)

The `iterator-upgrade` process in `~/projects/CSTs` (`processes/iterator-upgrade`,
built on `processes/iterator-upgrade-lib` and `datastructs/vcst-lib`) rewrote the
nine Chap18 sources and the eight Chap18 proof-time tests from the old wrapper
iterator model to Verus 0.2026.09.13's prophetic model. It ran on a fixture copy
of this tree (`processes/iterator-upgrade/tests/fixtures/APAS-VERUS`, baseline
commit `c520913`: main `4bd6b4f92` plus the uncommitted r207-r209 working tree
of 2026-09-21). The live tree was not edited; the result is the patch series in
`plans/r210-chap18-patches/`.

Tool reports: `~/projects/CSTs/processes/iterator-upgrade/analyses/iterator-upgrade-detect-20260921-110253.log`
and `…/iterator-upgrade-apply-20260921-110254.log`.

## 1. Detect summary by class

`iterator-upgrade -c <fixture> --chapter Chap18 --detect` found 188 sites in
16 of the 17 files (`ArraySeqSpecsAndLemmas.rs` has none) and 8 residuals.

| # | Group | Class | Sites |
|---|-------|-------|------:|
| 1 | a | delete-iter-struct | 8 |
| 2 | a | delete-ghost-struct | 8 |
| 3 | a | delete-view-impl | 16 |
| 4 | a | delete-iter-invariant | 8 |
| 5 | a | delete-forloop-ghost-impl | 16 |
| 6 | b | delete-iterator-impl | 8 |
| 7 | c | replace-iter-fn | 9 |
| 8 | c | replace-into-iter-impl | 15 |
| 9 | d | subst-view-index | 0 |
| 10 | d | subst-view-seq | 0 |
| 11 | d | delete-iter-invariant-conjunct | 0 |
| 12 | d | subst-wrapper-pos | 0 |
| 13 | d | subst-wrapper-elements | 0 |
| 14 | e | delete-fmt-impl | 32 |
| 15 | e | add-use-iter | 38 |
| 16 | e | delete-unused-use | 0 |
| 17 | f | ptt-regenerate-test | 30 |
| 18 | all | sites | 188 |

Class (d) is zero because no Chap18 source holds a manual loop over its own
iterator; the 38 `add-use-iter` sites are 8 sources and 30 test bodies.

| # | Residual class | Sites | Where |
|---|----------------|------:|-------|
| 1 | requires-on-into-iter | 1 | ArraySeqMtEphSlice.rs (into_iter for &Slice) |
| 2 | preexisting-unused-import | 7 | `use std::slice::Iter;` in 7 sources |
| 3 | all | 8 | |

The `requires` residual is the one non-mechanical site (hand fix 1). The seven
unused imports were unreferenced before the rewrite too, so the tool left them.

## 2. Apply

`--apply` wrote 16 files, 0 write failures. `git diff --stat` against the
fixture baseline immediately after the run:

| # | Chap | File | Lines |
|---|------|------|-------|
| 1 | 18 | ArraySeq.rs | 157 (+/-) |
| 2 | 18 | ArraySeqMtEph.rs | 146 |
| 3 | 18 | ArraySeqMtEphSlice.rs | 141 |
| 4 | 18 | ArraySeqMtPer.rs | 146 |
| 5 | 18 | ArraySeqStEph.rs | 146 |
| 6 | 18 | ArraySeqStPer.rs | 149 |
| 7 | 18 | LinkedListStEph.rs | 142 |
| 8 | 18 | LinkedListStPer.rs | 142 |
| 9 | 18 | ProveArraySeq.rs | 61 |
| 10 | 18 | ProveArraySeqMtEph.rs | 182 |
| 11 | 18 | ProveArraySeqMtEphSlice.rs | 114 |
| 12 | 18 | ProveArraySeqMtPer.rs | 182 |
| 13 | 18 | ProveArraySeqStEph.rs | 190 |
| 14 | 18 | ProveArraySeqStPer.rs | 61 |
| 15 | 18 | ProveLinkedListStEph.rs | 61 |
| 16 | 18 | ProveLinkedListStPer.rs | 61 |

Totals: 16 files changed, 545 insertions(+), 1537 deletions(-). After the two
hand fixes below: 548 insertions(+), 1538 deletions(-).

Old-model names remain only in comments (`// for-iter: … using
ForLoopGhostIterator` headers in the tests and doc lines in the sources); the
tool does not edit comments.

## 3. Hand fixes (Edit tool, one at a time)

| # | Chap | File | Line | What | Why |
|---|------|------|------|------|-----|
| 1 | 18 | ArraySeqMtEphSlice.rs | 1556 | into_iter: requires removed, ensures conditional | Verus rejects requires on ext-trait impl |
| 2 | 18 | ArraySeqStEph.rs | 770 | `#[verifier::rlimit(20)]` on impl `scan`, reverted | rlimit in an untouched loop; did not help |

Hand fix 1. Validate run 1 (`logs/validate.20260921-110310.log`, 3 s) stopped
with one front-end error at `ArraySeqMtEphSlice.rs:1556`: "trait method
implementation cannot declare requires clauses". The `IntoIterator for
&ArraySeqMtEphSliceS<T>` impl carried `requires self.spec_arrayseqmtephslice_wf()`
in the old model; the tool reports it as the `requires-on-into-iter` residual
because the replacement is a specification decision (IteratorMigrationStudy §4
row 15). The fix follows the `Chap05/SetStEph.rs` form: the `requires` is
removed and the constructor triple is made conditional on the well-formedness
predicate:

```rust
fn into_iter(self) -> (it: Self::IntoIter)
    ensures
        self.spec_arrayseqmtephslice_wf() ==> {
            &&& IteratorSpec::remaining(&it) == self.spec_backing_seq().as_ref()
            &&& vstd::std_specs::slice::into_iter_elts(it) == self.spec_backing_seq()
            &&& IteratorSpec::decrease(&it) is Some
        },
```

The exec body is unchanged. It has a follow-on that no spec change can close:
see "Needs discussion" below.

Hand fix 2. Validate run 2 (`logs/validate.20260921-110506.log`) reported
`error: while loop: Resource limit (rlimit) exceeded` at
`ArraySeqStEph.rs:780` — the loop of impl `scan`. The tool did not touch that
function (its hunks are at lines 31, 929, 1055 and 1117); the body is
token-identical to `scan` in `ArraySeqStPer.rs`, `LinkedListStEph.rs`,
`LinkedListStPer.rs`, `ArraySeqMtEph.rs` and `ArraySeqMtPer.rs`, which all
verified in the same run, and the only preamble difference is the trait-level
`f.ensures(..) <==> ret == spec_f(x, y)` (StEph) against `==>` (StPer), which
loop isolation keeps out of the loop query. The profile run
(`logs/validate.20260921-110926.log`, `--profile`) shows 2,133 user-quantifier
instantiations for `scan`, all in vstd `seq_lib`/`seq` axioms and none above
400 — no user-level matching loop. The attempted fix was the house pattern
for borderline proofs (17 `#[verifier::rlimit(N)]` uses in `src/`): one
`#[verifier::rlimit(20)]` attribute on the impl `scan`, no proof or exec text
changed. Validate run 4 (`logs/validate.20260921-111734.log`) still reported
the rlimit error at the same loop after 591 s (z3 RSS 2.9 GB), so the query
diverges rather than sitting at the budget's edge. The attribute was reverted
with the Edit tool; the patch series carries no change to `scan`. The error is
recorded under "Needs discussion" below.

## 4. Validate

| # | Run | Log | Result |
|---|-----|-----|--------|
| 1 | isolate Chap18 after apply | validate.20260921-110310.log | 1 front-end error (requires) |
| 2 | after hand fix 1 | validate.20260921-110506.log | 998 verified, 2 errors |
| 3 | run 2 with --profile | validate.20260921-110926.log | same 2 errors, scan profiled |
| 4 | with rlimit(20) on scan | validate.20260921-111734.log | 998 verified, 2 errors, 591 s |

Final state: run 2 is the validation of the patched sources as delivered (the
rlimit attribute of run 4 was reverted, so the sources of run 2 and of the
patch series are identical). Result: `998 verified, 2 errors`, 154 s, peak
rust_verify RSS 1021 MB, peak z3 RSS 1473 MB. The target of 0 errors is not
met: the two errors are the two items under "Needs discussion".

Warnings and trigger notes: 0 warnings, 0 "automatically chose triggers"
notes in run 2 and in the PTT harness run.

### Needs discussion (not fixed; exec body left as the tool wrote it)

`ArraySeqMtEphSlice.rs:1564`, in the `into_iter` of hand fix 1:

```
error: precondition not satisfied
    --> src/Chap18/ArraySeqMtEphSlice.rs:1564:28
     |
1564 |             let sl: &[T] = arc_vec_as_slice(&self.data, self.start, self.len);
     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: src/vstdplus/smart_ptrs.rs:60:18
     |
  60 |         requires start + len <= (*a)@.len(),
     |                  -------------------------- failed precondition
```

`arc_vec_as_slice` (`src/vstdplus/smart_ptrs.rs:59`, `external_body`) requires
`start + len <= (*a)@.len()`, which is exactly `spec_arrayseqmtephslice_wf()`.
The baseline impl discharged it through its `requires`; Verus 0.2026.09.13
rejects a `requires` on an external trait's impl method, and a conditional
`ensures` cannot discharge a precondition inside the body. Every closure of
this obligation changes the exec body, which rule 9 of r208 forbids without
discussion. Options: (a) an exec bounds check that returns `[].iter()` when
`self.start + self.len > data.len()` (O(1), keeps the impl; the conditional
ensures then holds in both branches); (b) drop the `IntoIterator for
&ArraySeqMtEphSliceS` impl and keep only the trait `iter(&self)` with its
`requires` (the four Slice PTTs use `iter()`, `into_iter` and `for … in &s`;
the two borrow-into tests would go); (c) a `#[verifier::type_invariant]` on
`ArraySeqMtEphSliceS` carrying the wf predicate, which needs every constructor
site to prove it (`slice`, `from_vec`, `empty`, `singleton`, `new`, the
struct literals in `lemma_sum_inner_lens_split` and `flatten_dc_vec`). None
was applied.

`ArraySeqStEph.rs:780`, impl `scan`, not a site of the migration:

```
error: while loop: Resource limit (rlimit) exceeded; consider rerunning with --profile for more details
   --> src/Chap18/ArraySeqStEph.rs:780:13
    |
780 |             while i < len
    |             ^^^^^^^^^^^^^
```

Tried: the profile of §3 (no user-level quantifier above 400 instantiations;
the top entries are `seq_lib.rs:3742`, `seq_lib.rs:942`, `seq.rs:1678` and
`seq.rs:1701`), and `#[verifier::rlimit(20)]`, which still exceeded the
budget after 591 s. Not tried, because each changes proof text outside the
migration: a loop-body `assert` that pins `acc == spec_f(old_acc,
a.spec_index(i))` before the `reveal(Seq::fold_left)`, dropping the
`reveal` in favour of `lemma_fold_left_split`, or matching the trait's
`<==>` closure bridge (line 300) to the `==>` form the other five files use.
The live tree has no Chap18 validate log under 0.2026.09.13 that passes
(`logs/validate.20260921-085917.log` and later stop at front-end errors), so
whether `scan` verified before the transform is not recorded; its body is
untouched by the patch series.

## 5. RTT

The vstd pin needed no edit: `Cargo.toml` names crates.io
`vstd = "0.0.0-2025-08-12-1837"` and its `[patch.crates-io]` section already
redirects `vstd` and `verus_state_machines_macros` to the git rev
`671956ec527d3b7164779f767bdbfe769bedce6c` that `rust_verify_test/Cargo.toml`
pins.

`cargo test --release` on the eight targets (`TestArraySeq`, `TestArraySeqMtPer`,
`TestArraySeqMtEph`, `TestChap18ArraySeqMtEphSlice`, `TestArraySeqStEph`,
`TestArraySeqStPer`, `TestLinkedListStEph`, `TestLinkedListStPer`), log
`logs/rtt.20260921-112745.log` in the fixture: the library does not build,
`could not compile apas-verus (lib) due to 168 previous errors; 1 warning
emitted`, so no test ran. The errors are tree-wide and predate the transform:

```
error[E0433]: cannot find `std_specs` in `vstd`
  --> src/Chap18/ArraySeq.rs:34:15
   |
34 |     use vstd::std_specs::iter::*;
   |               ^^^^^^^^^ could not find `std_specs` in `vstd`
   |
note: found an item that was configured out
  --> /home/milnes/.cargo/git/checkouts/verus-e4ebf515fa1de14c/671956e/source/vstd/vstd.rs:97:9
```

`vstd.rs:96-97` reads `#[cfg(verus_keep_ghost)] pub mod std_specs;`, so a
cargo build sees no `vstd::std_specs::iter` at all. Of the 168 errors, 8 are
the Chap18 `use vstd::std_specs::iter::*;` lines the tool added (the same
form `Chap05/SetStEph.rs:37`, `Chap06/DirGraphStEph.rs:32` and
`standards/iterators_standard.rs:82` use), 156 are `VerusForLoopWrapper` and
`std_specs` uses in the 22 r208 Chap05/Chap06 files and the standards, and 4
are unrelated (`vstd::iset::lemma_iset_finite_if_subset_of_seq`,
`vstd::iset::fold::is_fun_commutative`, `IteratorSpecImpl`). The live tree
has no `logs/rtt*.log`. The RTT check therefore cannot be reached for Chap18
in this tree state; the eight Chap18 test files themselves were not changed.
A tool option that emits `#[cfg(verus_keep_ghost)] use vstd::std_specs::iter::*;`
(the import-gate form CLAUDE.md allows) would remove Chap18's 8 errors; the
other 160 need the r208 files to adopt the same gate or a vstd build that
keeps `std_specs`.

## 6. PTT

The eight rewritten `rust_verify_test/tests/Chap18/Prove*.rs` files hold 38
`verus_code!` bodies (10, 6, 4, 6, 6, 2, 2, 2). `scratch/r210-extract-ptt-bodies.awk`
copies each body into a `pub mod <test> { use vstd::prelude::*; verus! { … } }`
block with `apas_verus::` rewritten to `crate::` (`scratch/ptt_bodies/ptt_Chap18_*.rs`),
and `scratch/r210-validate-ptt-chap18.sh` verifies them in the
`validate-standard.sh` harness form: a crate root that is `src/lib.rs` plus a
`pub mod ptt_chap18 { #[path] … }` block, with the `isolate Chap18` cfg flags.

| # | Run | Log | Result |
|---|-----|-----|--------|
| 1 | after hand fix 1 | validate-ptt-Chap18.20260921-111426.log | 1078 verified, 2 errors |

Run 1 is the final PTT result: the sources it verified are the sources of the
patch series (the rlimit attribute was added after it and reverted before the
patches were made). It verified 80 more items than the 998 of the source-only run and reported
no error in any test module: the two errors are the source errors of §4. The
regenerated tests cover the six loop patterns of `iterator_ptt_standard.rs`
(loop-borrow-iter, loop-borrow-into, loop-consume, for-borrow-iter,
for-borrow-into, for-consume) where the collection offers them; the
higher-order-function tests of `ProveArraySeq.rs` and the others were not
touched by the tool and still verify.

## 7. Alg Analysis annotations

Every `/// - Alg Analysis:` line in the eight algorithm files (350 Code-review
lines and 128 APAS cost-spec lines; `ArraySeqSpecsAndLemmas.rs` has none) was
read against the body of the function it annotates. None was edited. The
Code-review lines are tabled below; the APAS lines quote the textbook's cost
specification and were read for the comparison but carry no claim about the
body. Verdict: 309 confirmed, 41 disputed.

| # | Chap | File | Code review | APAS | Confirmed | Disputed |
|---|------|------|------------:|-----:|----------:|---------:|
| 1 | 18 | ArraySeq.rs | 50 | 21 | 43 | 7 |
| 2 | 18 | ArraySeqStPer.rs | 40 | 20 | 38 | 2 |
| 3 | 18 | ArraySeqStEph.rs | 42 | 20 | 40 | 2 |
| 4 | 18 | LinkedListStPer.rs | 41 | 10 | 38 | 3 |
| 5 | 18 | LinkedListStEph.rs | 43 | 10 | 39 | 4 |
| 6 | 18 | ArraySeqMtEph.rs | 52 | 21 | 40 | 12 |
| 7 | 18 | ArraySeqMtEphSlice.rs | 34 | 6 | 32 | 2 |
| 8 | 18 | ArraySeqMtPer.rs | 48 | 20 | 39 | 9 |
| 9 | 18 | all | 350 | 128 | 309 | 41 |

### Disputed annotations

| # | Chap | File | Line | Fn | Claim | Body says |
|---|------|------|------|----|-------|-----------|
| 1 | 18 | ArraySeq.rs | 316 | new | Span O(1) | std::vec::from_elem is sequential: Span… |
| 2 | 18 | ArraySeq.rs | 500 | scan_inclusive | Span O(1) | sequential loop: Span O(n) |
| 3 | 18 | ArraySeq.rs | 514 | subseq_copy | Span O(1) | sequential clone loop: Span O(length) |
| 4 | 18 | ArraySeq.rs | 526 | remove | Span O(1) | Vec::remove shifts the tail: Span O(n) |
| 5 | 18 | ArraySeq.rs | 537 | insert | Span O(1) | Vec::insert shifts the tail: Span O(n) |
| 6 | 18 | ArraySeq.rs | 548 | from_vec | Work O(n) worst case | wraps the Vec: Work O(1) |
| 7 | 18 | ArraySeq.rs | 978 | inject | Work O(n), Span O(n) | copy loop plus update loop: O(n + m) (t… |
| 8 | 18 | ArraySeqStEph.rs | 162 | from_vec | Work O(n) worst case | wraps the Vec: Work O(1) (impl line 438… |
| 9 | 18 | ArraySeqStEph.rs | 600 | inject | Work O(n), Span O(n) | copy loop plus update loop: O(n + m) (t… |
| 10 | 18 | ArraySeqStPer.rs | 153 | from_vec | Work O(n) worst case | wraps the Vec: Work O(1) (impl line 419… |
| 11 | 18 | ArraySeqStPer.rs | 581 | inject | Work O(n), Span O(n) | copy loop plus update loop: O(n + m) (t… |
| 12 | 18 | LinkedListStEph.rs | 109 | set | Work O(index) | Vec::set: Work O(1) (impl line 304 says… |
| 13 | 18 | LinkedListStEph.rs | 141 | from_vec | Work O(n) | wraps the Vec: Work O(1) (impl line 352… |
| 14 | 18 | LinkedListStEph.rs | 816 | clone | Span O(1) | Vec::clone is sequential: Span O(\|self… |
| 15 | 18 | LinkedListStEph.rs | 830 | eq | Span O(1) | Vec equality is sequential: Span O(\|se… |
| 16 | 18 | LinkedListStPer.rs | 133 | from_vec | Work O(n) | wraps the Vec: Work O(1) (impl line 334… |
| 17 | 18 | LinkedListStPer.rs | 798 | clone | Span O(1) | Vec::clone is sequential: Span O(\|self… |
| 18 | 18 | LinkedListStPer.rs | 812 | eq | Span O(1) | Vec equality is sequential: Span O(\|se… |
| 19 | 18 | ArraySeqMtEph.rs | 329 | inject | Work O(n), Span O(n) | copy loop plus update loop: O(n + m) (t… |
| 20 | 18 | ArraySeqMtEph.rs | 651 | map_par | Span O(log\|a\|) | subseq_copy and append are sequential O… |
| 21 | 18 | ArraySeqMtEph.rs | 706 | filter_par | Span O(log\|a\|) | subseq_copy and append are sequential O… |
| 22 | 18 | ArraySeqMtEph.rs | 791 | reduce_par | Span O(log\|a\|) | subseq_copy is sequential O(n) at the t… |
| 23 | 18 | ArraySeqMtEph.rs | 887 | reduce_dc | Span O(log n) | subseq_copy is sequential O(n) at the t… |
| 24 | 18 | ArraySeqMtEph.rs | 1309 | ninject_par | Span O(\|updates\|) | a.seq.clone() and result_vec.clone() ar… |
| 25 | 18 | ArraySeqMtEph.rs | 1474 | new | Span O(log length) | std::vec::from_elem is sequential: Span… |
| 26 | 18 | ArraySeqMtEph.rs | 1509 | subseq_copy | Span O(log length) | sequential clone loop: Span O(length) (… |
| 27 | 18 | ArraySeqMtEph.rs | 1536 | from_vec | Work O(n) worst case | wraps the Vec: Work O(1) (impl line 197… |
| 28 | 18 | ArraySeqMtEph.rs | 1578 | filter | Span O(lg n) | filter_dc splits with sequential subseq… |
| 29 | 18 | ArraySeqMtEph.rs | 1669 | reduce | Span O(lg n) | reduce_dc splits with sequential subseq… |
| 30 | 18 | ArraySeqMtEph.rs | 1700 | map | Span O(lg n + max S(f)) | map_dc splits with sequential subseq_co… |
| 31 | 18 | ArraySeqMtPer.rs | 106 | new | Span O(log length) | std::vec::from_elem is sequential: Span… |
| 32 | 18 | ArraySeqMtPer.rs | 132 | subseq_copy | Span O(log length) | sequential clone loop: Span O(length) (… |
| 33 | 18 | ArraySeqMtPer.rs | 159 | from_vec | Work O(n) worst case | wraps the Vec: Work O(1) (impl line 425… |
| 34 | 18 | ArraySeqMtPer.rs | 201 | filter | Span O(lg n) | filter_dc splits with sequential subseq… |
| 35 | 18 | ArraySeqMtPer.rs | 557 | inject | Work O(n), Span O(n) | copy loop plus update loop: O(n + m) (t… |
| 36 | 18 | ArraySeqMtPer.rs | 902 | map_par | Span O(log\|a\|) | subseq_copy and append are sequential O… |
| 37 | 18 | ArraySeqMtPer.rs | 1145 | filter_par | Span O(log\|a\|) | subseq_copy and append are sequential O… |
| 38 | 18 | ArraySeqMtPer.rs | 1244 | reduce_par | Span O(log\|a\|) | subseq_copy is sequential O(n) at the t… |
| 39 | 18 | ArraySeqMtPer.rs | 1561 | reduce_inner | Span O(log n) | subseq_copy is sequential O(n) at the t… |
| 40 | 18 | ArraySeqMtEphSlice.rs | 397 | tabulate | Span O(lg n * S(f)) | the rejoin copy loop at the top level i… |
| 41 | 18 | ArraySeqMtEphSlice.rs | 1416 | flatten | Span O(lg^2 \|a\| + max \|a[i]\|) | the top-level rejoin loop copies the ri… |

Full reasons per row (same numbering):

1. `ArraySeq.rs:316` `new`: annotation says Span O(1); std::vec::from_elem is sequential: Span O(length).
2. `ArraySeq.rs:500` `scan_inclusive`: annotation says Span O(1); sequential loop: Span O(n).
3. `ArraySeq.rs:514` `subseq_copy`: annotation says Span O(1); sequential clone loop: Span O(length).
4. `ArraySeq.rs:526` `remove`: annotation says Span O(1); Vec::remove shifts the tail: Span O(n).
5. `ArraySeq.rs:537` `insert`: annotation says Span O(1); Vec::insert shifts the tail: Span O(n).
6. `ArraySeq.rs:548` `from_vec`: annotation says Work O(n) worst case; wraps the Vec: Work O(1).
7. `ArraySeq.rs:978` `inject`: annotation says Work O(n), Span O(n); copy loop plus update loop: O(n + m) (trait line 486 says O(n + m)).
8. `ArraySeqStEph.rs:162` `from_vec`: annotation says Work O(n) worst case; wraps the Vec: Work O(1) (impl line 438 says O(1)).
9. `ArraySeqStEph.rs:600` `inject`: annotation says Work O(n), Span O(n); copy loop plus update loop: O(n + m) (trait line 243 says O(n + m)).
10. `ArraySeqStPer.rs:153` `from_vec`: annotation says Work O(n) worst case; wraps the Vec: Work O(1) (impl line 419 says O(1)).
11. `ArraySeqStPer.rs:581` `inject`: annotation says Work O(n), Span O(n); copy loop plus update loop: O(n + m) (trait line 234 says O(n + m)).
12. `LinkedListStEph.rs:109` `set`: annotation says Work O(index); Vec::set: Work O(1) (impl line 304 says O(1)).
13. `LinkedListStEph.rs:141` `from_vec`: annotation says Work O(n); wraps the Vec: Work O(1) (impl line 352 says O(1)).
14. `LinkedListStEph.rs:816` `clone`: annotation says Span O(1); Vec::clone is sequential: Span O(|self|).
15. `LinkedListStEph.rs:830` `eq`: annotation says Span O(1); Vec equality is sequential: Span O(|self|).
16. `LinkedListStPer.rs:133` `from_vec`: annotation says Work O(n); wraps the Vec: Work O(1) (impl line 334 says O(1)).
17. `LinkedListStPer.rs:798` `clone`: annotation says Span O(1); Vec::clone is sequential: Span O(|self|).
18. `LinkedListStPer.rs:812` `eq`: annotation says Span O(1); Vec equality is sequential: Span O(|self|).
19. `ArraySeqMtEph.rs:329` `inject`: annotation says Work O(n), Span O(n); copy loop plus update loop: O(n + m) (trait line 1617 says O(n + m)).
20. `ArraySeqMtEph.rs:651` `map_par`: annotation says Span O(log|a|); subseq_copy and append are sequential O(n) at the top level: Span O(n).
21. `ArraySeqMtEph.rs:706` `filter_par`: annotation says Span O(log|a|); subseq_copy and append are sequential O(n) at the top level: Span O(n).
22. `ArraySeqMtEph.rs:791` `reduce_par`: annotation says Span O(log|a|); subseq_copy is sequential O(n) at the top level: Span O(n).
23. `ArraySeqMtEph.rs:887` `reduce_dc`: annotation says Span O(log n); subseq_copy is sequential O(n) at the top level: Span O(n).
24. `ArraySeqMtEph.rs:1309` `ninject_par`: annotation says Span O(|updates|); a.seq.clone() and result_vec.clone() are sequential O(|a|): Span O(|a| + |updates|).
25. `ArraySeqMtEph.rs:1474` `new`: annotation says Span O(log length); std::vec::from_elem is sequential: Span O(length) (impl line 113 says O(n)).
26. `ArraySeqMtEph.rs:1509` `subseq_copy`: annotation says Span O(log length); sequential clone loop: Span O(length) (impl line 141 says O(j - i)).
27. `ArraySeqMtEph.rs:1536` `from_vec`: annotation says Work O(n) worst case; wraps the Vec: Work O(1) (impl line 197 says O(1)).
28. `ArraySeqMtEph.rs:1578` `filter`: annotation says Span O(lg n); filter_dc splits with sequential subseq_copy and rejoins with sequential append: Span O(n) (impl line 1108 says O(n)).
29. `ArraySeqMtEph.rs:1669` `reduce`: annotation says Span O(lg n); reduce_dc splits with sequential subseq_copy: Span O(n).
30. `ArraySeqMtEph.rs:1700` `map`: annotation says Span O(lg n + max S(f)); map_dc splits with sequential subseq_copy and rejoins with sequential append: Span O(n) (impl line 981 says O(n)).
31. `ArraySeqMtPer.rs:106` `new`: annotation says Span O(log length); std::vec::from_elem is sequential: Span O(length) (impl line 351 says O(n)).
32. `ArraySeqMtPer.rs:132` `subseq_copy`: annotation says Span O(log length); sequential clone loop: Span O(length) (impl line 369 says O(j - i)).
33. `ArraySeqMtPer.rs:159` `from_vec`: annotation says Work O(n) worst case; wraps the Vec: Work O(1) (impl line 425 says O(1)).
34. `ArraySeqMtPer.rs:201` `filter`: annotation says Span O(lg n); filter_dc splits with sequential subseq_copy and rejoins with sequential append: Span O(n) (impl line 494 says O(n)).
35. `ArraySeqMtPer.rs:557` `inject`: annotation says Work O(n), Span O(n); copy loop plus update loop: O(n + m) (trait line 240 says O(n + m)).
36. `ArraySeqMtPer.rs:902` `map_par`: annotation says Span O(log|a|); subseq_copy and append are sequential O(n) at the top level: Span O(n).
37. `ArraySeqMtPer.rs:1145` `filter_par`: annotation says Span O(log|a|); subseq_copy and append are sequential O(n) at the top level: Span O(n).
38. `ArraySeqMtPer.rs:1244` `reduce_par`: annotation says Span O(log|a|); subseq_copy is sequential O(n) at the top level: Span O(n); the annotation line itself is inside a `// Veracity: UNNEEDED proof block` comment prefix.
39. `ArraySeqMtPer.rs:1561` `reduce_inner`: annotation says Span O(log n); subseq_copy is sequential O(n) at the top level: Span O(n).
40. `ArraySeqMtEphSlice.rs:397` `tabulate`: annotation says Span O(lg n * S(f)); the rejoin copy loop at the top level is sequential O(n): Span O(n + S(f)).
41. `ArraySeqMtEphSlice.rs:1416` `flatten`: annotation says Span O(lg^2 |a| + max |a[i]|); the top-level rejoin loop copies the right half of all elements sequentially: Span O(sum |a[i]|).

Three patterns account for 38 of the 41: `from_vec` wraps its `Vec` in O(1)
but the trait line keeps an "O(n) worst case" (5 files); the trait-level
`inject` says O(n + m) while the impl line says O(n) for the same two-loop
body (5 files); and the Mt files' D&C helpers split with `subseq_copy` and
rejoin with `append`, both sequential O(n) loops, so their span is O(n), not
O(lg n) (14 lines across ArraySeqMtEph.rs, ArraySeqMtPer.rs and the Slice
file's `tabulate`/`flatten` rejoin loops). The Slice `reduce` (line 353) keeps
O(lg n): it splits with O(1) `slice` and combines with one `f` call.

### All Code-review annotations

| # | Chap | File | Line | Fn | Claim | Verdict |
|---|------|------|------|----|-------|---------|
| 1 | 18 | ArraySeq.rs | 316 | new | Work O(length), Span O(1). | disputed |
| 2 | 18 | ArraySeq.rs | 327 | set | Work O(1), Span O(1). | confirmed |
| 3 | 18 | ArraySeq.rs | 337 | length | Work O(1), Span O(1) | confirmed |
| 4 | 18 | ArraySeq.rs | 344 | nth | Work O(1), Span O(1) | confirmed |
| 5 | 18 | ArraySeq.rs | 351 | empty | Work O(1), Span O(1) | confirmed |
| 6 | 18 | ArraySeq.rs | 357 | singleton | Work O(1), Span O(1) | confirmed |
| 7 | 18 | ArraySeq.rs | 365 | subseq | Work O(j), Span O(j) (AD) | confirmed |
| 8 | 18 | ArraySeq.rs | 378 | append | Work O(\|a\| + \|b\|), Span O(\|a\| + \… | confirmed |
| 9 | 18 | ArraySeq.rs | 391 | filter | Work O(n), Span O(n) (AD) | confirmed |
| 10 | 18 | ArraySeq.rs | 414 | update | Work O(n), Span O(n) (AD) | confirmed |
| 11 | 18 | ArraySeq.rs | 427 | is_empty | Work O(1), Span O(1) | confirmed |
| 12 | 18 | ArraySeq.rs | 433 | is_singleton | Work O(1), Span O(1) | confirmed |
| 13 | 18 | ArraySeq.rs | 440 | iterate | Work O(n), Span O(n) (iterate is sequen… | confirmed |
| 14 | 18 | ArraySeq.rs | 451 | reduce | Work O(n), Span O(n) (AD) | confirmed |
| 15 | 18 | ArraySeq.rs | 465 | scan | Work O(n), Span O(n) (AD) | confirmed |
| 16 | 18 | ArraySeq.rs | 486 | inject | Work O(n + m), Span O(n + m) (AD) | confirmed |
| 17 | 18 | ArraySeq.rs | 500 | scan_inclusive | Work O(\|a\|), Span O(1). | disputed |
| 18 | 18 | ArraySeq.rs | 514 | subseq_copy | Work O(length), Span O(1). | disputed |
| 19 | 18 | ArraySeq.rs | 526 | remove | Work O(\|self\|), Span O(1). | disputed |
| 20 | 18 | ArraySeq.rs | 537 | insert | Work O(\|self\|), Span O(1). | disputed |
| 21 | 18 | ArraySeq.rs | 548 | from_vec | Work O(n) worst case, O(1) best case, S… | disputed |
| 22 | 18 | ArraySeq.rs | 555 | find_key | Work O(n), Span O(n) | confirmed |
| 23 | 18 | ArraySeq.rs | 575 | collect | Work O(n²), Span O(n²) (AD) | confirmed |
| 24 | 18 | ArraySeq.rs | 603 | new | Work O(n), Span O(n) | confirmed |
| 25 | 18 | ArraySeq.rs | 611 | set | Work O(1), Span O(1) | confirmed |
| 26 | 18 | ArraySeq.rs | 621 | length | Work O(1), Span O(1) | confirmed |
| 27 | 18 | ArraySeq.rs | 626 | nth | Work O(1), Span O(1) | confirmed |
| 28 | 18 | ArraySeq.rs | 631 | empty | Work O(1), Span O(1) | confirmed |
| 29 | 18 | ArraySeq.rs | 636 | singleton | Work O(1), Span O(1) | confirmed |
| 30 | 18 | ArraySeq.rs | 643 | subseq | Work O(j - i), Span O(j - i) | confirmed |
| 31 | 18 | ArraySeq.rs | 671 | append | Work O(\|a\| + \|b\|), Span O(\|a\| + \… | confirmed |
| 32 | 18 | ArraySeq.rs | 719 | filter | Work O(n), Span O(n) | confirmed |
| 33 | 18 | ArraySeq.rs | 775 | update | Work O(n), Span O(n) | confirmed |
| 34 | 18 | ArraySeq.rs | 815 | is_empty | Work O(1), Span O(1) | confirmed |
| 35 | 18 | ArraySeq.rs | 820 | is_singleton | Work O(1), Span O(1) | confirmed |
| 36 | 18 | ArraySeq.rs | 825 | iterate | Work O(n), Span O(n) | confirmed |
| 37 | 18 | ArraySeq.rs | 867 | reduce | Work O(n), Span O(n) | confirmed |
| 38 | 18 | ArraySeq.rs | 911 | scan | Work O(n), Span O(n) | confirmed |
| 39 | 18 | ArraySeq.rs | 978 | inject | Work O(n), Span O(n) | disputed |
| 40 | 18 | ArraySeq.rs | 1062 | scan_inclusive | Work O(n), Span O(n) | confirmed |
| 41 | 18 | ArraySeq.rs | 1119 | subseq_copy | Work O(j - i), Span O(j - i) | confirmed |
| 42 | 18 | ArraySeq.rs | 1150 | remove | Work O(n), Span O(n) | confirmed |
| 43 | 18 | ArraySeq.rs | 1160 | insert | Work O(n), Span O(n) | confirmed |
| 44 | 18 | ArraySeq.rs | 1170 | from_vec | Work O(1), Span O(1) | confirmed |
| 45 | 18 | ArraySeq.rs | 1176 | find_key | Work O(n), Span O(n) | confirmed |
| 46 | 18 | ArraySeq.rs | 1209 | collect | Work O(n), Span O(n) | confirmed |
| 47 | 18 | ArraySeq.rs | 1296 | map | Work O(n), Span O(n) (AD) | confirmed |
| 48 | 18 | ArraySeq.rs | 1327 | tabulate | Work O(n), Span O(n) (AD) | confirmed |
| 49 | 18 | ArraySeq.rs | 1361 | flatten | Work O(Σ\|a_i\|), Span O(Σ\|a_i\|) (A… | confirmed |
| 50 | 18 | ArraySeq.rs | 1421 | iterate_prefixes | Work O(n), Span O(n) | confirmed |
| 1 | 18 | ArraySeqStPer.rs | 100 | new | Work O(length), Span O(length) (AD) | confirmed |
| 2 | 18 | ArraySeqStPer.rs | 113 | length | Work O(1), Span O(1) | confirmed |
| 3 | 18 | ArraySeqStPer.rs | 120 | nth | Work O(1), Span O(1) | confirmed |
| 4 | 18 | ArraySeqStPer.rs | 126 | subseq_copy | Work O(length), Span O(length) (AD) | confirmed |
| 5 | 18 | ArraySeqStPer.rs | 140 | subseq | Work O(j), Span O(j) (AD) | confirmed |
| 6 | 18 | ArraySeqStPer.rs | 153 | from_vec | Work O(n) worst case, O(1) best case, S… | disputed |
| 7 | 18 | ArraySeqStPer.rs | 166 | empty | Work O(1), Span O(1) | confirmed |
| 8 | 18 | ArraySeqStPer.rs | 172 | singleton | Work O(1), Span O(1) | confirmed |
| 9 | 18 | ArraySeqStPer.rs | 181 | append | Work O(\|a\| + \|b\|), Span O(\|a\| + \… | confirmed |
| 10 | 18 | ArraySeqStPer.rs | 195 | filter | Work O(n), Span O(n) (AD) | confirmed |
| 11 | 18 | ArraySeqStPer.rs | 218 | update | Work O(n), Span O(n) (AD) | confirmed |
| 12 | 18 | ArraySeqStPer.rs | 234 | inject | Work O(n + m), Span O(n + m) (AD) | confirmed |
| 13 | 18 | ArraySeqStPer.rs | 249 | is_empty | Work O(1), Span O(1) | confirmed |
| 14 | 18 | ArraySeqStPer.rs | 255 | is_singleton | Work O(1), Span O(1) | confirmed |
| 15 | 18 | ArraySeqStPer.rs | 261 | iterate | Work O(n), Span O(n) (iterate is sequen… | confirmed |
| 16 | 18 | ArraySeqStPer.rs | 271 | reduce | Work O(n), Span O(n) (AD) | confirmed |
| 17 | 18 | ArraySeqStPer.rs | 284 | scan | Work O(n), Span O(n) (AD) | confirmed |
| 18 | 18 | ArraySeqStPer.rs | 301 | map | Work O(n), Span O(n) (AD) | confirmed |
| 19 | 18 | ArraySeqStPer.rs | 311 | tabulate | Work O(n), Span O(n) (AD) | confirmed |
| 20 | 18 | ArraySeqStPer.rs | 322 | flatten | Work O(Σ\|a_i\|), Span O(Σ\|a_i\|) (A… | confirmed |
| 21 | 18 | ArraySeqStPer.rs | 345 | new | Work O(n), Span O(n) | confirmed |
| 22 | 18 | ArraySeqStPer.rs | 353 | length | Work O(1), Span O(1) | confirmed |
| 23 | 18 | ArraySeqStPer.rs | 358 | nth | Work O(1), Span O(1) | confirmed |
| 24 | 18 | ArraySeqStPer.rs | 363 | subseq_copy | Work O(j - i), Span O(j - i) | confirmed |
| 25 | 18 | ArraySeqStPer.rs | 391 | subseq | Work O(j - i), Span O(j - i) | confirmed |
| 26 | 18 | ArraySeqStPer.rs | 419 | from_vec | Work O(1), Span O(1) | confirmed |
| 27 | 18 | ArraySeqStPer.rs | 427 | empty | Work O(1), Span O(1) | confirmed |
| 28 | 18 | ArraySeqStPer.rs | 432 | singleton | Work O(1), Span O(1) | confirmed |
| 29 | 18 | ArraySeqStPer.rs | 439 | append | Work O(\|a\| + \|b\|), Span O(\|a\| + \… | confirmed |
| 30 | 18 | ArraySeqStPer.rs | 488 | filter | Work O(n), Span O(n) | confirmed |
| 31 | 18 | ArraySeqStPer.rs | 541 | update | Work O(n), Span O(n) | confirmed |
| 32 | 18 | ArraySeqStPer.rs | 581 | inject | Work O(n), Span O(n) | disputed |
| 33 | 18 | ArraySeqStPer.rs | 660 | is_empty | Work O(1), Span O(1) | confirmed |
| 34 | 18 | ArraySeqStPer.rs | 665 | is_singleton | Work O(1), Span O(1) | confirmed |
| 35 | 18 | ArraySeqStPer.rs | 670 | iterate | Work O(n), Span O(n) | confirmed |
| 36 | 18 | ArraySeqStPer.rs | 709 | reduce | Work O(n), Span O(n) | confirmed |
| 37 | 18 | ArraySeqStPer.rs | 750 | scan | Work O(n), Span O(n) | confirmed |
| 38 | 18 | ArraySeqStPer.rs | 810 | map | Work O(n), Span O(n) | confirmed |
| 39 | 18 | ArraySeqStPer.rs | 831 | tabulate | Work O(n), Span O(n) | confirmed |
| 40 | 18 | ArraySeqStPer.rs | 850 | flatten | Work O(total length), Span O(total leng… | confirmed |
| 1 | 18 | ArraySeqStEph.rs | 100 | new | Work O(length), Span O(length) (AD) | confirmed |
| 2 | 18 | ArraySeqStEph.rs | 112 | set | Work O(1), Span O(1) | confirmed |
| 3 | 18 | ArraySeqStEph.rs | 122 | length | Work O(1), Span O(1) | confirmed |
| 4 | 18 | ArraySeqStEph.rs | 129 | nth | Work O(1), Span O(1) | confirmed |
| 5 | 18 | ArraySeqStEph.rs | 135 | subseq_copy | Work O(length), Span O(length) (AD) | confirmed |
| 6 | 18 | ArraySeqStEph.rs | 149 | subseq | Work O(j), Span O(j) (AD) | confirmed |
| 7 | 18 | ArraySeqStEph.rs | 162 | from_vec | Work O(n) worst case, O(1) best case, S… | disputed |
| 8 | 18 | ArraySeqStEph.rs | 175 | empty | Work O(1), Span O(1) | confirmed |
| 9 | 18 | ArraySeqStEph.rs | 181 | singleton | Work O(1), Span O(1) | confirmed |
| 10 | 18 | ArraySeqStEph.rs | 190 | append | Work O(\|a\| + \|b\|), Span O(\|a\| + \… | confirmed |
| 11 | 18 | ArraySeqStEph.rs | 204 | filter | Work O(n), Span O(n) (AD) | confirmed |
| 12 | 18 | ArraySeqStEph.rs | 227 | update | Work O(n), Span O(n) (AD) | confirmed |
| 13 | 18 | ArraySeqStEph.rs | 243 | inject | Work O(n + m), Span O(n + m) (AD) | confirmed |
| 14 | 18 | ArraySeqStEph.rs | 258 | is_empty | Work O(1), Span O(1) | confirmed |
| 15 | 18 | ArraySeqStEph.rs | 264 | is_singleton | Work O(1), Span O(1) | confirmed |
| 16 | 18 | ArraySeqStEph.rs | 270 | iterate | Work O(n), Span O(n) (iterate is sequen… | confirmed |
| 17 | 18 | ArraySeqStEph.rs | 280 | reduce | Work O(n), Span O(n) (AD) | confirmed |
| 18 | 18 | ArraySeqStEph.rs | 293 | scan | Work O(n), Span O(n) (AD) | confirmed |
| 19 | 18 | ArraySeqStEph.rs | 310 | map | Work O(n), Span O(n) (AD) | confirmed |
| 20 | 18 | ArraySeqStEph.rs | 320 | tabulate | Work O(n), Span O(n) (AD) | confirmed |
| 21 | 18 | ArraySeqStEph.rs | 331 | flatten | Work O(Σ\|a_i\|), Span O(Σ\|a_i\|) (A… | confirmed |
| 22 | 18 | ArraySeqStEph.rs | 354 | new | Work O(n), Span O(n) | confirmed |
| 23 | 18 | ArraySeqStEph.rs | 362 | set | Work O(1), Span O(1) | confirmed |
| 24 | 18 | ArraySeqStEph.rs | 372 | length | Work O(1), Span O(1) | confirmed |
| 25 | 18 | ArraySeqStEph.rs | 377 | nth | Work O(1), Span O(1) | confirmed |
| 26 | 18 | ArraySeqStEph.rs | 382 | subseq_copy | Work O(j - i), Span O(j - i) | confirmed |
| 27 | 18 | ArraySeqStEph.rs | 410 | subseq | Work O(j - i), Span O(j - i) | confirmed |
| 28 | 18 | ArraySeqStEph.rs | 438 | from_vec | Work O(1), Span O(1) | confirmed |
| 29 | 18 | ArraySeqStEph.rs | 446 | empty | Work O(1), Span O(1) | confirmed |
| 30 | 18 | ArraySeqStEph.rs | 451 | singleton | Work O(1), Span O(1) | confirmed |
| 31 | 18 | ArraySeqStEph.rs | 458 | append | Work O(\|a\| + \|b\|), Span O(\|a\| + \… | confirmed |
| 32 | 18 | ArraySeqStEph.rs | 507 | filter | Work O(n), Span O(n) | confirmed |
| 33 | 18 | ArraySeqStEph.rs | 560 | update | Work O(n), Span O(n) | confirmed |
| 34 | 18 | ArraySeqStEph.rs | 600 | inject | Work O(n), Span O(n) | disputed |
| 35 | 18 | ArraySeqStEph.rs | 679 | is_empty | Work O(1), Span O(1) | confirmed |
| 36 | 18 | ArraySeqStEph.rs | 684 | is_singleton | Work O(1), Span O(1) | confirmed |
| 37 | 18 | ArraySeqStEph.rs | 689 | iterate | Work O(n), Span O(n) | confirmed |
| 38 | 18 | ArraySeqStEph.rs | 728 | reduce | Work O(n), Span O(n) | confirmed |
| 39 | 18 | ArraySeqStEph.rs | 769 | scan | Work O(n), Span O(n) | confirmed |
| 40 | 18 | ArraySeqStEph.rs | 830 | map | Work O(n), Span O(n) | confirmed |
| 41 | 18 | ArraySeqStEph.rs | 851 | tabulate | Work O(n), Span O(n) | confirmed |
| 42 | 18 | ArraySeqStEph.rs | 870 | flatten | Work O(total length), Span O(total leng… | confirmed |
| 1 | 18 | LinkedListStPer.rs | 98 | new | Work O(n), Span O(n) (AD) | confirmed |
| 2 | 18 | LinkedListStPer.rs | 109 | length | Work O(1), Span O(1) | confirmed |
| 3 | 18 | LinkedListStPer.rs | 115 | nth | Work O(1), Span O(1) (AD) | confirmed |
| 4 | 18 | LinkedListStPer.rs | 121 | subseq_copy | Work O(j), Span O(j) (AD) | confirmed |
| 5 | 18 | LinkedListStPer.rs | 133 | from_vec | Work O(n), Span O(n) (AD) | disputed |
| 6 | 18 | LinkedListStPer.rs | 144 | empty | Work O(1), Span O(1) | confirmed |
| 7 | 18 | LinkedListStPer.rs | 148 | singleton | Work O(1), Span O(1) | confirmed |
| 8 | 18 | LinkedListStPer.rs | 156 | tabulate | Work O(n), Span O(n) | confirmed |
| 9 | 18 | LinkedListStPer.rs | 166 | map | Work O(n), Span O(n) | confirmed |
| 10 | 18 | LinkedListStPer.rs | 175 | append | Work O(\|a\| + \|b\|), Span O(\|a\| + \… | confirmed |
| 11 | 18 | LinkedListStPer.rs | 190 | filter | Work O(n), Span O(n) | confirmed |
| 12 | 18 | LinkedListStPer.rs | 208 | flatten | Work O(Σ\|a_i\|), Span O(Σ\|a_i\|) (A… | confirmed |
| 13 | 18 | LinkedListStPer.rs | 217 | update | Work O(n), Span O(n) (AD) | confirmed |
| 14 | 18 | LinkedListStPer.rs | 229 | is_empty | Work O(1), Span O(1) | confirmed |
| 15 | 18 | LinkedListStPer.rs | 233 | is_singleton | Work O(1), Span O(1) | confirmed |
| 16 | 18 | LinkedListStPer.rs | 237 | iterate | Work O(\|a\|), Span O(\|a\|) (AD) | confirmed |
| 17 | 18 | LinkedListStPer.rs | 246 | reduce | Work O(n), Span O(n) | confirmed |
| 18 | 18 | LinkedListStPer.rs | 258 | scan | Work O(n), Span O(n) | confirmed |
| 19 | 18 | LinkedListStPer.rs | 288 | new | Work O(n), Span O(n) | confirmed |
| 20 | 18 | LinkedListStPer.rs | 296 | length | Work O(1), Span O(1) | confirmed |
| 21 | 18 | LinkedListStPer.rs | 301 | nth | Work O(1), Span O(1) | confirmed |
| 22 | 18 | LinkedListStPer.rs | 306 | subseq_copy | Work O(j - i), Span O(j - i) | confirmed |
| 23 | 18 | LinkedListStPer.rs | 334 | from_vec | Work O(1), Span O(1) | confirmed |
| 24 | 18 | LinkedListStPer.rs | 342 | empty | Work O(1), Span O(1) | confirmed |
| 25 | 18 | LinkedListStPer.rs | 347 | singleton | Work O(1), Span O(1) | confirmed |
| 26 | 18 | LinkedListStPer.rs | 354 | tabulate | Work O(n), Span O(n) | confirmed |
| 27 | 18 | LinkedListStPer.rs | 373 | map | Work O(n), Span O(n) | confirmed |
| 28 | 18 | LinkedListStPer.rs | 394 | append | Work O(\|a\| + \|b\|), Span O(\|a\| + \… | confirmed |
| 29 | 18 | LinkedListStPer.rs | 442 | filter | Work O(n), Span O(n) | confirmed |
| 30 | 18 | LinkedListStPer.rs | 494 | flatten | Work O(total length), Span O(total leng… | confirmed |
| 31 | 18 | LinkedListStPer.rs | 549 | update | Work O(n), Span O(n) | confirmed |
| 32 | 18 | LinkedListStPer.rs | 588 | is_empty | Work O(1), Span O(1) | confirmed |
| 33 | 18 | LinkedListStPer.rs | 593 | is_singleton | Work O(1), Span O(1) | confirmed |
| 34 | 18 | LinkedListStPer.rs | 598 | iterate | Work O(n), Span O(n) | confirmed |
| 35 | 18 | LinkedListStPer.rs | 636 | reduce | Work O(n), Span O(n) | confirmed |
| 36 | 18 | LinkedListStPer.rs | 676 | scan | Work O(n), Span O(n) | confirmed |
| 37 | 18 | LinkedListStPer.rs | 745 | iter | Work O(1), Span O(1). | confirmed |
| 38 | 18 | LinkedListStPer.rs | 761 | into_iter | Work O(1), Span O(1). | confirmed |
| 39 | 18 | LinkedListStPer.rs | 775 | into_iter | Work O(1), Span O(1). | confirmed |
| 40 | 18 | LinkedListStPer.rs | 798 | clone | Work O(\|self\|), Span O(1). | disputed |
| 41 | 18 | LinkedListStPer.rs | 812 | eq | Work O(\|self\|), Span O(1). | disputed |
| 1 | 18 | LinkedListStEph.rs | 98 | new | Work O(n), Span O(n) (AD) | confirmed |
| 2 | 18 | LinkedListStEph.rs | 109 | set | Work O(index), Span O(index) (AD) | disputed |
| 3 | 18 | LinkedListStEph.rs | 117 | length | Work O(1), Span O(1) | confirmed |
| 4 | 18 | LinkedListStEph.rs | 123 | nth | Work O(1), Span O(1) (AD) | confirmed |
| 5 | 18 | LinkedListStEph.rs | 129 | subseq_copy | Work O(j), Span O(j) (AD) | confirmed |
| 6 | 18 | LinkedListStEph.rs | 141 | from_vec | Work O(n), Span O(n) (AD) | disputed |
| 7 | 18 | LinkedListStEph.rs | 152 | empty | Work O(1), Span O(1) | confirmed |
| 8 | 18 | LinkedListStEph.rs | 156 | singleton | Work O(1), Span O(1) | confirmed |
| 9 | 18 | LinkedListStEph.rs | 164 | tabulate | Work O(n), Span O(n) | confirmed |
| 10 | 18 | LinkedListStEph.rs | 174 | map | Work O(n), Span O(n) | confirmed |
| 11 | 18 | LinkedListStEph.rs | 183 | append | Work O(\|a\| + \|b\|), Span O(\|a\| + \… | confirmed |
| 12 | 18 | LinkedListStEph.rs | 198 | filter | Work O(n), Span O(n) | confirmed |
| 13 | 18 | LinkedListStEph.rs | 216 | flatten | Work O(Σ\|a_i\|), Span O(Σ\|a_i\|) (A… | confirmed |
| 14 | 18 | LinkedListStEph.rs | 225 | update | Work O(n), Span O(n) (AD) | confirmed |
| 15 | 18 | LinkedListStEph.rs | 237 | is_empty | Work O(1), Span O(1) | confirmed |
| 16 | 18 | LinkedListStEph.rs | 241 | is_singleton | Work O(1), Span O(1) | confirmed |
| 17 | 18 | LinkedListStEph.rs | 245 | iterate | Work O(\|a\|), Span O(\|a\|) (AD) | confirmed |
| 18 | 18 | LinkedListStEph.rs | 254 | reduce | Work O(n), Span O(n) | confirmed |
| 19 | 18 | LinkedListStEph.rs | 266 | scan | Work O(n), Span O(n) | confirmed |
| 20 | 18 | LinkedListStEph.rs | 296 | new | Work O(n), Span O(n) | confirmed |
| 21 | 18 | LinkedListStEph.rs | 304 | set | Work O(1), Span O(1) | confirmed |
| 22 | 18 | LinkedListStEph.rs | 314 | length | Work O(1), Span O(1) | confirmed |
| 23 | 18 | LinkedListStEph.rs | 319 | nth | Work O(1), Span O(1) | confirmed |
| 24 | 18 | LinkedListStEph.rs | 324 | subseq_copy | Work O(j - i), Span O(j - i) | confirmed |
| 25 | 18 | LinkedListStEph.rs | 352 | from_vec | Work O(1), Span O(1) | confirmed |
| 26 | 18 | LinkedListStEph.rs | 360 | empty | Work O(1), Span O(1) | confirmed |
| 27 | 18 | LinkedListStEph.rs | 365 | singleton | Work O(1), Span O(1) | confirmed |
| 28 | 18 | LinkedListStEph.rs | 372 | tabulate | Work O(n), Span O(n) | confirmed |
| 29 | 18 | LinkedListStEph.rs | 391 | map | Work O(n), Span O(n) | confirmed |
| 30 | 18 | LinkedListStEph.rs | 412 | append | Work O(\|a\| + \|b\|), Span O(\|a\| + \… | confirmed |
| 31 | 18 | LinkedListStEph.rs | 460 | filter | Work O(n), Span O(n) | confirmed |
| 32 | 18 | LinkedListStEph.rs | 512 | flatten | Work O(total length), Span O(total leng… | confirmed |
| 33 | 18 | LinkedListStEph.rs | 567 | update | Work O(n), Span O(n) | confirmed |
| 34 | 18 | LinkedListStEph.rs | 606 | is_empty | Work O(1), Span O(1) | confirmed |
| 35 | 18 | LinkedListStEph.rs | 611 | is_singleton | Work O(1), Span O(1) | confirmed |
| 36 | 18 | LinkedListStEph.rs | 616 | iterate | Work O(n), Span O(n) | confirmed |
| 37 | 18 | LinkedListStEph.rs | 654 | reduce | Work O(n), Span O(n) | confirmed |
| 38 | 18 | LinkedListStEph.rs | 694 | scan | Work O(n), Span O(n) | confirmed |
| 39 | 18 | LinkedListStEph.rs | 763 | iter | Work O(1), Span O(1). | confirmed |
| 40 | 18 | LinkedListStEph.rs | 779 | into_iter | Work O(1), Span O(1). | confirmed |
| 41 | 18 | LinkedListStEph.rs | 793 | into_iter | Work O(1), Span O(1). | confirmed |
| 42 | 18 | LinkedListStEph.rs | 816 | clone | Work O(\|self\|), Span O(1). | disputed |
| 43 | 18 | LinkedListStEph.rs | 830 | eq | Work O(\|self\|), Span O(1). | disputed |
| 1 | 18 | ArraySeqMtEph.rs | 113 | new | Work O(n), Span O(n) | confirmed |
| 2 | 18 | ArraySeqMtEph.rs | 121 | set | Work O(1), Span O(1) | confirmed |
| 3 | 18 | ArraySeqMtEph.rs | 131 | length | Work O(1), Span O(1) | confirmed |
| 4 | 18 | ArraySeqMtEph.rs | 136 | nth | Work O(1), Span O(1) | confirmed |
| 5 | 18 | ArraySeqMtEph.rs | 141 | subseq_copy | Work O(j - i), Span O(j - i) | confirmed |
| 6 | 18 | ArraySeqMtEph.rs | 169 | subseq | Work O(j - i), Span O(j - i) | confirmed |
| 7 | 18 | ArraySeqMtEph.rs | 197 | from_vec | Work O(1), Span O(1) | confirmed |
| 8 | 18 | ArraySeqMtEph.rs | 205 | empty | Work O(1), Span O(1) | confirmed |
| 9 | 18 | ArraySeqMtEph.rs | 210 | singleton | Work O(1), Span O(1) | confirmed |
| 10 | 18 | ArraySeqMtEph.rs | 217 | append | Work O(\|a\| + \|b\|), Span O(\|a\| + \… | confirmed |
| 11 | 18 | ArraySeqMtEph.rs | 266 | filter | Work O(n), Span O(n) | confirmed |
| 12 | 18 | ArraySeqMtEph.rs | 289 | update | Work O(n), Span O(n) | confirmed |
| 13 | 18 | ArraySeqMtEph.rs | 329 | inject | Work O(n), Span O(n) | disputed |
| 14 | 18 | ArraySeqMtEph.rs | 409 | ninject | Work O(n), Span O(n) | confirmed |
| 15 | 18 | ArraySeqMtEph.rs | 436 | is_empty | Work O(1), Span O(1) | confirmed |
| 16 | 18 | ArraySeqMtEph.rs | 441 | is_singleton | Work O(1), Span O(1) | confirmed |
| 17 | 18 | ArraySeqMtEph.rs | 446 | iterate | Work O(n), Span O(n) | confirmed |
| 18 | 18 | ArraySeqMtEph.rs | 485 | reduce | Work O(n), Span O(n) | confirmed |
| 19 | 18 | ArraySeqMtEph.rs | 492 | scan | Work O(n), Span O(n) | confirmed |
| 20 | 18 | ArraySeqMtEph.rs | 552 | map | Work O(n), Span O(n) | confirmed |
| 21 | 18 | ArraySeqMtEph.rs | 559 | tabulate | Work O(n), Span O(n) | confirmed |
| 22 | 18 | ArraySeqMtEph.rs | 578 | flatten | Work O(total length), Span O(total leng… | confirmed |
| 23 | 18 | ArraySeqMtEph.rs | 651 | map_par | Work O(\|a\|), Span O(log\|a\|). | disputed |
| 24 | 18 | ArraySeqMtEph.rs | 706 | filter_par | Work O(\|a\|), Span O(log\|a\|). | disputed |
| 25 | 18 | ArraySeqMtEph.rs | 791 | reduce_par | Work O(\|a\|), Span O(log\|a\|). | disputed |
| 26 | 18 | ArraySeqMtEph.rs | 887 | reduce_dc | Work O(n), Span O(log n) | disputed |
| 27 | 18 | ArraySeqMtEph.rs | 981 | map_dc | Work O(n), Span O(n) | confirmed |
| 28 | 18 | ArraySeqMtEph.rs | 1108 | filter_dc | Work O(n), Span O(n) | confirmed |
| 29 | 18 | ArraySeqMtEph.rs | 1309 | ninject_par | Work O(\|a\| + \|updates\|), Span O(\|u… | disputed |
| 30 | 18 | ArraySeqMtEph.rs | 1474 | new | Work O(length), Span O(log length). | disputed |
| 31 | 18 | ArraySeqMtEph.rs | 1486 | set | Work O(1), Span O(1). | confirmed |
| 32 | 18 | ArraySeqMtEph.rs | 1496 | length | Work O(1), Span O(1) | confirmed |
| 33 | 18 | ArraySeqMtEph.rs | 1503 | nth | Work O(1), Span O(1) | confirmed |
| 34 | 18 | ArraySeqMtEph.rs | 1509 | subseq_copy | Work O(length), Span O(log length). | disputed |
| 35 | 18 | ArraySeqMtEph.rs | 1523 | subseq | Work O(j), Span O(j) (AD) | confirmed |
| 36 | 18 | ArraySeqMtEph.rs | 1536 | from_vec | Work O(n) worst case, O(1) best case, S… | disputed |
| 37 | 18 | ArraySeqMtEph.rs | 1549 | empty | Work O(1), Span O(1) | confirmed |
| 38 | 18 | ArraySeqMtEph.rs | 1555 | singleton | Work O(1), Span O(1) | confirmed |
| 39 | 18 | ArraySeqMtEph.rs | 1564 | append | Work O(\|a\| + \|b\|), Span O(\|a\| + \… | confirmed |
| 40 | 18 | ArraySeqMtEph.rs | 1578 | filter | Work O(n), Span O(lg n) (AD) | disputed |
| 41 | 18 | ArraySeqMtEph.rs | 1601 | update | Work O(n), Span O(n) (AD) | confirmed |
| 42 | 18 | ArraySeqMtEph.rs | 1617 | inject | Work O(n + m), Span O(n + m) (AD) | confirmed |
| 43 | 18 | ArraySeqMtEph.rs | 1633 | ninject | Work O(n + m), Span O(n + m) (AD) | confirmed |
| 44 | 18 | ArraySeqMtEph.rs | 1647 | is_empty | Work O(1), Span O(1) | confirmed |
| 45 | 18 | ArraySeqMtEph.rs | 1653 | is_singleton | Work O(1), Span O(1) | confirmed |
| 46 | 18 | ArraySeqMtEph.rs | 1659 | iterate | Work O(n), Span O(n) (iterate is sequen… | confirmed |
| 47 | 18 | ArraySeqMtEph.rs | 1669 | reduce | Work O(n), Span O(lg n) (parallel divid… | disputed |
| 48 | 18 | ArraySeqMtEph.rs | 1683 | scan | Work O(n), Span O(n) (AD) | confirmed |
| 49 | 18 | ArraySeqMtEph.rs | 1700 | map | Work O(n), Span O(lg n + max S(f)) (AD) | disputed |
| 50 | 18 | ArraySeqMtEph.rs | 1713 | tabulate | Work O(n), Span O(n) (AD) | confirmed |
| 51 | 18 | ArraySeqMtEph.rs | 1726 | flatten | Work O(Σ\|a_i\|), Span O(Σ\|a_i\|) (A… | confirmed |
| 52 | 18 | ArraySeqMtEph.rs | 1740 | apply_ninject_updates | Work O(\|updates\|), Span O(\|updates\|… | confirmed |
| 1 | 18 | ArraySeqMtEphSlice.rs | 273 | length | Work O(1), Span O(1) | confirmed |
| 2 | 18 | ArraySeqMtEphSlice.rs | 278 | nth_cloned | Work O(1), Span O(1) — array index + … | confirmed |
| 3 | 18 | ArraySeqMtEphSlice.rs | 286 | slice | Work O(1), Span O(1) — Arc clone + wi… | confirmed |
| 4 | 18 | ArraySeqMtEphSlice.rs | 299 | from_vec | Work O(1), Span O(1) | confirmed |
| 5 | 18 | ArraySeqMtEphSlice.rs | 308 | empty | Work O(1), Span O(1) | confirmed |
| 6 | 18 | ArraySeqMtEphSlice.rs | 315 | singleton | Work O(1), Span O(1) | confirmed |
| 7 | 18 | ArraySeqMtEphSlice.rs | 323 | new | Work O(n), Span O(n) | confirmed |
| 8 | 18 | ArraySeqMtEphSlice.rs | 335 | to_vec | Work O(n), Span O(n) | confirmed |
| 9 | 18 | ArraySeqMtEphSlice.rs | 353 | reduce | Work O(n), Span O(lg n) — D&C + join,… | confirmed |
| 10 | 18 | ArraySeqMtEphSlice.rs | 367 | map | Work O(n), Span O(n) — D&C + join, O(… | confirmed |
| 11 | 18 | ArraySeqMtEphSlice.rs | 382 | filter | Work O(n), Span O(n) — D&C + join, O(… | confirmed |
| 12 | 18 | ArraySeqMtEphSlice.rs | 397 | tabulate | Work O(n * W(f)), Span O(lg n * S(f)) �… | disputed |
| 13 | 18 | ArraySeqMtEphSlice.rs | 410 | scan | Work O(n lg n), Span O(n) — D&C + joi… | confirmed |
| 14 | 18 | ArraySeqMtEphSlice.rs | 428 | is_empty | Work O(1), Span O(1) | confirmed |
| 15 | 18 | ArraySeqMtEphSlice.rs | 434 | is_singleton | Work O(1), Span O(1) | confirmed |
| 16 | 18 | ArraySeqMtEphSlice.rs | 440 | append | Work O(\|a\| + \|b\|), Span O(\|a\| + \… | confirmed |
| 17 | 18 | ArraySeqMtEphSlice.rs | 456 | update | Work O(n), Span O(n) | confirmed |
| 18 | 18 | ArraySeqMtEphSlice.rs | 470 | inject | Work O(n + m), Span O(n + m) | confirmed |
| 19 | 18 | ArraySeqMtEphSlice.rs | 484 | ninject | Work O(n + m), Span O(n + m) | confirmed |
| 20 | 18 | ArraySeqMtEphSlice.rs | 519 | length | Work O(1), Span O(1) | confirmed |
| 21 | 18 | ArraySeqMtEphSlice.rs | 524 | nth_cloned | Work O(1), Span O(1) | confirmed |
| 22 | 18 | ArraySeqMtEphSlice.rs | 530 | slice | Work O(1), Span O(1) | confirmed |
| 23 | 18 | ArraySeqMtEphSlice.rs | 540 | from_vec | Work O(1), Span O(1) | confirmed |
| 24 | 18 | ArraySeqMtEphSlice.rs | 550 | empty | Work O(1), Span O(1) | confirmed |
| 25 | 18 | ArraySeqMtEphSlice.rs | 559 | singleton | Work O(1), Span O(1) | confirmed |
| 26 | 18 | ArraySeqMtEphSlice.rs | 570 | new | Work O(n), Span O(n) | confirmed |
| 27 | 18 | ArraySeqMtEphSlice.rs | 593 | to_vec | Work O(n), Span O(n) | confirmed |
| 28 | 18 | ArraySeqMtEphSlice.rs | 694 | is_empty | Work O(1), Span O(1) | confirmed |
| 29 | 18 | ArraySeqMtEphSlice.rs | 699 | is_singleton | Work O(1), Span O(1) | confirmed |
| 30 | 18 | ArraySeqMtEphSlice.rs | 704 | append | Work O(\|a\| + \|b\|), Span O(\|a\| + \… | confirmed |
| 31 | 18 | ArraySeqMtEphSlice.rs | 744 | update | Work O(n), Span O(n) | confirmed |
| 32 | 18 | ArraySeqMtEphSlice.rs | 763 | inject | Work O(n + m), Span O(n + m) | confirmed |
| 33 | 18 | ArraySeqMtEphSlice.rs | 843 | ninject | Work O(n + m), Span O(n + m) | confirmed |
| 34 | 18 | ArraySeqMtEphSlice.rs | 1416 | flatten | Work O(sum \|a[i]\|), Span O(lg^2 \|a\|… | disputed |
| 1 | 18 | ArraySeqMtPer.rs | 106 | new | Work O(length), Span O(log length). | disputed |
| 2 | 18 | ArraySeqMtPer.rs | 119 | length | Work O(1), Span O(1) | confirmed |
| 3 | 18 | ArraySeqMtPer.rs | 126 | nth | Work O(1), Span O(1) | confirmed |
| 4 | 18 | ArraySeqMtPer.rs | 132 | subseq_copy | Work O(length), Span O(log length). | disputed |
| 5 | 18 | ArraySeqMtPer.rs | 146 | subseq | Work O(j), Span O(j) (AD) | confirmed |
| 6 | 18 | ArraySeqMtPer.rs | 159 | from_vec | Work O(n) worst case, O(1) best case, S… | disputed |
| 7 | 18 | ArraySeqMtPer.rs | 172 | empty | Work O(1), Span O(1) | confirmed |
| 8 | 18 | ArraySeqMtPer.rs | 178 | singleton | Work O(1), Span O(1) | confirmed |
| 9 | 18 | ArraySeqMtPer.rs | 187 | append | Work O(\|a\| + \|b\|), Span O(\|a\| + \… | confirmed |
| 10 | 18 | ArraySeqMtPer.rs | 201 | filter | Work O(n), Span O(lg n) (AD) | disputed |
| 11 | 18 | ArraySeqMtPer.rs | 224 | update | Work O(n), Span O(n) (AD) | confirmed |
| 12 | 18 | ArraySeqMtPer.rs | 240 | inject | Work O(n + m), Span O(n + m) (AD) | confirmed |
| 13 | 18 | ArraySeqMtPer.rs | 255 | is_empty | Work O(1), Span O(1) | confirmed |
| 14 | 18 | ArraySeqMtPer.rs | 261 | is_singleton | Work O(1), Span O(1) | confirmed |
| 15 | 18 | ArraySeqMtPer.rs | 267 | iterate | Work O(n), Span O(n) (iterate is sequen… | confirmed |
| 16 | 18 | ArraySeqMtPer.rs | 277 | reduce | Work O(n), Span O(n) (AD) | confirmed |
| 17 | 18 | ArraySeqMtPer.rs | 290 | scan | Work O(n), Span O(n) (AD) | confirmed |
| 18 | 18 | ArraySeqMtPer.rs | 307 | map | Work O(n), Span O(n) (AD) | confirmed |
| 19 | 18 | ArraySeqMtPer.rs | 317 | tabulate | Work O(n), Span O(n) (AD) | confirmed |
| 20 | 18 | ArraySeqMtPer.rs | 328 | flatten | Work O(Σ\|a_i\|), Span O(Σ\|a_i\|) (A… | confirmed |
| 21 | 18 | ArraySeqMtPer.rs | 351 | new | Work O(n), Span O(n) | confirmed |
| 22 | 18 | ArraySeqMtPer.rs | 359 | length | Work O(1), Span O(1) | confirmed |
| 23 | 18 | ArraySeqMtPer.rs | 364 | nth | Work O(1), Span O(1) | confirmed |
| 24 | 18 | ArraySeqMtPer.rs | 369 | subseq_copy | Work O(j - i), Span O(j - i) | confirmed |
| 25 | 18 | ArraySeqMtPer.rs | 397 | subseq | Work O(j - i), Span O(j - i) | confirmed |
| 26 | 18 | ArraySeqMtPer.rs | 425 | from_vec | Work O(1), Span O(1) | confirmed |
| 27 | 18 | ArraySeqMtPer.rs | 433 | empty | Work O(1), Span O(1) | confirmed |
| 28 | 18 | ArraySeqMtPer.rs | 438 | singleton | Work O(1), Span O(1) | confirmed |
| 29 | 18 | ArraySeqMtPer.rs | 445 | append | Work O(\|a\| + \|b\|), Span O(\|a\| + \… | confirmed |
| 30 | 18 | ArraySeqMtPer.rs | 494 | filter | Work O(n), Span O(n) | confirmed |
| 31 | 18 | ArraySeqMtPer.rs | 517 | update | Work O(n), Span O(n) | confirmed |
| 32 | 18 | ArraySeqMtPer.rs | 557 | inject | Work O(n), Span O(n) | disputed |
| 33 | 18 | ArraySeqMtPer.rs | 636 | is_empty | Work O(1), Span O(1) | confirmed |
| 34 | 18 | ArraySeqMtPer.rs | 641 | is_singleton | Work O(1), Span O(1) | confirmed |
| 35 | 18 | ArraySeqMtPer.rs | 646 | iterate | Work O(n), Span O(n) | confirmed |
| 36 | 18 | ArraySeqMtPer.rs | 685 | reduce | Work O(n), Span O(n) | confirmed |
| 37 | 18 | ArraySeqMtPer.rs | 726 | scan | Work O(n), Span O(n) | confirmed |
| 38 | 18 | ArraySeqMtPer.rs | 786 | map | Work O(n), Span O(n) | confirmed |
| 39 | 18 | ArraySeqMtPer.rs | 807 | tabulate | Work O(n), Span O(n) | confirmed |
| 40 | 18 | ArraySeqMtPer.rs | 826 | flatten | Work O(total length), Span O(total leng… | confirmed |
| 41 | 18 | ArraySeqMtPer.rs | 902 | map_par | Work O(\|a\|), Span O(log\|a\|). | disputed |
| 42 | 18 | ArraySeqMtPer.rs | 963 | filter_dc | Work O(n), Span O(n) | confirmed |
| 43 | 18 | ArraySeqMtPer.rs | 1145 | filter_par | Work O(\|a\|), Span O(log\|a\|). | disputed |
| 44 | 18 | ArraySeqMtPer.rs | 1244 | reduce_par | Work O(\|a\|), Span O(log\|a\|). | disputed |
| 45 | 18 | ArraySeqMtPer.rs | 1345 | map_inner | Work O(n), Span O(n) | confirmed |
| 46 | 18 | ArraySeqMtPer.rs | 1450 | filter_inner | Work O(n), Span O(n) | confirmed |
| 47 | 18 | ArraySeqMtPer.rs | 1561 | reduce_inner | Work O(n), Span O(log n) | disputed |
| 48 | 18 | ArraySeqMtPer.rs | 1651 | tabulate_inner | Work O(n), Span O(n) | confirmed |

"(AD)" abbreviates an "ACCEPTED DIFFERENCE" tail on the source line.

## 8. Patch series

Sixteen commits in the fixture, one per file, from baseline `c520913`, emitted
with `git format-patch` into `plans/r210-chap18-patches/`. Each was checked
with `git apply --check` on the live tree (`~/projects/APAS-VERUS`, whose
uncommitted working tree is the fixture baseline): all sixteen apply.

| # | Chap | Patch | File |
|---|------|-------|------|
| 1 | 18 | 0001-…-ArraySeq.rs.patch | src/Chap18/ArraySeq.rs |
| 2 | 18 | 0002-…-ArraySeqStPer.patch | src/Chap18/ArraySeqStPer.rs |
| 3 | 18 | 0003-…-ArraySeqStEph.patch | src/Chap18/ArraySeqStEph.rs |
| 4 | 18 | 0004-…-LinkedListStP.patch | src/Chap18/LinkedListStPer.rs |
| 5 | 18 | 0005-…-LinkedListStE.patch | src/Chap18/LinkedListStEph.rs |
| 6 | 18 | 0006-…-ArraySeqMtEph.patch | src/Chap18/ArraySeqMtEph.rs |
| 7 | 18 | 0007-…-ArraySeqMtEph.patch | src/Chap18/ArraySeqMtEphSlice.rs (with hand fix 1) |
| 8 | 18 | 0008-…-ArraySeqMtPer.patch | src/Chap18/ArraySeqMtPer.rs |
| 9 | 18 | 0009-…-tests-C.patch | rust_verify_test/tests/Chap18/ProveArraySeq.rs |
| 10 | 18 | 0010-…-tests-C.patch | …/ProveArraySeqStPer.rs |
| 11 | 18 | 0011-…-tests-C.patch | …/ProveArraySeqStEph.rs |
| 12 | 18 | 0012-…-tests-C.patch | …/ProveLinkedListStPer.rs |
| 13 | 18 | 0013-…-tests-C.patch | …/ProveLinkedListStEph.rs |
| 14 | 18 | 0014-…-tests-C.patch | …/ProveArraySeqMtEph.rs |
| 15 | 18 | 0015-…-tests-C.patch | …/ProveArraySeqMtEphSlice.rs |
| 16 | 18 | 0016-…-tests-C.patch | …/ProveArraySeqMtPer.rs |

Apply in order with `git am plans/r210-chap18-patches/*.patch` (or `git
apply` each). Patch 7 carries the one hand fix; the other fifteen are the
tool's output unchanged.

## 9. What the tool cannot yet do

- Chained iterators (`a.iter().map(..)`, `.enumerate()`, `.zip(..)`): the
  transform recognises only a wrapper struct whose single `inner` field is
  `std::slice::Iter` or `std::vec::IntoIter`; any other field type is the
  `custom-iterator` residual and the module is left alone.
- Custom iterators (a hand-written `next` with its own state, as in the
  wrapping_iterators standard): reported, not rewritten, since the prophetic
  spec of such an iterator is a proof, not a substitution.
- Consumer loops with `break` (class (d)): a manual `loop`/`while` over a
  wrapped iterator whose invariants name `it@.0`, `it@.1`, `iter_invariant(&it)`,
  `.pos` or `.elements` is reported (`manual-loop`, `loop-with-break`); the
  five class (d) substitutions are implemented for clause tokens but the
  conclusion `assert` before a `break` and the `VerusForLoopWrapper` rebinding
  are not generated. Chap18 has no such loop, so all class (d) counts are 0.
- Proof-time tests outside the six standard patterns (`ptt-unknown-pattern`),
  or whose body has no `let x: Collection<..>` binding (`ptt-no-binding`), are
  reported and left as they were.
- The `requires` on an external-trait `IntoIterator` impl (hand fix 1) is a
  specification decision the tool only reports.
- Comments are never edited, so old-model names in doc lines and test headers
  remain.
