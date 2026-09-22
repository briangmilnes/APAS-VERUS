# r208 agent 1 — verify Chap02 through Chap06 on Verus 0.2026.09.13, one chapter at a time

Date: 2026-09-20. One subagent, sequential. Inputs: `plans/r206-verus-upgrade-scoping.md`,
`plans/r207-round-plan.md` (its common rules bind this agent too),
`docs/NameResolutionUpgrade.md`, `docs/VstdplusReview.md`, `docs/StandardsUpgrade.md`,
`docs/AcceptReviewChap01to07.md`.

## Goal

`scripts/validate.sh isolate ChapNN` reports 0 errors and 0 warnings for Chap02,
Chap03, Chap05, and Chap06, in that order (Chap01, Chap04, and Chap07 have no
directory). Each chapter is finished, logged, and recorded in the doc before the
next one starts. The deliverable is `docs/Chap02to06Validation.md` plus the
source edits. Nothing is committed.

## Rules specific to this round

1. `panic!` is acceptable in exec code. Do not rewrite a `panic!` into
   `diverge()` or `unreached()`. (Chap02 verifies today with its `panic!`.)
2. vstd's `Set` and `Map` are finite by type. `finite()` is deprecated and
   `true`: delete every `.finite()` conjunct or `requires` you meet; never
   write one. `Set::new(f)` returns `Option<Set>`; where the predicate ranges
   over an existing finite set, write `that_set.filter(f)`; otherwise use
   `lemma_iset_finite_if_subset_of_seq` → `lemma_set_new_some` → `lemma_set_new`
   (see `src/standards/finite_sets_standard.rs`, rewritten in r207).
3. Iterators follow `src/standards/prophetic_iterators_standard.rs` and
   `src/standards/iterators_standard.rs` as rewritten in r207. A `Vec`- or
   `HashSet`-backed collection returns the std iterator directly; the custom
   `XxxIter`, its `View`, `iter_invariant`, and both `ForLoopGhostIterator`
   impls are deleted. For-loop invariants use `it.index()`, `it.seq()`,
   `it.history()`.
4. Section 4 of `plans/r206-verus-upgrade-scoping.md` applies verbatim:
   algorithms and their cost specifications must not change. The only exec
   changes allowed are (a) the iterator type substitution above, with the same
   `iter()` cost, and (b) replacing a `HashSetWithViewPlus<T>` field by the
   `std::collections::HashSet<T>` it wraps. `/// - Alg Analysis` lines are
   never edited.
5. Never add `assume`, `accept`, `// accept hole`, `admit`, `external_body`,
   `requires true`, or `// veracity: no_requires`. Never weaken an `ensures`.
   Existing holes stay as they are; closing them is a later round
   (`docs/AcceptReviewChap01to07.md`).
6. No `sed`, `perl`, regex scripts, or Python on source. Edit tool only, after
   reading the surrounding lines.
7. One Verus process at a time. Read every log in full from `logs/`. Do not
   rerun a validate whose log you have not read.
8. Do not spawn subagents. Do not commit, push, stash, reset, or `git clean`.
9. If any step would require changing a `/// - Alg Analysis` line, or an exec
   body beyond the two substitutions in rule 4, stop that step, leave the code
   as it is, write the proposed change and its work and span before and after
   into the doc under "Needs discussion", and continue with the next step. The
   user decides; the agent does not.

## Phase 0 — vstdplus, so that every isolate run can reach Z3

Measured start: `logs/validate.20260920-184251.log`, `isolate Chap02`: 690
verified, 13 errors (all `vstdplus/seq_set.rs`), 8 warnings (`Types.rs` 4,
`seq_set.rs` 4).

| # | Step | Detail | Check |
|---|------|--------|-------|
| 1 | Comment out vstdplus modules with zero users | in `src/lib.rs`, with a `// r208:` reason: `VecQueue`, `hash_set_specs`, `partial_order`, `sqrt` (0 users each per `docs/VstdplusReview.md` §1; re-derive with `grep -rlE 'vstdplus::<name>\b' src`). Leave every module that any chapter imports, even a later one. `hash_set_with_view_plus`, `hash_map_with_view_plus`, `arc_rwlock` are already commented out | `grep` shows 0 users |
| 2 | Replace `seq_set.rs` with a version built on vstd's finite `Set` | Do not repair the old file. Rename it `src/vstdplus/seq_set_pre_0913.rs` (`git mv`), list it commented out in `lib.rs` with a `// r208:` reason, and write a new `src/vstdplus/seq_set.rs` with the same module path and the same public lemma names, so the 171 call sites in Chap02–06 do not change. Read vstd's `set.rs`, `set_lib.rs`, `seq_lib.rs` (the finite `Set`, `Set::fold` at `set.rs:298`, `to_set` lemmas at `seq_lib.rs:757`, `:1403`, `:2129`, `:2242`, `:2418`) first. For each lemma Chap02–06 uses: where vstd has the same statement (`docs/VstdplusReview.md` §3.20 rows 3–11, 16, 26, 27), the new lemma's body is a call to the vstd lemma; where it has none (the weighted and signed-weighted fold sums used by the 13 `WeightedDirGraphStEph*.rs` files, `lemma_take_one_more_intersect`, the int/nat bridges), restate it over vstd's `Set::fold` with no `finite()` anywhere and prove it. Lemmas nothing in Chap02–06 uses are not carried over; list them in the doc | `isolate Chap02`: 0 errors |
| 3 | Delete the 8 `finite()` sites | `Types.rs:82,83,105,106` (graph-view wf conjuncts); `seq_set.rs:491,524,712,785` (`requires s.finite()` lines) | `isolate Chap02`: 0 warnings |
| 4 | RTT and PTT entries for the commented modules | `grep -lE 'hash_set_with_view_plus|hash_map_with_view_plus|arc_rwlock|VecQueue|partial_order|sqrt|hash_set_specs' tests rust_verify_test/tests Cargo.toml rust_verify_test/Cargo.toml`; comment out each `[[test]]` entry with a `# r208:` reason; do not delete test files | listed in the doc |

