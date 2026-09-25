# r227 report: BSTSetRBMtEph delete uses the red-black tree's delete (Chap37)

Branch `r227/rbset-delete`, from main `ec039f497`. The user approved this
change (r223 report, decision 4).

| # | Chap | File | Change |
|---|---|---|---|
| 1 | 37 | BSTSetRBMtEph.rs | `delete` calls `BSTRBMtEph::delete` instead of rebuilding |
| 2 | 37 | BSTSetRBMtEph.rs | Alg Analysis: O(n) rebuild → Work O(lg n), Span O(lg n) |
| 3 | 37 | TestBSTSetRBMtEph.rs | new `test_delete_keeps_red_black_shape` (n = 300) |

`delete` used to test membership, copy the in-order traversal minus the target
into a Vec, and rebuild a balanced tree from it: Work O(n). It now makes one
call to the tree's verified `delete` (r223). The spec is unchanged: requires
and ensures the set's wf. `rebuild_from_vec` is still used by the set's
constructors.

| # | Chap | Measure | Value |
|---|---|---|---|
| 1 | 37 | `validate.sh isolate Chap37` | 2062 verified, 0 errors |
| 2 | 37 | Seeds 1-8, BSTSetRBMtEph alone | 8 of 8, 34 verified, 0 errors |
| 3 | 37 | TestBSTSetRBMtEph binary | 43 of 43 pass |
| 4 | 37 | holes.sh BSTSetRBMtEph.rs | 0 holes |

Finding, not changed: `BSTSetRBMtEph` has no `View`, and most of its trait
functions ensure only wf or `true` (for example, `contains` ensures `true`).
Strengthening the set's specs is a separate round.
