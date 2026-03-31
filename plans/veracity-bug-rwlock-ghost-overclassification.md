# Veracity Bug: RWLOCK_GHOST structural_false_positive is too broad

## Problem

`veracity-review-proof-holes` classifies all `assume(...)` inside RwLock
read/write blocks as `info: structural_false_positive RWLOCK_GHOST`. This
suppresses them from the hole count. But not all of these are structural
— some are fixable proof obligations that should be counted as warnings.

## Evidence

Before R116, `AVLTreeSetMtPer.rs` had 25 assumes classified as
`structural_false_positive RWLOCK_GHOST`. Agent1 fixed 10 of them by
lifting `obeys_cmp_spec::<T>()` and `view_ord_consistent::<T>()` to
function requires clauses. These were never structural gaps — they were
missing requires that callers should prove.

The 10 removed assumes:

```
filter:       assume(obeys_cmp_spec::<T>());  assume(view_ord_consistent::<T>());
intersection: assume(obeys_cmp_spec::<T>());  assume(view_ord_consistent::<T>());
difference:   assume(obeys_cmp_spec::<T>());  assume(view_ord_consistent::<T>());
union:        assume(obeys_cmp_spec::<T>());  assume(view_ord_consistent::<T>());
delete:       assume(obeys_cmp_spec::<T>());  assume(view_ord_consistent::<T>());
insert:       (already had requires — no assume to remove)
```

These were not RwLock structural gaps. They were type-level predicates
that belong in requires clauses. The fix was mechanical — add requires,
remove assume.

## What remains (genuinely structural)

The remaining ~15 assumes in AVLTreeSetMtPer ARE structural RwLock gaps:

```
assume(count == self@.len());               // size through lock
assume(seq@.to_set() =~= self@);           // to_seq through lock
assume(filtered@.subset_of(self@));         // filter result through lock
assume(common@ == self@.intersect(other@)); // intersection result through lock
assume(updated@ == self@.insert(x@));       // insert result through lock
...
```

These assume that the inner StPer operation's ensures hold after passing
through the RwLock boundary. They ARE structural because the RwLock
pattern loses the inner function's postconditions.

## The distinction

- **Structural (correct to suppress)**: assumes that bridge inner StPer
  ensures through the RwLock boundary. These can't be fixed without
  changing the RwLock wrapper pattern itself.

- **Non-structural (should be counted)**: assumes about type-level
  predicates (`obeys_cmp_spec`, `view_ord_consistent`, `obeys_feq_full`,
  `obeys_feq_clone`) or function preconditions that should be requires
  clauses. These are fixable by adding requires to the function signature.

## Suggested fix

Classify assumes in RwLock blocks more precisely:

1. If the assume references a *type-level predicate* (obeys_cmp_spec,
   view_ord_consistent, obeys_feq_full, obeys_feq_clone, etc.), it is
   NOT structural — it should be a requires clause. Count as a warning.

2. If the assume references a *result relationship* (result == expr
   involving self@ and args), it IS structural — the RwLock boundary
   drops the inner ensures. Keep as info.

3. If the assume references a *wf predicate* on a return value
   (result.spec_wf()), check whether the function's ensures already
   claims wf. If not, it may be liftable to ensures. Count as warning.

## Scope

This likely affects every Mt module in the project. The `RWLOCK_GHOST`
classification was applied to ~200+ assumes across all Mt files (R114
converted 30 of these to accepts). If even 20% are non-structural, that's
40+ hidden holes.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse ensures/requires
blocks with brace/comma/semicolon awareness. A string-hacking detector will flag
and kill tools that corrupt source syntax.

## Priority

High. This directly affects hole count accuracy and proof targeting. The daily
proof table underreports real holes because of this.