## Chapter 2 — `HFSchedulerMtEph.rs`, `FibonacciHFScheduler.rs`

Expected 0 errors after Phase 0 (it had 0 in `20260920-184251`). Record the
`N verified` line. Do not touch the scheduler: its `Mutex`, `Condvar`, thread
spawn, and `panic!` stay as they are.

## Chapter 3 — `InsertionSortStEph.rs`, `KleeneStPer.rs`

No type errors in the last full run. Expected proof errors: none known; Z3
moved from 4.13 to 4.16, so read the log. `KleeneStPer.rs` had one broadcast
rename in r207.

## Chapter 5 — sets, relations, mappings (the large one)

Type errors before r207's comment-outs: `SetStEph.rs` 37, `SetMtEph.rs` 35,
`RelationStEph.rs` 10, `MappingStEph.rs` 4. Three of these files import
`HashSetWithViewPlus`, which is now commented out, so they do not compile at all
until step 1 below is done.

| # | Step | Detail |
|---|------|--------|
| 1 | Move `SetStEph`, `SetMtEph`, `MappingStEph` to vstd's hash model | the field becomes `std::collections::HashSet<T>`; specs come from `vstd::std_specs::hash` (`obeys_key_model::<T>()`, `HashSet::new/insert/remove/contains/len/clear/iter/clone`). vstd's `HashSetWithView<Key>` (`vstd/hash_set.rs`) has the `Set<Key::V>` view APAS wants but no `iter`, `Clone`, or `PartialEq`, so it cannot back `SetStEph`; use the raw `HashSet<T>` (view `Set<T>`) and define `SetStEph`'s view as `self.elements@.map(|x: T| x@)`. Keep every trait method's `ensures` as it is; `valid_key_type` keeps `obeys_key_model`. `spec_setsteph_wf` loses its `finite()` conjunct. Same exec bodies, same O(1) expected cost |
| 2 | Iterators | `iter()` returns `std::collections::hash_set::Iter<'_, T>`; delete `SetStEphIter`, `SetMtEphIter`, `RelationStEphIter`, `MappingStEphIter` and their ghost machinery; the constructor `ensures` follow the standard's triple |
| 3 | `MappingStEph.rs:94` `Map::new` | now `Map::new(dom: Set<K>, fv)`; the domain is `self.mapping@.map(|p| p.0)` or the equivalent `filter`; keep the `view` statement |
| 4 | `MappingStEph.rs` `Set::new` ×2, `RelationStEph.rs` | domain and range comprehensions become `filter`/`map` over the relation's finite view |
| 5 | The `PartialEq::eq` accepts and the `SetMtEph` lock-boundary accepts | leave them; not this round |
| 6 | `KleeneStPer.rs` | verify unchanged |

Order within the chapter: `SetStEph` → `RelationStEph` → `MappingStEph` →
`SetMtEph` → `KleeneStPer`, each followed by `isolate Chap05` and a log read.
The RTTs under `tests/Chap05/` compile against the new field only if they use
the trait API; read them and note any that break, but `rtt.sh` is not run in
this round.

## Chapter 6 — graphs

Type errors before r207: `DirGraphMtEph.rs` 17, `DirGraphStEph.rs` 10,
`LabDirGraphMtEph.rs` 8, `UnDirGraphMtEph.rs` 6, `LabDirGraphStEph.rs` 6,
`UnDirGraphStEph.rs` 4, `LabUnDirGraphMtEph.rs` 4, `LabUnDirGraphStEph.rs` 3.
The 13 `WeightedDirGraphStEph*.rs` files had none.

| # | Step | Detail |
|---|------|--------|
| 1 | `Set::new` comprehensions (59 sites) | every one has the shape `Set::new(\|w\| self@.A.contains((v, w)))` or `Set::new(\|w\| exists \|u\| vertices.contains(u) && ...)`; rewrite as `self@.V.filter(\|w\| ...)`, since `w` ranges over the vertex set, or as `self@.A.filter(...).map(...)`. State each rewrite in the doc with the old and new spec text |
| 2 | Old-model for-loops and iterators | per rule 3; these files re-expose `SetStEph`'s iterator |
| 3 | `Types.rs` graph-view wf | done in Phase 0 step 3 |
| 4 | The lock-boundary accepts in the four Mt files | leave them; not this round |

Order: `DirGraphStEph` → `UnDirGraphStEph` → `LabDirGraphStEph` →
`LabUnDirGraphStEph` → the four Mt files → the 13 weighted files, each followed
by `isolate Chap06`.

## Deliverable: `docs/Chap02to06Validation.md`

1. One table: Chap, file, errors before, `N verified, M errors` after, log name.
2. Per chapter: every edit, with Chap, file, line, old, new, and the reason
   (which upstream change forced it).
3. The vstdplus modules commented out and the RTT/PTT entries commented out.
4. Any exec change beyond rule 4's two permitted substitutions: none is
   expected; if one was unavoidable, its work and span before and after.
5. What is left: holes untouched, RTTs not run, anything that did not verify
   and the error text.
