<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# HashSet → HashSetWithView Replacement Fixes

Replace raw `std::collections::HashSet` with `HashSetWithViewPlus` (or vstd `HashSetWithView`) so the data has a correct spec-level view `Set<Key::V>`.

## Current State

- **HashSetWithViewPlus** (vstdplus): wrapper with `pub inner: HashSet<Key>`, `View` gives `Set<Key::V>`.
- **HashSetWithView** (vstd): wrapper with private `m: HashSet<Key>`, same view. APAS uses HashSetWithViewPlus to avoid `pub m` extension issues.
- **View**: `self@ == Set<Key::V>` — the set of view values of keys. For `Key = V` with `View for V` where `V::V = V`, view is `Set<V>`.

## Proposed Fixes Table

| # | File | Line | Current | Key Type | View | Fix |
|---|------|------|---------|----------|------|-----|
| 1 | Chap05/MappingStEph.rs | 597 | `HashSet::new()` | `&A` | N/A | Use `HashSetWithViewPlus<A>` + insert `pair.0.clone()`; requires `A: Clone + View + Eq + Hash`. Macro is `#[cfg(not(verus_keep_ghost))]` so runtime-only. |
| 2 | Chap64/TSPApproxStEph.rs | 79 | `HashSet::<(V,V)>::new()` | `(V,V)` | `Set<(V::V,V::V)>` | Replace with `HashSetWithViewPlus<(V,V)>`. vstd has `View for (A,B)`. Requires `V: View + Eq + Hash`. |
| 3 | Chap64/TSPApproxStEph.rs | 96 | param `HashSet<(V,V)>` | `(V,V)` | same | Change fn sig to `HashSetWithViewPlus<(V,V)>`. |
| 4 | Chap64/TSPApproxStEph.rs | 159 | `HashSet::<V>::new()` | `V` | `Set<V::V>` | Replace with `HashSetWithViewPlus<V>`. Requires `V: View + Eq + Hash`. |
| 5 | Chap65/PrimStEph.rs | 109 | `HashSet::<V>::new()` | `V` | `Set<V::V>` | Replace with `HashSetWithViewPlus<V>`. Requires `V: View + Eq + Hash`. |
| 6 | Chap65/UnionFindStEph.rs | 114 | `HashSet::new()` | `V` | `Set<V::V>` | Replace with `HashSetWithViewPlus<V>`. Requires `V: View + Eq + Hash`. |

## Done (2025-02-19)

All 6 fixes implemented. Validation: 2554 verified. RTT: 2477 passed.

## Already Correct

| File | Usage |
|------|-------|
| Chap05/SetStEph.rs | `HashSetWithViewPlus<T>` for elements |
| Chap05/SetMtEph.rs | `HashSetWithViewPlus<T>` for elements |
| Chap17/MathSeq.rs | `HashSetWithViewPlus<T>` for seen in range() |
| vstdplus/hash_set_with_view_plus.rs | Internal `HashSet` in wrapper — correct |

## Implementation Notes

1. **Key type**: Must be `View + Eq + Hash + Clone` for `HashSetWithViewPlus::new()` and `insert()`.
2. **View impl**: For `(V,V)`, vstd provides `impl View for (A,B)` with `V = (A::V, B::V)`.
3. **obeys_key_model / obeys_feq_full**: `HashSetWithViewPlus::new()` and `insert` require these. Use `broadcast use` of `group_hash_set_with_view_plus_axioms` or ensure key type satisfies them.
4. **Chap64, Chap65**: All HashSet usages are `#[cfg(not(verus_keep_ghost))]` — runtime-only. Replacing with HashSetWithViewPlus gives consistency and enables future verusification.
5. **MappingLit macro**: Uses `&pair.0` — switch to owned `pair.0.clone()` and `HashSetWithViewPlus<A>`; requires `A: Clone` in macro bounds.

## Priority Order

1. **Chap65/UnionFindStEph** — num_sets in trait impl; straightforward V replacement.
2. **Chap65/PrimStEph** — visited set; straightforward V replacement.
3. **Chap64/TSPApproxStEph** — visited, visited_edges; needs (V,V) and V.
4. **Chap05/MappingStEph** — macro; needs clone + HashSetWithViewPlus<A>.
