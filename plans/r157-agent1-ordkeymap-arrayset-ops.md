# R157 Agent 1 — Add domain/tabulate/restrict/subtract to OrdKeyMap. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap41/OrdKeyMap.rs` — your file. NOTE: OrdKeyMap moved from Chap38
to Chap41 to unblock this exact task.
Read `src/Chap43/OrderedTableStEph.rs` — reference implementations.
Read `src/Chap41/ArraySetStEph.rs` — the type you can now import.

Report file: `plans/r157-agent1-ordkeymap-arrayset-ops-report.md`

## Problem

OrdKeyMap was previously in Chap38 and couldn't import ArraySetStEph (Chap41)
due to circular dependency. It's now in Chap41 and can use ArraySetStEph.

## What to add

### domain

```rust
fn domain(&self) -> (keys: ArraySetStEph<K>)
    requires self.spec_ordkeymap_wf(),
    ensures keys@ =~= self@.dom();
```

Iterate via in_order, collect keys into ArraySetStEph.

### tabulate

```rust
fn tabulate<F: Fn(&K) -> V>(keys: &ArraySetStEph<K>, f: &F) -> (table: Self)
    requires
        keys.spec_arraysetsteph_wf(),
        forall|k: &K| #[trigger] f.requires((k,)),
    ensures
        table.spec_ordkeymap_wf(),
        table@.dom() =~= keys@;
```

Iterate keys, apply f, insert each (k, f(k)) pair.

### restrict

```rust
fn restrict(&self, keys: &ArraySetStEph<K>) -> (restricted: Self)
    requires self.spec_ordkeymap_wf(), keys.spec_arraysetsteph_wf(),
    ensures
        restricted.spec_ordkeymap_wf(),
        restricted@.dom() =~= self@.dom().intersect(keys@);
```

Keep only entries whose key is in the given set.

### subtract

```rust
fn subtract(&self, keys: &ArraySetStEph<K>) -> (remaining: Self)
    requires self.spec_ordkeymap_wf(), keys.spec_arraysetsteph_wf(),
    ensures
        remaining.spec_ordkeymap_wf(),
        remaining@.dom() =~= self@.dom().difference(keys@);
```

Remove entries whose key is in the given set.

## Approach

**Copy proof patterns from OrderedTableStEph.** These operations exist there.
Adapt references.

Add `use crate::Chap41::ArraySetStEph::ArraySetStEph::*;` to imports.

## Validation

`scripts/validate.sh isolate Chap41` during development.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify any Chap43 file.
- Do NOT add assumes, accepts, or external_body.
- All existing RTTs must pass.

## When done

RCP.
