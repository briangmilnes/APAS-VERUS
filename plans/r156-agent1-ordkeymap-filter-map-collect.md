# R156 Agent 1 — Add filter/map/reduce/collect/Clone to OrdKeyMap. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap38/OrdKeyMap.rs` — your file.
Read `src/Chap43/OrderedTableStEph.rs` — reference impls.

Report file: `plans/r156-agent1-ordkeymap-filter-map-collect-report.md`

## Task A: collect / in_order

```rust
fn collect(&self) -> (entries: Vec<Pair<K, V>>)
    requires self.spec_ordkeymap_wf(),
    ensures entries@.len() == self@.dom().len();
```

Delegate to `self.inner.in_order()`. ParamBST already has this.

## Task B: filter

```rust
fn filter<F: Fn(&K, &V) -> bool>(&self, pred: &F) -> (filtered: Self)
    requires
        self.spec_ordkeymap_wf(),
        forall|k: &K, v: &V| #[trigger] pred.requires((k, v)),
    ensures
        filtered.spec_ordkeymap_wf(),
        filtered@.dom().subset_of(self@.dom());
```

Iterate via in_order, conditionally insert into new OrdKeyMap.

## Task C: map (value transform)

```rust
fn map_values<F: Fn(&K, &V) -> V, U: StT + Ord>(&self, f: &F) -> (mapped: OrdKeyMap<K, U>)
    requires
        self.spec_ordkeymap_wf(),
        forall|k: &K, v: &V| #[trigger] f.requires((k, v)),
    ensures
        mapped.spec_ordkeymap_wf(),
        mapped@.dom() =~= self@.dom();
```

Iterate via in_order, apply f to each value, insert into new OrdKeyMap.

## Task D: reduce

```rust
fn reduce<F: Fn(&V, &V) -> V>(&self, f: &F, id: &V) -> (reduced: V)
    requires
        self.spec_ordkeymap_wf(),
        forall|v1: &V, v2: &V| #[trigger] f.requires((v1, v2)),
    ensures true;  // reduce spec is application-specific
```

## Task E: Clone for OrdKeyMap

StPer persistent operations need to clone OrdKeyMap. Delegate to
`ParamBST::clone()` if available, or iterate and rebuild.

```rust
impl<K: StT + Ord, V: StT + Ord> Clone for OrdKeyMap<K, V> {
    fn clone(&self) -> (cloned: Self)
        ensures cloned@ == self@
    {
        // ...
    }
}
```

## Approach

**Copy proof patterns from OrderedTableStEph.** These operations exist there
already. Adapt references from `self.tree.inner` to `self.inner`.

## Validation

`scripts/validate.sh isolate Chap38` during development.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify any Chap43 file.
- Do NOT add assumes, accepts, or external_body.
- All existing RTTs must pass.

## When done

RCP.
