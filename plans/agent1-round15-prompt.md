# Agent 1 — Round 15

## Status: 149 holes, 4078 verified, 38 clean chapters.

Round 14: you fully proved OrderedTableMtEph (11 → 0). Excellent. Now apply the same
MtEph/MtPer wrapper proving pattern to two more chapters.

## Your files

### Chap42/TableMtEph.rs (11 external_body)

All 11 holes are `external_body` stubs on the Mt wrapper around TableStEph. This is the
exact same pattern you proved for OrderedTableMtEph:
1. Remove `external_body`
2. `acquire_read` or `acquire_write` on the RwLock
3. Call the inner StEph method
4. Bridge ghost state using the lock invariant
5. Release lock, return result

The inner TableStEph is already fully proved (0 holes). Your job is just the Mt wrapper.

Functions to prove: `new`, `size`, `find`, `insert`, `delete`, `collect`, `entries`,
`from_sorted_entries`, `map`, `filter`, `reduce` — whatever the 11 stubs are.

Read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` first.

### Chap41/AVLTreeSetMtPer.rs (11 holes: 3 assume + 8 external_body)

Another Mt wrapper, this time persistent (MtPer). The inner type uses Arc+RwLock.

- 8 `external_body`: same lock-call-bridge pattern as TableMtEph
- 3 `assume`:
  - `assume(r == self@.len())` — size: bridge from inner StPer's ensures
  - `assume(obeys_feq_full::<T>())` — feq bridge: use `axiom_obeys_feq_full` broadcast
  - `assume(!self@.contains(x@))` — delete postcondition: bridge from inner

For the feq assume: you added `axiom_obeys_view_eq` to feq.rs in R13. Use
`broadcast use crate::vstdplus::feq::group_feq_axioms;` to get the axiom.

## DO NOT

- Touch Chap43 (Agent 2)
- Touch Chap39 (Agent 3)
- Touch Chap37, Chap47, Chap45, Chap41/MtEph, Chap41/StEph, Chap41/EnumMtEph (Agent 4)
- Touch Chap38

## Rules

- Run `scripts/validate.sh` after every change.
- NO accept(). NO assume→accept conversion.
- Read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` before starting.
- Read `src/standards/using_closures_standard.rs` before writing closure code.
- Push to `agent1/ready`. Write `plans/agent1-round15-report.md`.

## Target: -12 (stretch -18). Close Chap42.
