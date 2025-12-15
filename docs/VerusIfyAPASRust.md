# Verus-ifying APAS Rust Code

This document captures lessons learned from converting APAS (Algorithms Parallel and Sequential) Rust code to Verus-verified code.

## 1. Code Organization: The Correct Pattern

**IMPORTANT**: The `verus!` macro handles erasure - it keeps exec code when `verus_keep_ghost` is NOT set. You do NOT need duplicate struct definitions.

### Correct File Structure

```rust
pub mod MyModule {
    use vstd::prelude::*;
    use std::fmt::{Debug, Display, Formatter};

    verus! {
        // Verus-only imports (guarded)
        #[cfg(verus_keep_ghost)]
        use vstd::std_specs::hash::obeys_key_model;
        
        // Import feq - full module in Verus, just the fn in cargo
        #[cfg(verus_keep_ghost)]
        use crate::vstdplus::feq::feq::*;
        #[cfg(not(verus_keep_ghost))]
        use crate::vstdplus::feq::feq::feq;
        
        // ClonePlus works unconditionally (has stubs)
        use crate::vstdplus::clone_plus::clone_plus::ClonePlus;

        // ONE struct definition - verus! macro handles erasure
        #[verifier::reject_recursive_types(T)]
        pub struct MyStruct<T> {
            pub data: Vec<T>,
        }

        impl<T: View> View for MyStruct<T> { ... }

        impl<T: View> MyStruct<T> {
            pub fn new() -> Self { ... }
            // ... verified methods
        }
    } // verus!

    // Trait impls outside verus! - NO cfg guards needed
    impl<T: Clone> Clone for MyStruct<T> {
        fn clone(&self) -> Self { MyStruct { data: self.data.clone() } }
    }

    impl<T: PartialEq> PartialEq for MyStruct<T> {
        fn eq(&self, other: &Self) -> bool { self.data == other.data }
    }

    impl<T: Eq> Eq for MyStruct<T> {}

    impl<T: Debug> Debug for MyStruct<T> { ... }
    impl<T: Display> Display for MyStruct<T> { ... }

    // Macros outside verus!
    #[macro_export]
    macro_rules! MyLit { ... }

    // Helper methods using std types Verus doesn't support
    impl<T> MyStruct<T> {
        pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.data.iter_mut() }
    }
}
```

### What Goes Where

**Inside `verus!` blocks:**
- Struct definitions (ONE copy, with `#[verifier::reject_recursive_types(...)]`)
- Trait definitions with type bounds
- Spec functions (`spec fn`)
- Proof functions (`proof fn`)
- Exec functions that need verification
- `View` implementations
- Ghost iterators

**Outside `verus!` blocks (NO cfg guards):**
- `Clone`, `PartialEq`, `Eq` implementations
- `Display`, `Debug` implementations
- `From` implementations
- `Iterator` implementations (std iterator)
- `IntoIterator` implementations
- Macros
- Helper methods using std types Verus doesn't support (like `IterMut`)

### Keep the original traits (with comments) intact

When verusifying APAS source, **keep the original trait definitions exactly as written in
APAS-AI, including all their comments and complexity notes**. Do not split or rename
traits unless the APAS source already does so. Treat traits as the stable API surface;
only add the verification-specific machinery around them (e.g., `verus!` blocks, views,
ghost functions).

Preferred approach: keep the trait surface and comments, then add a `View` plus spec
accessors on the concrete type so you can write `requires`/`ensures` that reference the
ghost model without altering the trait signature.

## 2. Trait Bounds and `StT`

The base trait for single-threaded friendly types:

```rust
pub trait StT: Eq + Clone + Display + Debug + Sized + vstd::prelude::View {}
impl<T> StT for T where T: Eq + Clone + Display + Debug + Sized + vstd::prelude::View {}
```

- `View` is required for Verus ghost reasoning
- Hash collections need `StT + Hash`

## 3.5 Trait specs via views (pattern from `SetStEph`)

To place `requires`/`ensures` on trait methods, first give the struct a `View` and spec
accessors, then express the trait method contracts in terms of that view. Example
(`SetStEph` pattern):

1. Implement `View` for the concrete type, exposing its ghost model:
   ```rust
   impl<T: View> View for SetStEph<T> {
       type V = Set<T::V>;
       open spec fn view(&self) -> Set<T::V> { self.s@ }
   }
   ```
2. Add spec helpers on the concrete type if needed (e.g., `spec_contains`, `spec_len`).
3. In the trait, write `requires`/`ensures` using those spec accessors / `view()` so the contracts are abstract but grounded in the ghost model.
4. Implement the trait for the concrete type; the impl can refer to `self.view()` (or helper specs) in its own `requires`/`ensures`.

