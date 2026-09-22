# Iterator transformer, waves 1 and 2 (r212)

Date: 2026-09-22. Plan: `plans/r212-iterator-transformer-wave1-2.md`.
Tool: `~/projects/CSTs/processes/iterator-upgrade{,-lib}` (CSTs round r0728).
Fixture: `~/projects/CSTs/processes/iterator-upgrade/tests/fixtures/APAS-VERUS`,
a copy of this tree cut at fixture commit `fea1594` (the baseline; `diff -rq`
against the live tree was empty). Nothing in this tree was edited except this
document and `plans/r212-patches/`. Verus 0.2026.09.13.671956e.

Every log named below is under the fixture's `logs/`. The fixture has no
`cargo-nextest`, so run-time tests (RTT) ran through `scratch/r212-rtt.sh`
(`cargo test --release --no-fail-fast` on the chapter's `[[test]]` targets)
and proof-time tests (PTT) through `scratch/r212-ptt.sh` (compile the library
with the isolate cfg flags, then `cargo test --release` in `rust_verify_test`).

## 1. Results

| # | Chap | validate isolate | err | warn | RTT | PTT | logs (validate / RTT / PTT) |
|---|------|------------------|----:|-----:|-----|-----|-----------------------------|
| 1 | 18 | 1003 verified | 0 | 0 | 170 pass | 38 pass | 20260922-044407 / 20260921-165024 / ptt-Chap18.20260921-165055 |
| 2 | 19 | 824 verified | 0 | 0 | 156 pass | 23 pass | 20260922-045056 / 20260921-170139 / ptt-Chap19.20260921-170157 |
| 3 | 23 | 679 verified | 0 | 0 | 92 pass | 17 pass | 20260922-045100 / 20260921-170627 / ptt-Chap23.20260921-170639 |
| 4 | 37 | 1862 verified | 0 | 0 | 544 pass | 24 pass | 20260922-045027 / 20260921-174604 / ptt-Chap37.20260921-185845 |
| 5 | 38 | 1079 verified | 0 | 111 (1) | 53 pass | 2 pass | 20260922-045040 / 20260921-190203 / ptt-Chap38.20260921-190126 |
| 6 | 39 | 1221 verified | 0 | 139 (1) | 148 pass | 8 pass | 20260922-045047 / 20260921-190601 / ptt-Chap39.20260921-190608 |
| 7 | 40 | 1180 verified | 0 | 0 | 54 pass | 6 pass | 20260922-044658 / 20260921-191253 / ptt-Chap40.20260921-191301 |
| 8 | 57 | in Chap59 run (2) | 0 | 6 (1) | 48 pass | none exist | 20260922-044413 / 20260922-044548 / — |
| 9 | 58 | 1368 verified | 2 (3) | 0 | 41 pass (4) | none exist | 20260922-044447 / (4) / — |
| 10 | 59 | 2631 verified | 0 | 6 (1) | 41 pass | none exist | 20260922-044413 / 20260922-044554 / — |

Trigger notes: 0 in every run above.

1. Every warning is `use of deprecated method vstd::set::Set::<A>::finite:
   Every Set is always finite`, on pre-existing `x@.finite()` conjuncts in
   `BSTPara{St,Mt}Eph.rs` (Chap38), the treap files (Chap39), and
   `DijkstraStEph{U64,F64}.rs` (Chap57). They come from the verus upgrade, not
   from the iterator migration. Removing them means editing `requires` and
   `ensures` in 250 places outside this round's scope, so they are left.
2. `validate isolate Chap57` (log `20260922-043920`) verified every Chap57
   function but hit an rlimit in `insert_at_link` of Chap37 `AVLTreeSeq.rs`.
   That function now carries `#[verifier::rlimit(20)]`. The isolate Chap59
   run covers Chap57's closure (Chap57, 45, 37, 18, 19, 23, 06, 05, 02, 56)
   and verifies with 0 errors.
3. Chap58 remaining, exact text (isolate Chap58: Chap19, 06, 05, 02, 58, 56):
   ```
   error: postcondition not satisfied
      --> src/Chap19/ArraySeqStEph.rs:165:21
   165 |             ensures len as int == self.spec_len();
   447 |             self.seq.len()
   error: postcondition not satisfied
      --> src/Chap19/ArraySeqStEph.rs:173:21
   173 |             ensures *nth_elem == self.spec_index(index as int);
   452 |             &self.seq[index]
   error: precondition not satisfied
      --> src/Chap19/ArraySeqStEph.rs:452:14
   verification results:: 1368 verified, 2 errors
   ```
   Every Chap58 function verifies; the two failures are `length` and `nth` of
   Chap19 `ArraySeqStEph.rs`, whose bodies r212 did not touch. The same
   functions verify in isolate Chap19 (824/0), isolate Chap56 (948/0), and
   isolate Chap59 (2631/0), whose closure contains all of Chap58's. The
   failure shows only for this module set, and it reproduces on every run.
   The goal is `self.seq.len() == self.seq@.len()`, which is Vec's own
   `ensures`. That points at context-dependent resolution of the trait's
   `open spec fn spec_len`, not at the proof. I found no cause. It is left
   open.
4. Chap58's RTT log was overwritten by Chap59's, because both runs started in
   the same second (`rtt.20260922-044554.log`). The Chap58 run printed
   `13 passed` and `28 passed`, `cargo test rc: 0` (console capture
   `scratch/rtt-Chap58-a.txt`).

## 2. The IntoIterator decision

### 2.1 Step 1: form B

Verus 0.2026.09.13 rejects a `requires` on an external trait's method, and
47 `IntoIterator::into_iter` impls in 42 files carried one. Step 1 tried three
forms in `src/experiments/`. All three verify:

| # | Form | Experiment | Verified | Log |
|---|------|------------|---------:|-----|
| 1 | A: `#[verifier::type_invariant]` | `intoiter_form_a_type_invariant.rs` | 8 | `validate-standard-intoiter_form_a_type_invariant.20260921-164420` |
| 2 | B: total body, conditional `ensures` | `intoiter_form_b_total_body.rs` | 7 | `validate-standard-intoiter_form_b_total_body.20260921-164442` |
| 3 | C: no impl, callers use `iter()` | `intoiter_form_c_no_impl.rs` | 4 | `validate-standard-intoiter_form_c_no_impl.20260921-164448` |

- **Form B** was chosen because it is the smallest edit. Each impl gets an
  O(1) exec bounds check that returns an empty iterator when the check fails,
  and a `wf ==> triple` `ensures`. That is 2 exec lines and 1 premise per
  impl, and 0 call-site edits.
- **Form A** needs private fields, a lemma call, and a loop invariant at
  every consumer.
- **Form C** would change 266 PTT `(&a).into_iter()` sites, 46 RTT `for x in
  &coll` loops, and 16 `src` sites.

Form B went onto the two impls where it is possible: `IntoIterator for
&ArraySeqMtEphSliceS` in Chap18 and in Chap19. Their `requires` is a range
fact (`start + len <= data.len()`) that the body can check in O(1).

### 2.2 Why Chap37-40 used form C

Form B needs a precondition that the body can **check** in O(1) exec code.
None of the 30 `requires` on the Chap37-40 impls can be checked that way:

- Tree well-formedness (`spec_bstplainsteph_wf()`, `spec_avltreeseq_wf()`,
  `spec_bsttreapsteph_wf()`, …) is a recursive spec over the whole tree.
  Checking it in exec is O(n), and a body that returns an empty iterator on a
  malformed tree would still need the fact to prove anything.
- `obeys_feq_clone::<T>()`, `obeys_cmp::<T>()`, and
  `view_ord_consistent::<T>()` are facts about the type `T`. No exec check
  exists for them, and they cannot be a `type_invariant` (form A), because
  they are not properties of the value.

For those impls form B would be O(n) (breaking cost preservation) or
unprovable. So they took form C, the one experiment whose shape still
applied. That form is the tool's `--into-iter c` option.

What form C commented out:

| # | Item | Count | Where |
|---|------|------:|-------|
| 1 | `IntoIterator` impls (block comment under an r212 note) | 30 | Chap37 23 (19 by the tool, 4 by hand in the AVLTreeSeq files), Chap38 1, Chap39 3, Chap40 3 |
| 2 | PTT tests commented out (`*_borrow_into` / `*_consume`) | 30 | Chap37 28 (12 files), Chap38 2 |
| 3 | PTT tests rewritten from `borrow_into` to `borrow_iter` | 14 | Chap37 `ProveBSTSplayStEph.rs` 2, Chap39 6 (3 files), Chap40 6 (3 files) |
| 4 | RTT call sites changed to `.iter()` | 24 lines in 8 files | Chap37 1, Chap38 1, Chap39 4, Chap40 3 files |
| 5 | new inherent `iter()` fns | 7 | modules whose only entry point was the impl: `BSTSplayStEph`, `ParamTreap`, `BSTSetTreapMtEph`, `BSTTreapStEph`, `BSTSizeStEph`, `BSTKeyValueStEph`, `BSTReducedStEph` |
| 6 | `iter()` bodies that delegated to the impl, inlined | 4 | Chap37 `BST{Plain,AVL,BBAlpha,RB}StEph.rs` (tool class `inline-into-iter-body`) |

Each commented impl keeps its full text under a note naming its original
`requires` and pointing at `src/experiments/intoiter_form_c_no_impl.rs`, so it
can be restored when Verus allows a `requires` there. `iter()` keeps the
`requires`. Every RTT change is `(&x).into_iter()` → `x.iter()` or `for v in
&x` → `for v in x.iter()`, which does the same work.

## 3. The scan hole

The authorised hole has been **removed**. It was
`crate::vstdplus::accept::accept(acc == s.take(i as int + 1).fold_left(id,
spec_f))` in the `scan` loop of Chap18 `ArraySeqStEph.rs`. That was the form
in commit `9d30219`. Chap18 then verified 1000/0, and the four original lines
were kept as `// BYPASSED (r212):` comments.

The real proof that replaced it is a free lemma, placed in each file that
needs it:

```rust
proof fn lemma_take_fold_left_step<T>(s: Seq<T>, i: int, id: T, f: spec_fn(T, T) -> T)
    requires 0 <= i < s.len(),
    ensures s.take(i + 1).fold_left(id, f) == f(s.take(i).fold_left(id, f), s[i]),
{
    let t = s.take(i + 1);
    assert(t.drop_last() =~= s.take(i));
    reveal(Seq::fold_left);
}
```

The loop now calls `lemma_take_fold_left_step(s, i as int, id, spec_f)`.
Before, it inlined those three lines, and `reveal(Seq::fold_left)` met the
loop's quantified invariants in one query. Moving them into a lemma separates
the two. The `accept` stays only as a `// BYPASSED` comment.

The same fix went into the `scan` loops of `ArraySeqStPer.rs` and
`LinkedListStEph.rs`. Chap18 now has **0 new holes** and verifies at 1003/0
(log `20260922-044407`).

### 3.1 The Chap40 commit's "Chap18 scan rlimit"

Commit `50237d7` records Chap40 as 1174 verified with 3 errors, one of them a
"Chap18 scan rlimit". That error was **not** the authorised hole. It was
`while loop: Resource limit (rlimit) exceeded` at
`src/Chap18/LinkedListStEph.rs:706`, the `scan` loop of **LinkedListStEph**.
That is a different file from the one with the hole, but it has the same
inline `reveal(Seq::fold_left)` step.

The loop verified in isolate Chap18 (1000/0), where fewer modules are in the
query context. It exceeded the rlimit once isolate Chap40 added the Chap40
modules. The PTT library compile (the crate compiled as `apas_verus`) did the
same, as did the `ArraySeqStPer.rs` scan loop under isolate Chap59 (log
`20260922-044039`).

The hole only covered `ArraySeqStEph.rs`, so the sibling loops still counted.
With the lemma in all three files, the Chap40 run has no Chap18 error
(1178 verified, log `20260922-044632`).

### 3.2 The two find_link errors

The other two errors in that commit were `find_link` in Chap40
`BSTSizeStEph.rs` (ensures lines 371 and 373) and in Chap40
`BSTReducedStEph.rs` (ensures lines 457-459). The diagnostic was:

```
%return is Some ==>
    spec_content_link(old(link)).to_iset().contains(%return.0) ✘
        function is uninterpreted
```

The failing path was the `Greater` branch. Its proof block does not state
`spec_content_link(link) =~= left ∪ right ∪ {key}`. The `Less` branch does,
and it verifies. The new Verus reaches set membership through `to_iset()`,
and without the stated split it cannot connect the node's contents to the
right subtree's.

The fix adds that one assertion to the `Greater` branch in both files. It is
proof only. Chap40 then verified 1180/0 (log `20260922-044658`).

**Do these errors exist at the baseline `fea1594`?** They cannot be observed
there. At `fea1594`, `validate isolate Chap40` stops at compilation: the
baseline still uses `it@` on std iterators, which the new vstd no longer
specifies (E0599 `no method named view`), so no function is verified. r212
did not touch either `find_link` function. The Chap40 changes are limited to
the iterator sections, the new `iter()` fns, RTT, and PTT. The failures
therefore belong to the verus 0.2026.09.13 upgrade and were hidden behind the
baseline's compile errors. They are pre-existing in that sense, not
introduced by the migration.

## 4. Per chapter

The class counts come from the tool's apply reports in
`~/projects/CSTs/processes/iterator-upgrade/analyses/`. Detect and apply give
the same counts.

### 4.1 Chap18
- Apply (`iterator-upgrade-apply-20260921-164755.log`), 188 sites:
  - delete: iter-struct 8, ghost-struct 8, view-impl 16, iter-invariant 8,
    forloop-ghost-impl 16, iterator-impl 8, fmt-impl 32
  - replace: iter-fn 9, into-iter-impl 15
  - add-use-iter 38
  - ptt-regenerate-test 30
- Residuals: requires-on-into-iter 1, preexisting-unused-import 7.
- Diff stat: 16 files.
- Hand fixes:
  - `ArraySeqMtEphSlice.rs` `into_iter`: form B. The `requires` was removed,
    the `ensures` became conditional, and the body gained an O(1) bounds
    check. Cost O(1) → O(1).
  - `scan` in `ArraySeqStEph.rs`, `ArraySeqStPer.rs`, and
    `LinkedListStEph.rs`: `lemma_take_fold_left_step` (§3). Proof only.

### 4.2 Chap19
- Apply (`-165151`), 106 sites:
  - delete: iter-struct 4, ghost-struct 4, view-impl 8, iter-invariant 4,
    forloop-ghost-impl 8, iterator-impl 4, fmt-impl 16
  - replace: iter-fn 5, into-iter-impl 7
  - add-use-iter 25
  - ptt-regenerate-test 21
- Residuals: requires-on-into-iter 1, manual-loop 1,
  preexisting-unused-import 3, ptt-unknown-pattern 1.
- Hand fixes:
  - `ArraySeqMtEphSlice.rs` `into_iter`: form B. O(1) → O(1).
  - `ArraySeqMtEph.rs` `concat_seqs`: the while-loop invariants moved from
    `iter@` to `IteratorSpec::remaining`. Exec unchanged.
  - PTT `slice_iter_over_subslice` rewritten.
- The tool also re-exposed the chained wrappers of Chap19 in Chap41
  `ArraySetStEph.rs` and Chap42 `Table{StEph,StPer,MtEph}.rs` and their PTTs.
  That was needed for cargo to build the crate; those files are verified in
  waves 4-5.

### 4.3 Chap23
- Apply (`-170331`), 69 sites. Residuals: kept-ensures-conjunct 6,
  ptt-unknown-pattern 6, preexisting-unused-import 1.
- Hand fix: six `ProveBalBinTreeStEph.rs` tests rewritten to the loop and
  for templates.

### 4.4 Chap37
- Apply with `--into-iter c` (`-172409`, then `-174254` on the 14 BST files
  after the inline class was added), 279 sites:
  - delete: iter-struct 5, ghost-struct 18, view-impl 32, iter-invariant 16,
    forloop-ghost-impl 36, iterator-impl 15, snapshot-iter-struct 10,
    fmt-impl 34, unused-use 5
  - replace: iter-fn 22, into-iter-impl 1
  - add-use-iter 32
  - ptt-regenerate-test 14
  - comment-out-into-iter-impl 19, ptt-comment-out-test 20
  - the rerun added inline-into-iter-body 4
- Residuals: custom-iterator 3, kept-ensures-conjunct 1, ptt-no-definer 18.
- Hand fixes:
  - **The four custom iterators** (`AVLTreeSeq.rs AVLTreeSeqIter`,
    `AVLTreeSeqStEph.rs AVLTreeSeqIterStEph`, `AVLTreeSeqStPer.rs
    AVLTreeSeqStPerIter`, `AVLTreeSeqMtPer.rs AVLTreeSeqMtPerBorrowIter`):
    - Each got an `IteratorSpecImpl` (study §5, behaviour 2). `remaining` is
      the suffix of the in-order values from `pos`, `peek` reads them, and
      `decrease` is the remaining length.
    - The StEph, StPer, and MtPer files gained
      `spec_inorder_values(link) -> Seq<T>` and
      `lemma_inorder_values_maps_to_inorder`, and an `elts()` accessor.
      `AVLTreeSeq.rs` already had the equivalent.
    - StEph and StPer gained a `Ghost<Seq<T>>` field `values`, which costs
      nothing at run time.
    - Each `iter()` `ensures` gained `IteratorSpec::remaining(&it) ==
      elts.as_ref()` and `decrease is Some`, and none was removed. The bodies
      gained only ghost code; the cost is unchanged: O(1) for AVLTreeSeq and
      MtPer, O(lg n) for StEph and StPer (the spine push).
    - All four `next` bodies remain `#[verifier::external_body]`, which was
      already true at the baseline. Proving them needs a value-level `nth`
      `ensures` (study §5.1) or the stack-remaining lemma (study §5.2); that
      is not done here. No hole was added.
    - Each file's borrowing `IntoIterator` impl took form C by hand.
  - `AVLTreeSeqMtPer.rs`: the consuming impl's kept `true,` conjunct was
    removed.
  - `BSTSplayStEph.rs`: new inherent `iter()`. Cost O(n), the same as the
    commented `into_iter`. Its RTT (3 lines) and PTT (2 tests) now use
    `iter()`.
  - PTT: `ProveAVLTreeSeq{,StEph,StPer,MtPer}.rs` were rewritten. Two tests
    each moved to the `next()` loop with a ghost `pos` and the `for` template
    with `it.seq()`/`it.index()`; the two `borrow_into` tests in each file
    were commented out.
  - `BSTSplayMtEph.rs:766`: the assertion `assert(link_contains(orig_left_right,
    x))`, marked "UNNEEDED" before the upgrade, was restored, and one was added
    for `orig_root_right` in the `else` branch. Proof only.
  - `AVLTreeSeq.rs` `insert_at_link`: `#[verifier::rlimit(20)]` (§1 note 2).

### 4.5 Chap38
- Apply (`-185912`), 22 sites. Residual: kept-ensures-conjunct 1
  (`BSTParaStEph.rs:1686`, `it@.1.len() == self@.len()` kept beside the
  triple).
- Hand fixes:
  - `BSTParaMtEph.rs:935`: a `join_mid` precondition needed the left
    ordering asserted: `forall t. lr@.contains(t@) ==> t.cmp_spec(&root_key)
    == Less`.
  - Explicit `#[trigger]` on 6 quantifiers in `BSTPara{St,Mt}Eph.rs`.
  - RTT: 3 lines in `TestBSTParaStEph.rs`.

### 4.6 Chap39
- Apply (`-190217`), 63 sites. Residuals: kept-ensures-conjunct 1,
  ptt-no-definer 6.
- Hand fixes:
  - New `iter()` on `ParamTreap`, `BSTSetTreapMtEph`, and `BSTTreapStEph`.
    Each is O(n), the same as the `into_iter` it replaces.
  - `BSTParaTreapMtEph.rs:390`: `assert(right@.contains(t@))` inside an
    `assert forall`.
  - RTT: 4 files. PTT: 3 files.

### 4.7 Chap40
- Apply (`-190627`, then `-190738` after the PhantomData class), 29 + 30
  sites. Residuals: ptt-no-definer 6, then 4.
- Hand fixes:
  - New `iter()` on `BSTSizeStEph`, `BSTKeyValueStEph`, and
    `BSTReducedStEph`. Each is O(n), the same as the `into_iter` it replaces.
  - `find_link` in 2 files (§3.2).
  - RTT: 3 files. PTT: 3 files.

### 4.8 Chap57, 58, 59 (consumers)
- Detect found:
  - loop-with-break: 2 (Chap57), 4 (Chap58), 4 (Chap59).
  - Chap59 also had subst-wrapper-pos 4 and subst-wrapper-elements 4 in the
    Mt files; the tool applied those (`-191531`).
- Hand fixes, all in loop clauses:
  - Every loop moved to the r211 form: `IteratorSpec::obeys_prophetic_iter_laws`,
    `decrease is Some`, and `decreases IteratorSpec::decrease(&it)->0`.
  - Where the loop used the sequence, it gained a ghost `pos` plus
    `orig = vstd::std_specs::hash::into_iter_hash_keys(it)`, because
    `SetStEph::iter` returns `hash_set::Iter`.
  - The 4 Johnson Mt `for` loops use `iter.seq().unref() == arcs_seq`.
- Exec is unchanged in every file (`match it.next()` as before).
- Chap57: Dijkstra U64 and F64 carried their `used_edges` witness from `it@.0`
  to `pos`.

### 4.9 Totals
- Hand fixes, by kind:
  - 4 custom iterators, each with an `IteratorSpecImpl` and a hand form C
  - 7 new `iter()` fns
  - 2 form-B impls
  - 3 scan lemmas
  - 7 proof hints, one each in BSTSplayMtEph, BSTParaMtEph, BSTParaTreapMtEph,
    BSTSize and BSTReduced (`find_link`), Chap19 `concat_seqs`, and
    AVLTreeSeqMtPer (conjunct)
  - 6 trigger annotations
  - 1 rlimit attribute
  - 14 consumer loops
  - 29 rewritten PTT tests (Chap19 1, Chap23 6, Chap37 10, Chap39 6,
    Chap40 6)
  - 24 RTT lines in 8 files
- Exec edits: 9 functions — 2 form-B `into_iter` (O(1) → O(1)) and 7 new
  `iter()` (O(n), the cost of the `into_iter` each replaces). Every other
  exec change is a wrapper unwrap (`Wrapper { inner: e }` → `e`, O(1)), made
  by the tool.
- No `/// - Alg Analysis` line was edited. Only the 7 new fns carry new
  lines. No discrepancy was found, so nothing was added to
  `docs/AlgorithmicAnalysisIssues.md`.

## 5. Regression (step 7)

| # | Run | Result | Log |
|---|-----|--------|-----|
| 1 | isolate Chap02 | 631 verified, 0 errors | 20260922-044602 |
| 2 | isolate Chap03 | 622 verified, 0 errors | 20260922-044605 |
| 3 | isolate Chap05 | 760 verified, 0 errors | 20260922-044608 |
| 4 | isolate Chap06 | 1037 verified, 0 errors | 20260922-044611 |
| 5 | isolate Chap17 | 645 verified, 0 errors | 20260922-044621 |
| 6 | isolate Chap50 | 766 verified, 0 errors | 20260922-044623 |
| 7 | isolate Chap66 | 805 verified, 0 errors | 20260922-044627 |
| 8 | full `scripts/validate.sh` | stops at compile: 2 × E0425 `iter_invariant_tablemteph` in Chap42 `TableMtEph.rs:273,2433` | 20260922-044713 |
| 9 | `cargo test --release --no-fail-fast` | 264 test targets pass (4198 tests); doctests 1 pass, 12 fail | rtt-full.20260922-044746 |

- **Row 8:** the full crate cannot verify until waves 3-5 migrate the
  remaining chapters. The first stop is Chap42 `TableMtEph.rs`, a wave-5 file
  whose chained wrapper the tool removed in step 3 so that cargo builds.
- **Row 9:** the 12 failures are doc-comment code blocks in
  `src/standards/partial_eq_eq_clone_standard.rs` and
  `src/standards/spec_wf_standard.rs`. r212 did not touch those files.

## 6. Patches (step 8)

Directory: `plans/r212-patches/ChapNN/0001-r212-ChapNN-iterator-upgrade.patch`.
Each is `git diff fea1594 HEAD` limited to `src/ChapNN`, `tests/ChapNN`, and
`rust_verify_test/tests/ChapNN`; `step1-experiments/` covers the experiments
and `src/lib.rs`. Fixture HEAD is `db03e5d`. Every patch passes `git apply
--check` on this tree:

| # | Dir | Files | Lines | `--check` |
|---|-----|------:|------:|-----------|
| 1 | step1-experiments | 4 | 423 | OK |
| 2 | Chap18 | 16 | 3238 | OK |
| 3 | Chap19 | 8 | 1893 | OK |
| 4 | Chap23 | 4 | 1320 | OK |
| 5 | Chap37 | 32 | 5219 | OK |
| 6 | Chap38 | 4 | 421 | OK |
| 7 | Chap39 | 11 | 1323 | OK |
| 8 | Chap40 | 9 | 1052 | OK |
| 9 | Chap41 | 2 | 385 | OK |
| 10 | Chap42 | 5 | 915 | OK |
| 11 | Chap57 | 2 | 169 | OK |
| 12 | Chap58 | 2 | 74 | OK |
| 13 | Chap59 | 4 | 227 | OK |

These patches are not independent. Chap37 depends on the Chap18 and Chap19
patches, and the Chap41 and 42 patches only make the crate build under cargo
and are not verified. Apply them in the order listed.

## 7. Tool classes added (CSTs r0728)

| # | Class / feature | Commit |
|---|-----------------|--------|
| 1 | PTT loop template on `next()` + ghost `pos` | 2088bdd94 |
| 2 | chained wrapper (`Name { inner: e.iter() }`) | c486058e1 |
| 3 | `--only a.rs,b.rs`; suffixed `iter_invariant_<m>` | d07dc5f8b |
| 4 | wrapper element type, lifetimes, unnamed return | d53a79c8d |
| 5 | any constructor returning a deleted wrapper | 37e8019ac |
| 6 | `it@.1 =~= E` read like `==` | 582c587a3 |
| 7 | snapshot wrapper (`Vec` + `usize`), weak triple | b4294b516 |
| 8 | `--into-iter c`: comment-out-into-iter-impl, ptt-comment-out-test | d0a82876b |
| 9 | form C on definers with no wrapper left | 77c034ae7 |
| 10 | inline-into-iter-body; note indentation | 476452acc |
| 11 | `PhantomData` field does not make a wrapper custom | 38c8e49d2 |

Each class has a fixture test. Every commit passed the lib tests (15 in
`Testdefiner`) and the binary tests.

## 8. Tool limitations remaining

1. Custom iterators (fields other than one std iterator, a snapshot, or
   `PhantomData`) are residuals. Their `IteratorSpecImpl` is hand work.
2. Manual `loop`s with `break` (loop-with-break) are residuals. Their
   invariants need a ghost `pos` and a choice of `into_iter_elts` (slice or
   vec) or `into_iter_hash_keys` (hash set) that depends on the iterator's
   type. The tool does not resolve types.
3. When a module's `into_iter` is commented out (form C), its PTT tests are
   commented out, not regenerated onto `iter()`. The 14 `borrow_iter` rewrites
   in Chap37-40 were done by hand.
4. RTT files are not transformed. The 24 `.iter()` call-site edits were done
   by hand.
5. Two runs in the same second overwrite each other's
   `analyses/iterator-upgrade-*.log` (lost: the first Chap41 apply report).
   The fixture's `scratch/r212-rtt.sh` has the same one-second log-name
   collision.
6. Kept `ensures` conjuncts (`it@.1.len() == …` beside the triple) are
   reported, not rewritten.
7. A form-C impl whose text contains `*/` is left as a residual.
