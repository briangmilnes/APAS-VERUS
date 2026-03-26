# veracity-compare-par-mut fix: resolve type aliases before comparison (Round 4)

## Context

Phase 3b is done. The tool reports 46 errors, but several are false positives caused
by type alias mismatches. Do NOT re-run phases 1, 2, 3, or 3b. Fix the comparison
logic only.

## Problem

The project defines type aliases in `src/Types.rs`:
```rust
pub type N = usize;
pub type B = bool;
```

When comparing return types across variants, the tool sees `N` vs `usize` and
`B` vs `bool` as different types and reports errors. They are identical.

Similarly, `<T as View>::V` and `T::V` are the same associated type written
two ways.

## Fix 1: Type alias resolution

Before comparing return types, parameter types, or View types, normalize:
- `N` → `usize`
- `B` → `bool`

These are the only two aliases defined in `src/Types.rs` that cause this.
Hard-code them — don't try to build a general alias resolver.

## Fix 2: Associated type syntax normalization

Before comparing types, normalize:
- `< T as View > :: V` → `T :: V`
- `< V as View > :: V` → `V :: V`

Strip the `< ... as Trait >` wrapper when the result is just `:: AssocType`.

## Fix 3: Reclassify known Mt/St return type differences

These are real differences but expected patterns, not errors. Reclassify from
`error:` to `info:`:

**Owned vs borrowed returns** (Mt can't return references through RwLock):
- `Option<T>` vs `Option<&T>` — Mt returns owned, St returns borrowed
- `Vec<T>` vs `&Vec<T>` — same pattern
- `Arc<Vec<T>>` vs `&Vec<T>` — Mt wraps in Arc

**Iterator return types**: each variant has its own iterator type. Different
iterator struct names across variants is expected, not an error:
- `FooStEphIter` vs `FooMtEphIter` — expected

Classify as `info: fn foo: Mt returns owned T, St returns &T (RwLock pattern)`.

## What stays as error

- View type mismatches (e.g., `Seq<T>` vs `Set<T::V>`) — real divergence
- Return type differs and it's NOT an owned/borrowed or iterator pattern
- Supertrait mismatches (after alias normalization)

## Output format

Same emacs compilation-mode format. After fixes, the 46 errors should drop
significantly — the real errors are the View misalignments and genuine API drift.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All parsing must be token-aware or AST-aware.
