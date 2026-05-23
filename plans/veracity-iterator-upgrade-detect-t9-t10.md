# Plan — `veracity-iterator-upgrade --detect` T9 / T10

Status: SCOPING. Author: 2026-05-23.

Two new transform classes that absorb the easy U-OTHER clusters
identified in the Phase 1.5 report (§5). Together they convert
~50 U-OTHER findings into T-findings — a ~20% reduction in the
human-attention list before Phase 2 starts.

Companion: `plans/veracity-iterator-upgrade-detect.md` §5 (T1–T8) and
§13a (the gap list this plan extends).

## 1. Why these two

The top 5 U-OTHER patterns from the Phase 1.5 sweep all reduce
mechanically:

| # | Skeleton | Count | Reducible? |
|--:|:---------|------:|:-----------|
| 1 | `it@.0 <= <ident>.<ident> ()` | 23 | yes — `it.index() <= …` |
| 2 | `it@.1.<ident> ()` | 16 | yes — `it.seq().<ident>()` |
| 3 | `it@.1.<ident> (\|i,k\| k@).<ident> () == self@.<ident>` | 8 | yes — same substitution |
| 4 | `<ident> == it@.1` | 7 | yes — `<ident> == it.seq()` |
| 5 | `it@.1 =~= self.<ident> ()` | 5 | yes — `it.seq() =~= …` |

Every reducible case is one of two substitutions:
**`it@.0` → `it.index()`** or **`it@.1` → `it.seq()`**. T1–T8 each
match a single SHAPE (binary equality with one side a literal, or a
specific method call); they don't fire on the looser
`it@.<n>`-as-sub-expression patterns above. T9 and T10 do.

## 2. T9 — index-substitution

### Definition

**Fires when:** an expression contains `it@.0` as a sub-expression
AND it has not already been matched by T1 (`it@.0 == <intlit>`),
T6 (`<expr>.len() - it@.0` in `decreases`), or T7
(`it@.0 == <expr>.len()`).

**New form:** the same expression with every occurrence of `it@.0`
replaced by `it.index()`.

### AST matcher

After running T1–T8 against the current clause:

1. Walk the expression tree.
2. If any sub-node matches `Expr::Field { base: Expr::View { expr:
   Path("it") }, member: Member::Unnamed(0) }`, the expression is a
   T9 candidate.
3. Emit a single T9 finding at the clause's line/col, with `new_text`
   = the clause re-rendered after substituting every `it@.0`
   subexpression with the path `it.index()`.

Substitution is done at the AST level (clone the `Expr`, walk
mutably, replace matching nodes with a synthesized
`MethodCall { receiver: Path("it"), method: "index", args: [] }`).
No string regex.

### Examples (verbatim from the Phase 1.5 sweep)

| # | Old form | New form (T9 emits) |
|--:|:---------|:--------------------|
| 1 | `it@.0 <= seq@.len()` | `it.index() <= seq@.len()` |
| 2 | `forall\|i: int\| 0 <= i < it@.0 ==> P[i]` | `forall\|i: int\| 0 <= i < it.index() ==> P[i]` |
| 3 | `it@.0 <= it@.1.len()` | `it.index() <= it.seq().len()` (with T10 also firing — see §4) |

### Out of scope

- `it@.0`-bearing expressions in `requires` clauses where `it` is a
  bound parameter (not a loop iterator). Per plan §5 the matcher
  recognizes the literal name `it` regardless of binding context;
  T9 inherits that.
- Expressions where `it@.0` is on the LHS of an arithmetic op that
  changes semantics (e.g., `it@.0 + 1`). The substitution is purely
  syntactic; semantic equivalence is the reviewer's call. The new
  text shown is the mechanical substitution; if Verus rejects it,
  the human picks up the slack.

## 3. T10 — seq-substitution

### Definition

**Fires when:** an expression contains `it@.1` as a sub-expression
AND it has not already been matched by T2 (`it@.1 == self.seq@`),
T3 (`it@.1 == <expr>` at top level), or T5
(`it@.0 < it@.1.len()`).

**New form:** the same expression with every occurrence of `it@.1`
replaced by `it.seq()`.

### AST matcher

Symmetric with T9:

1. After T1–T8 + T9, walk the expression tree.
2. If any sub-node matches `Expr::Field { base: Expr::View { expr:
   Path("it") }, member: Member::Unnamed(1) }`, emit T10.
3. Substitution: replace each `it@.1` subexpression with
   `MethodCall { receiver: Path("it"), method: "seq", args: [] }`.

