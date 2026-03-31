# Veracity Bug: compare-par-mut subsumption misses feq implication chains

## Problem

`veracity-compare-par-mut` Phase 4 already detects when a requires clause
is subsumed by the MtEph wf predicate. For example, it correctly reports:

```
src/Chap42/TableMtEph.rs:597: info: `filter`: requires clause
  `obeys_feq_full :: < Pair < K , V > > ()` subsumed by MtEph wf predicate
```

But it emits **false-positive warnings** for two cases where the subsumption
holds through one additional implication step:

### False positive 1: `obeys_feq_full::<V>()` not recognized as subsumed

```
src/Chap42/TableMtEph.rs:663: warning: `find`: StEph has requires clause
  `obeys_feq_full :: < V > ()` with no match in MtEph
```

The wf predicate includes `obeys_feq_fulls::<K, V>()`, which is defined as
`obeys_feq_full::<K>() && obeys_feq_full::<V>()`. So `obeys_feq_full::<V>()`
is subsumed by wf through `obeys_feq_fulls`.

**Implication chain:** `spec_tablemteph_wf` → `obeys_feq_fulls::<K,V>()`
→ `obeys_feq_full::<V>()`.

### False positive 2: `obeys_feq_clone::<Pair<K,V>>()` not recognized as subsumed

```
src/Chap42/TableMtEph.rs:672: warning: `delete`: StEph has requires clause
  `obeys_feq_clone :: < Pair < K , V > > ()` with no match in MtEph
```

The wf predicate includes `obeys_feq_full::<Pair<K, V>>()`, which is defined as
`obeys_feq_view_injective::<T>() && obeys_feq_clone::<T>() && obeys_feq_eq::<T>()`.
So `obeys_feq_clone::<Pair<K,V>>()` is subsumed by wf through `obeys_feq_full`.

**Implication chain:** `spec_tablemteph_wf` → `obeys_feq_full::<Pair<K,V>>()`
→ `obeys_feq_clone::<Pair<K,V>>()`.

## Root Cause

The subsumption checker matches exact clause strings against the wf predicate's
body. It finds direct conjuncts (like `obeys_feq_full::<Pair<K,V>>()`) but does
not expand function definitions to find transitive implications.

## feq Implication Rules

These are the spec function definitions from `src/vstdplus/feq.rs`:

```rust
pub open spec fn obeys_feq_full<T: Eq + View + Clone + Sized>() -> bool {
    obeys_feq_view_injective::<T>()
    && obeys_feq_clone::<T>()
    && obeys_feq_eq::<T>()
}

pub open spec fn obeys_feq_fulls<T: ..., U: ...>() -> bool {
    obeys_feq_full::<T>() && obeys_feq_full::<U>()
}
```

So the complete subsumption lattice for feq predicates is:

```
obeys_feq_fulls::<K,V>()
  ├── obeys_feq_full::<K>()
  │     ├── obeys_feq_view_injective::<K>()
  │     ├── obeys_feq_clone::<K>()
  │     └── obeys_feq_eq::<K>()
  └── obeys_feq_full::<V>()
        ├── obeys_feq_view_injective::<V>()
        ├── obeys_feq_clone::<V>()
        └── obeys_feq_eq::<V>()
```

Any clause that appears as a leaf or subtree of a wf conjunct should be
reported as `info: ... subsumed by MtEph wf predicate`, not as a warning.

## Suggested Fix

When checking whether a missing requires clause is subsumed by the MtEph wf
predicate, expand one level of spec function definitions in the wf body
before matching. Specifically:

1. Parse the wf body into conjuncts (already done).
2. For each conjunct that is a call to `obeys_feq_full` or `obeys_feq_fulls`,
   expand it into its constituent sub-predicates.
3. Check the missing clause against both the original conjuncts AND the
   expanded sub-predicates.

This would turn the two false-positive warnings into info lines like:

```
info: `find`: requires clause `obeys_feq_full :: < V > ()`
  subsumed by MtEph wf predicate (via obeys_feq_fulls :: < K , V > ())
info: `delete`: requires clause `obeys_feq_clone :: < Pair < K , V > > ()`
  subsumed by MtEph wf predicate (via obeys_feq_full :: < Pair < K , V > > ())
```

## Impact

These two false positives generate 4 warning lines (2 count mismatches +
2 missing clause warnings). Fixing them would reduce Chap42 Phase 4
warnings from 4 to 0.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace
on Rust source. All edits must be token-aware or AST-aware. Parse
ensures/requires blocks with brace/comma/semicolon awareness. A
string-hacking detector will flag and kill tools that corrupt source syntax.
