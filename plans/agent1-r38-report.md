# Agent 1 — R38 Report: Chap47 Flat Hash Tables + ParaHashTable Warnings

## Baseline

- Main at `485299d3`, 4332 verified, 0 errors, 204 holes total
- Chap47: 51 holes (46 assume, 3 assume(false), 2 external_body), 15 warnings

## Summary

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|------------|-------|
| 1 | 47 | ParaHashTableStEph.rs | 1 | 1 | 0 |
| 2 | 47 | LinProbFlatHashTableStEph.rs | 6 | 6 | 0 |
| 3 | 47 | QuadProbFlatHashTableStEph.rs | 6 | 6 | 0 |
| 4 | 47 | DoubleHashFlatHashTableStEph.rs | 9 | 9 | 0 |
| 5 | 47 | StructChainedHashTable.rs | 5 | 5 | 0 |
| **Total** | | | **27** | **27** | **0** |

No holes eliminated. All 27 holes require architectural changes to the type system
or spec design. See analysis below.

## Verification

- 4332 verified, 0 errors (unchanged)
- 2613 RTT passed, 147 PTT passed

## Hole Analysis: Why Each Category Is Not Provable

### Pattern A: Eq Bridge (14 holes across 4 files)

```
assume(eq == spec_flat_has_key(table.table@[slot as int], key))
```

**Root cause**: `spec_flat_has_key(entry, k)` compares keys using spec-level `==`
(`ek == k`), but `PartialEq::eq(&k, &key)` provides no spec-level ensures for
generic `Key: StT`. The vstd `PartialEq::eq` spec is:

```
ensures Self::obeys_eq_spec() ==> r == self.eq_spec(other)
```

For generic Key, `obeys_eq_spec()` is uninterpreted (could be false), so the ensures
gives no information about `r`. Even if `obeys_eq_spec()` were true, `eq_spec` is
uninterpreted for generic types — we cannot prove `eq_spec(a, b) <==> (a == b)`.

**Experimentally confirmed**: Removed the assume at LinProbFlatHashTableStEph.rs:132
and ran validate. Verus reported assertion failure: cannot prove `eq ==> ek == key`
or `ek == key ==> eq`. Both directions fail.

**What would fix it**: Add `PartialEqSpecImpl` (with `obeys_eq_spec() = true` and
`eq_spec(a, b) = (a == b)`) to StT's trait bounds, OR change `spec_flat_has_key`
to use `eq_spec` instead of `==`. Both are invasive changes affecting the entire
codebase. Changing `spec_flat_has_key` to `eq_spec` also complicates the
no-duplicate-keys uniqueness proofs because `eq_spec` is uninterpreted for generic
types.

### Pattern B: Clone Bridge (6 holes in 3 flat hash files)

```
assume(key == pairs@[j as int].0)   // Clone bridge for Key
assume(value == pairs@[j as int].1) // Clone bridge for Value
```

**Root cause**: Same fundamental gap. `Clone::clone` for generic `Key: StT` has no
spec-level ensures in vstd. The clone returns a new value with no guaranteed
relationship to the original in spec land.

**What would fix it**: Add `Clone::clone` ensures `cloned == *self` to vstd for
types satisfying some injective-View property, OR restructure resize to avoid
cloning (e.g., use `Vec::pop()` for ownership transfer). The pop approach requires
a permutation lemma for `spec_seq_pairs_to_map` (order-independent for unique keys)
which is non-trivial.

### Pattern C: Table Full — assume(false) (3 holes)

```
assume(false); // Table full: unreachable with load factor < 1
```

**Root cause**: The insert function's requires is `num_elements < usize::MAX`, but
the table-full condition requires `num_occupied + num_deleted < current_size`. The
wf spec doesn't track `num_deleted`. Even a pigeonhole argument on `num_elements`
fails because Deleted slots are non-Empty: a table with m Deleted slots and 0
Occupied slots has `num_elements = 0 < m` but no Empty slot available.