### Examples (verbatim from the Phase 1.5 sweep)

| # | Old form | New form (T10 emits) |
|--:|:---------|:---------------------|
| 1 | `it@.1.no_duplicates()` | `it.seq().no_duplicates()` |
| 2 | `it@.1.map(\|i,k\| k@).to_set() == self@.dom()` | `it.seq().map(\|i,k\| k@).to_set() == self@.dom()` |
| 3 | `<expr> == it@.1` | `<expr> == it.seq()` |
| 4 | `it@.1 =~= self.spec_seq()` | `it.seq() =~= self.spec_seq()` |
| 5 | `forall\|i\| 0 <= i < n ==> it@.1[i] == X[i]` | `forall\|i\| 0 <= i < n ==> it.seq()[i] == X[i]` |
| 6 | `it@.1.len() == <expr>` | `it.seq().len() == <expr>` |

## 4. Priority order with existing T-classes

The matcher runs T-classes in priority order; first match wins. Add
T9 and T10 at the END of the priority chain (after T1–T8) so the
narrow shape-specific matchers keep their precedence.

Priority order (full):

| # | Class | Context guard | Match shape |
|--:|:------|:--------------|:------------|
|  1 | T8 | fn `ensures` | the constructor triple (`it@.0 + it@.1 + iter_invariant(&it)`) |
|  2 | T6 | `decreases` | `<expr>.len() - it@.0` |
|  3 | T5 | any | `it@.0 < it@.1.len()` |
|  4 | T1 | any | `it@.0 == <intlit>` |
|  5 | T7 | any | `it@.0 == <expr>.len()` |
|  6 | T2 | any | `it@.1 == self.seq@` |
|  7 | T3 | any | `it@.1 == <expr>` (top level) |
|  8 | T4 | any | `iter_invariant(&it)` |
|  9 | **T9** | any | expression contains `it@.0` as sub-expression |
| 10 | **T10** | any | expression contains `it@.1` as sub-expression |
| 11 | U-OTHER | any | expression mentions `it` but nothing above matched |

### When T9 and T10 both apply

An expression like `it@.0 <= it@.1.len()` (T5 doesn't match because
the comparison is `<=`, not `<`) contains both `it@.0` and `it@.1`.

**Choice (a) — two findings:** T9 fires, then T10 fires
independently. The compile log shows two `info:` lines at the same
line/col with different new-text. Reviewer applies both
substitutions in sequence.

**Choice (b) — one finding, both substitutions:** when T10 fires
and the same expression also contains `it@.0`, the new-text shown
applies BOTH substitutions. T9 does not fire separately for the
same expression. One finding, both rewrites in the new text.

**Recommendation: (b).** Less noise in the report; the reviewer's
mental model is "this clause migrates to that clause" not "this
clause has two independent substitutions." Implementation: when T10
fires, the `new_text` generator does both substitutions; T9 records
the indices T10 already handled and skips them.

## 5. Implementation outline

Single commit:

1. Add `match_t9_substitution(&Expr) -> Option<String>` and
   `match_t10_substitution(&Expr) -> Option<String>` in
   `src/bin/iterator_upgrade.rs`. Each returns the new-text on
   match.
2. Add `expr_contains_view_index(&Expr, idx: u32) -> bool` —
   recursive AST walk looking for the `View / Field / Unnamed(idx)`
   shape (a generalized version of the existing
   `view_field_index_of_it`).
3. Add `substitute_view_index(&mut Expr, idx: u32, replacement:
   &Expr)` — visit-mut walker that swaps every matching subexpression
   for `replacement` (a `MethodCall { receiver: Path("it"), method:
   <"index"|"seq">, args: [] }`).
4. In `match_single_expr` (after the existing T1–T8 fallthrough,
   before U-OTHER), add:
   - If `expr_contains_view_index(e, 1)`: do the seq substitution,
     emit T10 with new-text. If `expr_contains_view_index(e, 0)`
     also, fold the index substitution into the same new-text.
   - Else if `expr_contains_view_index(e, 0)`: emit T9 with
     index-substituted new-text.
5. The render path is unchanged — T9 and T10 use the existing
   `Transform` struct.

Lines of code: ~80–120. The substitution walker is the only new
helper; everything else is small.

## 6. Acceptance

### Matcher fixtures

Add two per-class fixtures alongside the existing
`tests/fixtures/iterator-upgrade-detect/{D1..D10,T1..T8}/`:

