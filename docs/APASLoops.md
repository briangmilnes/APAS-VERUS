# APAS-VERUS Loop Support

## Overview

This document defines which loop patterns APAS-VERUS data structures should support for
formal verification with Verus. Not all Rust loop patterns are equally provable in Verus.

See [ForLoopVerification.md](ForLoopVerification.md) for details on how Verus verifies
`for` loops using the `ForLoopGhostIterator` trait.

## Loop Patterns Supported

| # | Name | Pattern | Support | Requires |
|---|------|---------|---------|----------|
| 1 | loop-loop | `loop { match iter.next() { ... } }` | ✅ Required | `Iterator::next()` with specs |
| 2 | while-next | `while next.is_some()` | ❌ No | Complex invariants |
| 3 | while-let | `while let Some(x) = iter.next()` | ❌ No | Verus limitation |
| 4 | for-iter | `for x in collection.iter()` | ✅ Required | `ForLoopGhostIterator` impl |
| 5 | for-borrow | `for x in &collection` | ✅ Required | `ForLoopGhostIterator` impl |
| 6 | for-range | `for i in 0..collection.size()` | ✅ If indexable | Index operator with specs |
| 7 | while-size | `while i < collection.size()` | ✅ If indexable | Index operator with specs |

## Pattern Details

### loop-loop (Required)

The fundamental iterator pattern in Verus. All APAS collections with iterators must support this.

```rust
let mut it = collection.iter();
loop
    invariant /* user invariants */,
    decreases iter_seq.len() - it@.0,
{
    match it.next() {
        Some(x) => { /* process x */ }
        None => break,
    }
}
```

**Requirements:**
- `iter()` returns an iterator with a view `@` of type `(int, Seq<T>)`
- `next()` has Verus specs establishing invariants (see ForLoopVerification.md)

### while-next & while-let (Not Supported)

**while-next** requires calling `next()` before the loop and at the end of each iteration,
making invariants complex and error-prone.

**while-let** is not supported because Verus does not currently support `let` expressions
in `while` conditions.

### for-iter & for-borrow (Required)

These provide the cleanest syntax for iteration and require implementing `ForLoopGhostIterator`:

```rust
// for-iter: Explicit iter()
for x in collection.iter() { ... }

// for-borrow: Via IntoIterator
for x in &collection { ... }
```

**Requirements:**
- Implement `ForLoopGhostIterator` for the iterator type
- Implement `ForLoopGhostIteratorNew` to create ghost state from exec iterator
- See ForLoopVerification.md for trait method specifications

### for-range & while-size (If Indexable)

Only applicable to collections that support integer indexing (arrays, `Vec`, sequences).
Not applicable to unordered collections like `SetStEph`.

```rust
// for-range
for i in 0..collection.len() {
    let x = collection[i];
}

// while-size
let mut i = 0;
while i < collection.len()
    invariant i <= collection.len(),
    decreases collection.len() - i,
{
    let x = collection[i];
    i += 1;
}
```

**Requirements:**
- `Index` trait implementation with Verus specs
- `len()` or `size()` method with specs

## Implementation Priority

For each APAS data structure with iteration:

1. **First**: Implement loop-loop - foundation for all iteration proofs
2. **Second**: Implement for-iter & for-borrow via `ForLoopGhostIterator` - best ergonomics
3. **If indexable**: Implement for-range & while-size - natural for sequences/arrays

## Current Status

| # | Collection | loop-loop | for-iter | for-borrow | for-range | while-size |
|---|------------|-----------|----------|------------|-----------|------------|
| 1 | SetStEph | ✅ | ❌ | ❌ | N/A | N/A |
| 2 | SetMtEph | ❌ | ❌ | ❌ | N/A | N/A |
| 3 | MappingStEph | ❌ | ❌ | ❌ | N/A | N/A |
| 4 | RelationStEph | ❌ | ❌ | ❌ | N/A | N/A |
| 5 | ArraySeqStEph | ❌ | ❌ | ❌ | ❌ | ❌ |
| 6 | ArraySeqStPer | ❌ | ❌ | ❌ | ❌ | ❌ |
| 7 | ArraySeqMtEph | ❌ | ❌ | ❌ | ❌ | ❌ |
| 8 | ArraySeqMtPer | ❌ | ❌ | ❌ | ❌ | ❌ |
| 9 | LinkedListStEph | ❌ | ❌ | ❌ | N/A | N/A |
| 10 | LinkedListStPer | ❌ | ❌ | ❌ | N/A | N/A |
| 11 | MathSeq | ❌ | ❌ | ❌ | ❌ | ❌ |
