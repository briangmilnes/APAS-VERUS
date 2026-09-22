# r206 — Verus 0.2026.09.13 upgrade: scoping (the plan for the plan)

Date: 2026-09-20. Status: SCOPING. No source files changed.
Companion: `plans/r206-verus-upgrade-restart.md` (toolchain paths, committed at `4bd6b4f92`).

This document records what changed in Verus and vstd between the two releases,
which APAS-VERUS definitions each change affects, and the decisions the
migration plan needs before any agent round starts.

## 1. Current state

| # | Fact | Evidence |
|---|------|----------|
| 1 | Last release with 0 errors: `0.2026.04.20.8dcd677`, 5765 verified, 0 holes | migration plan §1 |
| 2 | May 22 run on `0.2026.05.21`: 178 compile errors in 50 files | `logs/validate.20260522-075541.log.gz` |
| 3 | `src/` is unchanged since that run; only `analyses/` differs | `git diff d08ef3d99..HEAD -- src` |
| 4 | Installed: `0.2026.09.13.671956e`, Rust 1.98.1, Z3 4.16.0 | `~/projects/verus/verus --version` |
| 5 | 283 commits on `main` between the releases; 80 change `source/vstd` | `gh api .../commits?path=source/vstd` |
| 6 | `~/projects/veracity` does not exist; the repository is on GitHub, last push 2026-05-30 | `gh repo list` |
| 7 | 18 scripts call veracity binaries (holes, style, fn-impls, cleanliness) | `grep -l veracity scripts/*` |
| 8 | `~/projects/verus` is now a git checkout at tag `release/0.2026.09.13.671956e`; the prebuilt binaries sit in `source/target-verus/release`; the flat prebuilt is preserved at `~/projects/verus-prebuilt-0.2026.09.13` | `git -C ~/projects/verus describe --tags` |
| 9 | `validate.sh isolate Chap05` on 09.13: 61 name-resolution errors, 0 verified; 20× `group_set_axioms`, 13× `seq_to_set_is_finite`, 26× `vstd::set::fold::lemma_fold_*`, 2× `group_map_axioms`. Type and proof errors cannot appear until these resolve | `logs/validate.20260920-085402.log` |

Every count below excludes `src/experiments/` and `analyses/`.

## 2. Upstream changes that affect APAS-VERUS

Ordered by number of affected files. "Class" states whether the fix is a
rename or re-shaping that a tool can apply, or proof work that Z3 must
re-discharge.

| # | PR | Date | Change | Files | Hits | Class |
|---|----|------|--------|------:|-----:|-------|
| 1 | #2163, #2608, #2736, #2739 | May–Aug | Prophetic iterator model; `initial_value_relation` removed | 119 | ~2,800 | tool + proof |
| 2 | #2486 | Jun 9 | `Set`/`Map` always finite; `finite()` deprecated; `ISet`/`IMap` added | 103 | 1,463 | mechanical + design |
| 3 | #2486 | Jun 9 | `Set::new(f)` returns `Option<Set>` | 21 | 92 | design + proof |
| 4 | #2486 | Jun 9 | `Map::new(fk, fv)` → `Map::new(dom: Set<K>, fv)` | 4 | 5 | proof |
| 5 | #2486, #2777 | Jun–Aug | `group_set_axioms` → `group_set_lemmas`; finiteness axioms removed | 65 | 84 | mechanical |
| 6 | #2486 | Jun 9 | `group_map_axioms` → `group_map_lemmas` | 33 | 33 | mechanical |
| 7 | #2625, #2678 | Jul | `group_seq_axioms` deprecated → `group_seq_lemmas`; `axiom_seq_*` → lemmas | 120 | 158 | mechanical |
| 8 | #2486 | Jun 9 | `seq_to_set_is_finite` removed | 5 | 23 | mechanical |
| 9 | #2486 | Jun 9 | `vstd::set::fold` moved to `iset::fold`; `Set::fold` wraps it | 1 | 26 | mechanical |
| 10 | — | — | `laws_eq::obeys_eq_spec` → `obeys_eq`; `laws_cmp::obeys_cmp_spec` → `obeys_cmp` (deprecated) | ? | 77 | mechanical |
| 11 | #2377 | May | spec `==` requires `SpecEq`; `*cloned == *arc` in `arc_rwlock.rs` fails | 1 | 1 | proof |
| 12 | #2578 | Jul 30 | Z3 4.16.0 | all | — | proof timing may change |
| 13 | #2871 | Sep 1 | New error for spec items in exec code | ? | ? | unknown until the crate compiles |
| 14 | #2929 | Sep 13 | `iter_len` → `ExactSizeIterator::exact_len` | 0 | 0 | experiments only |

