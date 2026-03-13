# vstd Total Order Infrastructure

Review of `~/projects/verus/source/vstd/` ordering infrastructure as of March 2026.

## Core Files

| # | File | Role |
|---|---|---|
| 1 | `vstd/std_specs/cmp.rs` | External trait specs for `PartialEq`, `PartialOrd`, `Ord` |
| 2 | `vstd/laws_cmp.rs` | Laws and lemmas for comparison operators |
| 3 | `vstd/relations.rs` | Relation properties (`total_ordering`, `partial_ordering`, etc.) |
| 4 | `vstd/seq_lib.rs` | `sorted_by`, `sort_by` |
| 5 | `vstd/std_specs/num.rs` | Numeric type ordering specs |
| 6 | `vstd/std_specs/option.rs` | Option ordering specs |

## Architecture: Three Layers

### Layer 1: Trait Extensions (`cmp.rs`)

Each Rust comparison trait gets a Verus spec mirror:

- **`PartialEqSpec`**: `eq_spec(&self, &other) -> bool`, gated by `obeys_eq_spec()`.
- **`PartialOrdSpec`**: `partial_cmp_spec(&self, &other) -> Option<Ordering>`, gated by `obeys_partial_cmp_spec()`.
- **`OrdSpec`**: `cmp_spec(&self, &other) -> Ordering`, gated by `obeys_cmp_spec()`.

Plus convenience blanket traits `PartialEqIs` / `PartialOrdIs` (added July 2025) giving `is_lt`, `is_le`, `is_gt`, `is_ge` shortcuts.

### Layer 2: Laws (`laws_cmp.rs`)

Opaque predicates bundling expected algebraic properties:

```rust
obeys_partial_cmp_spec_properties::<T>()   // Equal <==> eq_spec, Less <==> flip Greater, transitivity
obeys_cmp_partial_ord::<T>()               // eq_spec <==> Equal, obeys flags
obeys_cmp_ord::<T>()                       // partial_cmp == Some(cmp)
obeys_cmp_spec::<T>()                      // master predicate combining all above
```

Broadcast lemmas prove `obeys_cmp_spec::<T>()` for all integer types (u8..u128, i8..i128, usize, isize) and `Option<T>` (conditional on inner type).

### Layer 3: Relations (`relations.rs`)

Pure spec-fn predicates over `spec_fn(T, T) -> bool`:

```
total_ordering        = reflexive + antisymmetric + transitive + strongly_connected
strict_total_ordering = irreflexive + antisymmetric + transitive + connected
partial_ordering      = reflexive + transitive + antisymmetric
pre_ordering          = reflexive + transitive
equivalence_relation  = reflexive + symmetric + transitive
```

Plus set extrema: `is_least`, `is_minimal`, `is_greatest`, `is_maximal`.

## Key Design Decisions

1. **`obeys_*_spec()` gating pattern.** Every trait extension has a static `obeys_X() -> bool` flag. Postconditions are conditional: `ensures Self::obeys_cmp_spec() ==> r == self.cmp_spec(other)`. Types opt in by returning `true`. This avoids unsound blanket assumes.

2. **Opaque laws with reveal.** `obeys_partial_cmp_spec_properties` is opaque. You must `reveal()` it to use transitivity or symmetry in proofs. Deliberate: avoids trigger storms.

3. **Floats are uninterpreted.** f32/f64 use `uninterp spec fn` for all comparison results. No `obeys_cmp_spec() == true` for floats. Users supply domain axioms.

4. **`sorted_by` uses strict less-than.** `sorted_by(s, less_than)` checks `forall i < j: less_than(s[i], s[j])`. But `sort_by` takes a `leq` function and recommends `total_ordering(leq)`.

## Implications for APAS-VERUS

Our current `TotalOrder` trait in BST/OrderStat chapters defines its own `spec_leq` and `lemma_total_ordering`. The vstd infrastructure now provides a standard path:

- Use `obeys_cmp_spec::<T>()` as the requires instead of a custom `TotalOrder` trait.
- Bridge via `cmp_spec` / `partial_cmp_spec` instead of custom `spec_leq`.
- Get transitivity/antisymmetry from `reveal(obeys_partial_cmp_spec_properties)` instead of custom lemmas.
- Use `sorted_by` from `seq_lib` directly.

The July 2025 additions (`PartialEqIs`, `PartialOrdIs`) are convenience. The core architecture is the three-layer `obeys_*_spec` pattern. This is strictly more general than our custom `TotalOrder` trait and connects to the broader vstd ecosystem (sorted sequences, set operations, etc.).
