# PTT Iterator Style Guide

PTTs (Proof Time Tests) for iterators prove that a module's iterator can be
correctly used in verified loops. That's the only purpose — usability proof.
But "usability" means the full requires chain works: constructor ensures wf,
wf satisfies iter()'s requires, iter's ensures feed the loop invariants.

## The requires chain

This is the whole point. Every `iter()` requires `self.spec_<module>_wf()`.
The PTT proves the chain works end-to-end:

1. **Constructor** (`new()`, `singleton()`, etc.) `ensures` wf.
2. **iter()** `requires` wf — satisfied by (1).
3. **iter()** `ensures` iter_invariant, pos == 0, etc.
4. **Loop invariants** use iter_invariant and pos from (3).
5. **next()** preserves iter_invariant through the loop.

If the constructor has additional requires (type axioms like `obeys_feq_clone`,
`obeys_view_eq`), those must appear in the test function's `requires`.

**Read the module's constructor and iter() signatures.** The test function's
`requires` must include everything the constructor needs that isn't trivially
true. The constructor's `ensures` must include wf, which flows to iter().

## File naming

`rust_verify_test/tests/ChapNN/Prove<Module>.rs`

Example: `ProveAVLTreeSeqStEph.rs` for `src/Chap37/AVLTreeSeqStEph.rs`.

## Header

```rust
//! Proof tests for ChapNN <Module> iterators.
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter:   `loop { ... t.iter() ... }`
//!   - loop-borrow-into:   `loop { ... (&t).into_iter() ... }`
//!   - for-borrow-iter:    `for x in iter: t.iter()`
//!   - for-borrow-into:    `for x in iter: (&t).into_iter()`
```

List only the patterns the module supports (add loop-consume/for-consume if it
has a consuming IntoIterator).

## Boilerplate

```rust
#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;
```

## Test structure

Each pattern is one `test_verify_one_file!` block. Test name:
`<module_lowercase>_<pattern>` (e.g., `avltreeseqsteph_loop_borrow_iter`).

## Two categories

### Category A: No type axiom requires (simple types like u64)

Constructor has no requires beyond what's trivially true. Test function has
no `requires`. The wf chain works silently: `new()` ensures wf, `iter()`
requires wf — Verus proves it.

```rust
test_verify_one_file! {
    #[test] module_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::ChapNN::Module::Module::*;

        fn test_loop_borrow_iter() {
            let t = ModuleS::new(/* literal constructor args */);

            let mut it: ModuleIter<u64> = t.iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    iter_invariant(&it),
                    iter_seq == it@.1,
                    it@.0 <= iter_seq.len(),
                decreases iter_seq.len() - it@.0,
            {
                if let Some(x) = it.next() {
                    proof { items = items.push(*x); }
                } else {
                    break;
                }
            }

            assert(it@.0 == iter_seq.len());
            assert(items =~= iter_seq);
        }
    } => Ok(())
}
```

### Category B: Type axiom requires (Pair, feq, view_eq, etc.)

When the constructor or iter() has type axiom requires that callers must
satisfy, those propagate to the test function's `requires`. This proves the
axiom chain is satisfiable.

```rust
fn test_loop_borrow_iter()
    requires obeys_feq_clone::<Pair<u64, u64>>(), obeys_view_eq::<u64>(),
{
```

Import the axiom modules:
```rust
use apas_verus::vstdplus::feq::feq::*;
use vstd::laws_eq::obeys_view_eq;
```

**How to find the requires**: Read the module's `new()`/`singleton()` trait
signature. If it requires `obeys_feq_clone::<T>()` or `obeys_view_eq::<K>()`,
those go in the test function. The wf predicates do NOT — those are handled
by the constructor-to-iter chain.

## The four borrow patterns

All borrow-only modules support exactly 4 patterns:

| # | Pattern | Iterator source | `items.push` |
|---|---------|----------------|--------------|
| 1 | loop-borrow-iter | `t.iter()` | `*x` (deref) |
| 2 | loop-borrow-into | `(&t).into_iter()` | `*x` (deref) |
| 3 | for-borrow-iter | `for x in iter: t.iter()` | `*x` (deref) |
| 4 | for-borrow-into | `for x in iter: (&t).into_iter()` | `*x` (deref) |

## The two consume patterns (if module has consuming IntoIterator)

| # | Pattern | Iterator source | `items.push` |
|---|---------|----------------|--------------|
| 5 | loop-consume | `t.into_iter()` | `x` (owned) |
| 6 | for-consume | `for x in iter: t.into_iter()` | `x` (owned) |

Consuming patterns use `x` not `*x` because the item is owned.

## Loop invariants (manual loop)

Always the same four:
```rust
invariant
    items =~= iter_seq.take(it@.0 as int),
    iter_invariant(&it),
    iter_seq == it@.1,
    it@.0 <= iter_seq.len(),
decreases iter_seq.len() - it@.0,
```

## For-loop invariants

Always the same three:
```rust
for x in iter: it
    invariant
        iter.elements == iter_seq,
        items =~= iter_seq.take(iter.pos),
        iter.pos <= iter_seq.len(),
```

## View type in ghost variables

The `iter_seq` ghost variable type matches the iterator's View element type:
- `Seq<u64>` for simple element iterators (sets, sequences)
- `Seq<Pair<u64, u64>>` for table/map iterators (key-value pairs)
- `Seq<&u64>` — never. Borrow iterators yield `&T` but the ghost view is `Seq<T>`.

## Constructor

Build a small collection (2-4 elements) using the module's API. The
constructor's `ensures` must include wf — verify this by reading the trait.

- Sets: `singleton(1u64).insert(2u64).insert(3u64)`
- Tables: `singleton(1u64, 10u64).insert(2u64, 20u64).insert(3u64, 30u64)`
- Sequences: `new(3, 42u64)` or `singleton(1u64).push(2u64).push(3u64)`

Use explicit type suffixes (`1u64`) when the module is generic.

## Mt modules

Mt modules with iterators snapshot to a Vec internally. The constructor
and iter() signatures are the same pattern — check that `new()` ensures wf
and `iter()` requires wf. Mt modules may need additional lock-related setup.

## What NOT to include

- No runtime assertions (this is proof only).
- No complex logic — just iterate and collect.
- No testing of specific element values (we don't know the iteration order for
  sets/tables).
- No lemma calls unless the invariant doesn't prove without one.
