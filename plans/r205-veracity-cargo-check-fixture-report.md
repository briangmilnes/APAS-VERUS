# Report — r205 `cargo check` on the rewritten fixture

**Date:** 2026-05-24
**Author:** veracity side (Claude, working in `~/projects/veracity/`)
**Subject plan:** `plans/r205-veracity-cargo-check-fixture.md`
**Disposition:** `cargo check` GREEN. `cargo check --features verus_keep_ghost` NOT APPLICABLE — see §5.

## 1. Final status

| # | Variant | Result | Errors |
|--:|:--------|:-------|------:|
| 1 | `cargo check` | ✅ GREEN | 0 |
| 2 | `cargo check --features verus_keep_ghost` | ⚠ NOT APPLICABLE | — |

`cargo check` is clean on the rewritten fixture. The migration is
syntactically and type-correct Rust. Variant #2 is not achievable on
this fixture's stable toolchain; the proper gate for proof-clause
correctness is `scripts/validate.sh` (the Verus compiler) on the
APAS-VERUS side.

## 2. Mutator-source fixes shipped

Eight fixes to `~/projects/veracity/src/bin/iterator_upgrade.rs`
during the round-trip. Total errors went **268 → 0** across the
sequence.

| # | veracity commit | Summary | Error class fixed |
|--:|:----------------|:--------|:------------------|
| 1 | (pending commit) | D6 fires only for delegated wrappers; non-delegated *Iter structs (custom-style, multi-field) are left alone. Reverted prior over-aggressive deletion. | E0425 cannot find `OrderedTableMtEphIter` etc. (×34) |
| 2 | (pending commit) | D7/D8/D9/D10 gated on `delegated_iter_idents` set — only fire when the file's wrapper is single-std-iter / chained-APAS-iter. Custom files keep View, Iterator, iter_invariant. | E0277 `BSTSetSplayMtEphIter<T> is not an iterator` (×8) |
| 3 | (pending commit) | New "D6-companion" rewrite pass: for every D6-deleted wrapper, walk the file's AST (verus_syn for inside-verus!, syn for outside) and substitute Type::Path / Expr::Struct references to the wrapper with its resolved inner type. | E0425 cannot find `ArraySeqStEphIter`, `MappingStEphIter`, etc. (×~150) |
| 4 | (pending commit) | Wrapper inner type tracked WITH generics: `WrapperInfo { name, params: Vec<String>, inner_type_text: String }`. Per-file `wrapper_infos` is a Vec (BalBinTreeStEph has 3 wrappers: PreOrder/PostOrder/InOrder; each needs its own rewrite). | E0425 PreOrderIter, PostOrderIter not found (×4) |
| 5 | (pending commit) | Parametric param→arg substitution at use sites: tokenize the inner type, replace wrapper-param idents with use-site arg idents. Matches by KIND (lifetime vs type), not raw position, so elided lifetimes work (`Wrapper<T>` for `<'a, T>` maps 'a→'_, T→T). | E0107 struct takes 1 arg but 2 supplied — `OrderedTableStEphIter<K, V>` → `IntoIter<Pair<K, V>>` (×8 + 22 E0277) |
| 6 | (pending commit) | Token rendering honors `proc_macro2::Spacing::Joint` so `::` round-trips as `::` instead of `: :`. Same for `'a` lifetime tokens. | ~2700 cascading errors from broken `::` paths after first substitution attempt |
| 7 | (pending commit) | Bare `Iter` / `IntoIter` (relying on `use std::vec::IntoIter`) resolve to fully-qualified `std::vec::IntoIter` / `std::slice::Iter` at the use site, so cross-file callers without the same `use` statement still resolve. | E0425 `cannot find IntoIter` (×4 in AugOrderedTableStEph etc.) |
| 8 | (pending commit) | Multi-line forall/exists Substitutes correctly span `[line_start][col_start..]` + intermediate + `[line_end][..col_end]` and delete intervening lines. (This one was in Phase 2 already; surfaced again in r205 in different contexts.) | E0308 garbage-tail-suffix on multi-line clauses (×various) |

Recommended commit message for the consolidated change:

```
iterator-upgrade: r205 — D6-companion wrapper-usage rewrite + cargo check green

Adds the missing piece of the Phase 2 mutator: after deleting a D6
wrapper struct, the apply pass walks the file's AST (verus_syn for
inside-verus!, syn for outside) and substitutes every Type::Path /
Expr::Struct reference to the wrapper with its resolved inner type,
including parametric param-rename for the Pair-wrapping cases
(OrderedTable{StEph,StPer}, AugOrderedTable*, RelationStEph).

Also: D6–D10 deletions now gated on delegated_iter_idents so custom
wrappers (multi-field, e.g., BSTSet*MtEph, OrderedTableMtEph) survive
intact for the IteratorSpecImpl hand-port; pp_macro2 Spacing::Joint
honored so `::` paths round-trip correctly; bare `Iter`/`IntoIter`
resolve to qualified std::* form.

cargo check on the rewritten fixture: 0 errors (down from 268).
```

## 3. In-fixture hand-edits

One file required a manual fix because the mutator doesn't yet rewrite
`use` statements importing deleted wrapper names:

| # | File | Reason | Tag-line text |
|--:|:-----|:-------|:--------------|
| 1 | `tests/fixtures/APAS-VERUS/src/Chap05/SetMtEph.rs` | The `use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlusIter;` import survived D6 deletion of the wrapper. Hand-removed. | `// VERACITY-FIXTURE-MANUAL-FIX r205: removed stale 'use ... HashSetWithViewPlusIter;' import — the wrapper struct is deleted by D6` |

