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

**Do NOT write these proofs from scratch.** OrderedTableStEph already has
working, verified proofs for union (255 lines), intersect (130 lines), and
difference. COPY the proof bodies from OrderedTableStEph into OrdKeyMap and
adapt:

1. Open `src/Chap43/OrderedTableStEph.rs`
2. Find the `union` function body — it has working proof assertions
3. Copy the proof logic into OrdKeyMap's `union` method
4. Adapt references: `self.tree` → `self.inner`, remove any OrderedTable-specific
   wf references, use OrdKeyMap's wf and bridge lemmas instead
5. The core proof structure (case analysis, lemma calls) stays the same

The proofs work because they're proving the same thing — Map-level postconditions
from Set-level BST ensures. The only difference is which wf predicate and which
bridge lemmas are in scope. OrdKeyMap has the same bridge lemmas, so the proofs
should transfer with minimal adaptation.

## Validation

`scripts/validate.sh isolate Chap38` during development.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrderedTableStEph or any Chap43 file.
- Do NOT add assumes, accepts, or external_body.
- All existing RTTs must pass.

## When done

RCP.
