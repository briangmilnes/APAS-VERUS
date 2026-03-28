# Agent 4 — R92 Report: delete_vertex external_body removal

## Objective

Remove `external_body` from `delete_vertex` in all 3 AdjTableGraph files (Chap52).

## Results

| # | Chap | File | Change | Assumes Added |
|---|------|------|--------|---------------|
| 1 | 52 | AdjTableGraphStEph.rs | external_body removed | 3 (domain wf, neighbor wf, graph wf) |
| 2 | 52 | AdjTableGraphStPer.rs | external_body removed | 3 (domain wf, neighbor wf, graph wf) |
| 3 | 52 | AdjTableGraphMtPer.rs | external_body removed | 2 (graph wf, !dom.contains(v@)) |

## Verification

- Full validate: 5368 verified, 0 errors
- RTT: 3083 passed
- PTT: 157 passed

## Chap52 Holes

Before: 53 (38 assume, 8 rwlock, 7 external_body)
After: 55 (41 assume, 9 rwlock, 5 external_body)

Net: -2 external_body (complete function bodies now verified), +5 assumes for
ICE-blocked properties. The net hole count increased by 2 because each
external_body (1 hole) was replaced by 2-3 assumes.

## Approach

### StEph (ephemeral, mutable)

Loop pattern: delete v from table, iterate domain seq, for each remaining key
find its neighbor set, delete v from it, insert back. Loop invariant tracks
table wf, type predicates, and `!self.adj@.dom().contains(v@)`. The invariant
that v stays out of the domain is fully proved through the loop (delete removes
it, insert(u,...) where u is in domain doesn't re-add it).

Key optimization: pass `v` directly to `delete` and `neighbors.delete` instead
of cloning, eliminating the clone-view assume entirely.

### StPer (persistent, functional)

Same algorithm but persistent: `adj.delete(v)` returns new table, loop builds
new `result_adj` through persistent insert. Pattern mirrors StEph.

### MtPer (multi-threaded, persistent)

Simplest body: `adj.delete(v)` then `map` with a closure that deletes v from
each neighbor set. No loop needed. Weak `OrderedTableMtPer::map` ensures
(only `dom().finite()`) require assuming all postconditions.

## Assumes — Why Each Is Needed

| Assume | Files | Reason |
|--------|-------|--------|
| `domain.spec_arraysetsteph_wf()` | StEph, StPer | `domain()` ensures don't expose wf on result. Implementation maintains it but trait spec doesn't say so. |
| `ns_ref.spec_avltreesetsteph_wf()` | StEph, StPer | Stored neighbor-set wf requires quantifier over `Map<V::V, Set<V::V>>` domain — triggers Verus ICE. |
| `self.spec_adjtablegraphsteph_wf()` | StEph, StPer | Graph closure quantifier `forall|u, v| adj[u].contains(v) ==> dom.contains(v)` triggers Verus ICE on `Set<V::V>`. |
| `updated.spec_adjtablegraphmtper_wf()` | MtPer | Same ICE + weak `OrderedTableMtPer::delete/map` ensures. |
| `!updated.spec_adj().dom().contains(v@)` | MtPer | Weak `OrderedTableMtPer::delete` ensures (only `dom().finite()`, no view spec about removed key). |

## What's Verified vs Assumed

**Verified (inside verus!):**
- Loop structure, bounds, termination (StEph, StPer)
- Table operations type-check against their contracts
- `v` stays out of domain through the loop (StEph, StPer — proved from delete + insert postconditions)
- Closure body in map compiles correctly (MtPer)

**Assumed (ICE-blocked):**
- Graph-level wf (requires `Set<V::V>` quantifier in proof body)
- Stored neighbor-set wf (same quantifier)
- Domain wf from `domain()` (missing ensures, not ICE)
- MtPer postconditions (weak `OrderedTableMtPer` ensures, not ICE)

## Techniques

- Pass `v: &V` directly to `delete`/`neighbors.delete` instead of cloning, avoiding clone-view assumes
- Loop invariant carries type predicates (`obeys_view_eq`, `obeys_cmp_spec`, `obeys_feq_fulls`) needed by table/set operations
- `reveal(obeys_view_eq)` at function entry for find_ref/delete preconditions
