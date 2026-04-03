# R146 Agent 1 — TSM pilot: migrate BSTPlainMtEph with full View preserved. AFK.

## Record date
`date`

## What went wrong in R145

You dropped View and weakened all reader ensures to structural (`requires wf`).
That's NOT acceptable. The entire point of the migration is zero assumes WITH
the same value-level specs. Callers need `found == self@.tree_contains(*target)`,
`n as nat == self@.spec_size()`, etc. If we gut the specs to get zero assumes,
we've gained nothing — the specs were the whole reason the assumes existed.

**Do NOT change trait signatures. Do NOT drop View. Do NOT weaken ensures.**

## Setup

Read ALL files in `src/standards/` before starting. Pay close attention to:
- `rwlock_tsm_standard.rs` — the NEW standard.
- `toplevel_coarse_rwlocks_for_mt_modules.rs` — the OLD standard being replaced.

Read `src/Chap37/BSTPlainMtEph.rs` — the file to migrate. Read the CURRENT
version on the agent1/ready branch (your R145 changes). Then `git stash` or
`git checkout origin/main -- src/Chap37/BSTPlainMtEph.rs` to restore the
original pre-R145 version. Start the migration fresh from the original.

Read `src/experiments/bst_plain_mt_tsm.rs` — the experiment. Note that the
experiment tracks only `count: nat` and has NO View. That was a simplification
for the experiment. The real module needs more.

Report file: `plans/r146-agent1-tsm-pilot-bstplain-report.md`

## The requirement

Migrate BSTPlainMtEph from ghost+accepts to TSM. After migration:
1. **View stays.** `impl View for BSTPlainMtEph<T>` remains. `self@` returns `BalBinTree<T>`.
2. **Trait signatures identical.** Same `&mut self` for writers, same requires/ensures.
3. **Reader ensures stay value-level.** `found == self@.tree_contains(*target)`,
   `n as nat == self@.spec_size()`, etc.
4. **Zero ghost-lock bridge assumes.** The TSM proves ghost == inner.
5. **2 View bridge accepts per file are OK.** This is the documented trade-off
   from the architecture doc. The ghost View field stays, but instead of assuming
   `ghost_root@ == locked_data` everywhere, the TSM proves it inside the lock,
   and 2 structured accepts connect the ghost View to the TSM token.

## How it works with View preserved

The struct keeps BOTH the TSM and the ghost View:

```rust
struct BSTPlainMtEph<T: TotalOrder> {
    lock: RwLock<BSTPlainLockInterior<T>, BSTPlainTSMInv>,
    inst: Tracked<BSTPlainSM::Instance>,
    ghost_root: Ghost<BalBinTree<T>>,  // KEPT — provides View
}
```

The TSM token inside the lock proves `token.value == inner@`. The predicate
ties them. After acquire_write, the predicate gives you `token.value == inner@`
for free — no assume. You then accept `ghost_root@ == token.value` (1 accept
per writer, 1 per reader).

Wait — that's the same 2 accepts as before but with extra TSM boilerplate.
The value is: the TSM ALSO proves wf, size bounds, BST ordering on the inner
data. The old pattern assumed ALL of those. The new pattern proves them from
the predicate and only accepts the View bridge.

Actually, the real win: for READERS, the TSM predicate proves
`inner@ == token.value`. The ghost_root was set on the last write to equal
the token value. If `&mut self` guarantees no interleaving, then
`ghost_root@ == token.value == inner@`. The reader can prove return values
from `inner@` (which the predicate gives) without assuming anything about
`ghost_root@`. The reader accept is: `result proved from inner@ == self@`.

So readers go from N assumes → 0 assumes + 0 accepts. Writers go from
1 assume → 0 assumes + 1 accept (ghost View update).

## Generics and TSM

The TSM field should track `size: nat` (not the full tree — generics in
tokenized_state_machine! may not work). The predicate is generic:

```rust
impl<T: TotalOrder> RwLockPredicate<BSTPlainLockInterior<T>> for BSTPlainTSMInv {
    open spec fn inv(self, v: BSTPlainLockInterior<T>) -> bool {
        v.inner.tree_is_bst()
        && v.inner.spec_size() == v.ghost_token@.value()  // size matches token
        && v.inner.spec_size() <= usize::MAX
        && v.inner.spec_height() <= usize::MAX
        && v.ghost_token@.instance_id() == self.instance.id()
    }
}
```

The predicate proves BST ordering + size bounds on `v.inner`. Reader operations
call `contains_node(inner, target)` which has ensures `found == inner.tree_contains(*target)`.
Since the predicate proves `inner.tree_is_bst()`, the inner operation's ensures
chain through. The return value is proved from `inner`, not from `self@`.

For the ensures `found == self@.tree_contains(*target)`: the reader needs to
connect `inner` to `self@`. After acquire_read, `inner` is the locked data.
`self@` is `ghost_root@`. The connection: `ghost_root` was set on the last
release_write to equal the locked data. With `&mut self` writers, no other
thread can have changed the data since. So `inner == ghost_root@ == self@`.
This is the inductive argument — and it's the 1 reader accept.

But wait: if the reader can prove the return value from `inner` directly,
and the ensures says `found == self@.tree_contains(*target)`, then the only
thing we need is `inner == self@`. That's 1 accept per reader. But it's the
SAME accept the old pattern had. The difference: the old pattern ALSO assumed
wf, size, height. The TSM proves those from the predicate.

So the improvement is: old pattern had 1-3 assumes per reader (bridge + value).
New pattern: 1 accept per reader (just the View bridge). Writers: old 1 assume,
new 1 accept. Net: 14 holes → ~11 accepts. Still a reduction from assumes
(unaudited) to accepts (documented), and wf/size/height are fully proved.

If that's not a big enough win, explore whether `&self` readers can avoid
the accept entirely by NOT referencing `self@` in ensures — instead reference
the locked data directly through a returned proof token. But that changes
the trait signature, which we said not to do.

## Validation

Run `scripts/validate.sh isolate Chap37`. Then `scripts/rtt.sh`.

## Rules

- Do NOT change trait signatures.
- Do NOT drop View.
- Do NOT weaken ensures.
- Writers: `&mut self`, update ghost_root after release.
- Readers: `&self`, accept `inner == self@` once.
- tokenized_state_machine! OUTSIDE verus!, everything else INSIDE.
- Layer 1 code (insert_node, delete_node, etc.) must not change.

## Record date again
`date`

## When done

Report: assumes before/after, accepts before/after, per operation table.

RCP.
