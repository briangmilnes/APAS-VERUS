# R36 Agent 4: Rework OrderedTableMtEph + AugOrderedTableMtEph + Chap38

## Goal

Fix the OrderedTableMtEph loop→delegation regression, finish
AugOrderedTableMtEph, then tackle BSTParaMtEph if time.

## Tier 1: Rework OrderedTableMtEph.rs (17 → ~7 holes)

In R35 you wrote loop-based implementations for first_key, last_key,
previous_key, next_key, rank_key. This was wrong — **Mt modules
delegate to St modules through RwLock, they do NOT reimplement the
algorithm.** Each loop body has 3-4 assumes. Proper delegation needs 1.

### Replace each loop implementation with:

```rust
fn first_key(&self) -> (first: Option<K>) where K: TotalOrder {
    proof { use_type_invariant(self); }
    let rlock = self.base_table.inner.acquire_read();
    let result = rlock.first_key();
    self.base_table.inner.release_read(rlock);
    proof { assume(inner@ =~= self@); }
    result
}
```

Do this for all 5: first_key, last_key, previous_key, next_key, rank_key.

For select_key: delegate if the StEph method exists and works. If it
truly requires Vec::sort() with no Verus specs, leave as external_body.

**Expected: 17 → ~7 holes** (5 delegations × 1 assume each + filter
assume + select_key external_body + existing structural assumes).

## Tier 2: AugOrderedTableMtEph.rs (2 remaining)

1. **calculate_reduction**: Needs closure requires cascade. Agent3
   solved this in AugOrderedTableStEph.rs — added reducer totality
   to wf. Read `src/Chap43/AugOrderedTableStEph.rs` to see the pattern.
   Lift `forall|v1: &V, v2: &V| reducer.requires((v1, v2))` into
   wf or function requires.

2. **reduce_range_parallel**: Uses ParaPair! for fork-join parallelism.
   Do NOT sequentialize. If blocked by fork-join closure specs, leave
   as external_body and report what's needed.

**Expected: -1 to -2 holes.**

## Tier 3: BSTParaMtEph.rs (Chap38, 8 external_body)

If Tiers 1-2 go quickly, start on BSTParaMtEph.rs. Same Mt RwLock
delegation pattern wrapping BSTParaStEph. Read the file first, look
at any already-proved methods.

8 external_body functions — should be mechanical delegation.

**Expected: -5 to -8 holes if reached.**

## Rules

- assume() only. NEVER accept().
- Do NOT modify CLAUDE.md.
- Do NOT modify ~/projects/veracity/.
- **Mt modules DELEGATE to St through RwLock. They do NOT reimplement.**
- One assume per delegation for the ghost bridge.
- Read existing proved methods in the same file FIRST.
- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agent4-round36-report.md`.
- Commit, push to `agent4/ready`.
