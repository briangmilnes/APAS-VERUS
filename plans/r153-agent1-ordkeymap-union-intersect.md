# R153 Agent 1 — Add union/intersect/difference to OrdKeyMap. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `docs/ordered-bst-refactor.md` — the design doc.
Read `src/Chap38/OrdKeyMap.rs` — the module you're extending (built in R152).
Read `src/Chap43/OrderedTableStEph.rs` — reference impl for union/intersect/difference.

Report file: `plans/r153-agent1-ordkeymap-report.md`

## Problem

OrdKeyMap has new/size/is_empty/find/insert/delete/split but lacks
union/intersect/difference. These are the bulk operations that OrderedTable
needs. Without them, OrderedTable can't migrate to OrdKeyMap.

## What to add

### union

```rust
fn union(&self, other: &Self) -> (combined: Self)
    requires self.spec_ordkeymap_wf(), other.spec_ordkeymap_wf(),
    ensures
        combined.spec_ordkeymap_wf(),
        combined@.dom() =~= self@.dom().union(other@.dom()),
        forall|k: K::V| self@.contains_key(k) && !other@.contains_key(k)
            ==> combined@[k] == self@[k],
        forall|k: K::V| other@.contains_key(k)
            ==> combined@[k] == other@[k];
```

Implementation: delegate to `self.inner.union(&other.inner)` (ParamBST set union).
Bridge proof: pair_set_to_map distributes over set union when keys are unique.
The "other wins on collision" semantics matches ParamBST's behavior.

### intersect

```rust
fn intersect(&self, other: &Self) -> (common: Self)
    requires self.spec_ordkeymap_wf(), other.spec_ordkeymap_wf(),
    ensures
        common.spec_ordkeymap_wf(),
        common@.dom() =~= self@.dom().intersect(other@.dom());
```

Delegate to `self.inner.intersect(&other.inner)`. Bridge: same pattern.

### difference

```rust
fn difference(&self, other: &Self) -> (remaining: Self)
    requires self.spec_ordkeymap_wf(), other.spec_ordkeymap_wf(),
    ensures
        remaining.spec_ordkeymap_wf(),
        remaining@.dom() =~= self@.dom().difference(other@.dom());
```

Delegate to `self.inner.difference(&other.inner)`. Bridge: same pattern.

## Approach

Look at how OrderedTableStEph's `union` (255 lines) works. Most of those
lines are bridge proof. The actual delegation is short. Extract the bridge
proof pattern into a helper lemma if possible.

ParamBST's union/intersect/difference already ensure:
- `combined@ == self@.union(other@)` (in Set terms)
- `combined@.finite()`
- wf preserved

Your job is to prove the Map-level postconditions from the Set-level ensures.
The key lemma: `spec_pair_set_to_map(a.union(b)) == spec_pair_set_to_map(a).union_prefer_right(spec_pair_set_to_map(b))` when both sides have unique keys.

## Validation

`scripts/validate.sh isolate Chap38` during development.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrderedTableStEph or any Chap43 file.
- Do NOT add assumes, accepts, or external_body.
- All existing RTTs must pass.

## When done

RCP.
