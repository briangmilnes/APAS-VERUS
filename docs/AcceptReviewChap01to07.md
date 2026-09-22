# Accept review, Chap01–07, for Verus 0.2026.09.13

Date: 2026-09-20. Round r207, agent 4. Toolchain reviewed against:
`~/projects/verus` at tag `release/0.2026.09.13.671956e`, vstd at
`~/projects/verus/source/vstd`. No source file was edited; no Verus run was
made. Every claim of "provable" below is a reading of the vstd specs, not a
prover result, except where an existing experiment or standard is cited.

Chapters 01, 04, and 07 have no directory. Chapters 02, 03, 05, and 06 were
covered file by file: 29 files, of which 8 contain holes. Line numbers are
those of commit `4bd6b4f92`; agent 1's concurrent renames in the working tree
are one line for one line and leave every cited line number unchanged.

## 1. Counts, re-derived

The plan's opening table (68 `accept`, 19 `// accept hole`, 19 `assume`,
35 `external_body`) does not match `grep -c` on the four directories. The
re-derived counts, one line per annotated site:

| # | Chap | File | accept | hole | assume | ext_body | sites |
|---|------|------|-------:|-----:|-------:|---------:|------:|
| 1 | 02 | HFSchedulerMtEph.rs | 0 | 10 | 1 (a) | 6 (b) | 10 |
| 2 | 02 | FibonacciHFScheduler.rs | 0 | 0 | 0 | 0 | 0 |
| 3 | 03 | InsertionSortStEph.rs | 0 | 0 | 0 | 0 | 0 |
| 4 | 05 | KleeneStPer.rs | 0 | 0 | 0 | 0 | 0 |
| 5 | 05 | MappingStEph.rs | 1 | 0 | 0 | 0 | 1 |
| 6 | 05 | RelationStEph.rs | 0 | 0 | 0 | 0 | 0 |
| 7 | 05 | SetMtEph.rs | 4 | 0 | 1 | 0 | 5 |
| 8 | 05 | SetStEph.rs | 1 | 0 | 0 | 0 (c) | 1 |
| 9 | 06 | DirGraphMtEph.rs | 11 | 0 | 0 | 0 | 11 |
| 10 | 06 | DirGraphStEph.rs | 0 | 0 | 0 | 0 | 0 |
| 11 | 06 | LabDirGraphMtEph.rs | 2 | 0 | 2 | 0 | 4 |
| 12 | 06 | LabDirGraphStEph.rs | 0 | 0 | 0 | 0 | 0 |
| 13 | 06 | LabUnDirGraphMtEph.rs | 5 | 0 | 2 | 0 | 7 |
| 14 | 06 | LabUnDirGraphStEph.rs | 0 | 0 | 0 | 0 | 0 |
| 15 | 06 | UnDirGraphMtEph.rs | 7 | 0 | 0 | 0 | 7 |
| 16 | 06 | UnDirGraphStEph.rs | 0 | 0 | 0 | 0 | 0 |
| 17 | 06 | WeightedDirGraphStEph*.rs (13) | 0 | 0 | 0 | 0 | 0 |
| 18 | — | Total | 31 | 10 | 6 | 6 | 46 |

(a) The `assume(false)` at HFSchedulerMtEph.rs:163 is inside a line that
begins `// Veracity: UNNEEDED proof block`, so the whole `proof { assume(false); }`
is a comment. The live Err arm is `diverge()` alone, inside an `external_body`
fn, so Verus never sees either. Live `assume` count in Chap02 is 0.

(b) The 6 `external_body` attributes are 6 of the 10 `// accept hole` lines;
the other 4 are 2 struct markers outside `verus!`, 1
`external_type_specification`, and the commented assume. The 10 sites are
counted once each.

(c) SetStEph.rs:949 matches `external_body` only inside a comment
("HashSetWithView* eq is external_body"). It is not an attribute.

## 2. Summary

| # | Chap | File | sites | closable | permitted | open |
|---|------|------|------:|---------:|----------:|-----:|
| 1 | 02 | HFSchedulerMtEph.rs | 10 | 7 | 1 | 2 |
| 2 | 05 | MappingStEph.rs | 1 | 0 | 1 (d) | 0 |
| 3 | 05 | SetMtEph.rs | 5 | 4 | 1 | 0 |
| 4 | 05 | SetStEph.rs | 1 | 0 | 1 | 0 |
| 5 | 06 | DirGraphMtEph.rs | 11 | 11 | 0 | 0 |
| 6 | 06 | LabDirGraphMtEph.rs | 4 | 4 | 0 | 0 |
| 7 | 06 | LabUnDirGraphMtEph.rs | 7 | 7 | 0 | 0 |
| 8 | 06 | UnDirGraphMtEph.rs | 7 | 7 | 0 | 0 |
| 9 | — | Total | 46 | 40 | 4 | 2 |

Column meanings. "closable": a concrete change is given below that removes
the site with no new `assume`, `accept`, or `external_body` on algorithmic
logic. "permitted": one of the four CLAUDE.md patterns (assume in
`PartialEq::eq`, assume in `Clone::clone`, `assume(false); diverge()` in a
thread-join Err arm, `external_body` at a thread-spawn boundary); §4 says for
each whether 09.13 removes the need. "open": stays as a trust boundary with
the reason stated.

(d) MappingStEph.rs:659 is the permitted `PartialEq::eq` pattern, but the
postcondition it accepts is false for non-well-formed inputs (§4.2). It needs
a user decision on the spec, not only a proof.

The 40 closable sites fall into three shapes (§5): 33 lock-boundary sites
closed by one transformation (shape A), 7 HFScheduler sites closed by moving
`TaskState` into `verus!` (shape C). The 4 permitted sites have a
09.13-enabled alternative (shape D) that needs a user decision because it
changes `PartialEqSpecImpl::obeys_eq_spec`.

## 3. Upstream changes that apply

