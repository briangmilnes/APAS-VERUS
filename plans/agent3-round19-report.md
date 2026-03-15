# Agent 3 — Round 19 Report: Tier 2 Spec Audit

## Task

Spec audit for Chap05 (Sets, Relations, Mappings, Kleene), Chap18 (Sequence ADT), and Chap19 (Parametric Sequence Implementation). Compare trait fn ensures/requires against APAS prose definitions, classify each as strong/partial/weak/missing, fix gaps, validate.

## Results

### Chap05 — Before/After

| # | File | Strong Before | Strong After | Fixes |
|---|------|---------------|--------------|-------|
| 1 | SetStEph.rs | 15/15 | 15/15 | None needed |
| 2 | SetMtEph.rs | 15/15 | 15/15 | None needed |
| 3 | RelationStEph.rs | 9/9 | 9/9 | None needed |
| 4 | MappingStEph.rs | 10/13 | 13/13 | 3 fixes |
| 5 | KleeneStPer.rs | 4/4 | 4/4 | None needed |

MappingStEph fixes:
- `size`: Added `ensures size == self@.dom().len()` (was missing)
- `from_vec`: Added content bridge ensures (was wf-only)
- `from_relation`: Added content bridge ensures (was wf-only)
- All 3 impl bodies marked `#[verifier::external_body]`

### Chap18 — Before/After

| # | File | Strong Before | Strong After | Fixes |
|---|------|---------------|--------------|-------|
| 1 | ArraySeqStEph.rs | 21/21 | 21/21 | None needed |
| 2 | ArraySeqStPer.rs | 21/21 | 21/21 | None needed |
| 3 | ArraySeqMtEph.rs | 21/21 | 21/21 | None needed |
| 4 | ArraySeqMtPer.rs | 21/21 | 21/21 | None needed |

All 84 Chap18 specs already strong.

### Chap19 — Before/After

| # | File | Strong Before | Strong After | Fixes |
|---|------|---------------|--------------|-------|
| 1 | ArraySeqStEph.rs | 19/24 | 24/24 | 5 fixes |
| 2 | ArraySeqStPer.rs | 19/24 | 24/24 | 5 fixes |
| 3 | ArraySeqMtEph.rs | 19/24 | 24/24 | 5 fixes |
| 4 | ArraySeqMtEphSlice.rs | 9/9 | 9/9 | None needed |

Per-function fixes (applied to all 3 files):
- `iterate_iter`: Added `Ghost(spec_f)` param, ensures `== spec_iterate(a.seq@, spec_f, seed)`
- `iterate`: Same pattern
- `reduce_iter`: Added `Ghost(spec_f)` + `spec_monoid(spec_f, id)`, ensures `== spec_iterate(a.seq@, spec_f, id)`
- `reduce`: Same pattern
- `scan`: Added `Ghost(spec_f)` + monoid, pointwise prefix ensures + total
- All 15 impl bodies marked `#[verifier::external_body]`
- Added `spec_iterate` spec fn (fold_left bridge) to StPer (StEph and MtEph already had it)
- Added monoid import to StPer

### Totals

| Chapter | Strong Before | Strong After | Delta |
|---------|---------------|--------------|-------|
| Chap05 | 53/56 | 56/56 | +3 |
| Chap18 | 84/84 | 84/84 | +0 |
| Chap19 | 66/81 | 81/81 | +15 |
| **Total** | **203/221** | **221/221** | **+18** |

## Holes

New `external_body` holes added: 18 (3 Chap05 + 15 Chap19).
New `Ghost::assume_new()` holes: 9 (recursive calls in external_body iterate/reduce).
These are proof targets for future rounds — the specs are now correct and strong.

Chap05 holes: 3 (all new external_body from this round).
Chap19 holes: 24 (15 external_body + 9 assume_new from this round).

## Technical Notes

- Chap19's single-trait structure (`ArraySeqStEphTrait`) caused a cyclic self-reference when ensures clauses used `a.spec_len()` / `a.spec_index(i)` inside `spec_iterate` (recursive via fold_left). Fixed by using `a.seq@` directly — equivalent since `spec_len()` unfolds to `a.seq@.len()`.
- Chap18 avoids this cycle because `spec_len`/`spec_index` are in `BaseTrait` while `iterate`/`reduce`/`scan` are in `RedefinableTrait` (separate trait).
- MtEph's `reduce_par` (bare impl, not trait) was unaffected — it already had strong specs.

## Verification

```
verification results:: 4012 verified, 0 errors
```

## Files Changed

- `src/Chap05/MappingStEph.rs` — strengthened 3 trait specs, external_body on 3 impls
- `src/Chap05/analyses/spec-audit.md` — created
- `src/Chap18/analyses/spec-audit.md` — created
- `src/Chap19/ArraySeqStEph.rs` — added spec_iterate, strengthened 5 trait specs, external_body on 5 impls
- `src/Chap19/ArraySeqStPer.rs` — added monoid import + spec_iterate, strengthened 5 trait specs, external_body on 5 impls
- `src/Chap19/ArraySeqMtEph.rs` — strengthened 5 trait specs, external_body on 5 impls
- `src/Chap19/analyses/spec-audit.md` — created
