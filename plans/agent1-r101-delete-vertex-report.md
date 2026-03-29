# Agent1 R101 Report: delete_vertex graph closure proof

## Objective

Prove the delete_vertex graph closure assumes in AdjTableGraphStEph and AdjTableGraphStPer.
These were the last 2 actionable Chap52 holes that Agent4 R100 proved but lost in merge conflicts.

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 52 | AdjTableGraphStEph.rs | 1 | 0 | -1 |
| 2 | 52 | AdjTableGraphStPer.rs | 1 | 0 | -1 |

**Chap52 total: 5 → 3 holes** (remaining 3 are in AdjTableGraphMtPer.rs: 2 num_edges capacity, 1 rwlock predicate).

## Technique

The delete_vertex loop iterates over the domain, removing `v@` from each neighbor set.
The graph closure invariant (`adj[u].contains(w) ==> dom.contains(w)`) requires proving
that `v@` is no longer in ANY neighbor set after the loop.

### Proof structure

1. **New loop invariant**: `forall|j| 0 <= j < i ==> (dom.contains(seq@[j]) ==> !adj[seq@[j]].contains(v@))` — tracks that processed keys have had `v@` removed from their neighbor sets.

2. **`clone_plus` + `lemma_cloned_view_eq`**: Changed `seq.nth(i).clone()` to `seq.nth(i).clone_plus()` with a `lemma_cloned_view_eq` call. This establishes `u@ == seq@[i]` which is needed for `no_duplicates` to fire (proving `seq@[j] != u@` for `j < i`).

3. **`seq.lemma_view_index(i)`**: Bridges `seq@[i]` (Seq::index on the View) with `seq.spec_index(i)@` (the stored element's view).

4. **Helper forall**: `assert forall|k| k != u@ && dom.contains(k) ==> pre_insert.dom().contains(k) && adj@[k] == pre_insert[k]` — needed because nested quantifier instantiation fails with `=~=` domain equalities.

5. **`seq@.no_duplicates()`**: Added as loop invariant to keep the fact available for Z3.

6. **Post-loop proof**: From the completed v-removal invariant (`i == len`), for any `k` in the domain, `k` appears in `seq@.to_set()` (since `seq@.to_set() ⊇ domain`). Z3 Skolemizes the existential to get an index `j`, then the invariant gives `!adj[k].contains(v@)`. Combined with `adj[u].contains(w)`, this gives `w != v@`, enabling the old graph closure to prove `dom.contains(w)`.

### Key Z3 challenges overcome

- **Nested quantifier instantiation**: `=~=` generates forall, which doesn't instantiate inside another forall. Solved with a helper forall at top level.
- **`clone()` has no ensures**: Standard `Clone::clone` doesn't preserve view in Verus. Replaced with `clone_plus()` + `lemma_cloned_view_eq`.
- **`no_duplicates` trigger**: Needs both `seq@[j]` and `seq@[i]` as terms. `lemma_view_index` makes `seq@[i]` visible to Z3.

## Verification

- `scripts/validate.sh`: 5391 verified, 0 errors
- `scripts/rtt.sh`: 3083 passed
- `scripts/ptt.sh`: 157 passed

## Steps used: 8 of 20
