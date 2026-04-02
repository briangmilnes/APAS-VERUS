# R138 Agent 2 — Fix 6 Chap41 capacity assumes with exec-time guards at the lock boundary. AFK.

## Setup

Read ALL files in `src/standards/` before starting. Pay close attention to:
- `toplevel_coarse_rwlocks_for_mt_modules.rs` — the capacity bounds section
- `capacity_bounds_standard.rs` — capacity in requires, not wf

Report file: `plans/r138-agent2-chap41-capacity-report.md`

## Problem

`src/Chap41/AVLTreeSetMtEph.rs` and `src/Chap41/AVLTreeSetMtPer.rs` have 6
capacity assumes that need to become exec-time guards returning Result.

```
MtEph:270  assume(out@.len() < usize::MAX)           — to_seq
MtEph:421  assume(self.tree@.len() < usize::MAX)     — delete
MtPer:231  assume(vals@.len() < usize::MAX)           — to_seq
MtPer:378  assume(self + other len <= usize::MAX)     — union
MtPer:396  assume(tree@.len() < usize::MAX)           — delete
MtPer:408  assume(tree@.len() < usize::MAX)           — insert
```

## Critical rule: Result ONLY at the lock boundary

A previous agent (R131) put Result returns on the **public trait** methods. This
was wrong. The trait operations (insert, delete, union, to_seq) should NOT return
Result — they are the unlocked interface that operates on owned data.

The Result goes at the **locked wrapper** level — the code that acquires the lock,
checks capacity, calls the unlocked operation, and releases. If Chap41 currently
has no RwLock (it was restructured to store ParamBST directly), then either:

**Option A**: Put the RwLock back. The struct becomes:
```rust
struct AVLTreeSetMtEph<T> {
    lock: RwLock<ParamBST<T>, Inv>,
    ghost_view: Ghost<Set<T::V>>,
}
```
The locked trait methods acquire, check capacity, call ParamBST operations on
owned data, release. Result is on the locked trait. ParamBST operations don't
return Result — they're the unlocked layer.

**Option B**: If the current no-lock architecture must stay, add a `checked_`
wrapper layer that does the capacity check and returns Result, leaving the
core trait methods unchanged. The core methods (insert, delete, etc.) keep
their current signatures. The checked wrappers call them after the guard.

**Prefer Option A** — it matches the standard and the architecture doc.

## The fix pattern (Option A)

```rust
// Locked trait (public API):
fn insert(&mut self, x: T) -> Result<(), ()>
    requires old(self).wf(),
    ensures ...
{
    let (tree, write_handle) = self.lock.acquire_write();
    if tree.size() < usize::MAX {
        let new_tree = tree.insert(x);  // ParamBST insert — no Result
        // update ghost
        write_handle.release_write(new_tree);
        Ok(())
    } else {
        write_handle.release_write(tree);
        Err(())
    }
}

// The ParamBST operations (unlocked layer) do NOT return Result.
// They are called on owned data after the capacity check passes.
```

## For each assume

| # | File | Function | Guard |
|---|------|----------|-------|
| 1 | MtEph | to_seq | `if tree.size() < usize::MAX` before collect_in_order |
| 2 | MtEph | delete | `if tree.size() < usize::MAX` before delete (always true but prove it) |
| 3 | MtPer | to_seq | `if tree.size() < usize::MAX` before collect_in_order |
| 4 | MtPer | union | `if self.size() + other.size() <= usize::MAX` |
| 5 | MtPer | delete | `if tree.size() < usize::MAX` before delete |
| 6 | MtPer | insert | `if tree.size() < usize::MAX` before insert |

## Callers

When trait signatures change to return Result, update callers:
- Test files: `.unwrap()` is fine for tests
- Source callers: propagate the Result or use `match { Ok => ..., Err => ... }`
- Do NOT use `assume(false)` in Err arms of callers. Propagate the error.

## Validation

Run `scripts/validate.sh isolate Chap41`. Then `scripts/rtt.sh`.

## Rules

- Result ONLY at the lock boundary, NOT on unlocked/inner operations.
- Do NOT add assumes, accepts, or external_body.
- Do NOT use `assume(false)` in caller Err arms.
- Do NOT add capacity to wf (standard says not to).
- Propagate Result to callers. If a caller can't handle Err, that's a design
  issue to document, not an excuse for assume(false).

## When done

RCP.
