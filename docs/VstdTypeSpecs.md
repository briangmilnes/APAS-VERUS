# vstd Type Specifications: Analysis and Issues

This document analyzes how vstd specifies Rust standard library types and identifies fundamental inconsistencies that make proving textbook algorithms difficult.

## Overview

vstd provides two approaches for adding Verus specifications to Rust types:

| Approach | Mechanism | Creates New Type? |
|----------|-----------|-------------------|
| **SPECIFY** | `external_type_specification` | No - specs std type directly |
| **WRAP** | New struct containing std type | Yes - wrapper type |

## Complete Table of SPECIFY'd Types

### Collections

| # | std Type | Has View? | View Type | Subtypes Require View? |
|---|----------|-----------|-----------|------------------------|
| 1 | `Vec<T>` | ✓ | `Seq<T>` | **No** |
| 2 | `VecDeque<T>` | ✓ | `Seq<T>` | **No** |
| 3 | `HashSet<K>` | ✓ | `Set<K>` | **No** |
| 4 | `HashMap<K,V>` | ✓ | `Map<K,V>` | **No** |
| 5 | `[T]` (slice) | ✓ | `Seq<T>` | **No** |
| 6 | `String` | ✓ | `Seq<char>` | N/A |

### Iterators

| # | std Type | Has View? | View Type | Subtypes Require View? |
|---|----------|-----------|-----------|------------------------|
| 7 | `hash_set::Iter<K>` | ✓¹ | `(int, Seq<K>)` | **No** |
| 8 | `hash_map::Iter<K,V>` | ✓¹ | `(int, Seq<(K,V)>)` | **No** |
| 9 | `hash_map::Keys<K,V>` | ✓¹ | `(int, Seq<K>)` | **No** |
| 10 | `hash_map::Values<K,V>` | ✓¹ | `(int, Seq<V>)` | **No** |
| 11 | `slice::Iter<T>` | ✓ | `(int, Seq<T>)` | **No** |
| 12 | `vecdeque::Iter<T>` | ✓ | `(int, Seq<T>)` | **No** |
| 13 | `Range<T>` | ✓² | varies | **No** |

¹ Uses `SetIterAdditionalSpecFns` trait, not standard `View`  
² `RangeInclusive` only

### Smart Pointers

| # | std Type | Has View? | View Type | Subtypes Require View? |
|---|----------|-----------|-----------|------------------------|
| 14 | `Box<T>` | ✓ | `T::V` | **Yes** |
| 15 | `Rc<T>` | ✓ | `T::V` | **Yes** |
| 16 | `Arc<T>` | ✓ | `T::V` | **Yes** |
| 17 | `Cow<'a, T>` | ✓ | `T::V` | **Yes** |

### Other Types

| # | std Type | Has View? | View Type | Subtypes Require View? |
|---|----------|-----------|-----------|------------------------|
| 18 | `Option<T>` | ✓ | `Option<T>` | **No** |
| 19 | `Result<T,E>` | ✗ | — | — |
| 20 | `DefaultHasher` | ✓ | `Seq<Seq<u8>>` | N/A |
| 21 | `Ordering` | ✗ | — | N/A |
| 22 | `Duration` | ✗ | — | N/A |
| 23 | `PhantomData<T>` | ✗ | — | N/A |
| 24-29 | Various errors/internals | ✗ | — | N/A |

---

## The Fundamental Inconsistency

### Smart Pointers vs Collections

| Category | Subtypes Require View? | View Type |
|----------|------------------------|-----------|
| Smart Pointers (`Box`, `Rc`, `Arc`, `Cow`) | **Yes** | `T::V` (mapped) |
| Collections (`Vec`, `HashSet`, `HashMap`) | **No** | `T` (raw) |

**Smart pointers got it right.** `Box<T>` requires `T: View` and gives view type `T::V`.

**Collections got it wrong.** `Vec<T>` does NOT require `T: View` and gives view type `Seq<T>` (raw).

### What This Means in Practice

