# R90 Agent 1 Report: Table::empty ensures wf + AdjTableGraph proofs

## Summary

Strengthened Table::empty/insert/delete ensures and proved 3 AdjTableGraph holes.

## Changes

### TableStEph (src/Chap42/TableStEph.rs)

| # | Chap | Function | Change |
|---|------|----------|--------|
| 1 | 42 | `empty()` trait+impl | Added `ensures empty.spec_tablesteph_wf()` |
| 2 | 42 | `insert()` trait+impl | Added `ensures self.spec_tablesteph_wf()` + `spec_stored_value` for inserted key |
| 3 | 42 | `delete()` trait | Added `ensures self.spec_tablesteph_wf()` |

The `spec_stored_value` ensures on `insert()` are:
- New key: `self.spec_stored_value(key@) == value` (exec-level, not just view)
- Existing key: `old_v == old(self).spec_stored_value(key@) && self.spec_stored_value(key@) == r`
  where r is the combine result

These enable callers to track exec-level properties (like wf) of stored values through inserts.

### AdjTableGraphStEph (src/Chap52/AdjTableGraphStEph.rs)

| # | Chap | Function | Change |
|---|------|----------|--------|
| 1 | 52 | `empty()` | Removed external_body, proved. Added requires for `obeys_cmp_spec` + `view_ord_consistent` |

### AdjTableGraphStPer (src/Chap52/AdjTableGraphStPer.rs)

| # | Chap | Function | Change |
|---|------|----------|--------|
| 1 | 52 | `empty()` | Removed external_body, proved (trivial: empty map, vacuous graph closure) |
| 2 | 52 | `from_table()` | Removed external_body, proved (graph closure from requires) |

## Hole Count

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 52 | AdjTableGraphStEph.rs | 9 | 8 | -1 |
| 2 | 52 | AdjTableGraphStPer.rs | 12 | 10 | -2 |
| 3 | 52 | Total Chap52 | 33 | 30 | -3 |

## Verification

- Isolate Chap42: 2156 verified, 0 errors
- Isolate Chap43: 2571 verified, 0 errors (no caller breakage)
- Isolate Chap52: 2769 verified, 0 errors (was 2767)
- Full: 5314 verified, 1 error (pre-existing TableMtEph flakiness, same on clean branch)
- RTT: pre-existing compilation error (CycleDetectStPer)

## Blockers Identified

### 1. Clone Gap (blocks 8+ AdjTableGraph holes)

Table::insert/delete rebuild entries by cloning non-key entries via `clone_plus()`. After the operation, `spec_stored_value(k)` for non-key entries returns the **cloned** value, not the original. Since:
- `#[derive(Clone)]` for AVLTreeSetStEph has no Verus spec
- `clone_plus` only ensures `cloned(x, y)`, which gives view equality but not wf preservation

We cannot prove that stored neighbor sets remain wf after Table mutations. This blocks:
- `insert_vertex`, `insert_edge`, `delete_edge`, `delete_vertex` (StEph)
- Same functions in StPer (plus the combine closure clone issue)

**Potential fix**: Add a `clone_preserves_wf` broadcast axiom for AVLTreeSetStEph (admitted), or add manual Clone impl with wf-preserving ensures.

### 2. Verus ICE on StPer proof bodies

Adding `self.adj.spec_tablestper_wf()` to AdjTableGraphStPer's wf triggers a Verus Internal Compiler Error:
```
abstract datatype should be boxed Datatype(Path(Some("vstd"), ["set" :: "Set"]), ...)
```
This prevents adding Table wf to the StPer graph wf, blocking proofs of num_vertices, has_edge, etc.

### 3. Non-broadcastable type axioms

`obeys_cmp_spec::<V>()` and `view_ord_consistent::<V>()` have no broadcast proofs for generic types. They must be added as requires on constructors or assumed in wf. For StEph, this required adding requires to `empty()`.

## Techniques Used

1. **feq trigger firing**: `assert(obeys_feq_full_trigger::<T>())` to activate broadcast axioms for feq predicates in proof bodies
2. **spec_stored_value uniqueness via spec_keys_no_dups**: Proved `choose` selects the unique index by contradiction from no-duplicate-keys invariant
3. **Ghost exec sequence tracking**: Captured `old_exec_seq = self.entries.seq@` to reason about exec-level stored values across the entries reassignment
4. **Find-then-insert pattern**: For StPer insert_vertex, checking existence first avoids the clone-in-combine issue (though Verus ICE prevented completion)

## Steps Used: 15 of 20
