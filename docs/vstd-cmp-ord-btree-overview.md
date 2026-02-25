<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# vstd Equality, Ordering, and BTree Specs: Impact on APAS-VERUS

**Date:** 2026-02-19
**Verus version:** `aa60c2c8` (rolling release 2026-02-24)

## Overview

Verus now ships three layers of equality/ordering infrastructure in vstd that overlap
significantly with what we built in `vstdplus`. This document maps the new vstd
infrastructure, assesses what it replaces, what it doesn't cover, and what chapters
need to change.

## The Three Layers

### Layer 1: cmp.rs — Trait Extension Specs

**File:** `~/projects/verus/source/vstd/std_specs/cmp.rs` (346 lines)
**Landed:** 2025-07-09 (Hawblitzel, PR #1569)
**Updated:** 2025-07-15 (blanket `is_lt`/`is_le`/`is_gt`/`is_ge`)

Provides `external_trait_specification` + `external_trait_extension` for Rust's
four comparison traits. Each spec method is guarded by an `obeys_*_spec()` flag
that must be proven true before the spec is useful.

| Rust Trait | Extension Trait | Spec Method | Guard |
|---|---|---|---|
| `PartialEq` | `PartialEqSpec` | `eq_spec(&self, &other) -> bool` | `obeys_eq_spec()` |
| `Eq` | (marker only) | — | — |
| `PartialOrd` | `PartialOrdSpec` | `partial_cmp_spec(&self, &other) -> Option<Ordering>` | `obeys_partial_cmp_spec()` |
| `Ord` | `OrdSpec` | `cmp_spec(&self, &other) -> Ordering` | `obeys_cmp_spec()` |

**Convenience blanket traits** (`PartialEqIs`, `PartialOrdIs`) provide inlined
spec fns `is_eq`, `is_ne`, `is_lt`, `is_le`, `is_gt`, `is_ge` that desugar into
`eq_spec` / `partial_cmp_spec` comparisons.

**Float specs** are deliberately empty shells. `eq_ensures`, `lt_ensures`, etc. are
uninterpreted functions with no axioms. Users must supply their own.

### Layer 2: laws_eq.rs + laws_cmp.rs — Proof Obligations

**Files:**
- `~/projects/verus/source/vstd/laws_eq.rs` (179 lines)
- `~/projects/verus/source/vstd/laws_cmp.rs` (137 lines)

The master predicate `obeys_cmp_spec<T: Ord>()` is a conjunction of four opaque sub-predicates:

```
obeys_cmp_spec<T>() =
    obeys_eq_spec<T>()                        // T::obeys_eq_spec() + symmetry + transitivity
    && obeys_cmp_partial_ord<T>()             // eq_spec ↔ partial_cmp_spec(Equal), both flags true
    && obeys_cmp_ord<T>()                     // partial_cmp_spec(x,y) == Some(cmp_spec(x,y))
    && obeys_partial_cmp_spec_properties<T>() // Equal↔eq_spec, Less↔Greater, transitivity×2
```

All four are `#[verifier::opaque]`. Proving `obeys_cmp_spec` for a new type requires
revealing each one and proving the five order properties (reflexivity via Equal↔eq_spec,
symmetry via Less↔Greater, two transitivities, plus the eq_spec properties).

**Pre-proven types** via `group_laws_cmp`:

| Category | Types |
|---|---|
| Integer | u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize |
| Compound | `Option<T>` (requires `obeys_cmp_spec::<T>()`) |

**Not covered:** tuples, strings, char, Vec, Box, user structs.

### Layer 3: btree.rs — BTreeMap and BTreeSet Specs

**File:** `~/projects/verus/source/vstd/std_specs/btree.rs` (1193 lines)
**Landed:** 2026-02-23 (two days ago)

Models `BTreeMap<K,V>` as `View<V = Map<K,V>>` and `BTreeSet<K>` as `View<V = Set<K>>`.
Every mutating operation's postcondition is guarded by `obeys_cmp_spec::<Key>()`.

**Specified operations:**

| Operation | BTreeMap | BTreeSet |
|---|---|---|
| new | yes | yes |
| default | yes | yes |
| insert | yes | yes |
| get | yes | yes |
| contains_key/contains | yes | yes |
| remove | yes | yes |
| clear | yes | yes |
| len | yes | yes |
| is_empty | yes | yes |
| clone | yes | yes |
| iter | yes (kv pairs) | yes |
| keys | yes | — |
| values | yes | — |

All iterator types have full `ForLoopGhostIterator` implementations for verified
`for` loops.

The `Borrow<Q>` pattern (for `get(&str)` on `BTreeMap<String, _>`) uses
uninterpreted spec fns (`contains_borrowed_key`, `maps_borrowed_key_to_value`,
`borrowed_key_removed`) with special-case axioms for `Key = Q` and `Key = Box<Q>`.

**Key design detail:** `key_obeys_cmp_spec::<Key>()` is an uninterpreted predicate
linked to `obeys_cmp_spec::<K>()` via broadcast axiom only when `K: Ord`. This
handles BTreeMap's "late binding" where methods like `iter` don't require `K: Ord`
in Rust.

## What This Replaces in vstdplus

### vstdplus/total_order.rs — LIKELY REPLACEABLE

Our `TotalOrder` trait provides:
- `spec fn le(self, other) -> bool`
- `proof fn reflexive`, `transitive`, `antisymmetric`, `total`
- `fn cmp(&self, &other) -> Ordering`

with implementations for all 14 integer types.

vstd's `OrdSpec` + `laws_cmp` now provides the same thing through a different
interface (`cmp_spec` returning `Ordering` instead of `le` returning `bool`,
properties via opaque predicates instead of trait proof fns).

**Difference:** Our `TotalOrder::le` is a direct `spec fn` that callers use in
specs. vstd's equivalent requires going through `cmp_spec` and matching on
`Ordering::Less | Ordering::Equal`. Our trait is more ergonomic for proofs that
reason about `<=`.

**14 chapter files** import `total_order`:
- Chap03: InsertionSortStEph
- Chap35: OrderStatSelect (4 files)
- Chap36: QuickSort (2 files)
- Chap37: BST variants (7 files)
- Chap41: ArraySetStEph (uses `TotalOrder` bound)

### vstdplus/partial_order.rs — LIKELY REPLACEABLE

Our `PartialOrder` trait mirrors `TotalOrder` without the totality proof, plus
`fn compare -> Option<Ordering>`. vstd's `PartialOrdSpec` now covers the same
ground. No chapter files import `partial_order` currently.

### vstdplus/float.rs — NOT REPLACED

Our `FloatTotalOrder` trait provides:
- `float_wf(x)` (excludes NaN/infinity)
- `le(self, other)` via uninterpreted `le_ensures`
- `exec float_cmp` returning `Ordering`
- `broadcast group_float_finite_total_order` (reflexive, antisymmetric, transitive, total)

vstd's float specs are deliberately empty — just uninterpreted `eq_ensures`,
`lt_ensures`, etc. with no axioms at all. Our float infrastructure remains necessary.

**16 chapter files** import `float`.

### PartialEqSpecImpl Pattern — UNCHANGED

Our `PartialEqSpecImpl` pattern (in 37 files across Chap05-Chap50) uses vstd's
`PartialEqSpec` trait extension. This is already aligned with the vstd approach.
The `assume` inside `fn eq` is our standard pattern and is unaffected by the new
cmp infrastructure.

### hash_map/hash_set_with_view_plus — PARTIALLY AFFECTED

Our `obeys_key_model` precondition on HashMap/HashSet operations serves a similar
role to BTree's `obeys_cmp_spec` guard. The HashMap specs in vstd
(`vstd::std_specs::hash`) have their own `obeys_key_model` that we already use.

BTreeMap/BTreeSet could now **replace** some HashMapWithViewPlus usage in chapters
where we need an ordered collection. However, BTree requires `K: Ord` while HashMap
requires `K: Hash + Eq`, so they're not drop-in replacements — it depends on the
algorithm's needs.

## Impact Assessment

### No Change Required (Low Risk)

| # | Area | Why |
|---|---|---|
| 1 | PartialEqSpecImpl pattern | Already aligned with vstd |
| 2 | vstdplus/float.rs | vstd has no float axioms |
| 3 | HashMap/HashSet usage | Different trait bounds |
| 4 | Chap50 (MatrixChain, OBST) | Use HashMap, not ordered collections |

### Potential Migration (Medium Risk)

| # | Area | Files | What Changes |
|---|---|---|---|
| 1 | `TotalOrder` trait | 14 files | Replace with `OrdSpec` + `obeys_cmp_spec` |
| 2 | `TotalOrder::le` in specs | 14 files | Rewrite as `cmp_spec` matches or `is_le` |
| 3 | `TotalOrder::cmp` in exec | 14 files | Use std `Ord::cmp` directly |
| 4 | `TotalOrder` proof calls | 14 files | Replace `reflexive()` etc. with broadcast use |

### Potential New Capability (High Value)

| # | Area | What's New |
|---|---|---|
| 1 | BTreeMap/BTreeSet | Can use std BTree with full specs (for `Ord` key types) |
| 2 | Verified `for` loops on BTree | Full ghost iterator support for BTree iterators |
| 3 | BST chapters (Chap37) | Could validate BST ordering against `cmp_spec` |
| 4 | Priority queues (Chap45) | Could use BTreeSet as verified backing store |
| 5 | Sorted collections (Chap41) | `increasing_seq` predicate for sortedness proofs |

## Migration Strategy

### Phase 0: Don't Touch What Works

The 37 files using `PartialEqSpecImpl` are fine. The 16 files using `float.rs` are
fine. The HashMap-based Chap50 files are fine. Leave them alone.

### Phase 1: Experiment

Before touching any chapter code:

1. Write an experiment in `src/experiments/` that:
   - Implements `OrdSpecImpl` for a simple struct
   - Proves `obeys_cmp_spec` for it
   - Uses a `BTreeMap` with that struct as key
   - Iterates over it with a verified `for` loop

2. Write an experiment that:
   - Uses `is_le` / `is_lt` in specs where we currently use `TotalOrder::le`
   - Calls `Ord::cmp` in exec where we currently call `TotalOrder::cmp`

This will reveal how painful (or not) the migration actually is.

### Phase 2: Migrate vstdplus/total_order.rs Callers

If experiments show the vstd approach is workable:

1. Update the 14 files that import `total_order`
2. Replace `TotalOrder` bounds with `Ord` bounds
3. Replace `TotalOrder::le(x, y)` specs with `x.is_le(&y)` or `x.cmp_spec(&y) is Less | Equal`
4. Replace `TotalOrder::cmp` calls with `Ord::cmp`
5. Replace `TotalOrder::reflexive()` proof calls with `broadcast use group_laws_cmp`
6. Deprecate `vstdplus/total_order.rs`

### Phase 3: Evaluate BTree Adoption

Determine whether any chapters benefit from switching HashMap to BTreeMap:
- Chapters needing ordered iteration (Chap37 BSTs, Chap41 sorted sets)
- Chapters where `Ord` is already required by the algorithm

### Phase 4: Float Axioms

vstd's empty float specs open a question: should we upstream our `FloatTotalOrder`
axioms? Or keep them local? This depends on whether the Verus team wants to take a
position on float determinism (they explicitly declined per RFC 3514 comments in cmp.rs).

## Risks

1. **Opaque predicate ceremony.** vstd's `obeys_cmp_spec` requires revealing four
   opaque predicates. Our `TotalOrder` is direct — you call `reflexive()` and get the
   fact. The vstd approach may be more verbose for proof-heavy chapters.

2. **No `le` spec fn.** vstd doesn't provide a direct `le(self, other) -> bool` spec fn
   the way our `TotalOrder` does. Everything goes through `cmp_spec` or `partial_cmp_spec`.
   This makes simple specs like `forall|i,j| i < j ==> s[i].le(s[j])` wordier.

3. **Broadcast group load.** `group_laws_cmp` brings in proofs for all 12 integer types
   plus Option. If many chapters broadcast-use it, solver load may increase.

4. **User type Ord proofs.** For any struct that implements `Ord`, you must write the
   full `OrdSpecImpl` + prove `obeys_cmp_spec`. There's no `derive` or shortcut. This
   is equivalent effort to implementing our `TotalOrder` trait, just different ceremony.

## Summary

| vstdplus Module | vstd Replacement | Status | Action |
|---|---|---|---|
| `total_order.rs` | `OrdSpec` + `laws_cmp` | Overlapping | Experiment first |
| `partial_order.rs` | `PartialOrdSpec` + `laws_cmp` | Overlapping | Experiment first |
| `float.rs` | Nothing (empty shells) | No replacement | Keep |
| PartialEqSpecImpl pattern | Already uses vstd | Aligned | No change |
| `hash_map_with_view_plus` | BTreeMap (different use case) | Complementary | Evaluate per chapter |
| `hash_set_with_view_plus` | BTreeSet (different use case) | Complementary | Evaluate per chapter |

The bottom line: this is real infrastructure that works for primitive types today.
For user types, it's equivalent ceremony to what we already have — different shape,
same effort. The BTree specs are genuinely new capability. The float story is
unchanged. Experiment before migrating.
