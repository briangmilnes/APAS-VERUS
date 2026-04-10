# Veracity Bug: Marker-shift corruption persists on MtEph files with RwLock

## The fix didn't work for MtEph files

The R175 veracity rebuild fixed marker-shift for StEph files (Johnson, Dijkstra,
BSTRBMtEph, PQMinStPer — all pass RTT). But MtEph files with RwLock threading
still produce deadlocks after minimize.

## Reproduction

```bash
cd ~/projects/APAS-VERUS-agent3
~/projects/veracity/target/release/veracity-minimize-proofs \
  -c . -l src/vstdplus --project APAS --chapter Chap43 \
  -F src/Chap43/OrderedTableMtEph.rs \
  -a -p --no-lib-min --danger \
  --max-incremental 0.00 --max-memory-increase 0.00
```

After minimize: 14 asserts + 23 proof blocks removed. Verus verification passes
(5598 verified, 0 errors). But RTT deadlocks:

```
SIGTERM [117s] TestOrderedTableMtEph test_ordered_table_mt_eph_empty_operations
SIGTERM [117s] TestOrderedTableMtEph test_ordered_table_mt_eph_difference
SIGTERM [117s] TestOrderedTableMtEph test_ordered_table_mt_eph_difference_empty
SIGTERM [117s] TestOrderedTableMtEph test_ordered_table_mt_eph_restrict
SIGTERM [119s] TestAugOrderedTableMtEph test_difference_operation
SIGTERM [119s] TestAugOrderedTableMtEph test_restrict_operation
```

These tests pass on the pre-minimize version of the same files.

## Affected files (R175)

| # | File | Items removed | RTT result |
|---|------|--------------|------------|
| 1 | OrderedTableMtEph.rs | 14 asserts + 23 proof blocks | **6 SIGTERM deadlocks** |
| 2 | AugOrderedTableMtEph.rs | 0 (all skipped, prior markers) | **deadlock via OrderedTableMtEph dep** |

## Files that work fine with the fix

| # | File | Items removed | RTT result |
|---|------|--------------|------------|
| 1 | BSTRBMtEph.rs | 0 | 3776 passed |
| 2 | PQMinStPer.rs | 0 | 3776 passed |
| 3 | DijkstraStEphF64.rs | 0 | 3776 passed |
| 4 | DijkstraStEphU64.rs | 0 | 3776 passed |
| 5 | JohnsonStEphF64.rs | 2 proof blocks | 3776 passed |
| 6 | JohnsonStEphI64.rs | 3 proof blocks | 3776 passed |
| 7 | JohnsonMtEphF64.rs | 0 (NEEDED only) | 3776 passed |
| 8 | JohnsonMtEphI64.rs | 0 (NEEDED only) | 3776 passed |
| 9 | AugOrderedTableStEph.rs | 1 proof block | 3776 passed |

## What's different about OrderedTableMtEph

The failing file uses coarse-grained RwLock for thread safety:
- `RwLock<OrdKeyMap<...>>` as the inner data structure
- `read()` / `write()` lock acquisition in every operation
- `proof { }` blocks interleaved with lock acquire/release
- Ghost state (`Ghost`, `Tracked`) alongside the lock

The deadlocking tests (`difference`, `restrict`, `empty_operations`) involve
operations that acquire locks on two OrderedTable instances. If marker insertion
reorders exec code relative to lock acquire/release, the lock ordering can change,
causing deadlock.

## Root cause hypothesis

The NEEDED marker insertion is still displacing exec code in files where `proof {}`
blocks are interleaved with lock operations (`self.inner.write()`, `other.inner.read()`).
The StEph fix works because StEph files have no locks — code displacement there
changes ghost bindings but not exec-visible behavior. In MtEph files, the displacement
changes the position of lock acquire/release relative to other operations.

## What to check

1. Diff `OrderedTableMtEph.rs` before and after minimize — are any `let` bindings
   (especially lock acquisitions) displaced relative to `proof {}` blocks?
2. Check if any NEEDED marker landed between a `write()`/`read()` call and its
   paired proof block.
3. The `difference` and `restrict` operations acquire locks on both `self` and `other`.
   Check if the lock ordering changed.

## The pre-minimize and post-minimize files

Pre-minimize (clean): `git show 45d08a8d4:src/Chap43/OrderedTableMtEph.rs`
Post-minimize (deadlocks): in agent3 worktree at
`~/projects/APAS-VERUS-agent3/src/Chap43/OrderedTableMtEph.rs`

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse ensures/requires
blocks with brace/comma/semicolon awareness. A string-hacking detector will flag
and kill tools that corrupt source syntax.
