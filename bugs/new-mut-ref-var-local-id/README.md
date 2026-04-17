# ICE: var_local_id failed with -V new-mut-ref + match in loop invariant

**Status**: Confirmed reproducible, standalone, unfixed as of `origin/main` (2026-04-16)  
**Verus version**: `release/rolling/0.2026.04.10.fc697a7`  
**Flag required**: `-V new-mut-ref`  
**Panic location**: `rustc_mir_build/src/builder/mod.rs:275`

## To reproduce

```bash
cd bugs/new-mut-ref-var-local-id
verus reproducer.rs --crate-type lib -V new-mut-ref
```

Expected output (partial):

```
thread 'rustc' panicked at rustc_mir_build/src/builder/mod.rs:275:13:
Verus Internal Error: var_local_id failed: (5, 38)
```

Without `-V new-mut-ref` the file verifies clean: `2 verified, 0 errors`.

## Minimal reproducer

```rust
use vstd::prelude::*;
verus! {
fn minimal(n: u64) {
    let mut found: Option<u64> = None;
    let mut i: u64 = 0;
    while i < n
        invariant
            match found { Some(k) => k < 1000u64, None => true },
        decreases n - i,
    {
        found = Some(i);
        i += 1;
    }
}
}
```

**Trigger**: a loop `invariant` that contains `match <opt> { Some(k) => expr(k), None => _ }`,
where `<opt>` is an `Option` variable and `k` is used in the `Some` branch expression.

No `&mut`, no `Vec`, no ghost variables, no `break` — just the match-binding pattern in a
`while` invariant under `-V new-mut-ref`.

## Minimization notes

| Variant | Panics? | Notes |
|---------|---------|-------|
| `match found { Some(_) => true, None => true }` | No | `k` not used in expr |
| `match found { Some(k) => k < 1000u64, None => true }` | **Yes** | `k` used |
| Remove `-V new-mut-ref` | No | Mode-specific |
| Add `&mut` parameter | Yes | Not required |
| Add `Vec` and `@` | Yes | Not required |
| Add `break` | Yes | Not required |
| Add `let ghost seq = v@` | Yes | Not required |

The only required elements are: (1) `while` loop, (2) `invariant` with `Some(k)` binding
where `k` appears in the branch body, (3) `-V new-mut-ref`.

## Verus panic comment

The comment at `rustc_mir_build/src/builder/mod.rs:260` identifies two root causes:

```
// Possible causes:
// 1. bug in verus_time_travel_prevention.rs, failing to emit the binding
// 2. rust_verify/src/erase.rs computes inconsistent vars map:
//    2a. a variable binding is Erase, but a use of that var is not Erase
//    2b. a variable binding is Erase, but a use of that var is missing from the map
```

The `Some(k)` match binding in a loop invariant likely creates a variable whose
Erase status is computed inconsistently between the binding site (in the invariant)
and the use site (in the invariant body), under the new erasure logic introduced
by `-V new-mut-ref`.

