# Agent 3 — R97 Report: AdjTableGraphMtPer insert_edge assumes

## Objective

Prove remaining MtPer assumes in `AdjTableGraphMtPer.rs` using strengthened
OrderedTableMtPer ensures.

## Result

**3 assumes removed from insert_edge** via feq clone-view lemma + domain tracking.

| # | Chap | File | Metric | Before | After | Delta |
|---|------|------|--------|--------|-------|-------|
| 1 | 52 | AdjTableGraphMtPer.rs | algorithmic | 10 | 7 | -3 |
| 2 | 52 | AdjTableGraphMtPer.rs | rwlock:predicate | 5 | 5 | 0 |
| 3 | 52 | AdjTableGraphMtPer.rs | total assumes | 15 | 12 | -3 |
| 4 | 52 | (all Chap52) | total holes | 26 | 23 | -3 |

## Assumes removed (3)

| # | Chap | File | Function | Assume | Technique |
|---|------|------|----------|--------|-----------|
| 1 | 52 | AdjTableGraphMtPer.rs | insert_edge | `updated.spec_adj().dom().contains(u@)` | feq lemma + insert domain ensures |
| 2 | 52 | AdjTableGraphMtPer.rs | insert_edge | `updated.spec_adj().dom().contains(v@)` | feq lemma + insert domain ensures |
| 3 | 52 | AdjTableGraphMtPer.rs | insert_edge | `new_adj@.dom().len() + 1 < usize::MAX` (2nd insert capacity) | domain size tracking through match + fn requires |

## Key proof technique

**feq clone-view chain**: The fundamental blocker for domain containment was that
for generic `V`, `u.clone()@` is not guaranteed to equal `u@`. When
`OrderedTableMtPer::insert(u.clone(), ...)` is called, the domain gets
`u.clone()@` added, not `u@`. Without clone-view equivalence, can't prove
`u@ ∈ dom`.

Solution: The `vstdplus::feq` module provides `lemma_cloned_view_eq::<V>(x, y)`,
which requires `cloned(x, y)` + `obeys_feq_full::<V>()` and proves `x@ == y@`.

1. Assert `obeys_feq_full_trigger::<V>()` — triggers broadcast axiom, establishes
   `obeys_feq_full::<V>()` (V satisfies `Eq + View + Clone + Sized` via StT).
2. Bind `let u_clone = u.clone()` — Verus emits `cloned(u, u_clone)`.
3. Call `lemma_cloned_view_eq::<V>(u, u_clone)` — proves `u@ == u_clone@`.
4. `new_adj.insert(u_clone, empty)` ensures `dom =~= old.dom().insert(u_clone@)`.
5. Since `u_clone@ == u@`: `dom =~= old.dom().insert(u@)`, so `u@ ∈ dom`.

**Domain size tracking**: After the first conditional insert (match on find u),
assert `new_adj@.dom().len() <= orig_dom_len + 1` in both arms. At the second
conditional insert, this gives `dom.len() + 1 <= orig_dom_len + 2 < usize::MAX`
(from fn requires `dom.len() + 2 < usize::MAX`).

## Changes made

1. **Added feq import**: `obeys_feq_full_trigger` from `vstdplus::feq::feq`.
2. **Restructured insert_edge**: Changed from `if find.is_none()` to `match find`
   for both conditional inserts. Named clone results, called `lemma_cloned_view_eq`
   in proof blocks. Added domain-containment assertions per match arm.
3. **Added ghost domain size tracking**: `let ghost orig_dom_len = self.adj@.dom().len()`,
   with `assert(new_adj@.dom().len() <= orig_dom_len)` in Some arm and
   `assert(new_adj@.dom().len() <= orig_dom_len + 1)` in None arm.

## 12 remaining assumes — all blocked

| # | Chap | File | Function | Blocker |
|---|------|------|----------|---------|
| 1 | 52 | AdjTableGraphMtPer.rs | num_edges | algorithmic sum reasoning |
| 2 | 52 | AdjTableGraphMtPer.rs | num_edges | algorithmic sum reasoning |
| 3 | 52 | AdjTableGraphMtPer.rs | insert_vertex | insert value ensures |
| 4 | 52 | AdjTableGraphMtPer.rs | delete_vertex | map value/domain ensures |
| 5 | 52 | AdjTableGraphMtPer.rs | delete_vertex | map domain ensures |
| 6 | 52 | AdjTableGraphMtPer.rs | insert_edge | insert value ensures (u_neighbors wf) |
| 7 | 52 | AdjTableGraphMtPer.rs | insert_edge | insert value ensures (neighbor len) |
| 8 | 52 | AdjTableGraphMtPer.rs | insert_edge | capacity tight (needs +3 in requires, have +2) |
| 9 | 52 | AdjTableGraphMtPer.rs | insert_edge | insert value ensures (graph wf) |
| 10 | 52 | AdjTableGraphMtPer.rs | insert_edge | insert value ensures (edge membership) |
| 11 | 52 | AdjTableGraphMtPer.rs | delete_edge | insert value ensures (graph wf) |
| 12 | 52 | AdjTableGraphMtPer.rs | delete_edge | insert value ensures (postcondition) |

**Common blockers**:
- **Insert value ensures** (9 of 12): `OrderedTableMtPer::insert` is `external_body`
  and only ensures domain info, not `updated@[k@] == v@` or value preservation.
  Graph closure requires knowing values at all keys.
- **Map value/domain ensures** (2 of 12): `OrderedTableMtPer::map` ensures only
  `dom.finite()`, not domain preservation or value correspondence.
- **Sum reasoning** (2 of 12): `num_edges` needs a framework for domain-iteration
  sum correspondence.
- **Capacity tight bound** (1 of 12): Final insert in insert_edge needs
  `dom.len() + 1 < usize::MAX` but after two conditional inserts, dom.len() can
  be orig + 2, and fn requires only gives orig + 2 < usize::MAX.

## Validation

```
verification results:: 5388 verified, 0 errors
RTT: 3083 passed
PTT: 157 passed
```