This keeps the trait API identical to APAS-AI while allowing full specifications. Use this
pattern when you need trait-level specs; otherwise leave the original trait
signatures/comments untouched.

## 3. Custom Traits for Verified Operations

### `ClonePlus`

Clone with a postcondition that Verus can reason about:

```rust
pub trait ClonePlus: Clone {
    fn clone_plus(&self) -> (res: Self)
        ensures res == self
}
```

### `feq` (Full Equality)

Bridges executable `==` to spec `@` (view) equality:

```rust
pub fn feq<T: Eq + View>(x: &T, y: &T) -> (eq: bool)
    ensures eq <==> x@ == y@
```

### Key Model Axioms

For hash collection correctness:

- `obeys_key_model::<T>()` - hash/eq consistency
- `obeys_feq_full::<T>()` - full equality properties (reflexive, symmetric, transitive, view-consistent)

## 4. Unsupported Std Library Items

Verus does not support:

- `Mutex` and its methods
- `Default` trait
- Many iterator methods
- Various std library functions

Use `#[cfg(verus_keep_ghost)]` guards for `vstd::std_specs::*` imports:

```rust
#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;
```

## 5. The View Pattern

Every verified type needs a `View` implementation:

```rust
impl<K: View, V: View> View for Pair<K, V> {
    type V = (K::V, V::V);
    
    open spec fn view(&self) -> (K::V, V::V) {
        (self.0@, self.1@)
    }
}
```

Custom types like `Pair`, `Edge`, `Triple` need View impls. Axioms may be needed for properties like injectivity:

```rust
pub broadcast proof fn axiom_Pair_view_injective<K: View, V: View>(p1: Pair<K, V>, p2: Pair<K, V>)
    requires #[trigger] p1@ == #[trigger] p2@,
    ensures p1 == p2,
{ admit(); }
```

## 6. Iterator Pattern

### Ghost Iterator Model

Verus uses ghost iterators with either:
- `(pos: int, elements: Seq<T>)` - position-based model
- `(visited: Set<T>, current: Option<T>, remaining: Set<T>)` - set-based model

### Implementation Pattern

```rust
verus! {
    // Ghost iterator definition
    pub struct MyIterGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: PhantomData<&'a ()>,
    }
    
    impl<'a, T> ForLoopGhostIteratorNew for MyIter<'a, T> { ... }
    impl<'a, T> ForLoopGhostIterator for MyIterGhostIterator<'a, T> { ... }
}

// Standard Iterator impl goes OUTSIDE verus! block
impl<'a, T> Iterator for MyIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}
```

Custom iterator wrappers hide `std::collections` iterators from Verus.

## 7. Conditional Compilation

Enable same code to verify under Verus and compile under cargo:

```rust
// Verus-only imports
#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;

// Stub for non-Verus builds
#[cfg(not(verus_keep_ghost))]
pub fn feq<T: Eq>(x: &T, y: &T) -> bool { *x == *y }
```

Add to `Cargo.toml` to suppress warnings:

```toml
[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(verus_keep_ghost)'] }
```

## 8. External Dependencies

Verus doesn't see Cargo dependencies directly when invoked as `verus --crate-type=lib src/lib.rs`.

Workarounds:
- Run `cargo build` first to resolve dependencies, then run verus
- Exclude modules using unsupported crates (e.g., `ordered_float`, `rayon`)
- Comment out problematic imports when not needed

## 9. Macro Placement

Macros must be defined **outside** `verus!` blocks for `use crate::MacroName;` to work:

```rust
// OUTSIDE verus! block
#[macro_export]
macro_rules! SetLit {
    () => { SetStEph::empty() };
    ( $( $x:expr ),* $(,)? ) => {{
        let mut s = SetStEph::empty();
        $( let _ = s.insert($x); )*
        s
    }};
}
```

## 10. Struct Attributes

### `reject_recursive_types`

For generic structs inside `verus!` blocks, Verus needs to know that the type parameter
won't create infinite recursion. Use `#[verifier::reject_recursive_types(T)]` for each
type parameter:

```rust
#[verifier::reject_recursive_types(V)]
#[derive(Copy, PartialEq, Eq, Hash, Debug)]
pub struct Edge<V: StT>(pub V, pub V);

// Multiple type parameters - one attribute per parameter
#[verifier::reject_recursive_types(V)]
#[verifier::reject_recursive_types(L)]
#[derive(Copy, PartialEq, Eq, Hash, Debug)]
pub struct LabEdge<V: StT, L: StT + Hash>(pub V, pub V, pub L);
```

This tells Verus: "I promise `V` won't be instantiated with a type that contains `Edge<V>` itself" (which would create an infinite type).