**What would fix it**: Track `num_deleted` in the table struct, add
`num_elements + num_deleted < current_size` to insert's requires or wf, and prove
pigeonhole. Alternatively, add `num_elements < current_size` as a simpler
approximation (ignoring deleted slots).

### DoubleHash Wf Bridges (3 holes in DoubleHashFlatHashTableStEph.rs)

```
assume(forall |j: int| ... exists |n: int| ... (hh + n * step as int) % m == j ...)
```

**Root cause**: `compute_second_hash` is `external_body`, so Verus doesn't know the
runtime `step` value corresponds to the existential witness `s` in the wf spec. The
wf spec uses `exists |s: int| s >= 1 && ...` but the code computes a specific step.

**What would fix it**: Either (1) make `compute_second_hash` non-external (requires
bridging `std::hash::Hasher` which is infeasible), or (2) add a spec function for
the second hash and store its result as a ghost field, or (3) change the wf spec to
parametrize on the step value.

### ParaHashTableStEph Warnings (8 fn_missing_wf, 1 external_body)

The 8 veracity warnings are **false positives**. The trait methods already have wf
via `Self::spec_impl_wf(table)` (line 564-566), which defaults to
`spec_hashtable_wf(table)` and is overridden by each impl
(e.g., `spec_linprobflathashsteph_wf` for LinProb).

Cannot add `spec_hashtable_wf(table)` directly to the trait methods because
`spec_hashtable_wf` requires keys to live at their hash slot — this DOES NOT hold
for flat hash tables where collision probing displaces keys. Adding it would make
flat hash table operations uncallable.

The `call_hash_fn` external_body (line 463) wraps an opaque `Fn` closure. Verus
cannot reason about generic `Fn` closures, so external_body is structural.

### StructChainedHashTable Eq/Clone Bridges (4 holes) + resize external_body

Same Pattern A (eq bridge) and Pattern B (clone bridge) as flat hash tables.
The resize external_body is structural: the function body is complete with
invariants but Verus cannot verify it through the external_body annotation.
Removing external_body would require proving the collect-reinsert cycle preserves
the abstract map, which is a substantial proof obligation.

## Architectural Recommendations

To make Chap47 holes provable in future rounds:

1. **Add PartialEqSpecImpl to StT** (or create a new bound `StTWithEqSpec`): This
   enables PartialEq::eq to provide spec-level ensures for generic key types.
   Impact: all modules using StT.

2. **Change spec_flat_has_key to use eq_spec**: Once PartialEqSpecImpl is available,
   `spec_flat_has_key(entry, k)` can use `ek.eq_spec(&k)` instead of `ek == k`.
   The eq bridges become trivially provable. Impact: FlatHashTable.rs + all 3 flat
   hash table wf specs + all proofs using spec_flat_has_key.

3. **Track num_deleted for table-full proof**: Add a `num_deleted: usize` ghost
   field and maintain `num_elements + num_deleted < current_size` in insert's
   requires. Impact: ParaHashTableStEph trait + all impls.

4. **Store second hash as ghost field for DoubleHash**: Compute the second hash
   once and store as `ghost step: int` in the table, then use it in the wf spec
   instead of an existential. Impact: DoubleHash only.

## Techniques Attempted

- Direct assertion of eq bridge without assume → Verus error (confirmed by experiment)
- Analysis of vstd PartialEq ensures for generic types → conditional on obeys_eq_spec
- Analysis of changing spec_flat_has_key to use @-equality → breaks wf when hash
  function isn't View-aware
- Analysis of changing spec_flat_has_key to use eq_spec → viable but requires
  PartialEqSpecImpl bounds cascading through the codebase
- Pigeonhole argument for table-full → fails because Deleted slots are non-Empty
- Pop-based resize to eliminate clone bridges → requires permutation lemma for
  spec_seq_pairs_to_map
