# Agent 2 — R119 Hole Fills Report

## Summary

Investigated 3 holes across 3 files (Chap43, Chap52, Chap53). All assumes are
structural — they bridge gaps in Verus's RwLock API, table view boundary, and
closure ensures instantiation. None could be removed.

## Results by File

### 1. Chap43/OrderedSetMtEph.rs — 1 hole (10 assumes), ALL STRUCTURAL

All assumes stem from a single structural gap: Verus's RwLock API provides no
mechanism to prove that the locked value matches the ghost tracking field
(`ghost_locked_set`). The `View` impl returns `ghost_locked_set@`, but after
`acquire_read`/`acquire_write`, the locked value is a separate object with no
proven relationship to the ghost field.

| # | Chap | Line | Category | Assume | Why structural |
|---|------|------|----------|--------|----------------|
| 1 | 43 | 290 | B: result | `count == self@.len()` | inner.size() ensures count == inner@.len(), but inner@ != self@ (ghost gap) |
| 2 | 43 | 320 | B: result | `found == self@.contains(x@)` | inner.find() ensures found == inner@.contains(x@), but inner@ != self@ |
| 3 | 43 | 330 | C: capacity | `locked_val@.len() + 1 < usize::MAX` | trait requires old(self)@.len()+1 < MAX, but locked_val@ != self@ |
| 4 | 43 | 378 | C: capacity | `locked_val@.len() + other_ref@.len() < usize::MAX` | same gap for union |
| 5 | 43 | 404 | A: view eq | `inner@ =~= self@` | RwLock inv has no connection to ghost field |
| 6 | 43 | 461 | A: view eq | `inner@ =~= self@` | same, in first() |
| 7 | 43 | 473 | A: view eq | `inner@ =~= self@` | same, in last() |
| 8 | 43 | 485 | A: view eq | `inner@ =~= self@` | same, in previous() |
| 9 | 43 | 497 | A: view eq | `inner@ =~= self@` | same, in next() |
| 10 | 43 | 504 | C: capacity | `locked_val@.len() + 1 < usize::MAX` | same gap for split |
| 11 | 43 | 511-512 | D: result wf | `left/right.spec_orderedsetsteph_wf()` | split returns wf from inner, but inner wf != proven at ghost level |
| 12 | 43 | 524 | C: capacity | `locked_val@.len() + other_inner@.len() < usize::MAX` | same gap for join |
| 13 | 43 | 535 | C: capacity | `inner@.len() + 1 < usize::MAX` | same gap for get_range |
| 14 | 43 | 538 | D: result wf | `range.spec_orderedsetsteph_wf()` | get_range result wf from inner |
| 15 | 43 | 549 | A: view eq | `inner@ =~= self@` | same, in rank() |
| 16 | 43 | 562 | A: view eq | `inner@ =~= self@` | same, in select() |
| 17 | 43 | 569 | C: capacity | `locked_val@.len() + 1 < usize::MAX` | same gap for split_rank |
| 18 | 43 | 575-577 | D: result wf | `left/right.spec_orderedsetsteph_wf()` | split_rank result wf from inner |

**Root cause**: Verus's `RwLock` provides `RwLockPredicate::inv(v)` after
acquire, guaranteeing the locked value is wf. But the Mt wrapper's `self@` is
defined as `self.ghost_locked_set@` (a separate ghost field), not as the locked
value's view. There is no API to prove `locked_value@ == ghost_field@`. This is
the standard RwLock ghost tracking gap that affects all Mt modules.

**Would fix**: A Verus RwLock extension that returns both the locked value and a
ghost proof that it matches an expected view. Or a `type_invariant` on the Mt
struct that relates the ghost field to the lock contents (but type_invariant
can't reference lock contents).

### 2. Chap52/AdjTableGraphMtPer.rs — 1 hole, STRUCTURAL

| # | Chap | Line | Assume | Why structural |
|---|------|------|--------|----------------|
| 1 | 52 | 440 | `neighbors.spec_avltreesetmtper_wf()` | Map callback receives `&AVLTreeSetMtPer<V>` but table's contract operates at view level (`Map<V::V, Set<V::V>>`). Value wf info is lost at the view boundary. |

**Root cause**: `OrderedTableMtPer::map` iterates stored values and passes them
to a callback. The table's wf predicate and the graph's wf predicate track
key-level invariants (domain finiteness, graph closure) but not value-level wf
(`spec_avltreesetmtper_wf`). The table's `@` maps keys to value views
(`Set<V::V>`), not concrete `AVLTreeSetMtPer<V>` objects, so value wf cannot be
expressed through the view boundary.

**Would fix**: Add a value-wf conjunct to `spec_adjtablegraphmtper_wf` and
thread it through all table operations. Requires cross-chapter changes to
OrderedTableMtPer to expose concrete-value-level wf guarantees — significant
structural work beyond a single hole fill.

### 3. Chap53/GraphSearchMtPer.rs — 1 hole, STRUCTURAL

| # | Chap | Line | Assume | Why structural |
|---|------|------|--------|----------------|
| 1 | 53 | 179 | `neighbors.spec_avltreesetmtper_wf()` | Graph closure `G: Fn(&V) -> AVLTreeSetMtPer<V>` returns a set but Verus can't instantiate a forall-ensures constraint for MtPer types. |

**Root cause**: Attempted the `forall|v, r| graph.ensures((v,), r) ==>
r.spec_avltreesetmtper_wf()` pattern (which works for StEph in
GraphSearchStEph.rs and PQMinStEph.rs). For MtPer, Verus fails to prove
`neighbors.spec_avltreesetmtper_wf()` even with explicit trigger and proof
assertions. The error indicates `ghost_set_view.finite()` is "uninterpreted"
after forall instantiation — Verus cannot resolve the trait method body through
the generic closure ensures. This appears to be a Verus limitation specific to
`AVLTreeSetMtPer` (Arc+RwLock wrapping) vs `AVLTreeSetStEph` (direct tree).

**Attempted**: Added `forall|v, r| #[trigger] graph.ensures((v,), r) ==>
r.spec_avltreesetmtper_wf()` to all function requires and loop invariants.
Tried explicit proof assertions, different trigger placements, raw
`r@.finite()` instead of `spec_avltreesetmtper_wf()`. All failed with
"function is uninterpreted" on `ghost_set_view.finite()`.

**Would fix**: The same forall-ensures pattern works for StEph types. The MtPer
version may need a Verus fix for trait method resolution through closure ensures,
or a redesign using a GoodGraph trait wrapper that hides the closure.

## Holes Before/After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 43 | OrderedSetMtEph.rs | 1 | 1 | 0 |
| 2 | 52 | AdjTableGraphMtPer.rs | 1 | 1 | 0 |
| 3 | 53 | GraphSearchMtPer.rs | 1 | 1 | 0 |

**Total**: 3 holes before, 3 holes after, delta 0.

No code changes made. All files verified clean (the only error in isolate runs
is a pre-existing Chap37/AVLTreeSeqStEph.rs rlimit failure, unrelated).
