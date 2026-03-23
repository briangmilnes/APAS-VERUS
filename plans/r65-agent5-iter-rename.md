# R65 Agent 5: Phase 1 — Rename 52 iterative functions to `_iter`

Read these files first:
- `src/standards/iterative_vs_recursive_standard.rs` — the pattern
- `plans/iterative-vs-recursive-plan.md` — the full plan (you're doing Tier 0)
- `plans/iterative-vs-recursive-inventory-v2.md` — the inventory with all 52 functions listed

## Task

Rename 52 MISMATCH-RENAME functions to `_iter` across 6 files. Mechanical, no new proofs.

For each function:
1. Add `fn foo_iter(...)` to the **trait** with the same requires/ensures as `fn foo(...)`.
2. Rename the impl body to `fn foo_iter(...)` (same body, same ensures).
3. The impl's `fn foo(...)` becomes a one-line delegation: `self.foo_iter(x)`.
4. Callers are unaffected.

Doc comments: the `_iter` variant gets `/// Iterative alternative to \`foo\`.`
The default keeps its existing APAS cost spec comment.

## Order

Validate after each file.

1. `src/Chap41/AVLTreeSetStEph.rs` — 7 fns: find, insert, delete, filter, intersection, union, difference
2. `src/Chap41/AVLTreeSetStPer.rs` — 7 fns: same
3. `src/Chap43/OrderedSetStEph.rs` — 8 fns: first, last, previous, next, rank, split, get_range, split_rank
4. `src/Chap43/OrderedSetStPer.rs` — 8 fns: same
5. `src/Chap43/OrderedTableStEph.rs` — 11 fns: find, insert, delete, first_key, last_key, previous_key, next_key, rank_key, split_key, get_key_range, split_rank_key
6. `src/Chap43/OrderedTableStPer.rs` — 11 fns: same

## NOT Renamed

- `from_seq` — MATCH (textbook is iterate insert)
- `select` / `select_key` — MATCH-DIFF-ALG (O(log n) via nth)
- `join` / `join_key` — delegation to union, inherits fix
- `to_seq` — not recursive in textbook

## Constraints

- Do NOT change backing store types. Do NOT add new algorithms.
- Do NOT add `assume`, `accept`, or `external_body`.
- `scripts/validate.sh` after each file. Fix any issues before moving to the next file.
- Run `scripts/rtt.sh` and `scripts/ptt.sh` after all 6 files are done.
- Commit: `R65: Phase 1 rename 52 iterative functions to _iter (Chap41, Chap43)`
- Push to `agent5/ready`.

AFK.
