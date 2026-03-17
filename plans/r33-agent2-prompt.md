R33: Chap43 trivial fixes + OrderedSetStEph delegation wrapper pilot.

TASK 1 — Add missing wf ensures (2 trivial holes).

(a) src/Chap43/OrderedTableMtEph.rs — `from_sorted_entries` (line ~887).
    Add `ensures constructed.spec_orderedtablemteph_wf()` to the
    ensures clause. The function constructs a well-formed table —
    the proof should follow from construction.

(b) src/Chap43/OrderedTableMtPer.rs — `from_st_table` (line ~73).
    Add `ensures s.spec_orderedtablemtper_wf()` to the ensures clause.

TASK 2 — Prove delegation wrappers in OrderedSetStEph.rs.

OrderedSetStEph wraps AVLTreeSetStEph. Each trait method calls the
corresponding AVLTreeSetStEph method. The external_body is unnecessary
because the base method's ensures clause directly satisfies the
wrapper's ensures clause.

Pattern for each function:
1. Read the external_body function.
2. Remove `#[verifier::external_body]`.
3. The function body already calls the base operation.
4. If the ensures don't verify automatically, add proof assertions
   connecting the base ensures to the wrapper ensures.

Target these functions (read the file to find them):
- `first`, `last`, `previous`, `next_key`, `find`
- `rank`, `select`, `size`
- `delete`, `insert`
- `from_seq`, `to_seq`

Some functions may need intermediate assertions if the spec
transformation is non-trivial (e.g., set containment vs ordered
set containment). Focus on the ones that are straightforward
delegation first.

TASK 3 — Fix fn_missing_requires in Chap43 files.

(a) src/Chap43/OrderedSetStEph.rs — fn_missing_requires (1).
(b) src/Chap43/OrderedSetStPer.rs — fn_missing_requires (1).
(c) src/Chap43/AugOrderedTableStPer.rs — fn_missing_requires (1).
(d) src/Chap43/AugOrderedTableMtEph.rs — fn_missing_requires (1).

Read each function, add the real precondition.

Do NOT add assume, accept, or external_body.
Do NOT add `requires true` or tautological requires.
Every quantifier must have explicit #[trigger].
Run scripts/validate.sh after changes — 0 errors required.
