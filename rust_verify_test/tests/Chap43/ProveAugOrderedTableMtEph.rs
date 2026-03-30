// SKIPPED: All AugOrderedTableMtEph iterator PTTs.
// Reason: AugOrderedTableMtEph requires MtReduceFn<V> which has a Clone bound.
// Verus does not recognize Clone on function items or closures, so singleton()
// cannot be called in PTT context. The borrow iterators themselves work — the
// construction is the blocker.
