<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 1 — Round 58 Report

**Task**: Fix 8 `fn_missing_wf_requires` / `fn_missing_wf_ensures` warnings in
`src/Chap47/ParaHashTableStEph.rs`.

## Result: No net change. Warnings cannot be fixed within constraints.

**Verification**: 4484–4485 verified, 0–1 errors (the 1 error is a pre-existing Z3
nondeterminism issue in `src/Chap43/OrderedSetStPer.rs:910`, unrelated to Chap47).

## Investigation

### What was tried

Added `spec_hashtable_wf(old(table))` / `spec_hashtable_wf(table)` to the requires/ensures
of all 8 flagged functions per the assignment's fix pattern. This eliminated the veracity
warnings but introduced **7 Verus verification errors** in the flat hash table files:

| # | Chap | File | Error |
|---|:----:|---|---|
| 1 | 47 | DoubleHashFlatHashTableStEph.rs | precondition (insert in resize loop) |
| 2 | 47 | DoubleHashFlatHashTableStEph.rs | postcondition (resize ensures) |
| 3 | 47 | LinProbFlatHashTableStEph.rs | precondition (insert in resize loop) |
| 4 | 47 | LinProbFlatHashTableStEph.rs | postcondition (resize ensures) |
| 5 | 47 | QuadProbFlatHashTableStEph.rs | precondition (insert in resize loop) |
| 6 | 47 | QuadProbFlatHashTableStEph.rs | postcondition (resize ensures) |
| 7 | 43 | OrderedSetStPer.rs | Z3 nondeterminism (unrelated) |

All changes were reverted. `ParaHashTableStEph.rs` is unchanged from the R57 state.

### Root cause of the incompatibility

`spec_hashtable_wf` is the **chained table invariant**: it requires every key to be stored
at exactly its home hash slot (`j == hash(k) % current_size`). This is correct for chained
tables (each bucket is a chain, keys go to their home bucket), but fundamentally
incompatible with flat/probe-addressing tables.

Flat tables (LinProb, QuadProb, DoubleHash) override `spec_impl_wf` with their own
probe-chain invariant (`spec_linprobflathashsteph_wf`, etc.), which explicitly allows keys
to be stored at probe positions away from their home slot. After any insertion by a flat
table, `spec_hashtable_wf` is **provably false** — the Z3 debugger confirms:

```
spec_hashtable_wf(new_table) ✘   (key at non-home slot after flat insert)
spec_impl_wf(new_table)     ✔   (probe-chain wf holds)
```

When the flat tables' `resize` implementations call `Self::insert(&mut new_table, ...)` in
a loop to rebuild the table, the new precondition `spec_hashtable_wf(old(new_table))` fails
after the first insertion because the intermediate `new_table` no longer satisfies the
chained invariant.

### Why the fix requires touching other files

To fix the warnings properly, one of the following redesigns is needed:

1. **Modify flat table impls** (DoubleHash, LinProb, QuadProb) to explicitly declare
   weaker `requires` on their `insert` overrides — prohibited by the assignment.

2. **Weaken `spec_hashtable_wf`** to a structural-only predicate (size checks only),
   rename the current key-placement invariant to `spec_chained_hashtable_wf`, and update
   the chained table files — requires touching multiple files outside the scope.

3. **Move `spec_hashtable_wf` out of the parametric trait** — the trait's abstract methods
   should use `Self::spec_impl_wf` only, and `spec_hashtable_wf` should be a
   chained-table-specific spec. This is the correct design but requires refactoring several
   Chap47 files.

### Note on `createTable`

`createTable`'s ensures already contains `spec_hashtable_wf(&table)` (line 420). The
veracity `fn_missing_wf_ensures` warning fires anyway, apparently because veracity matches
`spec_hashtable_wf(table)` (without `&`) but the code has `spec_hashtable_wf(&table)`. The
`&table` form is correct (the function returns owned `HashTable`, so `spec_hashtable_wf`
requires the `&` operator since it takes `&HashTable`). This appears to be a veracity
parser limitation.

## Holes Summary (unchanged from R57)

| # | Chap | File | Actionable Holes | Warnings |
|---|:----:|---|:---:|:---:|
| 1 | 47 | ParaHashTableStEph.rs | 2 | 8 |

Actionable holes: 1 × `assume()` (clone bridge), 1 × `external_body` (hash fn call).
Warnings: 6 × `fn_missing_wf_requires`, 2 × `fn_missing_wf_ensures`.

## Recommendation

The 8 `fn_missing_wf` warnings should be resolved in a dedicated refactoring round that
is permitted to touch all affected Chap47 files. The design fix is:

1. Split `spec_hashtable_wf` into `spec_basic_hashtable_wf` (structural: size checks)
   and `spec_chained_hashtable_wf` (structural + key placement).
2. Have the trait's abstract methods require `spec_basic_hashtable_wf` (satisfiable
   by all implementations).
3. Have `spec_impl_wf` default to `spec_chained_hashtable_wf` for chained tables.
4. Let flat tables continue overriding `spec_impl_wf` with their probe-chain predicates.
