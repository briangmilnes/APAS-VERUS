# R75 Agent 4 — Prove Chap37 BSTRBMtEph + BSTSplayMtEph (14 holes)

## Objective

Prove or eliminate 14 holes across 2 files in Chap37 — the multi-threaded Red-Black and
Splay tree implementations.

## Files and holes

| # | Chap | File | Holes | Root causes |
|---|------|------|-------|-------------|
| 1 | 37 | BSTRBMtEph.rs | 8 | 5 root, 2 downstream + 1 assume |
| 2 | 37 | BSTSplayMtEph.rs | 6 | 2 root, 3 downstream + 1 assume |

### BSTRBMtEph.rs (8 holes — 7 external_body + 1 assume)

Root causes (5):
- `rotate_left()` — line ~186 — external_body (mut rotation)
- `rotate_right()` — line ~270 — external_body (mut rotation)
- `flip_colors()` — line ~354 — external_body (mut color flip)
- `filter_parallel()` — line ~747 — external_body (Fn through Arc)
- `reduce_parallel()` — line ~772 — external_body (Fn through Arc)

Downstream (2):
- `fix_up()` — line ~381 — external_body (blocked by flip_colors)
- `insert_link()` — line ~423 — external_body (blocked by fix_up)

Assume (1):
- `height()` — line ~1003 — assume (link_height < usize::MAX — needs RB balance lemma)

**Mut rotation strategy**: The rotations use `&mut Link<T>` (Option<Box<Node<T>>>). The
R74 Agent 4 report says these are blocked by Verus `Option::as_mut()` limitation — Verus
doesn't support `as_mut()` on `Option<Box<_>>` for `&mut` extraction. Explore alternatives:

1. **Pattern match**: Instead of `option.as_mut().unwrap()`, use `if let Some(ref mut node) = *link { ... }`.
2. **Take-modify-put**: `let mut node = link.take().unwrap(); ... *link = Some(node);`
3. **Search experiments**: Check `src/experiments/` for any `&mut Option<Box<>>` tests.
4. **Check other Chap37 files**: BSTAVLMtEph.rs is clean — see how it handles rotations.

**filter_parallel / reduce_parallel**: These pass `Fn` through `Arc`. The challenge is that
Verus can't track closure specs through `Arc<dyn Fn(...)>`. Check if there's a way to use
the closure standard's named-closure pattern with thread spawning.

**height assume**: Proving `link_height(link) < usize::MAX` requires a lemma that Red-Black
tree height is O(log n) and n < usize::MAX implies height < usize::MAX. This is a
mathematical lemma about RB tree balance. Check if a similar lemma exists for AVL in
BSTAVLMtEph.rs.

### BSTSplayMtEph.rs (6 holes — 5 external_body + 1 assume)

Root causes (2):
- `splay()` — line ~149 — external_body (mut rotations)
- `clone()` — line ~1802 — external_body (Clone impl for tree)

Downstream (3):
- `build_balanced()` — line ~1454 — external_body (blocked by clone)
- `filter_parallel()` — line ~1480 — external_body (blocked by clone)
- `reduce_parallel()` — line ~1513 — external_body (blocked by clone)

Assume (1):
- `height()` — line ~1730 — assume (link_height < usize::MAX — same issue as RB)

**Splay strategy**: Splay uses zig-zig, zig-zag rotations on `&mut` tree. Same
`Option::as_mut()` issue as RB rotations. Try the same workaround patterns.

**Clone strategy**: The `clone` function deep-copies the tree. Check how other Chap37
Clone impls work (BSTAVLMtEph has clean Clone). The pattern from
`partial_eq_eq_clone_standard.rs` requires `assume` inside the clone body — that's
acceptable per project rules.

## Key resources

- `src/Chap37/BSTRBMtEph.rs`, `src/Chap37/BSTSplayMtEph.rs`
- `src/Chap37/BSTAVLMtEph.rs` — clean reference for rotations and Clone
- `src/standards/partial_eq_eq_clone_standard.rs` — Clone workaround
- `src/standards/using_closures_standard.rs` — closure patterns
- `src/standards/mut_standard.rs` — &mut patterns in Verus
- `src/experiments/` — check for &mut Option<Box<>> experiments

## Approach

1. Read BSTAVLMtEph.rs to see how it handles rotations cleanly. This is your reference.
2. Read BSTRBMtEph.rs rotation functions — understand the current external_body bodies.
3. Try take-modify-put pattern for rotations: extract node, modify, put back.
4. If rotations prove, fix_up and insert_link should cascade.
5. For BSTSplayMtEph clone, apply the standard Clone workaround pattern.
6. For height assume, attempt an O(log n) height lemma or leave with documentation.

## Validation

Run `scripts/validate.sh` after each file change. Run `scripts/rtt.sh` and `scripts/ptt.sh`
before committing. Push to `agent4/ready`.

## Report

Write `plans/agent4-round75-report.md` with holes before/after per file (table with Chap column).
