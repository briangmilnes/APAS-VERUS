// SKIPPED: All AugOrderedTableStPer iterator PTTs.
// Reason: AugOrderedTableStPer requires a reducer function with Clone bound.
// Verus does not recognize Clone on function items or closures, so singleton()
// cannot be called in PTT context. The borrow iterators themselves work — the
// construction is the blocker.
