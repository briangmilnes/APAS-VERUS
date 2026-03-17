# Veracity Proof Hole Counting Rules

Feedback for the veracity AI on what counts as a "hole" and what doesn't.

## Definition

A **hole** is a proof obligation that is not discharged. The proof team
works to reduce the hole count toward zero. Inflating the count with
non-holes wastes human time on triage.

## What IS a hole (count as `error:`)

| Type | Example | Why it's a hole |
|------|---------|----------------|
| `assume()` | `assume(x < n)` | Unverified assertion. SMT takes it on faith. |
| `external_body` on algorithmic logic | `#[verifier::external_body] fn insert(...)` | Function body not verified. |
| `unsafe impl Send/Sync` | `unsafe impl Send for Foo {}` | Rust safety obligation not machine-checked. |
| `assume_specification` | Rare. Assumes a spec for an external type. | Unverified spec. |
| `external` on algorithmic logic | `#[verifier::external] fn map(...)` | Entire function invisible to verifier. |

## What is NOT a hole (count as `info:` or separate section)

### accept() — NEVER a hole

`accept()` is the project's function for intentional, human-reviewed
holes. It replaces `assume()` after a human decides the obligation is
either structurally unprovable (e.g., RwLock ghost gap) or acceptable
(e.g., eq/clone workaround). Counting accept() as a hole defeats its
purpose — the human already reviewed and accepted it.

- `accept()` → `info: accept()`
- `external_body` with accept annotation → `info: external_body_accept_hole`
- `external_type_specification` with accept → `info: external_type_specification_accept_hole`

### fn_missing_* warnings — NEVER a hole

Missing `requires` or `ensures` is a spec-completeness warning, not a
proof gap. A function with no `requires` but correct behavior has zero
unproved obligations — it just lacks documentation of its contract.

- `fn_missing_requires` → warning section, not holes
- `fn_missing_wf_requires` → warning section, not holes
- `fn_missing_wf_ensures` → warning section, not holes
- `fn_missing_requires_ensures` → warning section, not holes

These should appear in a separate "Warnings" section of the output,
never in "Holes Found."

### external_type_specification — NEVER a hole (when accepted)

`external_type_specification` wraps a std type to give it Verus specs.
There's nothing to "prove" — the type already exists in Rust. When
marked as accepted, it's informational.

### Structural false positives — label but still count

These ARE in the hole count (they're real assume/external_body) but are
flagged as structural FPs because they can't be removed due to
language limitations:

- EQ_CLONE_ASSUME: assume/accept inside PartialEq::eq or Clone::clone
- STD_TRAIT_IMPL: external_body on Iterator::next, Ord::cmp, etc.
- THREAD_SPAWN: external_body on thread spawn boundaries
- RWLOCK_GHOST: assume bridging ghost state across RwLock
- UNSAFE_SEND_SYNC: unsafe impl Send/Sync on Ghost-containing types
- OPAQUE_EXTERNAL: external_body calling unspecified std functions

Note: EQ_CLONE_ASSUME are typically accept() calls, so they should
already be excluded by the accept() rule above. Only flag as
EQ_CLONE_ASSUME if they're still assume() (not yet converted to accept).

## Output structure

```
Holes Found: N total          ← only real holes (assume, external_body, unsafe impl)
   X × assume()
   Y × external_body
   ...

Warnings: M total             ← separate section
   A × fn_missing_requires
   B × fn_missing_wf_requires
   ...

Accepted (reviewed): P total  ← informational
   C × accept()
   D × external_body_accept_hole
   ...

Structural False Positives: Q detected
   ...

Real Actionable Holes: N - Q structural FPs in count
```

## The test

Before shipping a hole count, ask: "Would a proof engineer need to
write proof code to close this?" If no, it's not a hole.

- `assume(x < n)` → yes, need to prove x < n → hole
- `accept(x < n)` → no, human already reviewed → not a hole
- `fn_missing_requires` → no, just add the spec annotation → not a hole
- `external_body fn insert(...)` → yes, need to write the body → hole
- `external_body_accept_hole` → no, human accepted → not a hole
