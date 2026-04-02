# R137 Agent 4 — Fix 11 OrderedTable in_order bugs: O(n) → O(lg n). AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `prompts/Chap43.txt` — APAS tells you exactly how each operation should work.
Read your own R136 audit: `plans/r136-agent4-chap43-in-order-audit-report.md`.
Read `src/Chap38/BSTParaStEph.rs` for the BST operations available (find, min_key,
split, expose, size).

Report file: `plans/r137-agent4-chap43-fix-report.md`

## Problem

11 operations in OrderedTableStEph and OrderedTableStPer use `in_order()` + linear
scan where they should use BST operations directly. APAS CS 43.2 says all operations
are O(lg n). Your R136 audit identified them all.

## Prerequisite: add max_key to BSTParaStEph

`src/Chap38/BSTParaStEph.rs` has `min_key` (traverse left branches) but no `max_key`.
The textbook says: "last need only to traverse right branches."

Add `max_key` to the `ParamBSTTrait` in BSTParaStEph.rs — mirror of `min_key`,
traverse right instead of left. Same ensures pattern:
```rust
fn max_key(&self) -> (maximum: Option<T>)
    ensures
        self@.len() == 0 ==> maximum.is_none(),
        maximum.is_some() ==> self@.contains(maximum.unwrap()@),
        maximum.is_some() ==> forall|x: T| self@.contains(x@) ==> x <= maximum.unwrap();
```

This unblocks last_key, previous_key.

## The 11 fixes (each in BOTH StEph and StPer)

Fix these in `src/Chap43/OrderedTableStEph.rs` and `src/Chap43/OrderedTableStPer.rs`.
Agent 1 is fixing `bst_find_by_key` — do NOT touch that function. Fix the other 10.

### No blockers (fix now):

| # | Function | Current | Fix (from APAS) |
|---|----------|---------|-----------------|
| 1 | first_key_iter | in_order + scan for min | Call `self.tree.min_key()` → extract key from Pair |
| 2 | last_key_iter | in_order + scan for max | Call `self.tree.max_key()` → extract key (needs max_key above) |
| 3 | next_key_iter | in_order + scan forward | `self.tree.split(k)` → right half → `right.min_key()` |
| 4 | previous_key_iter | in_order + scan backward | `self.tree.split(k)` → left half → `left.max_key()` |
| 5 | split_key_iter | in_order + rebuild two trees | Call `self.tree.split(k)` directly, wrap halves in OrderedTable |
| 6 | get_key_range_iter | in_order + scan range | Two splits: `split(k1)` then `split(k2)` on the right half |

### Need rank/select (fix if ParamBST has size-based ops):

| # | Function | Current | Fix (from APAS) |
|---|----------|---------|-----------------|
| 7 | rank_key_iter | in_order + count | Walk BST using `NodeInner.size` — count left subtrees |
| 8 | select_key | in_order + index | Walk BST using `NodeInner.size` — descend by rank |
| 9 | split_rank_key_iter | in_order + rebuild | `select(rank)` → `split(selected_key)` |

For 7-9: `ParamBST` stores `NodeInner.size` already. Write helper functions
`bst_rank` and `bst_select` that use the size field to walk the tree in O(lg n).
These go in `OrderedTableStEph.rs` as free functions (not on ParamBST itself —
they're Pair-aware, keyed by K not Pair<K,V>).

Check if `src/Chap40/BSTSizeStEph.rs` has rank/select implementations you can
reference for the algorithm. It uses `Link<T>` not `ParamBST<T>` but the algorithm
is the same.

## Implementation pattern

For split-based operations, the key issue is that `ParamBST::split` takes `&T`
where `T = Pair<K,V>`. To split by key K, you need a dummy Pair. Since Pair's Ord
compares only keys, `Pair(k, any_v)` finds the right split point. If constructing
a dummy V is hard, write a key-only split helper using `expose()` + recursive descent.

## Proof pattern

Each fix replaces a sequential in_order + scan with a BST operation. The ensures
must match the existing ones (don't weaken). The BST operations' ensures (from
Chap38) should provide what you need — min_key returns the minimum, split produces
correct halves, etc.

## Validation

Run `scripts/validate.sh isolate Chap43`. Then `scripts/rtt.sh`.

## Rules

- Do NOT touch `bst_find_by_key` — agent 1 is fixing that.
- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken ensures.
- Every fix must be O(lg n). No O(n) workarounds.
- Update alg analysis annotations from O(n) to O(lg n) for each fixed function.

## When done

RCP.
