# veracity-compare-par-mut Phase 3 — Requires/Ensures Clause Text Comparison

## Current State

Phase 2 compares presence of requires/ensures across variants:
```
warning: `find`: StEph has requires but MtEph does not
warning: `singleton`: StPer has requires but MtEph does not
```

It detects MISSING clauses but not MISMATCHED clauses. If both variants have
requires, Phase 2 says nothing — even if the clauses are completely different.

## Phase 3 Objective

Compare the actual requires/ensures clause TEXT between matched functions
across variants. Report when clauses differ beyond expected variant substitution
(StEph→MtEph, StPer→MtPer, etc.).

## Expected output format

```
src/Chap42/TableMtEph.rs:555: warning: `singleton` requires mismatch:
  StPer: requires obeys_feq_clone::<Pair<K, V>>()
  MtEph: (none)

src/Chap41/AVLTreeSetMtPer.rs:167: warning: `insert` requires mismatch:
  StPer: requires self.spec_avltreesetstper_wf(), self@.len() + 1 < usize::MAX as nat
  MtPer: (none)

src/Chap37/BSTAVLMtEph.rs:200: info: `insert` requires equivalent after substitution:
  StEph: requires self.spec_bstavlsteph_wf()
  MtEph: requires self.spec_bstavlmteph_wf()
```

## Substitution rules

When comparing clause text, apply these substitutions before diffing:
- `StEph` → variant name (`MtEph`, `StPer`, `MtPer`)
- `steph` → variant suffix (`mteph`, `stper`, `mtper`)
- Module-specific wf name: `spec_foosteph_wf` → `spec_foomteph_wf`
- Type names: `TableStEph` → `TableMtEph` (variant swap on the concrete type)
- `old(self)` → `self` when comparing Eph→Per (Eph mutates, Per returns new)
- Generic bounds: `StT` → `StTInMtT` or `MtKey`/`MtVal` (known substitutions)

After substitution, if the clauses are structurally identical → `info: equivalent`
If they differ → `warning: mismatch` with both clauses shown

## What to parse

For each matched function pair (already identified in Phase 2):
1. Extract the `requires` block text (everything between `requires` and the next
   keyword: `ensures`, `decreases`, `{`, or end of signature)
2. Extract the `ensures` block text (same approach)
3. Normalize whitespace, apply substitutions
4. Compare. Report mismatches.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse ensures/requires
blocks with brace/comma/semicolon awareness. A string-hacking detector will flag
and kill tools that corrupt source syntax.

## Clause normalization before comparison

Requires/ensures clauses are comma-separated conjuncts. Before comparing:

1. **Split on `,`** — but respect nesting (parens, angle brackets, braces).
   `requires foo(a, b), bar(c)` splits into `["foo(a, b)", "bar(c)"]`, not
   `["foo(a", "b)", "bar(c)"]`.
2. **Trim whitespace** on each conjunct.
3. **Sort alphabetically** — clause order is not semantically meaningful.
4. **Apply variant substitutions** (see substitution rules above).
5. **Compare sorted sets** — if the sets are equal, the clauses are equivalent.
   If they differ, report the missing/extra conjuncts on each side.

This eliminates false positives from different ordering of the same conjuncts.

Example:
```
StEph: requires self.spec_tablesteph_wf(), obeys_view_eq::<K>()
MtEph: requires obeys_view_eq::<K>(), self.spec_tablemteph_wf()
```
After substitution + sort:
```
StEph: ["obeys_view_eq::<K>()", "self.spec_tablesteph_wf()"]
MtEph: ["obeys_view_eq::<K>()", "self.spec_tablemteph_wf()"]
```
After variant substitution (`steph` → `mteph`): **equivalent**.

## What NOT to compare

- Return types (already compared in Phase 2)
- Generic bounds (already compared in Phase 2)
- Function body content (out of scope)
- Spec function bodies (Phase 4)

## Testing

Run against APAS-VERUS. The 72 existing warnings include many "has requires but
other does not" — Phase 3 should catch additional mismatches where BOTH variants
have requires/ensures but the clauses differ.

Expected new findings: MtEph/MtPer variants often have weaker requires/ensures
than their St counterparts (e.g., missing wf, missing capacity bounds). These
are real conformance gaps that should be reported as warnings.