```rust
struct MyData {
    value: u64,
    cache: u64,  // implementation detail
}

impl View for MyData {
    type V = u64;  // only value matters logically
    fn view(&self) -> u64 { self.value }
}

// With Box (correct):
let b: Box<MyData> = Box::new(my_data);
// b@ : u64  -- the logical value!

// With Vec (incorrect):
let v: Vec<MyData> = vec![my_data];
// v@ : Seq<MyData>  -- includes cache field!
// To get Seq<u64>, must use DeepView + deep_view()
```

---

## View vs DeepView

vstd provides TWO view traits:

| Trait | Method | Purpose |
|-------|--------|---------|
| `View` | `@` / `view()` | Shallow - container view, elements unchanged |
| `DeepView` | `deep_view()` | Deep - recursively maps element views |

### Primitive Views (Identity)

All primitives have identity View AND DeepView:

| Type | `View::V` | `DeepView::V` |
|------|-----------|---------------|
| `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | Self | Self |
| `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | Self | Self |
| `bool` | `bool` | `bool` |
| `char` | `char` | `char` |
| `()` | `()` | `()` |

**Note:** There is NO implicit coercion from `u64` to `int`. You must use `x as int`.

### Collection DeepView Implementations

| # | Type | `View::V` | `DeepView::V` | Requires |
|---|------|-----------|---------------|----------|
| 1 | `Vec<T>` | `Seq<T>` | `Seq<T::V>` | `T: DeepView` |
| 2 | `VecDeque<T>` | `Seq<T>` | `Seq<T::V>` | `T: DeepView` |
| 3 | `HashSet<K>` | `Set<K>` | `Set<K::V>` | `K: DeepView` |
| 4 | `HashMap<K,V>` | `Map<K,V>` | `Map<K::V,V::V>` | `K,V: DeepView` |
| 5 | `[T]` (slice) | `Seq<T>` | `Seq<T::V>` | `T: DeepView` |
| 6 | `[T; N]` (array) | `Seq<T>` | `Seq<T::V>` | `T: DeepView` |
| 7 | `Option<T>` | `Option<T>` | `Option<T::V>` | `T: DeepView` |

### Smart Pointer DeepView Implementations

| # | Type | `View::V` | `DeepView::V` | Requires |
|---|------|-----------|---------------|----------|
| 8 | `Box<T>` | `T::V` | `T::V` | `T: View` / `T: DeepView` |
| 9 | `Rc<T>` | `T::V` | `T::V` | `T: View` / `T: DeepView` |
| 10 | `Arc<T>` | `T::V` | `T::V` | `T: View` / `T: DeepView` |
| 11 | `Cow<'a, T>` | `T::V` | `T::V` | `T: View + Clone` |
| 12 | `Cow<'a, [T]>` | `Seq<T>` | `Seq<T::V>` | `T: DeepView + Clone` |
| 13 | `&T` | `T::V` | `T::V` | `T: View` / `T: DeepView` |

### Iterator DeepView Implementations

| # | Type | `View::V` | `DeepView::V` | Requires |
|---|------|-----------|---------------|----------|
| 14 | `slice::Iter<T>` | `(int, Seq<T>)` | `(int, Seq<T::V>)` | `T: DeepView` |

### String Types

| # | Type | `View::V` | `DeepView::V` |
|---|------|-----------|---------------|
| 15 | `str` | `Seq<char>` | `Seq<char>` |
| 16 | `String` | `Seq<char>` | `Seq<char>` |
| 17 | `Chars<'a>` | `(int, Seq<char>)` | same |

### Tuple DeepView

Tuples have DeepView if all elements have DeepView:
```rust
impl<A: DeepView, B: DeepView, ...> DeepView for (A, B, ...) {
    type V = (A::V, B::V, ...);
}
```

### The Problem with DeepView

To get mapped views, you must:
1. Call `deep_view()` instead of `@`
2. Element type must implement `DeepView`
3. Remember to use it consistently