| # | Change | Where | Applies to |
|---|--------|-------|------------|
| 1 | `resource::ghost_var` (#2142, May 1) | `vstd/resource/impls/ghost_var.rs` | all 33 lock-boundary sites |
| 2 | impl spec may extend trait spec (#2540) | `vir/traits.rs`, test `traits.rs` | 3 `HashSet` eq accepts |
| 3 | `Set` finite by type (#2486) | `vstd/set.rs:227` | no hole; wf conjuncts become `true` |
| 4 | `Ghost<T>: Send + Sync` (#2287) | `builtin/src/lib.rs:637` | none in Chap02–06 |
| 5 | `panic!` supported (#2861) | `vstd/pervasive.rs:437` | HFScheduler `wait`; see §4.1 |
| 6 | `Option::expect` spec | `std_specs/option.rs:193` | HFScheduler `wait` |
| 7 | HashMap::clone spec (#2513) | `std_specs/hash.rs:578` | model for a HashSet clone spec |

Row 1 is not in the plan's table and is the important one. `GhostVarAuth<T>`
and `GhostVar<T>` are two halves of one fractional token: "the pair of tokens
always agree on the value, but they can be owned separately"
(`ghost_var.rs`, doc comment). One half lives inside the lock, one in the
wrapper; `agree` proves equality after every acquire, `update` moves both.
This is the "split tokens" approach that
`src/experiments/rwlock_no_ghost_field.rs` names as Approach 2 and did not
implement, because vstd at 04.20 had no such token. The API:

```rust
pub proof fn new(v: T) -> (tracked result: (GhostVarAuth<T>, GhostVar<T>))
    ensures result.0.id() == result.1.id(), result.0@ == v, result.1@ == v;
pub proof fn agree(tracked &self, tracked v: &GhostVar<T>)
    requires self.id() == v.id(),
    ensures self@ == v@;
pub proof fn update(tracked &mut self, tracked v: &mut GhostVar<T>, new_val: T)
    requires old(self).id() == old(v).id(),
    ensures final(self).id() == old(self).id(), final(v).id() == old(v).id(),
            old(self)@ == old(v)@, final(self)@ == new_val, final(v)@ == new_val;
```

Rows 8–10 of the plan's table (#2326 logical atomicity, #2720 atomics via
pointer, #2884 tracked borrows) do not apply: no site in Chap02–06 touches
atomics or raw pointers, and the HFScheduler holes are std `Mutex`/`Condvar`
calls, for which vstd 09.13 still has no specification (grep of
`std_specs/` finds no `Mutex`, `Condvar`, or `LazyLock`). `vstd/rwlock.rs`
and `vstd/thread.rs` have no commits since 04.20.

## 4. One entry per hole

Column key for the per-hole tables: "prop" is the proposition accepted or
assumed, or the `ensures` an `external_body` promises; "reason" is the
recorded reason (git or comment); "up" is the row of §3 that applies;
"shape" is the transformation in §5; "LOP" is estimated lines of proof
added (negative: net removal); "conf" is H (no experiment), M (experiment
listed in §6), or U (user decision).

### 4.1. Chap02 HFSchedulerMtEph.rs

Reason recorded for all 10 sites: file header, "Outside verus! because
Condvar/Mutex and LazyLock closure are not Verus-friendly", and the
`external_body` attributes on every pub fn, added when the scheduler was
written (`git log` shows no later change to these lines).

| # | Chap | File | line | kind | prop | up | shape | LOP | conf |
|---|------|------|-----:|------|------|----|-------|----:|------|
| 1 | 02 | HFSchedulerMtEph.rs | 31 | hole marker | `PoolState` outside `verus!` | — | open | 0 | H |
| 2 | 02 | HFSchedulerMtEph.rs | 37 | hole marker | `TaskState` outside `verus!` | — | C | 6 | M (E3) |
| 3 | 02 | HFSchedulerMtEph.rs | 95 | ext_type_spec | `ExTaskState` wraps `TaskState` | — | C | -3 | M (E3) |
| 4 | 02 | HFSchedulerMtEph.rs | 96 | ext_body | on `ExTaskState` | — | C | -1 | M (E3) |
| 5 | 02 | HFSchedulerMtEph.rs | 109 | ext_body | `join`: fa/fb ensures on pair | — | C | 0 | M (E3) |
| 6 | 02 | HFSchedulerMtEph.rs | 118 | ext_body | `spawn_join`: same | — | C | 1 | M (E3) |
| 7 | 02 | HFSchedulerMtEph.rs | 144 | ext_body | `spawn`: predicate ==> f.ensures | — | C | 4 | M (E3) |
| 8 | 02 | HFSchedulerMtEph.rs | 163 | assume(false) | Err arm (commented out) | 5 | permitted | 0 | H |
| 9 | 02 | HFSchedulerMtEph.rs | 174 | ext_body | `wait`: task.predicate(result) | 6 | C | 2 | M (E3) |
| 10 | 02 | HFSchedulerMtEph.rs | 195 | ext_body | `set_parallelism`: no ensures | — | open | 0 | H |

Sites 1 and 10 stay open: `PoolState` holds `std::sync::Mutex<usize>` and
`Condvar`, and `set_parallelism` writes a `std::sync::RwLock` static. vstd
09.13 specifies none of these types. They are configuration and thread-pool
accounting, not algorithm.

Site 8: the Err arm of a thread join is the permitted pattern. 09.13 does
not remove the need. `panic!` is now accepted inside `verus!` (#2861) but
`__call_panic` requires `allow_panic()`, which is a cargo feature and is off,
so a `panic!` in the Err arm must be proved unreachable, which it is not.
The idiom stays `proof { assume(false); } diverge()`. Note that the current
text has the proof block commented out; once `spawn_join` is verified (site
6) the block must be live again.

Sites 2–7 and 9 close together (shape C). The algorithmic content of this
file, the help-first decision and the fork-join protocol, is currently
unverified only because the whole fns are `external_body`. The only calls
Verus cannot specify are `try_acquire()` and `release()` (a `Mutex` lock,
compare, decrement / increment, `Condvar::notify_one`). Shape C makes those
two the trust boundary, with `ensures true`, and verifies everything else
against the existing specs. `spawn_plus` and `JoinHandlePlus::join` in
`vstdplus/threads_plus.rs` already carry the specs the bodies need.

Accounting for the file after shape C: `external_body` on `set_parallelism`,
`try_acquire`, `release` (3, was 6); `external_type_specification` 0 (was
1); `assume(false); diverge()` 2 (was 1 commented; `wait`'s `panic!` becomes
the same permitted idiom); `PoolState` marker 1. Six sites remain, all
either permitted or on std synchronisation primitives; the algorithm is
verified. `wait` gains `requires task.spec_taskstate_wf()` (a wf
precondition, within the round's rule) and `spawn` gains
`ensures task.spec_taskstate_wf()`. The one caller outside Chap02,
`SetMtEph::cartesian_product` (SetMtEph.rs:554–708), stores tasks in
`handles: Vec<TaskState<..>>`; its two loop invariants need one conjunct
each: `forall|i: int| 0 <= i < handles@.len() ==> #[trigger] handles@[i].spec_taskstate_wf()`.

Dead code noticed: `fn acquire()` (line 74) is defined and never called.

### 4.2. Chap05 MappingStEph.rs

| # | Chap | File | line | kind | prop | up | shape | LOP | conf |
|---|------|------|-----:|------|------|----|-------|----:|------|
| 1 | 05 | MappingStEph.rs | 659 | accept | `r == (self@ == other@)` | — | D' | 6 | U |

Reason: commit 872ee870b (2026-02-06) "Move PartialEq inside verus! for
Chap05 types"; the comment at line 657 says "Verus BUG is preventing this as
of Version: 0.2026.02.05". `r` is `self.mapping == other.mapping`, and
`RelationStEph::eq` (RelationStEph.rs:420–423 and its `PartialEq`) ensures
`r == (self.mapping@ == other.mapping@)`. The view is
`Map::new(|x| exists y. mapping@.contains((x,y)), |x| choose y. ...)`.

Two directions. Forward, `mapping@ == other.mapping@ ==> self@ == other@`:
both maps are the same spec function applied to equal sets; Verus lifts spec
closures to functions of their captures, so congruence gives it, and
`assert(self@ =~= other@)` after `assert(self.mapping@ =~= other.mapping@)`
is the expected proof at 09.13. The 02.05 "bug" is unverifiable now because
the `Map::new(fk, fv)` form no longer exists: at 09.13 `Map::new` takes
`(dom: Set<K>, fv)` (scoping §2 row 4), so agent 1 rewrites this `View`
impl first, and the forward direction must be re-tried on the new form
(E5).

Backward, `self@ == other@ ==> mapping@ == other.mapping@`, is false without
functionality: `{(1,a),(1,b)}` and `{(1,a)}` induce the same map when
`choose` picks `a`. `PartialEq::eq` cannot take `requires wf` (ExPartialEq
declares none, and #2540 lets an impl strengthen `ensures`, not add
`requires`), so the unconditional `ensures equal == (self@ == other@)` is not
provable for any Verus; the accept hides a postcondition that does not hold
on non-functional relations. Options, both a user decision:

1. Keep the accept as the permitted pattern and record that it is a spec
   defect, not a Verus limitation.
2. State the true contract:
   `ensures equal == (self.mapping@ == other.mapping@), equal ==> self@ == other@,
   self.spec_mappingsteph_wf() && other.spec_mappingsteph_wf() ==> equal == (self@ == other@)`
   and set `eq_spec` to `self.mapping@ == other.mapping@`. The third conjunct
   needs the functionality argument: from `self@ == other@` and
   `is_functional_set(self.mapping@)`, each `(x,y)` in `self.mapping@` has
   `other@[x] == self@[x] == y` and `x` in `other@.dom()`, so
   `(x, choose ...)` is in `other.mapping@` with the chosen `y' == y` by
   `is_functional_set(other.mapping@)`. About 6 lines inside an
   `assert forall ... by`. Callers that compare mappings under wf lose nothing.

Option 2 is not a weakening of the postcondition in the sense the round
forbids: the current postcondition is not true, so no caller can have
soundly depended on it outside wf.

### 4.3. Chap05 SetMtEph.rs

Reason for sites 1–4: commit f675da655 (2026-03-11) "Add coarse RwLock
locked wrappers for Chap05/06 Mt collections" wrote them as `assume`;
dac76575c (2026-03-30, R114) converted the reader sites to `accept` and left
the writer as `assume`. The standard
`toplevel_coarse_rwlocks_for_mt_modules.rs` records the reason: "The
RwLockPredicate is frozen at construction ... type_invariant can't see inside
the RwLock ... The ghost shadow is the only way to get value-level specs at
Layer 2." `docs/current-architecture-coarse-lock-parallel.md` §5 calls two
per file the floor. Both statements were true of vstd 04.20 and are false at
09.13 because of §3 row 1.

| # | Chap | File | line | kind | prop | up | shape | LOP | conf |
|---|------|------|-----:|------|------|----|-------|----:|------|
| 1 | 05 | SetMtEph.rs | 1145 | accept | `size`: `inner@ == self@` | 1 | A | 1 | M (E1) |
| 2 | 05 | SetMtEph.rs | 1156 | accept | `mem`: `inner@ == self@` | 1 | A | 1 | M (E1) |
| 3 | 05 | SetMtEph.rs | 1166 | assume | `insert`: ghost == locked_val@ | 1 | A | 1 | M (E1) |
| 4 | 05 | SetMtEph.rs | 1179 | accept | `choose`: `inner@ == self@` | 1 | A | 1 | M (E1) |
| 5 | 05 | SetMtEph.rs | 1223 | accept | `eq`: `equal == (self@ == other@)` | 2 | D | 4 | U |

Site 3 is spelled `assume`, not `accept`, and is not one of the four
permitted patterns; it is the "writer accept" of the standard under the
wrong name. Shape A removes it rather than renaming it.

Site 5 is the permitted `PartialEq::eq` pattern. Reason: the same commit
872ee870b; `HashSetWithViewPlus::eq` (vstdplus/hash_set_with_view_plus.rs)
is `external_body` with no `ensures`, so nothing about `self.elements ==
other.elements` reaches the caller. 09.13 does not remove the need by
itself, but #2540 makes shape D possible (§5.4); that changes
`obeys_eq_spec` and is a user decision.

### 4.4. Chap05 SetStEph.rs

| # | Chap | File | line | kind | prop | up | shape | LOP | conf |
|---|------|------|-----:|------|------|----|-------|----:|------|
| 1 | 05 | SetStEph.rs | 951 | accept | `eq`: `equal == (self@ == other@)` | 2 | D | 4 | U |

Same as SetMtEph site 5. The comment at line 949 states the reason exactly:
"HashSetWithView* eq is external_body so we have to trust it here."

### 4.5. Chap06 DirGraphMtEph.rs

Reason for all 11: as §4.3 (commits f675da655, dac76575c). The locked trait
`LockedDirGraphMtEphTrait` (lines 893–1020) has no mutating method, so this
file also admits the cheaper shape B (§5.2).

| # | Chap | File | line | kind | prop | up | shape | LOP | conf |
|---|------|------|-----:|------|------|----|-------|----:|------|
| 1 | 06 | DirGraphMtEph.rs | 1057 | accept | `vertices`: `inner@ == self@` | 1 | A or B | 1 / 0 | H (B) |
| 2 | 06 | DirGraphMtEph.rs | 1069 | accept | `arcs`: same | 1 | A or B | 1 / 0 | H (B) |
| 3 | 06 | DirGraphMtEph.rs | 1081 | accept | `sizeV`: same | 1 | A or B | 1 / 0 | H (B) |
| 4 | 06 | DirGraphMtEph.rs | 1092 | accept | `sizeA`: same | 1 | A or B | 1 / 0 | H (B) |
| 5 | 06 | DirGraphMtEph.rs | 1104 | accept | `neighbor`: same | 1 | A or B | 1 / 0 | H (B) |
| 6 | 06 | DirGraphMtEph.rs | 1116 | accept | `n_plus`: same | 1 | A or B | 1 / 0 | H (B) |
| 7 | 06 | DirGraphMtEph.rs | 1128 | accept | `n_minus`: same | 1 | A or B | 1 / 0 | H (B) |
| 8 | 06 | DirGraphMtEph.rs | 1140 | accept | `ng`: same | 1 | A or B | 1 / 0 | H (B) |
| 9 | 06 | DirGraphMtEph.rs | 1153 | accept | `n_plus_of_vertices`: same | 1 | A or B | 1 / 0 | H (B) |
| 10 | 06 | DirGraphMtEph.rs | 1165 | accept | `n_minus_of_vertices`: same | 1 | A or B | 1 / 0 | H (B) |
| 11 | 06 | DirGraphMtEph.rs | 1176 | accept | `ng_of_vertices`: same | 1 | A or B | 1 / 0 | H (B) |

Shape B's confidence is H because it uses only `RwLock::new` (`ensures
s.pred() == pred`), `acquire_read` (`ensures self.inv(read_handle.view())`)
and `ReadHandle::borrow` (`ensures val == self.view()`), all unchanged since
04.20, and the same predicate-with-ghost-field mechanism that
`src/standards/hfscheduler_standard.rs` (`BoundedCounterInv { ghost max_val }`)
verified at 04.20.

### 4.6. Chap06 LabDirGraphMtEph.rs

| # | Chap | File | line | kind | prop | up | shape | LOP | conf |
|---|------|------|-----:|------|------|----|-------|----:|------|
| 1 | 06 | LabDirGraphMtEph.rs | 893 | assume | `add_vertex`: ghost == locked_val@ | 1 | A | 1 | M (E1) |
| 2 | 06 | LabDirGraphMtEph.rs | 906 | assume | `add_labeled_arc`: same | 1 | A | 1 | M (E1) |
| 3 | 06 | LabDirGraphMtEph.rs | 920 | accept | `n_plus`: `inner@ == self@` | 1 | A | 1 | M (E1) |
| 4 | 06 | LabDirGraphMtEph.rs | 931 | accept | `n_minus`: same | 1 | A | 1 | M (E1) |

Sites 1–2 are `assume`, not a permitted pattern (see §4.3 site 3). This file
has mutators, so shape B does not apply.

### 4.7. Chap06 LabUnDirGraphMtEph.rs

| # | Chap | File | line | kind | prop | up | shape | LOP | conf |
|---|------|------|-----:|------|------|----|-------|----:|------|
| 1 | 06 | LabUnDirGraphMtEph.rs | 835 | accept | `vertices`: `inner@ == self@` | 1 | A | 1 | M (E1) |
| 2 | 06 | LabUnDirGraphMtEph.rs | 847 | accept | `labeled_edges`: same | 1 | A | 1 | M (E1) |
| 3 | 06 | LabUnDirGraphMtEph.rs | 859 | accept | `edges`: same | 1 | A | 1 | M (E1) |
| 4 | 06 | LabUnDirGraphMtEph.rs | 871 | accept | `has_edge`: same | 1 | A | 1 | M (E1) |
| 5 | 06 | LabUnDirGraphMtEph.rs | 883 | accept | `ng`: same | 1 | A | 1 | M (E1) |
| 6 | 06 | LabUnDirGraphMtEph.rs | 894 | assume | `add_vertex`: ghost == locked_val@ | 1 | A | 1 | M (E1) |
| 7 | 06 | LabUnDirGraphMtEph.rs | 906 | assume | `add_labeled_edge`: same | 1 | A | 1 | M (E1) |

Note on this trait: `vertices`, `labeled_edges`, `edges`, `has_edge`,
`add_vertex`, `add_labeled_edge` have no `requires self.spec_..._wf()`
(lines 743–795). Shape A needs the id-link predicate in scope, so these six
gain a wf precondition; that is the wf case the round's rule 4 allows. The
only consumers of the locked wrappers are the five defining files (grep of
`src/`, `tests/`, `rust_verify_test/`), so no caller changes.

### 4.8. Chap06 UnDirGraphMtEph.rs

No mutators in `LockedUnDirGraphMtEphTrait` (lines 601–672); shape B applies.

| # | Chap | File | line | kind | prop | up | shape | LOP | conf |
|---|------|------|-----:|------|------|----|-------|----:|------|
| 1 | 06 | UnDirGraphMtEph.rs | 709 | accept | `vertices`: `inner@ == self@` | 1 | A or B | 1 / 0 | H (B) |
| 2 | 06 | UnDirGraphMtEph.rs | 721 | accept | `edges`: same | 1 | A or B | 1 / 0 | H (B) |
| 3 | 06 | UnDirGraphMtEph.rs | 733 | accept | `sizeV`: same | 1 | A or B | 1 / 0 | H (B) |
| 4 | 06 | UnDirGraphMtEph.rs | 746 | accept | `sizeE`: same | 1 | A or B | 1 / 0 | H (B) |
| 5 | 06 | UnDirGraphMtEph.rs | 758 | accept | `neighbor`: same | 1 | A or B | 1 / 0 | H (B) |
| 6 | 06 | UnDirGraphMtEph.rs | 770 | accept | `ng`: same | 1 | A or B | 1 / 0 | H (B) |
| 7 | 06 | UnDirGraphMtEph.rs | 781 | accept | `ng_of_vertices`: same | 1 | A or B | 1 / 0 | H (B) |

## 5. Repeated shapes and the transformation for each

### 5.1. Shape A: lock-boundary bridge via `GhostVarAuth`/`GhostVar` (33 sites)

Every site has the form

```rust
let read_handle = self.locked_x.acquire_read();
let inner = read_handle.borrow();
proof { accept(inner@ == self@); }          // reader, 28 sites
// or
let (mut locked_val, write_handle) = self.locked_x.acquire_write();
proof { assume(self.ghost_locked_x@ == locked_val@); }   // writer, 5 sites
```

The wrapper's `View` reads a `Ghost<..>` field that nothing ties to the
lock's contents. Shape A replaces the `Ghost` field by the outside half of a
ghost-variable pair and puts the inside half in the lock next to the data;
the lock predicate ties the inside half to the data, `agree` ties the two
halves, and the outer wf predicate ties the ids. The exec algorithm does not
change: the same acquire, the same inner method call, the same release. The
lock interior becomes a two-field struct whose second field is zero-sized at
run time.

Worked transformation for SetMtEph.rs (the other four files are the same
with their own inner type and view type):

```rust
// Section 2 imports (add)
use vstd::resource::Loc;
use vstd::resource::ghost_var::{GhostVar, GhostVarAuth};

// Section 4b (replace `pub struct SetMtEphInv;`)
pub struct SetMtEphLockInterior<T: StT + Hash> {
    pub inner: SetMtEph<T>,
    pub auth: Tracked<GhostVarAuth<Set<<T as View>::V>>>,
}

pub struct SetMtEphInv {
    pub ghost id: Loc,
}

// Section 11b (replace the RwLockPredicate impl)
impl<T: StT + Hash> RwLockPredicate<SetMtEphLockInterior<T>> for SetMtEphInv {
    open spec fn inv(self, v: SetMtEphLockInterior<T>) -> bool {
        &&& v.auth@.id() == self.id
        &&& v.auth@@ == v.inner@
        &&& valid_key_type::<T>()
    }
}

// Section 4c (replace the Ghost field)
#[verifier::reject_recursive_types(T)]
pub struct LockedSetMtEph<T: StT + Hash> {
    pub(crate) locked_set: RwLock<SetMtEphLockInterior<T>, SetMtEphInv>,
    pub(crate) ghost_locked_set: Tracked<GhostVar<Set<<T as View>::V>>>,
}

// Section 9c (replace the type_invariant block)
impl<T: StT + Hash> LockedSetMtEph<T> {
    pub closed spec fn spec_ghost_locked_set(self) -> Set<<T as View>::V> {
        self.ghost_locked_set@@
    }
    /// The lock's predicate and the outside token name the same ghost variable.
    pub closed spec fn spec_lockedsetmteph_wf(self) -> bool {
        self.locked_set.pred().id == self.ghost_locked_set@.id()
    }
}
// View is unchanged: `self.spec_ghost_locked_set()`.

// Section 8c: the trait gains `spec fn spec_lockedsetmteph_wf(&self) -> bool;`,
// `empty` ensures it, `size`/`mem`/`choose` require it, `insert` requires
// `old(self).spec_lockedsetmteph_wf()` and ensures `self.spec_lockedsetmteph_wf()`.

// Section 9c impl
fn empty() -> (s: Self) {
    let inner = SetMtEph::empty();
    let tracked (auth, var) = GhostVarAuth::<Set<<T as View>::V>>::new(inner@);
    let ghost id = auth.id();
    let interior = SetMtEphLockInterior { inner, auth: Tracked(auth) };
    LockedSetMtEph {
        locked_set: RwLock::new(interior, Ghost(SetMtEphInv { id })),
        ghost_locked_set: Tracked(var),
    }
}

fn size(&self) -> (size: usize) {
    let read_handle = self.locked_set.acquire_read();
    let interior = read_handle.borrow();
    proof { interior.auth.borrow().agree(self.ghost_locked_set.borrow()); }
    let size = interior.inner.size();
    read_handle.release_read();
    size
}
// `mem` and `choose`: the same one-line proof block replaces the accept.

fn insert(&mut self, x: T) -> (inserted: std::result::Result<bool, ()>) {
    let (mut interior, write_handle) = self.locked_set.acquire_write();
    let inserted = interior.inner.insert(x);
    proof {
        interior.auth.borrow_mut().update(self.ghost_locked_set.borrow_mut(), interior.inner@);
    }
    write_handle.release_write(interior);
    Ok(inserted)
}
```

Why each obligation discharges. After `acquire_read`, vstd gives
`self.locked_set.inv(read_handle.view())` and `borrow` gives
`*interior == read_handle.view()`, so `interior.auth@.id() == pred().id` and
`interior.auth@@ == interior.inner@`. The wf precondition gives
`pred().id == self.ghost_locked_set@.id()`, which is `agree`'s `requires`;
its `ensures` gives `interior.auth@@ == self.ghost_locked_set@@ == self@`.
That is the accepted proposition. For `insert`, `update`'s `ensures`
`old(auth)@ == old(var)@` plus the predicate at acquire gives
`old(self)@ == inner@ before insert`; `SetMtEph::insert` ensures
`inner@ == old(inner)@.insert(x@)`; `update` sets both halves to `inner@`,
so `self@ == old(self)@.insert(x@)`. `release_write` requires `inv(interior)`:
ids preserved (`final(self).id() == old(self).id()`), `auth@@ == inner@` by
`update`, `valid_key_type` from `SetMtEph::insert`'s ensured wf. The outer
wf is preserved because `final(v).id() == old(v).id()`.

Estimated lines per file: interior struct 4, predicate 3, wf accessor 3,
`new`/`empty` +3, one proof line per reader (replacing the accept line), one
per writer (replacing the assume line): SetMtEph 15, LabDirGraphMtEph 15,
LabUnDirGraphMtEph 18, DirGraphMtEph 22, UnDirGraphMtEph 18. Net hole
change: 33 to 0.

The architecture doc's argument that "2 accepts per file is the floor"
(`docs/current-architecture-coarse-lock-parallel.md` §5;
`docs/architecture-coarse-lock-parallel-mt.md` §3.4) rests on the ghost
field being unrelated to the lock. With the split token the induction the doc
describes in prose ("`new()` sets ghost = inner; every write updates both;
`&mut self` prevents interleaving") is exactly what `agree`/`update` check.
The doc's constraint still holds and is now enforced by types: an `Arc` clone
of the wrapper would copy neither `Tracked` half, so only one owner can ever
hold the outside token.

Two points need an experiment (§6 E1, E2): whether Verus accepts
`self.ghost_locked_set.borrow_mut()` while `write_handle` borrows the sibling
field `self.locked_set` (Rust's disjoint-field rule says yes; the `WriteHandle`
type has no `Drop`, so the borrow ends at last use), and whether
`LockedSetMtEph<T>` stays `Send + Sync` with a `Tracked<GhostVar<Set<T::V>>>`
field (the same question was answered yes for `Tracked<SM::token>` in
`src/experiments/bst_plain_mt_tsm.rs`, which is `SUCCEEDS` in `lib.rs`).

### 5.2. Shape B: predicate-carried view for read-only wrappers (18 of the 33)

`LockedDirGraphMtEphTrait` and `LockedUnDirGraphMtEphTrait` have no
mutating method. For a value that never changes, the frozen lock predicate
is the exact specification, and no token is needed:

```rust
// Section 4b
pub struct DirGraphMtEphInv<V: StTInMtT + Hash + 'static> {
    pub ghost view: GraphView<<V as View>::V>,
}

// Section 11b
impl<V: StTInMtT + Hash + 'static> RwLockPredicate<DirGraphMtEph<V>> for DirGraphMtEphInv<V> {
    open spec fn inv(self, v: DirGraphMtEph<V>) -> bool {
        &&& v@ == self.view
        &&& spec_graphview_wf(v@)
        &&& valid_key_type_for_graph::<V>()
    }
}

// Section 4c: drop `ghost_locked_graph` and the type_invariant block.
pub struct LockedDirGraphMtEph<V: StTInMtT + Hash + 'static> {
    pub(crate) locked_graph: RwLock<DirGraphMtEph<V>, DirGraphMtEphInv<V>>,
}

// Section 5c
impl<V: StTInMtT + Hash + 'static> View for LockedDirGraphMtEph<V> {
    type V = GraphView<<V as View>::V>;
    open spec fn view(&self) -> Self::V { self.locked_graph.pred().view }
}

// Section 9c
fn new(V: SetStEph<V>, A: SetStEph<Edge<V>>) -> (s: Self) {
    let g = DirGraphMtEph::from_sets(V, A);
    let ghost gv = g@;
    LockedDirGraphMtEph { locked_graph: RwLock::new(g, Ghost(DirGraphMtEphInv { view: gv })) }
}

fn sizeV(&self) -> (n: usize) {
    let read_handle = self.locked_graph.acquire_read();
    let inner = read_handle.borrow();
    // inv(read_handle.view()) gives inner@ == self.locked_graph.pred().view == self@.
    let n = inner.sizeV();
    read_handle.release_read();
    n
}
```

The accept line is deleted and nothing replaces it: `acquire_read` ensures
`self.inv(read_handle.view())`, whose first conjunct is the accepted
proposition. Zero proof lines; net LOP negative. The trait's
`spec_dirgraphmteph_wf` stays `spec_graphview_wf(self@)` and is now provable
from the predicate at `new`. If a mutator is ever added to these two traits
the file must move to shape A; shape A also works for them today, so the
orchestrator may prefer one transformation for all five files.

### 5.3. Shape C: HFScheduler bodies inside `verus!` (7 sites)

```rust
verus! {

    #[verifier::reject_recursive_types(T)]
    pub enum TaskState<T> {
        Spawned   { handle: JoinHandlePlus<T> },
        Completed { result: Option<T> },
    }

    impl<T> TaskState<T> {
        /// The return values this task may produce: its thread's postcondition, or
        /// the value already computed on this thread.
        pub open spec fn predicate(&self, ret: T) -> bool {
            match self {
                TaskState::Spawned { handle } => handle.predicate(ret),
                TaskState::Completed { result } => *result == Some(ret),
            }
        }

        /// A completed task still holds its result.
        pub open spec fn spec_taskstate_wf(&self) -> bool {
            match self {
                TaskState::Completed { result } => result is Some,
                TaskState::Spawned { .. } => true,
            }
        }
    }

    /// Trust boundary: std Mutex on the pool counter. vstd has no Mutex spec.
    #[verifier::external_body] // accept hole
    fn try_acquire() -> bool { /* body unchanged */ }

    /// Trust boundary: std Mutex and Condvar::notify_one.
    #[verifier::external_body] // accept hole
    fn release() { /* body unchanged */ }

    pub fn join<A, B, FA, FB>(fa: FA, fb: FB) -> (joined_pair: (A, B))
        where /* bounds unchanged */
        requires fa.requires(()), fb.requires(()),
        ensures fa.ensures((), joined_pair.0), fb.ensures((), joined_pair.1),
    {
        if try_acquire() {
            let joined_pair = spawn_join(fa, fb);
            release();
            joined_pair
        } else {
            (fa(), fb())
        }
    }

    pub fn spawn_join<A, B, FA, FB>(fa: FA, fb: FB) -> (joined_pair: (A, B))
        where /* bounds unchanged */
        requires fa.requires(()), fb.requires(()),
        ensures fa.ensures((), joined_pair.0), fb.ensures((), joined_pair.1),
    {
        let handle: JoinHandlePlus<B> = spawn_plus(fb);
        let a = fa();
        let b = match handle.join() {
            Ok(val) => val,
            Err(_) => { proof { assume(false); } diverge() }  // accept hole: thread join error arm
        };
        (a, b)
    }

    pub fn spawn<T, F>(f: F) -> (task: TaskState<T>)
        where F: FnOnce() -> T + Send + 'static, T: Send + 'static,
        requires f.requires(()),
        ensures
            task.spec_taskstate_wf(),
            forall|ret: T| #[trigger] task.predicate(ret) ==> f.ensures((), ret),
    {
        if try_acquire() {
            TaskState::Spawned { handle: spawn_plus(f) }
        } else {
            let result = f();
            TaskState::Completed { result: Some(result) }
        }
    }

    pub fn wait<T: Send + 'static>(task: TaskState<T>) -> (task_result: T)
        requires task.spec_taskstate_wf(),
        ensures task.predicate(task_result),
    {
        match task {
            TaskState::Spawned { handle: h } => {
                let task_result = match h.join() {
                    Ok(val) => val,
                    Err(_) => { proof { assume(false); } diverge() }  // accept hole: thread join error arm
                };
                release();
                task_result
            }
            TaskState::Completed { result } => result.unwrap(),
        }
    }

} // verus!
```

`PoolState`, `PARALLELISM`, `POOL`, `init_pool`, and `acquire` stay outside
`verus!` unchanged. Obligations: in `join`'s sequential arm `fa()` requires
`fa.requires(())` and ensures `fa.ensures((), a)`; in `spawn_join`,
`spawn_plus` ensures `forall ret. handle.predicate(ret) ==> fb.ensures((), ret)`
and `JoinHandlePlus::join` ensures `Ok(r) ==> handle.predicate(r)`; in
`spawn`'s `Completed` arm `predicate(ret)` unfolds to `Some(result) == Some(ret)`;
in `wait`'s `Completed` arm `unwrap` requires `result is Some`, which is
`spec_taskstate_wf`. `wait`'s `panic!("Thread panicked")` cannot stay: under
#2861 `panic!` requires `allow_panic()`, which is off, so the arm takes the
permitted join-Err idiom. `TaskState` outside `verus!` was never required:
`JoinHandlePlus<T>` is declared inside `verus!` in
`vstdplus/threads_plus.rs`, and `Option<T>` is a Verus type; only `PoolState`
holds std synchronisation types.

### 5.4. Shape D: `HashSet` equality (3 permitted sites, user decision)

`vstd/std_specs/hash.rs` at 09.13 specifies `HashSet::{new, with_capacity,
len, is_empty, insert, contains, get, remove, clear, iter}` and nothing for
`<HashSet as PartialEq>::eq` or `<HashSet as Clone>::clone`. #2540 (§3 row 2)
adds the ability to write `assume_specification[ <S as Tr>::f ]` for an
external impl of an external-trait-specified method and have the stronger
postcondition apply at concrete call sites; the test
`direct_concrete_call_with_assume_specification` in
`rust_verify_test/tests/traits.rs` is that case. `ExPartialEq` is such a
trait. So the eq accept can move from three chapter files to one trusted
spec in `vstdplus/hash_set_specs.rs`, of the same kind vstd itself uses for
`HashSet::insert`:

```rust
// vstdplus/hash_set_specs.rs (add; mirrors vstd's HashSet specs)
pub assume_specification<T: Eq + Hash, S: BuildHasher, A: Allocator>
    [ <HashSet<T, S, A> as PartialEq>::eq ](a: &HashSet<T, S, A>, b: &HashSet<T, S, A>) -> (r: bool)
    ensures
        obeys_key_model::<T>() && builds_valid_hashers::<S>() ==> r == (a@ == b@),
;

// vstdplus/hash_set_with_view_plus.rs: eq loses external_body and gains
impl<Key: View + Eq + Hash> PartialEq for HashSetWithViewPlus<Key> {
    fn eq(&self, other: &Self) -> (equal: bool)
        ensures obeys_key_model::<Key>() ==> equal == (self.inner@ == other.inner@),
    { self.inner == other.inner }
}

// Chap05 SetStEph.rs section 12 (SetMtEph.rs is identical)
#[cfg(verus_keep_ghost)]
impl<T: StT + Hash> PartialEqSpecImpl for SetStEph<T> {
    open spec fn obeys_eq_spec() -> bool { valid_key_type::<T>() }
    open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
}

impl<T: StT + Hash> PartialEq for SetStEph<T> {
    fn eq(&self, other: &Self) -> (equal: bool)
        ensures valid_key_type::<T>() ==> equal == (self@ == other@),
    {
        let equal = self.elements == other.elements;
        proof {
            if valid_key_type::<T>() {
                lemma_reveal_view_injective::<T>();
                assert((self.elements.inner@ == other.elements.inner@)
                    == (self.elements.inner@.map(|k: T| k@) == other.elements.inner@.map(|k: T| k@)));
            }
        }
        equal
    }
}
```

The `assert` is the only proof: forward by congruence; backward, an element
`k` of one raw set has `k@` in the other mapped set, so some `k'` with
`k'@ == k@` is in the other raw set, and view injectivity gives `k' == k`.
About 4 lines, possibly with an `assert forall` for the backward direction.

The cost is the conditional postcondition. `ExPartialEq::eq` ensures
`Self::obeys_eq_spec() ==> r == self.eq_spec(other)`, so with
`obeys_eq_spec() == true` the unconditional `r == (self@ == other@)` must be
proved, and that is not provable for any `T`: `HashSet` equality is specified
only under `obeys_key_model`, and `self@` is a set of views, which coincides
with raw-set equality only under view injectivity. Both facts are exactly
`valid_key_type::<T>()`, which every `SetStEph` method already requires
through wf. Callers that compare sets under wf lose nothing; callers without
wf never had a sound guarantee, they had an accept. The change to
`obeys_eq_spec` affects `laws_eq` reasoning (`obeys_eq::<SetStEph<T>>` becomes
conditional) and the vstdplus `feq` axioms that assume `obeys_feq_full` for
every type; that interaction is agent 2's area and is why this is marked U.
If the user keeps the accepts, they remain the permitted pattern and 09.13
changes nothing about them.

The same #2540 mechanism gives `HashSet::clone` a real spec
(`ensures obeys_key_model::<T>() ==> other@ == this@`, by the key model's
clone-equality clause; compare the `HashMap::clone` spec at hash.rs:578),
which would let `HashSetWithViewPlus::clone` and `SetStEph::clone` drop their
`external_body` and their `clone@ == self@` bridges. Those are outside
Chap02–06 (vstdplus) and are noted for agent 2.

## 6. Experiments needed

Each is one file under `src/experiments/`, `RESULT:` header, dated, and
never uncommented in `lib.rs` until it reads `SUCCEEDS`. Written by whoever
implements the round-2 edits; not by this review.

| # | Experiment | Topic | Decides |
|---|------------|-------|---------|
| 1 | E1 | ghost_var bridge through RwLock | shape A, all 33 sites |
| 2 | E2 | Send + Sync with a GhostVar field | shape A for Mt use |
| 3 | E3 | TaskState and spawn/wait in verus! | shape C, 7 sites |
| 4 | E4 | assume_specification for HashSet eq | shape D, 3 sites |
| 5 | E5 | MappingStEph eq on the 09.13 view | §4.2, 1 site |

The question each must answer:

E1. With `GhostVarAuth` inside an `RwLock` interior and `GhostVar` in the
wrapper, does `agree` after `acquire_read` prove `inner@ == self@`, and does
`update` inside a `&mut self` method pass borrow checking while a
`WriteHandle` on the sibling lock field is live?

E2. Is a struct holding `Tracked<GhostVar<Set<A>>>` and
`RwLock<Interior, Inv>` still `Send + Sync` for `A = <V as View>::V`,
`V: StTInMtT`?

E3. Does an enum inside `verus!` with a `JoinHandlePlus<T>` variant, an open
`predicate` spec, and `spawn`/`wait` bodies that call `f()` on an `FnOnce`
and match on `JoinHandlePlus::join` verify against the specs in §5.3?

E4. Does 09.13 accept `assume_specification[ <HashSet<T,S,A> as PartialEq>::eq ]`
given `ExPartialEq` is an `external_trait_specification`, and does the
concrete `==` on `HashSet` then see the ensures?

E5. On the 09.13 `Map::new(dom, fv)` form of `MappingStEph`'s view, does
`self.mapping@ =~= other.mapping@` give `self@ =~= other@` by congruence,
and is the converse provable under `is_functional_set` on both sides?

E1 and E2 can be one file. Shape B needs no experiment.

## 7. Findings outside the hole list

1. `finite()` is deprecated and `true` at 09.13. In these eight files it
   appears in `spec_setmteph_wf`, `spec_setsteph_wf`, `SetMtEphInv::inv`,
   `LockedSetMtEph::wf`, every graph `new`'s `requires`, and `Clone`'s
   `ensures`. None is a hole; all become deletions in the planned
   `finite()` round. `axiom_hash_set_with_view_plus_finite` in
   `vstdplus/hash_set_with_view_plus.rs` has an `admit()` body that the same
   round makes deletable (agent 2).
2. `vstdplus/hash_set_with_view_plus.rs` has 9 `external_body` fns
   (`new`, `with_capacity`, `len`, `contains`, `insert`, `iter`, `clone`,
   `hash`, `eq`) with no `// accept hole` markers; every Chap05/06 spec rests
   on them. vstd's `HashSet` specs at 09.13 cover all of `new`,
   `with_capacity`, `len`, `contains`, `insert`, `iter` conditionally on
   `obeys_key_model`, so those bodies can be verified once the wrapper's
   ensures are stated conditionally (the same design question as shape D).
   Agent 2's review.
3. `Concurrency.rs` `diverge` is `external_body // accept hole` with
   `exec_allows_no_decreases_clause`; it is the target of every join-Err arm.
   Not in Chap02–06.
4. `HFSchedulerMtEph.rs:74` `fn acquire()` is unreachable code.
5. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` ("the ghost
   shadow is the only way"), `src/standards/rwlock_tsm_standard.rs` ("2
   accepts per file for the View bridge"), and both architecture docs state
   that the lock-boundary accept is a floor. At 09.13 that is no longer so;
   if shape A is adopted, agent 3's standards upgrade should replace the
   ghost-shadow pattern with the ghost-variable pattern and the two accept
   categories with none.
6. The five writer sites are spelled `assume` and are not one of the four
   permitted patterns in CLAUDE.md; they are the standard's "writer accept"
   under a different name. Shape A removes them; if it is not adopted they
   should at least be classified consistently with the reader sites.
