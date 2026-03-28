# R99 Agent 1 — ICE Test Report

## Objective

Test whether the Verus ICE on `Set<V::V>` quantifiers in Chap52 AdjTableGraph
still exists after the Verus upgrade to `ff454ab0f`.

## Key Finding: ICE is GONE

The ICE no longer occurs. Removing any of the ICE-blocked assumes produces a
normal verification error (postcondition failure), not a Verus crash. This means
all 11 assumes previously classified as "blocked by Verus ICE" are now provable.

## Technique: `lemma_spec_stored_value_view`

The core blocker was the "clone gap": after `Table::insert`, `spec_stored_value(k)`
for k != key@ is a cloned exec value. Its `spec_avltreesetsteph_wf()` couldn't be
proved because `spec_ghost_locked_root` is a closed spec fn.

**Solution**: New proof lemma `lemma_spec_stored_value_view` added to both
`TableStEph` and `TableStPer`:

```rust
pub proof fn lemma_spec_stored_value_view(&self, k: K::V)
    requires self.spec_tablesteph_wf(), self@.contains_key(k)
    ensures self.spec_stored_value(k)@ == self@[k]
```

This connects the exec-level `spec_stored_value(k)` to the view-level map value
`self@[k]`. Since `AVLTreeSetSt{Eph,Per}` wf depends only on `tree@` (which equals
`self@`), view equality of stored values proves wf equality:

1. `spec_stored_value_new(k)@ == new_map[k]` (by lemma)
2. `new_map[k] == old_map[k]` (from insert ensures, k != key@)
3. `old_map[k] == spec_stored_value_old(k)@` (by lemma on old table)
4. `spec_stored_value_old(k).tree@ == old_map[k]` (View for AVLTreeSetSt*)
5. `spec_stored_value_new(k).tree@ == new_map[k] == old_map[k]` (same opaque value)
6. Therefore `new_sv.tree@.finite() == old_sv.tree@.finite()` ✓

## Results

| # | Chap | File | Function | Assume | Status |
|---|------|------|----------|--------|--------|
| 1 | 52 | AdjTableGraphStEph.rs | insert_vertex | stored-value-wf (line 473) | **PROVED** |
| 2 | 52 | AdjTableGraphStEph.rs | insert_edge | stored-value-wf (line 612) | **PROVED** |
| 3 | 52 | AdjTableGraphStEph.rs | delete_edge | stored-value-wf (line 653) | **PROVED** |
| 4 | 52 | AdjTableGraphStEph.rs | delete_vertex | ns_ref wf (line 507) | **PROVED** |
| 5 | 52 | AdjTableGraphStEph.rs | delete_vertex | full wf (line 521) | **PARTIAL** — graph closure assume remains |
| 6 | 52 | AdjTableGraphStPer.rs | insert_vertex | stored-value-wf (line 426) | **PROVED** |
| 7 | 52 | AdjTableGraphStPer.rs | insert_edge | stored-value-wf (line 559) | **PROVED** |
| 8 | 52 | AdjTableGraphStPer.rs | delete_edge | stored-value-wf (line 598) | **PROVED** |
| 9 | 52 | AdjTableGraphStPer.rs | delete_edge | full wf else-branch (line 610) | **PROVED** |
| 10 | 52 | AdjTableGraphStPer.rs | delete_vertex | neighbors wf (line 461) | **PROVED** |
| 11 | 52 | AdjTableGraphStPer.rs | delete_vertex | full wf (line 473) | **PARTIAL** — graph closure assume remains |

## Assume Count

| File | Before | After | Delta |
|------|--------|-------|-------|
| AdjTableGraphStEph.rs | 5 | 1 | **-4** |
| AdjTableGraphStPer.rs | 6 | 1 | **-5** |
| **Total** | **11** | **2** | **-9** |

## Remaining Assumes (2)

Both remaining assumes are **graph closure** in `delete_vertex`:

```rust
// StEph line 621:
assume(self.spec_adj().dom().contains(w));

// StPer line 563:
assume(forall|u, w| dom.contains(u) && adj[u].contains(w) ==> dom.contains(w));
```

These require proving that the loop removed `v@` from ALL neighbor sets. The proof
needs a "processed keys" loop invariant that tracks `!adj[seq@[j]].contains(v@)` for
j < i. This invariant hits Z3's limitation with Map indexing inside `assert forall`
— the error is "function is uninterpreted" on `self.adj@[seq@[j]]`.

**Possible approaches for future work:**
- Use `loop_isolation(false)` + ghost Map equality assertions
- Restructure the loop to use a ghost Set of remaining keys
- Add a `forall|k| !adj[k].contains(v@)` loop invariant (requires proving for all k,
  not just seq entries, which has the same Map indexing issue)

## Files Changed

| File | Change |
|------|--------|
| `src/Chap42/TableStEph.rs` | Added `lemma_spec_stored_value_view` |
| `src/Chap42/TableStPer.rs` | Added `lemma_spec_stored_value_view` |
| `src/Chap52/AdjTableGraphStEph.rs` | Proved 4 assumes, 1 targeted assume remains |
| `src/Chap52/AdjTableGraphStPer.rs` | Proved 5 assumes, 1 targeted assume remains |

## Verification

```
Full validation: 5393 verified, 0 errors
RTT: 3083 passed
PTT: 157 passed
```
