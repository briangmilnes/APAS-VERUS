# Verus Equality Axioms

## What Works: Built-in Types (like `u64`)

For concrete built-in types, Verus provides **automatic axiomatization** of equality. When you write `s[i] == elt` in executable code where both are `u64`, Verus knows:

1. **Executable equality** (`s[i] == elt` in Rust code) corresponds to **spec-level equality** (`s@[i as int] == elt` in ghost code)
2. Equality is reflexive: `x == x` is always true
3. Equality is symmetric: `x == y` implies `y == x`
4. Equality is transitive: `x == y && y == z` implies `x == z`

### Example: `u64` member function (works)

```rust
pub fn usize_array_member(s: &[usize], elt: usize) -> (result: bool)
    ensures result == seq_usize_member(s@, elt)
{
    let mut i: usize = 0;
    while i < s.len()
        invariant
            i <= s@.len(),
            forall|j: int| 0 <= j < i ==> s@[j] != elt,
        decreases s@.len() - i,
    {
        if s[i] == elt {  // ← Executable equality check
            // Verus automatically knows: s@[i as int] == elt (spec equality)
            return true;
        }
        // Verus automatically knows: s@[i as int] != elt
        i += 1;
    }
    false
}
```

**Why this works:** Verus has built-in axioms that connect:
- Rust's `PartialEq` implementation for `usize`
- SMT solver's equality theory for integers
- The spec-level view of sequences

## What Doesn't Work: Generic Types (like `T: Eq`)

For generic types, even with `T: Eq`, Verus **does not automatically axiomatize** the connection between executable and spec equality.

### Example: Generic `T` member function (fails)

```rust
pub open spec fn seq_t_mem<T: Eq>(s: Seq<T>, elt: T) -> bool {
    exists|i: int| 0 <= i < s.len() && s[i] == elt
}

pub fn t_array_mem<T: Eq>(s: &[T], elt: T) -> (result: bool)
    ensures result == seq_t_mem(s@, elt)
{
    let mut i: usize = 0;
    while i < s.len()
        invariant
            i <= s@.len(),
            forall|j: int| 0 <= j < i ==> s@[j] != elt,
        decreases s@.len() - i,
    {
        if s[i] == elt {  // ← Executable equality check
            assert(s@[i as int] == elt);  // ✗ FAILS! Cannot prove!
            return true;
        }
        assert(s@[i as int] != elt);  // ✗ FAILS! Cannot prove!
        i += 1;
    }
    false
}
```

**Error:**
```
error: assertion failed
   --> src/experiments/seq_basics.rs:155:24
    |
155 |                 assert(s@[i as int] == elt);
    |                        ^^^^^^^^^^^^^^^^^^^ assertion failed
```

**Why this fails:** 
- The trait bound `T: Eq` tells Rust that `T` has an `==` operator
- But it does NOT tell Verus's SMT solver:
  - That executable `s[i] == elt` implies spec `s@[i as int] == elt`
  - That the equality is reflexive, symmetric, or transitive
  - How to reason about equality in quantifiers

## What's Missing for Generic Types

For `u64`, Verus internally has something like this (conceptual, not actual syntax):

```rust
// BUILT-IN AXIOM for integer types:
axiom forall s: Slice<u64>, i: usize, elt: u64 {
    // Executable equality implies spec equality
    (s[i] == elt) <==> (s@[i as int] == elt)
}

axiom forall x: u64, y: u64 {
    // Reflexive
    x == x
    // Symmetric
    (x == y) <==> (y == x)
    // Transitive
    (x == y && y == z) ==> (x == z)
}
```

For generic `T: Eq`, we would need to add these axioms explicitly:

```rust
// WHAT WE WOULD NEED for generic T (not automatically provided by Verus):
axiom forall<T: Eq> s: Slice<T>, i: usize, elt: T {
    // Connect executable to spec
    (s[i] == elt) <==> (s@[i as int] == elt)
}

axiom forall<T: Eq> x: T, y: T, z: T {
    // Reflexive
    x == x
    // Symmetric  
    (x == y) <==> (y == x)
    // Transitive
    (x == y && y == z) ==> (x == z)
}
```

### Why Doesn't Verus Provide These Automatically?

**Soundness.** Verus intentionally does NOT provide default specifications for `Eq` (or `PartialEq`, `Clone`, etc.) to prevent unsound proofs.

Here's the problem:
1. In Rust, anyone can implement `Eq` for their type
2. The implementation might be **buggy** (not actually an equivalence relation)
3. If Verus automatically assumed all `Eq` impls are correct, it could **prove false things**

Example of a bad (but legal) `Eq` implementation:

```rust
struct BadType { x: i32 }

impl PartialEq for BadType {
    fn eq(&self, other: &Self) -> bool {
        use std::time::SystemTime;
        // Returns different values at different times!
        SystemTime::now().elapsed().unwrap().as_secs() % 2 == 0
    }
}

impl Eq for BadType {}  // Claims to be an equivalence relation, but isn't!
```

If Verus automatically assumed this `Eq` impl was reflexive/symmetric/transitive, it could prove:
- `x == x` (reflexive) - but the actual impl might return `false`!
- `x == y ==> y == x` (symmetric) - but the actual impl might violate this!

**The fundamental issue:** Rust's trait system doesn't enforce correctness, so Verus can't trust trait implementations.

### Can You Manually Add These Axioms?

**Yes, but it's hard.** You can manually axiomatize equality for specific types using:
- `#[verifier::external_type_specification]` - to specify behavior of types
- Proof functions that assert axioms for your specific type
- `assume` statements (which defeat the purpose of verification)

**The catch:** You're now responsible for ensuring your axioms are sound. If you axiomatize a buggy `Eq` impl as correct, Verus will prove false things.

**Why it's not worth it:**
1. Complex and error-prone
2. You have to do it for every generic type you want to verify
3. You lose the soundness guarantee - you're trusting your own axioms
4. Much easier to just write separate functions for `int`, `usize`, `i64`, etc.

## Current Workarounds

### 1. Stick to Concrete Types
The simplest solution: write separate functions for each concrete type you need:
- `int_array_member`
- `usize_array_member`
- `i64_array_member`
- etc.

### 2. Trait Specification (Advanced)
Verus supports `#[verifier::external_type_specification]` and related attributes to axiomatize trait behavior, but this is complex and error-prone.

### 3. Assume the Connection
For prototyping, you can use `assume` to tell Verus to trust the connection:

```rust
pub fn t_array_mem<T: Eq>(s: &[T], elt: T) -> (result: bool)
    ensures result == seq_t_mem(s@, elt)
{
    let mut i: usize = 0;
    while i < s.len()
        invariant
            i <= s@.len(),
            forall|j: int| 0 <= j < i ==> s@[j] != elt,
        decreases s@.len() - i,
    {
        if s[i] == elt {
            assume(s@[i as int] == elt);  // Trust me, they're equal!
            return true;
        }
        assume(s@[i as int] != elt);  // Trust me, they're not equal!
        i += 1;
    }
    false
}
```

**Note:** Using `assume` defeats the purpose of verification - you're not proving correctness, just asserting it.

## Bottom Line

- **Built-in types (int, usize, i64, etc.):** Equality "just works" in Verus
- **Generic types (T: Eq):** You must explicitly axiomatize equality, which is currently difficult in Verus
- **Recommendation:** Stick to concrete types for now, or accept that generic equality proofs require advanced Verus features

## References

- [Verus Guide: Equality](https://verus-lang.github.io/verus/guide/equality.html)
- [Verus Guide: Trait Specifications](https://verus-lang.github.io/verus/guide/spec-trait.html)

