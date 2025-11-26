# Verus Style for Verification

This document captures patterns and lessons learned for writing verifiable Verus code, focusing on trait design and specification style.

## 1. Trait Specification via `View` Bound

**Problem**: You cannot define spec functions in a trait and reference them in `ensures` clauses of the same trait - this creates a cyclic self-reference error.

```rust
// THIS DOESN'T WORK - cyclic reference
pub trait MyTrait<T> {
    spec fn spec_data(&self) -> Set<T>;  // defined in trait
    
    fn empty() -> (result: Self)
        ensures result.spec_data() == Set::empty();  // references trait's spec fn = CYCLE
}
```

**Solution**: Use the `View` trait bound on the trait itself. The `View` implementation lives on the concrete struct, breaking the cycle.

```rust
// THIS WORKS
pub trait MyTrait<T: View>: View<V = Set<T::V>> + Sized {
    fn empty() -> (result: Self)
        ensures result@ == Set::empty();  // uses View's @ operator
    
    fn size(&self) -> (n: usize)
        ensures n == self@.len();
}

// View is implemented on the concrete struct
impl<T: View> View for MyStruct<T> {
    type V = Set<T::V>;
    open spec fn view(&self) -> Set<T::V> { self.elements@ }
}
```

### Pattern Summary

1. **Trait declares**: `trait MyTrait<T>: View<V = SomeSpecType> + Sized`
2. **Struct implements**: `impl View for MyStruct { type V = SomeSpecType; ... }`
3. **Ensures clauses use**: `self@`, `result@`, `old(self)@` etc.

### Examples from APAS-VERUS

**SetStEph**:
```rust
pub trait SetStEphTrait<T: StT + Hash>: View<V = Set<<T as View>::V>> + Sized {
    fn empty() -> (empty: Self)
        requires valid_key_type::<T>()
        ensures empty@ == Set::<<T as View>::V>::empty();
    
    fn mem(&self, x: &T) -> (contains: B)
        requires valid_key_type::<T>()
        ensures contains == self@.contains(x@);
}
```

**RelationStEph**:
```rust
pub trait RelationStEphTrait<X: StT + Hash, Y: StT + Hash>: 
    View<V = Set<(<X as View>::V, <Y as View>::V)>> + Sized {
    
    fn empty() -> (empty: Self)
        requires valid_key_type_Pair::<X, Y>()
        ensures empty@ == Set::<(<X as View>::V, <Y as View>::V)>::empty();
}
```

## 2. Complex Views for Multi-Field Structs

When a struct has multiple fields that need to be reasoned about separately, use a tuple view:

```rust
pub struct DirGraphStEph<V: StT + Hash> {
    V: SetStEph<V>,    // vertices
    A: SetStEph<Edge<V>>,  // arcs
}

impl<V: StT + Hash> View for DirGraphStEph<V> {
    type V = (Set<V::V>, Set<(V::V, V::V)>);  // (vertices, arcs)
    
    open spec fn view(&self) -> Self::V {
        (self.V@, self.A@)
    }
}

// In trait ensures clauses:
pub trait DirGraphStEphTrait<V: StT + Hash>: 
    View<V = (Set<V::V>, Set<(V::V, V::V)>)> + Sized {
    
    fn empty() -> (g: Self)
        ensures g@.0 =~= Set::empty(),  // vertices empty
                g@.1 =~= Set::empty();  // arcs empty
    
    fn sizeV(&self) -> (n: N)
        ensures n == self@.0.len();  // vertex count
    
    fn sizeA(&self) -> (n: N)
        ensures n == self@.1.len();  // arc count
}
```

### When to Use Named Spec Functions

For complex derived properties that aren't directly in the view, define spec functions in the **impl block** (not the trait):

```rust
impl<V: StT + Hash> DirGraphStEph<V> {
    // Derived spec functions in impl block
    pub open spec fn spec_neighbor(&self, u: V::V, v: V::V) -> bool {
        self@.1.contains((u, v))  // uses view
    }
    
    pub open spec fn spec_nplus(&self, v: V::V) -> Set<V::V> {
        Set::new(|w: V::V| self@.1.contains((v, w)))
    }
}
```

