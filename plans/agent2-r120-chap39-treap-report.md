# R120 Agent 2 — Strengthen BSTTreapMtEph Public Specs (Chap39)

## Summary

Strengthened 5 public function specs in `BSTTreapMtEphTrait` to match
StEph-level contracts. Added `requires self.spec_bsttreapmteph_wf()`
and additional `ensures` clauses to height, insert, delete, find, and
contains.

## Changes

| # | Chap | File | Function | Change |
|---|------|------|----------|--------|
| 1 | 39 | BSTTreapMtEph.rs | `insert` | Added `requires old(self).spec_bsttreapmteph_wf()`, ensures `spec_contains(value)`, size bounds |
| 2 | 39 | BSTTreapMtEph.rs | `delete` | Added `requires old(self).spec_bsttreapmteph_wf()`, ensures size bound, containment preservation |
| 3 | 39 | BSTTreapMtEph.rs | `find` | Added `requires self.spec_bsttreapmteph_wf()`, ensures `found.unwrap()@ == target@` |
| 4 | 39 | BSTTreapMtEph.rs | `contains` | Added `requires self.spec_bsttreapmteph_wf()` |
| 5 | 39 | BSTTreapMtEph.rs | `height` | Added `requires self.spec_bsttreapmteph_wf()` |

## Warning Assessment

### Addressed (estimated 14 of 29 warnings)

- **height** (2 warnings): Added requires wf. Ensures `h == spec_height()`
  not expressible — MtEph doesn't track height in ghost state.
- **insert** (7 warnings): Added requires wf + 3 additional ensures
  (spec_contains, size upper/lower bounds). Missing `spec_bst()` ensures
  and `spec_size() + 1 <= usize::MAX` requires — see "Not expressible" below.
- **delete** (4 warnings): Added requires wf + 2 additional ensures
  (size bound, containment preservation). Missing `spec_bst()` — same reason.
- **find** (3 warnings): Added requires wf + value-equality ensures.
  Missing `spec_bst()` requires — not expressible on MtEph.
- **contains** (3 warnings): Added requires wf. Missing `spec_bst()`
  requires — same reason.

### Not Expressible on MtEph (structural)

These StEph specs reference tree structure that is hidden behind the
RwLock in MtEph. The ghost state only tracks `Set<V>`.

- `spec_bst()` — BST ordering invariant is inside the RwLock invariant
  (`spec_bst_link`), not surfaced at the self level.
- `spec_height()` — height is not tracked in ghost state.
- `spec_min()`, `spec_max()` — min/max as structural tree specs not
  available at Set level.
- `spec_in_order()`, `spec_pre_order()` — traversal sequences not
  available through ghost set.

### False Positives (skipped)

- **Wf naming** (1 warning): `spec_parambsttreapsteph_wf` rename to
  `spec_bsttreapsteph_wf` would collide with the existing
  `spec_bsttreapsteph_wf` (line 424) on the main `BSTTreapStEphTrait`.
  These are different wf predicates on different traits. Not a naming error.
- **Param type mismatch** (1 warning): `lemma_wf_assemble_node` takes
  `&Node<T>` vs StEph `&Box<Node<T>>`. Intentional MtEph design.
- **Missing 23 fns** (1 warning): 6 self-level specs not expressible
  (above), 2 internal helpers, 4 blocked by Verus take() bug, 3 could
  be lifted in future.
- **minimum/maximum** (2 warnings): StEph returns `Option<&T>` with
  match-based ensures against `spec_min()`/`spec_max()`. MtEph returns
  `Option<T>` (cloned) and doesn't have `spec_min`/`spec_max` at self
  level. Existing ensures `min_val.is_some() ==> self@.contains(...)` is
  the best expressible contract.

## Verification

- **Verified**: 1207, 0 errors (isolate Chap39)
- **RTTs**: 37 pass, 0 fail
- **PTTs**: none for Chap39