**Note:** `DeepView` is NOT documented in the [Verus Guide](https://verus-lang.github.io/verus/guide/). 
It exists in vstd but is an undocumented feature.

```rust
let v: Vec<u64> = vec![1, 2, 3];

// View (shallow):
let seq: Seq<u64> = v@;           // Elements are u64

// DeepView (mapped):
let seq: Seq<u64> = v.deep_view(); // Still u64 (identity for primitives)

// To get Seq<int>, must map explicitly:
let seq_int: Seq<int> = v@.map(|i: int, x: u64| x as int);
```

**DeepView helps for nested structures but doesn't solve the primitive→int conversion.**

---

## The Equality Problem

Beyond View, vstd also fails to specify equality properly.

### What Rust Guarantees About `Eq`

| Property | Rust Guarantees? |
|----------|------------------|
| Reflexive: `x == x` | No |
| Symmetric: `x == y <==> y == x` | No |
| Transitive: `x == y && y == z ==> x == z` | No |

**Rust's `Eq` trait is a promise, not a proof.** Anyone can write a bad impl.

### What vstd Specifies

| Property | vstd Specifies? |
|----------|-----------------|
| Reflexive | **No** |
| Symmetric | **No** |
| Transitive | **No** |
| `x == y ==> x@ == y@` | **No** |
| `x@ == y@ ==> x == y` | Partial (precondition only) |

### The Consequence

To prove anything about set membership or map keys, you need:
```rust
forall|k1: K, k2: K| k1@ == k2@ <==> k1 == k2
```

vstd doesn't provide this. It only requires ONE direction as a precondition.

---

## vstdplus/APAS-VERUS Solutions

### 1. `HashSetWithViewPlus`

Wrapper around `HashSet` that:
- Requires `obeys_feq_full::<K>()` for coherent equality
- Provides `iter()` method (missing from `HashSetWithView`)
- Maps view to `Set<K::V>`

### 2. `feq` (Full Equality)

Specifies what vstd refuses to:

```rust
pub open spec fn obeys_feq_full<T: Eq + View + Clone + Sized>() -> bool {
    &&& feq_reflexive::<T>()      // x == x
    &&& feq_symmetric::<T>()      // x == y <==> y == x
    &&& feq_transitive::<T>()     // x == y && y == z ==> x == z
    &&& obeys_feq_view::<T>()     // x == y ==> x@ == y@
    &&& obeys_feq_view_injective::<T>()  // x@ == y@ ==> x == y
    &&& obeys_feq_clone::<T>()    // clone coherence
    &&& obeys_feq_eq::<T>()       // eq_spec <==> ==
}
```

### 3. Closed Spec Iterator Pattern

For iterator wrappers (`SetStEphIter`):
- Private `inner` field for encapsulation
- `closed spec fn view()` accesses inner but hides HOW
- External code uses `self@` without seeing implementation

---

## Summary: The Textbook Problem

To prove a textbook algorithm about sets/maps in Verus:

| Requirement | vstd Provides? | vstdplus Provides? |
|-------------|----------------|-------------------|
| View on element type | Optional | Required |
| Mapped view (`Set<K::V>`) | Only via WRAP | ✓ |
| Equality is equivalence relation | **No** | ✓ (`feq`) |
| `==` coherent with `@` | Partial | ✓ |
| Collection iteration | SPEC only | WRAP with closed spec |

**The textbook says "Let S be a set of elements..."**  
**Verus asks: "What's the View? Is Eq reflexive? Are equal elements equal?"**

These should be axioms of the system, not user-provided preconditions.

---

## References

- `/home/milnes/projects/APAS-VERUS/src/vstdplus/feq.rs` - Full equality specification
- `/home/milnes/projects/APAS-VERUS/src/vstdplus/hash_set_with_view_plus.rs` - HashSet wrapper with iter()
- `/home/milnes/projects/APAS-VERUS/src/Chap05/SetStEph.rs` - Closed spec iterator pattern
- `/home/milnes/projects/verus/source/vstd/std_specs/hash.rs` - vstd HashSet specs
- `/home/milnes/projects/verus/source/vstd/hash_set.rs` - vstd HashSetWithView wrapper