### Clone derive

Remove `Clone` from derive and implement manually outside the verus! block:

```rust
// Outside verus! block
impl<V: StT> Clone for Edge<V> {
    fn clone(&self) -> Self {
        Edge(self.0.clone(), self.1.clone())
    }
}
```

## 11. Partial Verification: Skipping Termination Proofs

When converting code incrementally, you can skip termination proofs for loops using:

```rust
#[verifier::exec_allows_no_decreases_clause]
fn my_function_with_loop(&self) -> Result {
    let mut iter = self.items.iter();
    loop
        invariant true,  // minimal invariant for now
    {
        match iter.next() {
            None => break,
            Some(item) => { /* ... */ }
        }
    }
    result
}
```

This allows the function to verify without proving termination - useful as an intermediate
step when verus-ifying code. The `invariant true` is a placeholder that can be
strengthened later.

## 12. Partial Verification: External Body

When a function body uses operations with preconditions you haven't yet proven (e.g.,
`obeys_key_model`, `obeys_feq_full`), you can temporarily skip verification of the entire
body:

```rust
#[verifier::external_body]
fn my_function(&self) -> Result {
    // Body is not verified - Verus trusts it
    self.set.insert(value)
}
```

This is a **temporary step** during incremental verus-ification. The function:
- Compiles and runs correctly
- Is not verified by Verus (body is trusted)
- Can have `requires`/`ensures` clauses that callers must satisfy

Use this when:
1. Converting plain Rust code to Verus incrementally
2. Functions call methods with preconditions you haven't proven yet
3. You want the code to pass verification while you work on other parts

Remove `#[verifier::external_body]` and add proper proofs once the supporting axioms and preconditions are in place.

## 13. Spec Functions for Data Structures

For complex data structures, define spec functions that describe the abstract/ghost model:

```rust
impl<V: StT + Hash> DirGraphStEph<V> {
    // Spec functions for ghost reasoning
    pub open spec fn spec_vertices(&self) -> Set<V::V> {
        self.V@
    }

    pub open spec fn spec_arcs(&self) -> Set<(V::V, V::V)> {
        self.A@
    }

    pub open spec fn spec_neighbor(&self, u: V::V, v: V::V) -> bool {
        self.spec_arcs().contains((u, v))
    }

    pub open spec fn spec_nplus(&self, v: V::V) -> Set<V::V> {
        Set::new(|w: V::V| self.spec_arcs().contains((v, w)))
    }

    pub open spec fn spec_nminus(&self, v: V::V) -> Set<V::V> {
        Set::new(|u: V::V| self.spec_arcs().contains((u, v)))
    }
}
```

Then use these in `ensures` clauses:

```rust
fn NPlus(&self, v: &V) -> (result: SetStEph<V>)
    requires valid_key_type_Edge::<V>()
    ensures result@ == self.spec_nplus(v@);
```

This pattern:
1. Defines the abstract semantics in spec functions
2. Links exec functions to spec functions via ensures
3. Allows callers to reason about results using the spec model

## 14. Broadcast Groups

Broadcast groups make axioms automatically available to the SMT solver. Use them at module level inside `verus!` blocks:

```rust
verus! {

// Combine multiple broadcast groups in a single statement (only one allowed per module)
broadcast use {
    vstd::std_specs::hash::group_hash_axioms,
    vstd::set_lib::group_set_lib_default,
    crate::vstdplus::feq::feq::group_feq_axioms,
    crate::Types::Types::group_Pair_axioms,
};

// ... rest of module
}
```

### Common Broadcast Groups

**From vstd:**
- `vstd::std_specs::hash::group_hash_axioms` - HashMap/HashSet axioms
- `vstd::set_lib::group_set_lib_default` - Set properties
- `vstd::seq_lib::group_seq_extra` - Sequence properties
- `vstd::map_lib::group_map_properties` - Map properties
- `vstd::std_specs::vec::group_vec_axioms` - Vec axioms

**From vstdplus:**
- `crate::vstdplus::feq::feq::group_feq_axioms` - Full equality axioms
- `crate::vstdplus::clone_view::group_clone_view_axioms` - Clone/view axioms
- `crate::vstdplus::set_axioms::group_set_axioms_plus` - Additional set axioms

**From Types:**
- `crate::Types::Types::group_Pair_axioms` - Pair view injectivity and feq axioms

## 14. Using `clone_plus` and `feq`

When verus-ifying code, replace standard operations with verified equivalents:

### Clone → clone_plus

```rust
// Before (no spec)
let x = value.clone();

// After (has ensures res == self)
let x = value.clone_plus();
```

Import: `use crate::vstdplus::clone_plus::clone_plus::ClonePlus;`

