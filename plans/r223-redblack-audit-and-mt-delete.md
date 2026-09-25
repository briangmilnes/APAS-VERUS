# r223: red-black trees — MtEph public delete and full audit (Chap37)

Branch `r223/redblack-audit`, worktree `~/projects/APAS-VERUS-r223`, from main
`c22859345` (r221 and r222 merged). Toolchain: Verus `0.2026.09.13.671956e`,
Z3 4.16.0.

## Problem

r221 left `BSTRBMtEph` without a public `delete`. Layer 1 `delete_link` is
proved, but Layer 2 needs the writer lock-boundary step that `insert` already
uses, `assume(self.ghost_root@ == current)` (RWLOCK_GHOST). The user has now
authorized fixing this, which means adding exactly that one RWLOCK_GHOST
assume in `delete`, with the same form as in `insert`. No other assume may be
added.

The user also asked for a full audit of both red-black files, so that nothing
from r221 is left unchecked.

## Part A: MtEph public delete

1. Add `delete` to `BSTRBMtEphTrait`, with the ensures of the MtEph BB
   `delete`:
   - `Ok` ⇒ the target is absent;
   - the key set is (old keys) − {target};
   - full wf afterwards.
2. Body, as r221 described:
   1. `acquire_write`;
   2. the RWLOCK_GHOST assume;
   3. if `find_link` does not find the key, return `Ok` with the tree unchanged;
   4. color the root red if its left child is black;
   5. `delete_link`;
   6. color the root black;
   7. set the ghost root;
   8. `release_write`.
3. The proof mirrors StEph `delete`. The lock predicate keeps the full wf.
4. Alg Analysis: Work O(lg n), Span O(lg n).
5. MtEph delete RTTs (n = 500), checked after every operation by
   `check_red_black` and `height_within_two_lg`:
   - delete to empty in ascending, descending, and random order;
   - deletes of absent keys;
   - 2000-step random insert/delete interleaving against a `BTreeSet`.
   Check `BSTSetRBMtEph` (if it exists): if it gains delete through the tree,
   add one test there.

## Part B: audit of BSTRBStEph.rs and BSTRBMtEph.rs

For every public and Layer-1 function in both files, record a table row:

- spec strength (strong, partial, weak, or none) against `BSTPlain*` and APAS;
- whether the full wf is required and ensured wherever a tree is taken or
  returned;
- whether the Alg Analysis matches the body;
- holes.

Fix what the audit finds:

- ensures missing wf, key-set, or size facts that the proofs can supply;
- weak `ensures true` on traversals (`in_order`, `pre_order`): give them
  exact sequence ensures, as in BB StEph;
- TOC order and section headers;
- Every quantifier needs an explicit trigger (zero trigger notes); remove any
  `#![auto]`.
- Stale comments.

Also check the following:

1. Independent RTT check: every RB test file must build the tree shape
   from `pre_order()` and check BST order, no red-red, left-leaning, equal
   black height, and height ≤ 2·lg(n+1) after every operation. Fill any gaps.
2. The r221 known warnings (fn_missing_requires on `toggle_color` and
   `update`): report them only. Do not add `requires true` or
   `// veracity: no_requires`.
3. Readers' lock-boundary assumes in MtEph: list them in the report. Do not
   add or remove any.

## Steps

1. Read CLAUDE.md and all of `src/standards/`.
2. Part A; `scripts/validate.sh isolate Chap37` until 0 errors, 0 warnings,
   0 trigger notes.
3. Part B audit table, then its fixes; isolate validate after each change.
4. Filtered RTTs: `scripts/rtt.sh RB`.
5. Seeds 1-8 on each changed module (whole module per seed).
6. After r224's background agent finishes, run the full `scripts/validate.sh`,
   `scripts/rtt.sh`, and `scripts/ptt.sh` sequentially. They must not overlap
   with the agent's runs.
7. `scripts/holes.sh src/Chap37/`: the only new hole is the one RWLOCK_GHOST
   assume in MtEph `delete`.
8. Report `plans/r223-redblack-audit-and-mt-delete-report.md`, with `#` and
   `Chap` columns in every table.
9. Commit on the branch with `git add -A`; push. Merge only when the user asks.

## Constraints

- No assume, admit, accept, or external_body other than the authorized
  RWLOCK_GHOST assume in MtEph `delete`.
- No rlimit raised; no spec weakened; no `#![auto]`; no formatters.
- No `// veracity: no_requires`.
- Run validate, rtt, and ptt sequentially.

## Success criteria

- MtEph has a public `delete` with the full spec, and its tests pass.
- The audit table has one row per function in both files, and every finding
  is fixed or explained.
- Full crate 0 errors; PTT 328 of 328; RTT failures limited to Chap64 if r224
  has not been merged yet.
