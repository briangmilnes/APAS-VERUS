# R35 Agent 4 Update 2: Rework OrderedTableMtEph + Finish AugOrderedTableMtEph

## What you did

AugOrderedTableMtEph was excellent — -6 holes, zero new assumes. The
lemma_aug_view delegation works perfectly because the View is open.

OrderedSetMtEph was solid — proper RwLock delegation with 1 assume per
function, filter fully proved.

## What went wrong

OrderedTableMtEph ordering operations (first_key, last_key, previous_key,
next_key, rank_key) were implemented as **loop-based algorithms** that
iterate over entries. This is wrong. Mt modules delegate to St modules
through RwLock — they do NOT reimplement the algorithm.

The result: 7→17 holes. Each loop body has 2-4 assumes documenting
postconditions. The correct pattern has 1 assume per function.

## Mandatory: Rework OrderedTableMtEph ordering operations

Replace the loop-based implementations with proper RwLock delegation:

```rust
fn first_key(&self) -> (first: Option<K>)
    where K: TotalOrder
{
    // Same pattern as OrderedSetMtEph.first
    proof { use_type_invariant(self); }
    let rlock = self.base_table.inner.acquire_read();
    let result = rlock.first_key();
    self.base_table.inner.release_read(rlock);
    proof { assume(inner@ =~= self@); }
    result
}
```

Do this for all 5: first_key, last_key, previous_key, next_key, rank_key.

For select_key: you said it needs Vec::sort() which has no Verus specs.
If the StEph method works without sort, just delegate. If it truly needs
sort, leave it as external_body and report why.

**Expected after rework: 17 → ~7 holes** (1 assume per delegation + 1
external_body for filter or select_key if blocked).

## Then: Finish AugOrderedTableMtEph (2 remaining)

1. **calculate_reduction**: Same closure requires cascade that agent3
   used in AugOrderedTableStEph. Lift reducer requires into function
   requires or into wf. Read agent3's approach in AugOrderedTableStEph.rs
   (it's on main at this point or in your file from earlier).

2. **reduce_range_parallel**: Uses ParaPair! for fork-join. Do NOT
   sequentialize. If you can prove it with the parallel structure, do
   so. If blocked by fork-join closure specs, leave as external_body
   and report what's needed.

## Rules

- assume() only. NEVER accept().
- Do NOT modify CLAUDE.md.
- Do NOT modify ~/projects/veracity/.
- Mt modules DELEGATE to St through RwLock. They do NOT reimplement.
- One assume per delegation function for the ghost bridge.
- Run `scripts/validate.sh` after changes. 0 errors required.
- Update report at `plans/agent4-round35-report.md`.
- Commit, push to `agent4/ready`.
