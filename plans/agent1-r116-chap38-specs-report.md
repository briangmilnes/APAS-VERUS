# R116 Agent 1 — Strengthen BSTParaMtEph specs to match StEph

## Result

Verified: 1059 (isolate Chap38), 0 errors. RTT: 3529 passed, 0 skipped.

## Changes Made

### Missing requires added (2 of 3 warnings fixed)

| # | Chap | File | Function | Change |
|---|------|------|----------|--------|
| 1 | 38 | BSTParaMtEph.rs | `insert` | Added `old(self).spec_bstparamteph_wf()` to requires |
| 2 | 38 | BSTParaMtEph.rs | `delete` | Added `old(self).spec_bstparamteph_wf()` and `old(self)@.len() < usize::MAX as nat` to requires |

**`join_pair` requires — no change needed.** The MtEph `join_pair` delegates to `union_inner` which uses `split` internally and does not require disjointness or ordering. StEph's `join_pair` requires those because its impl directly decomposes the `other` tree. The MtEph impl is correct with its current 3 requires.

### Missing ensures added (3 of 4 warnings fixed)

| # | Chap | File | Function | Change |
|---|------|------|----------|--------|
| 1 | 38 | BSTParaMtEph.rs | `reduce` | Added return name `(reduced: T)` and `ensures self@.len() == 0 ==> reduced@ == base@` |
| 2 | 38 | BSTParaMtEph.rs | `in_order` | Added `forall\|v: T::V\| self@.contains(v) <==> seq@.contains(v)` ensures + proof |
| 3 | 38 | BSTParaMtEph.rs | `collect_in_order` (new trait method) | Full containment/preservation ensures matching StEph |

**`join_pair` ensures — false positive confirmed.** MtEph has `joined@ == self@.union(other@)` which is strictly stronger than StEph's `joined@ =~= self@.union(other@)`.

### Missing functions implemented (3)

| # | Chap | File | Function | Notes |
|---|------|------|----------|-------|
| 1 | 38 | BSTParaMtEph.rs | `join_m` | Trivial wrapper around `join_mid`, matching StEph |
| 2 | 38 | BSTParaMtEph.rs | `min_key` | Full ordering proof (delegates to strengthened `min_key_inner`) |
| 3 | 38 | BSTParaMtEph.rs | `collect_in_order` | Trait method delegating to strengthened `collect_in_order_inner` |

### Internal refactoring

- Renamed free fn `min_key` → `min_key_inner`, strengthened ensures to prove ordering property (`min.cmp_spec(&t) == Less || min@ == t@`).
- Renamed free fn `collect_in_order` → `collect_in_order_inner`, strengthened ensures with containment/preservation proofs matching StEph.

## Warnings resolved: 10 of 11

The only unresolved warning is the `join_pair` ensures false positive (`==` vs `=~=`).