Row 1 detail. The 10-component iterator standard no longer compiles. Files
that still implement or consume it:

| # | Chap | Files | Notes |
|---|------|------:|-------|
| 1 | 05 | 4 | SetStEph.rs and SetMtEph.rs produce 72 of the 178 errors |
| 2 | 06 | 21 | 8 graph files plus consumers |
| 3 | 17–19, 23 | 15 | ArraySeq family, LinkedList, tree sequences |
| 4 | 37–43 | 44 | BST, AVL, Table, OrderedSet families |
| 5 | 45, 57–59, 62–66 | 21 | for-loop consumers only |
| 6 | standards | 11 | 6 standards files show the old shape |
| 7 | vstdplus | 2 | hash_set_with_view_plus.rs, hash_map_with_view_plus.rs |
| 8 | PTT | 89 of 97 | `rust_verify_test/tests/**` |

At 09.13, `IteratorSpecImpl` declares five spec fns: `obeys_prophetic_iter_laws`,
`remaining` (prophetic), `will_return_none` (prophetic), `decrease`, and `peek`.
`initial_value_relation` was removed on Aug 1 (#2739). Therefore
`src/standards/prophetic_iterators_standard.rs`, its PTT, and the three
prophetic experiments, all written against 05.21, no longer match the trait.
Loop invariants refer to `it.index()`, `it.seq()`, `it.history()`, and `it.wf()`.
`ExactSizeIteratorSpecImpl::exact_len` and `DoubleEndedIteratorSpecImpl::peek_back`
are optional. Reference implementation: `examples/guide/iterators.rs` at tag
`release/0.2026.09.13.671956e`.

Rows 2–3 detail. `Set::new(|x| p(x))` now has type `Option<Set<A>>` and is
`Some` exactly when `ISet::new(p).finite()` holds (`lemma_set_new_some` and
`lemma_set_new`, both members of `group_set_lemmas`). `Set::filter(f)` still
returns `Set<A>`. Call sites:

| # | Chap | File | Hits | Shape |
|---|------|------|-----:|-------|
| 1 | 06 | DirGraphMtEph.rs | 17 | `Set::new(\|w\| A.contains((v,w)))` |
| 2 | 41 | ArraySetEnumMtEph.rs | 13 | `Set::new(\|i: usize\| i < n && bit(i))` |
| 3 | 06 | DirGraphStEph.rs | 10 | as row 1 |
| 4 | 06 | LabDirGraphMtEph.rs | 8 | as row 1 |
| 5 | 06 | LabDirGraphStEph.rs | 7 | as row 1 |
| 6 | 06 | UnDirGraphMtEph.rs | 6 | as row 1 |
| 7 | 52 | EdgeSetGraphMtPer.rs | 5 | neighbour sets |
| 8 | 06 | UnDirGraphStEph.rs | 4 | as row 1 |
| 9 | 06 | LabUnDirGraphMtEph.rs | 4 | as row 1 |
| 10 | 06 | LabUnDirGraphStEph.rs | 3 | as row 1 |
| 11 | 05 | MappingStEph.rs | 2 | domain and range comprehensions |
| 12 | 52 | EdgeSetGraph{St,Mt}Eph.rs, StPer.rs | 3 | 1 each |
| 13 | — | vstdplus/hash_set_with_view_plus.rs | 1 | — |

`Map::new` call sites: Chap05 `MappingStEph.rs` (the `View` impl), Chap41
`OrdKeyMap.rs`, Chap43 `OrderedSpecsAndLemmas.rs`, Chap62 `StarPartitionMtEph.rs` (2).

`Set::finite()` carries `#[deprecated]` and its body is `true`. There are 1,463
call sites, 605 of them `dom().finite()`, and 32 inside `spec_*_wf` bodies; the
rest are in `requires`, `ensures`, loop invariants, and asserts.
`finite_sets_standard.rs` states that finiteness is carried by wf; at 09.13 it is
carried by the type, so the standard's premise is false.

## 3. Upstream changes that do not affect APAS-VERUS, or that add capability

| # | PR | Change | Relevance |
|---|----|--------|-----------|
| 1 | #2507, #2929 | `ExactSizeIteratorSpecImpl` | optional `len()` spec for custom iterators |
| 2 | #2492, #2515, #2858, #2449 | specs for `map`, `filter`, `zip`, `collect` | possible later replacement for hand-written loops; out of scope |
| 3 | #2831 | `BTreeMap` specs | possible Chap43 representation; out of scope |
| 4 | #2284 | `#[verifier::allow(..)]` | must not be used to suppress warnings |
| 5 | #2861 | `panic!` and `assert!` inside `verus!` | the 37 `assert!` hits are in RTTs |
| 6 | #2888, #2890 | `decreases` through arrays and slices | may shorten tree-sequence proofs |
| 7 | #2727, #2744 | index assignment `v[i] = x`; slice range write-back | may simplify `mut_standard.rs` |
| 8 | #2540 | impl fns may extend trait specs | confirm the trait-impl pattern still verifies |
| 9 | #2780 | `impls_cannot_extend_spec` | not needed |
| 10 | #2591 | `loop_isolation_boundary` attribute | `loop_isolation(false)` appears 462 times; confirm the syntax is unchanged |
| 11 | #2856 | bit-shift precondition forbids negative shift counts | ArraySetEnumMtEph.rs bit operations |

## 4. Rule: algorithms and their cost specifications must not change

A previous upgrade attempt replaced algorithms with ones of different work or
span. Every phase of this migration is subject to the following checks.

| # | Requirement | Check |
|---|-------------|-------|
| 1 | Exec bodies of `iter()`, `next()`, and every algorithm fn are unchanged, except the documented type substitution (wrapper struct → std iterator) | reviewer diffs exec lines per file |
| 2 | No `/// - Alg Analysis` line changes | `git diff -- src \| grep 'Alg Analysis'` is empty each round |
| 3 | No new `external_body`, `assume`, `accept`, or `admit` | `scripts/holes.sh` reports 0 per chapter |
| 4 | No `ensures` weakened; no `requires` strengthened beyond wf | reviewer reads every spec diff |
| 5 | No algorithm replaced to simplify a proof: no sort substitution, no flattening of a lazy iterator, no sequentializing an Mt file | agent prompt states the rule; the round report lists every exec change with its work and span before and after |
| 6 | The 3 lazy AVLTreeSeq iterators keep their O(lg n) `iter()`; they get hand-written `IteratorSpecImpl` impls | migration plan §10 cost table |

Every agent prompt quotes this section verbatim.

## 5. Decisions required before the migration plan is written

| # | Decision | Options | Recommendation |
|---|----------|---------|----------------|
| 1 | Target release | (a) `0.2026.09.13` stable; (b) rolling `0.2026.09.20` | (a); do not move until 0 errors |
| 2 | Restore veracity | clone `briangmilnes/veracity` to `~/projects/veracity`; `cargo build --release`; confirm it builds on Rust 1.98.1 | first task; 18 scripts and the iterator rewriter depend on it |
| 3 | vstd source for search locations 2–4 | done: `~/projects/verus/source/vstd` at the release tag; CLAUDE.md paths already match | none needed |
| 4 | Iterator rewrite | (a) rerun `veracity-iterator-upgrade --apply` on a fixture copy, compare its output against the 09.13 trait, then apply a patch series to live `src/`; (b) edit by hand | (a); 1,256 findings |
| 5 | Rewrite scope | include `rust_verify_test/tests/**` (89 files), or first delete PTTs outside the two permitted cases and rewrite the remainder | delete first, then rewrite |
| 6 | `Set::new` replacement | (a) `filter` on the enclosing finite set; (b) `Set::new(f).unwrap()` plus a finiteness lemma; (c) `ISet` views | (a) where an enclosing set exists (every Chap06 and Chap52 shape); (b) for ArraySetEnumMtEph.rs; never (c) |
| 7 | `finite()` | (a) delete all 1,463 conjuncts and rewrite `finite_sets_standard.rs`; (b) keep them and accept 1,463 deprecation warnings | (a), as its own round after every other error is fixed |
| 8 | Broadcast renames | one round of renames: set, map, and seq groups; `laws_eq` and `laws_cmp`; removed axioms | yes; it is the first round |
| 9 | Ordering | `validate.sh` reports no verified functions until the whole crate compiles, so rounds 1–3 measure compile-error count, not verified count | accept |
| 10 | Agent assignment | 4 worktrees split by chapter group once the crate compiles | as migration plan §6 |

## 6. Proposed phases of the migration plan

| # | Phase | Scope | Acceptance criterion |
|---|-------|-------|----------------------|
| 0 | Tools | restore veracity; `validate.sh isolate Chap05` to confirm the toolchain wiring; record the error count by kind | log read; error inventory written |
| 1 | Renames | §2 rows 5–10: groups, removed axioms, `fold`, `obeys_eq` | compile-error count decreases; no exec-line diff |
| 2 | Set and Map API | §2 rows 3–4, per decision 6 | Chap05, 06, 41, 52, 62 compile |
| 3 | Iterators | rewrite standards and PTTs for 09.13; apply the patch series; hand-write the 3 AVLTreeSeq impls; fix `arc_rwlock.rs` `SpecEq` | whole crate compiles; first verified-function count recorded |
| 4 | Proof repair | per-chapter `isolate` rounds, 4 agents | 5765 verified, 0 errors, 0 holes, 0 trigger notes |
| 5 | RTT, then PTT | `scripts/rtt.sh`, then `scripts/ptt.sh` | all tests pass, none skipped |
| 6 | `finite()` removal | decision 7 | 0 deprecation warnings |
| 7 | Completion | regenerate analyses; refresh the LOC table; write the report; commit; push | RCP |

## 7. Risks

| # | Risk | Mitigation |
|---|------|------------|
| 1 | Z3 4.16 changes proof timing; rlimit failures in Chap35, 36, 65 that were intermittent before | profile before changing any proof |
| 2 | The rewriter's output was compared against 05.21 shapes only | rerun the fixture comparison before applying to live `src/` |
| 3 | #2871 may reject spec-in-exec patterns the codebase uses | unknown until Phase 3; allocate one round to classify the errors |
| 4 | Finite-by-type removes lemmas some proofs used (`seq_to_set_is_finite`, 23 calls) | most calls can be deleted; where a length fact was needed, use `lemma_set_insert_len` |
| 5 | Prophetic `seq()` is not allowed in `decreases`; manual `loop`s need `decrease()` | documented in migration plan §9 |
| 6 | Agents on out-of-date branches change `lib.rs` | merge checklist in CLAUDE.md |

## 8. Sources

- `gh api repos/verus-lang/verus/releases` (release bodies are empty)
- `gh api repos/verus-lang/verus/commits?since=2026-05-21` (283 commits; 80 in `source/vstd`)
- `~/projects/verus` at `release/0.2026.09.13.671956e`: `source/vstd`, `examples/guide/iterators.rs`, `source/docs/guide/src/iterator-specs*.md`
- `logs/validate.20260522-075541.log.gz`; `plans/verus-0.2026.05.21-iterator-migration.md`; `plans/r204-*` and `plans/r205-*` reports
