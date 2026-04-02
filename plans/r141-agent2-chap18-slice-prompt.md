# R141 Agent 2 — Create Chap18 ArraySeqMtEphSlice. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap19/ArraySeqMtEphSlice.rs` — the source to copy from.
Read `src/Chap18/ArraySeqMtEph.rs` — the Chap18 Vec-backed interface.

Report file: `plans/r141-agent2-chap18-slice-report.md`

## Problem

We have a slice-backed sequence in Chap19 but not Chap18. APAS introduces
sequences in Chap18 and extends them in Chap19. We need a Chap18 slice version
with the Chap18 interface.

## What to do

1. Copy `src/Chap19/ArraySeqMtEphSlice.rs` to `src/Chap18/ArraySeqMtEphSlice.rs`.

2. Update the module path: `Chap19::ArraySeqMtEphSlice` → `Chap18::ArraySeqMtEphSlice`.

3. Remove `subseq_copy` from the trait and impl — that's the Chap19 addition.
   Keep `slice` (which is the O(1) operation).

4. Update the module name, copyright, doc comments to say Chap18.

5. Register the new file in `src/Chap18/mod.rs` (or wherever Chap18 modules
   are declared) and in `Cargo.toml` if needed.

6. Register in `src/lib.rs` under the Chap18 module.

7. Add a test file `tests/Chap18/TestArraySeqMtEphSlice.rs` — can be a copy
   of the Chap19 tests minus subseq_copy tests. Register in Cargo.toml.

## The Chap18 interface

These operations should be in the Chap18 slice:
- length, nth_cloned, empty, singleton, new, slice, from_vec, to_vec
- iter
- reduce, map, filter, tabulate, scan, flatten
- is_empty, is_singleton (if agent1 adds them to Chap19 slice first; if not,
  add them here)
- append, update, inject, ninject, set (same caveat — if agent1 hasn't added
  these yet, add them here, agent1 can use yours as reference)

Do NOT include: subseq_copy (that's Chap19-only).

## Validation

Run `scripts/validate.sh isolate Chap18`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Module must be standalone (no imports from Chap19 slice).
- Follow the table of contents standard.

## When done

RCP.
