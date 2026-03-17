# R36 Agent 3: OrderedTableMtPer + OrderedSet Cleanup

## Goal

Prove Mt delegation in OrderedTableMtPer.rs (8 holes), then clean up
remaining OrderedSet holes across StEph, StPer, and MtEph.

## Tier 1: OrderedTableMtPer.rs (8 holes)

6 external_body (ordering operations) + 2 assumes (RwLock ghost bridges).

This is the Mt persistent ordered table — delegation through
Arc<RwLock<...>> to OrderedTableStPer.

### Read first:
- Existing proved methods in OrderedTableMtPer.rs (insert, delete, find,
  size) — they show the delegation pattern for this specific file
- `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`

### Pattern:
```rust
fn first_key(&self) -> (first: Option<K>) where K: TotalOrder {
    proof { use_type_invariant(self); }
    let rlock = self.inner.acquire_read();
    let result = rlock.first_key();
    self.inner.release_read(rlock);
    proof { assume(inner@ =~= self@); }
    result
}
```

One assume per delegation. The 2 existing assumes are structural
(RwLock ghost bridges) — leave them as assume.

### Expected targets:
first_key, last_key, previous_key, next_key, rank_key, select_key

**Expected: -6 holes.**

## Tier 2: OrderedSet remaining holes

### OrderedSetStEph.rs (4 external_body)

These are the harder operations agent1 didn't prove in R35:
- rank: count elements less than k (TotalOrder scan)
- select: find k-th element by rank
- split: partition set at key (needs `where T: TotalOrder` on trait)
- to_seq: convert to sequence

Read agent1's R35 report for what blocked these. `rank` and `select`
involve filter-based specs with existential quantifiers. `split` needs
TotalOrder on the trait method.

Try rank first — it's a counting scan similar to first/last but counts
instead of tracking min/max.

### OrderedSetStPer.rs (4 external_body)

Mirror of StEph. Do StEph first.

### OrderedSetMtEph.rs (1 external_body)

`to_seq` — returns ArraySeqStPerS but StEph returns AVLTreeSeqStPerS.
May need a conversion loop.

**Expected: -2 to -6 more holes depending on difficulty.**

## Rules

- assume() only. NEVER accept().
- Do NOT modify CLAUDE.md.
- Do NOT modify ~/projects/veracity/.
- Read existing proved methods in each file FIRST.
- StEph first, then StPer, then Mt.
- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agent3-round36-report.md`.
- Commit, push to `agent3/ready`.
