# R149 Agent 4 — Fix Copyright Format + Generic Return Names. AFK.

## Setup

Read ALL files in `src/standards/` before starting.

Report file: `plans/r149-agent4-copyright-return-names-report.md`

## Task A: Copyright Format [24] — 68 files

### Problem

Veracity rule [24] expects the first line of every source file to be:

```
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
```

Most files have `//  Copyright ...` (double space, regular comment) or
`// Copyright ...` (single space). The fix is to change the first line to use
`//!` (doc comment prefix, single space after `//!`).

### How to find

```bash
~/projects/veracity/target/release/veracity-review-verus-style \
  -c ~/projects/APAS-VERUS \
  -e Chap21 -e vstdplus -e Types.rs -e Concurrency.rs -e experiments \
  -e lib.rs -e standards 2>&1 | grep 'warning: \[24\]'
```

### Fix

For each flagged file, change line 1 from whatever it is to exactly:

```
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
```

This is a single-line edit per file. Do NOT change line 2 or anything else.

## Task B: Generic Return Names [19] — 66 warnings

### Problem

Veracity rule [19] flags return values named `r` or `result` that should have
more descriptive names. These are all in exec functions returning `Result<(), ()>`
from Mt module trait methods.

### How to find

```bash
~/projects/veracity/target/release/veracity-review-verus-style \
  -c ~/projects/APAS-VERUS \
  -e Chap21 -e vstdplus -e Types.rs -e Concurrency.rs -e experiments \
  -e lib.rs -e standards 2>&1 | grep 'warning: \[19\]'
```

### Fix

For each flagged function, rename the return value from `r` or `result` to
something that describes the operation's outcome. Use these conventions:

| Function pattern | Return name |
|---|---|
| `insert(...)` | `inserted` |
| `delete(...)` / `remove(...)` | `removed` |
| `add_vertex(...)` | `added` |
| `add_arc(...)` / `add_edge(...)` / `add_labeled_arc(...)` / `add_labeled_edge(...)` | `added` |
| `set(...)` | `updated` |
| `clone(...)` | `cloned` |
| `eq(...)` | `equal` |
| `from_sorted_entries(...)` | `table` |
| `complex_query(...)` | `found` |
| `pq_entry_new(...)` | `entry` |
| Other | Use the function name's verb as past tense, or the return type's meaning |

When you rename the return value, you must also update:
1. The `ensures` clause where the return name appears.
2. Any `let` binding in the body that uses the old name.
3. The final expression if it references the old name.

Example:

```rust
// Before:
fn insert(&mut self, value: T) -> (r: Result<(), ()>)
    requires ...,
    ensures match r { Ok(_) => ..., Err(_) => ... };

// After:
fn insert(&mut self, value: T) -> (inserted: Result<(), ()>)
    requires ...,
    ensures match inserted { Ok(_) => ..., Err(_) => ... };
```

**IMPORTANT**: The return name appears in the trait declaration AND the impl.
Both must be updated to match. Search for every occurrence of the old return
name in the function's signature, ensures, and body.

## Validation

After all changes, run `scripts/validate.sh` (full, not isolate — you're touching
many chapters). Then `scripts/rtt.sh`.

## Rules

- Do NOT change function logic or specs (only return variable names).
- Do NOT add assumes, accepts, or external_body.
- All existing RTTs must pass.
- Copyright: change ONLY line 1. Do not touch line 2 or any other content.

## When done

RCP.
