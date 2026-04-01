# R131 Agent 3 — Chap41: prove to_seq bridge + PartialEq holes. AFK.

## Setup

Read ALL files in `src/standards/` before starting. Pay close attention to:
- `partial_eq_eq_clone_standard.rs` — PartialEq/Clone patterns
- `spec_wf_standard.rs` — wf predicates

Report file: `plans/r131-agent3-chap41-proof-report.md`

## Problem

`src/Chap41/AVLTreeSetMtEph.rs` and `src/Chap41/AVLTreeSetMtPer.rs` have proof
holes that are provable with the right lemmas/assertions.

## Holes to prove

### 1. to_seq bridge (4 holes — 2 per file)

```
MtEph:267: assume(seq@.to_set() =~= self@)
MtEph:269: assume(forall|i| 0 <= i < seq@.len() ==> self@.contains(seq@[i]))
MtPer:229: assume(seq@.to_set() =~= self@)
MtPer:230: assume(forall|i| 0 <= i < seq@.len() ==> ...)
```

These follow from `collect_in_order` on the BST. Read what `collect_in_order`
(or the in-order traversal function) ensures. The ensures should say something
like "the returned sequence contains exactly the tree's elements." If the ensures
are too weak, strengthen them in `BSTParaMtEph` or add intermediate assertions.

The chain: `self.tree.collect_in_order()` returns a Vec whose elements are exactly
the tree's set. `from_vec` converts to ArraySeqStEphS. Then `seq@.to_set()` should
equal `self.tree@` which equals `self@` (View delegates to tree@).

### 2. PartialEq early return (1 hole — MtPer only)

```
MtPer:390: assume(false == (self@ == other@))
```

When `self.size() != other.size()`, the sets are unequal. For finite sets,
`self@.len() != other@.len() ==> self@ != other@`. This follows from
set extensionality: equal sets have equal cardinality.

Try: `assert(self@.len() != other@.len())` then use the contrapositive —
if `self@ == other@` then `self@.len() == other@.len()`, contradiction.

### 3. clone-preserves-view (1 hole — MtEph only)

```
MtEph:307: assume(elem@ == seq@[i])
```

This is the clone workaround. Use `axiom_cloned_implies_eq_owned` from
`crate::vstdplus::feq::feq`. The pattern:

```rust
let elem = seq[i].clone();
proof {
    assert(cloned(seq@[i as int], elem));
    axiom_cloned_implies_eq_owned(seq@[i as int], elem);
    // Now: elem == seq@[i as int], therefore elem@ == seq@[i as int]@
}
```

Read `partial_eq_eq_clone_standard.rs` for the full pattern.

## Validation

Run `scripts/validate.sh isolate Chap41`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken ensures.
- If a hole can't be proved, leave it and report what you tried and where you got stuck.
