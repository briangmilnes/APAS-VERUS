# R73 Agent 2 — Fix Chap37 BSTAVLMtEph + BSTRBMtEph and their Set wrappers

## Objective

Fix 4 broken Mt modules so they compile, verify, and pass RTT. These were
hidden behind `all_chapters` and never cargo-compiled. They are now commented
out in lib.rs and need to be uncommented once fixed.

## Assigned files

| # | File | Lines | St model |
|---|------|-------|----------|
| 1 | src/Chap37/BSTAVLMtEph.rs | 648 | BSTAVLStEph.rs (1133 lines) |
| 2 | src/Chap37/BSTSetAVLMtEph.rs | 529 | (wraps BSTAVLMtEph) |
| 3 | src/Chap37/BSTRBMtEph.rs | 1136 | BSTRBStEph.rs (681 lines) |
| 4 | src/Chap37/BSTSetRBMtEph.rs | 528 | (wraps BSTRBMtEph) |

## Approach

Each BST Mt module is a top-level coarse-locking wrapper around the St
algorithm. The pattern is established — read these before starting:

1. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — the RwLock pattern
2. `src/standards/mod_standard.rs` — module structure
3. `src/vstdplus/arc_rwlock.rs` — `new_arc_rwlock`, `clone_arc_rwlock` bridges
4. The **St model** file for each module — this is the algorithm source of truth

### For each BST Mt file:

1. Read the St model (BSTAVLStEph.rs or BSTRBStEph.rs).
2. Read the broken Mt file. Identify stale imports and compilation errors.
3. Fix imports to match current module structure.
4. Ensure the Mt module wraps the St algorithm with RwLock per the standard.
5. Specs must match the St version — same ensures, same view.
6. Uncomment the module in src/lib.rs.
7. Run `scripts/validate.sh` to verify.

### For each BSTSet wrapper:

1. Read the existing BSTSet file. It wraps the BST Mt module.
2. Fix imports (the BST Mt module it depends on should now compile).
3. Uncomment in lib.rs.
4. Validate.

### Also look at a working Mt example:

`src/Chap41/AVLTreeSetMtEph.rs` (663 lines) is a working Mt module with
iterators, RwLock, the full pattern. Use it as a reference for what the
finished product should look like.

## Validation

1. `scripts/validate.sh` — must show 0 errors with the new modules uncommented
2. `scripts/rtt.sh` — the RTT files exist in tests/Chap37/, make sure they pass
3. Fix all warnings (triggers, missing requires, etc.)

## Rules

- Read `CLAUDE.md` on startup.
- Do NOT weaken specs. The St version's ensures are the spec.
- Do NOT add assume or accept on algorithmic logic.
- The `assume` inside PartialEq::eq and Clone::clone bodies is the only
  allowed assume pattern.
- Uncomment modules in lib.rs ONLY after they compile and verify.
- Report: files fixed, verification count, any remaining issues.
