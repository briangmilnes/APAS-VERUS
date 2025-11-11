# Verus Iterators in vstd

## Overview

Verus provides specifications for standard Rust iterators. Each iterator has a **ghost iterator** structure that tracks iteration state in spec mode, enabling verification of loops.

## Common Pattern

All vstd iterators follow this pattern:

- **Ghost Iterator Fields:**
  - `pos: int` - Current position (how many elements consumed)
  - `elements/keys/values/kv_pairs: Seq<T>` - Full unchanging sequence of all elements
  
- **Ghost Iterator View (`it@`):**
  - Returns consumed elements: `it.elements.take(it.pos)`
  - This is what you've iterated over so far

## Iterator Specifications

### 1. std::slice::Iter<'_, T>

**Location:** `vstd/std_specs/slice.rs:62`

**Ghost Iterator:**
```rust
pub struct IterGhostIterator<'a, T> {
    pub pos: int,
    pub elements: Seq<T>,
    pub phantom: Option<&'a T>,
}
```

**View:** `it@ = it.elements.take(it.pos)`

**Usage:** Borrowed iteration over slice elements
```rust
for x in it: slice.iter()
    invariant
        it.elements == original_seq,
        it.pos <= it.elements.len(),
        v@ == it@,  // v contains consumed elements
```

---

### 2. std::vec::IntoIter<T, A>

**Location:** `vstd/std_specs/vec.rs:311`

**Ghost Iterator:**
```rust
pub struct IntoIterGhostIterator<T, A: Allocator> {
    pub pos: int,
    pub elements: Seq<T>,
    pub phantom: core::marker::PhantomData<A>,
}
```

**View:** `it@ = it.elements.take(it.pos)`

**Usage:** Owned iteration over Vec (consumes the Vec)
```rust
for x in it: vec.into_iter()
    invariant
        it.elements == original_seq,
        v@ == it@,
```

---

### 3. std::collections::vecdeque::Iter<'_, T>

**Location:** `vstd/std_specs/vecdeque.rs:263`

**Ghost Iterator:**
```rust
pub struct IterGhostIterator<'a, T> {
    pub pos: int,
    pub elements: Seq<T>,
    pub phantom: Option<&'a T>,
}
```

**View:** `it@ = it.elements.take(it.pos)`

**Usage:** Borrowed iteration over VecDeque elements

---

### 4. std::collections::hash_map::Keys<'_, K, V>

**Location:** `vstd/std_specs/hash.rs:285`

**Ghost Iterator:**
```rust
pub struct KeysGhostIterator<'a, Key, Value> {
    pub pos: int,
    pub keys: Seq<Key>,
    pub phantom: Option<&'a (Key, Value)>,
}
```

**View:** `it@ = it.keys.take(it.pos)`

**Usage:** Iterate over HashMap keys only

---

### 5. std::collections::hash_map::Values<'_, K, V>

**Location:** `vstd/std_specs/hash.rs:394`

**Ghost Iterator:**
```rust
pub struct ValuesGhostIterator<'a, Key, Value> {
    pub pos: int,
    pub values: Seq<Value>,
    pub phantom: Option<&'a (Key, Value)>,
}
```

**View:** `it@ = it.values.take(it.pos)`

**Usage:** Iterate over HashMap values only

---

### 6. std::collections::hash_map::Iter<'_, K, V>

**Location:** `vstd/std_specs/hash.rs:507`

**Ghost Iterator:**
```rust
pub struct MapIterGhostIterator<'a, Key, Value> {
    pub pos: int,
    pub kv_pairs: Seq<(Key, Value)>,
    pub phantom: Option<&'a (Key, Value)>,
}
```

**View:** `it@ = it.kv_pairs.take(it.pos)`

**Usage:** Iterate over HashMap key-value pairs

---

### 7. std::collections::hash_set::Iter<'_, Key>

**Location:** `vstd/std_specs/hash.rs:1086`

**Ghost Iterator:**
```rust
pub struct SetIterGhostIterator<'a, Key> {
    pub pos: int,
    pub elements: Seq<Key>,
    pub phantom: Option<&'a Key>,
}
```

**View:** `it@ = it.elements.take(it.pos)`

**Usage:** Iterate over HashSet elements (unordered)
```rust
for x in it: hash_set.iter()
    invariant
        it.elements.to_set() =~= original_set,  // Full sequence represents the set
        v@ == it@,                               // Vec matches consumed elements
```

**Key Property:** 
```rust
ensures 
    let (index, s) = hash_set.iter()@;
    index == 0 && s.to_set() == hash_set@ && s.no_duplicates()
```

---

### 8. std::ops::Range<A>

**Location:** `vstd/std_specs/range.rs:31`

**Ghost Iterator:**
```rust
pub struct RangeGhostIterator<A> {
    pub start: A,
    pub cur: A,
}
```

**View:** `it@ = Seq::new(it.cur - it.start, |i| it.start + i)`

**Usage:** Iterate over integer ranges
```rust
for idx in it: 0..n
    invariant
        it.cur == idx,
        v@.len() == it.cur,
```

---

## Key Insights

### Invariant Structure

**Starting invariants:** Properties of the full sequence
- `it.elements == original_seq` (for ordered collections)
- `it.elements.to_set() == original_set` (for sets)

**Maintaining invariants:** Track progress
- `v@.len() == it.pos` (output size matches position)
- `v@ == it@` (output matches consumed elements)

**Ending invariants:** Properties after completion
- `it.pos == it.elements.len()` (consumed everything)

### Type Conversions

For types with Views (like `HashSet<T>` where `T: View`):
- `it.elements` is `Seq<T>` (concrete types)
- `self@` is `Set<T::V>` (view types)
- Need to convert: `it.elements.map(|i: int, t: T| t@).to_set()`

### Common Patterns

**Building a Vec from iteration:**
```rust
let mut v = Vec::new();
for x in it: collection.iter()
    invariant
        v@ == it@,  // v contains exactly what we've consumed
```

**Counting elements:**
```rust
let mut count = 0;
for x in it: collection.iter()
    invariant
        count == it.pos,
```

---

## Notes

- These are **not defined in traits** but as scattered `assume_specification` declarations
- Ghost iterators are implementation details of the `ForLoopGhostIterator` framework
- The `for x in it: expr` syntax is Verus-specific for accessing ghost iterator state
- All sequence-based iterators use `.take()` to represent consumed elements


