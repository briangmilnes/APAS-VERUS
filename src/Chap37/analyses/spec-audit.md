# Chap37 Spec Audit — Round 17

## AVLTreeSeqStEph.rs

| # | Function | Old ensures | New ensures | Status |
|---|----------|-------------|-------------|--------|
| 1 | `set` | (none) | `outcome is Ok, wf, seq =~= old(seq).update(i, item@)` | external_body |
| 2 | `update` | (none) | `wf, seq =~= old(seq).update(i, item@)` | Verified (from set) |
| 3 | `singleton` | `len == 1, wf` | `len == 1, seq[0] == item@, wf` | Verified |
| 4 | `insert_value` | `seq =~= old(seq).push(value@)` | Added `wf` | Verified |

All other trait functions had correct specs matching the prose.

## AVLTreeSeqStPer.rs

| # | Function | Old ensures | New ensures | Status |
|---|----------|-------------|-------------|--------|
| 1 | `set` | (none) | `outcome is Ok, wf, seq =~= self.seq.update(i, item@)` | external_body |
| 2 | `subseq_copy` | (none) | `wf` | external_body |
| 3 | `values_in_order` | (none) | `values@.map_values(\|t\| t@) =~= self.spec_seq()` | external_body |
| 4 | `to_arrayseq` | (none) | `seq.spec_len() == self.spec_seq().len()` | external_body |

StPer is weaker than StEph overall due to Arc path-copying complexity.

## BSTSplayStEph.rs

| # | Function | Old ensures | New ensures | Status |
|---|----------|-------------|-------------|--------|
| 1 | `insert` | `wf` only | `wf, contains(value), preserves contains` | external_body |
| 2 | `find` | one-directional | `some <==> contains(target), value match` | external_body |
| 3 | `contains` | `true` | `found == contains(target)` | Verified (from find) |
| 4 | `minimum` | contains only | Added `forall\|x\| contains(x) ==> le(min, x)` | external_body |
| 5 | `maximum` | contains only | Added `forall\|x\| contains(x) ==> le(x, max)` | external_body |
| 6 | `in_order` | `true` | `seq.spec_len() == spec_in_order().len()` | external_body |
| 7 | `pre_order` | `true` | `seq.spec_len() == spec_pre_order().len()` | external_body |

Added spec fns: `spec_in_order_link`, `spec_pre_order_link`, `spec_in_order`, `spec_pre_order`.
Note: `spec_bstsplaysteph_wf` is still `true` — BST invariant preservation through splay
rotations is a separate proof obligation.
