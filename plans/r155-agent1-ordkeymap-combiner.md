# R155 Agent 1 — Add Combiner-Function Union/Intersect to OrdKeyMap + Fix new() wf. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap38/OrdKeyMap.rs` — your file.
Read `src/Chap43/OrderedTableStEph.rs` — reference impl for combiner union/intersect.

Report file: `plans/r155-agent1-ordkeymap-combiner-report.md`

## Task A: Fix new() to ensure wf

`OrdKeyMap::new()` currently ensures `self@ == Map::empty()` but NOT
`self.spec_ordkeymap_wf()`. Fix it so callers (OrderedTable::empty,
OrderedTable::singleton) can delegate.

## Task B: Add union_with and intersect_with

OrderedTable's union/intersect take a combiner function `F: Fn(&V,&V)->V`
for collision resolution. OrdKeyMap's current union/intersect are fixed
(right-biased / self-preserving). Add combiner variants:

```rust
fn union_with<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: &F) -> (combined: Self)
    requires
        self.spec_ordkeymap_wf(),
        other.spec_ordkeymap_wf(),
        forall|v1: &V, v2: &V| #[trigger] combine.requires((v1, v2)),
    ensures
        combined.spec_ordkeymap_wf(),
        combined@.dom() =~= self@.dom().union(other@.dom()),
        forall|k: K::V| self@.contains_key(k) && !other@.contains_key(k)
            ==> #[trigger] combined@[k] == self@[k],
        forall|k: K::V| !self@.contains_key(k) && other@.contains_key(k)
            ==> #[trigger] combined@[k] == other@[k],
        forall|k: K::V| self@.contains_key(k) && other@.contains_key(k)
            ==> combine.ensures((&self@[k], &other@[k]), combined@[k]);

fn intersect_with<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: &F) -> (common: Self)
    requires
        self.spec_ordkeymap_wf(),
        other.spec_ordkeymap_wf(),
        forall|v1: &V, v2: &V| #[trigger] combine.requires((v1, v2)),
    ensures
        common.spec_ordkeymap_wf(),
        common@.dom() =~= self@.dom().intersect(other@.dom()),
        forall|k: K::V| #[trigger] common@.contains_key(k) ==>
            combine.ensures((&self@[k], &other@[k]), common@[k]);
```

**Do NOT write these proofs from scratch.** Read OrderedTableStEph's `union`
(~255 lines) and `intersection` (~130 lines). Copy the proof logic. Adapt
references from `self.tree.inner` to `self.inner`.

## Task C: Strengthen split ensures with disjointness

OrdKeyMap::split currently ensures left/right partition but not Map-level
disjointness. Add:

```rust
ensures
    ...existing ensures...,
    parts.0@.dom().disjoint(parts.2@.dom()),
```

This comes from ParamBST's split which already ensures set disjointness.
Bridge: `pair_set_to_map` preserves disjointness of domains when key-unique.

## Validation

`scripts/validate.sh isolate Chap38` during development.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify any Chap43 file.
- Do NOT add assumes, accepts, or external_body.
- All existing RTTs must pass.

## When done

RCP.
