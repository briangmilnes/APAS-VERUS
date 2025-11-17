# APAS-VERUS For-Loop Audit

**Date**: 2025-11-17  
**Total For-Loops Found**: 72 loops across 15 source files

## Summary of Patterns

### ✅ Pattern 1: Range-Based For Loops (SAFE - 32 instances)
**Works**: Index-based iteration with `0..len` or `0..n`  
**Verification**: ✅ Fully supported by Verus built-in range iterators

```rust
for i in 0..len
    invariant
        len == s1.elements.len(),
        sn@ =~= s1.elements@.take(i as int),
{
    sn.push(s1.elements[i]);
}
```

**Files using this pattern**:
- `src/experiments/seq_for_basic_proofs.rs` - 14 instances
- `src/experiments/simple_seq_iter.rs` - 1 instance (for_range)
- `src/experiments/verus_wrapped_iter_loops.rs` - 3 instances
- `src/experiments/VSTDLoopProofs.rs` - 3 instances
- `src/experiments/executable_use_of_int.rs` - 1 instance
- `src/experiments/SetLoops.rs` - 2 instances
- `src/experiments/ForFor.rs` - 3 instances
- `src/experiments/ForLoops.rs` - 2 instances
- `src/experiments/ArrayVecSet.rs` - 2 instances
- `src/Chap05/SetStEph.partialsave.rs` - 1 instance

### ⚠️ Pattern 2: Named Ghost Iterator For Loops (PROBLEMATIC - 11 instances)
**Syntax**: `for x in iter: collection.iter()`  
**Verification**: ❌ Requires `ForLoopGhostIterator` trait implementation  
**Status**: Works for vstd types (Vec, HashSet), fails for custom wrappers

```rust
for item in iter: source.iter()
    invariant
        iter.elements == g_elements,
        dest@ == iter@,
{
    dest.push(*item);
}
```

**Files using this pattern**:
- `src/experiments/hash_set_with_view_plus_loops.rs` - 5 instances (Vec, HashSet - WORKS with vstd)
- `src/experiments/hash_set_iter.rs` - 1 instance
- `src/experiments/SetLoops.rs` - 2 instances (0..v.len() - WORKS as range)
- `src/Chap05/SetStEph.rs` - 2 instances (custom iterator - FAILS)
- `src/Chap05/SetStEph.partialsave.rs` - 1 instance (0..v.len() - WORKS as range)
- `src/experiments/simple_seq_iter.rs` - 1 instance (custom iterator - FAILS)

### 🔧 Pattern 3: Direct Rust Iterator For Loops (UNVERIFIED - 24 instances)
**Syntax**: `for x in collection.iter()` (no `iter:` ghost tracking)  
**Verification**: ⚠️ Executable only, no verification  
**Usage**: Quick prototype code, nested loops where verification not critical

```rust
for x in self.data.iter() {
    for subset in parts.data.iter() {
        if subset.data.contains(x) {
            count += 1;
        }
    }
}
```

**Files using this pattern**:
- `src/Chap05/SetStEph.rs` - 6 instances
- `src/vstdplus/hash_set_with_view_plus.rs` - 3 instances
- `src/Chap05/RelationStEph.rs` - 3 instances
- `src/Chap05/SetStEph.partialsave.rs` - 4 instances
- `src/experiments/hash_set_with_view_plus_loops.rs` - 1 instance
- Others - various

### 📦 Pattern 4: Owned Iterator For Loops (5 instances)
**Syntax**: `for x in v` (no `.iter()`)  
**Status**: Consumes the collection

```rust
for x in v {
    let _ = s.insert(x);
}
```

---

## Key Findings

### ✅ What Works Today

1. **Range iterators**: `for i in 0..len` - Built-in Verus support
2. **vstd type iterators with ghost tracking**: `for x in iter: vec.iter()` - Vec, HashSet from stdlib
3. **Unverified exec loops**: `for x in collection.iter()` - Executable but no proofs

### ❌ What Doesn't Work

1. **Custom wrapper iterators with ForLoopGhostIterator**: Our `SimpleSeqIter` and similar custom types
   - `exec_invariant` failures
   - Ghost state synchronization issues
   - Compiler lacks necessary axioms for custom `Clone` types

### 🔥 Impact Assessment

**Problem**: If we need to refactor all custom iterator for-loops to index-based loops:
- **Direct impact**: ~11 loops need conversion (Pattern 2 with custom iterators)
- **Potential impact**: ~24 unverified loops if we want full verification (Pattern 3)
- **No impact**: ~32 range-based loops already work (Pattern 1)

**Realistic refactoring scope**: 11-35 loops depending on verification goals

---

## Recommendations

### Option A: Use Index-Based For Loops (RECOMMENDED)
**Convert this**:
```rust
for elem in iter: s.iter()
    invariant sn@ =~= iter@,
{
    sn.push(elem);
}
```

**To this**:
```rust
for i in 0..s.elements.len()
    invariant sn@ =~= s.elements@.take(i as int),
{
    sn.push(s.elements[i]);
}
```

**Pros**: 
- Works today in Verus
- Well-tested pattern
- Simple invariants

**Cons**:
- Less idiomatic Rust
- Requires indexable collections
- Manual bounds checking in invariants

### Option B: Use While/Loop with Manual Iterator
**Works for verification**:
```rust
let mut it = s.iter();
loop
    invariant
        it.vec@ =~= s.elements@,
        sn@ =~= it@,
    decreases it.vec@.len() - it.pos,
{
    match it.next() {
        Some(elem) => sn.push(elem),
        None => break,
    }
}
```

**Pros**:
- Full control over iterator state
- Can verify complex invariants
- Works with custom iterators

**Cons**:
- More verbose than for-loops
- Manual decreases clause required
- More complex invariant management

### Option C: Wait for Verus Improvements
**Future work**: Better compiler support for custom `ForLoopGhostIterator` implementations

---

## Files Requiring Attention

### High Priority (Custom Iterator For-Loops)
1. `src/Chap05/SetStEph.rs` - Lines 258, 285 (custom iterator with ghost tracking)
2. `src/experiments/simple_seq_iter.rs` - Line 163 (our test case)

### Medium Priority (Unverified For-Loops)
1. `src/Chap05/SetStEph.rs` - Lines 311, 318, 320, 340, 341, 357, 377, 393, 400
2. `src/vstdplus/hash_set_with_view_plus.rs` - Lines 184, 195, 208
3. `src/Chap05/RelationStEph.rs` - Lines 123, 135, 177

### Low Priority (Already Work)
- All range-based loops in `src/experiments/*` files
- All vstd iterator loops in `hash_set_with_view_plus_loops.rs`

---

## Conclusion

**Good News**: Only ~11 custom iterator for-loops need immediate refactoring.  
**Pattern to adopt**: Use `for i in 0..len` with direct indexing for verified code.  
**Alternative**: Use `while`/`loop` with manual `next()` calls when iterator state tracking is critical.

The "261 for-loops" concern is overestimated - most loops use range iterators which already work perfectly in Verus.

