# Report — r205 `cargo check` on the rewritten fixture

**Date:** 2026-05-24
**Author:** veracity side (Claude, working in `~/projects/veracity/`)
**Subject plan:** `plans/r205-veracity-cargo-check-fixture.md`
**Disposition:** `cargo check` GREEN.

## 1. Final `cargo check` status

```
cd ~/projects/veracity/tests/fixtures/APAS-VERUS
cargo check
# → 0 errors. Finished dev profile [unoptimized + debuginfo].
```

✅ GREEN. The migration is syntactically and type-correct Rust.

Error reduction across the round-trip: **268 → 102 → 58 → 53 → 14 → 4 → 2 → 0** as the mutator fixes shipped (§2) and one stale `use` statement was hand-edited (§3).

Per plan §1, the `verus_keep_ghost` variant is documented as not applicable to this fixture (vstd's nightly-feature crate attrs aren't parseable by plain rustc, ~8400 errors in vstd alone). The proper gate for spec-clause Rust correctness is `scripts/validate.sh` — APAS-VERUS-side per plan §4.

## 2. Mutator-source fixes shipped

All eight fixes consolidated in veracity commit **`f60414c`**.

| # | veracity commit | Summary | Error class |
|--:|:----------------|:--------|:------------|
| 1 | `f60414c` | D6 fires only for delegated wrappers; non-delegated *Iter structs (custom, multi-field — BSTSet*MtEph, OrderedTableMtEph) are left alone | E0425 cannot find `OrderedTableMtEphIter` etc. |
| 2 | `f60414c` | D7/D8/D9/D10 gated on `delegated_iter_idents` — custom files keep View, Iterator, iter_invariant intact for the IteratorSpecImpl hand-port | E0277 `BSTSetSplayMtEphIter<T> is not an iterator` |
| 3 | `f60414c` | New D6-companion rewrite pass — for each deleted wrapper, walk the AST (verus_syn inside `verus!`, syn outside) and substitute Type::Path / Expr::Struct refs to the wrapper with its resolved inner type | E0425 cannot find `ArraySeqStEphIter`, `MappingStEphIter`, etc. |
| 4 | `f60414c` | `WrapperInfo { name, params, inner_type_text }`; per-file `wrapper_infos` is a Vec so files with multiple wrappers (BalBinTreeStEph: PreOrder/PostOrder/InOrder) work | E0425 `PreOrderIter`, `PostOrderIter` not found |
| 5 | `f60414c` | Parametric param→arg substitution at use sites — tokenize the inner type, replace wrapper-param idents with use-site arg idents, match by KIND (lifetime vs type) so elided lifetimes (`Wrapper<T>` for `<'a, T>`) work | E0107 `OrderedTableStEphIter<K, V>` → `IntoIter<Pair<K, V>>` (Pair-wrapping cases) |
| 6 | `f60414c` | `proc_macro2::Spacing::Joint` honored when re-rendering token streams so `::` round-trips as `::` not `: :` (and `'a` not `' a`) | ~2700 cascading "path not found" errors from broken `::` after first substitution attempt |
| 7 | `f60414c` | Bare `Iter` / `IntoIter` heads resolve to qualified `std::slice::Iter` / `std::vec::IntoIter` — cross-file callers without `use std::vec::IntoIter` still resolve | E0425 `cannot find type IntoIter` in AugOrderedTable*, BalBinTreeStEph |
| 8 | `f60414c` | `extract_top_generic_args` uses `syn::parse_str::<syn::Type>` instead of manual depth-counting on raw bytes — satisfies the string-hacking gate | (no error class; static-analysis gate) |

## 3. In-fixture hand-edits

| # | File | Reason | Tag-line text |
|--:|:-----|:-------|:--------------|
| 1 | `tests/fixtures/APAS-VERUS/src/Chap05/SetMtEph.rs` | `use ... HashSetWithViewPlusIter;` survived D6 deletion of the wrapper struct; one stale import, hand-removed | `// VERACITY-FIXTURE-MANUAL-FIX r205: removed stale 'use ... HashSetWithViewPlusIter;' import — the wrapper struct is deleted by D6` |

One file. The mutator's miss is bounded (use-statement cleanup is the obvious follow-up; one occurrence didn't justify the plumbing today). If a second similar case surfaces, promote to mutator per plan §2.1.

## 4. Errors veracity could not fix and is escalating

None.

## 5. References

- Plan: `plans/r205-veracity-cargo-check-fixture.md`
- Phase 2 plan: `plans/r204-veracity-iterator-upgrade-apply.md`
- Phase 2 report: `plans/r204-veracity-iterator-upgrade-apply-report.md`
- Mutator source: `~/projects/veracity/src/bin/iterator_upgrade.rs`
- Fixture root: `~/projects/veracity/tests/fixtures/APAS-VERUS/`
- Hand-edited file (tagged): `tests/fixtures/APAS-VERUS/src/Chap05/SetMtEph.rs`
