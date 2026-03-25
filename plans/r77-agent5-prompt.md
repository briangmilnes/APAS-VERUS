# R77 Agent 5 — Apply constructor_feq_standard to BSTSetPlainMtEph + BSTSetBBAlphaMtEph

## Objective

The BSTSetPlainMtEph and BSTSetBBAlphaMtEph files are currently clean (0 holes), but
they don't yet follow the `constructor_feq_standard`. Their wf predicates don't include
`obeys_feq_clone::<T>()`, which means any future feq_clone usage would require scattered
assumes. Proactively align them with the new standard.

Also: audit all Chap37 BSTSet files for consistency — verify that the constructor_feq
pattern (wf includes feq_clone, empty/singleton require it, no assumes in bodies) is
uniformly applied across all 5 BSTSet variants (Plain, BBAlpha, AVL, RB, Splay).

## Files

| # | Chap | File | Current status | Action |
|---|------|------|----------------|--------|
| 1 | 37 | BSTSetPlainMtEph.rs | 0 holes, clean | Add feq_clone to wf, requires on constructors |
| 2 | 37 | BSTSetBBAlphaMtEph.rs | 0 holes, clean | Add feq_clone to wf, requires on constructors |
| 3 | 37 | BSTSetAVLMtEph.rs | 5 holes | Audit: confirm pattern is applied |
| 4 | 37 | BSTSetRBMtEph.rs | 5 holes | Audit: confirm pattern is applied |
| 5 | 37 | BSTSetSplayMtEph.rs | 5 holes | Audit: confirm pattern is applied |

## Pattern to apply (from src/standards/constructor_feq_standard.rs)

1. Wf predicate includes `obeys_feq_clone::<T>()`:
```rust
open spec fn spec_bstsetplainmteph_wf(&self) -> bool {
    self.tree.spec_bstplainmteph_wf() && obeys_feq_clone::<T>()
}
```

2. Constructors require it:
```rust
fn empty() -> (set: Self)
    requires obeys_feq_clone::<T>()
    ensures set.spec_bstsetplainmteph_wf();
```

3. No `assume(obeys_feq_clone)` in any function body.

4. Import: `#[cfg(verus_keep_ghost)] use crate::vstdplus::feq::feq::obeys_feq_clone;`

## For BSTSetPlainMtEph and BSTSetBBAlphaMtEph

These files may not currently use `in_order()` or clone in ways that need feq_clone.
Read the files first. If they don't call any function requiring feq_clone, adding it to
wf is still correct (forward-looking) but check that all callers still satisfy the new
requires. Callers in test code (RTT) are fine — requires not checked at runtime.

Check for internal `Self::empty()` calls in union/intersection/difference — they need
wf in scope to satisfy the new requires on empty().

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent5/ready`.

## Report

Write `plans/agent5-round77-report.md`. Confirm all 5 BSTSet variants follow the same
pattern. Table showing each file's status.
