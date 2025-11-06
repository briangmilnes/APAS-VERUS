# APAS-VERUS Porting Guidelines

## Overview

APAS-VERUS is a port of APAS-AI (Algorithms Parallel and Sequential in Rust) to Verus for formal verification. The goal is to maintain the same API and structure while adding verification.

## Porting Strategy

### Copy and Minimally Modify

We follow a **copy and minimally modify** approach for:
- **`src/`** - Source code modules
- **`tests/`** - Test files
- **`benches/`** - Benchmark files

### Minimal Modifications

When porting from APAS-AI to APAS-VERUS, make only the **minimum necessary changes** to enable verification:

1. **Wrap code in `verus! { }`** blocks
2. **Add specifications** (`requires`, `ensures`, `invariant`)
3. **Add `pub open spec fn view()`** for data structure abstractions
4. **Use `#[verifier::external_body]`** when trusting implementations (e.g., hash collections)
5. **Add `vstd` imports** as needed for verification

### What to Preserve

**Keep from APAS-AI:**
- API signatures (trait methods, function names, parameters)
- Module structure and organization
- Algorithm implementations
- Test cases and assertions
- Benchmark structure
- Comments and documentation

### What to Change

**Modify only when necessary:**
- Add Verus verification syntax
- Change data structures to verified wrappers (e.g., `HashSetWithView`)
- Add proof obligations
- Adjust imports for `vstd` instead of `std` when needed

## Example Transformation

### APAS-AI Version
```rust
pub fn insertion_sort(a: &mut [u64]) {
    for i in 1..a.len() {
        let key = a[i];
        let mut j = i;
        while j > 0 && a[j - 1] > key {
            a[j] = a[j - 1];
            j -= 1;
        }
        a[j] = key;
    }
}
```

### APAS-VERUS Version
```rust
verus! {

pub fn insertion_sort(a: &mut [u64])
    ensures 
        a@.to_multiset() == old(a)@.to_multiset(),
        sorted_by(a@, |x: u64, y: u64| x <= y),
{
    for i in 1..a.len()
        invariant
            a@.to_multiset() == old(a)@.to_multiset(),
            sorted_by(a@.subrange(0, i as int), |x: u64, y: u64| x <= y),
    {
        let key = a[i];
        let mut j = i;
        while j > 0 && a[j - 1] > key
            invariant
                // ... loop invariants
        {
            a[j] = a[j - 1];
            j -= 1;
        }
        a[j] = key;
    }
}

} // verus!
```

## Divergence from APAS-AI

We **intentionally diverge** in these areas:

### 1. Hash Collection Wrappers (Chapter 5)

**APAS-AI approach:**
- Wraps `std::collections::HashSet` and `HashMap` directly
- Generic implementations with full trait API

**APAS-VERUS approach:**
- Uses `vstd::hash_set::HashSetWithView` and `HashMapWithView`
- Generic implementations with `#[verifier::external_body]` (trusted)
- Rationale: Verus cannot verify generic `obeys_key_model` at monomorphization time
- Trade-off: Trust the wrappers, verify everything built on top

### 2. Verification Annotations

**APAS-VERUS adds:**
- Specifications on all public functions
- Loop invariants for all loops
- `view()` functions for abstract reasoning
- Proof functions and lemmas as needed

### 3. File Structure

**Both APAS-AI and APAS-VERUS:**
- All module declarations in `src/lib.rs`
- No `mod.rs` files (see `VerusRules.md` for details)

## Testing Strategy

### APAS-AI Tests
- Standard Rust `#[test]` with `assert_eq!`, `assert!`
- Focus on correctness via test cases

### APAS-VERUS Tests
- Same test cases wrapped in `verus! { }` blocks
- Use Verus `assert()` for verified assertions
- Tests compile and run with `cargo test`
- Tests verify with `verus`

## Documentation

When porting:
1. **Preserve APAS copyright headers**
2. **Keep APAS complexity annotations** (e.g., "Work Θ(n), Span Θ(1)")
3. **Add Verus-specific notes** when diverging from APAS-AI
4. **Document trusted assumptions** when using `external_body` or `assume`

## Summary

- **Copy** source, tests, and benches from APAS-AI
- **Modify minimally** - only add what's necessary for verification
- **Preserve** the API, structure, and algorithmic approach
- **Diverge intentionally** only when Verus limitations require it
- **Document** all divergences and trusted components

