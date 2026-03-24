# R71 Agent 1: Iterator Standard Review — Chap05-06, Chap17, Chap18/ArraySeq

## Goal

Review 14 files for compliance with the iterator standard (`src/standards/iterators_standard.rs`
and `src/standards/wrapping_iterators_standard.rs`). This is a **review-only** task — produce
a report, do NOT modify source files.

## Files to Review

| # | Chap | File |
|---|------|------|
| 1 | 05 | MappingStEph.rs |
| 2 | 05 | RelationStEph.rs |
| 3 | 05 | SetMtEph.rs |
| 4 | 05 | SetStEph.rs |
| 5 | 06 | DirGraphMtEph.rs |
| 6 | 06 | DirGraphStEph.rs |
| 7 | 06 | LabDirGraphMtEph.rs |
| 8 | 06 | LabDirGraphStEph.rs |
| 9 | 06 | LabUnDirGraphMtEph.rs |
| 10 | 06 | LabUnDirGraphStEph.rs |
| 11 | 06 | UnDirGraphMtEph.rs |
| 12 | 06 | UnDirGraphStEph.rs |
| 13 | 17 | MathSeq.rs |
| 14 | 18 | ArraySeq.rs |

## The Standard (10 Required Components)

Read `src/standards/iterators_standard.rs` FIRST. The standard defines 10 components:

1. **Custom iterator struct** — e.g. `FooIter<'a, T>` with `inner: std::slice::Iter<'a, T>`
2. **View for iterator** — `type V = (int, Seq<T>)`, delegates to `self.inner@`
3. **iter_invariant spec fn** — `0 <= it@.0 <= it@.1.len()`
4. **Iterator::next** — two-arm ensures (None: unchanged, pos >= len; Some: pos advances, element at old pos)
5. **Ghost iterator struct** — `FooGhostIterator<'a, T>` with `pos: int, elements: Seq<T>, phantom`
6. **ForLoopGhostIteratorNew impl** — `ghost_iter()` creates ghost state from exec iterator
7. **ForLoopGhostIterator impl** — 6 spec fns: `exec_invariant`, `ghost_invariant`, `ghost_ensures`, `ghost_decrease`, `ghost_peek_next`, `ghost_advance`
8. **View for ghost iterator** — `type V = Seq<T>`, returns `self.elements.take(self.pos)`
9. **iter() method** — ensures `it@.0 == 0, it@.1 == self.data@, iter_invariant(&it)`
10. **IntoIterator for &Self** — enables `for x in &collection`

Optional: **IntoIterator for Self** (consuming pattern) — uses `std::vec::IntoIter<T>`.

Also read `src/standards/wrapping_iterators_standard.rs` — MappingStEph wraps RelationStEph,
RelationStEph wraps SetStEph, so they use the wrapping pattern.

## Review Checklist Per File

For each file, check:

- [ ] Has all 10 components? List which are present/missing.
- [ ] Iterator struct wraps `std::slice::Iter<'a, T>` (or inner module's iterator for wrappers)?
- [ ] View type is `(int, Seq<T>)` and delegates to `self.inner@`?
- [ ] iter_invariant is `0 <= it@.0 <= it@.1.len()`?
- [ ] Iterator::next ensures matches the two-arm pattern exactly?
- [ ] Ghost iterator has `pos: int, elements: Seq<T>, phantom`?
- [ ] ForLoopGhostIteratorNew creates ghost from exec?
- [ ] ForLoopGhostIterator has all 6 spec fns with correct bodies?
- [ ] Ghost iterator View is `self.elements.take(self.pos)`?
- [ ] iter() ensures: `it@.0 == 0, it@.1 == self.data@, iter_invariant(&it)`?
- [ ] IntoIterator for &Self present with matching ensures?
- [ ] IntoIterator for Self (consuming) present? (Optional but note if missing.)
- [ ] Has `// 10. iterators` section header?
- [ ] Iterator next() body: does it delegate to `self.inner.next()`? Or hand-rolled with pos/len?
- [ ] Any `assume` or `accept` in iterator code?
- [ ] Any `unsafe` in iterator code?
- [ ] Any `external_body` on iterator functions?
- [ ] Debug/Display impls for iterator and ghost iterator structs?

## Output Format

Write report to `plans/agent1-round71-report.md`.

For each file produce a row in a summary table:

| # | Chap | File | Components | Missing | Issues |
|---|------|------|------------|---------|--------|

Then for each file with issues, a detailed section listing exactly what's wrong.

At the end, a summary: how many files fully comply, how many need work, total missing components.

## Constraints

- **Review only.** Do NOT modify any source files.
- Read each file's iterator section carefully.
- Do NOT run validate/rtt/ptt.
- Write report to `plans/agent1-round71-report.md` when done.
