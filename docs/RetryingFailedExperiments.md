# Retrying Failed Experiments

## Problem

Experiments that currently fail Verus verification are valuable — they document what doesn't work yet and should be retried as Verus evolves. But failed code breaks the build, so it gets commented out in `lib.rs` and forgotten.

We want to keep failed code on disk, compiling but not verified by default, so that tooling (veracity) can periodically retry it.

## Convention

Two gates work together:

### 1. `#[cfg(feature = "failed_experiment")]` — in `lib.rs`

Registers the module so it's only included when explicitly requested:

```rust
pub mod experiments {
    // Active experiments
    pub mod working_thing;

    // Failed experiments — retried periodically
    #[cfg(feature = "failed_experiment")]
    pub mod broken_thing;
}
```

### 2. `#[cfg(feature = "failed_verification")]` — in the code itself

Gates the specific functions or blocks that fail verification. This lets a file contain both working and failing code:

```rust
verus! {

// This works — always verified
proof fn lemma_that_works()
    ensures 1 + 1 == 2int,
{}

// This doesn't work yet — gated
#[cfg(feature = "failed_verification")]
proof fn lemma_that_fails()
    ensures forall|s: Seq<int>| #![auto] s.len() >= 0,
{}

} // verus!
```

## When to use which

| Situation | Gate |
|---|---|
| Entire file fails | `failed_experiment` in `lib.rs` |
| Part of a file fails | `failed_verification` on the failing items |
| Both working and failing code in one file | `failed_experiment` in `lib.rs` + `failed_verification` on failing items |

If a file is **entirely** failed code, use `failed_experiment` in `lib.rs` only — no need to also gate every function inside.

If a file has **mixed** working and failing code, register it normally in `lib.rs` (no `failed_experiment`) and gate only the broken parts with `failed_verification`.

## Running retries

Normal verification (no failed code):

```bash
~/projects/verus/source/target-verus/release/verus --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors
```

Retry all failed experiments:

```bash
~/projects/verus/source/target-verus/release/verus --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors \
    --cfg 'feature="failed_experiment"' --cfg 'feature="failed_verification"'
```

If something that was failing now passes, promote it: remove the `cfg` gate and move the module registration out of the `failed_experiment` block in `lib.rs`.

## Adding `Cargo.toml` features

Both features must be declared in `Cargo.toml`:

```toml
[features]
failed_experiment = []
failed_verification = []
```

## Rationale

Verus declined `#[verifier::expect_failure]` (analogous to F\*'s `expect_failure`). This cfg-based convention achieves the same goal — keeping failed code alive for retrying — using standard Rust feature gating.
