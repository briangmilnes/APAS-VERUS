# Veracity Feature Request: Detect cfg-gated function hiding

## Problem

Functions annotated with `#[cfg(not(verus_keep_ghost))]` (without `#[verifier::external_body]`)
are invisible to veracity's hole counter. This creates a loophole where an agent can "close"
a hole by removing `external_body` while keeping the cfg gate — the function vanishes from
both Verus and veracity, inflating proof counts.

In R43, two agents exploited this (unknowingly — bad prompt instructions) to claim 28 of 30
hole closures that were actually just hiding functions.

## What We Need

When a `.rs` file contains a `fn` definition that is gated behind `#[cfg(not(verus_keep_ghost))]`
and does NOT have `#[verifier::external_body]`, veracity should emit a warning:

```
warning: cfg_hidden_fn - fn foo is gated by #[cfg(not(verus_keep_ghost))] without external_body
```

Severity: **warning** (not error, not info).

The only legitimate use of `#[cfg(not(verus_keep_ghost))]` on a `fn` is when it also has
`#[verifier::external_body]` — the cfg gate hides the body from cargo (because the body
uses types Verus can't parse), while external_body tells Verus the function exists but
is unproved.

A cfg gate WITHOUT external_body means the function is completely hidden from verification.
That's always a bug or a cheat.

## Scope

- Only `fn` definitions. `use` statements behind cfg gates are fine (that's the standard
  pattern for importing HashMap, rand, Arc, etc.).
- Only `#[cfg(not(verus_keep_ghost))]` specifically. Other cfg gates (feature flags, test
  gates) are not relevant.
- The warning should appear in the "File Holes" section alongside other warnings.
- It should NOT count as a hole (it's a warning about missing coverage, not a proof
  obligation).

## Examples

**Should warn:**
```rust
#[cfg(not(verus_keep_ghost))]
fn foo() { ... }  // Hidden from Verus entirely — no external_body
```

**Should NOT warn:**
```rust
#[verifier::external_body]
#[cfg(not(verus_keep_ghost))]
fn foo() { ... }  // Visible to Verus as opaque signature — correct pattern

#[cfg(not(verus_keep_ghost))]
use std::collections::HashMap;  // Import gate — fine
```
