# R149b Agent 1 — Add Missing Debug/Display Impls: Chap02-Chap19. AFK.

## CRITICAL SAFETY RULES — READ FIRST

1. **NEVER modify `~/projects/verus/`.** Not a single file. Not for any reason.
2. **NEVER run `rm -rf` on any directory.** Not `target/`, not `target/verus/`,
   not anything. If something seems broken, STOP and report it.
3. **NEVER run `verus` or `cargo verus` directly.** Only use `scripts/validate.sh`,
   `scripts/rtt.sh`, and `scripts/ptt.sh`.
4. **Do NOT run PTTs.** Skip `scripts/ptt.sh` entirely. Adding impl blocks outside
   `verus!` shifts Verus's internal impl numbering, which breaks PTT function
   resolution. This is a known Verus infrastructure issue. The orchestrator will
   handle PTT regeneration after merging. Your job is: validate + RTT only.
5. **Do NOT delete the `target/` directory or any subdirectory of it.**

## Setup

Read ALL files in `src/standards/` before starting.
Pay close attention to `table_of_contents_standard.rs` — Debug/Display go in
section 14 (derive impls outside verus!), OUTSIDE `verus!` but INSIDE `pub mod`.

Report file: `plans/r149b-agent1-debug-display-report.md`

## Problem

Veracity rule [14] flags structs missing `impl Debug` and/or `impl Display` outside
`verus!`. These impls are required by the module standard for every struct.

## Your chapters

Chap02, Chap03, Chap05, Chap06, Chap11, Chap12, Chap17, Chap18, Chap19.

## How to find the warnings

```bash
~/projects/veracity/target/release/veracity-review-verus-style \
  -c ~/projects/APAS-VERUS \
  -e Chap21 -e vstdplus -e Types.rs -e Concurrency.rs -e experiments \
  -e lib.rs -e standards 2>&1 | grep 'warning: \[14\]' | grep -E 'Chap0[2356]|Chap1[12789]'
```

## Pattern

For every flagged struct, add Debug and Display impls outside `verus!` but inside
the `pub mod`. Follow the existing impls in the same file as a model.

### Simple structs (no generics, few fields)

```rust
impl Debug for MyStruct {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "MyStruct({}, {})", self.field1, self.field2)
    }
}

impl Display for MyStruct {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.field1, self.field2)
    }
}
```

### Generic structs

```rust
impl<T: Debug> Debug for MyStruct<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "MyStruct({:?})", self.seq)
    }
}

impl<T: Display> Display for MyStruct<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.seq.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}
```

### Ghost/marker structs (no meaningful fields)

```rust
impl Debug for MyInv {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "MyInv")
    }
}

impl Display for MyInv {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "MyInv")
    }
}
```

### Iterator structs

```rust
impl<'a, T: Debug> Debug for MyIter<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "MyIter({:?})", self.inner)
    }
}

impl<'a, T> Display for MyIter<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "MyIter")
    }
}
```

### Ghost iterator structs

```rust
impl<'a, T> Debug for MyGhostIterator<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "MyGhostIterator")
    }
}

impl<'a, T> Display for MyGhostIterator<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "MyGhostIterator")
    }
}
```

## Imports

If the file does not already have `use std::fmt::{Debug, Display, Formatter};`,
add it at the top of the module (inside `pub mod`, before `verus!`).

## Struct fields behind verus

Some structs have `Ghost<T>`, `Tracked<T>`, or `pub(crate)` fields that are not
accessible outside `verus!`. For these, print what you can:
- Skip ghost/tracked fields in the format output.
- For `pub(crate)` fields, they ARE accessible within the module — use them.
- For `RwLock` fields, just print the type name (not the lock contents).

## Ordering

Add Debug/Display impls in bottom-up order matching the struct definition order.
If the file already has some Debug/Display impls, add the missing ones adjacent
to the existing ones in section 14.

## Validation

Run `scripts/validate.sh isolate ChapNN` for each chapter you modify. Debug/Display
are outside verus!, so they should not affect verification. Then `scripts/rtt.sh`.

**Do NOT run `scripts/ptt.sh`.** PTT regeneration is the orchestrator's job.

## Rules

- Do NOT modify anything inside `verus!`.
- Do NOT add assumes, accepts, or external_body.
- Do NOT change function logic or specs.
- Do NOT modify `~/projects/verus/` for any reason.
- Do NOT delete `target/` or any subdirectory.
- Do NOT run PTTs.
- All existing RTTs must pass.

## When done

RCP.
