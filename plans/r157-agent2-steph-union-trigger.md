# R157 Agent 2 — Backport StPer Union Trigger Technique to StEph + Final Delegation. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap41/OrdKeyMap.rs` — all available methods.
Read `src/Chap43/OrderedTableStEph.rs` — your file.
Read `src/Chap43/OrderedTableStPer.rs` — agent 3 (R156) got union/intersect
working here using a trigger bridge technique. READ HOW THEY DID IT.

Report file: `plans/r157-agent2-steph-union-trigger-report.md`

## Problem

R156 agent 2 could NOT delegate StEph's union/intersection to OrdKeyMap because
of a "Verus closure-ensures identity gap." But R156 agent 3 successfully
delegated StPer's union/intersection using a trigger bridge:

```rust
proof {
    assert forall|k: K::V| hyp(k) implies exists|r: V::V| ... by {
        let vk = result@[k];
        assert(result@[k] == vk);  // materializes ground term for Z3
    }
}
```

Apply the same technique to StEph.

## Task A: Delegate union to OrdKeyMap::union_with

1. Read OrderedTableStPer's `union` delegation (from R156 agent 3)
2. Apply same pattern to OrderedTableStEph's `union`
3. Delete the ~255 line iterative implementation
4. Validate

## Task B: Delegate intersection to OrdKeyMap::intersect_with

Same approach.

## Task C: Delete remaining dead code

After union/intersect delegate, check for dead bridge lemmas and spec fns.
Delete anything unreferenced.

## Task D: Delegate any remaining methods

Check all methods still using `self.tree.inner`. For each:
- If OrdKeyMap has an equivalent, delegate
- If not, leave it
- Report what's left

OrdKeyMap now has: new, size, is_empty, find, insert, delete, split,
union, union_with, intersect, intersect_with, difference, next_key,
prev_key, rank_key, select_key, first_key, last_key, get_key_range,
split_rank_key, collect, filter, map_values, reduce, Clone.

Agent 1 may also add domain/tabulate/restrict/subtract this round.

## Validation

`scripts/validate.sh isolate Chap43` after each change.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrdKeyMap.rs.
- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken any ensures.
- All existing RTTs must pass.

## When done

RCP. Report line count before/after.