**Recommendation for the mutator (follow-up):** add a use-statement
deletion pass: walk `ItemUse` trees, detect any leaf that resolves to
a known wrapper name, schedule the use statement (or just that leaf
path) for deletion. One file's worth of fix-up doesn't justify the
plumbing today; if more files surface, promote to mutator.

## 4. Errors veracity could not fix and is escalating

None. Every error surfaced by `cargo check` was either fixed in the
mutator or hand-edited per §3.

## 5. `cargo check --features verus_keep_ghost` — not applicable

The plan §1 directed running this second variant. On this fixture
state it is not achievable with plain rustc, for two compounding
reasons:

1. **`verus_keep_ghost` is not a Cargo feature in this fixture's
   `Cargo.toml`.** It's a Verus-specific cfg flag, typically set via
   `--cfg verus_keep_ghost` in `rustc` invocation (see
   `scripts/verusdoc.sh` in the fixture).
2. **Setting `--cfg verus_keep_ghost` via `RUSTFLAGS` enables
   Verus-specific syntax in `vstd`** that plain rustc cannot parse.
   The result is **8383 errors in `vstd` alone** — none of which are
   in the rewritten APAS-VERUS code. Sample errors:

   ```
   error[E0554]: `#![feature]` may not be used on the stable release channel
    --> ~/projects/verus/source/builtin/src/lib.rs:5:5
   ```

   `vstd`'s ghost mode uses crate-level nightly features
   (`feature(rustc_attrs)`, `feature(negative_impls)`, etc.) plus
   `register_tool(verus)` — these need either the Verus compiler
   (which interprets them) or nightly + `-Zcrate-attr` plumbing.
   Plain `cargo check`, even on nightly, doesn't supply that.

The intent behind variant #2 (typecheck the spec clauses) is the
right thing to want. The right gate for that is `scripts/validate.sh`
— it invokes the Verus compiler, which knows the
`ensures`/`requires`/`invariant` shape. That's APAS-VERUS-side per
plan §8 and out of scope here.

If the orchestrator wants r205 to retry variant #2, two paths:

- **Add a `verus_keep_ghost` Cargo feature** to the fixture's
  `Cargo.toml` that turns on Verus-style cfgs WITHOUT pulling in the
  Verus-only syntax (essentially a no-op cfg). The check would then
  cover ensures/requires shape in user code, not vstd. Worth a
  separate plan if pursued.
- **Run `scripts/validate.sh`** on the rewritten fixture. This is the
  real gate. Veracity has not run validate (APAS-VERUS-side
  responsibility per §8); if you want veracity to attempt it, say so
  in a follow-up plan.

## 6. Numbers

Error count by mutator-fix step. Lower is better.

| # | Step | Errors |
|--:|:-----|------:|
| 1 | Initial `cargo check` after first Phase 2 apply | 268 |
| 2 | + D6-companion ident-only substitution | 102 |
| 3 | + D6 gated on delegated_iter_idents | 58 |
| 4 | + outside-verus! D6-companion pass (syn-based) | 53 |
| 5 | + D7/D8/D9/D10 gated on delegation | (no change — already covered) |
| 6 | + parametric substitution (broken `::` rendering) | 2698 |
| 7 | + `proc_macro2::Spacing::Joint` rendering fix | 14 |
| 8 | + kind-based param/arg matching (lifetime vs type) | 4 |
| 9 | + extract_top_generic_args via `syn` (string-hacking fix) | 2 |
| 10 | + 1 hand-edit (stale `use` statement) | **0** |

## 7. Matcher regression check

After all mutator changes, the existing per-class matcher fixtures
still pass:

```
cd tests/fixtures/iterator-upgrade-detect
for d in D1..D10 T1..T10; do
  veracity-iterator-upgrade --detect --root $d --out-dir /tmp/x
  diff <produced> <golden>
done
# PASS=20 FAIL=0
```

`veracity-review-string-hacking -f src/bin/iterator_upgrade.rs` → 0
violations.

## 8. What's next

| # | Item | Where | Effort |
|--:|:-----|:------|:-------|
| 1 | Commit + push the mutator changes from §2 | veracity side | minutes |
| 2 | Run `scripts/validate.sh` on the rewritten fixture | APAS-VERUS side (per plan §8) | per-chapter Verus time |
| 3 | If proof errors surface, decide: fix proofs (APAS-VERUS), tighten matcher (veracity), or hand-port the 3 U-CUSTOM AVLTreeSeq files | mixed | substantial |
| 4 | Optional: add use-statement deletion to the mutator (§3 promotion candidate) | veracity | ~2 hours |
| 5 | Optional: revisit `cargo check --features verus_keep_ghost` if a non-vstd cfg-flag scheme is wanted | APAS-VERUS Cargo.toml + veracity report variant | half a day |

## 9. References

- Subject plan: `plans/r205-veracity-cargo-check-fixture.md`
- Phase 2 report: `plans/r204-veracity-iterator-upgrade-apply-report.md`
- Mutator source: `~/projects/veracity/src/bin/iterator_upgrade.rs`
- Fixture root: `~/projects/veracity/tests/fixtures/APAS-VERUS/`
- Matcher fixtures: `~/projects/veracity/tests/fixtures/iterator-upgrade-detect/`
- Hand-edited file (only one): `tests/fixtures/APAS-VERUS/src/Chap05/SetMtEph.rs` — tagged with `VERACITY-FIXTURE-MANUAL-FIX r205`
