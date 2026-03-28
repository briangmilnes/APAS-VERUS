# R91 Agent 1 Report — Table StPer/MtEph wf ensures

## Objective
Extend the Table wf ensures work from R90 (TableStEph only) to TableStPer and TableMtEph.

## Changes

### TableStPer (src/Chap42/TableStPer.rs)

**Trait insert ensures strengthened with `spec_stored_value`:**
- New-key case: `updated.spec_stored_value(key@) == value`
- Existing-key case: `old_v == self.spec_stored_value(key@) && updated.spec_stored_value(key@) == r`

**Proof additions (insert impl):**
- Proved that the `choose` in `spec_stored_value` uniquely selects the last entry (where the inserted key lands), using `spec_keys_no_dups` uniqueness.
- For existing-key case: proved old table's `spec_stored_value` equals `witness_old_v` via uniqueness of `found_idx` in old entries.
- Pattern: connect `entries.seq@[last]` to `all@[last]` via `spec_index`, then prove `choose == last` by contradiction with `spec_keys_no_dups`.

Note: `spec_stored_value`, `find_ref`, `empty` wf, `delete` wf, and `insert` wf were all already present in StPer from prior work.

### TableMtEph (src/Chap42/TableMtEph.rs)

**insert: added `self.spec_tablemteph_wf()` to ensures.**
- Requires was already `old(self).spec_tablemteph_wf()`.
- Both branches (key-exists, key-not-found) already proved `spec_keys_no_dups(self.entries@)`. The `obeys_feq` conditions propagate from the precondition.
- Zero proof additions needed — Z3 figured it out from existing assertions.

**delete: added `requires old(self).spec_tablemteph_wf()` and `ensures self.spec_tablemteph_wf()`.**
- Added explicit `spec_keys_no_dups` proof for output entries (subsequence of no-dups input preserves uniqueness, via `src` mapping monotonicity).
- Callers in Chap41, Chap43, Chap52 all verified clean — wf was available at all call sites.

## Validation

| Step | Result |
|------|--------|
| validate isolate Chap42 | 2156 verified, 0 errors |
| validate isolate Chap43 | 2571 verified, 0 errors |
| validate isolate Chap52 | 2769 verified, 0 errors |
| validate isolate Chap41 | 2031 verified, 0 errors |
| validate (full) | 5320 verified, 0 errors |
| RTT | 3076 passed |
| PTT | 157 passed |

## What blocks MtEph spec_stored_value / find_ref

`spec_stored_value` and `find_ref` don't make sense for MtEph. The MtEph `find` returns `Option<V>` (cloned value) — there's no persistent reference through the lock to return as `&V`. MtEph callers that need stored-value identity should use the inner StEph table's `find_ref` while holding the lock.

## Summary

| # | Chap | File | Change | Holes +/- |
|---|------|------|--------|-----------|
| 1 | 42 | TableStPer.rs | insert stored_value ensures + proof | 0 |
| 2 | 42 | TableMtEph.rs | insert wf ensures | 0 |
| 3 | 42 | TableMtEph.rs | delete wf requires + ensures + proof | 0 |
