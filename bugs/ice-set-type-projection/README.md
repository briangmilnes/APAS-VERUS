# ICE: Set<V::V> in proof quantifier — CANNOT REPRODUCE standalone

The standalone reproducer does NOT trigger the crash. All 4 test patterns
in reproducer.rs verify clean.

The ICE occurs only in the full APAS-VERUS codebase context when writing
`assert forall` over `Map<V::V, Set<V::V>>` in AdjTableGraph proof bodies.
The crash likely depends on interactions between TableStEph's `spec_stored_value`
(choose-based), the graph closure quantifier, and the full module compilation context.

## To reproduce from the full codebase

1. In `src/Chap52/AdjTableGraphStEph.rs`, find an `assume(updated.spec_adjtablegraphsteph_wf())`
2. Replace with the actual proof: `assert forall|u, v| ...` graph closure quantifier
3. Run `verus --record` on the full crate
4. Use `source/tools/minimizers/` to shrink the recording

## Status

Parked. The standalone reproducer approach failed. Need full-crate minimization.
