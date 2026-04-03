# R145 Agent 1 — TSM pilot: migrate BSTPlainMtEph to RwLock+TSM. AFK.

## Record date
`date'

## Setup

Read ALL files in `src/standards/` before starting. Pay close attention to:
- `rwlock_tsm_standard.rs` — the NEW standard. Follow this pattern exactly.
- `toplevel_coarse_rwlocks_for_mt_modules.rs` — the OLD standard being replaced.

Read `src/experiments/bst_plain_mt_tsm.rs` — the experiment that proved this
pattern works for this exact data structure. 10 operations, zero assumes.

Read `src/Chap37/BSTPlainMtEph.rs` — the file to migrate.

Report file: `plans/r145-agent1-tsm-pilot-bstplain-report.md`

## Problem

`BSTPlainMtEph.rs` has 13 assumes + 1 accept at lock boundaries. All are
ghost-lock bridge assumes (ghost field != locked data). The TSM pattern
eliminates all of them.

## What to do

Migrate `src/Chap37/BSTPlainMtEph.rs` from the old RwLock+ghost pattern to
RwLock+TSM following `src/standards/rwlock_tsm_standard.rs`.

### Step 1: Define the TSM (OUTSIDE verus!)

```rust
tokenized_state_machine!(
    BSTPlainSM {
        fields {
            #[sharding(variable)]
            pub tree_view: ParamBSTView<T>,  // or Set<T::V>, whatever the abstract state is
        }
        init!{ initialize(initial: ...) { init tree_view = initial; } }
        transition!{ tr_insert(old_view, new_view) {
            require pre.tree_view == old_view;
            update tree_view = new_view;
        }}
        transition!{ tr_delete(old_view, new_view) { ... } }
        transition!{ tr_noop() { } }  // for failed insert/delete
        // ... one transition per mutating operation
    }
);
```

Read the experiment (`bst_plain_mt_tsm.rs`) to see how it defines the SM.
The experiment uses a simple count tracker. For the real module, track the
full abstract state (the BST view) so that read operations can prove their
return values.

### Step 2: Define the lock interior

```rust
pub struct BSTPlainLockInterior<T> {
    pub inner: ParamBST<T>,               // the real data
    pub ghost_token: Tracked<BSTPlainSM::tree_view>,  // TSM token
}
```

### Step 3: Define the predicate

```rust
pub ghost struct BSTPlainTSMInv<T> {
    pub instance: BSTPlainSM::Instance,
}

impl RwLockPredicate<BSTPlainLockInterior<T>> for BSTPlainTSMInv<T> {
    open spec fn inv(self, v: BSTPlainLockInterior<T>) -> bool {
        v.inner@ == v.ghost_token@.value()
        && v.ghost_token@.instance_id() == self.instance.id()
        && v.inner.spec_wf()  // whatever wf the inner type needs
    }
}
```

### Step 4: Replace the struct

Remove the `ghost ghost_root: Ghost<ParamBSTView<T>>` field.
Add `inst: Tracked<BSTPlainSM::Instance>`.
Change the RwLock to hold `BSTPlainLockInterior<T>`.

### Step 5: Convert each operation

For each writer (insert, delete, union):
1. `acquire_write` → get `(mut interior, write_handle)`
2. Predicate proves `interior.inner@ == interior.ghost_token@.value()` — NO ASSUME
3. Call the inner StEph operation on `interior.inner`
4. Step the TSM token in a proof block
5. `release_write(interior)`

For each reader (find, contains, size, height, is_empty, min, max):
1. `acquire_read` → get `read_handle`
2. Call the inner operation on `read_handle.borrow().inner`
3. Return value is proved by the inner operation's ensures — NO ACCEPT
4. `release_read()`

### Step 6: Remove all assumes and accepts

Every `assume(self.ghost_root@ == tree)` and `accept(...)` at the lock
boundary should be gone. If any remain, something is wrong with the TSM
setup.

### Step 7: Verify callably identical

The trait signatures should NOT change. Same `&self`/`&mut self`, same
requires/ensures. Callers don't know about the internal TSM.

RTTs should pass unchanged.

## Validation

Run `scripts/validate.sh isolate Chap37`. Then `scripts/rtt.sh`.

## Generics and tokenized_state_machine!

BSTPlainMtEph is generic over `T: TotalOrder`. The TSM must also be generic.
The `tokenized_state_machine!` macro may or may not support type parameters
directly. Investigate:

1. Try `tokenized_state_machine!(BSTPlainSM<T: TotalOrder> { ... })`.
2. If the macro doesn't support generics, check how other Verus codebases
   handle this. Search `~/projects/verus/source/rust_verify_test/tests/` and
   `~/projects/VerusCodebases/` for generic tokenized_state_machine examples.
3. If generics aren't supported, possible workarounds:
   - Erase the type: track `Set<int>` or a type-erased abstract state
   - Use a concrete instantiation for the pilot (e.g., `u64`) and document
     the generics limitation
   - Use `state_machine_macros::state_machine!` instead if it supports generics

The TSM field must track enough abstract state that reader ensures work.
The experiment (`bst_plain_mt_tsm.rs`) only tracks a count — that's why it
has no View and no reader specs. For the real module, the TSM must track
the full tree view (`BalBinTree<T>` or equivalent) so that `contains`,
`find`, `min`, `max` can prove their return values from the predicate.

If tracking the full tree view is not possible with the TSM macro, fall back
to tracking just `Set<T::V>` (the abstract set of values). This is enough
for `contains` and `find` but may not suffice for `min`/`max` ordering specs.
Document what you can and cannot prove.

## Rules

- ZERO assumes in the migrated code (except Clone/PartialEq Verus workarounds).
- ZERO accepts at lock boundaries.
- Do NOT change trait signatures.
- Do NOT change RTTs.
- Follow `rwlock_tsm_standard.rs` exactly.
- tokenized_state_machine! goes OUTSIDE verus!. Everything else INSIDE verus!.
- If generics don't work in the TSM macro, document the limitation clearly
  and implement the best alternative you can find.

## Record date again
`date'

## When done

Report: assumes before/after per operation. Verify zero ghost-lock assumes remain.

RCP.

