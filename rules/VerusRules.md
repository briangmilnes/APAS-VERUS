## Verus Rules for APAS-VERUS

### Verus File Structure

Every Verus-verified Rust file follows a standard structure:

1. **File header**: Copyright and module documentation comments
2. **Vstd prelude import**: `use vstd::prelude::*;`
3. **Verus block**: All verified code enclosed in `verus! { ... }`
4. **Closing brace**: `}` to end the verus block

#### Standard Template

```rust
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Module description.

use vstd::prelude::*;

verus! {

// All verified code goes here:
// - type definitions
// - traits
// - implementations
// - functions
// - proofs

} // verus!
```

#### Key Points

- The `vstd::prelude::*` import provides Verus standard library items needed for verification
- The `verus! { }` macro delimits code that will be verified by the Verus verifier
- All verified definitions (types, traits, impls, functions, proofs) must be inside the `verus!` block
- The closing brace should include a comment `// verus!` for clarity

#### Conditional Compilation

For files that need to compile with both standard Rust and Verus:

```rust
#[cfg(verus_keep_ghost)]
use builtin::*;
#[cfg(verus_keep_ghost)]
use builtin_macros::*;

#[cfg(not(verus_keep_ghost))]
macro_rules! verus {
    ($($item:tt)*) => { $($item)* };
}

verus! {
    // verified code
}
```

This pattern allows the code to compile with `cargo` while remaining verifiable with the `verus` tool.

