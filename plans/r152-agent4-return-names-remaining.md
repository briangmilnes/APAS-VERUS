# R152 Agent 4 — Fix Remaining Return Names [19]. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.

Report file: `plans/r152-agent4-return-names-report.md`

## Problem

Veracity rule [19] flags ~66 return values named `r` or `result`. Tocify may
have reverted some R149 fixes. Fix all of them.

## How to find

```bash
~/projects/veracity/target/release/veracity-review-verus-style \
  -c ~/projects/APAS-VERUS \
  -e Chap21 -e vstdplus -e Types.rs -e Concurrency.rs -e experiments \
  -e lib.rs -e standards 2>&1 | grep 'warning: \[19\]'
```

## Fix

Rename return values from `r` or `result` to descriptive names:

| Function pattern | Return name |
|---|---|
| `insert(...)` | `inserted` |
| `delete(...)` / `remove(...)` | `removed` |
| `add_vertex(...)` | `added` |
| `add_arc/edge/labeled_*` | `added` |
| `set(...)` | `updated` |
| `clone(...)` | `cloned` |
| `eq(...)` | `equal` |
| `from_sorted_entries(...)` | `table` |
| `complex_query(...)` | `found` |
| `pq_entry_new(...)` | `entry` |
| `partition_*` | `partitioned` |
| `map_dc_vec` | `mapped` |
| `filter_dc_vec` | `filtered` |
| `tabulate_dc_vec` | `tabulated` |
| `flatten_dc_vec` | `flattened` |
| `concat_*` | `concatenated` |
| `build_*` | the thing being built |

When renaming, update:
1. The return name in the trait declaration AND the impl.
2. The `ensures` clause where the old name appears.
3. Any `let` binding or `proof { assume(...) }` in the body that uses the old name.

## Validation

Run `scripts/validate.sh` (full). Then `scripts/rtt.sh`.

## Rules

- Do NOT change function logic or specs (only return variable names).
- Do NOT add assumes, accepts, or external_body.
- All existing RTTs must pass.

## When done

RCP.
