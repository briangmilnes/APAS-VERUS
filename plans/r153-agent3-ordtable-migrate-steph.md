# R153 Agent 3 — Migrate OrderedTableStEph to use OrdKeyMap. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `docs/ordered-bst-refactor.md` — the design doc.
Read `src/Chap38/OrdKeyMap.rs` — the bridge layer you're migrating to.
Read `src/Chap43/OrderedTableStEph.rs` thoroughly — this is the file you're
modifying.

Report file: `plans/r153-agent3-ordtable-migrate-report.md`

## Problem

OrderedTableStEph has 5,569 lines, 3,809 proof lines. It wraps
`ParamBST<Pair<K,V>>` directly and contains 17 bridge lemmas + 6 large
`bst_*_by_key` re-implementations to convert Set→Map. OrdKeyMap now provides
this bridge. OrderedTable should delegate to OrdKeyMap instead.

## What to do

### Step 1: Change the struct field

```rust
// Before:
pub struct OrderedTableStEphS<K, V> {
    pub tree: ParamBST<Pair<K, V>>,
}

// After:
pub struct OrderedTableStEphS<K, V> {
    pub tree: OrdKeyMap<K, V>,
}
```

Update the import:
```rust
use crate::Chap38::OrdKeyMap::OrdKeyMap::*;
```

### Step 2: Update View

```rust
// Before:
impl View for OrderedTableStEphS<K, V> {
    type V = Map<K::V, V::V>;
    open spec fn view(&self) -> Map<K::V, V::V> {
        spec_pair_set_to_map(self.tree@)  // bridge conversion
    }
}

// After:
impl View for OrderedTableStEphS<K, V> {
    type V = Map<K::V, V::V>;
    open spec fn view(&self) -> Map<K::V, V::V> {
        self.tree@  // OrdKeyMap already views as Map
    }
}
```

### Step 3: Simplify trait methods

For each method that OrdKeyMap implements (find, insert, delete, split, size,
is_empty), simplify to delegate:

```rust
// Before (insert — ~143 lines with bridge proof):
fn insert(&mut self, key: K, val: V) {
    // ... 143 lines of BST manipulation + bridge proof
}

// After:
fn insert(&mut self, key: K, val: V) {
    self.tree.insert(key, val);
}
```

The ensures stay the same — OrdKeyMap's ensures match what OrderedTable needs.

### Step 4: Delete bridge lemmas

Once all methods delegate to OrdKeyMap, the 17 bridge lemmas in
OrderedTableStEph are dead code. Delete them (they live in OrdKeyMap now).

Also delete:
- `spec_pair_set_to_map` — in OrdKeyMap
- `spec_key_unique_pairs_set` — in OrdKeyMap
- `spec_set_pair_view_generated` — in OrdKeyMap
- `bst_find_by_key` — replaced by `self.tree.find()`
- `bst_split_by_key` — replaced by `self.tree.split()`

### Step 5: Methods OrdKeyMap doesn't have yet

OrdKeyMap may not yet have union/intersect/difference/next/prev/rank/select
(Agent 1 and 2 are building those in parallel). For methods OrdKeyMap doesn't
have yet, KEEP the existing OrderedTable implementation but change it to access
`self.tree.inner` (the underlying ParamBST). This is a temporary bridge until
OrdKeyMap gets those methods.

```rust
// Temporary for union until OrdKeyMap has it:
fn union(&self, other: &Self) -> (combined: Self) {
    // Access the underlying ParamBST through OrdKeyMap
    let merged_inner = self.tree.inner.union(&other.tree.inner);
    // ... existing bridge proof, using self.tree.inner instead of self.tree
    Self { tree: OrdKeyMap { inner: merged_inner } }
}
```

### Step 6: wf predicate

```rust
// Before:
open spec fn spec_orderedtablesteph_wf(&self) -> bool {
    self.tree.spec_bstparasteph_wf()
    && spec_key_unique_pairs_set(self.tree@)
    && ... lots of conditions
}

// After:
open spec fn spec_orderedtablesteph_wf(&self) -> bool {
    self.tree.spec_ordkeymap_wf()  // OrdKeyMap's wf bundles everything
}
```

## Approach

Do this incrementally. After each method migration:
1. `scripts/validate.sh isolate Chap43`
2. If it fails, check that the ensures match
3. Fix any bridge proof gaps

Start with the easy ones (new, size, is_empty, find), then insert/delete,
then split. Leave union/intersect/difference/next/prev/rank/select for last
(they may need OrdKeyMap support from agents 1 and 2).

## Expected reduction

From ~5,569 lines to ~2,000-2,500 lines. The 17 bridge lemmas (~500 lines),
the `bst_*` functions (~1,500 lines), and the bridge proof in each method
body (~1,000 lines) should be eliminated.

## Validation

`scripts/validate.sh isolate Chap43` after each method.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrdKeyMap.rs or any Chap38 file.
- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken any ensures on OrderedTableStEphTrait.
- All existing RTTs must pass.
- COMMENT OUT deleted code with `// BYPASSED:` first, validate, then delete
  only after confirming the delegation works.

## When done

RCP. Report line count before/after, which methods migrated, which are temporary.
