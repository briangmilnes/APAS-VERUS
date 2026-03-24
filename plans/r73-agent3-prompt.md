# R73 Agent 3 — Fix Chap37 BSTSplayMtEph + AVLTreeSeq/Set MtPer

## Objective

Fix 4 broken Mt modules so they compile, verify, and pass RTT. These were
hidden behind `all_chapters` and never cargo-compiled. They are now commented
out in lib.rs and need to be uncommented once fixed.

## Assigned files

| # | File | Lines | St model |
|---|------|-------|----------|
| 1 | src/Chap37/BSTSplayMtEph.rs | 1872 | BSTSplayStEph.rs (1787 lines) |
| 2 | src/Chap37/BSTSetSplayMtEph.rs | 531 | (wraps BSTSplayMtEph) |
| 3 | src/Chap37/AVLTreeSeqMtPer.rs | 953 | AVLTreeSeqStPer.rs (1063 lines) |
| 4 | src/Chap41/AVLTreeSetMtPer.rs | 510 | AVLTreeSetStPer.rs (726 lines) |

## Approach

Each Mt module is a top-level coarse-locking wrapper around the St
algorithm. The pattern is established — read these before starting:

1. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — the RwLock pattern
2. `src/standards/mod_standard.rs` — module structure
3. `src/vstdplus/arc_rwlock.rs` — `new_arc_rwlock`, `clone_arc_rwlock` bridges
4. The **St model** file for each module — this is the algorithm source of truth

### Note on BSTSplayMtEph

Splay trees mutate the tree on every access (find, contains, etc.). This
makes the Mt version trickier — reads require write locks. Read
BSTSplayStEph.rs carefully to understand which operations mutate. The Mt
wrapper needs `acquire_write()` even for "read" operations like `find`.

### For each Mt file:

1. Read the St model.
2. Read the broken Mt file. Identify stale imports and compilation errors.
3. Fix imports to match current module structure.
4. Ensure the Mt module wraps the St algorithm with RwLock per the standard.
5. Specs must match the St version — same ensures, same view.
6. Uncomment the module in src/lib.rs.
7. Run `scripts/validate.sh` to verify.

### Working Mt examples for reference:

- `src/Chap41/AVLTreeSetMtEph.rs` (663 lines) — Mt set with RwLock
- `src/Chap18/ArraySeqMtEph.rs` — Mt sequence with RwLock
- `src/Chap37/AVLTreeSeqStPer.rs` — the St model for AVLTreeSeqMtPer

## Validation

1. `scripts/validate.sh` — must show 0 errors with the new modules uncommented
2. `scripts/rtt.sh` — RTT files exist in tests/Chap37/ and tests/Chap41/
3. Fix all warnings (triggers, missing requires, etc.)

## Rules

- Read `CLAUDE.md` on startup.
- Do NOT weaken specs. The St version's ensures are the spec.
- Do NOT add assume or accept on algorithmic logic.
- The `assume` inside PartialEq::eq and Clone::clone bodies is the only
  allowed assume pattern.
- Uncomment modules in lib.rs ONLY after they compile and verify.
- Report: files fixed, verification count, any remaining issues.
