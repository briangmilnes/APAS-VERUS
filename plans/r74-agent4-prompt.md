# R74 Agent 4 — Prove Chap37 BSTRBMtEph assumes + BSTSplayMtEph (21 holes)

## Objective

Prove or eliminate 21 holes across 2 files:

### BSTRBMtEph.rs — 20 holes (10 assume + 10 external_body)

**10 assumes** (all algorithmic, in trait impl starting at line ~933):
These are rwlock-ghost assumes in the trait impl methods (contains, size, is_empty, height,
in_order, pre_order, from_sorted_slice, filter, reduce, delete). Each assumes a property
about `*data` (the value read from the RwLock):
- `spec_is_bst_link(*data)` — 4 occurrences
- `link_spec_size(*data) <= usize::MAX` — 5 occurrences
- `link_height(*data) < usize::MAX` — 1 occurrence

The underlying StEph functions (BSTRBStEph) already prove these properties. The Mt wrapper
needs to propagate them through the RwLock invariant.

**10 external_body** (free functions, lines ~169-769):
- `flip_colors` (line ~169) — root cause, mutable node manipulation
- `rotate_left` (line ~253) — root cause, mutable rotation
- `rotate_right` (line ~337) — root cause, mutable rotation
- `fix_up` (line ~364) — blocked by flip_colors
- `insert_link` (line ~406) — blocked by fix_up
- `in_order_parallel` (line ~686) — sequential, takes &Link
- `pre_order_parallel` (line ~705) — sequential, takes &Link
- `build_balanced` (line ~725) — sequential, takes &[T]
- `filter_parallel` (line ~744) — sequential, takes &Link
- `reduce_parallel` (line ~769) — sequential, takes &Link

The last 5 are sequential free functions that Verus could verify if we remove
external_body — they don't use ParaPair. Focus on these first (easy wins), then
tackle the rotation functions.

### BSTSplayMtEph.rs — 8 holes (1 assume + 7 external_body)

**1 assume**: `link_height(*data) < usize::MAX` in `height` (line ~1762).

**7 external_body**: Splay tree operations. Read BSTSplayStEph.rs (0 holes) as the model.

**IMPORTANT**: BSTSplayStEph.rs has 5 functions with `// veracity: no_requires` annotations.
Do NOT add requires to splay helpers — it destabilizes SMT proofs. See CLAUDE.md.

## Assigned files

| # | File | Holes |
|---|------|-------|
| 1 | src/Chap37/BSTRBMtEph.rs | 10 assume + 10 external_body |
| 2 | src/Chap37/BSTSplayMtEph.rs | 1 assume + 7 external_body |

## Validation

```bash
scripts/validate.sh    # must pass: 4735+ verified, 0 errors
scripts/rtt.sh         # must pass: 2619+ tests
```

Fix all warnings in your assigned files before committing.

## Required reading (before writing any code)

1. `CLAUDE.md` — project rules.
2. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — RwLock Mt wrapper pattern.
3. `src/standards/partial_eq_eq_clone_standard.rs` — eq/clone assume pattern (ONLY in eq/clone bodies).
4. `src/Chap37/BSTRBStEph.rs` — the St counterpart for BSTRBMtEph. Read this FIRST to
   understand what ensures the St functions provide. Your Mt wrapper must use those ensures
   to prove its own postconditions (spec_is_bst_link, link_spec_size bounds, etc.).
5. `src/Chap37/BSTSplayStEph.rs` — the St counterpart for BSTSplayMtEph. Note the 5
   functions with `// veracity: no_requires` — do NOT add requires to splay helpers.

## Rules

- Do NOT weaken ensures. Do NOT add `accept()`.
- Do NOT sequentialize parallel code. Mt files must remain parallel.
- Commit to your branch, push to `origin/agent4/ready`.
- Write report to `plans/agent4-round74-report.md`.
