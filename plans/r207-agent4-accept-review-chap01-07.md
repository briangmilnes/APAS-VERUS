# r207 agent 4 — review every `accept`, `// accept hole`, `assume`, and `external_body` in Chap01–07

Read `plans/r207-round-plan.md` first and obey its common rules. This is a
review. You edit nothing under `src/`. Your only output file is
`docs/AcceptReviewChap01to07.md`.

## Question

Chapters 02, 03, 05, and 06 exist (01, 04, 07 have no directory). They contain
68 `accept(...)` calls, 19 `// accept hole` comments, 19 `assume(...)` calls,
and 35 `external_body` attributes. Each is a proof hole recorded under an older
Verus. For each one: what obligation does it discharge, why was it needed then,
and does Verus 0.2026.09.13 or vstd now let us prove it? Propose the concrete
change. Expect many.

Per-file counts at the start:

| # | Chap | File | accept | hole | assume | external_body |
|---|------|------|-------:|-----:|-------:|--------------:|
| 1 | 02 | HFSchedulerMtEph.rs | 0 | 10 | 1 | 6 |
| 2 | 05 | MappingStEph.rs | 1 | 0 | 0 | 0 |
| 3 | 05 | SetMtEph.rs | 4 | 0 | 1 | 0 |
| 4 | 05 | SetStEph.rs | 1 | 0 | 0 | 1 |
| 5 | 06 | DirGraphMtEph.rs | 11 | 0 | 0 | 0 |
| 6 | 06 | LabDirGraphMtEph.rs | 2 | 0 | 2 | 0 |
| 7 | 06 | LabUnDirGraphMtEph.rs | 5 | 0 | 2 | 0 |
| 8 | 06 | UnDirGraphMtEph.rs | 7 | 0 | 0 | 0 |

Re-derive the counts with `grep`; some `accept` calls in Chap02/03 may be in
files the first count missed. Cover every file in the four directories.

## What changed upstream that may close holes

| # | Change | Likely relevance |
|---|--------|------------------|
| 1 | `Set`/`Map` finite by type (`#2486`) | every `accept(x.finite())` and finiteness `assume` is now vacuous and deletable |
| 2 | `Set::new` returns `Option`; `lemma_set_new_some`, `Set::filter` | comprehension holes in Chap06 views |
| 3 | `Ghost<T>: Send + Sync` (since April) | `unsafe impl` or `external_body` around ghost fields |
| 4 | `#2540` impl fns may extend trait specs | holes that bridged trait spec to impl |
| 5 | `#2760` `bool::then` requires the closure precondition; `#2873` `find`/`all`/`any` preconditions | closure-related accepts |
| 6 | `#2522` blanket `IteratorSpecImpl` for `&mut I`; the prophetic model | accepts inside `iter()`/`next()` |
| 7 | HashMap/HashSet specs: `#2513` `clone` fix; `std_specs/hash.rs` at 09.13 | accepts in SetStEph/SetMtEph over `HashSetWithViewPlus` |
| 8 | `#2326` logical atomicity; `#2720` atomics via pointer | HFScheduler thread holes |
| 9 | `#2861` `panic!`/`assert!` supported | diverging arms |
| 10 | `#2884` tracked borrow fns for `Vec`/slice | Arc/RwLock bridge holes |

The four permitted hole patterns (CLAUDE.md) stay: `assume` inside
`PartialEq::eq` and `Clone::clone` bodies, `assume(false); diverge()` in a
thread-join error arm, and `external_body` at a thread-spawn boundary. Classify
those as "permitted pattern" and still say whether 09.13 removes the need.

## Procedure

For each hole, in file order: read the whole function; state the exact
proposition accepted or assumed, or the `ensures` the `external_body`
promises; find the reason in the surrounding comments, `git log -S` on the
line, or the `plans/` reports; search vstd 09.13 for a lemma or spec that
discharges it (`grep -rn` under `~/projects/verus/source/vstd`); write the
proposed replacement proof in a code block. Where the proof needs an
experiment, name the question the experiment must answer and put it in the
doc; do not write the experiment.

## Deliverable: `docs/AcceptReviewChap01to07.md`

1. Summary table: Chap, file, holes by kind, holes proposed closable, holes
   permitted-pattern, holes still open with reason.
2. One entry per hole: Chap, file, line, kind, proposition, original reason,
   upstream change that applies (or none), proposed change with code, effort
   (lines of proof estimated), confidence (an experiment is needed, or not).
3. A list of repeated shapes across files (the same proposition accepted in
   several Mt graph files) so a single transformation can close them together.
4. Experiments needed, one question each.
