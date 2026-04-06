# R149 Agent 3 — Add Missing Debug/Display Impls: Chap44-Chap66. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Pay close attention to `table_of_contents_standard.rs` — Debug/Display go in
section 14 (derive impls outside verus!), OUTSIDE `verus!` but INSIDE `pub mod`.

Report file: `plans/r149-agent3-debug-display-report.md`

## Problem

Veracity rule [14] flags structs missing `impl Debug` and/or `impl Display` outside
`verus!`. These impls are required by the module standard for every struct.

## Your chapters

Chap44, Chap45, Chap47, Chap49, Chap50, Chap51, Chap52, Chap53, Chap54, Chap55,
Chap56, Chap57, Chap58, Chap59, Chap61, Chap62, Chap63, Chap64, Chap65, Chap66.

Expected ~100 [14] warnings in these chapters.

## How to find the warnings

```bash
~/projects/veracity/target/release/veracity-review-verus-style \
  -c ~/projects/APAS-VERUS \
  -e Chap21 -e vstdplus -e Types.rs -e Concurrency.rs -e experiments \
  -e lib.rs -e standards 2>&1 | grep 'warning: \[14\]' | grep -E 'Chap4[4-9]|Chap5[0-9]|Chap6[1-6]'
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

### Ghost/marker/Inv structs

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

### Iterator and ghost iterator structs

Same patterns as agents 1 and 2.

## Imports

If the file does not already have `use std::fmt::{Debug, Display, Formatter};`,
add it at the top of the module (inside `pub mod`, before `verus!`).

## Struct fields behind verus

Some structs have `Ghost<T>`, `Tracked<T>`, or `pub(crate)` fields. For these:
- Skip ghost/tracked fields in the format output.
- For `pub(crate)` fields, they ARE accessible within the module — use them.
- For `RwLock` fields, just print the type name.

## Ordering

Add impls in bottom-up order matching struct definition order. Place adjacent
to existing section 14 impls if any exist.

## Validation

Run `scripts/validate.sh isolate ChapNN` for each chapter you modify. Then
`scripts/rtt.sh`.

## Rules

- Do NOT modify anything inside `verus!`.
- Do NOT add assumes, accepts, or external_body.
- Do NOT change function logic or specs.
- All existing RTTs must pass.

## When done

RCP.
