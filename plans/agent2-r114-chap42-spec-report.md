# R114 Agent 2 — Chap42 TableMtEph Spec Strengthening Report

## Objective

Strengthen `TableMtEphTrait` specs to match `TableStEphTrait` counterparts,
closing compare-par-mut warnings for MtEph vs StEph.

## File Modified

| # | Chap | File | Warnings Before | Warnings After |
|---|------|------|-----------------|----------------|
| 1 | 42 | TableMtEph.rs | 50 (total phase 4) | 29 |

MtEph-specific warnings: 21 before, 2 after. Remaining 2 are MtEph STRONGER
than StEph (extra `spec_tablemteph_wf()` ensures on singleton, extra
`obeys_feq_clone::<K>()` requires on insert).

The 27 remaining warnings are all StEph-vs-StPer comparisons (out of scope).

## Changes

### Trait declaration (requires/ensures added)

| # | Function | Change |
|---|----------|--------|
| 1 | `domain` | Added `domain.spec_arraysetsteph_wf()` ensures |
| 2 | `tabulate` | Added `forall` ensures (closure result = tabulated value) |
| 3 | `map` | Added `forall` ensures (closure result = mapped value) |
| 4 | `intersection` | Added `wf` requires (self, other), `obeys_feq_clone`, `obeys_view_eq`; added combine ensures |
| 5 | `union` | Added `wf` requires (self, other), `obeys_feq_clone`, `obeys_view_eq`; added combine ensures for overlap |
| 6 | `difference` | Added `wf` requires, `obeys_view_eq` |
| 7 | `delete` | Added `obeys_view_eq` requires |
| 8 | `insert` | Added combine ensures for key-exists case |
| 9 | `restrict` | Added `wf` requires |
| 10 | `subtract` | Added `wf` requires |

### Proof work (impl bodies)

- **tabulate**: Proved closure ensures via `lemma_entries_to_map_get` + loop invariant witness.
- **map**: Proved closure ensures connecting `f.ensures` from loop invariant through map get.
- **intersection**: Added ghost `old_self_raw`/`other_raw` tracking, combine ensures invariant,
  strict self_srcs ordering, no-dups proof, and post-loop combine witness.
- **union**: Added ghost `old_self_raw`/`other_raw`/`combine_idx`/`phase1_kept` tracking.
  Phase 1 invariant carries combine ensures. Phase 2 invariant carries
  phase1_kept preservation + phase2_sources not-in-self constraint.
  Post-loop proof: no-dups across Phase 1/2, combine witness for overlap keys.
- **insert**: Added ghost `old_entry_raw`/`combine_result`, loop invariant for
  `final_value@` at match_index, post-loop combine ensures witness.

## Verification Results

- **validate**: 5388 verified, 0 errors
- **RTT**: 3529 passed
- **PTT**: 221 passed
