# R132 Agent 1 — Strengthen OrderedTableMtEph::find ensures. AFK. DOT.

## Setup

Read ALL files in `src/standards/` before starting.

Report file: `plans/r132-agent1-chap43-find-report.md`

## Problem

`src/Chap43/OrderedTableMtEph.rs` — the `find` method's ensures only guarantees
key containment, NOT value correspondence:

```rust
// Current (weak):
ensures Some(v) => self@.contains_key(k@)
// Should be (strong):
ensures Some(v) => self@.contains_key(k@) && v@ == self@[k@]
```

The StEph version (`OrderedTableStEph::find`) has the strong spec. The MtEph wrapper
loses the value through the lock-boundary ghost gap.

## What to do

1. Read `src/Chap43/OrderedTableStEph.rs` — find the `find` method and its ensures.
2. Read `src/Chap43/OrderedTableMtEph.rs` — find the `find` method, understand the
   lock boundary (acquire_read → borrow → call StEph find → release).
3. Strengthen the MtEph `find` ensures to include `v@ == self@[k@]`.
4. The proof needs to bridge through the lock: the predicate proves inner wf,
   StEph find ensures value correspondence on the inner value, and the ghost/inner
   accept bridges to self@. Follow the standard accept pattern.

## Validation

Run `scripts/validate.sh isolate Chap43`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add new assumes or external_body.
- The accept at the lock boundary is the standard allowed pattern.
- Do NOT change the StEph file.