Then reference them in the impl's ensures (not the trait's):

```rust
impl<V: StT + Hash> DirGraphStEphTrait<V> for DirGraphStEph<V> {
    fn Neighbor(&self, u: &V, v: &V) -> (b: B)
        ensures b == self.spec_neighbor(u@, v@)  // OK: concrete type
    { ... }
}
```

## 3. Why This Pattern Works

The key insight is **where the spec function is defined**:

| Location | Can reference in trait ensures? | Why |
|----------|--------------------------------|-----|
| Trait (abstract) | ❌ No | Creates cycle: trait → spec fn → trait |
| Impl block (concrete) | ✅ Yes | No cycle: trait → impl's spec fn (separate) |
| View trait | ✅ Yes | View is a separate trait, already resolved |

The `View` pattern works because:
1. `View` is defined in vstd, not your trait
2. Your struct implements `View` separately
3. The trait just requires `View` as a bound
4. `self@` in ensures resolves to the struct's `View::view()` method

## 4. Spec Function Visibility

Use `open spec fn` for spec functions that callers need to reason about:

```rust
// open = callers can see the definition
pub open spec fn spec_vertices(&self) -> Set<V::V> {
    self.V@
}

// closed = opaque to callers (only know signature)
pub closed spec fn internal_helper(&self) -> bool {
    // implementation hidden from callers
}
```

For View implementations, always use `open spec fn view`:

```rust
impl<V: StT + Hash> View for DirGraphStEph<V> {
    type V = (Set<V::V>, Set<(V::V, V::V)>);
    open spec fn view(&self) -> Self::V { (self.V@, self.A@) }  // MUST be open
}
```

## 5. Trait Method Ensures vs Impl Method Ensures

**Trait ensures**: Use `self@` (View) for specifications that should be part of the abstract interface:

```rust
pub trait SetStEphTrait<T>: View<V = Set<T::V>> + Sized {
    fn mem(&self, x: &T) -> (contains: B)
        ensures contains == self@.contains(x@);  // abstract spec
}
```

**Impl ensures**: Can add additional guarantees using concrete spec functions:

```rust
impl<V> DirGraphStEphTrait<V> for DirGraphStEph<V> {
    fn NPlus(&self, v: &V) -> (result: SetStEph<V>)
        ensures result@ == self.spec_nplus(v@)  // concrete spec fn
    { ... }
}
```

Note: Impl ensures must be **implied by** trait ensures (if any). You can strengthen but not contradict.

## 6. Summary: The Verified Trait Pattern

```rust
verus! {

// 1. Define the trait with View bound
pub trait MyTrait<T: StT + Hash>: View<V = MyViewType> + Sized {
    fn operation(&self) -> (result: ResultType)
        requires preconditions()
        ensures result_spec_using_view(self@, result);
}

// 2. Define the struct
#[verifier::reject_recursive_types(T)]
pub struct MyStruct<T: StT + Hash> {
    field: SomeVerifiedType<T>,
}

// 3. Implement View for the struct
impl<T: StT + Hash> View for MyStruct<T> {
    type V = MyViewType;
    open spec fn view(&self) -> Self::V { self.field@ }
}

// 4. Implement the trait (can add derived spec functions here)
impl<T: StT + Hash> MyStruct<T> {
    pub open spec fn derived_spec(&self) -> DerivedType {
        // compute from self@
    }
}

impl<T: StT + Hash> MyTrait<T> for MyStruct<T> {
    fn operation(&self) -> (result: ResultType)
        // Can add: ensures result@ == self.derived_spec()
    {
        // implementation
    }
}

} // verus!
```

This pattern:
- ✅ Avoids cyclic references
- ✅ Maximizes specification in the trait (via View)
- ✅ Allows derived specs in impl
- ✅ Works with Verus verification