- `T9/fixture.rs` — minimal `verus!` with a loop invariant
  `0 <= i < it@.0` (not equality, not `.len()` comparison — must
  not match T1, T6, T7).
- `T10/fixture.rs` — minimal `verus!` with `it@.1.no_duplicates()`
  in a loop invariant (not equality with `self.seq@`, must not
  match T2, T3).

Both with checked-in `golden.compile`. Add `"T9"` and `"T10"` to
`CLASSES` in `tests/iterator_upgrade_matchers.rs`. `cargo test`
must pass on the augmented set.

### End-to-end on the fixture

After the binary lands, re-run the sweep:

```
cd ~/projects/veracity/tests/fixtures/APAS-VERUS && \
  ~/projects/veracity/target/release/veracity-iterator-upgrade \
    --detect --root . --out-dir analyses
```

**Expected deltas:**

| # | Metric | Before T9/T10 | After T9/T10 (expected) |
|--:|:-------|--------------:|------------------------:|
| 1 | T9 count | n/a | ~30 (top cluster #1 + index-bearing forall) |
| 2 | T10 count | n/a | ~50 (top clusters #2 + #3 + #4 + #5 + sub-expressions) |
| 3 | U-OTHER total | 253 | ~170 |
| 4 | Total T | 395 | ~475 |

If T9 count is < 25 or T10 count is < 40 after the run, the matcher
isn't covering the easy clusters; investigate before merging.

### Promotion logic update

The Phase 1.5 report flags clusters with count ≥ 5 as `→ T(new)` in
the U-OTHER table. After T9/T10 land, those rows should disappear
from the cluster table (they become T-findings, not U-OTHER). The
remaining U-OTHER clusters in row 5+ of the original table (the
multi-line `forall` quantifiers, the `OrderedSet::new(|p| ...)`
shapes) become the candidates for T11+ — but those are not trivial
substitutions and may stay U-OTHER permanently.

## 7. Out of scope

- **Quantifier-trigger rewriting.** Patterns like
  `forall|i| #![trigger seq[i]@.0] ...` contain `seq[i]@.0`, which
  shares a tail (`@.0`) with our matcher but is NOT
  `it@.0`-as-iterator. T9 will not fire on these (the matcher
  checks the View receiver is the path `it`, not any expression).
- **Spec-fn body rewrites.** T9/T10 fire on `ensures` / `requires`
  / `invariant` / `decreases` clauses, same as T1–T8. Spec-fn
  bodies (the contents of `open spec fn iter_invariant(...)`) are
  not touched — they're deleted wholesale via D10.
- **Cross-method rewrites.** A method named `index` or `seq` on a
  different receiver still parses fine; we're emitting calls on
  `it`, never on other paths. No risk of name collision.

## 8. Risks

1. **`it.index()` already taken.** If a Verus standard library
   defines `index` as a different method on the iterator type, we'd
   collide. Check `vstd/std_specs/iter.rs` for the `IteratorSpec`
   trait surface before merging. (Per plan §5 the prophetic model
   uses `it.index()` and `it.seq()` — so we're following the
   standard.)
2. **Trigger-bearing expressions.** Some U-OTHER clauses include
   `#[trigger]` annotations. T9/T10 substitution may move trigger
   markers in subtle ways; the matcher should leave attribute
   nodes alone and only swap the `View` subexpression. Verify with
   a trigger-bearing fixture (`T10_trigger/fixture.rs`).
3. **`old(it)@.0` and `old(it)@.1`.** Verus's `old()` produces a
   distinct `View` expression where the receiver is
   `Call { func: "old", args: [Path("it")] }`, not `Path("it")`
   directly. T9/T10 should NOT fire on these (different receiver).
   The matcher's existing `view_field_index_of_it` already requires
   the inner expression to be the literal path `it`, so this is
   safe by construction. Add a fixture proving it.

## 9. See also

- `~/projects/APAS-VERUS/plans/veracity-iterator-upgrade-detect.md` §5 (T1–T8 spec).
- `~/projects/APAS-VERUS/plans/veracity-iterator-upgrade-detect-phase1-5-report.md` §5 (U-OTHER clustering that motivated T9/T10).
- `~/projects/veracity/src/bin/iterator_upgrade.rs` `match_single_expr` — where T9/T10 will be inserted, after the existing T1–T8 cascade and before the U-OTHER fallthrough.
- `~/projects/veracity/tests/fixtures/iterator-upgrade-detect/` — where the new T9/T10 fixtures go.
