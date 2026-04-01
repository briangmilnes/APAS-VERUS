# R131 Agent 1 — Chap41: convert capacity assumes to exec-time guards. AFK. DOT.

## Setup

Read ALL files in `src/standards/` before starting. Pay close attention to:
- `toplevel_coarse_rwlocks_for_mt_modules.rs` — the capacity bounds section (newly added)
- `capacity_bounds_standard.rs` — capacity goes in requires, not wf

Report file: `plans/r131-agent1-chap41-capacity-report.md`

## Problem

`src/Chap41/AVLTreeSetMtEph.rs` and `src/Chap41/AVLTreeSetMtPer.rs` have 5 capacity
assumes like `assume(self.tree@.len() < usize::MAX)` before insert/delete/union calls.

These are NOT proof obligations. They are exec-time precondition checks. The standard
says: check capacity after acquiring the lock (or before the operation), return `Err`
if the check fails. Same pattern as `count_down` in the rwlock standard.

## The 5 capacity assumes to fix

```
AVLTreeSetMtEph.rs:264: assume(out@.len() < usize::MAX)        — to_seq
AVLTreeSetMtEph.rs:386: assume(self.tree@.len() < usize::MAX)  — delete
AVLTreeSetMtPer.rs:226: assume(vals@.len() < usize::MAX)       — to_seq
AVLTreeSetMtPer.rs:348: assume(self.tree@.len() + other.tree@.len() <= usize::MAX) — union
AVLTreeSetMtPer.rs:364: assume(tree@.len() < usize::MAX)       — delete
AVLTreeSetMtPer.rs:375: assume(tree@.len() < usize::MAX)       — insert
```

## Fix pattern

For each assume, replace with an exec-time check:

```rust
// BEFORE (hole):
fn delete(&mut self, x: &T) {
    assume(self.tree@.len() < usize::MAX);
    self.tree.delete(x);
}

// AFTER (no hole):
fn delete(&mut self, x: &T) -> Result<(), ()> {
    if self.tree.size() < usize::MAX {
        self.tree.delete(x);
        Ok(())
    } else {
        Err(())
    }
}
```

If the trait method already returns `Result`, just add the check. If it doesn't,
you'll need to change the trait signature to return `Result`. Check callers before
changing signatures.

For `to_seq`: the capacity bound is on the output (from_vec requires len < MAX).
Check `self.tree.size() < usize::MAX` before calling collect_in_order + from_vec.

For `union`: check `self.tree.size() + other.tree.size() <= usize::MAX` (both
sizes are available via tree.size()).

## Also fix Chap43

`src/Chap43/OrderedSetMtEph.rs:547` has the same pattern:
`assume(inner@.len() + 1 < usize::MAX)` in insert/get_range.
Apply the same exec-time guard fix.

## Validation

Run `scripts/validate.sh isolate Chap41` then `scripts/validate.sh isolate Chap43`.
Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT add capacity to wf (standard says not to).
- Exec-time guards returning Result — that's the fix.
