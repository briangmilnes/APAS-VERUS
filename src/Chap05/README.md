# Chapter 5: Sets, Relations, and Mappings

## Overview

Chapter 5 implements fundamental abstract data types:
- **Set**: Unordered collection of unique elements
- **Relation**: Binary relation (set of pairs)
- **Mapping**: Function from keys to values (also known as a dictionary/map)

## APAS-AI Implementation

The APAS-AI implementations use Rust's standard library:
- `SetStEph` wraps `std::collections::HashSet`
- `RelationStEph` uses `HashSet<Pair<T, U>>`
- `MappingStEph` wraps `std::collections::HashMap`

These are **executable** implementations focused on performance.

## Verus's vstd Implementations

Verus provides **specification-only** (spec) versions in `vstd`:

### `vstd::set::Set<A>`
- **Type**: `Set<A>` is spec-only, backed by `spec_fn(A) -> bool` (a predicate)
- **Key operations**:
  - `Set::empty()` - empty set
  - `Set::full()` - set of all `A`
  - `set.contains(a)` - membership test
  - `set.insert(a)`, `set.remove(a)` - add/remove elements
  - `set.union(s2)`, `set.intersect(s2)` - set operations
  - `set.finite()` - predicate for finite sets
  - `set.len()` - cardinality (for finite sets)
- **Extensional equality**: Use `=~=` to prove sets equal

### `vstd::map::Map<K, V>`
- **Type**: `Map<K, V>` is spec-only, backed by `spec_fn(K) -> Option<V>`
- **Key operations**:
  - `Map::empty()` - empty map
  - `map.dom()` - domain as a `Set<K>`
  - `map[key]` - lookup (requires key in domain)
  - `map.insert(key, val)` - add/update mapping
  - `map.remove(key)` - remove mapping
  - `map.union_prefer_right(m2)` - merge maps
- **Extensional equality**: Use `=~=` to prove maps equal

### Relations

**vstd has no executable relation wrapper**, but:

**`vstd::relations`** provides spec-level relation properties:
- `reflexive`, `transitive`, `symmetric`, `antisymmetric`, etc.
- `total_ordering`, `equivalence_relation`, `partial_ordering`
- These apply to `spec_fn(T, T) -> bool` predicates

**For executable relations as data**, we need to build our own using:
- **Option A**: `HashSetWithView<(T, U)>` - set of pairs, view as `Set<(T, U)>`
- **Option B**: `HashMapWithView<T, HashSetWithView<U>>` - multimap, view as `Map<T, Set<U>>`

## vstd's Executable Wrappers (Already Verified!)

**Good news**: vstd already provides verified executable wrappers!

### `vstd::hash_set::HashSetWithView<Key>`
- **Wraps**: `std::collections::HashSet<Key>`
- **View type**: `Set<<Key as View>::V>`
- **Verified operations**: `new()`, `insert()`, `remove()`, `contains()`, `len()`, `is_empty()`, etc.
- **Trust model**: Uses `#[verifier::external_body]` to trust Rust's std lib implementation
- **Key requirement**: `Key: View + Eq + Hash` and must satisfy `obeys_key_model()`

### `vstd::hash_map::HashMapWithView<Key, Value>`
- **Wraps**: `std::collections::HashMap<Key, Value>`
- **View type**: `Map<<Key as View>::V, Value>`
- **Verified operations**: `new()`, `insert()`, `remove()`, `get()`, `contains_key()`, `len()`, `is_empty()`, etc.
- **Trust model**: Uses `#[verifier::external_body]` to trust Rust's std lib implementation
- **Key requirement**: `Key: View + Eq + Hash` and must satisfy `obeys_key_model()`

### Example Usage

```rust
use vstd::prelude::*;
use vstd::hash_set::*;

verus! {

pub fn example() {
    let mut set = HashSetWithView::<u64>::new();
    
    assert(set@ == Set::<u64>::empty());
    
    set.insert(42);
    assert(set@.contains(42));
    assert(set@.len() == 1);
    
    let contains = set.contains(&42);
    assert(contains);
}

} // verus!
```

## Implementation Strategy

**We chose Option 2: Wrap vstd's wrappers**

### Completed Wrappers

#### `SetStEph<T>`
- Wraps: `HashSetWithView<T>`
- View type: `Set<<T as View>::V>`
- Verified operations: `empty()`, `singleton()`, `size()`, `mem()`, `insert()`
- Bounds: `T: View + Eq + Hash`

#### `MappingStEph<K, V>`
- Wraps: `HashMapWithView<K, V>`
- View type: `Map<<K as View>::V, V>`
- Verified operations: `empty()`, `size()`, `mem()`, `get()`, `insert()`
- Bounds: `K: View + Eq + Hash`, `V: PartialEq`

#### `RelationStEph<T, U>`
- Wraps: `HashSetWithView<(T, U)>`
- View type: `Set<(<T as View>::V, <U as View>::V)>`
- Verified operations: `empty()`, `size()`, `mem()`, `insert()`
- Bounds: `T: View + Eq + Hash + Copy`, `U: View + Eq + Hash + Copy`
- **Note**: Uses Rust tuples `(T, U)` instead of `Pair<T, U>` because vstd's `View` trait is pre-implemented for tuples

### Limitations

- **No iterators**: `HashSetWithView` doesn't expose iterators, so `union()`, `intersection()`, `domain()`, `range()` operations are not yet implementable
- **Copy requirement**: `RelationStEph::mem()` requires `T` and `U` to be `Copy` to avoid move issues with `contains()`

## Files

- `SetStEph.rs` - Set implementation
- `RelationStEph.rs` - Binary relation implementation  
- `MappingStEph.rs` - Mapping/dictionary implementation
- `tests/TestSetStEph.rs` - Set tests
- `tests/TestRelationStEph.rs` - Relation tests
- `tests/TestMappingStEph.rs` - Mapping tests

## References

- [vstd::set documentation](https://verus-lang.github.io/verus/verusdoc/vstd/set/)
- [vstd::map documentation](https://verus-lang.github.io/verus/verusdoc/vstd/map/)
- [Verus Guide: Container verification (BST example)](https://verus-lang.github.io/verus/guide/container_bst.html)