### `==` → `feq`

```rust
// Before (no spec linking exec == to spec @)
if x == *v { ... }

// After (ensures eq <==> x@ == y@)
if feq(&x, v) { ... }
```

Import: `use crate::vstdplus::feq::feq::feq;`

These imports need `#[cfg(verus_keep_ghost)]` guards for non-Verus builds if you only want them in verified code. However, if you provide non-Verus stubs (see Section 16), you can import them unconditionally.

## 16. Providing Non-Verus Stubs for vstdplus Traits

To make `ClonePlus` and `feq` work in both Verus verification and cargo tests, provide stub implementations:

```rust
// In vstdplus/clone_plus.rs
#[cfg(verus_keep_ghost)]
pub mod clone_plus {
    // ... full Verus implementation with ensures clause ...
}

#[cfg(not(verus_keep_ghost))]
pub mod clone_plus {
    /// ClonePlus trait for non-Verus builds - just delegates to clone()
    pub trait ClonePlus: Clone + Sized {
        fn clone_plus(&self) -> Self;
    }

    impl<T: Clone> ClonePlus for T {
        fn clone_plus(&self) -> Self {
            self.clone()
        }
    }
}
```

```rust
// In vstdplus/feq.rs
#[cfg(not(verus_keep_ghost))]
pub mod feq {
    /// Stub feq function for non-Verus builds - just uses ==
    pub fn feq<T: Eq>(x: &T, y: &T) -> bool {
        *x == *y
    }
}
```

With stubs, imports can be unconditional:

```rust
use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
use crate::vstdplus::feq::feq::feq;
```

## 17. Crate Feature Flag for Expression Attributes

Add to `src/lib.rs` to enable `#[verifier::...]` attributes on expressions:

```rust
#![feature(stmt_expr_attributes)]
```

This is needed because Verus uses attributes like `#[verifier::loop_isolation(false)]` on loop expressions.

## 18. Wrapping Verifier Attributes with cfg_attr

Verifier attributes must be wrapped for non-Verus builds:

```rust
// Won't compile with cargo (verifier module doesn't exist)
#[verifier::loop_isolation(false)]
loop { ... }

// Works with both Verus and cargo
#[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
loop { ... }
```

Common attributes that need wrapping:
- `#[verifier::loop_isolation(false)]`
- `#[verifier::external_body]`
- `#[verifier::exec_allows_no_decreases_clause]`

## 15. Test Files

Test files run with `cargo test`, NOT Verus verification. They are plain Rust.

### Key Differences from Verified Code

1. **No `verus!` blocks** - Tests are standard Rust
2. **No `requires`/`ensures`** - No verification contracts
3. **Standard assertions** - Use `assert!`, `assert_eq!`, `assert_ne!`
4. **Standard iterators** - Use `.iter()`, `.cloned()`, `.collect()` freely
5. **No ghost code** - `proof`, `spec`, `ghost` blocks are erased at runtime

### Imports

```rust
// Import from the crate's public API
use apas_verus::Chap06::DirGraphStEph::DirGraphStEph::*;
use apas_verus::Types::Types::*;
use apas_verus::{SetLit, EdgeLit, DirGraphStEphLit};
```

### Using `clone_plus` and `feq` in Tests

Both `ClonePlus` and `feq` have non-Verus stubs that work in tests:
- `clone_plus()` delegates to `clone()`
- `feq(&x, &y)` delegates to `*x == *y`

This means verified code using `clone_plus` and `feq` runs correctly in tests without modification.

### Cargo.toml Test Entries

Each test file needs an entry in `Cargo.toml`:

```toml
[[test]]
name = "TestDirGraphStEph"
path = "tests/Chap06/TestDirGraphStEph.rs"
```

### Running Tests

Tests require `RUSTC_BOOTSTRAP=1` to enable unstable features used by Verus:

```bash
RUSTC_BOOTSTRAP=1 cargo test
# or
RUSTC_BOOTSTRAP=1 cargo nextest run

# Run a specific test file
RUSTC_BOOTSTRAP=1 cargo test --test TestDirGraphStEph
```

### Example Test

```rust
#[test]
fn test_empty_graph() {
    let g: DirGraphStEph<i32> = DirGraphStEph::empty();
    assert_eq!(g.sizeV(), 0);
    assert_eq!(g.sizeA(), 0);
}

#[test]
fn test_graph_from_sets() {
    let g = DirGraphStEphLit![
        V: [1, 2, 3],
        A: [(1, 2), (2, 3)]
    ];
    assert_eq!(g.sizeV(), 3);
    assert_eq!(g.sizeA(), 2);
    assert!(g.Neighbor(&1, &2));
}
```

