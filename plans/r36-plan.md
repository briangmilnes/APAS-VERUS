# R36 Plan: 4 Agents, 94 Real Actionable Holes

## Current State

4265 verified, 0 errors. 129 total holes, 94 real actionable (35 SFPs).
37 clean chapters, 9 holed.

### Holes by chapter (excluding Examples, warnings, SFPs)

| # | Chap | Holes | Files |
|---|------|-------|-------|
| 1 | 43 | 70 | 10 files |
| 2 | 41 | 15 | 3 files (excl Example) |
| 3 | 38 | 10 | 2 files |
| 4 | 39 | 10 | 1 file |
| 5 | 47 | 10 | 5 files |
| 6 | 57 | 4 | 1 file |
| 7 | 37 | 2 | 2 files |
| 8 | 59 | 1 | 1 file |

### Chap43 breakdown (the mass)

| # | File | Holes | Type |
|---|------|-------|------|
| 1 | OrderedTableMtEph.rs | 17 | 14 rwlock + 2 algorithmic + 1 ext_body |
| 2 | OrderedSetMtEph.rs | 14 | 8 rwlock + 5 rwlock:pred + 1 ext_body |
| 3 | OrderedTableStEph.rs | 10 | 10 ext_body |
| 4 | OrderedTableStPer.rs | 9 | 9 ext_body |
| 5 | OrderedTableMtPer.rs | 8 | 6 ext_body + 2 rwlock |
| 6 | OrderedSetStPer.rs | 4 | 4 ext_body |
| 7 | OrderedSetStEph.rs | 4 | 4 ext_body |
| 8 | AugOrderedTableStPer.rs | 2 | 1 closure + 1 proof_fn |
| 9 | AugOrderedTableMtEph.rs | 2 | 2 ext_body |

## Agent Assignments

### Agent 1: Chap41 AVLTreeSetMt (13 ext_body + 3 assume)

Target: AVLTreeSetMtPer.rs (8 ext_body) + AVLTreeSetMtEph.rs (5 ext_body +
1 rwlock assume + 2 unsafe impl).

These are Mt RwLock delegation modules wrapping AVLTreeSetStEph/StPer.
Same pattern as OrderedSetMtEph — acquire_read, call St method,
release_read, assume bridge.

Read OrderedSetMtEph.rs (agent4's R35 work) for the delegation pattern.
The 2 unsafe impl Send/Sync are structural FPs — leave them.

**Expected: -10 to -13 holes. Could clean both files.**

### Agent 2: Chap43 OrderedTableStEph/StPer ordering operations (19 ext_body)

Continue R35 work. The TotalOrder bridging technique is now proven —
agent1 solved it in OrderedSetStEph.rs (lines 369-460). Use
`TotalOrder::cmp` + match on Ordering + `T::reflexive`/`T::transitive`.

Read `src/Chap43/OrderedSetStEph.rs` lines 369-460 FIRST.

Targets:
- OrderedTableStEph.rs: first_key, last_key, previous_key, next_key,
  rank_key, select_key, get_key_range, split_key (10 ext_body)
- OrderedTableStPer.rs: same operations (9 ext_body)

For `&pair.0 >= k1` PartialOrd issue: use `TotalOrder::cmp(&pair.0, k1)`
instead of `&pair.0 >= k1`.

StEph first, then mirror to StPer.

**Expected: -10 to -15 holes.**

### Agent 3: Chap43 OrderedTableMtPer + OrderedSetMtEph/StEph/StPer cleanup

Tier 1 — OrderedTableMtPer.rs (8 holes):
- 6 ext_body: ordering operations (first_key through select_key)
- 2 assumes: RwLock ghost bridges (structural, leave as assume)

Same Mt RwLock delegation pattern. Read existing proved methods in the
file (insert, delete, find, size). acquire_read → call StPer → release_read
→ assume(inner@ =~= self@).

Tier 2 — if time, clean up remaining OrderedSet holes:
- OrderedSetStEph.rs: 4 ext_body (rank, select, split, to_seq)
- OrderedSetStPer.rs: 4 ext_body (rank, select, split, to_seq)
- OrderedSetMtEph.rs: 1 ext_body (to_seq)

**Expected: -6 to -12 holes.**

### Agent 4: Rework OrderedTableMtEph + AugOrderedTableMtEph + Chap38/39

Tier 1 — OrderedTableMtEph.rs rework (17 → ~7):
Replace the 5 loop-based ordering implementations (first_key, last_key,
previous_key, next_key, rank_key) with proper RwLock delegation:

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

One assume per delegation, not 3-4 per loop. select_key: if StEph method
works, just delegate. If blocked, leave external_body.

Tier 2 — AugOrderedTableMtEph.rs (2 remaining):
- calculate_reduction: closure requires cascade (same pattern agent3 used
  in AugOrderedTableStEph)
- reduce_range_parallel: fork-join, do NOT sequentialize. If blocked by
  closure specs, leave external_body.

Tier 3 — if time, BSTParaMtEph.rs (Chap38, 8 ext_body) or
BSTParaTreapMtEph.rs (Chap39, 10 ext_body). Same Mt delegation pattern.

**Expected: -10 to -15 holes.**

## Rules (all agents)

- assume() only. NEVER accept().
- Do NOT modify CLAUDE.md.
- Do NOT modify ~/projects/veracity/.
- StEph first, then StPer, then Mt.
- Read existing proved methods in the same file FIRST.
- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agentN-round36-report.md`.
- Commit, push to `agentN/ready`.

## Expected R36 Outcome

-36 to -55 holes. Real actionable: 94 → ~40-58.
